use anyhow::Result;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::Read;
use std::time::Instant;
/*
https://adventofcode.com/2024/day/18
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

const GRID_SIZE: usize = 71; // EXAMPLE == 7 and ACTUAL == 71 Please don't forget ot change

fn bfs(grid: &[Vec<char>], start: (usize, usize), goal: (usize, usize)) -> Option<usize> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some(((x, y), steps)) = queue.pop_front() {
        if (x, y) == goal {
            return Some(steps);
        }

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < GRID_SIZE as isize && ny >= 0 && ny < GRID_SIZE as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if grid[ny][nx] == '.' && !visited.contains(&(nx, ny)) {
                    queue.push_back(((nx, ny), steps + 1));
                    visited.insert((nx, ny));
                }
            }
        }
    }

    None
}

fn parse_byte_positions(contents: &str) -> Result<Vec<(usize, usize)>> {
    let mut positions = Vec::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        let x = parts[0].parse::<usize>()?;
        let y = parts[1].parse::<usize>()?;
        positions.push((x, y));
    }
    Ok(positions)
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = match read_input(INPUT_FILE) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading input file: {}", err);
            return;
        }
    };

    let byte_positions = match parse_byte_positions(&contents) {
        Ok(positions) => positions,
        Err(err) => {
            eprintln!("Error parsing byte positions: {}", err);
            return;
        }
    };

    let mut grid = vec![vec!['.'; GRID_SIZE]; GRID_SIZE];

    let start = Instant::now();
    for &(x, y) in byte_positions.iter() {
        grid[y][x] = '#';

        if bfs(&grid, (0, 0), (GRID_SIZE - 1, GRID_SIZE - 1)).is_none() {
            let duration = start.elapsed();
            println!("{},{}", x, y);
            println!("Part2 took: {:?}", duration);
            return;
        }
    }
}
