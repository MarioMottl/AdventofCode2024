use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

/*
https://adventofcode.com/2024/day/6
*/

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

const INPUT_FILE: &str = "input.txt";

fn find_starting_position(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell == '^' {
                return Some((row_index, col_index));
            }
        }
    }
    None
}

#[derive(Eq, Hash, PartialEq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<char>]) {
    for row in grid {
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn simulate_guard(
    grid: &mut [Vec<char>],
    start_position: (usize, usize),
    allow_out_of_bounds: bool,
) -> Result<HashSet<(usize, usize)>> {
    let mut direction = Direction::Up;
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut visited_with_direction: HashSet<((usize, usize), Direction)> = HashSet::new();
    let mut position = start_position;
    visited_positions.insert(position);
    visited_with_direction.insert((position, direction.clone())); // Clone the direction

    let directions = [
        (-1, 0), // Up
        (0, 1),  // Right
        (1, 0),  // Down
        (0, -1), // Left
    ];

    loop {
        let (dx, dy) = match direction {
            Direction::Up => directions[0],
            Direction::Right => directions[1],
            Direction::Down => directions[2],
            Direction::Left => directions[3],
        };

        let next_position = (
            (position.0 as isize + dx) as usize,
            (position.1 as isize + dy) as usize,
        );

        if next_position.0 >= grid.len() || next_position.1 >= grid[0].len() {
            if allow_out_of_bounds {
                break;
            } else {
                return Err(anyhow::anyhow!("Out of bounds"));
            }
        }

        if grid[next_position.0][next_position.1] == '#' {
            direction = direction.turn_right();
        } else {
            position = next_position;
            if !visited_with_direction.insert((position, direction.clone())) {
                break; // Loop detected
            }
            visited_positions.insert(position);
        }
    }

    Ok(visited_positions)
}

fn main() {
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let orig_grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start_position = find_starting_position(&orig_grid).unwrap();

    let mut grid = orig_grid.clone();
    let visited_positions = simulate_guard(&mut grid, start_position, true).unwrap();

    let allow_out_of_bounds = false;

    let mut loop_positions = HashSet::new();

    for &(row, col) in &visited_positions {
        if (row, col) != start_position {
            grid[row][col] = '#';
            if let Ok(_new_visited_positions) =
                simulate_guard(&mut grid, start_position, allow_out_of_bounds)
            {
                loop_positions.insert((row, col));
            }
        }
        grid[row][col] = '.';
    }

    println!(
        "Number of positions to place obstruction: {}",
        loop_positions.len()
    );
}
