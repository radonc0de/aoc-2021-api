pub fn begin(part: u8, input: String) -> u32{
    match part {
        1 => return part_1::begin(input),
        2 => return part_2::begin(input),
        _ => panic!("Invalid part entered.")
    }
}

mod part_1 {
    pub fn begin(input: String) -> u32 {
        0
    }
}

mod part_2 {
    pub fn begin(input: String) -> u32 {
        0
    }
}