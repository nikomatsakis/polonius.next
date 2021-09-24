use super::*;

#[test]
fn let_test() {
    let p = parse_ast(
        "
        let x: i32; 
    ",
    );

    insta::assert_debug_snapshot!(p, @r###"
    Ok(
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
        },
    )
    "###);
}

#[test]
fn statement_test() {
    let p = parse_ast(
        "
        bb0: {
            x = 22
        }
    ",
    );

    insta::assert_debug_snapshot!(p, @r###"
    Ok(
        Program {
            struct_decls: [],
            fn_prototypes: [],
            variables: [],
            basic_blocks: [
                BasicBlock {
                    name: "bb0",
                    statements: [
                        Assign(
                            Place {
                                base: "x",
                                fields: [],
                            },
                            Number {
                                value: 22,
                            },
                        ),
                    ],
                    successors: [],
                },
            ],
        },
    )
    "###);
}

#[test]
fn borrow_test() {
    let p = parse_ast(
        "
        bb0: {
            x = 22
            y = &'y x
            z = &'z mut x
        }
    ",
    );

    insta::assert_debug_snapshot!(p, @r###"
    Ok(
        Program {
            struct_decls: [],
            fn_prototypes: [],
            variables: [],
            basic_blocks: [
                BasicBlock {
                    name: "bb0",
                    statements: [
                        Assign(
                            Place {
                                base: "x",
                                fields: [],
                            },
                            Number {
                                value: 22,
                            },
                        ),
                        Assign(
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
                        Assign(
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
                    ],
                    successors: [],
                },
            ],
        },
    )
    "###);
}
