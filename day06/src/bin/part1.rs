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

enum Direction {
    Up,
    Right,
    Down,
    Left,
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

fn main() {
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut direction = Direction::Up;
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut position = find_starting_position(&grid).unwrap();
    visited_positions.insert(position);

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
            break;
        }

        if grid[next_position.0][next_position.1] == '#' {
            direction = direction.turn_right();
        } else {
            position = next_position;
            visited_positions.insert(position);
        }
    }

    println!("Distinct positions visited: {}", visited_positions.len());
}
