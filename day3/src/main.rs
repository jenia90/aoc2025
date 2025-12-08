use clap::Parser;
use rayon::prelude::*;
use std::fs::read_to_string;

#[derive(Debug, Parser)]
struct Args {
    input_path: String,
    num_batteries: usize,
}

fn get_joltage(line: &str, num_batteries: usize) -> u64 {
    let values: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut stack: Vec<u64> = Vec::new();
    let mut remove = values.len() - num_batteries;
    for value in values {
        while let Some(_) = stack.pop_if(|&mut v| v < value.into() && remove > 0) {
            remove -= 1;
        }
        stack.push(value.into());
    }

    stack
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join("")[..num_batteries]
        .parse()
        .unwrap()
}

fn main() {
    let args = Args::parse();
    let total: u64 = read_to_string(&args.input_path)
        .expect("Failed to read input file")
        .trim()
        .lines()
        .collect::<Vec<_>>()
        .par_iter()
        .map(|&line| get_joltage(&line, args.num_batteries))
        .sum();

    println!("Total: {}", total);
}
