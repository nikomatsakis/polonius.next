mod ast;
mod ast_parser;
mod fact_parser;
mod graphviz;

use std::{path::PathBuf, process::Command};

use eyre::Context;
pub use fact_parser::generate_facts;

pub fn test_harness(dir_name: &str) -> eyre::Result<()> {
    // let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let manifest_dir = PathBuf::from(".");

    let path = manifest_dir.join(&dir_name);
    let input_path = path.join("program.txt");
    let facts_path = path.join("facts");
    let data = std::fs::read_to_string(input_path)?;

    std::fs::create_dir_all(&facts_path)?;
    generate_facts(&data, &facts_path)?;

    let output_path = path.join("output");
    std::fs::create_dir_all(&output_path)?;

    let _ = Command::new("souffle")
        .args(&[
            manifest_dir.join("src/polonius.dl").display().to_string(),
            "-F".to_string(),
            facts_path.display().to_string(),
            "-D".to_string(),
            output_path.display().to_string(),
        ])
        .output()
        .wrap_err("failed to run souffle")?;

    let dot_path = output_path.join("graph.dot");
    graphviz::create_graph(path.as_path(), dot_path.as_path());

    if std::env::var("BLESS").is_ok() {
        let status = Command::new("cp")
            .args(&[
                output_path.join("invalidated_origin_accessed.csv"),
                path.join("invalidated_origin_accessed.csv"),
            ])
            .status()
            .wrap_err("failed to copy blessed output")?;
        if !status.success() {
            eyre::bail!("failed to bless output");
        }
    }

    let status = Command::new("diff")
        .args(&[
            path.join("invalidated_origin_accessed.csv"),
            output_path.join("invalidated_origin_accessed.csv"),
        ])
        .status()
        .wrap_err("failed to run diff")?;

    assert!(status.success());

    Ok(())
}
