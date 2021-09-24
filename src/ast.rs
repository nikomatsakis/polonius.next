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
    pub statements: Vec<Statement>,
    pub successors: Vec<Name>,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Assign(Place, Expr),
    Drop(Expr),
}

#[derive(Clone, Debug)]
pub enum Expr {
    Access { kind: AccessKind, place: Place },

    Call { name: Name, arguments: Vec<Expr> },
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

#[derive(Clone, Debug)]
pub struct Place {
    base: Name,
    fields: Vec<Name>,
}

pub type Name = String;
