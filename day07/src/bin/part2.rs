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

const INPUT_FILE: &str = "input.txt";

fn evaluate_expression(numbers: &[u64], operators: &[char]) -> u64 {
    let mut result = numbers[0];
    let mut i = 0;
    while i < operators.len() {
        match operators[i] {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            '|' => {
                let concatenated = format!("{}{}", result, numbers[i + 1]);
                result = concatenated
                    .parse::<u64>()
                    .expect("Failed to parse concatenated number");
            }
            _ => panic!("Unknown operator"),
        }
        i += 1;
    }
    result
}

fn find_valid_expression(numbers: &[u64], operators: &[char], test_value: u64) -> bool {
    if operators.len() == numbers.len() - 1 {
        return evaluate_expression(numbers, operators) == test_value;
    }

    let mut new_operators = operators.to_vec();
    new_operators.push('+');
    if find_valid_expression(numbers, &new_operators, test_value) {
        return true;
    }

    new_operators.pop();
    new_operators.push('*');
    if find_valid_expression(numbers, &new_operators, test_value) {
        return true;
    }

    new_operators.pop();
    new_operators.push('|');
    if find_valid_expression(numbers, &new_operators, test_value) {
        return true;
    }

    false
}

fn main() {
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let mut total_calibration_result = 0;

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        let test_value: u64 = parts[0].parse().expect("Invalid number");
        let numbers: Vec<u64> = parts[1]
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid number"))
            .collect();

        if find_valid_expression(&numbers, &[], test_value) {
            total_calibration_result += test_value;
        }
    }

    println!("Total calibration result: {}", total_calibration_result);
}
