use super::*;
use insta::assert_debug_snapshot;

#[test]
fn assignments_read_ref_arguments() {
    let facts = expect_facts(
        "
        let a: &'a i32;
        let ref: &'ref i32;

        bb0: {
            a = copy ref;
        }
    ",
    );
    assert_debug_snapshot!(facts.access_origin, @r###"
    [
        (
            "'ref",
            "bb0[0]",
        ),
    ]
    "###);
}

#[test]
fn function_calls_read_ref_arguments() {
    let facts = expect_facts(
        "
        let i: i32;
        let ref: &'ref i32;

        bb0: {
            i = call(move ref);
        }
    ",
    );
    assert_debug_snapshot!(facts.access_origin, @r###"
    [
        (
            "'ref",
            "bb0[0]",
        ),
    ]
    "###);
}
