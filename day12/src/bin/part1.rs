use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

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

fn calculate_fence_price(map: &Vec<Vec<char>>) -> i32 {
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if !visited.contains(&(i, j)) {
                let (area, perimeter) = flood_fill(map, i, j, map[i][j], &mut visited);
                let price = area * perimeter;
                println!(
                    "Region of {} plants with area {} and perimeter {} has price {}",
                    map[i][j], area, perimeter, price
                );
                total_price += price;
            }
        }
    }
    total_price
}

fn flood_fill(
    map: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    plant: char,
    visited: &mut HashSet<(usize, usize)>,
) -> (i32, i32) {
    let mut stack = vec![(x, y)];
    let mut area = 0;
    let mut perimeter = 0;

    while let Some((i, j)) = stack.pop() {
        if i >= map.len() || j >= map[0].len() || visited.contains(&(i, j)) || map[i][j] != plant {
            continue;
        }

        visited.insert((i, j));
        area += 1;

        let neighbors = vec![
            (i.wrapping_sub(1), j),
            (i + 1, j),
            (i, j.wrapping_sub(1)),
            (i, j + 1),
        ];

        for &(ni, nj) in &neighbors {
            if ni >= map.len() || nj >= map[0].len() || map[ni][nj] != plant {
                perimeter += 1;
            } else if !visited.contains(&(ni, nj)) {
                stack.push((ni, nj));
            }
        }
    }

    (area, perimeter)
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let total_price = calculate_fence_price(&map);
    println!("Total price: {}", total_price);
}
