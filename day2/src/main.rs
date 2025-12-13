use clap::Parser;
use rayon::prelude::*;
use std::fs::read_to_string;

#[derive(Debug, Parser)]
struct Args {
    input_path: String,
}

fn run_part1(input_path: String) -> u64 {
    let total: u64 = read_to_string(input_path)
        .expect("Failed to read input file")
        .trim()
        .split(",")
        .collect::<Vec<_>>()
        .par_iter()
        .fold(
            || 0,
            |acc, &range| {
                let mut parts = range.split('-');
                let start: u64 = parts.next().unwrap().parse().unwrap();
                let end: u64 = parts.next().unwrap().parse().unwrap();
                let mut count = 0;
                for i in start..end + 1 {
                    // println!("total: {}; i: {}", total, i);
                    let stri = format!("{}", i);
                    let stri_len = stri.len();
                    if stri_len % 2 != 0 {
                        continue;
                    } else if stri[..stri_len / 2] == stri[stri_len / 2..] {
                        count += i;
                    }
                }
                acc + count
            },
        )
        .sum();
    total
}

fn run_part2(input_path: String) -> u64 {
    let total: u64 = read_to_string(input_path)
        .expect("Failed to read input file")
        .trim()
        .split(",")
        .collect::<Vec<_>>()
        .par_iter()
        .fold(
            || 0,
            |acc, &range| {
                let mut parts = range.split('-');
                let start: u64 = parts.next().unwrap().parse().unwrap();
                let end: u64 = parts.next().unwrap().parse().unwrap();
                let mut count = 0;
                for i in start..end + 1 {
                    let s = format!("{}", i);
                    let t = format!("{}{}", s, s);
                    if t[1..t.len() - 1].contains(s.as_str()) {
                        count += i;
                    }
                }
                acc + count
            },
        )
        .sum();
    total
}

fn main() {
    let args = Args::parse();
    let result_part1 = run_part1(args.input_path.clone());
    let result_part2 = run_part2(args.input_path);

    println!("Result part1: {}", result_part1);
    println!("Result part2: {}", result_part2);
}

#[cfg(test)]
mod tests {
    use crate::{run_part1, run_part2};

    #[test]
    fn test_run_part1() {
        let path = "inputs/test-input.txt".to_string();
        assert_eq!(run_part1(path), 1227775554)
    }

    #[test]
    fn test_run_part2() {
        let path = "inputs/test-input.txt".to_string();
        assert_eq!(run_part2(path), 4174379265)
    }
}
