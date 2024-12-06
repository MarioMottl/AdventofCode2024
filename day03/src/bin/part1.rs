use anyhow::Result;
use regex::Regex;
use std::fs::File;
use std::io::Read;

/*
https://adventofcode.com/2024/day/3
*/

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

const INPUT_FILE: &str = "input.txt";
#[allow(dead_code)]
const EXAMPLE_FILE: &str = "example.txt";

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let mut result: u32 = 0;
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    for mat in re.find_iter(&contents) {
        println!("Match found: {}", mat.as_str());
        let (lhs, rhs): (u32, u32) = {
            let parts: Vec<u32> = mat
                .as_str()
                .replace("mul(", "")
                .replace(")", "")
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            (parts[0], parts[1])
        };
        result += lhs * rhs;
    }
    println!("Result: {}", result);
}
