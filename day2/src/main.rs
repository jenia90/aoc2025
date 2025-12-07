use clap::Parser;
use std::fs::read_to_string;
use rayon::prelude::*;

#[derive(Debug, Parser)]
struct Args {
    input_path: String
}

fn process_range_part1(start: u64, end: u64) -> u64 {
    let mut total = 0;
    for i in start..end+1 {
        // println!("total: {}; i: {}", total, i);
        let stri = format!("{}", i);
        let stri_len = stri.len();
        if stri_len % 2 != 0 {
            continue;
        } else if stri[..stri_len / 2] == stri[stri_len / 2..] {
            total += i;
        }
    }
    total
}

fn process_range_part2(start: u64, end: u64) -> u64 {
    let mut total = 0;
    for i in start..end+1 {
        let s = format!("{}", i);
        let t = format!("{}{}", s, s);
        if t[1..t.len()-1].contains(s.as_str()) {
            total += i;
        }

    }
    total
}

fn main() {
    let args = Args::parse();
    let total: u64 = read_to_string(&args.input_path)
        .expect("Failed to read input file")
        .trim()
        .split(",")
        .collect::<Vec<_>>()
        .par_iter()
        .map(|&range| {
            let mut parts = range.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            process_range_part2(start, end)
        })
        .sum();
    println!("Total: {}", total);
}
