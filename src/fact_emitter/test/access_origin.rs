use super::*;
use insta::assert_debug_snapshot;

#[test]
fn assignments_read_rhs() {
    // ref
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
            "a",
        ),
    ]
    "###);

    // type with origins
    let facts = expect_facts(
        "
        let u: Vec<&'u i32>;
        let v: Vec<&'v i32>;

        bb0: {
            u = move v;
        }
    ",
    );
    assert_debug_snapshot!(facts.access_origin, @r###"
    [
        (
            "'v",
            "a",
        ),
    ]
    "###);

    // ref of type with origins
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
            "a",
        ),
        (
            "'data_v",
            "a",
        ),
    ]
    "###);
}

#[test]
fn function_calls_read_arguments() {
    // ref
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
            "a",
        ),
    ]
    "###);

    // type with origins
    let facts = expect_facts(
        "
        let v: Vec<&'v i32>;

        bb0: {
            Vec_len(move v);
        }
    ",
    );
    assert_debug_snapshot!(facts.access_origin, @r###"
    [
        (
            "'v",
            "a",
        ),
    ]
    "###);

    // ref of type with origins
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
            "a",
        ),
        (
            "'data",
            "a",
        ),
    ]
    "###);
}

#[test]
fn mutable_borrows_are_considered_writes() {
    // mutable ref of type with origins
    let facts = expect_facts(
        "
        let v: Vec<&'v i32>;
        let ref: &'ref mut Vec<&'data i32>;

        bb0: {
            ref = &'L_v mut v;
        }
    ",
    );
    assert_debug_snapshot!(facts.access_origin, @r###"
    [
        (
            "'v",
            "a",
        ),
    ]
    "###);
}
