use super::*;

fn expect_parse(s: &str) -> ast::Program {
    match super::ast_parser::program(s) {
        Ok(p) => p,
        Err(e) => {
            let offset = e.location.offset;
            panic!(
                "parsing failed: {}\n{} (*) {}",
                e.expected,
                &s[..offset],
                &s[offset..]
            );
        }
    }
}

#[test]
fn let_test() {
    let p = expect_parse(
        "
        let x: i32; 
    ",
    );

    insta::assert_debug_snapshot!(p, @r###"
    Program {
        struct_decls: [],
        fn_prototypes: [],
        variables: [
            VariableDecl {
                name: "x",
                ty: I32,
            },
        ],
        basic_blocks: [],
    }
    "###);
}

#[test]
fn statement_test() {
    let p = expect_parse(
        "
        bb0: {
            x = 22;
        }
    ",
    );

    insta::assert_debug_snapshot!(p, @r###"
    Program {
        struct_decls: [],
        fn_prototypes: [],
        variables: [],
        basic_blocks: [
            BasicBlock {
                name: "bb0",
                statements: [
                    Spanned {
                        span: Span {
                            start: 28,
                            end: 35,
                        },
                        inner: Assign(
                            Place {
                                base: "x",
                                fields: [],
                            },
                            Number {
                                value: 22,
                            },
                        ),
                    },
                ],
                successors: [],
            },
        ],
    }
    "###);
}

#[test]
fn borrow_test() {
    let p = expect_parse(
        "
        bb0: {
            x = 22;
            y = &'y x;
            z = &'z mut x;
            goto bb1, bb2;
        }

        bb1: { }
        bb2: { }
    ",
    );

    insta::assert_debug_snapshot!(p, @r###"
    Program {
        struct_decls: [],
        fn_prototypes: [],
        variables: [],
        basic_blocks: [
            BasicBlock {
                name: "bb0",
                statements: [
                    Spanned {
                        span: Span {
                            start: 28,
                            end: 35,
                        },
                        inner: Assign(
                            Place {
                                base: "x",
                                fields: [],
                            },
                            Number {
                                value: 22,
                            },
                        ),
                    },
                    Spanned {
                        span: Span {
                            start: 48,
                            end: 58,
                        },
                        inner: Assign(
                            Place {
                                base: "y",
                                fields: [],
                            },
                            Access {
                                kind: Borrow(
                                    "'y",
                                ),
                                place: Place {
                                    base: "x",
                                    fields: [],
                                },
                            },
                        ),
                    },
                    Spanned {
                        span: Span {
                            start: 71,
                            end: 85,
                        },
                        inner: Assign(
                            Place {
                                base: "z",
                                fields: [],
                            },
                            Access {
                                kind: BorrowMut(
                                    "'z",
                                ),
                                place: Place {
                                    base: "x",
                                    fields: [],
                                },
                            },
                        ),
                    },
                ],
                successors: [
                    "bb1",
                    "bb2",
                ],
            },
            BasicBlock {
                name: "bb1",
                statements: [],
                successors: [],
            },
            BasicBlock {
                name: "bb2",
                statements: [],
                successors: [],
            },
        ],
    }
    "###);
}

#[test]
fn copy_move_test() {
    let p = expect_parse(
        "
        let x: i32;
        let y: i32;
        let z: i32;
        bb0: {
            x = 22;
            y = copy x;
            z = move x;
        }
    ",
    );

    insta::assert_debug_snapshot!(p, @r###"
    Program {
        struct_decls: [],
        fn_prototypes: [],
        variables: [
            VariableDecl {
                name: "x",
                ty: I32,
            },
            VariableDecl {
                name: "y",
                ty: I32,
            },
            VariableDecl {
                name: "z",
                ty: I32,
            },
        ],
        basic_blocks: [
            BasicBlock {
                name: "bb0",
                statements: [
                    Spanned {
                        span: Span {
                            start: 88,
                            end: 95,
                        },
                        inner: Assign(
                            Place {
                                base: "x",
                                fields: [],
                            },
                            Number {
                                value: 22,
                            },
                        ),
                    },
                    Spanned {
                        span: Span {
                            start: 108,
                            end: 119,
                        },
                        inner: Assign(
                            Place {
                                base: "y",
                                fields: [],
                            },
                            Access {
                                kind: Copy,
                                place: Place {
                                    base: "x",
                                    fields: [],
                                },
                            },
                        ),
                    },
                    Spanned {
                        span: Span {
                            start: 132,
                            end: 143,
                        },
                        inner: Assign(
                            Place {
                                base: "z",
                                fields: [],
                            },
                            Access {
                                kind: Move,
                                place: Place {
                                    base: "x",
                                    fields: [],
                                },
                            },
                        ),
                    },
                ],
                successors: [],
            },
        ],
    }
    "###);
}

#[test]
fn struct_test() {
    let p = expect_parse(
        "struct Iter<'me, T> { vec: &'me Vec<T>, position: i32 }
        struct Vec<T> { item0: T }
        
    ",
    );

    insta::assert_debug_snapshot!(p, @r###"
    Program {
        struct_decls: [
            StructDecl {
                name: "Iter",
                generic_decls: [
                    Origin(
                        "'me",
                    ),
                    Ty(
                        "T",
                    ),
                ],
                field_decls: [
                    VariableDecl {
                        name: "vec",
                        ty: Ref {
                            origin: "'me",
                            ty: Struct {
                                name: "Vec",
                                parameters: [
                                    Ty(
                                        Struct {
                                            name: "T",
                                            parameters: [],
                                        },
                                    ),
                                ],
                            },
                        },
                    },
                    VariableDecl {
                        name: "position",
                        ty: I32,
                    },
                ],
            },
            StructDecl {
                name: "Vec",
                generic_decls: [
                    Ty(
                        "T",
                    ),
                ],
                field_decls: [
                    VariableDecl {
                        name: "item0",
                        ty: Struct {
                            name: "T",
                            parameters: [],
                        },
                    },
                ],
            },
        ],
        fn_prototypes: [],
        variables: [],
        basic_blocks: [],
    }
    "###);
}

#[test]
fn fn_test() {
    let p = expect_parse(
        "
        struct Vec<T> { element: T }
        fn Vec_push<'v, T>(v: &'v mut Vec<T>, element: T) -> ();
    ",
    );

    insta::assert_debug_snapshot!(p, @r###"
    Program {
        struct_decls: [
            StructDecl {
                name: "Vec",
                generic_decls: [
                    Ty(
                        "T",
                    ),
                ],
                field_decls: [
                    VariableDecl {
                        name: "element",
                        ty: Struct {
                            name: "T",
                            parameters: [],
                        },
                    },
                ],
            },
        ],
        fn_prototypes: [
            FnPrototype {
                name: "Vec_push",
                generic_decls: [
                    Origin(
                        "'v",
                    ),
                    Ty(
                        "T",
                    ),
                ],
                arg_tys: [
                    RefMut {
                        origin: "'v",
                        ty: Struct {
                            name: "Vec",
                            parameters: [
                                Ty(
                                    Struct {
                                        name: "T",
                                        parameters: [],
                                    },
                                ),
                            ],
                        },
                    },
                    Struct {
                        name: "T",
                        parameters: [],
                    },
                ],
                ret_ty: Unit,
            },
        ],
        variables: [],
        basic_blocks: [],
    }
    "###);
}

#[test]
fn example_vec_temp() {
    let p = expect_parse("
        let x: i32;
        let v: Vec<&'v mut i32>;
        let p: &'p i32;
        let tmp: &'tmp0 mut Vec<&'tmp1 mut i32>;
        let len: i32;

        bb0: {
            x = 22;
            v = Vec_new();
            p = &'L_x x;
            tmp = &'L_v mut v;
            Vec_push(move tmp, move p);
            x = 44;
            len = Vec_len(copy v);
        }
    ");
    insta::assert_debug_snapshot!(p, @r###"
    Program {
        struct_decls: [],
        fn_prototypes: [],
        variables: [
            VariableDecl {
                name: "x",
                ty: I32,
            },
            VariableDecl {
                name: "v",
                ty: Struct {
                    name: "Vec",
                    parameters: [
                        Ty(
                            RefMut {
                                origin: "'v",
                                ty: I32,
                            },
                        ),
                    ],
                },
            },
            VariableDecl {
                name: "p",
                ty: Ref {
                    origin: "'p",
                    ty: I32,
                },
            },
            VariableDecl {
                name: "tmp",
                ty: RefMut {
                    origin: "'tmp0",
                    ty: Struct {
                        name: "Vec",
                        parameters: [
                            Ty(
                                RefMut {
                                    origin: "'tmp1",
                                    ty: I32,
                                },
                            ),
                        ],
                    },
                },
            },
            VariableDecl {
                name: "len",
                ty: I32,
            },
        ],
        basic_blocks: [
            BasicBlock {
                name: "bb0",
                statements: [
                    Spanned {
                        span: Span {
                            start: 177,
                            end: 184,
                        },
                        inner: Assign(
                            Place {
                                base: "x",
                                fields: [],
                            },
                            Number {
                                value: 22,
                            },
                        ),
                    },
                    Spanned {
                        span: Span {
                            start: 197,
                            end: 211,
                        },
                        inner: Assign(
                            Place {
                                base: "v",
                                fields: [],
                            },
                            Call {
                                name: "Vec_new",
                                arguments: [],
                            },
                        ),
                    },
                    Spanned {
                        span: Span {
                            start: 224,
                            end: 236,
                        },
                        inner: Assign(
                            Place {
                                base: "p",
                                fields: [],
                            },
                            Access {
                                kind: Borrow(
                                    "'L_x",
                                ),
                                place: Place {
                                    base: "x",
                                    fields: [],
                                },
                            },
                        ),
                    },
                    Spanned {
                        span: Span {
                            start: 249,
                            end: 267,
                        },
                        inner: Assign(
                            Place {
                                base: "tmp",
                                fields: [],
                            },
                            Access {
                                kind: BorrowMut(
                                    "'L_v",
                                ),
                                place: Place {
                                    base: "v",
                                    fields: [],
                                },
                            },
                        ),
                    },
                    Spanned {
                        span: Span {
                            start: 280,
                            end: 307,
                        },
                        inner: Expr(
                            Call {
                                name: "Vec_push",
                                arguments: [
                                    Access {
                                        kind: Move,
                                        place: Place {
                                            base: "tmp",
                                            fields: [],
                                        },
                                    },
                                    Access {
                                        kind: Move,
                                        place: Place {
                                            base: "p",
                                            fields: [],
                                        },
                                    },
                                ],
                            },
                        ),
                    },
                    Spanned {
                        span: Span {
                            start: 320,
                            end: 327,
                        },
                        inner: Assign(
                            Place {
                                base: "x",
                                fields: [],
                            },
                            Number {
                                value: 44,
                            },
                        ),
                    },
                    Spanned {
                        span: Span {
                            start: 340,
                            end: 362,
                        },
                        inner: Assign(
                            Place {
                                base: "len",
                                fields: [],
                            },
                            Call {
                                name: "Vec_len",
                                arguments: [
                                    Access {
                                        kind: Copy,
                                        place: Place {
                                            base: "v",
                                            fields: [],
                                        },
                                    },
                                ],
                            },
                        ),
                    },
                ],
                successors: [],
            },
        ],
    }
    "###);
}

#[test]
fn example_issue_47680() {
    let p = expect_parse("
        let temp: &'temp mut Thing;
        let t0: &'t0 mut Thing;
        let v: &'v mut Thing;

        bb0: {
            temp = &'L_Thing mut Thing;
            goto bb1;
        }

        bb1: {            
            t0 = &'L_*temp mut *temp;
            v = MaybeNext(move t0);
            goto bb2, bb3;
        }

        bb2: {
            temp = move v;
            goto bb4;
        }

        bb3: {
            goto bb4;
        }

        bb4: {
            goto bb1;
        }
    ");
    insta::assert_debug_snapshot!(p, @r###"
    Program {
        struct_decls: [],
        fn_prototypes: [],
        variables: [
            VariableDecl {
                name: "temp",
                ty: RefMut {
                    origin: "'temp",
                    ty: Struct {
                        name: "Thing",
                        parameters: [],
                    },
                },
            },
            VariableDecl {
                name: "t0",
                ty: RefMut {
                    origin: "'t0",
                    ty: Struct {
                        name: "Thing",
                        parameters: [],
                    },
                },
            },
            VariableDecl {
                name: "v",
                ty: RefMut {
                    origin: "'v",
                    ty: Struct {
                        name: "Thing",
                        parameters: [],
                    },
                },
            },
        ],
        basic_blocks: [
            BasicBlock {
                name: "bb0",
                statements: [
                    Spanned {
                        span: Span {
                            start: 127,
                            end: 154,
                        },
                        inner: Assign(
                            Place {
                                base: "temp",
                                fields: [],
                            },
                            Access {
                                kind: BorrowMut(
                                    "'L_Thing",
                                ),
                                place: Place {
                                    base: "Thing",
                                    fields: [],
                                },
                            },
                        ),
                    },
                ],
                successors: [
                    "bb1",
                ],
            },
            BasicBlock {
                name: "bb1",
                statements: [
                    Spanned {
                        span: Span {
                            start: 227,
                            end: 252,
                        },
                        inner: Assign(
                            Place {
                                base: "t0",
                                fields: [],
                            },
                            Access {
                                kind: BorrowMut(
                                    "'L_*temp",
                                ),
                                place: Place {
                                    base: "*temp",
                                    fields: [],
                                },
                            },
                        ),
                    },
                    Spanned {
                        span: Span {
                            start: 265,
                            end: 288,
                        },
                        inner: Assign(
                            Place {
                                base: "v",
                                fields: [],
                            },
                            Call {
                                name: "MaybeNext",
                                arguments: [
                                    Access {
                                        kind: Move,
                                        place: Place {
                                            base: "t0",
                                            fields: [],
                                        },
                                    },
                                ],
                            },
                        ),
                    },
                ],
                successors: [
                    "bb2",
                    "bb3",
                ],
            },
            BasicBlock {
                name: "bb2",
                statements: [
                    Spanned {
                        span: Span {
                            start: 354,
                            end: 368,
                        },
                        inner: Assign(
                            Place {
                                base: "temp",
                                fields: [],
                            },
                            Access {
                                kind: Move,
                                place: Place {
                                    base: "v",
                                    fields: [],
                                },
                            },
                        ),
                    },
                ],
                successors: [
                    "bb4",
                ],
            },
            BasicBlock {
                name: "bb3",
                statements: [],
                successors: [
                    "bb4",
                ],
            },
            BasicBlock {
                name: "bb4",
                statements: [],
                successors: [
                    "bb1",
                ],
            },
        ],
    }
    "###);
}
