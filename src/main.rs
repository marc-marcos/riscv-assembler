use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

mod decoding;
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
        let encoded_instruction = decoding::decode_instruction(line).unwrap();
        println!("{}: {}", line, encoded_instruction);
    }
}
