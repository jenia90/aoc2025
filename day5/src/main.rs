use clap::Parser;
use std::fs::read_to_string;

#[derive(Debug, Parser)]
struct Args {
    input_path: String,
}

fn merge_intervals(mut intervals: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if intervals.is_empty() {
        return intervals;
    }
    intervals.sort_unstable_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let mut merged = Vec::with_capacity(intervals.len());
    let mut current = intervals[0];

    for &(s, e) in &intervals[1..] {
        if s <= current.1 {
            if e > current.1 {
                current.1 = e;
            }
        } else {
            merged.push(current);
            current = (s, e);
        }
    }

    merged.push(current);
    merged
}

fn run_part1(input_path: String) -> u64 {
    let input_string = read_to_string(&input_path).expect("Failed to read input file");
    let lines = input_string.lines().map(|l| l.trim()).collect::<Vec<_>>();

    let sep = lines.iter().position(|l| l.is_empty());

    let interval_lines = match sep {
        Some(idx) => &lines[..idx],
        None => &lines[..],
    };

    let query_lines = match sep {
        Some(idx) => &lines[idx + 1..],
        None => &[][..],
    };

    let mut ranges = Vec::new();
    for &line in interval_lines {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split('-');
        let start: u64 = parts
            .next()
            .expect("missing start")
            .parse()
            .expect("invalid start");
        let end: u64 = parts
            .next()
            .expect("missing end")
            .parse()
            .expect("invalid end");
        // treat as inclusive
        ranges.push((start, end));
    }

    let merged = merge_intervals(ranges);

    let count = query_lines
        .iter()
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }
            Some(l.parse::<u64>().expect("invalid query number"))
        })
        .filter(|&x| merged.iter().any(|&(s, e)| s <= x && x <= e))
        .count();

    count as u64
}

fn run_part2(input_path: String) -> usize {
    let input_string = read_to_string(&input_path).expect("Failed to read input file");
    let lines = input_string.lines().map(|l| l.trim()).collect::<Vec<_>>();

    let sep = lines.iter().position(|l| l.is_empty());
    let interval_lines = match sep {
        Some(idx) => &lines[..idx],
        None => &lines[..],
    };

    let mut intervals = Vec::new();
    for &line in interval_lines {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split('-');
        let start: u64 = parts
            .next()
            .expect("missing start")
            .parse()
            .expect("invalid start");
        let end: u64 = parts
            .next()
            .expect("missing end")
            .parse()
            .expect("invalid end");
        intervals.push((start, end));
    }

    let merged = merge_intervals(intervals);

    let total: u64 = merged
        .into_iter()
        .map(|(s, e)| if e >= s { e - s + 1 } else { 0 })
        .sum();

    total as usize
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
        assert_eq!(run_part1(path), 3)
    }

    #[test]
    fn test_run_part2() {
        let path = "inputs/test-input.txt".to_string();
        assert_eq!(run_part2(path), 14)
    }
}
