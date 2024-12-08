use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::usize;

/*
https://adventofcode.com/2024/day/8
*/

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

const INPUT_FILE: &str = "input";

fn node_diff(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    (a.0 - b.0, a.1 - b.1)
}

fn parse_input(
    input: &str,
) -> (
    HashMap<char, HashSet<(isize, isize)>>,
    HashMap<(isize, isize), usize>,
) {
    let mut nodes: HashMap<char, HashSet<(isize, isize)>> = HashMap::new();
    let mut indices: HashMap<(isize, isize), usize> = HashMap::new();

    for (i, row) in input.trim().lines().enumerate() {
        for (j, c) in row.chars().enumerate() {
            indices.insert((i as isize, j as isize), 0);
            if c != '.' {
                nodes
                    .entry(c)
                    .or_insert_with(HashSet::new)
                    .insert((i as isize, j as isize));
            }
        }
    }

    (nodes, indices)
}

fn calculate_antinodes(
    nodes: &HashMap<char, HashSet<(isize, isize)>>,
    indices: &HashMap<(isize, isize), usize>,
) -> (HashSet<(isize, isize)>, HashSet<(isize, isize)>) {
    let mut uniq: HashSet<(isize, isize)> = HashSet::new();
    let mut uniq2: HashSet<(isize, isize)> = HashSet::new();

    for antennas in nodes.values() {
        for &a in antennas {
            uniq2.insert(a);
            for &b in antennas {
                if a == b {
                    continue;
                }
                let d = node_diff(a, b);
                let mut node_ = (a.0 + d.0, a.1 + d.1);
                if indices.contains_key(&node_) {
                    uniq.insert(node_);
                    uniq2.insert(node_);
                    loop {
                        node_ = (node_.0 + d.0, node_.1 + d.1);
                        if indices.contains_key(&node_) {
                            uniq2.insert(node_);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    (uniq, uniq2)
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let (nodes, indices) = parse_input(&contents);
    let (uniq, _uniq2) = calculate_antinodes(&nodes, &indices);

    println!("Part 1: {}", uniq.len());
}
