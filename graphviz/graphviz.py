import sys
from pathlib import Path
import html

input_facts_dir = sys.argv[1]
output_facts_dir = sys.argv[2]

node_texts = {}
node_predecessors = {}
input_per_node = {}
output_per_node = {}

# Process input facts: load fact files from the provided input facts directory,
# and store the atoms (without locations) in the files as facts at each node in the CFG
for p in Path(input_facts_dir).glob("*.facts"):
    relation = p.stem
    facts = p.read_text().splitlines()

    # Except `cfg_edge`, all input relations have the node location as the last atom
    for line in facts:
        atoms = line.split("\t")
        if relation == "node_text":
            # The text to summarize each node
            node = atoms.pop()
            # To be displayed in hackmd, escape the node text so that ticks and ampersands show up
            node_texts[node] = html.escape(atoms[0])
        elif relation != "cfg_edge":
            # Actual facts happening at the node
            node = atoms.pop()
            if node not in input_per_node:
                input_per_node[node] = []

            pretty_atoms = ", ".join(atoms)
            fact = f"{relation}({pretty_atoms})"
            input_per_node[node].append(fact)
        else:
            # The edges in the CFG to transform into graphviz edges
            [p, q] = atoms
            if q not in node_predecessors:
                node_predecessors[q] = []
            node_predecessors[q].append(p)

# Process output facts, in a similar fashion as the input facts: the relations
# are also suffixed by the node.
for p in Path(output_facts_dir).glob("*.csv"):
    relation = p.stem
    facts = p.read_text().splitlines()

    for line in facts:
        atoms = line.split("\t")
        node = atoms.pop()
        if node not in output_per_node:
            output_per_node[node] = []

        pretty_atoms = ", ".join(atoms)
        fact = f"{relation}({pretty_atoms})"
        output_per_node[node].append(fact)

# Output the graphviz file in the format used in the hackmd. First, the header.
print("""digraph G {
    rankdir = "TD"
    node [ shape = "rectangle" ]
""")
for node in sorted(input_per_node):
    input_facts = input_per_node[node]
    node_text = node_texts[node]

    # Then the body: the graph nodes, formatted as
    # - the node header setting up the table with facts as rows
    # - the node text
    # - a row per input fact
    # - if output facts exists, a separator, then a row per output fact
    # - edges from the predecessors to the node, if any

    rows = [f"    <tr><td>{fact}</td></tr>" for fact in sorted(input_facts)]

    if node in output_per_node:
        output_facts = output_per_node[node]
        rows.append("    <tr><td>-------------------</td></tr>")
        rows.extend([f"    <tr><td>{fact}</td></tr>" for fact in sorted(output_facts)])

    lines = "\n".join(rows)
    print(f"""    {node} [ label = <<table border="0">
    <tr><td>{node_text}</td></tr>
{lines}
    </table>> ]""")

    for pred in node_predecessors.get(node, []):
        print(f"    {pred} -> {node}")
    print()

print("}")