use anyhow::Result;
use std::fs::File;
use std::io::Read;

/*
https://adventofcode.com/2024/day/5
*/

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

const INPUT_FILE: &str = "input.txt";

#[derive(Debug)]
struct OrderingRule {
    before: u32,
    after: u32,
}

#[derive(Debug)]
struct Sequence {
    elements: Vec<u32>,
}

#[derive(Debug)]
struct InputData {
    ordering_rules: Vec<OrderingRule>,
    sequences: Vec<Sequence>,
}

fn slice_before_index<T>(vec: &Vec<T>, index: usize) -> &[T] {
    let (before, _) = vec.split_at(index);
    before
}

fn get_after_rules(input: u32, ordering_rules: &[OrderingRule]) -> Vec<u32> {
    ordering_rules
        .iter()
        .filter_map(|rule| {
            if rule.before == input {
                Some(rule.after)
            } else {
                None
            }
        })
        .collect()
}

fn get_before_rules(input: u32, ordering_rules: &[OrderingRule]) -> Vec<u32> {
    ordering_rules
        .iter()
        .filter_map(|rule| {
            if rule.after == input {
                Some(rule.before)
            } else {
                None
            }
        })
        .collect()
}

fn get_middle_element<T>(vec: &Vec<T>) -> Option<&T> {
    if vec.is_empty() {
        None
    } else {
        let middle_index = vec.len() / 2;
        vec.get(middle_index)
    }
}

fn check_sequence(sequence: &Sequence, ordering_rules: &[OrderingRule]) -> bool {
    for i in 0..sequence.elements.len() {
        let elem = sequence.elements[i];
        let after_rules = get_after_rules(elem, ordering_rules);
        if after_rules.is_empty() {
            continue;
        }
        let before_rules = get_before_rules(elem, ordering_rules);
        if before_rules.is_empty() {
            continue;
        }

        let (vec_before, vec_after) = sequence.elements.split_at(i);
        //check if vec_after contains before
        for rule in before_rules {
            if vec_after.contains(&rule) {
                return false;
            }
        }
        for rule in after_rules {
            if vec_before.contains(&rule) {
                return false;
            }
        }
    }
    true
}

fn reorder_sequence(sequence: &mut Vec<u32>, ordering_rules: &[OrderingRule]) {
    sequence.sort_by(|a, b| {
        for rule in ordering_rules {
            if rule.before == *a && rule.after == *b {
                return std::cmp::Ordering::Less;
            } else if rule.before == *b && rule.after == *a {
                return std::cmp::Ordering::Greater;
            }
        }
        std::cmp::Ordering::Equal
    });
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let mut ordering_rules: Vec<OrderingRule> = Vec::new();
    let mut sequences: Vec<Sequence> = Vec::new();
    let mut is_reading_rules = true;

    for line in contents.lines() {
        if line.trim().is_empty() {
            is_reading_rules = false;
            continue;
        }

        if is_reading_rules {
            let parts: Vec<&str> = line.split("|").collect();
            if parts.len() == 2 {
                let before = parts[0].parse::<u32>().expect("Failed to convert before");
                let after = parts[1].parse::<u32>().expect("Failed to convert after");
                ordering_rules.push(OrderingRule { before, after });
            }
        } else {
            let elements: Vec<u32> = line
                .split(',')
                .map(|s| s.trim().parse::<u32>())
                .collect::<Result<Vec<u32>, _>>()
                .expect("Failed to convert elements");
            sequences.push(Sequence { elements });
        }
    }

    let mut input_data = InputData {
        ordering_rules,
        sequences,
    };
    let mut sum: u32 = 0;
    for sequence in &mut input_data.sequences {
        if !check_sequence(sequence, &input_data.ordering_rules) {
            reorder_sequence(&mut sequence.elements, &input_data.ordering_rules);
            if let Some(middle) = get_middle_element(&sequence.elements) {
                sum += *middle;
            }
        }
    }
    println!("Sum of middle elements: {}", sum);
}
