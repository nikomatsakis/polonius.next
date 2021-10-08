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
            .find(|(origin, _)| origin == "'ref"),
        Some(("'ref".to_string(), "bb0[1]".to_string()))
    );
}

// #[test]
// fn all_origins_in_type_are_cleared_on_assignments() {
//     // TODO
// }
