use super::*;
use insta::assert_debug_snapshot;

#[test]
fn no_edges() {
    // Empty block
    assert!(expect_facts("bb0: {}").cfg_edge.is_empty());

    // Single statement
    let program = "
        let x: i32;
        bb0: {
            x = 1;
        }
    ";
    assert!(expect_facts(program).cfg_edge.is_empty());
}

#[test]
fn single_block_edges() {
    let program = "
        let x: i32;
        bb0: {
            x = 1;
            x = 2;
        }
    ";
    assert_debug_snapshot!(expect_facts(program).cfg_edge, @r###"
    [
        (
            "bb0[0]",
            "bb0[1]",
        ),
    ]
    "###);
}

#[test]
fn single_sucessor_block() {
    let program = "
        bb0: {
            goto bb1;
        }

        bb1: {}
    ";
    assert_debug_snapshot!(expect_facts(program).cfg_edge, @r###"
    [
        (
            "bb0[0]",
            "bb1[0]",
        ),
    ]
    "###);
}

#[test]
fn multiple_successor_blocks() {
    let program = "
        bb0: {
            goto bb1, bb2;
        }

        bb1: {}
        bb2: {}
    ";
    assert_debug_snapshot!(expect_facts(program).cfg_edge, @r###"
    [
        (
            "bb0[0]",
            "bb1[0]",
        ),
        (
            "bb0[0]",
            "bb2[0]",
        ),
    ]
    "###);
}
