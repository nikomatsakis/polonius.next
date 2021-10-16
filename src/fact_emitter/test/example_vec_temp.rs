use super::*;
use insta::assert_display_snapshot;

#[test]
// Port of /polonius.next/tests/vec-temp/program.txt
fn vec_temp() {
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
            Vec_len(copy v);
        }
    ";
    assert_display_snapshot!(expect_facts(program), @r###"
    bb0[0]: "x = 22" {
    	invalidate_origin('L_x)
    	goto bb0[1]
    }

    bb0[1]: "v = Vec_new()" {
    	invalidate_origin('L_v)
    	clear_origin('v)
    	goto bb0[2]
    }

    bb0[2]: "p = &'L_x x" {
    	clear_origin('p)
    	clear_origin('L_x)
    	introduce_subset('L_x, 'p)
    	goto bb0[3]
    }

    bb0[3]: "tmp = &'L_v mut v" {
    	access_origin('v)
    	invalidate_origin('L_v)
    	clear_origin('tmp0)
    	clear_origin('tmp1)
    	clear_origin('L_v)
    	introduce_subset('L_v, 'tmp0)
    	goto bb0[4]
    }

    bb0[4]: "Vec_push(move tmp, move p)" {
    	access_origin('tmp0)
    	access_origin('tmp1)
    	access_origin('p)
    	goto bb0[5]
    }

    bb0[5]: "x = 23" {
    	invalidate_origin('L_x)
    	goto bb0[6]
    }

    bb0[6]: "Vec_len(copy v)" {
    	access_origin('v)
    	goto
    }
    "###);
}
