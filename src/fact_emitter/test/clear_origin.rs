use super::*;

#[test]
fn assignments_to_references() {
    let facts = expect_facts(
        "
        let i: i32;
        let ref: &'ref i32;

        bb0: {
            i = 0;
            ref = &'L_i i;
        }
    ",
    );
    assert_eq!(
        facts
            .clear_origin
            .into_iter()
            .find(|(origin, _)| origin.0 == "'ref"),
        Some(("'ref".into(), "b".into()))
    );
}

#[test]
fn all_origins_in_type_are_cleared_on_assignments_to_references() {
    let facts = expect_facts(
        "
        let v: Vec<&'v i32>;
        let ref: &'ref Vec<&'vec i32>;

        bb0: {
            ref = &'L_v v;
        }
    ",
    );
    assert_debug_snapshot!(facts.clear_origin, @r###"
    [
        (
            "'ref",
            "a",
        ),
        (
            "'vec",
            "a",
        ),
        (
            "'L_v",
            "a",
        ),
    ]
    "###);
}

#[test]
fn all_origins_in_type_are_cleared_on_assignments() {
    let facts = expect_facts(
        "
        let v: Vec<&'v i32>;

        bb0: {
            v = Vec_new();
        }
    ",
    );
    assert_debug_snapshot!(facts.clear_origin, @r###"
    [
        (
            "'v",
            "a",
        ),
    ]
    "###);
}

#[test]
fn shared_borrows_clear_their_origin() {
    let facts = expect_facts(
        "
        let i: i32;
        let ref: &'ref i32;

        bb0: {
            i = 0;
            ref = &'L_i i;
        }
    ",
    );
    assert_eq!(
        facts
            .clear_origin
            .into_iter()
            .find(|(origin, _)| origin.0 == "'L_i"),
        Some(("'L_i".into(), "b".into()))
    );
}

#[test]
fn mut_borrows_clear_their_origin() {
    let facts = expect_facts(
        "
        let i: i32;
        let ref: &'ref mut i32;

        bb0: {
            i = 0;
            ref = &'L_i mut i;
        }
    ",
    );
    assert_eq!(
        facts
            .clear_origin
            .into_iter()
            .find(|(origin, _)| origin.0 == "'L_i"),
        Some(("'L_i".into(), "b".into()))
    );
}
