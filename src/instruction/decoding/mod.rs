use regex::Regex;
use crate::utils;

pub mod test;

pub fn decode_instruction(instruction: &str) -> Option<String> {
    let decoded_instruction: String;

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
            let reg = Regex::new(r"s\d+|a\d+|t\d+").unwrap();

            let result: Vec<String> = reg
                .find_iter(rest)
                .map(|mat| mat.as_str().to_string())
                .collect();

            if result.len() != 3 {
                dbg!(result);
                dbg!(instruction);
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
            let reg = Regex::new(r"s\d+|a\d+|t\d+").unwrap();
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
                    let opcode = match mnemonic {
                        "lb" | "lh" | "lw" | "lbu" | "lhu" => 0b0000011,
                        &_ => 0b0010011 
                    };

                    let rd = utils::get_reg_number_from_name(&result[0]);
                    let rs1 = utils::get_reg_number_from_name(&result[1]);

                    let inm = match mnemonic {
                        "slli" | "srli" => &utils::extract_range_bits(inmediate[0].parse::<u32>().unwrap(), 0, 4),
                        "srai" => &((0x20 << 5) | utils::extract_range_bits(inmediate[0].parse::<u32>().unwrap(), 0, 4)),
                        &_ => &inmediate[0].parse::<u32>().unwrap()
                    };

                    let funct3 = match mnemonic {
                            "addi" | "lb" => 0x0,
                            "xori" | "lbu" => 0x4,
                            "ori" => 0x6,
                            "andi" => 0x7,
                            "slli" | "lh" => 0x1,
                            "srli" | "lhu" => 0x5,
                            "srai" => 0x5,
                            "slti" | "lw" => 0x2,
                            "sltiu" => 0x3,
                            &_ => 0
                    };

                    let final_instruction = (inm << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode;
                    format!("{:08x}", final_instruction)
                }
                (_, _) => String::from("")
            }
        }
        Some('S') => {
            let reg = Regex::new(r"s\d+|a\d+|t\d+|\d+").unwrap();

            let result: Vec<String> = reg
                .find_iter(rest)
                .map(|mat| mat.as_str().to_string())
                .collect();

            let imm = &result[1].parse::<u32>().unwrap();
            let rs1 = utils::get_reg_number_from_name(&result[2]);
            let rs2 = utils::get_reg_number_from_name(&result[0]);

            let funct3 = match mnemonic {
                "sb" => 0,
                "sh" => 1,
                "sw" => 2,
                _ => 0
            };

            let final_instruction = (utils::extract_range_bits(*imm, 5, 11) << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (utils::extract_range_bits(*imm, 0, 4) << 7) | 0b0100011;
            decoded_instruction = format!("{:08x}", final_instruction);
        }
        Some('B') => {
            let reg = Regex::new(r"s\d+|a\d+|t\d+|\d+").unwrap();

            let result: Vec<String> = reg
                .find_iter(rest)
                .map(|mat| mat.as_str().to_string())
                .collect();
        
            let rs1 = utils::get_reg_number_from_name(&result[0]); 
            let rs2 = utils::get_reg_number_from_name(&result[1]); 
            let imm = &result[2].parse::<u32>().unwrap();

            let funct3 = match mnemonic {
                "beq" => 0,
                "bne" => 1,
                "blt" => 4,
                "bge" => 5,
                "bltu" => 6,
                "bgeu" => 7,
                _ => 0
            };

            let final_instruction = (utils::extract_single_bit(*imm, 12) << 31) | (utils::extract_range_bits(*imm, 5, 10) << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (utils::extract_range_bits(*imm, 1, 4) << 8) | (utils::extract_single_bit(*imm, 11) << 7) | 0b1100011;
            decoded_instruction = format!("{:08x}", final_instruction);
        }
        Some('U') => {
            let reg = Regex::new(r"s\d+|a\d+|t\d+|\d+").unwrap();

            let result: Vec<String> = reg
                .find_iter(rest)
                .map(|mat| mat.as_str().to_string())
                .collect();

            let rd = utils::get_reg_number_from_name(&result[0]);
            let imm = &result[1].parse::<u32>().unwrap();

            let opcode = match mnemonic {
                "lui" => 0b0110111,
                "auipc" => 0b0010111,
                _ => 0b0000000,
            };

            let final_instruction = utils::extract_range_bits(*imm, 12, 31) << 12 | rd << 7 | opcode;
            decoded_instruction = format!("{:08x}", final_instruction);
        }

        Some('J') => {
            let reg = Regex::new(r"s\d+|a\d+|t\d+|\d+").unwrap();

            let result: Vec<String> = reg
                .find_iter(rest)
                .map(|mat| mat.as_str().to_string())
                .collect();

            let rd = utils::get_reg_number_from_name(&result[0]);
            let imm = &result[1].parse::<u32>().unwrap();

            let final_instruction = (utils::extract_single_bit(*imm, 20) << 31) | (utils::extract_range_bits(*imm, 1, 10) << 21) | (utils::extract_single_bit(*imm, 11) << 20) | (utils::extract_range_bits(*imm, 12, 19) << 12) | (rd << 7) | 0b1101111;
            decoded_instruction = format!("{:08x}", final_instruction);
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