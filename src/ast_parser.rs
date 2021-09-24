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

// use eyre::WrapErr;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::path::Path;

// use crate::ast;

// peg::parser! {
//     grammar ast_parser() for str {
//         pub rule program() -> ast::Program = (
//             _
//             //n:struct_decl()**__ f:fn_prototype()**__
//             v:var_decl()**__
//             // b:basic_block()**__
//             _
//         ) {
//             ast::Program {
//                 struct_decls: s,
//                 fn_prototypes: f,
//                 variables: v,
//                 basic_blocks: b,
//             }
//         }

//         rule whitespace() -> () = [' ' | '\n']
//         rule comment() -> () = "//" [^'\n']* "\n" { () }
//         rule skip() -> () = whitespace() / comment()
//         rule _ = quiet!{skip()*}
//         rule __ = quiet!{skip()+}

//         //rule struct_decl() -> StructDecl = "struct" _ name:ident() _ "{" "}"
//         //    {StructDecl { name, generic_decls: vec![], field_decls: vec![] }
//         //}

//         rule var_decl() -> VariableDecl = "let" _ name:ident() _ "=" _ ty:ty() _ ";" {
//             VariableDecl { name, ty }
//         }

//         rule ident() -> String = [a-zA-Z_][a-zA-Z_0-9]* {
//             VariableDecl { name, ty }
//         }
//     }
// }
