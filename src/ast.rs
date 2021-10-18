use derive_more::IsVariant;

use crate::span::Spanned as Sp;

#[derive(Clone, Debug)]
pub struct Program {
    pub struct_decls: Vec<StructDecl>,
    pub fn_prototypes: Vec<FnPrototype>,
    pub variables: Vec<VariableDecl>,
    pub basic_blocks: Vec<BasicBlock>,
}

#[derive(Clone, Debug)]
pub struct StructDecl {
    pub name: Name,
    pub generic_decls: Vec<GenericDecl>,
    pub field_decls: Vec<VariableDecl>,
}

#[derive(Clone, Debug)]
pub struct VariableDecl {
    pub name: Name,
    pub ty: Ty,
}

#[derive(Clone, Debug)]
pub struct FnPrototype {
    pub name: Name,
    pub generic_decls: Vec<GenericDecl>,
    pub arg_tys: Vec<Ty>,
    pub ret_ty: Ty,
}

#[derive(Clone, Debug)]
pub enum GenericDecl {
    Origin(Name),
    Ty(Name),
}

#[derive(Clone, Debug)]
pub struct BasicBlock {
    pub name: Name,
    pub statements: Vec<Sp<Statement>>,
    pub successors: Vec<Name>,
}

#[derive(Clone, Debug)]
pub enum Statement {
    /// An assignment (`place = expr;`).
    Assign(Place, Expr),

    /// A bare expression (`expr;`).
    Expr(Expr),
}

#[derive(Clone, Debug)]
pub enum Expr {
    Access(ExprAccess),
    Number { value: i32 },
    Call { name: Name, arguments: Vec<Expr> },
    Unit,
}

#[derive(Clone, Debug)]
pub struct ExprAccess {
    pub place: Place,
    pub kind: AccessKind,
}

#[derive(Clone, Debug)]
pub enum AccessKind {
    Copy,
    Move,
    Borrow(Name),
    BorrowMut(Name),
}

#[derive(Clone, Debug)]
pub enum Ty {
    Ref {
        origin: Name,
        ty: Box<Ty>,
    },

    RefMut {
        origin: Name,
        ty: Box<Ty>,
    },

    I32,

    Unit,

    Struct {
        name: Name,
        parameters: Vec<Parameter>,
    },
}

#[derive(Clone, Debug)]
pub enum Parameter {
    Origin(Name),
    Ty(Ty),
}

#[derive(Clone, Debug, PartialEq, Eq, IsVariant)]
pub enum Projection {
    Field(Name),
    Deref,
}

#[derive(Clone, Debug)]
pub struct Place {
    pub base: Name,

    /// Any projections on `base`, starting from the innermost one.
    ///
    /// For example, `x.f1.f2` would give `vec!["f1", "f2"]`.
    pub projections: Vec<Projection>,
}


impl Place {
    /// Two places are disjoint if one is not a prefix of the other.
    pub fn is_disjoint(&self, other: &Place) -> bool {
        if self.base != other.base {
            return true;
        }

        self.projections
            .iter()
            .zip(other.projections.iter())
            .any(|(a, b)| a != b)
    }

    pub fn num_derefs(&self) -> usize {
        self.projections.iter().filter(|p| p.is_deref()).count()
    }
}


pub type Name = String;
