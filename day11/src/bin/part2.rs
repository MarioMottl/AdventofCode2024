use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

/*
https://adventofcode.com/2024/day/11
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

/*
* In the end it was way easier than expected.
* As the stones dont interact with each other, we can just simulate the process for each stone.
* This saves my PC and my sanity.
* */

fn count_blink_stones(s: u64, blinks_left: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    if blinks_left == 0 {
        return 1;
    }

    if let Some(&cached_result) = cache.get(&(s, blinks_left)) {
        return cached_result;
    }

    let result = if s == 0 {
        count_blink_stones(1, blinks_left - 1, cache)
    } else {
        let digits = s.to_string();
        if digits.len() % 2 == 0 {
            let mid = digits.len() / 2;
            let left: u64 = digits[..mid].parse().unwrap();
            let right: u64 = digits[mid..].parse().unwrap();
            count_blink_stones(left, blinks_left - 1, cache)
                + count_blink_stones(right, blinks_left - 1, cache)
        } else {
            count_blink_stones(s * 2024, blinks_left - 1, cache)
        }
    };

    cache.insert((s, blinks_left), result);
    result
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let stones: Vec<u64> = contents
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut cache: HashMap<(u64, u32), u64> = HashMap::new();

    let start = Instant::now();
    let answer: u64 = stones
        .iter()
        .map(|&s| count_blink_stones(s, 75, &mut cache))
        .sum();
    let duration = start.elapsed();

    println!("Number of stones after 75 blinks: {}", answer);
    println!("Time taken: {:?}", duration);
}
