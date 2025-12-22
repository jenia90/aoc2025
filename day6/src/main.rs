use clap::Parser;
use std::fs::read_to_string;

#[derive(Debug, Parser)]
struct Args {
    input_path: String,
}

fn run_part1(input_path: String) -> u64 {
    let input = read_to_string(input_path).expect("Failed to read input file");
    let lines = input.lines().collect::<Vec<_>>();
    let operators = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|v| match v {
            "*" => |a: u64, b: u64| a * b,
            "+" => |a: u64, b: u64| a + b,
            _ => panic!("Invalid operator"),
        })
        .collect::<Vec<_>>();

    let mut values = lines[..lines.len() - 1].iter().map(|&line| {
        line.split_whitespace()
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    });
    let first_row = values.next().unwrap();
    values
        .fold(first_row, |mut acc, row| {
            for (j, &v) in row.iter().enumerate() {
                let op = &operators[j];
                acc[j] = op(acc[j], v);
            }
            acc
        })
        .iter()
        .sum()
}

fn run_part2(input_path: String) -> u64 {
    let input = read_to_string(input_path).expect("Failed to read input file");
    let lines = input.lines().collect::<Vec<_>>();
    let operators = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|v| match v {
            "*" => |a: u64, b: u64| a * b,
            "+" => |a: u64, b: u64| a + b,
            _ => panic!("Invalid operator"),
        })
        .collect::<Vec<_>>();

    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let first_line = lines.first().unwrap();
    let mut current_col = Vec::new();
    for col in (0..(first_line.len())).rev() {
        let mut num = String::new();
        for row in 0..lines.len() - 1 {
            let c = lines[row].chars().nth(col).unwrap();
            if c == ' ' {
                continue;
            } else {
                num.push(c);
            }
        }
        if !num.is_empty() {
            current_col.push(num.parse().unwrap());
        } else {
            numbers.push(current_col.clone());
            current_col.clear();
        }

        if col == 0 {
            numbers.push(current_col.clone());
        }
    }
    let mut total = 0;
    numbers.iter().enumerate().for_each(|(i, vec)| {
        let op = operators[operators.len() - 1 - i];
        let mut vec = vec.iter();
        let first = *vec.next().unwrap();

        let current = vec.fold(first, |acc, &x| op(acc, x));
        total += current;
    });
    total
}

fn main() {
    let args = Args::parse();
    let result_part1 = run_part1(args.input_path.clone());
    let result_part2 = run_part2(args.input_path);

    println!("Part 1: {}", result_part1);
    println!("Part 2: {}", result_part2);
}

#[cfg(test)]
mod tests {
    use crate::{run_part1, run_part2};

    #[test]
    fn test_run_part1() {
        let path = "inputs/test-input.txt".to_string();
        assert_eq!(run_part1(path), 4277556)
    }

    #[test]
    fn test_run_part2() {
        let path = "inputs/test-input.txt".to_string();
        assert_eq!(run_part2(path), 3263827)
    }
}
