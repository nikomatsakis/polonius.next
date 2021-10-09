use super::*;
use insta::assert_debug_snapshot;

#[test]
fn simple_borrow_assignment() {
    let facts = expect_facts(
        "
        let a: &'a i32;
        let b: i32;

        bb0: {
            a = &'b b;
        }
    ",
    );
    assert_debug_snapshot!(facts.introduce_subset, @r###"
    [
        (
            "'b",
            "'a",
            "bb0[0]",
        ),
    ]
    "###);
}

#[test]
fn simple_ref_assignment() {
    let facts = expect_facts(
        "
        let a: &'a i32;
        let b: &'b i32;

        bb0: {
            a = move b;
        }
    ",
    );
    assert_debug_snapshot!(facts.introduce_subset, @r###"
    [
        (
            "'b",
            "'a",
            "bb0[0]",
        ),
    ]
    "###);
}
