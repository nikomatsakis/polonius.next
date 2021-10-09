use super::*;
use insta::assert_debug_snapshot;

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
    assert_debug_snapshot!(expect_facts(program), @r###"
    Facts {
        access_origin: [
            (
                "'t0",
                "bb1[1]",
            ),
            (
                "'v",
                "bb2[0]",
            ),
        ],
        cfg_edge: [
            (
                "bb0[0]",
                "bb1[0]",
            ),
            (
                "bb1[0]",
                "bb1[1]",
            ),
            (
                "bb1[1]",
                "bb2[0]",
            ),
            (
                "bb1[1]",
                "bb3[0]",
            ),
            (
                "bb2[0]",
                "bb4[0]",
            ),
            (
                "bb3[0]",
                "bb4[0]",
            ),
            (
                "bb4[0]",
                "bb1[0]",
            ),
        ],
        clear_origin: [
            (
                "'L_Thing",
                "bb0[0]",
            ),
            (
                "'temp",
                "bb0[0]",
            ),
            (
                "'L_*temp",
                "bb1[0]",
            ),
            (
                "'t0",
                "bb1[0]",
            ),
            (
                "'v",
                "bb1[1]",
            ),
            (
                "'temp",
                "bb2[0]",
            ),
        ],
        introduce_subset: [
            (
                "'L_Thing",
                "'temp",
                "bb0[0]",
            ),
            (
                "'L_*temp",
                "'t0",
                "bb1[0]",
            ),
            (
                "'v",
                "'temp",
                "bb2[0]",
            ),
        ],
        invalidate_origin: [],
    }
    "###);
}
