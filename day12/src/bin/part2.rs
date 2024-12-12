use anyhow::Result;
use lazy_static::lazy_static;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::Read;
use std::time::Instant;

/*
https://adventofcode.com/2024/day/12
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

// Seen in [What I learned by solving 50 Advent of Code challenges in Rust - Luciano Mammino](https://www.youtube.com/watch?v=udHjmno-tfA)
lazy_static! {
    static ref DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
}

fn calculate_fence_price(map: &Vec<Vec<char>>) -> i32 {
    let mut visited_cells = HashSet::new();
    let mut total_price = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if !visited_cells.contains(&(row, col)) {
                let (area, sides) = flood_fill(map, row, col, map[row][col], &mut visited_cells);
                let price = area * sides;
                println!(
                    "Region of {} plants with area {} and sides {} has price {}",
                    map[row][col], area, sides, price
                );
                total_price += price;
            }
        }
    }
    total_price
}

fn flood_fill(
    map: &Vec<Vec<char>>,
    start_row: usize,
    start_col: usize,
    plant_type: char,
    visited_cells: &mut HashSet<(usize, usize)>,
) -> (i32, i32) {
    let mut queue = VecDeque::new();
    queue.push_back((start_row, start_col));
    let mut area = 0;
    let mut perimeter_map = std::collections::HashMap::new();

    while let Some((current_row, current_col)) = queue.pop_front() {
        if current_row >= map.len()
            || current_col >= map[0].len()
            || visited_cells.contains(&(current_row, current_col))
            || map[current_row][current_col] != plant_type
        {
            continue;
        }

        visited_cells.insert((current_row, current_col));
        area += 1;

        for &(delta_row, delta_col) in DIRECTIONS.iter() {
            let neighbor_row = current_row.wrapping_add(delta_row as usize);
            let neighbor_col = current_col.wrapping_add(delta_col as usize);

            if neighbor_row < map.len()
                && neighbor_col < map[0].len()
                && map[neighbor_row][neighbor_col] == plant_type
            {
                queue.push_back((neighbor_row, neighbor_col));
            } else {
                perimeter_map
                    .entry((delta_row, delta_col))
                    .or_insert_with(HashSet::new)
                    .insert((current_row, current_col));
            }
        }
    }

    let sides = bfs_count_sides(&perimeter_map);

    (area, sides)
}

fn bfs_count_sides(
    perimeter_map: &std::collections::HashMap<(isize, isize), HashSet<(usize, usize)>>,
) -> i32 {
    let mut sides = 0;

    for (_, perimeter_cells) in perimeter_map {
        let mut seen_perimeter_cells = HashSet::new();
        for &(perimeter_row, perimeter_col) in perimeter_cells {
            if !seen_perimeter_cells.contains(&(perimeter_row, perimeter_col)) {
                sides += 1;
                let mut queue = VecDeque::new();
                queue.push_back((perimeter_row, perimeter_col));
                while let Some((current_row, current_col)) = queue.pop_front() {
                    if seen_perimeter_cells.contains(&(current_row, current_col)) {
                        continue;
                    }
                    seen_perimeter_cells.insert((current_row, current_col));
                    for &(delta_row, delta_col) in DIRECTIONS.iter() {
                        let neighbor_row = current_row.wrapping_add(delta_row as usize);
                        let neighbor_col = current_col.wrapping_add(delta_col as usize);
                        if perimeter_cells.contains(&(neighbor_row, neighbor_col)) {
                            queue.push_back((neighbor_row, neighbor_col));
                        }
                    }
                }
            }
        }
    }

    sides
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start = Instant::now();
    let total_price = calculate_fence_price(&map);
    let duration = start.elapsed();

    println!("Total price: {}", total_price);
    println!("Time taken: {:?}", duration);
}
