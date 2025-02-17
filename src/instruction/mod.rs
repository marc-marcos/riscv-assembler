pub mod decoding;

struct Instruction {
    reads: Option<Vec<u8>>,
    writes: Option<Vec<u8>>,
    binary_translation : String
}