use polonius::generate_facts;
use std::path::Path;

fn main() -> eyre::Result<()> {
    for arg in std::env::args().skip(1) {
        // Treat arg as a directory name:
        let path = Path::new(&arg);
        let input_path = path.join("program.txt");
        let facts_path = path.join("facts");
        let data = std::fs::read_to_string(input_path)?;
        std::fs::create_dir_all(&facts_path)?;
        generate_facts(&data, &facts_path)?;
    }
    Ok(())
}
