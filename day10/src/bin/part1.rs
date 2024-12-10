use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

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

const INPUT_FILE: &str = "input.txt";
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

fn dfs(grid: &[Vec<u32>], position: (usize, usize), visited: &mut HashSet<(usize, usize)>, current_height: u32) -> HashSet<(usize, usize)> {
    let (x, y) = position;
    let mut reachable_nines = HashSet::new();
    if grid[x][y] == 9 {
        reachable_nines.insert(position);
    }
    visited.insert(position);

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for &(dx, dy) in &directions {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        if new_x >= 0 && new_x < grid.len() as isize && new_y >= 0 && new_y < grid[0].len() as isize {
            let new_pos = (new_x as usize, new_y as usize);
            if !visited.contains(&new_pos) && grid[new_pos.0][new_pos.1] == current_height + 1 {
                reachable_nines.extend(dfs(grid, new_pos, visited, current_height + 1));
            }
        }
    }
    reachable_nines
}

fn calculate_trailhead_scores(grid: &[Vec<u32>]) -> u32 {
    let trailheads = find_trailheads(grid);
    let mut total_score = 0;

    for &trailhead in &trailheads {
        let mut visited = HashSet::new();
        let reachable_nines = dfs(grid, trailhead, &mut visited, 0);
        total_score += reachable_nines.len() as u32;
    }

    total_score
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let grid = parse_input(&contents);

    let total_score = calculate_trailhead_scores(&grid);
    println!("Total Score: {}", total_score);
}
