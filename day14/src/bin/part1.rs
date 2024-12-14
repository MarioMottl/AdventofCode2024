use anyhow::Result;
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

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let mut robots = parse_input(&contents).unwrap();
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.update_position();
        }
    }

    let mut quadrants = [0, 0, 0, 0];

    for robot in &robots {
        if robot.x == (WIDTH / 2) as isize || robot.y == (HEIGHT / 2) as isize {
            continue;
        }
        if robot.x < (WIDTH / 2) as isize && robot.y < (HEIGHT / 2) as isize {
            quadrants[0] += 1;
        } else if robot.x >= (WIDTH / 2) as isize && robot.y < (HEIGHT / 2) as isize {
            quadrants[1] += 1;
        } else if robot.x < (WIDTH / 2) as isize && robot.y >= (HEIGHT / 2) as isize {
            quadrants[2] += 1;
        } else {
            quadrants[3] += 1;
        }
    }

    let safety_factor = quadrants.iter().product::<usize>();
    println!("Safety factor: {}", safety_factor);
}
