use super::*;
use insta::assert_display_snapshot;

#[test]
// Port of /polonius.next/tests/canonical-liveness/program.txt
fn canonical_liveness() {
    let program = "
        let p: i32;
        let q: i32;
        let x: &'x i32;

        bb0: {
            p = 22;
            q = 44;
            x = &'L_p p;
            x = &'L_q q;
            p = 33; // OK, `x` no longer borrows `p`
            use(move x);
        }
    ";
    assert_display_snapshot!(expect_facts(program), @r###"
    a: "p = 22" {
        invalidate_origin('L_p)
        goto b
    }

    b: "q = 44" {
        invalidate_origin('L_q)
        goto c
    }

    c: "x = &'L_p p" {
        clear_origin('x)
        clear_origin('L_p)
        introduce_subset('L_p, 'x)
        goto d
    }

    d: "x = &'L_q q" {
        clear_origin('x)
        clear_origin('L_q)
        introduce_subset('L_q, 'x)
        goto e
    }

    e: "p = 33" {
        invalidate_origin('L_p)
        goto f
    }

    f: "use(move x)" {
        access_origin('x)
        goto
    }
    "###);
}

#[test]
// Port of /polonius.next/tests/canonical-liveness-err/program.txt
fn canonical_liveness_err() {
    let program = "
        let p: i32;
        let x: &'x i32;

        bb0: {
            p = 22;
            x = &'L_p p;
            p = 33; // invalidates loans `p`
            use(move x); // ERROR
        }
    ";
    assert_display_snapshot!(expect_facts(program), @r###"
    a: "p = 22" {
        invalidate_origin('L_p)
        goto b
    }

    b: "x = &'L_p p" {
        clear_origin('x)
        clear_origin('L_p)
        introduce_subset('L_p, 'x)
        goto c
    }

    c: "p = 33" {
        invalidate_origin('L_p)
        goto d
    }

    d: "use(move x)" {
        access_origin('x)
        goto
    }
    "###);
}
