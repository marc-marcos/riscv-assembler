pub fn get_reg_number_from_name(name : &str) -> u32 {
    let ret = match name {
        "zero" => 0,
        "ra" => 1,
        "sp" => 2,
        "gp" => 3,
        "tp" => 4,
        "t0" => 5,
        "t1" => 6,
        "t2" => 7,
        "s0" => 8,
        "s1" => 9,
        "a0" => 10,
        "a1" => 11,
        "a2" => 12,
        "a3" => 13,
        "a4" => 14,
        "a5" => 15,
        "a6" => 16,
        "a7" => 17,
        "s2" => 18,
        "s3" => 19,
        "s4" => 20,
        "s5" => 21,
        "s6" => 22,
        "s7" => 23,
        "s8" => 24,
        "s9" => 25,
        "s10" => 26,
        "s11" => 27,
        "t3" => 28,
        "t4" => 29,
        "t5" => 30,
        "t6" => 31,
        _ => 0
    };

    ret
}

pub fn extract_single_bit(value : u32, bit_position : u32) -> u32 {
    (value >> bit_position) & 1
}

pub fn extract_range_bits(value : u32, start : u32, end : u32) -> u32 {
    if (end - start) == 31 {
        return value
    }

    let mask = (1 << (end - start + 1)) - 1; // Create a mask for the range
    (value >> start) & mask
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_single_bit() {
        let number_1 : u32 = 0b000;
        let number_2 : u32 = 0b001;
        let number_3 : u32 = 0b010;
        let number_4 : u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000;

        assert_eq!(extract_single_bit(number_1, 0), 0);
        assert_eq!(extract_single_bit(number_2, 0), 1);
        assert_eq!(extract_single_bit(number_3, 1), 1);
        assert_eq!(extract_single_bit(number_4, 31), 1);
    }

    #[test]
    fn test_extract_range_bits() {
        let number_1 : u32 = 0b0000_0000_0000_0000_0000_0000_0000_1001;
        let number_2 : u32 = 0b0000_0000_0000_0000_0000_0000_1111_0000;
        let number_3 : u32 = 0b0000_0000_0000_0000_0000_0000_0000_0000;
        let number_4 : u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

        assert_eq!(extract_range_bits(number_1, 0, 3), 0b1001);

        assert_eq!(extract_range_bits(number_2, 4, 7), 0b1111);

        assert_eq!(extract_range_bits(number_3, 0, 31), 0);

        assert_eq!(extract_range_bits(number_4, 0, 31), 4294967295);
    }
}