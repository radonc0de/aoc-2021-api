use std::env;
use std::fs;

mod day_1;
mod day_2;
mod day_5;
mod day_14;

fn main() {

    let args: Vec<String> = env::args().collect(); //arguments: day part input
    if args.len() != 4{ panic!("Please run in format: 'cargo run [DAY] [PART] [INPUT]'") }

    let day: u8 = args[1].parse::<u8>().expect("Day entered is not a number.");
    let part: u8 = args[2].parse::<u8>().expect("Part entered is not a number.");

    let input: String;
    match fs::read_to_string(&args[3]){
        Ok(str) => input = str,
        Err(e) => panic!("Error importing input: {}", e)
    };

    let ans: u32;
    match day {
        1 => ans = day_1::begin(part, input),
        2 => ans = day_2::begin(part, input),
        5 => ans = day_5::begin(part, input),
        14 => ans = day_14::begin(part, input),
        _ => panic!("Invalid day entered!")
    }
    
    println!("The answer to Day {}: Part {} is {}", day, part, ans);

}
