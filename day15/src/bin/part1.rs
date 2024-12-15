use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

/*
https://adventofcode.com/2024/day/15
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

fn parse_input(input: &str) -> (Vec<String>, String) {
    let mut warehouse = Vec::new();
    let mut moves = String::new();
    let mut reading_moves = false;

    for line in input.lines() {
        if line.is_empty() {
            reading_moves = true;
            continue;
        }
        if reading_moves {
            moves.push_str(&line);
        } else {
            warehouse.push(line.to_string());
        }
    }

    (warehouse, moves)
}

fn print_warehouse(
    warehouse: &Vec<String>,
    robot_pos: (usize, usize),
    boxes: &HashSet<(usize, usize)>,
) {
    for (y, row) in warehouse.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if (y, x) == robot_pos {
                print!("@");
            } else if boxes.contains(&(y, x)) {
                print!("O");
            } else {
                print!("{}", ch);
            }
        }
        println!();
    }
    println!();
}

fn simulate_robot(mut warehouse: Vec<String>, moves: String) -> usize {
    let mut robot_pos = (0, 0);
    let mut boxes: HashSet<(usize, usize)> = HashSet::new();
    for (y, row) in warehouse.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == 'O' {
                boxes.insert((y, x));
            } else if ch == '@' {
                robot_pos = (y, x);
            }
        }
    }

    //print_warehouse(&warehouse, robot_pos, &boxes);

    for mv in moves.chars() {
        let (dy, dx) = match mv {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => continue,
        };

        let new_robot_pos = (
            (robot_pos.0 as isize + dy) as usize,
            (robot_pos.1 as isize + dx) as usize,
        );
        if warehouse[new_robot_pos.0]
            .chars()
            .nth(new_robot_pos.1)
            .unwrap()
            == '#'
        {
            continue;
        }

        if boxes.contains(&new_robot_pos) {
            let mut current_pos = new_robot_pos;
            let mut box_positions = Vec::new();

            // Collect all consecutive boxes in the direction of the movement
            while boxes.contains(&current_pos) {
                box_positions.push(current_pos);
                current_pos = (
                    (current_pos.0 as isize + dy) as usize,
                    (current_pos.1 as isize + dx) as usize,
                );
            }

            // Check if the last position is valid for the last box
            if warehouse[current_pos.0].chars().nth(current_pos.1).unwrap() == '#'
                || boxes.contains(&current_pos)
            {
                continue;
            }

            // Move all boxes in the line
            for &pos in box_positions.iter().rev() {
                boxes.remove(&pos);
                let new_box_pos = (
                    (pos.0 as isize + dy) as usize,
                    (pos.1 as isize + dx) as usize,
                );
                boxes.insert(new_box_pos);
            }
        }

        // Clear the old robot position
        let old_robot_pos = robot_pos;
        robot_pos = new_robot_pos;
        warehouse[old_robot_pos.0].replace_range(old_robot_pos.1..=old_robot_pos.1, ".");

        //print_warehouse(&warehouse, robot_pos, &boxes);
    }

    boxes.iter().map(|&(y, x)| y * 100 + x).sum()
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let (warehouse, moves) = parse_input(&contents);
    let gps_sum = simulate_robot(warehouse, moves);
    println!("Sum of GPS coordinates: {}", gps_sum);
}
