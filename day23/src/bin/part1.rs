use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

fn find_connected_triples(input: &str) -> usize {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        if line.is_empty() { continue; }
        let (a, b) = line.split_once('-').unwrap();
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
    }

    let nodes: Vec<&str> = graph.keys().copied().collect();
    let mut t_triples = 0;

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            for k in (j + 1)..nodes.len() {
                let a = nodes[i];
                let b = nodes[j];
                let c = nodes[k];

                if graph[a].contains(b) && graph[a].contains(c) && graph[b].contains(c) && (a.starts_with('t') || b.starts_with('t') || c.starts_with('t')) {
                    t_triples += 1;
                }
            }
        }
    }

    t_triples
}

#[allow(dead_code)]
const INPUT_FILE: &str = "input.txt";
#[allow(dead_code)]
const EXAMPLE_FILE: &str = "example.txt";

fn main() {
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let result = find_connected_triples(&contents);
    println!("Number of triples containing 't': {}", result);
}
