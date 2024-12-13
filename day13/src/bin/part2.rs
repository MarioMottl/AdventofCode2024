use anyhow::Result;
use std::fs::File;
use std::io::Read;

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

fn parse_input(data: &str) -> Vec<(&str, &str, &str)> {
    data.split("\n\n")
        .map(|group| {
            let lines: Vec<&str> = group.split('\n').collect();
            (lines[0], lines[1], lines[2])
        })
        .collect()
}

fn extract_coordinates(button: &str, prefix_len: usize) -> (i64, i64) {
    let coords: Vec<&str> = button[prefix_len..].split(", ").collect();
    let x: i64 = coords[0][2..].parse().unwrap();
    let y: i64 = coords[1][2..].parse().unwrap();
    (x, y)
}

/*
*
* Diophantine equation: https://en.wikipedia.org/wiki/Diophantine_equation
* Cramer's rule: https://en.wikipedia.org/wiki/Cramer%27s_rule
*
*/
fn calculate_total(groups: Vec<(&str, &str, &str)>, offset: i64) -> i64 {
    let mut total = 0;

    for (button_a, button_b, button_p) in groups {
        let (ax, ay) = extract_coordinates(button_a, 10);
        let (bx, by) = extract_coordinates(button_b, 10);
        let (px, py) = extract_coordinates(button_p, 7);
        let px = px + offset;
        let py = py + offset;

        // Cramer's rule
        let denominator = ax * by - ay * bx; // determinant

        if denominator == 0 {
            continue;
        }

        let m = (px * by - py * bx) / denominator;

        if m * denominator != (px * by - py * bx) {
            continue;
        }

        let n = (py - ay * m) / by;

        if n * by != (py - ay * m) {
            continue;
        }

        // Diophantine equation with the solutions by Cramer's rule
        total += 3 * m + n;
    }

    total
}

fn main() {
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let start_time = std::time::Instant::now();

    let groups = parse_input(&contents);
    let result = calculate_total(groups, 10_i64.pow(13));

    let calculation_time = start_time.elapsed().as_millis();

    println!(
        "Part 2: {} (Calculation Time: {} ms)",
        result, calculation_time
    );
}
