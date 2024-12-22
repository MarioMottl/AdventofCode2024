use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

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

#[inline(always)]
fn next_secret(mut secret: u64) -> u64 {
    secret = (secret ^ (secret * 64)) % 16777216;
    secret = (secret ^ (secret / 32)) % 16777216;
    secret = (secret ^ (secret * 2048)) % 16777216;
    secret
}

fn get_sequence(initial: u64) -> Vec<i32> {
    let mut seq = Vec::with_capacity(2001);
    let mut current = initial;
    seq.push((current % 10) as i32);

    for _ in 0..2000 {
        current = next_secret(current);
        seq.push((current % 10) as i32);
    }
    seq
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

fn solve_part2(initials: &[u64]) -> i32 {
    let mut pattern_values = HashMap::new();

    for &initial in initials {
        let prices = get_sequence(initial);
        let diffs: Vec<_> = prices.windows(2)
            .map(|w| w[1] - w[0])
            .collect();

        for i in 0..diffs.len().saturating_sub(3) {
            let pattern = (diffs[i], diffs[i + 1], diffs[i + 2], diffs[i + 3]);
            pattern_values.entry(pattern)
                .or_insert_with(HashMap::new)
                .entry(initial)
                .or_insert(prices[i + 4]);
        }
    }

    pattern_values.values()
        .map(|init_map| init_map.values().sum::<i32>())
        .max()
        .unwrap_or(0)
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let initials = parse_input(&contents);
    println!("Part 2: {}", solve_part2(&initials));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let example = "1\n2\n3\n2024\n";
        let initials = parse_input(example);
        assert_eq!(solve_part2(&initials), 23);
    }
}
