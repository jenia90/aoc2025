use clap::Parser;
use rayon::prelude::*;
use std::fs::read_to_string;

#[derive(Debug, Parser)]
struct Args {
    input_path: String,
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

fn run_part1(input_path: String) -> u64 {
    read_to_string(input_path)
        .expect("Failed to read input file")
        .trim()
        .lines()
        .collect::<Vec<_>>()
        .par_iter()
        .map(|&line| get_joltage(&line, 2))
        .sum()
}

fn run_part2(input_path: String) -> u64 {
    read_to_string(input_path)
        .expect("Failed to read input file")
        .trim()
        .lines()
        .collect::<Vec<_>>()
        .par_iter()
        .map(|&line| get_joltage(&line, 12))
        .sum()
}

fn main() {
    let args = Args::parse();
    let result_part1 = run_part1(args.input_path.clone());
    let result_part2 = run_part2(args.input_path.clone());

    println!("Result part1: {}", result_part1);
    println!("Result part2: {}", result_part2);
}

#[cfg(test)]
mod tests {
    use crate::{run_part1, run_part2};

    #[test]
    fn test_run_part1() {
        let path = "inputs/test-input.txt".to_string();
        assert_eq!(run_part1(path), 357)
    }

    #[test]
    fn test_run_part2() {
        let path = "inputs/test-input.txt".to_string();
        assert_eq!(run_part2(path), 3121910778619)
    }
}
