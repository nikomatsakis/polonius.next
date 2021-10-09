#[cfg(test)]
mod test;

use crate::ast::{self, *};
use crate::ast_parser::parse_ast;
use std::fmt;

#[derive(Default, PartialEq, Eq)]
struct Origin(String);

#[derive(Default, PartialEq, Eq)]
struct Node(String);

impl<S> From<S> for Origin
where
    S: AsRef<str> + ToString,
{
    fn from(s: S) -> Self {
        Self(s.to_string())
    }
}

impl fmt::Debug for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<S> From<S> for Node
where
    S: AsRef<str> + ToString,
{
    fn from(s: S) -> Self {
        Self(s.to_string())
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Default, Debug)]
struct Facts {
    access_origin: Vec<(Origin, Node)>,
    cfg_edge: Vec<(Node, Node)>,
    clear_origin: Vec<(Origin, Node)>,
    introduce_subset: Vec<(Origin, Origin, Node)>,
    invalidate_origin: Vec<(Origin, Node)>,
}

#[allow(dead_code)]
fn emit_facts(program: &str) -> eyre::Result<Facts> {
    let program = parse_ast(program)?;
    let emitter = FactEmitter { program };
    let mut facts = Default::default();
    emitter.emit_facts(&mut facts);
    Ok(facts)
}

struct FactEmitter {
    program: ast::Program,
}

impl FactEmitter {
    fn emit_facts(&self, facts: &mut Facts) {
        for bb in &self.program.basic_blocks {
            self.emit_block_facts(bb, facts);
        }
    }

    fn emit_block_facts(&self, bb: &BasicBlock, facts: &mut Facts) {
        // Emit CFG facts for the block
        self.emit_cfg_edges(&bb, facts);

        for (idx, s) in bb.statements.iter().enumerate() {
            // TODO: emit `node_text` per statement, but that string could be
            // collected at the parser level

            match &**s {
                Statement::Assign(place, expr) => {
                    // Emit facts about the assignment RHS: evaluate the `expr`
                    self.emit_expr_facts(bb, idx, expr, facts);

                    // Emit facts about the assignment LHS
                    let lhs_ty = self.ty_of_place(place);
                    match &lhs_ty {
                        Ty::Ref { origin, .. } | Ty::RefMut { origin, .. } => {
                            // Assignments to references clear all origins in their type
                            //
                            // TODO: actually clear all origins in `ty` and not just the root
                            facts
                                .clear_origin
                                .push((origin.into(), node_at(&bb.name, idx)));
                        }

                        _ => {
                            // Assignments to non-references invalidate the loan origin
                            //
                            // TODO: handle assignments to fields.
                            //
                            // TMP: until then: only support assignments to variables, and use
                            // their name as the loan origin name.
                            //
                            // TODO: collect the loan origins actually used in the program and
                            // invalidate those. If nothing is borrowing from the value, we don't
                            // need to do this invalidation or propagate it.
                            let v = self
                                .program
                                .variables
                                .iter()
                                .find(|v| v.name == place.base)
                                .unwrap_or_else(|| panic!("Can't find variable {}", place.base));
                            facts
                                .invalidate_origin
                                .push((format!("'L_{}", v.name).into(), node_at(&bb.name, idx)));
                        }
                    }

                    // Introduce subsets: `expr` flows into `place`
                    //
                    // TODO: do we need some type checking to ensure this assigment is valid
                    // with respect to the LHS/RHS types, mutability, etc ?
                    //
                    // TODO: handles simple subsets only for now, complete this.
                    //
                    // TODO: if the `expr` is a call, we probably also need subsets between
                    // the arguments, the return value and the LHS ?
                    //
                    // We're in an assignment and we assume the LHS and RHS have the same shape,
                    // for example `&'a Type<&'b i32> = &'1 Type<'2 i32>`.
                    //
                    match lhs_ty {
                        Ty::Ref {
                            origin: target_origin,
                            ..
                        }
                        | Ty::RefMut {
                            origin: target_origin,
                            ..
                        } => {
                            let mut emit_subset_fact = |source_origin, target_origin| {
                                facts.introduce_subset.push((
                                    source_origin,
                                    target_origin,
                                    node_at(&bb.name, idx),
                                ));
                            };

                            match expr {
                                Expr::Access {
                                    kind:
                                        AccessKind::Borrow(source_origin)
                                        | AccessKind::BorrowMut(source_origin),
                                    ..
                                } => {
                                    emit_subset_fact(source_origin.into(), target_origin.into());
                                }

                                Expr::Access {
                                    kind: AccessKind::Copy | AccessKind::Move,
                                    place,
                                } => {
                                    let rhs_ty = self.ty_of_place(place);
                                    match rhs_ty {
                                        Ty::Ref {
                                            origin: source_origin,
                                            ..
                                        }
                                        | Ty::RefMut {
                                            origin: source_origin,
                                            ..
                                        } => {
                                            emit_subset_fact(
                                                source_origin.into(),
                                                target_origin.into(),
                                            );
                                        }

                                        _ => {
                                            // The RHS has no refs, there are no subsets to emit
                                        }
                                    }
                                }

                                _ => {
                                    // The expr is not borrowing anything, there are no
                                    // subsets to emit
                                }
                            }
                        }

                        _ => {
                            // The LHS contains no origins, there are no subsets to emit
                        }
                    }
                }

                Statement::Expr(expr) => {
                    // Evaluate the `expr`
                    self.emit_expr_facts(bb, idx, expr, facts);

                    // TODO: is there something more to do because we're in a "drop" ?
                }
            }
        }
    }

    fn emit_expr_facts(&self, bb: &BasicBlock, idx: usize, expr: &Expr, facts: &mut Facts) {
        match expr {
            Expr::Access { kind, place } => {
                match kind {
                    // Borrowing clears its origin: it's issuing a fresh origin of the same name
                    AccessKind::Borrow(origin) | AccessKind::BorrowMut(origin) => {
                        facts
                            .clear_origin
                            .push((origin.into(), node_at(&bb.name, idx)));
                    }

                    AccessKind::Copy | AccessKind::Move => {
                        // FIXME: currently function call parameters are not parsed without access
                        // kinds, check if there's some special behaviour needed for copy/moves,
                        // instead of just being "reads" (e.g. maybe moves also need clearing
                        // or invalidations)

                        // Reading a reference accesses its origin
                        // TODO: it probably accesses _all_ the origins in its type
                        match self.ty_of_place(place) {
                            Ty::Ref { origin, .. } | Ty::RefMut { origin, .. } => {
                                facts
                                    .access_origin
                                    .push((origin.into(), node_at(&bb.name, idx)));
                            }

                            _ => {}
                        }
                    }
                }
            }

            Expr::Call { arguments, .. } => {
                // Calls evaluate their arguments
                arguments
                    .iter()
                    .for_each(|expr| self.emit_expr_facts(bb, idx, expr, facts));

                // TODO: Depending on the signature of the function, some subsets can be introduced
                // between the arguments to the call
            }

            _ => {}
        }
    }

    fn emit_cfg_edges(&self, bb: &BasicBlock, facts: &mut Facts) {
        let statement_count = bb.statements.len();

        // Emit intra-block CFG edges between statements
        for idx in 1..statement_count {
            facts
                .cfg_edge
                .push((node_at(&bb.name, idx - 1), node_at(&bb.name, idx)));
        }

        // Emit inter-block CFG edges between a block and its successors
        for succ in &bb.successors {
            // Note: `goto`s are not statements, so a block with a single goto
            // has no statements but still needs a node index in the CFG.
            facts.cfg_edge.push((
                node_at(&bb.name, statement_count.saturating_sub(1)),
                node_at(succ, 0),
            ));
        }
    }

    fn ty_of_place(&self, place: &Place) -> Ty {
        // The `base` is always a variable of the program
        let v = self
            .program
            .variables
            .iter()
            .find(|v| v.name == place.base)
            .unwrap_or_else(|| panic!("Can't find variable {}", place.base));

        let ty = if place.fields.is_empty() {
            &v.ty
        } else {
            // If there are any fields, then this must be a struct
            assert!(matches!(v.ty, Ty::Struct { .. }));

            // Find the type of each field in sequence, to return the last field's type
            place.fields.iter().fold(&v.ty, |ty, field_name| {
                // Find the struct decl for the current step's ty
                let (struct_name, struct_substs) = match ty {
                    Ty::Struct { name, parameters } => (name, parameters),
                    _ => panic!("Ty {:?} must be a struct to access its fields", ty),
                };
                let decl = self
                    .program
                    .struct_decls
                    .iter()
                    .find(|s| &s.name == struct_name)
                    .unwrap_or_else(|| {
                        panic!("Can't find struct {} at field {}", struct_name, field_name,)
                    });

                // Find the expected named field inside the struct decl
                let field = decl
                    .field_decls
                    .iter()
                    .find(|v| &v.name == field_name)
                    .unwrap_or_else(|| {
                        panic!("Can't find field {} in struct {}", field_name, struct_name)
                    });

                // It's possible that the field has a generic type, which we need to substitute
                // with the matching type from the struct's arguments
                match &field.ty {
                    Ty::Struct {
                        name: field_ty_name,
                        ..
                    } => {
                        if let Some(idx) = decl.generic_decls.iter().position(|d| match d {
                            GenericDecl::Ty(param_ty_name) => param_ty_name == field_ty_name,
                            _ => false,
                        }) {
                            // We found the field ty in the generic decls, so return the subst
                            // at the same index
                            match &struct_substs[idx] {
                                Parameter::Ty(subst_ty) => subst_ty,

                                // TODO: handle generic origins
                                _ => panic!("The parameter at idx {} should be a Ty", idx),
                            }
                        } else {
                            // Otherwise, the field ty is a regular type
                            &field.ty
                        }
                    }
                    _ => &field.ty,
                }
            })
        };

        ty.clone()
    }
}

fn node_at(block: &str, idx: usize) -> Node {
    format!("{}[{}]", block, idx).into()
}
