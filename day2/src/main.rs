use clap::Parser;
use std::fs::read_to_string;

#[derive(Debug, Parser)]
struct Args {
    input_path: String
}

fn process_range(start: u64, end: u64) -> u64 {
    let mut total = 0;
    for i in start..end+1 {
        // println!("total: {}; i: {}", total, i);
        let stri = format!("{}", i);
        let stri_len = stri.len();
        if stri_len % 2 != 0 {
            continue;
        } else if stri[..stri_len / 2] == stri[(stri_len / 2)..] {
            total += i;
        }
    }
    total
}

fn main() {
    let args = Args::parse();
    let ranges: Vec<(u64, u64)> = read_to_string(&args.input_path)
        .expect("Failed to read input file")
        .trim()
        .split(",")
        .map(|range| {
            let mut parts = range.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            println!("start: {}; end: {}", start, end);
            (start, end)
        })
        .collect();

    let mut total = 0;
    for (start, end) in ranges.iter() {
        // println!("Range: {}-{}", start, end);
        total += process_range(*start, *end);
    }
    println!("Total: {}", total);
}
