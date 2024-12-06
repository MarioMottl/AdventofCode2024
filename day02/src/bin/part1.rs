use anyhow::Result;
use std::fs::File;
use std::io::Read;
/*
https://adventofcode.com/2024/day/2
*/

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

const INPUT_FILE: &str = "input.txt";

fn is_valid_report(report: &str) -> bool {
    let mut last_num: u32 = report
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut increasing: bool = false;
    let mut decreasing: bool = false;

    for c in report.split_whitespace().skip(1) {
        let current_num = c.parse::<u32>().unwrap();
        println!("last_num: {}, current_num: {}", last_num, current_num);
        let diff = (current_num as i32 - last_num as i32).unsigned_abs();
        println!("Diff: {}", diff);

        if !(1..=3).contains(&diff) {
            return false;
        }

        match current_num.cmp(&last_num) {
            std::cmp::Ordering::Greater => {
                if decreasing {
                    return false;
                }
                increasing = true;
            }
            std::cmp::Ordering::Less => {
                if increasing {
                    return false;
                }
                decreasing = true;
            }
            std::cmp::Ordering::Equal => {}
        }
        last_num = current_num;
    }
    true
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let mut save_reports: u32 = 0;
    for line in contents.lines() {
        if is_valid_report(line) {
            save_reports += 1;
        }
    }
    println!("Num save reports: {}", save_reports)
}
