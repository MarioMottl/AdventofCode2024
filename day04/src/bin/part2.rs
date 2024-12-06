use anyhow::Result;
use std::fs::File;
use std::io::Read;

/*
https://adventofcode.com/2024/day/4
*/

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

const INPUT_FILE: &str = "input.txt";

fn is_x_mas(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || x >= grid.len() - 1 || y >= grid[0].len() - 1 {
        return false;
    }
    if grid[x][y] != 'A' {
        return false;
    }

    let ul = grid
        .get(x.wrapping_sub(1))
        .and_then(|row| row.get(y.wrapping_sub(1)));
    let dr = grid.get(x + 1).and_then(|row| row.get(y + 1));
    let ur = grid.get(x.wrapping_sub(1)).and_then(|row| row.get(y + 1));
    let dl = grid.get(x + 1).and_then(|row| row.get(y.wrapping_sub(1)));

    if !(ul == Some(&'M') && dr == Some(&'S') || ul == Some(&'S') && dr == Some(&'M')) {
        return false;
    }

    ur == Some(&'M') && dl == Some(&'S') || ur == Some(&'S') && dl == Some(&'M')
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut count: u32 = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if is_x_mas(&grid, x, y) {
                count += 1;
            }
        }
    }
    println!("XMAS Count: {}", count);
}
