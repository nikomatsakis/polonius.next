use super::*;

pub fn expect_parse(s: &str) -> ast::Program {
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
                                projections: [],
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
                                projections: [],
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
                                projections: [],
                            },
                            Access {
                                kind: Borrow(
                                    "'y",
                                ),
                                place: Place {
                                    base: "x",
                                    projections: [],
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
                                projections: [],
                            },
                            Access {
                                kind: BorrowMut(
                                    "'z",
                                ),
                                place: Place {
                                    base: "x",
                                    projections: [],
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
                                projections: [],
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
                                projections: [],
                            },
                            Access {
                                kind: Copy,
                                place: Place {
                                    base: "x",
                                    projections: [],
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
                                projections: [],
                            },
                            Access {
                                kind: Move,
                                place: Place {
                                    base: "x",
                                    projections: [],
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
    let program = "
        let x: i32;
        let v: Vec<&'v mut i32>;
        let p: &'p i32;
        let tmp: &'tmp0 mut Vec<&'tmp1 mut i32>;

        bb0: {
            x = 22;
            v = Vec_new();
            p = &'L_x x;
            tmp = &'L_v mut v;
            Vec_push(move tmp, move p);
            x = 44;
            Vec_len(copy v);
        }
    ";
    insta::assert_debug_snapshot!(expect_parse(program));
}

#[test]
fn example_issue_47680() {
    let program = "
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
    ";
    insta::assert_debug_snapshot!(expect_parse(program));
}
