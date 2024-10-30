use clap::{Parser, Subcommand};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: String,
    output: String,
}

fn main() {
    let args = Args::parse();

    let mut input = File::open(&args.input).unwrap();
    let mut contents = String::new();

    input.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        // Decode instruction
        let mut encoded_instruction: String = decode_instruction(line).unwrap();
    }
}

fn decode_instruction(instruction: &str) -> Option<String> {
    let mut decoded_instruction: String = String::new();

    // Split the instruction, one item should be the left of the first space, the other item to the
    // right of the first space

    let (mnemonic, rest) = match instruction.find(' ') {
        Some(index) => instruction.split_at(index),
        None => panic!("Invalid instruction"),
    };

    // Get the format of the instruction

    let instruction_format = match mnemonic {
        "add" | "sub" | "xor" | "or" | "and" | "sll" | "srl" | "sra" | "slt" | "sltu" => Some('R'),
        "addi" | "xori" | "ori" | "andi" | "slli" | "srli" | "srai" | "slti" | "sltiu" | "lb"
        | "lh" | "lw" | "lbu" | "lhu" | "jalr" | "ecall" | "ebreak" => Some('I'),
        "sb" | "sh" | "sw" => Some('S'),
        "beq" | "bne" | "blt" | "bge" | "bltu" | "bgeu" => Some('B'),
        "jal" => Some('J'),
        "lui" | "auipc" => Some('U'),
        _ => None,
    };

    if instruction_format.is_none() {
        return None;
    }

    match instruction_format {
        Some('R') => {
            let reg = Regex::new(r"t\d+").unwrap();

            let result: Vec<String> = reg
                .find_iter(rest)
                .map(|mat| mat.as_str().to_string())
                .collect();

            for i in result {
                println!("Register: {:?}", i);
            }
        }
        Some('I') => {
            let reg = Regex::new(r"t\d+|\d+").unwrap();

            let result: Vec<String> = reg
                .find_iter(rest)
                .map(|mat| mat.as_str().to_string())
                .collect();

            for i in result {
                println!("Register: {:?}", i);
            }
        }
        Some('S') => {
            let reg = Regex::new(r"t\d+|\d+").unwrap();

            let result: Vec<String> = reg
                .find_iter(rest)
                .map(|mat| mat.as_str().to_string())
                .collect();

            for i in result {
                println!("Register: {:?}", i);
            }
        }
        Some('B') => {
            let reg = Regex::new(r"t\d+|\d+").unwrap();

            let result: Vec<String> = reg
                .find_iter(rest)
                .map(|mat| mat.as_str().to_string())
                .collect();

            for i in result {
                println!("Register: {:?}", i);
            }
        }
        Some('U') => {
            let reg = Regex::new(r"t\d+|\d+").unwrap();

            let result: Vec<String> = reg
                .find_iter(rest)
                .map(|mat| mat.as_str().to_string())
                .collect();

            for i in result {
                println!("Register: {:?}", i);
            }
        }

        None => {
            return None;
        }

        _ => {
            return None;
        }
    }

    println!("Instruction format: {:?}", instruction_format);
    println!("Rest: {:?}", rest);

    Some(decoded_instruction)
}
