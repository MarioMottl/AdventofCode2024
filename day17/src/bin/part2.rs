use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
/*
https://adventofcode.com/2024/day/17
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

#[derive(Debug)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

impl Registers {
    fn new(a: u64, b: u64, c: u64) -> Self {
        Self { a, b, c }
    }
}

#[derive(Debug)]
enum Operand {
    Literal(u64),
    Combo(usize),
}

#[derive(Debug)]
enum Instruction {
    Adv(Operand),
    Bxl(i32),
    Bst(Operand),
    Jnz(i32),
    Bxc,
    Out(Operand),
    Bdv(Operand),
    Cdv(Operand),
}

impl Instruction {
    fn from_opcode(opcode: u8, operand: u8) -> Self {
        match opcode {
            0 => Instruction::Adv(Operand::Combo(operand as usize)),
            1 => Instruction::Bxl(operand as i32),
            2 => Instruction::Bst(Operand::Combo(operand as usize)),
            3 => Instruction::Jnz(operand as i32),
            4 => Instruction::Bxc,
            5 => Instruction::Out(Operand::Combo(operand as usize)),
            6 => Instruction::Bdv(Operand::Combo(operand as usize)),
            7 => Instruction::Cdv(Operand::Combo(operand as usize)),
            _ => panic!("Invalid opcode"),
        }
    }
}

fn get_operand_value(operand: &Operand, registers: &Registers) -> u64 {
    match operand {
        Operand::Literal(value) => *value,
        Operand::Combo(value) => match value {
            0..=3 => *value as u64,
            4 => registers.a,
            5 => registers.b,
            6 => registers.c,
            _ => panic!("Invalid combo operand"),
        },
    }
}


fn run_program(registers: &mut Registers, program: &[u8]) -> String {
    let mut output = Vec::new();
    let mut instruction_pointer = 0;

    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];
        let instruction = Instruction::from_opcode(opcode, operand);

        match instruction {
            Instruction::Adv(op) => {
                let value = get_operand_value(&op, registers);
                if value == 0 {
                    panic!("Attempt to divide by zero");
                }
                let denominator = 2_u64.pow(value as u32);
                registers.a /= denominator;
            }
            Instruction::Bxl(value) => {
                registers.b ^= value as u64;
            }
            Instruction::Bst(op) => {
                registers.b = get_operand_value(&op, registers) % 8;
            }
            Instruction::Jnz(value) => {
                if registers.a != 0 {
                    instruction_pointer = value as usize;
                    continue;
                }
            }
            Instruction::Bxc => {
                registers.b ^= registers.c;
            }
            Instruction::Out(op) => {
                output.push((get_operand_value(&op, registers) % 8).to_string());
            }
            Instruction::Bdv(op) => {
                let value = get_operand_value(&op, registers);
                // maybe before we divide with zero we return a joined string that is u32::MAX or something???
                if value == 0 {
                    panic!("Attempt to divide by zero");
                }
                let denominator = 2_u64.pow(value as u32);
                registers.b = registers.a / denominator;
            }
            Instruction::Cdv(op) => {
                let value = get_operand_value(&op, registers);
                // maybe before we divide with zero we return a joined string that is u32::MAX or something???
                let denominator = 2_u64.pow(value as u32);
                registers.c = registers.a / denominator;
            }
        }

        instruction_pointer += 2;
    }

    output.join(",")
}


fn parse_input(input: &str) -> Result<(Registers, Vec<u8>), &'static str> {
    let mut lines = input.lines();
    let a = lines.next().ok_or("Missing Register A")?.split(": ").nth(1).ok_or("Invalid format for Register A")?.parse().map_err(|_| "Invalid value for Register A")?;
    let b = lines.next().ok_or("Missing Register B")?.split(": ").nth(1).ok_or("Invalid format for Register B")?.parse().map_err(|_| "Invalid value for Register B")?;
    let c = lines.next().ok_or("Missing Register C")?.split(": ").nth(1).ok_or("Invalid format for Register C")?.parse().map_err(|_| "Invalid value for Register C")?;
    let registers = Registers::new(a, b, c);

    // Skip the empty line
    lines.next();

    let program = lines
        .next()
        .ok_or("Missing Program")?
        .split(": ")
        .nth(1)
        .ok_or("Invalid format for Program")?
        .split(',')
        .map(|s| s.parse().map_err(|_| "Invalid value in Program"))
        .collect::<Result<Vec<u8>, _>>()?;

    Ok((registers, program))
}

//This will probably not work in human time :(
fn find_initial_value(program: &[u8], register_a: u64) -> u64 {
    let mut registers = Registers::new(0, 0, 0);
    let mut initial_value = register_a;

    loop {
        println!("Trying out: {}", initial_value);
        registers.a = initial_value;
        let output = run_program(&mut registers, program);
        let expected_output: String = program.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(",");

        if output == expected_output {
            return initial_value;
        }

        initial_value += 1;
    }
}

//MORE POWER!!!!!!!!!
fn find_initial_value_multithreaded(program: &[u8], start_value: u64, num_threads: usize) -> u64 {
    let program = Arc::new(program.to_vec());
    let result = Arc::new(Mutex::new(None));

    let mut handles = vec![];

    for i in 0..num_threads {
        let program = Arc::clone(&program);
        let result = Arc::clone(&result);
        let start = start_value + i as u64;

        let handle = thread::spawn(move || {
            let mut initial_value = start;
            let multiplier = 8u64.pow(i as u32);

            loop {
                {
                    let result = result.lock().unwrap();
                    if result.is_some() {
                        break;
                    }
                }

                let output = run_simplified(initial_value);
                let expected_output: String = program.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(",");

                if output == expected_output {
                    let mut result = result.lock().unwrap();
                    *result = Some(initial_value);
                    break;
                }

                // Increment the initial value by the multiplier
                initial_value += multiplier;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = result.lock().unwrap();
    result.unwrap()
}


fn get_out(a: u64) -> u64 {
    let partial = (a % 8) ^ 2;
    ((partial ^ (a >> partial)) ^ 7) % 8
}

fn run_simplified(a: u64) -> String {
    let mut out = Vec::new();
    let mut a = a;
    while a > 0 {
        out.push(get_out(a));
        a >>= 3;
    }
    out.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(",")
}

fn find_initial_value_yet_another(program: &[u8]) -> Result<u64, &'static str> {
    fn get_best_input_maybe(program: &[u8], cursor: usize, waterlevel: u64) -> Option<u64> {
        for candidate in 0..8 {
            let initial_value = waterlevel * 8 + candidate;
            let mut registers = Registers::new(initial_value, 0, 0);
            if run_program(&mut registers, program) == program[cursor..].iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(",") {
                if cursor == 0 {
                    return Some(initial_value);
                }
                if let Some(ret) = get_best_input_maybe(program, cursor - 1, initial_value) {
                    return Some(ret);
                }
            }
        }
        None
    }

    get_best_input_maybe(program, program.len() - 1, 0).ok_or("No valid initial value found")
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let Ok((mut registers, program)) = parse_input(&contents) else { panic!("Couldnt parse input!!!") };
    let start = Instant::now();
    let initial_value = find_initial_value_yet_another(&program);
    let duration = start.elapsed();
    match initial_value {
        Ok(value) => println!("{}", value),
        Err(err) => println!("Error: {}", err),
    }
    println!("Part2 took: {:?}", duration);
}
