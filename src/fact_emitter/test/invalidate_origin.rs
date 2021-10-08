use super::*;
use insta::assert_debug_snapshot;

#[test]
fn assignment_to_non_references() {
    // Simple assignment
    let program = "
        let x: i32;

        bb0: {
            x = 22;
        }
    ";
    assert_debug_snapshot!(expect_facts(program).invalidate_origin, @r###"
    [
        (
            "'L_x",
            "bb0[0]",
        ),
    ]
    "###);

    // Function call return value
    let program = "
        let v: Vec;
        bb0: {
            v = Vec_new();
        }
    ";
    assert_debug_snapshot!(expect_facts(program).invalidate_origin, @r###"
    [
        (
            "'L_v",
            "bb0[0]",
        ),
    ]
    "###);
}
