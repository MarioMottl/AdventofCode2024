use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
/*
https://adventofcode.com/2024/day/1
*/

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

const INPUT_FILE: &str = "input.txt";

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    println!("{}", contents);
    let mut left_side: Vec<u32> = Vec::new();
    let mut right_side: Vec<u32> = Vec::new();
    for line in contents.lines() {
        let split_line: Vec<&str> = line.split("   ").collect();
        let lhs: u32 = FromStr::from_str(split_line[0]).unwrap();
        let rhs: u32 = FromStr::from_str(split_line[1]).unwrap();
        left_side.push(lhs);
        right_side.push(rhs);
    }
    left_side.sort();
    right_side.sort();

    let difference: u32 = left_side
        .iter()
        .zip(right_side)
        .fold(0, |acc, (&lhs, rhs)| {
            acc + (lhs as i32 - rhs as i32).unsigned_abs()
        });
    println!("Total distance: {}", difference);
}
