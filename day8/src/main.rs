use std::env;
use std::char;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use std::fmt::Error;
use std::fmt::Formatter;

struct Node {
    label: String,
    children: Vec<usize>,
    metadata_entries: Vec<u32>,
}

struct NodeTree {
    nodes: Vec<Node>,
}

impl std::fmt::Display for NodeTree {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for node in self.nodes.iter() {
            writeln!(f, "{}", node.label)?;
            let children = node.children.iter().
                map(|idx| (&self.nodes[*idx].label).to_string()).
                collect::<Vec<String>>().
                join(", ");
            writeln!(f, "   - Children [{}]", children)?;

            let entries = node.metadata_entries.iter().
                map(|c| c.to_string()).
                collect::<Vec<String>>().
                join(", ");
            writeln!(f, "   - Entries [{}]", entries)?;
        }
        writeln!(f, "")
    }
}

impl std::str::FromStr for NodeTree {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        let mut values = s.split_whitespace().
            map(|v| v.parse::<u32>().unwrap()).peekable();

        let mut stack: Vec<(usize, u32, u32)> = Vec::with_capacity(s.len());
        let mut node_index: usize = 0;
        let mut nodes: Vec<Node> = vec![];

        loop {
            // Exit the loop if we have no more values
            if values.peek() == None {
                break;
            }

            // Do we have a parent on the stack?
            let mut pop_stack = false;
            if !stack.is_empty() {
                let t = stack.last_mut().unwrap();
                let parent_id = t.0;
                let children = t.1;
                let mut entries = t.2;

                // Have we finished processing all children for this node?
                if children == 0 {
                    while entries > 0 {
                        let entry = values.next().expect("Expected entry!");
                        nodes[parent_id].metadata_entries.push(entry);
                        entries -= 1;
                    }

                    // Finished with this node
                    pop_stack = true;
                } else {
                    // Decrease the number of children to process
                    t.1 = children - 1;

                    // Add index to parent for child we are about to create
                    nodes[parent_id].children.push(node_index);
                }
            }

            if pop_stack {
                stack.pop().unwrap();
                continue;
            }

            // Create a new node
            let label = char::from_u32((b'A' as u32) + (node_index as u32)).
                unwrap().to_string();
            nodes.push(Node{label: label.clone(), children: vec![], metadata_entries: vec![]});

            let children_count = values.next().expect("Expected children count!");
            let metadata_entry_count = values.next().expect("Expected metadata entry count!");
            stack.push((node_index, children_count, metadata_entry_count));
            node_index += 1;
        }

        Ok(NodeTree{nodes: nodes})
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Too few arguments!") }

    let f = File::open(&args[1]).expect("File not found!");
    let mut reader = BufReader::new(&f);

    let mut contents= String::new();
    reader.read_to_string(&mut contents).expect("Error reading file content!");

    let node_tree: NodeTree = contents.parse().expect("Error parsing node tree!");

    let checksum: u32 = node_tree.nodes.iter().
        flat_map(|node| node.metadata_entries.iter()).sum();

    println!("{}", checksum);
}
