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
    let levels: Vec<u32> = report
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    if check_levels(&levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut modified_levels = levels.clone();
        modified_levels.remove(i);
        if check_levels(&modified_levels) {
            return true;
        }
    }
    false
}

fn check_levels(levels: &[u32]) -> bool {
    let mut last_num = levels[0];
    let mut increasing = false;
    let mut decreasing = false;

    for &current_num in &levels[1..] {
        let diff = (current_num as i32 - last_num as i32).unsigned_abs();

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
