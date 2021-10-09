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

// Returns the type of the given place's path in the given program.
fn ty_of_place(program: &str, path: &str) -> ast::Ty {
    let program = parse_ast(program).expect("Unexpected parsing error");
    let emitter = FactEmitter { program };

    let mut path: Vec<_> = path.split('.').map(ToString::to_string).collect();
    let base = path.remove(0);
    let place = ast::Place { base, fields: path };

    emitter.ty_of_place(&place)
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

#[test]
fn example_vec_temp() {
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

#[test]
fn example_issue_47680() {
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
        introduce_subset: [],
        invalidate_origin: [],
    }
    "###);
}
