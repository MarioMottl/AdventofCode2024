use anyhow::Result;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
const INPUT_FILE: &str = "input.txt";
#[allow(dead_code)]
const EXAMPLE_FILE: &str = "example.txt";

const DIRECTIONAL_PAD: &str = "X^A<v>";
const NUMERIC_PAD: &str = "789456123X0A";
const MAX_VALUE: i64 = i64::MAX;

struct Visit {
    row: i32,
    col: i32,
    presses: String,
}

fn read_input(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


fn hash(curr_r: i32, curr_c: i32, dest_r: i32, dest_c: i32, nrobots: i32) -> u64 {
    let mut result = curr_r as u64;
    result *= 4;
    result += curr_c as u64;
    result *= 4;
    result += dest_r as u64;
    result *= 4;
    result += dest_c as u64;
    result *= 30;
    result += nrobots as u64;
    result
}

fn cheapest_robot(presses: &str, nrobots: i32, memo: &mut HashMap<u64, i64>) -> i64 {
    if nrobots == 1 {
        return presses.len() as i64;
    }

    let mut result = 0;
    let mut curr_r = 0;
    let mut curr_c = 2;

    for press in presses.chars() {
        for next_r in 0..2 {
            for next_c in 0..3 {
                if DIRECTIONAL_PAD.chars().nth(next_r * 3 + next_c).unwrap() == press {
                    result += cheapest_dir_pad(curr_r, curr_c, next_r.try_into().unwrap(), next_c.try_into().unwrap(), nrobots, memo);
                    curr_r = next_r as i32;
                    curr_c = next_c as i32;
                }
            }
        }
    }
    result
}

fn cheapest_dir_pad(curr_r: i32, curr_c: i32, dest_r: i32, dest_c: i32, nrobots: i32, memo: &mut HashMap<u64, i64>) -> i64 {
    let h = hash(curr_r, curr_c, dest_r, dest_c, nrobots);
    if let Some(&result) = memo.get(&h) {
        return result;
    }

    let mut answer = MAX_VALUE;
    let mut queue = VecDeque::new();
    queue.push_back(Visit {
        row: curr_r,
        col: curr_c,
        presses: String::new(),
    });

    while let Some(v) = queue.pop_front() {
        if v.row == dest_r && v.col == dest_c {
            let mut new_presses = v.presses.clone();
            new_presses.push('A');
            let rec = cheapest_robot(&new_presses, nrobots - 1, memo);
            answer = answer.min(rec);
            continue;
        }

        if v.row == 0 && v.col == 0 {
            continue;
        }

        if v.row < dest_r {
            let mut new_presses = v.presses.clone();
            new_presses.push('v');
            queue.push_back(Visit {
                row: v.row + 1,
                col: v.col,
                presses: new_presses,
            });
        } else if v.row > dest_r {
            let mut new_presses = v.presses.clone();
            new_presses.push('^');
            queue.push_back(Visit {
                row: v.row - 1,
                col: v.col,
                presses: new_presses,
            });
        }

        if v.col < dest_c {
            let mut new_presses = v.presses.clone();
            new_presses.push('>');
            queue.push_back(Visit {
                row: v.row,
                col: v.col + 1,
                presses: new_presses,
            });
        } else if v.col > dest_c {
            let mut new_presses = v.presses.clone();
            new_presses.push('<');
            queue.push_back(Visit {
                row: v.row,
                col: v.col - 1,
                presses: new_presses,
            });
        }
    }
    memo.insert(h, answer);
    answer
}

fn cheapest(curr_r: i32, curr_c: i32, dest_r: i32, dest_c: i32, memo: &mut HashMap<u64, i64>, layers: i32) -> i64 {
    let mut answer = MAX_VALUE;
    let mut queue = VecDeque::new();
    queue.push_back(Visit {
        row: curr_r,
        col: curr_c,
        presses: String::new(),
    });

    while let Some(v) = queue.pop_front() {
        if v.row == dest_r && v.col == dest_c {
            let mut new_presses = v.presses.clone();
            new_presses.push('A');
            let rec = cheapest_robot(&new_presses, layers, memo);
            answer = answer.min(rec);
            continue;
        }

        if v.row == 3 && v.col == 0 {
            continue;
        }

        if v.row < dest_r {
            let mut new_presses = v.presses.clone();
            new_presses.push('v');
            queue.push_back(Visit {
                row: v.row + 1,
                col: v.col,
                presses: new_presses,
            });
        } else if v.row > dest_r {
            let mut new_presses = v.presses.clone();
            new_presses.push('^');
            queue.push_back(Visit {
                row: v.row - 1,
                col: v.col,
                presses: new_presses,
            });
        }

        if v.col < dest_c {
            let mut new_presses = v.presses.clone();
            new_presses.push('>');
            queue.push_back(Visit {
                row: v.row,
                col: v.col + 1,
                presses: new_presses,
            });
        } else if v.col > dest_c {
            let mut new_presses = v.presses.clone();
            new_presses.push('<');
            queue.push_back(Visit {
                row: v.row,
                col: v.col - 1,
                presses: new_presses,
            });
        }
    }
    answer
}

fn main() {
    let contents = read_input(INPUT_FILE).expect("Failed to read input");
    let mut sum = 0i64;
    let mut memo = HashMap::new();

    for line in contents.lines().filter(|l| !l.is_empty()) {
        let mut result = 0i64;
        let mut curr_r = 3;
        let mut curr_c = 2;

        let code: i64 = line[..3].parse().expect("Failed to parse code");

        for ch in line.chars() {
            for next_r in 0..4 {
                for next_c in 0..3 {
                    if NUMERIC_PAD.chars().nth(next_r * 3 + next_c).unwrap() == ch {
                        result += cheapest(curr_r, curr_c, next_r.try_into().unwrap(), next_c.try_into().unwrap(), &mut memo, 26);
                        curr_r = next_r as i32;
                        curr_c = next_c as i32;
                    }
                }
            }
        }

        sum += result * code;
        println!("{} {}", code, result);
        println!("Sum: {}", sum);
    }
}
