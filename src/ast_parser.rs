//! Parser for "fact files", a compact way to represent facts.
//!
//! ```notrust
//! Program    := Statement,
//! Statement  := Ident: String { Fact* goto Ident* }
//! Fact       := Ident ( Symbol, )
//! Ident      := [a-zA-Z_][a-zA-Z_0-9]*    /* regular expression */
//! Symbol     := Ident | 'Ident
//! String     := "[^"]*"   /* regular expression */
//! ```

use eyre::WrapErr;
use itertools::Itertools;
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

use crate::ast;

#[cfg(test)]
mod test;

peg::parser! {
    grammar ast_parser() for str {
        pub rule program() -> ast::Program = (
            _
            variables:var_decl()**__ _
            basic_blocks:basic_block()**__ _
            _
        {
            ast::Program {
                struct_decls: vec![], // s,
                fn_prototypes: vec![], // f,
                variables,
                basic_blocks, // b,
            }
        }
    )

        rule whitespace() -> () = [' ' | '\n']
        rule comment() -> () = "//" [^'\n']* "\n" { () }
        rule skip() -> () = whitespace() / comment()
        rule _ = quiet!{skip()*}
        rule __ = quiet!{skip()+}

        //rule struct_decl() -> StructDecl = "struct" _ name:ident() _ "{" "}"
        //    {StructDecl { name, generic_decls: vec![], field_decls: vec![] }
        //}

        rule var_decl() -> ast::VariableDecl = "let" _ name:ident() _ ":" _ ty:ty() _ ";" {
            ast::VariableDecl { name, ty }
        }

        rule ty() -> ast::Ty = ref_mut_ty() / ref_ty() / i32_ty() / struct_ty()

        rule ref_ty() -> ast::Ty = "&" _ origin:ident() _ ty:ty() {
            ast::Ty::Ref { origin, ty: Box::new(ty) }
        }

        rule ref_mut_ty() -> ast::Ty = "&" _ "mut" _ origin:ident() _ ty:ty() {
            ast::Ty::RefMut { origin, ty: Box::new(ty) }
        }

        rule i32_ty() -> ast::Ty = "i32" {
            ast::Ty::I32
        }

        rule struct_ty() -> ast::Ty = name:ident() parameters:parameters() {
            ast::Ty::Struct { name, parameters }
        }

        rule parameters() -> Vec<ast::Parameter> = (
            "<" _ p:parameter()**comma() _ ">" { p } /
            () { vec![] }
        )

        rule parameter() -> ast::Parameter = (
            o:origin_ident() { ast::Parameter::Origin(o) } /
            t:ty() { ast::Parameter::Ty(t) }
        )

        rule comma() -> () = _ "," _ { }

        rule basic_block() -> ast::BasicBlock = (
            name:ident() _ ":" _ "{" _ statements:statement()**__ _ successors:goto() _ "}" {
                ast::BasicBlock { name, statements, successors }
            }
        )

        rule goto() -> Vec<ast::Name> = (
            "goto" _ names:ident()**comma() _ ";" { names } /
            () { vec![] }
        )

        rule statement() -> ast::Statement = (
            place:place() _ "=" _ expr:expr() _ ";" { ast::Statement::Assign(place, expr) } /
            expr:expr() _ ";" { ast::Statement::Drop(expr) }
        )

        rule expr() -> ast::Expr = (
            kind:access_kind() _ place:place() { ast::Expr::Access { kind, place } } /
            n:$(['0'..='9']+) { ast::Expr::Number { value: i32::from_str(n).unwrap() } } /
            name:ident() _ "(" _ arguments:expr()**comma() _ ")" { ast::Expr::Call { name, arguments} }
        )

        rule place() -> ast::Place = (
            base:ident() _ dot() _ fields:ident()**dot() { ast::Place { base, fields } } /
            base:ident() { ast::Place { base, fields: vec![] } }
        )

        rule access_kind() -> ast::AccessKind = (
            "copy" { ast::AccessKind::Copy } /
            "move" { ast::AccessKind::Move } /
            "&" _ o:origin_ident() _ "mut" { ast::AccessKind::BorrowMut(o) } /
            "&" _ o:origin_ident() { ast::AccessKind::Borrow(o) }
        )

        rule dot() -> () = _ "." _

        rule ident() -> ast::Name = t:$(['a'..='z' | 'A'..='Z' | '_' | '0' ..= '9' | '*' ]+) {
            t.to_string()
        }

        rule origin_ident() -> ast::Name = t:$("'"['a'..='z' | 'A'..='Z' | '_' | '0' ..= '9' | '*' ]+) {
            t.to_string()
        }

    }
}

fn parse_ast(input: &str) -> eyre::Result<ast::Program> {
    Ok(ast_parser::program(input)?)
}
