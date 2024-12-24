use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
/*
https://adventofcode.com/2024/day/24
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


#[derive(Debug, Clone)]
enum Gate {
    And(String, String, String),  // input1, input2, output
    Or(String, String, String),
    Xor(String, String, String),
}

fn parse_input(contents: &str) -> (HashMap<String, bool>, Vec<Gate>) {
    let mut initial_values = HashMap::new();
    let mut gates = Vec::new();

    let parts: Vec<&str> = contents.split("\n\n").collect();

    // Parse initial values
    for line in parts[0].lines() {
        if line.trim().is_empty() { continue; }
        let parts: Vec<&str> = line.split(':').collect();
        let wire = parts[0].trim().to_string();
        let value = parts[1].trim() == "1";
        initial_values.insert(wire, value);
    }

    // Parse gates
    for line in parts[1].lines() {
        if line.trim().is_empty() { continue; }
        let parts: Vec<&str> = line.split("->").collect();
        let inputs = parts[0].trim();
        let output = parts[1].trim().to_string();

        let input_parts: Vec<&str> = inputs.split_whitespace().collect();
        let input1 = input_parts[0].to_string();
        let gate_type = input_parts[1];
        let input2 = input_parts[2].to_string();

        let gate = match gate_type {
            "AND" => Gate::And(input1, input2, output),
            "OR" => Gate::Or(input1, input2, output),
            "XOR" => Gate::Xor(input1, input2, output),
            _ => panic!("Unknown gate type: {}", gate_type),
        };

        gates.push(gate);
    }

    (initial_values, gates)
}

fn simulate_circuit(initial_values: &HashMap<String, bool>, gates: &[Gate]) -> HashMap<String, bool> {
    let mut values = initial_values.clone();
    let mut changed = true;

    while changed {
        changed = false;
        for gate in gates {
            match gate {
                Gate::And(in1, in2, out) => {
                    if let (Some(&val1), Some(&val2)) = (values.get(in1), values.get(in2)) {
                        if !values.contains_key(out) {
                            values.insert(out.clone(), val1 && val2);
                            changed = true;
                        }
                    }
                }
                Gate::Or(in1, in2, out) => {
                    if let (Some(&val1), Some(&val2)) = (values.get(in1), values.get(in2)) {
                        if !values.contains_key(out) {
                            values.insert(out.clone(), val1 || val2);
                            changed = true;
                        }
                    }
                }
                Gate::Xor(in1, in2, out) => {
                    if let (Some(&val1), Some(&val2)) = (values.get(in1), values.get(in2)) {
                        if !values.contains_key(out) {
                            values.insert(out.clone(), val1 != val2);
                            changed = true;
                        }
                    }
                }
            }
        }
    }

    values
}

fn calculate_result(values: &HashMap<String, bool>) -> u64 {
    let mut z_wires: Vec<(String, bool)> = values
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .map(|(k, &v)| (k.clone(), v))
        .collect();

    // Sort by wire number to ensure correct bit order
    z_wires.sort_by(|a, b| {
        let a_num = a.0[1..].parse::<u32>().unwrap_or(0);
        let b_num = b.0[1..].parse::<u32>().unwrap_or(0);
        b_num.cmp(&a_num)  // Changed from a_num.cmp(&b_num) to reverse the order
    });

    // For debugging
    println!("Ordered z-wires:");
    for (wire, value) in &z_wires {
        println!("{}: {}", wire, value);
    }

    let mut result = 0;
    for (_, value) in z_wires {
        result = (result << 1) | (value as u64);
    }

    result
}


fn main() {
    let contents = read_input(INPUT_FILE).unwrap();
    let start = Instant::now();
    let (initial_values, gates) = parse_input(&contents);
    let parsing_duration = start.elapsed();
    let final_values = simulate_circuit(&initial_values, &gates);
    let simulation_duration = start.elapsed() - parsing_duration;
    let result = calculate_result(&final_values);
    let calculation_duration = start.elapsed() - simulation_duration;

    println!("Result: {}", result);
    println!("Durations:");
    println!("  Parsing:      {:?}", parsing_duration);
    println!("  Simulation:   {:?}", simulation_duration);
    println!("  Calculation:  {:?}", calculation_duration);
}
