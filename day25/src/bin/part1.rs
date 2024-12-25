use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

#[allow(dead_code)]
const INPUT_FILE: &str = "input.txt";
#[allow(dead_code)]
const EXAMPLE_FILE: &str = "example.txt";

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

#[derive(Debug)]
struct Pattern {
    heights: Vec<usize>,
    is_lock: bool,
}

impl Pattern {
    fn from_str(input: &str) -> Self {
        let mut lines: Vec<&str> = input.lines().collect();
        let is_lock = lines[0].chars().filter(|&c| c == '#').count() == 5;

        if !is_lock {
            lines.reverse();
        }

        let mut heights = Vec::new();
        let cols = lines[0].len();

        // Process each column
        for col in 0..cols {
            let mut height = 0;
            for row in 0..lines.len() {
                if lines[row].chars().nth(col) == Some('#') {
                    height += 1;
                } else {
                    break;
                }
            }
            heights.push(height);
        }

        Self { heights, is_lock }
    }
}

fn can_fit(key: &Pattern, lock: &Pattern) -> bool {
    assert!(key.is_lock && !lock.is_lock);
    assert_eq!(lock.heights.len(), key.heights.len());

    key.heights.iter().zip(lock.heights.iter())
        .all(|(&k, &l)| k + l <= 7)
}


fn main() {
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));


    let start = Instant::now();
    let patterns: Vec<Pattern> = contents
        .split("\n\n")
        .map(Pattern::from_str)
        .collect();
    let parsing_duration = start.elapsed();

    let (keys, locks): (Vec<&Pattern>, Vec<&Pattern>) = patterns
        .iter()
        .partition(|p| p.is_lock);

    let start = Instant::now();
    let mut valid_count = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if can_fit(key, lock) {
                valid_count += 1;
            }
        }
    }
    let calculation_duration = start.elapsed();

    println!("Part 1: {}", valid_count);
    println!("Durations:");
    println!("  Parsing:      {:?}", parsing_duration);
    println!("  Calculation:  {:?}", calculation_duration);
    println!("  Total:        {:?}", start.elapsed());
}
