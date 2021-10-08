mod access_origin;
mod cfg_edge;
mod clear_origin;
mod introduce_subset;
mod invalidate_origin;

use super::*;
use insta::assert_debug_snapshot;

fn expect_facts(program: &str) -> Facts {
    match emit_facts(program) {
        Ok(facts) => facts,
        Err(e) => {
            panic!("Couldn't emit facts because of error: {}", e);
        }
    }
}

#[test]
fn type_of_vars() {
    // type
    assert_eq!(ty_of_place("let x: i32;", "x"), ast::Ty::I32);

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
        struct Vec<T> { e: T }
        struct Ref<'a, T> { ref: &'a T }
        let r: Ref<&'r Vec<i32>>;
    ";
    assert_debug_snapshot!(ty_of_place(program, "r"), @r###"
    Struct {
        name: "Ref",
        parameters: [
            Ty(
                Ref {
                    origin: "'r",
                    ty: Struct {
                        name: "Vec",
                        parameters: [
                            Ty(
                                I32,
                            ),
                        ],
                    },
                },
            ),
        ],
    }
    "###);
}

#[test]
fn type_of_fields() {
    let program = "
        struct Vec { e: i32 }
        let v: Vec;
    ";
    assert_eq!(ty_of_place(program, "v.e"), ast::Ty::I32);

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
    assert_eq!(ty_of_place(program, "a.b"), ast::Ty::I32);

    let program = "
        struct A<T> { b: T }
        struct B<T> { c: T }
        struct C<T> { d: T }
        let a: A<B<C<i32>>>;
    ";
    assert_eq!(ty_of_place(program, "a.b.c.d"), ast::Ty::I32);
}

// Returns the type of the given place's path in the given program.
fn ty_of_place(program: &str, path: &str) -> ast::Ty {
    let program = parse_ast(program).expect("Unexpected parsing error");
    let emitter = FactEmitter { program };

    let mut path: Vec<_> = path.split('.').map(ToString::to_string).collect();
    let base = path.remove(0);
    let place = ast::Place { base, fields: path };

    emitter.ty_of_place(&place)
}