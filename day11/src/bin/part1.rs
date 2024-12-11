use anyhow::Result;
use std::fs::File;
use std::io::Read;

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

fn blink(stones: Vec<u64>) -> Vec<u64> {
    let mut new_stones = Vec::new();
    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
        } else if stone.to_string().len() % 2 == 0 {
            let digits = stone.to_string();
            let mid = digits.len() / 2;
            let left = digits[..mid].parse::<u64>().unwrap();
            let right = digits[mid..].parse::<u64>().unwrap();
            new_stones.push(left);
            new_stones.push(right);
        } else {
            new_stones.push(stone * 2024);
        }
    }
    new_stones
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let mut stones = contents
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let blinks: u32 = 25;
    for _ in 0..blinks {
        stones = blink(stones);
    }
    println!("Number of stones after {} blinks: {}", blinks, stones.len());
}
