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

    let facts = expect_facts(
        "
        let u: &'u Vec<&'data_u i32>;
        let v: &'v Vec<&'data_v i32>;

        bb0: {
            u = move v;
        }
    ",
    );
    assert_debug_snapshot!(facts.access_origin, @r###"
    [
        (
            "'v",
            "bb0[0]",
        ),
        (
            "'data_v",
            "bb0[0]",
        ),
    ]
    "###);
}

#[test]
fn function_calls_read_ref_arguments() {
    let facts = expect_facts(
        "
        let ref: &'ref i32;

        bb0: {
            call(move ref);
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

    let facts = expect_facts(
        "
        let v: &'v Vec<&'data i32>;

        bb0: {
            Vec_len(move v);
        }
    ",
    );
    assert_debug_snapshot!(facts.access_origin, @r###"
    [
        (
            "'v",
            "bb0[0]",
        ),
        (
            "'data",
            "bb0[0]",
        ),
    ]
    "###);
}
