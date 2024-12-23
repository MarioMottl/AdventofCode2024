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

fn find_largest_clique(input: &str) -> String {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        if line.is_empty() { continue; }
        let (a, b) = line.split_once('-').unwrap();
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
    }

    let nodes: Vec<&str> = graph.keys().copied().collect();
    let mut largest_clique: HashSet<&str> = HashSet::new();

    for &start_node in &nodes {
        let mut potential_clique: HashSet<&str> = HashSet::from([start_node]);
        let mut candidates: HashSet<&str> = graph[start_node].clone();

        while !candidates.is_empty() {
            if let Some(&next_node) = candidates.iter().find(|&&node| {
                potential_clique.iter().all(|&clique_node| graph[node].contains(clique_node))
            }) {
                potential_clique.insert(next_node);
                candidates = candidates
                    .iter()
                    .copied()
                    .filter(|&node| node != next_node &&
                        potential_clique.iter().all(|&clique_node| graph[node].contains(clique_node)))
                    .collect();
            } else {
                break;
            }
        }

        if potential_clique.len() > largest_clique.len() {
            largest_clique = potential_clique;
        }
    }

    let mut result_vec: Vec<&str> = largest_clique.into_iter().collect();
    result_vec.sort_unstable();
    result_vec.join(",")
}

#[allow(dead_code)]
const INPUT_FILE: &str = "input.txt";
#[allow(dead_code)]
const EXAMPLE_FILE: &str = "example.txt";

fn main() {
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let password = find_largest_clique(&contents);
    println!("LAN party password: {}", password);
}
