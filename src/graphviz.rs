use glob::glob;
use html_escape;
use itertools::Itertools;
use std::{collections::HashMap, fs, io::Write, path::Path, process::Command};
const IMPORTANT_RELATIONS: &[&str] = &["invalidated_origin_accessed"];

#[derive(Debug, Default)]
struct Data {
    pub(crate) node_texts: HashMap<String, String>,
    pub(crate) input_per_node: HashMap<String, Vec<(String, Importance)>>,
    pub(crate) node_predecessors: HashMap<String, Vec<String>>,
    pub(crate) output_per_node: HashMap<String, Vec<(String, Importance)>>,
}

impl Data {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Importance {
    High,
    Low,
}

impl Default for Importance {
    fn default() -> Self {
        Self::Low
    }
}

impl Importance {
    fn style(&self) -> &'static str {
        match self {
            Self::High => r#" bgcolor= "yellow""#,
            Self::Low => "",
        }
    }
}

impl From<bool> for Importance {
    fn from(is_important: bool) -> Self {
        match is_important {
            true => Self::High,
            false => Self::Low,
        }
    }
}

pub(crate) fn create_graph(fact_directory: &Path, output_file_path: &Path) {
    // Resolve name-only output paths
    let output_file_path = if output_file_path.components().count() == 1 {
        fact_directory.join(output_file_path)
    } else {
        output_file_path.to_path_buf()
    };
    let input_facts_directory = fact_directory.join("facts");
    let output_facts_directory = fact_directory.join("output");

    // Process input facts: load fact files from the provided input facts directory, and store the
    // atoms (without locations) in the files as facts at each node in the CFG
    let mut data = Data::new();
    let pattern = input_facts_directory.join("*.facts");
    for path in glob(pattern.to_str().expect("fact path was not UTF-8"))
        .unwrap()
        .filter_map(Result::ok)
    {
        let relation = path.file_stem().unwrap().to_str().unwrap();
        let facts = fs::read_to_string(&path).expect(&format!(
            "could not read relation file '{}'",
            path.to_string_lossy()
        ));

        // Except `cfg_edge`, all input relations have the node location as the last atom
        for line in facts.lines() {
            let mut atoms = line.split('\t');
            match relation {
                "node_text" => {
                    // The text to summarize each node
                    let node = atoms.next_back().unwrap();
                    let text = atoms.next().unwrap();
                    // To be displayed, escape the node text so that ticks and ampersands show up
                    let text = format!("{}: {}", node, text);
                    data.node_texts.insert(
                        node.to_string(),
                        html_escape::encode_text(&text).to_string(),
                    );
                }
                "cfg_edge" => {
                    // The edges in the CFG to transform into graphviz edges
                    let p = atoms.next().unwrap();
                    let q = atoms.next().unwrap();

                    if !data.node_predecessors.contains_key(q) {
                        data.node_predecessors.insert(q.to_string(), Vec::new());
                    }

                    data.node_predecessors
                        .get_mut(q)
                        .unwrap()
                        .push(p.to_string());
                }
                _ => {
                    // Actual facts happening at the node
                    let node = atoms.next_back().unwrap();
                    if !data.input_per_node.contains_key(node) {
                        data.input_per_node.insert(node.to_string(), Vec::new());
                    }
                    let pretty_atoms: String = Itertools::intersperse(atoms, ", ").collect();
                    data.input_per_node.get_mut(node).unwrap().push((
                        format!("{}({})", relation, pretty_atoms),
                        IMPORTANT_RELATIONS.contains(&relation).into(),
                    ));
                }
            }
        }
    }

    // Process output facts, in a similar fashion as the input facts: the relations are also
    // suffixed by the node.
    let pattern = output_facts_directory.join("*.csv");
    for path in glob(pattern.to_str().expect("output path was not UTF-8"))
        .unwrap()
        .filter_map(Result::ok)
    {
        let relation = path.file_stem().unwrap().to_str().unwrap();
        let facts = fs::read_to_string(&path).expect(&format!(
            "could not read relation file '{}'",
            path.to_string_lossy()
        ));

        for line in facts.lines() {
            let mut atoms = line.split('\t');
            let node = atoms.next_back().unwrap();
            if !data.output_per_node.contains_key(node) {
                data.output_per_node.insert(node.to_string(), Vec::new());
            }
            let pretty_atoms: String = Itertools::intersperse(atoms, ", ").collect();
            data.output_per_node.get_mut(node).unwrap().push((
                format!("{}({})", relation, pretty_atoms),
                IMPORTANT_RELATIONS.contains(&relation).into(),
            ));
        }
    }

    // Output the graphviz file.
    // First, the header.
    let mut output_dot = r#"digraph G {
    rankdir = "TD"
    node [ shape = "rectangle" ]
    "#
    .to_string();
    let no_input_facts = Vec::new();
    for node in data.node_texts.keys().sorted() {
        let input_facts = data.input_per_node.get(node).unwrap_or(&no_input_facts);
        let node_text = &data.node_texts[node];

        // Then the body: the graph nodes, formatted as
        // - the node header setting up the table with facts as rows
        // - the node text
        // - a row per input fact
        // - if output facts exists, a separator, then a row per output fact
        // - edges from the predecessors to the node, if any
        let mut rows: Vec<_> = input_facts
            .into_iter()
            .sorted()
            .map(|(fact, importance)| {
                format!(r#"    <tr><td{}>{}</td></tr>"#, importance.style(), fact)
            })
            .collect();
        if data.output_per_node.contains_key(node) {
            let output_facts = &data.output_per_node[node];
            rows.push("    <tr><td>-------------------</td></tr>".into());
            rows.extend(output_facts.into_iter().sorted().map(|(fact, importance)| {
                format!(r#"    <tr><td{}>{}</td></tr>"#, importance.style(), fact)
            }));
        }
        let lines: String = Itertools::intersperse(rows.iter().map(|s| s.as_str()), "\n").collect();
        output_dot += &format!(
            r#"    {} [ label = <<table border="0">
    <tr><td>{}</td></tr>
    <tr><td>-------------------</td></tr>
{}
    </table>> ]"#,
            node, node_text, lines
        );

        if let Some(preds) = data.node_predecessors.get(node) {
            for pred in preds {
                output_dot += &format!("    {} -> {}", pred, node);
            }
            output_dot += "\n";
        }
    }

    output_dot += "}";

    let mut output_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_file_path)
        .expect("could not open output file");
    output_file
        .write(output_dot.as_bytes())
        .expect("could not write to output file");

    // Try producing a PDF image from the dotfile
    match Command::new("dot")
        .args(&[
            "-Tpdf",
            "-O",
            output_file_path.display().to_string().as_str(),
        ])
        .output()
    {
        _ => {} // ignore Result
    }
}
