use anyhow::Result;
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

fn transform_warehouse(warehouse: Vec<String>) -> Vec<String> {
    let mut new_warehouse = Vec::new();
    for row in warehouse {
        let mut new_row = String::new();
        for ch in row.chars() {
            match ch {
                '#' => new_row.push_str("##"),
                'O' => new_row.push_str("[]"),
                '.' => new_row.push_str(".."),
                '@' => new_row.push_str("@."),
                _ => new_row.push(ch),
            }
        }
        new_warehouse.push(new_row);
    }
    new_warehouse
}

fn can_push(
    warehouse: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    delta_x: isize,
    delta_y: isize,
) -> bool {
    let next_x = (start_x as isize + delta_x) as usize;
    let next_y = (start_y as isize + delta_y) as usize;
    if next_x >= warehouse.len() || next_y >= warehouse[0].len() {
        return false;
    }
    match warehouse[next_x][next_y] {
        '#' => false,
        '.' => true,
        ']' => {
            if next_y == 0 {
                return false;
            }
            can_push(warehouse, next_x, next_y, delta_x, delta_y)
                && can_push(warehouse, next_x, next_y - 1, delta_x, delta_y)
        }
        '[' => {
            if next_y + 1 >= warehouse[0].len() {
                return false;
            }
            can_push(warehouse, next_x, next_y, delta_x, delta_y)
                && can_push(warehouse, next_x, next_y + 1, delta_x, delta_y)
        }
        _ => false,
    }
}

fn push_boxes(
    warehouse: &mut Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    delta_x: isize,
    delta_y: isize,
) {
    let next_x = (start_x as isize + delta_x) as usize;
    let next_y = (start_y as isize + delta_y) as usize;
    if next_x >= warehouse.len() || next_y >= warehouse[0].len() {
        return;
    }
    match warehouse[next_x][next_y] {
        '#' => return,
        '.' => {
            warehouse[start_x][start_y] = '.';
            warehouse[next_x][next_y] = '@';
        }
        ']' => {
            if next_y == 0 {
                return;
            }
            push_boxes(warehouse, next_x, next_y, delta_x, delta_y);
            push_boxes(warehouse, next_x, next_y - 1, delta_x, delta_y);
            warehouse[start_x][start_y] = '.';
            warehouse[next_x][next_y] = '@';
        }
        '[' => {
            if next_y + 1 >= warehouse[0].len() {
                return;
            }
            push_boxes(warehouse, next_x, next_y, delta_x, delta_y);
            push_boxes(warehouse, next_x, next_y + 1, delta_x, delta_y);
            warehouse[start_x][start_y] = '.';
            warehouse[next_x][next_y] = '@';
        }
        _ => {}
    }
}

fn simulate(warehouse: Vec<String>, moves: String) -> usize {
    let mut warehouse_grid: Vec<Vec<char>> =
        warehouse.iter().map(|row| row.chars().collect()).collect();
    let mut gps_sum = 0;
    let rows = warehouse_grid.len();
    let cols = warehouse_grid[0].len();
    let mut robot_x = 0;
    let mut robot_y = 0;

    let delta_x = [0, -1, 0, 1];
    let delta_y = [-1, 0, 1, 0];

    for mv in moves.chars() {
        let direction = match mv {
            '^' => 1,
            '<' => 0,
            '>' => 2,
            'v' => 3,
            _ => continue,
        };

        if can_push(
            &warehouse_grid,
            robot_x,
            robot_y,
            delta_x[direction],
            delta_y[direction],
        ) {
            push_boxes(
                &mut warehouse_grid,
                robot_x,
                robot_y,
                delta_x[direction],
                delta_y[direction],
            );
            robot_x = (robot_x as isize + delta_x[direction]) as usize;
            robot_y = (robot_y as isize + delta_y[direction]) as usize;
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            if warehouse_grid[i][j] == '[' {
                gps_sum += 100 * i + j;
            }
        }
    }

    gps_sum
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let (warehouse, moves) = parse_input(&contents);
    let transformed_warehouse = transform_warehouse(warehouse);
    let gps_sum = simulate(transformed_warehouse, moves);
    println!("Sum of GPS coordinates: {}", gps_sum);
}
