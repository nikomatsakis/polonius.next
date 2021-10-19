use super::*;
use insta::assert_display_snapshot;

#[test]
// Port of /polonius.next/tests/vec-temp/program.txt
fn vec_temp() {
    // FIXME: while having `tmp` hold mutable references is interesting to test variance,
    // and is faithful to the manual test format example, it's not what the comments at the
    // top of the example are expressing. Clean this up into two separate examples.
    // (Also: clean up the `Vec::len` call in that example, since this function 
    // takes a reference and not `v`)
    let program = "
        let x: i32;
        let v: Vec<&'v mut i32>;
        let p: &'p i32;
        let tmp: &'tmp0 mut Vec<&'tmp1 mut i32>;

        bb0: {
            x = 22;
            v = Vec_new();
            p = &'L_x x;
            tmp = &'L_v mut v;
            Vec_push(move tmp, move p);
            x = 23;
            Vec_len(move v);
        }
    ";

    // Notes about the current output:
    // - node e: missing subset between the call's arguments, the fn signatures lack lifetime bounds

    assert_display_snapshot!(expect_facts(program), @r###"
    a: "x = 22" {
    	invalidate_origin('L_x)
    	goto b
    }

    b: "v = Vec_new()" {
    	invalidate_origin('L_v)
    	clear_origin('v)
    	goto c
    }

    c: "p = &'L_x x" {
    	clear_origin('p)
    	clear_origin('L_x)
    	introduce_subset('L_x, 'p)
    	goto d
    }

    d: "tmp = &'L_v mut v" {
    	access_origin('v)
    	invalidate_origin('L_v)
    	clear_origin('tmp0)
    	clear_origin('tmp1)
    	clear_origin('L_v)
    	introduce_subset('L_v, 'tmp0)
    	introduce_subset('v, 'tmp1)
    	introduce_subset('tmp1, 'v)
    	goto e
    }

    e: "Vec_push(move tmp, move p)" {
    	access_origin('tmp0)
    	access_origin('tmp1)
    	access_origin('p)
    	goto f
    }

    f: "x = 23" {
    	invalidate_origin('L_x)
    	goto g
    }

    g: "Vec_len(move v)" {
    	access_origin('v)
    	goto
    }
    "###);
}
