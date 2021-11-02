//! Parser for "fact files", a compact way to represent facts.
//!
//! ```notrust
//! Program    := Fact* Statement,
//! Statement  := Ident: String { Fact* goto Ident* }
//! Fact       := Ident ( Symbol, )*
//! Ident      := [a-zA-Z_][a-zA-Z_0-9]*    /* regular expression */
//! Symbol     := Ident | 'Ident
//! String     := "[^"]*"   /* regular expression */
//! ```
use eyre::WrapErr;
use itertools::Itertools;
use std::collections::HashMap;
use std::path::Path;

pub struct Program {
    pub global_facts: Vec<Fact>,
    pub statements: Vec<Statement>,
}

pub struct Statement {
    pub name: String,
    pub text: String,
    pub facts: Vec<Fact>,
    pub successors: Vec<String>,
}

pub struct Fact {
    pub name: String,
    pub arguments: Vec<String>,
}

peg::parser! {
    grammar fact_parser() for str {
        pub rule program() -> Program = comment()* _ g:fact()**__ _ n:statement()**__ _ {
            Program {
                global_facts: g,
                 statements: n
            }
        }

        rule _ = quiet!{[' ' | '\n']*}
        rule __ = quiet!{[' ' | '\n']+}

        rule comment() -> () = _ "//" [^'\n']* "\n" { () }

        rule statement() -> Statement = name:ident() _ ":" _ text:string() _ "{" _ facts:fact()**__ _ "goto" _ successors:ident()**__ _ "}" {
            Statement { name, text, facts, successors }
        }

        rule fact() -> Fact = comment()* _ name:ident() _ "(" _ arguments:symbol()**comma() _ ")" {
            Fact { name, arguments }
        }

        rule comma() -> () =  _ "," _ { () }

        rule symbol() -> String = ident() / string()

        rule ident() -> String = t:$("'"?['a'..='z' | 'A'..='Z' | '_' | '0' ..= '9' | '*' ]+) {
            t.to_string()
        }

        rule string() -> String = ['"'] t:$([^'"']*) ['"'] {
            t.to_string()
        }
    }
}

fn parse_facts(input: &str) -> eyre::Result<Program> {
    Ok(fact_parser::program(input)?)
}

pub fn generate_facts(input: &str, output_path: &Path) -> eyre::Result<()> {
    let program = parse_facts(input).wrap_err("failed to parse input")?;
    let facts = collect_facts(&program)?;

    for (fact_name, fact_rows) in facts.into_iter() {
        let fact_path = output_path.join(fact_name).with_extension("facts");
        let file_contents: String = fact_rows
            .into_iter()
            .map(|fact_row| format!("{}\n", fact_row.iter().format("\t")))
            .collect();
        std::fs::write(&fact_path, file_contents)
            .wrap_err_with(|| format!("failed to write facts to `{}`", fact_path.display()))?;
    }

    Ok(())
}

const EXPECTED_GLOBAL_FACT_NAMES: &[&str] = &["mark_as_loan_origin"];
const EXPECTED_LOCAL_FACT_NAMES: &[&str] = &[
    "access_origin",
    "cfg_edge",
    "clear_origin",
    "introduce_subset",
    "invalidate_origin",
];

/// Maps a program into a set of facts:
fn collect_facts(program: &Program) -> eyre::Result<HashMap<String, Vec<Vec<String>>>> {
    let mut facts = HashMap::new();

    for expected in EXPECTED_GLOBAL_FACT_NAMES
        .iter()
        .chain(EXPECTED_LOCAL_FACT_NAMES.iter())
    {
        facts.insert(expected.to_string(), vec![]);
    }
    facts.insert("node_text".to_string(), vec![]);
    facts.insert("cfg_edge".to_string(), vec![]);

    for global_fact in &program.global_facts {
        if !EXPECTED_GLOBAL_FACT_NAMES.contains(&global_fact.name.as_str()) {
            return Err(eyre::eyre!(
                "unexpected global fact name `{}`, valid names are `{:?}`",
                global_fact.name,
                EXPECTED_GLOBAL_FACT_NAMES
            ));
        }

        facts
            .get_mut(&global_fact.name)
            .unwrap()
            .push(global_fact.arguments.iter().cloned().collect());
    }

    // When a statement S has a fact F(A0, .., An),
    // we insert a mapping F -> [A0, .., An, S] into
    // facts hashmap.
    for statement in &program.statements {
        facts
            .get_mut("node_text")
            .unwrap()
            .push(vec![statement.text.clone(), statement.name.clone()]);

        for successor in &statement.successors {
            facts
                .get_mut("cfg_edge")
                .unwrap()
                .push(vec![statement.name.clone(), successor.clone()]);
        }

        for fact in &statement.facts {
            if !EXPECTED_LOCAL_FACT_NAMES
                .iter()
                .any(|expected| *expected == fact.name)
            {
                return Err(eyre::eyre!(
                    "unexpected fact name `{}`, valid names are `{:?}`",
                    fact.name,
                    EXPECTED_LOCAL_FACT_NAMES
                ));
            }

            facts.get_mut(&fact.name).unwrap().push(
                fact.arguments
                    .iter()
                    .chain(Some(&statement.name))
                    .cloned()
                    .collect(),
            );
        }
    }

    Ok(facts)
}
