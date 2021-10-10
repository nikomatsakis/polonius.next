use super::*;
use insta::assert_display_snapshot;

#[test]
// Port of /polonius.next/tests/issue-47680/program.txt
fn issue_47680() {
    let program = "
        let temp: &'temp mut Thing;
        let t0: &'t0 mut Thing;
        let v: &'v mut Thing;

        bb0: {
            temp = &'L_Thing mut Thing;
            goto bb1;
        }

        bb1: {
            t0 = &'L_*temp mut *temp;
            v = MaybeNext(move t0);
            goto bb2, bb3;
        }

        bb2: {
            temp = move v;
            goto bb4;
        }

        bb3: {
            goto bb4;
        }

        bb4: {
            goto bb1;
        }
    ";
    assert_display_snapshot!(expect_facts(program), @r###"
    bb0[0]: {
    	clear_origin('L_Thing)
    	clear_origin('temp)
    	introduce_subset('L_Thing, 'temp)
    	goto bb1[0]
    }

    bb1[0]: {
    	clear_origin('L_*temp)
    	clear_origin('t0)
    	introduce_subset('L_*temp, 't0)
    	goto bb1[1]
    }

    bb1[1]: {
    	access_origin('t0)
    	clear_origin('v)
    	goto bb2[0] bb3[0]
    }

    bb2[0]: {
    	access_origin('v)
    	clear_origin('temp)
    	introduce_subset('v, 'temp)
    	goto bb4[0]
    }

    bb3[0]: {
    	goto bb4[0]
    }

    bb4[0]: {
    	goto bb1[0]
    }
    "###);
}
