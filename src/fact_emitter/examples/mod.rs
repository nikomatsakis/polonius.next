//! Tests porting the existing examples using the manual fact format, to the new frontend format

mod canonical_liveness;
mod issue_47680;
mod vec_temp;

use super::test::*;
use insta::assert_display_snapshot;

#[test]
// Port of /polonius.next/tests/example-a/program.txt
fn example_a() {
    let program = "
        let x: i32;
        let y: &'y i32;

        bb0: {
            x = 3;
            y = &'L_x x;
            x = 4;
            use(move y);
        }
    ";

    assert_display_snapshot!(expect_facts(program), @r###"
    a: "x = 3" {
        invalidate_origin('L_x)
        goto b
    }

    b: "y = &'L_x x" {
        clear_origin('y)
        clear_origin('L_x)
        introduce_subset('L_x, 'y)
        goto c
    }

    c: "x = 4" {
        invalidate_origin('L_x)
        goto d
    }

    d: "use(move y)" {
        access_origin('y)
        goto
    }
    "###);
}
