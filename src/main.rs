use clap::Parser;
use regex::Regex;
use utils::get_reg_number_from_name;
use std::fs::File;
use std::io::prelude::*;

mod utils;

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
        let encoded_instruction: String = decode_instruction(line).unwrap();
        println!("{}: {}", line, encoded_instruction);
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

    match instruction_format {
        Some('R') => {
            let reg = Regex::new(r"t\d+").unwrap();

            let result: Vec<String> = reg
                .find_iter(rest)
                .map(|mat| mat.as_str().to_string())
                .collect();

            if result.len() != 3 {
                panic!("Instruction format incorrect.");
            }

            else {
                let rd : u32 = utils::get_reg_number_from_name(&result[0]);
                let rs1 : u32 = utils::get_reg_number_from_name(&result[1]);
                let rs2 : u32 = utils::get_reg_number_from_name(&result[2]);

                let opcode = 0b0110011;

                let (funct7, funct3) = match mnemonic {
                    "add" => (0x00, 0x00),
                    "sub" => (0x20, 0x0),
                    "xor" => (0x00, 0x4),
                    "or" => (0x00, 0x6),
                    "and" => (0x00, 0x7),
                    "sll" => (0x00, 0x1),
                    "srl" => (0x00, 0x5),
                    "sra" => (0x20, 0x5),
                    "slt" => (0x00, 0x2),
                    "sltu" => (0x00, 0x3),
                    &_ => (0, 0)
                };

                // This will have to be returned
                let final_instruction = (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | (opcode);
                decoded_instruction = format!("{:08x}", final_instruction);
            }

        }
        Some('I') => {
            let reg = Regex::new(r"t\d+").unwrap();
            let inm = Regex::new(r"\b\d+\b").unwrap();

            let result: Vec<String> = reg
                .find_iter(rest)
                .map(|mat| mat.as_str().to_string())
                .collect();
        
            let inmediate : Vec<String> = inm
                .find_iter(rest)
                .map(|mat| mat.as_str().to_string())
                .collect();



            decoded_instruction = match (result.len(), inmediate.len()) {
                (2, 1) => {
                    let opcode = 0b0010011;
                    let rd = get_reg_number_from_name(&result[0]);
                    let rs1 = get_reg_number_from_name(&result[1]);
                    let inm = &inmediate[0].parse::<u32>().unwrap();

                    let funct3 = match mnemonic {
                            "add" => 0x00,
                            "sub" => 0x0,
                            "xor" => 0x4,
                            "or" => 0x6,
                            "and" => 0x7,
                            "sll" => 0x1,
                            "srl" => 0x5,
                            "sra" => 0x5,
                            "slt" => 0x2,
                            "sltu" => 0x3,
                            &_ => 0
                    };

                    let final_instruction = (inm << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | (opcode);
                    format!("{:08x}", final_instruction)
                }
                (_, _) => String::from("")
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

    Some(decoded_instruction)
}
