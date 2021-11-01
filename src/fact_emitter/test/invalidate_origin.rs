use super::*;
use insta::assert_debug_snapshot;

#[test]
fn assignment_to_non_references() {
    // Simple assignment
    let program = "
        let x: i32;
        let y: &'y i32;

        bb0: {
            x = 22;
            y = &'L_x x;
        }
    ";
    assert_debug_snapshot!(expect_facts(program).invalidate_origin, @r###"
    [
        (
            "'L_x",
            "a",
        ),
    ]
    "###);

    // Function call return value
    let program = "
        let v: Vec;
        let ref: &'ref Vec;
        bb0: {
            v = Vec_new();
            ref = &'L_v v;
        }
    ";
    assert_debug_snapshot!(expect_facts(program).invalidate_origin, @r###"
    [
        (
            "'L_v",
            "a",
        ),
    ]
    "###);
}

#[test]
fn mut_borrows_invalidate_loans() {
    // Similar to the first test above, but with the mut mode, creating an
    // additional invalidation
    let program = "
        let x: i32;
        let y: &'y mut i32;

        bb0: {
            x = 22;
            y = &'L_x mut x;
        }
    ";
    assert_debug_snapshot!(expect_facts(program).invalidate_origin, @r###"
    [
        (
            "'L_x",
            "a",
        ),
        (
            "'L_x",
            "b",
        ),
    ]
    "###);
}
