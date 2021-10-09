use super::*;
use insta::assert_debug_snapshot;

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
            x = 44;
            Vec_len(copy v);
        }
    ";
    assert_debug_snapshot!(expect_facts(program), @r###"
    Facts {
        access_origin: [
            (
                "'tmp0",
                "bb0[4]",
            ),
            (
                "'p",
                "bb0[4]",
            ),
        ],
        cfg_edge: [
            (
                "bb0[0]",
                "bb0[1]",
            ),
            (
                "bb0[1]",
                "bb0[2]",
            ),
            (
                "bb0[2]",
                "bb0[3]",
            ),
            (
                "bb0[3]",
                "bb0[4]",
            ),
            (
                "bb0[4]",
                "bb0[5]",
            ),
            (
                "bb0[5]",
                "bb0[6]",
            ),
        ],
        clear_origin: [
            (
                "'L_x",
                "bb0[2]",
            ),
            (
                "'p",
                "bb0[2]",
            ),
            (
                "'L_v",
                "bb0[3]",
            ),
            (
                "'tmp0",
                "bb0[3]",
            ),
        ],
        introduce_subset: [],
        invalidate_origin: [
            (
                "'L_x",
                "bb0[0]",
            ),
            (
                "'L_v",
                "bb0[1]",
            ),
            (
                "'L_x",
                "bb0[5]",
            ),
        ],
    }
    "###);
}
