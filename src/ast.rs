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
    Access { kind: AccessKind, place: Place },
    Number { value: i32 },
    Call { name: Name, arguments: Vec<Expr> },
    Unit,
}

#[derive(Clone, Debug)]
pub enum AccessKind {
    Copy,
    Move,
    Borrow(Name),
    BorrowMut(Name),
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

impl Ty {
    /// If this is a reference type, returns the type of the target of that reference.
    pub fn target(&self) -> Option<&Ty> {
        match self {
            Self::Ref { ty, .. } | Self::RefMut { ty, .. } => Some(&*ty),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Parameter {
    Origin(Name),
    Ty(Ty),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Projection {
    Field(Name),
    Deref,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Place {
    pub base: Name,

    /// Any projections on `base`, starting from the innermost one.
    ///
    /// For example, `x.f1.f2` would give `vec!["f1", "f2"]`.
    pub projections: Vec<Projection>,
}

pub type Name = String;
