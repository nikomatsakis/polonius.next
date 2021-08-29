mod fact_parser;

use std::path::Path;

pub use fact_parser::generate_facts;

pub fn test_harness(dir_name: &str) -> eyre::Result<()> {
    let path = Path::new(&dir_name);
    let input_path = path.join("program.txt");
    let facts_path = path.join("facts");
    let data = std::fs::read_to_string(input_path)?;
    std::fs::create_dir_all(&facts_path)?;
    generate_facts(&data, &facts_path)?;
    Ok(())
}
