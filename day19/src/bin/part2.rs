use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::time::Instant;
/*
https://adventofcode.com/2024/day/19
*/

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

#[allow(dead_code)]
const INPUT_FILE: &str = "input.txt";
#[allow(dead_code)]
const EXAMPLE_FILE: &str = "example.txt";

fn parse_file(input: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = input.lines();
    let patterns_line = lines.next().unwrap();
    let patterns: Vec<String> = patterns_line.split(", ").map(String::from).collect();
    let designs: Vec<String> = lines.filter(|line| !line.is_empty()).map(String::from).collect();
    (patterns, designs)
}

fn count_ways(design: &str, patterns: &HashSet<&str>, memo: &mut HashMap<String, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&result) = memo.get(design) {
        return result;
    }
    let mut total_ways = 0;
    for pattern in patterns {
        if design.starts_with(pattern) {
            total_ways += count_ways(&design[pattern.len()..], patterns, memo);
        }
    }
    memo.insert(design.to_string(), total_ways);
    total_ways
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let (patterns, designs) = parse_file(&contents);
    let patterns_set: HashSet<&str> = patterns.iter().map(String::as_str).collect();
    let mut memo = HashMap::new();
    let mut total_count = 0;

    let start = Instant::now();
    for design in designs {
        total_count += count_ways(&design, &patterns_set, &mut memo);
    }
    let duration = start.elapsed();

    println!("Total number of ways to make all designs: {}", total_count);
    println!("Part2 took: {:?}", duration);
}
