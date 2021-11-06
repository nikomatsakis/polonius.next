//! Tests dedicated to specific relations
mod access_origin;
mod cfg_edge;
mod clear_origin;
mod introduce_subset;
mod invalidate_origin;

use super::*;
use crate::ast_parser::test::expect_parse;
use crate::ast_parser as parse;
use insta::assert_debug_snapshot;

pub(crate) fn expect_facts(input: &str) -> Facts {
    let program = expect_parse(input);
    let emitter = FactEmitter::new(program, input, true);
    let mut facts = Default::default();
    emitter.emit_facts(&mut facts);
    facts
}

fn create_emitter(input: &str) -> FactEmitter {
    let program = expect_parse(input);
    FactEmitter::new(program, input, true)
}

// Returns the type of the given place's path in the given program.
fn find_ty(program: &str, path: &str) -> Ty {
    let emitter = create_emitter(program);
    let place = parse::place(path).expect("Invalid place");
    emitter.ty_of_place(&place).clone()
}

// Returns the origins present in the type of the given place's path in the given program.
fn find_origins(program: &str, path: &str) -> Vec<Origin> {
    let emitter = create_emitter(program);
    let place = parse::place(path).expect("Invalid place");
    emitter.origins_of_place(&place)
}

#[test]
fn type_of_vars() {
    // type
    assert_eq!(find_ty("let x: i32;", "x"), Ty::I32);

    // struct
    let program = "
        struct Vec { e: i32 }
        let v: Vec;
    ";
    assert_debug_snapshot!(find_ty(program, "v"), @r###"
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
    assert_debug_snapshot!(find_ty(program, "v"), @r###"
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
    assert_debug_snapshot!(find_ty(program, "r"), @r###"
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
    // assert_debug_snapshot!(find_ty(program, "r.ref"), @"");

    // TODO ?
    // // generic struct: origins and types, and derefs
    // let program = "
    //     struct Vec<T> { e: T }
    //     struct Ref<'a, T> { ref: &'a T }
    //     let r: Ref<'r, Vec<i32>>;
    // ";
    // assert_eq!(find_ty(program, "(*r.ref).e"), Ty::I32);
}

#[test]
fn type_of_fields() {
    let program = "
        struct Vec { e: i32 }
        let v: Vec;
    ";
    assert_eq!(find_ty(program, "v.e"), Ty::I32);

    let program = "
        struct A { b: B }
        struct B { c: C }
        struct C { d: &'d i32 }
        let a: A;
    ";
    assert_debug_snapshot!(find_ty(program, "a.b.c.d"), @r###"
    Ref {
        origin: "'d",
        ty: I32,
    }
    "###);

    let program = "
        struct A<T> { b: T }
        let a: A<i32>;
    ";
    assert_eq!(find_ty(program, "a.b"), Ty::I32);

    let program = "
        struct A<T> { b: T }
        struct B<T> { c: T }
        struct C<T> { d: T }
        let a: A<B<C<i32>>>;
    ";
    assert_eq!(find_ty(program, "a.b.c.d"), Ty::I32);
}

#[test]
fn origins_in_ty() {
    assert_eq!(find_origins("let a: i32;", "a"), []);
    assert_eq!(find_origins("let b: &'b i32;", "b"), [Origin::from("'b")]);
    assert_eq!(
        find_origins("let c: &'c &'b i32;", "c"),
        [Origin::from("'c"), Origin::from("'b")]
    );
    assert_eq!(
        find_origins("let d: Vec<&'d i32>;", "d"),
        [Origin::from("'d")]
    );
    assert_eq!(
        find_origins("let e: Vec<&'e Vec<&'d i32>>;", "e"),
        [Origin::from("'e"), Origin::from("'d")]
    );
    assert_eq!(
        find_origins("let f: &'f Vec<&'e Vec<&'d i32>>;", "f"),
        [Origin::from("'f"), Origin::from("'e"), Origin::from("'d")]
    );
}
