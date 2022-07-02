pub fn begin(part: u8, input: String) -> u32 {
    match part {
        1 => return part_1::begin(input),
        2 => return part_2::begin(input),
        _ => panic!("Invalid part entered.")
    }
}

mod part_1 {
    pub fn begin(input: String) -> u32 {
        let lines = input.lines().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        
        let mut count = 0;
        let mut prev = &lines[0];
        
        for line in &lines[1..] {
            if line > prev { count += 1 }
            prev = line;
        }

        count
    }
}


mod part_2 {
    pub fn begin(input: String) -> u32{
        let lines = input.lines().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        
        let mut count = 0;
        let mut prev: u32 = lines[0] + lines[1] + lines[2];
        let mut curr: u32 = lines[1] + lines[2];
        let mut next: u32 = lines[2];
        
        for line in &lines[3..] {
            curr += line;
            next += line;
            if curr > prev { count += 1 }
            prev = curr;
            curr = next;
            next = *line;
        }
        
        count
    }
}