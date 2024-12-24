use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

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
struct Circuit {
    gates: Vec<(String, String, String, String)>, // (wire1, op, wire2, output)
}

impl Circuit {
    fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.split("\n\n").collect();
        let mut gates = Vec::new();

        for line in parts[1].lines() {
            if line.trim().is_empty() { continue; }
            let parts: Vec<&str> = line.split(" -> ").collect();
            let inputs = parts[0].trim();
            let output = parts[1].trim().to_string();

            let input_parts: Vec<&str> = inputs.split_whitespace().collect();
            let wire1 = input_parts[0].to_string();
            let op = input_parts[1].to_string();
            let wire2 = input_parts[2].to_string();

            gates.push((wire1, op, wire2, output));
        }

        Circuit { gates }
    }

    fn find_gate(&self, a: &str, b: &str, op: &str) -> Option<String> {
        for (wire1, gate_op, wire2, output) in &self.gates {
            if (wire1 == a && wire2 == b || wire1 == b && wire2 == a) && gate_op == op {
                return Some(output.clone());
            }
        }
        None
    }

    fn find_swaps(&self) -> Vec<String> {
        let mut swapped = HashSet::new();
        let mut carry: Option<String> = None;

        // Check each bit position 0-44
        for i in 0..45 {
            let n = format!("{:02}", i);
            let x = format!("x{}", n);
            let y = format!("y{}", n);

            // Find half adder components
            let mut m1 = self.find_gate(&x, &y, "XOR");
            let mut n1 = self.find_gate(&x, &y, "AND");

            // Process carry chain if we have previous carry
            if let Some(c0) = &carry {
                if let (Some(m1_val), Some(n1_val)) = (&m1, &n1) {
                    let mut r1 = self.find_gate(c0, m1_val, "AND");

                    // If no r1 found, try swapping m1 and n1
                    if r1.is_none() {
                        swapped.insert(m1_val.clone());
                        swapped.insert(n1_val.clone());
                        std::mem::swap(&mut m1, &mut n1);
                        r1 = self.find_gate(c0, m1.as_ref().unwrap(), "AND");
                    }

                    let mut z1 = self.find_gate(c0, m1.as_ref().unwrap(), "XOR");

                    // Check and swap z-prefixed wires
                    if let Some(m1_val) = &m1 {
                        if m1_val.starts_with('z') {
                            swapped.insert(m1_val.clone());
                            swapped.insert(z1.as_ref().unwrap().clone());
                            std::mem::swap(&mut m1, &mut z1);
                        }
                    }

                    if let Some(n1_val) = &n1 {
                        if n1_val.starts_with('z') {
                            swapped.insert(n1_val.clone());
                            swapped.insert(z1.as_ref().unwrap().clone());
                            std::mem::swap(&mut n1, &mut z1);
                        }
                    }

                    if let Some(r1_val) = &r1 {
                        if r1_val.starts_with('z') {
                            swapped.insert(r1_val.clone());
                            swapped.insert(z1.as_ref().unwrap().clone());
                            std::mem::swap(&mut r1, &mut z1);
                        }
                    }

                    // Find carry out
                    let c1 = self.find_gate(r1.as_ref().unwrap(), n1.as_ref().unwrap(), "OR");

                    // Check if carry needs to be swapped
                    if let Some(c1_val) = &c1 {
                        if c1_val.starts_with('z') && c1_val != "z45" {
                            swapped.insert(c1_val.clone());
                            swapped.insert(z1.as_ref().unwrap().clone());
                            carry = Some(z1.unwrap());
                            continue;
                        }
                    }
                    carry = c1;
                } else {
                    carry = None;
                }
            } else {
                // First bit - carry is just the AND gate output
                carry = n1;
            }
        }

        let mut result: Vec<_> = swapped.into_iter().collect();
        result.sort();
        result
    }
}

fn main() {
    let contents = read_input(INPUT_FILE).unwrap();

    let start = Instant::now();
    let circuit = Circuit::new(&contents);
    let circuit_creation_duration = start.elapsed();

    let start = Instant::now();
    let swapped = circuit.find_swaps();
    let find_swaps_duration = start.elapsed();

    println!("Swapped wires: {}", swapped.join(","));
    println!("Durations:");
    println!("  Circuit creation: {:?}", circuit_creation_duration);
    println!("  Find swaps:       {:?}", find_swaps_duration);

}
