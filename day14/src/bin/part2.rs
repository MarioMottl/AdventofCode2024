use anyhow::Result;
use image::{Rgb, RgbImage};
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

/*
https://adventofcode.com/2024/day/14
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

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

struct Robot {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

impl Robot {
    #[allow(dead_code)]
    fn update_position(&mut self) {
        self.x = (self.x + self.vx).rem_euclid(WIDTH as isize);
        self.y = (self.y + self.vy).rem_euclid(HEIGHT as isize);
    }
}

fn parse_input(input: &str) -> Result<Vec<Robot>> {
    let mut robots = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let position = parts[0]
            .trim_start_matches("p=")
            .split(',')
            .collect::<Vec<&str>>();
        let velocity = parts[1]
            .trim_start_matches("v=")
            .split(',')
            .collect::<Vec<&str>>();

        let x = position[0].parse::<isize>().unwrap();
        let y = position[1].parse::<isize>().unwrap();
        let vx = velocity[0].parse::<isize>().unwrap();
        let vy = velocity[1].parse::<isize>().unwrap();

        robots.push(Robot { x, y, vx, vy });
    }

    Ok(robots)
}

#[allow(dead_code)]
fn display_grid(robots: &Vec<Robot>) {
    let mut grid = vec![vec!['.'; WIDTH]; HEIGHT];

    for robot in robots {
        grid[robot.y as usize][robot.x as usize] = '#';
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

#[allow(dead_code)]
fn get_grid_pattern(robots: &Vec<Robot>) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; WIDTH]; HEIGHT];

    for robot in robots {
        grid[robot.y as usize][robot.x as usize] = '#';
    }

    grid
}

#[allow(dead_code)]
fn save_pattern_as_png(pattern: &Vec<Vec<char>>, seconds: u32) -> Result<()> {
    let pattern_width = WIDTH as u32;
    let pattern_height = HEIGHT as u32;

    // Create a new image
    let mut img = RgbImage::new(pattern_width, pattern_height);

    // Draw the pattern
    for (y, row) in pattern.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let color = if cell == '#' {
                Rgb([0, 0, 0]) // Black for robots
            } else {
                Rgb([255, 255, 255]) // White for empty space
            };
            img.put_pixel(x as u32, y as u32, color);
        }
    }

    // Ensure the output directory exists
    let output_dir = "output";

    // Save the image as a PNG file
    img.save(format!("{}/pattern_{}.png", output_dir, seconds))?;

    Ok(())
}

fn calculate_multiplier(robots: &Vec<Robot>) -> usize {
    let mut final_positions = HashSet::new();
    let mut multiplier = 0;

    while final_positions.len() != robots.len() {
        final_positions.clear();
        multiplier += 1;

        for robot in robots {
            let final_x = (robot.x + robot.vx * multiplier as isize).rem_euclid(WIDTH as isize);
            let final_y = (robot.y + robot.vy * multiplier as isize).rem_euclid(HEIGHT as isize);
            final_positions.insert((final_x, final_y));
        }
    }

    multiplier
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let robots = parse_input(&contents).unwrap();

    let start = std::time::Instant::now();
    let multiplier = calculate_multiplier(&robots);
    let elapsed = start.elapsed();
    println!("Multiplier: {}", multiplier);
    println!("Elapsed: {:?}", elapsed);

    // FOR PNG GENERATION AND FIGURING OUT THE WHAT THE CHRISTMAS TREE LOOKS LIKE
    //let mut seen_patterns = HashSet::new();
    //let mut seconds = 0;
    //let mut patterns = Vec::new();
    //
    //loop {
    //    // Update positions
    //    for robot in &mut robots {
    //        robot.update_position();
    //    }
    //
    //    // Get the current grid pattern
    //    let pattern = get_grid_pattern(&robots);
    //
    //    // Check if the pattern is unique
    //    if seen_patterns.insert(pattern.clone()) {
    //        patterns.push((seconds, pattern));
    //    }
    //
    //    // Increment the seconds counter
    //    seconds += 1;
    //
    //    // Add a break condition to avoid an infinite loop during testing
    //    if seconds > WIDTH as u32 * HEIGHT as u32 * 2 {
    //        break;
    //    }
    //}
    //
    //// Save each pattern as a separate PNG file
    //for (seconds, pattern) in patterns {
    //    save_pattern_as_png(&pattern, seconds).unwrap();
    //}
}
