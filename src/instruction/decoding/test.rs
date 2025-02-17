use super::super::decoding;

#[test]
fn r_format_instructions() {
    assert_eq!(decoding::decode_instruction("add t0, t1, t2"), Some(String::from("007302b3")));
    assert_eq!(decoding::decode_instruction("sub a0, a1, a2"), Some(String::from("40c58533")));
    assert_eq!(decoding::decode_instruction("xor a3, a4, a5"), Some(String::from("00f746b3")));
    assert_eq!(decoding::decode_instruction("or a6, a7, s1"), Some(String::from("0098e833")));
    assert_eq!(decoding::decode_instruction("and s2, s3, s4"), Some(String::from("0149f933")));
    assert_eq!(decoding::decode_instruction("srl s5, s6, s7"), Some(String::from("017b5ab3")));
    assert_eq!(decoding::decode_instruction("sll s8, s9, s10"), Some(String::from("01ac9c33")));
    assert_eq!(decoding::decode_instruction("sra s11, t3, t4"), Some(String::from("41de5db3")));
    assert_eq!(decoding::decode_instruction("slt t5, t6, t0"), Some(String::from("005faf33")));
    assert_eq!(decoding::decode_instruction("sltu t0, t0, t0"), Some(String::from("0052b2b3")));
}

#[test]
fn i_format_arithmetic_instructions() {
    assert_eq!(decoding::decode_instruction("addi t0, t1, 8191"), Some(String::from("fff30293")));
    assert_eq!(decoding::decode_instruction("xori t2, a0, 0"), Some(String::from("00054393")));
    assert_eq!(decoding::decode_instruction("ori a1, a2, 0"), Some(String::from("00066593")));
    assert_eq!(decoding::decode_instruction("andi a3, a4, 8191"), Some(String::from("fff77693")));
    assert_eq!(decoding::decode_instruction("slli a6, a7, 0"), Some(String::from("00089813")));
    assert_eq!(decoding::decode_instruction("srli s1, s2, 31"), Some(String::from("01f95493")));
    assert_eq!(decoding::decode_instruction("srai s3, s4, 0"), Some(String::from("400a5993")));
    assert_eq!(decoding::decode_instruction("slti s5, s6, 31"), Some(String::from("01fb2a93")));
    assert_eq!(decoding::decode_instruction("sltiu s7, s8, 0"), Some(String::from("000c3b93")));
}

#[test]
fn i_format_memory_instructions() {
    assert_eq!(decoding::decode_instruction("lh a0, 0(t1)"), Some(String::from("00031503")));
    assert_eq!(decoding::decode_instruction("lb t0, 8191(t1)"), Some(String::from("fff30283")));
    assert_eq!(decoding::decode_instruction("lw s0, 8191(t1)"), Some(String::from("fff32403")));
    assert_eq!(decoding::decode_instruction("lbu t0, 0(t1)"), Some(String::from("00034283")));
    assert_eq!(decoding::decode_instruction("lhu t0, 8191(t1)"), Some(String::from("fff35283")));
}

#[test]
fn s_format_instructions() {
    assert_eq!(decoding::decode_instruction("sb t0, 8191(s1)"), Some(String::from("fe548fa3")));
    assert_eq!(decoding::decode_instruction("sh s0, 0(s1)"), Some(String::from("00849023")));
    assert_eq!(decoding::decode_instruction("sw a0, 0(a2)"), Some(String::from("00a62023")));
}

#[test]
fn b_format_instructions() {
    assert_eq!(decoding::decode_instruction("beq t0, t1, 8191"), Some(String::from("fe628fe3")));
    assert_eq!(decoding::decode_instruction("bne s0, s1, 8191"), Some(String::from("fe941fe3")));
    assert_eq!(decoding::decode_instruction("blt a0, a1, 0"), Some(String::from("00b54063")));
    assert_eq!(decoding::decode_instruction("bge t0, t1, 0"), Some(String::from("0062d063")));
    assert_eq!(decoding::decode_instruction("bltu t0, t1, 0"), Some(String::from("0062e063")));
    assert_eq!(decoding::decode_instruction("bgeu t0, t1, 0"), Some(String::from("0062f063")));
}

#[test]
fn j_format_instructions() {
    assert_eq!(decoding::decode_instruction("jal t0, 2097151"), Some(String::from("fffff2ef")));
    assert_eq!(decoding::decode_instruction("jal a0, 0"), Some(String::from("0000056f")));
    assert_eq!(decoding::decode_instruction("jal s0, 0"), Some(String::from("0000046f")));
}

#[test]
fn u_format_instructions() {
    assert_eq!(decoding::decode_instruction("lui t0, 4294967295"), Some(String::from("fffff2b7")));
    assert_eq!(decoding::decode_instruction("lui s0, 0"), Some(String::from("00000437")));
    assert_eq!(decoding::decode_instruction("lui a0, 0"), Some(String::from("00000537")));
}