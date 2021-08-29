mod fact_parser;

use std::{path::Path, process::Command};

pub use fact_parser::generate_facts;

pub fn test_harness(dir_name: &str) -> eyre::Result<()> {
    let path = Path::new(&dir_name);
    let input_path = path.join("program.txt");
    let facts_path = path.join("facts");
    let data = std::fs::read_to_string(input_path)?;

    std::fs::create_dir_all(&facts_path)?;
    generate_facts(&data, &facts_path)?;

    let output_path = path.join("output");
    std::fs::create_dir_all(&output_path)?;

    let _ = Command::new("souffle")
        .args(&[
            "src/polonius.dl".to_string(),
            "-F".to_string(),
            facts_path.display().to_string(),
            "-D".to_string(),
            output_path.display().to_string(),
        ])
        .output()?;

    let dot = Command::new("python3")
        .args(&[
            "graphviz/graphviz.py".to_string(),
            facts_path.display().to_string(),
            output_path.display().to_string(),
        ])
        .output()?;

    std::fs::write(output_path.join("graph.dot"), dot.stdout)?;

    Ok(())
}
