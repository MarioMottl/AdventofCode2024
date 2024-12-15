use anyhow::Result;
use enum_iterator::Sequence;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
#[allow(dead_code)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

#[allow(dead_code)]
impl Dir {
    pub fn from(c: char) -> Dir {
        match c {
            '^' => Dir::N,
            '>' => Dir::E,
            '<' => Dir::W,
            'v' => Dir::S,
            _ => panic!("Unknown Dir"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(pub i64, pub i64);

#[allow(dead_code)]
impl Pos {
    pub fn step(&self, dir: Dir) -> Pos {
        match dir {
            Dir::N => Pos(self.0, self.1 - 1),
            Dir::E => Pos(self.0 + 1, self.1),
            Dir::S => Pos(self.0, self.1 + 1),
            Dir::W => Pos(self.0 - 1, self.1),
        }
    }
}

#[derive(Clone)]
pub struct Map {
    pub width: i64,
    pub height: i64,
    content: Vec<Vec<char>>,
    background: char,
}

#[allow(dead_code)]
impl Map {
    pub fn from(content: Vec<Vec<char>>) -> Map {
        assert!(!content.is_empty());
        assert!(!content.get(0).unwrap().is_empty());
        let height = content.len() as i64;
        let width = content.get(0).unwrap().len() as i64;
        assert!(content.iter().all(|x| x.len() as i64 == width));
        Map {
            width,
            height,
            content,
            background: ' ',
        }
    }

    pub fn valid_pos(&self, pos: Pos) -> bool {
        pos.0 >= 0 && pos.0 < self.width && pos.1 >= 0 && pos.1 < self.height
    }

    pub fn get(&self, pos: Pos) -> char {
        if self.valid_pos(pos) {
            self.content[pos.1 as usize][pos.0 as usize].clone()
        } else {
            self.background
        }
    }

    pub fn set(&mut self, pos: Pos, c: char) {
        assert!(self.valid_pos(pos));
        self.content[pos.1 as usize][pos.0 as usize] = c
    }

    pub fn find(&self, c: char) -> Vec<Pos> {
        let mut res = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos(x, y);
                if self.get(pos) == c {
                    res.push(pos);
                }
            }
        }
        res
    }

    pub fn to_string(&self) -> String {
        self.content
            .iter()
            .map(|v| v.into_iter().collect::<String>() + "\n")
            .into_iter()
            .collect()
    }
}

fn can_step(m: &Map, p: Pos, d: Dir) -> bool {
    let i = m.get(p);
    let t = p.step(d);
    match i {
        '.' => true,
        '#' => false,
        '[' => match d {
            Dir::N | Dir::S => can_step(m, t, d) && can_step(m, t.step(Dir::E), d),
            _ => can_step(m, t.step(d), d),
        },
        ']' => match d {
            Dir::N | Dir::S => can_step(m, t, d) && can_step(m, t.step(Dir::W), d),
            _ => can_step(m, t.step(d), d),
        },
        _ => can_step(m, t, d),
    }
}

fn do_step(m: &mut Map, p: Pos, d: Dir) {
    let i = m.get(p);
    let t = p.step(d);
    match i {
        '.' => {}
        '#' => panic!("Trying to move walls"),
        '[' => match d {
            Dir::N | Dir::S => {
                do_step(m, t, d);
                do_step(m, t.step(Dir::E), d);
                m.set(t, '[');
                m.set(t.step(Dir::E), ']');
                m.set(p, '.');
                m.set(p.step(Dir::E), '.');
            }
            Dir::E => {
                do_step(m, t.step(d), d);
                m.set(p.step(d).step(d), ']');
                m.set(p.step(d), '[');
                m.set(p, '.');
            }
            _ => panic!("Illegal move"),
        },
        ']' => match d {
            Dir::N | Dir::S => {
                do_step(m, t, d);
                do_step(m, t.step(Dir::W), d);
                m.set(t, ']');
                m.set(t.step(Dir::W), '[');
                m.set(p, '.');
                m.set(p.step(Dir::W), '.');
            }
            Dir::W => {
                do_step(m, t.step(d), d);
                m.set(p.step(d).step(d), '[');
                m.set(p.step(d), ']');
                m.set(p, '.');
            }
            _ => panic!("Illegal move"),
        },
        _ => {
            do_step(m, t, d);
            m.set(t, '@');
            m.set(p, '.');
        }
    }
}

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

fn parse_input(input: &str) -> (Vec<String>, String) {
    let mut warehouse = Vec::new();
    let mut moves = String::new();
    let mut reading_moves = false;

    for line in input.lines() {
        if line.is_empty() {
            reading_moves = true;
            continue;
        }
        if reading_moves {
            moves.push_str(&line);
        } else {
            warehouse.push(line.to_string());
        }
    }

    (warehouse, moves)
}

fn transform_warehouse(warehouse: Vec<String>) -> Vec<String> {
    let mut new_warehouse = Vec::new();
    for row in warehouse {
        let mut new_row = String::new();
        for ch in row.chars() {
            match ch {
                '#' => new_row.push_str("##"),
                'O' => new_row.push_str("[]"),
                '.' => new_row.push_str(".."),
                '@' => new_row.push_str("@."),
                _ => new_row.push(ch),
            }
        }
        new_warehouse.push(new_row);
    }
    new_warehouse
}

fn gps(map: &Map, c: char) -> i64 {
    map.find(c).iter().map(|p| p.0 + 100 * p.1).sum()
}

fn simulate(warehouse: Vec<String>, moves: String) -> i64 {
    let warehouse_grid: Vec<Vec<char>> =
        warehouse.iter().map(|row| row.chars().collect()).collect();
    let mut map = Map::from(warehouse_grid);
    let mut robot_pos = map.find('@').first().unwrap().clone();

    for mv in moves.chars() {
        let dir = Dir::from(mv);
        if can_step(&map, robot_pos, dir) {
            do_step(&mut map, robot_pos, dir);
            robot_pos = robot_pos.step(dir);
        }
    }

    gps(&map, '[')
}

fn main() {
    #[allow(unused_variables)]
    let start = std::time::Instant::now();
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let (warehouse, moves) = parse_input(&contents);
    let transformed_warehouse = transform_warehouse(warehouse);
    println!("[INPUT+PARSING] Elapsed time: {:?}", start.elapsed());
    let start = std::time::Instant::now();
    let gps_sum = simulate(transformed_warehouse, moves);
    println!("[Part2] Elapsed time: {:?}", start.elapsed());
    println!("Sum of GPS coordinates: {}", gps_sum);
}
