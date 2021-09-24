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
