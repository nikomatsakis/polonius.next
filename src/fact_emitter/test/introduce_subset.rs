use super::*;
use insta::assert_debug_snapshot;

#[test]
fn shared_reference_assignment() {
    let facts = expect_facts(
        "
        let a: &'a i32;
        let b: i32;
        let c: &'c i32;

        bb0: {
            a = &'b b;
            a = move c;
        }
    ",
    );
    assert_debug_snapshot!(facts.introduce_subset, @r###"
    [
        (
            "'b",
            "'a",
            "a",
        ),
        (
            "'c",
            "'a",
            "b",
        ),
    ]
    "###);
}

#[test]
fn unique_reference_assignment() {
    let facts = expect_facts(
        "
        let a: &'a mut i32;
        let b: i32;
        let c: &'c mut i32;

        bb0: {
            a = &'b mut b;
            a = move c;
        }
    ",
    );
    assert_debug_snapshot!(facts.introduce_subset, @r###"
    [
        (
            "'b",
            "'a",
            "a",
        ),
        (
            "'c",
            "'a",
            "b",
        ),
    ]
    "###);
}

#[test]
fn shared_reference_to_generic_type() {
    // &T references are covariant in T
    let program = "
        // Refs to simple generic types
        let a: &'a Vec<&'c i32>;
        let b:     Vec<&'d i32>;

        // Refs requiring generic arg traversal
        let e: &'e Vec<Vec<Vec<Vec<Vec<&'g i32>>>>>;
        let f:     Vec<Vec<Vec<Vec<Vec<&'h i32>>>>>;

        bb0: {
            a = &'b b;
            e = &'f f;
        }
    ";
    assert_debug_snapshot!(expect_facts(program).introduce_subset, @r###"
    [
        (
            "'b",
            "'a",
            "a",
        ),
        (
            "'d",
            "'c",
            "a",
        ),
        (
            "'f",
            "'e",
            "b",
        ),
        (
            "'h",
            "'g",
            "b",
        ),
    ]
    "###);
}

#[test]
fn unique_reference_to_generic_type() {
    // &mut T references are invariant in T
    let program = "
        // Refs to simple generic types
        let a: &'a mut Vec<&'c i32>;
        let b:         Vec<&'d i32>;

        // Refs requiring generic arg traversal
        let e: &'e mut Vec<Vec<Vec<Vec<Vec<&'g i32>>>>>;
        let f:         Vec<Vec<Vec<Vec<Vec<&'h i32>>>>>;

        bb0: {
            a = &'b mut b;
            e = &'f mut f;
        }
    ";
    assert_debug_snapshot!(expect_facts(program).introduce_subset, @r###"
    [
        (
            "'b",
            "'a",
            "a",
        ),
        (
            "'d",
            "'c",
            "a",
        ),
        (
            "'c",
            "'d",
            "a",
        ),
        (
            "'f",
            "'e",
            "b",
        ),
        (
            "'h",
            "'g",
            "b",
        ),
        (
            "'g",
            "'h",
            "b",
        ),
    ]
    "###);
}

#[test]
fn chain_of_mixed_references_to_generic_type() {
    // &mut T references make the next recursive pairs invariant regardless of their immediate
    // parent's ty relationship.
    let program = "
        let a: &'a Vec<&'c mut Vec<&'e Vec<&'g i32>>>;
        let b:     Vec<&'d mut Vec<&'f Vec<&'h i32>>>;

        bb0: {
            a = &'b b;
        }
    ";
    assert_debug_snapshot!(expect_facts(program).introduce_subset, @r###"
    [
        (
            "'b",
            "'a",
            "a",
        ),
        (
            "'d",
            "'c",
            "a",
        ),
        (
            "'f",
            "'e",
            "a",
        ),
        (
            "'e",
            "'f",
            "a",
        ),
        (
            "'h",
            "'g",
            "a",
        ),
        (
            "'g",
            "'h",
            "a",
        ),
    ]
    "###);
}

#[test]
fn values_of_generic_types() {
    let program = "
        let a: Vec<&'a i32>;
        let b: Vec<&'b i32>;
        bb0: {
            a = move b;
        }
    ";
    assert_debug_snapshot!(expect_facts(program).introduce_subset, @r###"
    [
        (
            "'b",
            "'a",
            "a",
        ),
    ]
    "###);
}
