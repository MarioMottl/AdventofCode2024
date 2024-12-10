use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
/*
https://adventofcode.com/2024/day/10
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

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect()
}

fn find_trailheads(grid: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &height) in row.iter().enumerate() {
            if height == 0 {
                trailheads.push((row_index, col_index))
            }
        }
    }
    trailheads
}

fn dfs(grid: &[Vec<u32>], position: (usize, usize), visited: &mut HashSet<(usize, usize)>, current_height: u32) -> u32 {
    let (x, y) = position;
    if grid[x][y] == 9 {
        return 1;
    }
    visited.insert(position);

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut trail_count = 0;
    for &(dx, dy) in &directions {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        if new_x >= 0 && new_x < grid.len() as isize && new_y >= 0 && new_y < grid[0].len() as isize {
            let new_pos = (new_x as usize, new_y as usize);
            if !visited.contains(&new_pos) && grid[new_pos.0][new_pos.1] == current_height + 1 {
                trail_count += dfs(grid, new_pos, visited, current_height + 1);
            }
        }
    }
    visited.remove(&position);
    trail_count
}

fn calculate_trailhead_ratings(grid: &[Vec<u32>]) -> u32 {
    let trailheads = find_trailheads(grid);
    let mut total_rating = 0;

    for &trailhead in &trailheads {
        let mut visited = HashSet::new();
        let rating = dfs(grid, trailhead, &mut visited, 0);
        total_rating += rating;
    }

    total_rating
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let start_parse = Instant::now();
    let grid = parse_input(&contents);
    let duration_parse = start_parse.elapsed();
    println!("Time taken to parse input: {:?}", duration_parse);

    let start_calculate = Instant::now();
    let total_score = calculate_trailhead_ratings(&grid);
    let duration_calculate = start_calculate.elapsed();
    println!("Time taken to calculate trailhead ratings: {:?}", duration_calculate);

    println!("Total Score: {}", total_score);
}
