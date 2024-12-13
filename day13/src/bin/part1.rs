use anyhow::Result;
use std::fs::File;
use std::io::Read;

use regex::Regex;

/*
https://adventofcode.com/2024/day/13
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

fn solve_diophantine(
    x_a: i32,
    y_a: i32,
    x_b: i32,
    y_b: i32,
    x_prize: i32,
    y_prize: i32,
) -> Option<(i32, i32)> {
    for a_presses in 0..=100 {
        for b_presses in 0..=100 {
            if a_presses * x_a + b_presses * x_b == x_prize
                && a_presses * y_a + b_presses * y_b == y_prize
            {
                return Some((a_presses, b_presses));
            }
        }
    }
    None
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\s+Button B: X\+(\d+), Y\+(\d+)\s+Prize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let mut total_tokens = 0;
    let mut prizes_won = 0;

    for machines in contents.split("\n\n") {
        let caps = re.captures(machines).unwrap();
        let x_a: i32 = caps[1].parse().unwrap();
        let y_a: i32 = caps[2].parse().unwrap();
        let x_b: i32 = caps[3].parse().unwrap();
        let y_b: i32 = caps[4].parse().unwrap();
        let x_prize: i32 = caps[5].parse().unwrap();
        let y_prize: i32 = caps[6].parse().unwrap();

        println!(
            "xA: {}, yA: {}, xB: {}, yB: {}, xPrize: {}, yPrize: {}",
            x_a, y_a, x_b, y_b, x_prize, y_prize
        );

        if let Some((a_presses, b_presses)) =
            solve_diophantine(x_a, y_a, x_b, y_b, x_prize, y_prize)
        {
            let tokens = a_presses * 3 + b_presses;
            total_tokens += tokens;
            prizes_won += 1;
        }
    }
    println!("Total tokens: {}", total_tokens);
    println!("Prizes won: {}", prizes_won);
}
