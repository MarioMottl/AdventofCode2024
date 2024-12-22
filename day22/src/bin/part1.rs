use anyhow::Result;
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

fn get_nth_secret(initial: u64, n: usize) -> u64 {
    let mut secret = initial;
    for _ in 0..n {
        secret = next_secret(secret);
    }
    secret
}

fn solve_part1(contents: &str) -> u64 {
    contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<u64>().unwrap())
        .map(|initial| get_nth_secret(initial, 2000))
        .sum()
}

fn main() {
    let contents = read_input(INPUT_FILE).unwrap();
    let result = solve_part1(&contents);
    println!("Part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let example = "1\n10\n100\n2024\n";
        assert_eq!(solve_part1(example), 37327623);
    }

    #[test]
    fn test_single_sequence() {
        let secret = 123;
        let expected = vec![
            15887950, 16495136, 527345, 704524, 1553684,
            12683156, 11100544, 12249484, 7753432, 5908254
        ];

        let mut current = secret;
        for &expected_value in &expected {
            current = next_secret(current);
            assert_eq!(current, expected_value);
        }
    }
}
