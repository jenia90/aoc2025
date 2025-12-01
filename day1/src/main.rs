use clap::Parser;
use std::fs::read_to_string;

#[derive(Debug, Parser)]
struct Args {
    input_path: String
}



fn main() {
    let args = Args::parse();
    let input = read_to_string(&args.input_path).expect("Failed to read input file");
    let rotations = input.trim().lines().map(|line| match line.chars().nth(0) {
        Some('L') => -i32::from_str_radix(&line[1..], 10).expect("Failed to parse line"),
        Some('R') => i32::from_str_radix(&line[1..], 10).expect("Failed to parse line"),
        _ => panic!("Invalid direction"),
    });
    let mut count = 0;
    let mut state = 50;
    for rot in rotations {
        let old_state = state;
        count += rot.abs() / 100;

        state += rot % 100;

        if state < 0 {
            if old_state > 0 { count += 1}
            state = 100 + (state % 100);
        } else if state > 99 {
            if state > 100 {count += 1};
            state %= 100;
        }

        if state == 0 {
            count += 1;
        }
    }

    println!("Answer: {}", count);
}
