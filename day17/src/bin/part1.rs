use anyhow::Result;
use std::fs::File;
use std::io::Read;

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
    a: i32,
    b: i32,
    c: i32,
}

impl Registers {
    fn new(a: i32, b: i32, c: i32) -> Self {
        Self { a, b, c }
    }
}

#[derive(Debug)]
enum Operand {
    Literal(i32),
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

fn get_operand_value(operand: &Operand, registers: &Registers) -> i32 {
    match operand {
        Operand::Literal(value) => *value,
        Operand::Combo(value) => match value {
            0..=3 => *value as i32,
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
                let denominator = 2_i32.pow(get_operand_value(&op, registers) as u32);
                registers.a /= denominator;
            }
            Instruction::Bxl(value) => {
                registers.b ^= value;
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
                let denominator = 2_i32.pow(get_operand_value(&op, registers) as u32);
                registers.b = registers.a / denominator;
            }
            Instruction::Cdv(op) => {
                let denominator = 2_i32.pow(get_operand_value(&op, registers) as u32);
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


fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let Ok((mut registers, program)) = parse_input(&contents) else { panic!("Couldnt parse input!!!") };
    let output = run_program(&mut registers, &program);
    println!("Output: {}", output);
}
