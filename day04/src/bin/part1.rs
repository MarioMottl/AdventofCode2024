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

fn search_word(grid: &[Vec<char>], word: &str) -> u32 {
    let directions = [
        (0, 1), //right
        (1, 0),
        (1, 1),
        (1, -1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
    ];

    let word_len = word.len();
    let rows = grid.len();
    let cols = grid[0].len();
    let word_chars: Vec<char> = word.chars().collect();
    let mut count: u32 = 0;

    for row in 0..rows {
        for col in 0..cols {
            for &(dx, dy) in &directions {
                let mut found = true;
                for i in 0..word_len {
                    let new_row = row as isize + i as isize * dx;
                    let new_col = col as isize + i as isize * dy;
                    if new_row < 0
                        || new_row >= rows as isize
                        || new_col < 0
                        || new_col >= cols as isize
                    {
                        found = false;
                        break;
                    }
                    if grid[new_row as usize][new_col as usize] != word_chars[i] {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let word = "XMAS";
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let count = search_word(&grid, word);
    println!("XMAS Count: {}", count);
}
