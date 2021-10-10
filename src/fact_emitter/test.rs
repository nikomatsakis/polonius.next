// Tests dedicated to specific relations
mod access_origin;
mod cfg_edge;
mod clear_origin;
mod introduce_subset;
mod invalidate_origin;

// Tests porting the existing examples using the manual fact format, to the new frontend format
mod example_issue_47680;
mod example_vec_temp;

use super::*;
use insta::{assert_debug_snapshot, assert_display_snapshot};

fn expect_facts(program: &str) -> Facts {
    match emit_facts(program) {
        Ok(facts) => facts,
        Err(e) => {
            panic!("Couldn't emit facts because of error: {}", e);
        }
    }
}

// Returns the type of the given place's path in the given program.
fn ty_of_place(program: &str, path: &str) -> Ty {
    let program = parse_ast(program).expect("Unexpected parsing error");
    let emitter = FactEmitter::new(program);

    let mut path: Vec<_> = path.split('.').map(ToString::to_string).collect();
    let base = path.remove(0);
    let place = Place { base, fields: path };

    emitter.ty_of_place(&place)
}

#[test]
fn type_of_vars() {
    // type
    assert_eq!(ty_of_place("let x: i32;", "x"), Ty::I32);

    // struct
    let program = "
        struct Vec { e: i32 }
        let v: Vec;
    ";
    assert_debug_snapshot!(ty_of_place(program, "v"), @r###"
    Struct {
        name: "Vec",
        parameters: [],
    }
    "###);

    // generic struct: tys
    let program = "
        struct Vec<T> { e: T }
        let v: Vec<i32>;
    ";
    assert_debug_snapshot!(ty_of_place(program, "v"), @r###"
    Struct {
        name: "Vec",
        parameters: [
            Ty(
                I32,
            ),
        ],
    }
    "###);

    // generic struct: origins
    let program = "
        struct Ref<'ref> { e: &'ref i32 }
        let r: Ref<'static>;
    ";
    assert_debug_snapshot!(ty_of_place(program, "r"), @r###"
    Struct {
        name: "Ref",
        parameters: [
            Origin(
                "'static",
            ),
        ],
    }
    "###);

    // TODO ?
    // // generic struct: origins and types
    // let program = "
    //     struct Vec<T> { e: T }
    //     struct Ref<'a, T> { ref: &'a T }
    //     let r: Ref<'r, Vec<i32>>;
    // ";
    // assert_debug_snapshot!(ty_of_place(program, "r.ref"), @"");

    // TODO ?
    // // generic struct: origins and types, and derefs
    // let program = "
    //     struct Vec<T> { e: T }
    //     struct Ref<'a, T> { ref: &'a T }
    //     let r: Ref<'r, Vec<i32>>;
    // ";
    // assert_eq!(ty_of_place(program, "r.ref.e"), Ty::I32);
}

#[test]
fn type_of_fields() {
    let program = "
        struct Vec { e: i32 }
        let v: Vec;
    ";
    assert_eq!(ty_of_place(program, "v.e"), Ty::I32);

    let program = "
        struct A { b: B }
        struct B { c: C }
        struct C { d: &'d i32 }
        let a: A;
    ";
    assert_debug_snapshot!(ty_of_place(program, "a.b.c.d"), @r###"
    Ref {
        origin: "'d",
        ty: I32,
    }
    "###);

    let program = "
        struct A<T> { b: T }
        let a: A<i32>;
    ";
    assert_eq!(ty_of_place(program, "a.b"), Ty::I32);

    let program = "
        struct A<T> { b: T }
        struct B<T> { c: T }
        struct C<T> { d: T }
        let a: A<B<C<i32>>>;
    ";
    assert_eq!(ty_of_place(program, "a.b.c.d"), Ty::I32);
}

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
    bb0[0]: {
    	invalidate_origin('L_x)
    	goto bb0[1]
    }

    bb0[1]: {
    	clear_origin('L_x)
    	clear_origin('y)
    	introduce_subset('L_x, 'y)
    	goto bb0[2]
    }

    bb0[2]: {
    	invalidate_origin('L_x)
    	goto bb0[3]
    }

    bb0[3]: {
    	access_origin('y)
    }
    "###);
}
