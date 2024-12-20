use anyhow::Result;
use hashbrown::HashMap;
use itertools::Itertools;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;


/*
https://adventofcode.com/2024/day/20
*/

#[allow(dead_code)]
const INPUT_FILE: &str = "input.txt";
#[allow(dead_code)]
const EXAMPLE_FILE: &str = "example.txt";

type Point = (usize, usize);
type Grid = Vec<Vec<u8>>;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_map(contents: &str) -> Grid {
    contents.lines()
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

fn find_start_end(grid: &Grid) -> Result<(Point, Point)> {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            match cell {
                b'S' => start = (i, j),
                b'E' => end = (i, j),
                _ => continue,
            }
        }
    }

    Ok((start, end))
}

fn bfs(grid: &[Vec<u8>], start: Point, end: Point, max_step: usize) -> usize {
    let mut queue = VecDeque::from([(start.0, start.1, 0usize)]);
    let mut distances = HashMap::new();

    while let Some((row, col, steps)) = queue.pop_front() {
        if distances.contains_key(&(row, col)) {
            continue;
        }

        distances.insert((row, col), steps);

        if (row, col) == end {
            continue;
        }

        for (dr, dc) in DIRECTIONS {
            let new_row = (row as i32 + dr) as usize;
            let new_col = (col as i32 + dc) as usize;

            if new_row < grid.len() && new_col < grid[0].len() && grid[new_row][new_col] != b'#' {
                queue.push_back((new_row, new_col, steps + 1));
            }
        }
    }

    let mut result = 0;
    for ((&(r1, c1), &n1), (&(r2, c2), &n2)) in distances.iter().tuple_combinations() {
        let manhattan_dist = r1.abs_diff(r2) + c1.abs_diff(c2);
        if manhattan_dist > max_step {
            continue;
        }

        let time_saved = n2.abs_diff(n1) - manhattan_dist;
        if time_saved >= 100 {
            result += 1;
        }
    }

    result
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let grid = read_map(&contents);
    let (start, end) = find_start_end(&grid).expect("No Start/End found");

    let result = bfs(&grid, start, end, 2);
    println!("Part 1: {}", result);
}
