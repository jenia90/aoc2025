use clap::Parser;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

#[derive(Debug, Parser)]
struct Args {
    input_path: String,
}

fn run_part1(input_path: String) -> u64 {
    let input = read_to_string(input_path).expect("Failed to read input file");
    let lines = input.lines().collect::<Vec<_>>();

    let start_col = lines[0].find("S").unwrap();
    let mut current_beams: HashSet<usize> = HashSet::new();
    current_beams.insert(start_col);

    let mut total = 0;
    for line in lines.iter().skip(1) {
        let mut next_beams: HashSet<usize> = HashSet::new();
        for &beam_col in current_beams.iter() {
            if beam_col < line.len() && line.chars().nth(beam_col).unwrap() == '^' {
                total += 1;
                if beam_col > 0 {
                    next_beams.insert(beam_col - 1);
                }

                if beam_col < line.len() - 1 {
                    next_beams.insert(beam_col + 1);
                }
            } else {
                next_beams.insert(beam_col);
            }
        }
        current_beams = next_beams;
    }
    total
}

fn run_part2(input_path: String) -> u64 {
    let input = read_to_string(input_path).expect("Failed to read input file");
    let lines = input.lines().collect::<Vec<_>>();

    let start_col = lines[0].find("S").unwrap();
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    fn count_paths(
        row: usize,
        col: usize,
        lines: &Vec<&str>,
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if row >= lines.len() - 1 {
            return 1;
        }

        if let Some(&count) = memo.get(&(row, col)) {
            return count;
        }

        let next_row = row + 1;
        let next_line = lines[next_row];

        let total = if col < next_line.len() && next_line.chars().nth(col).unwrap() == '^' {
            let mut sum = 0;
            if col > 0 {
                sum += count_paths(next_row, col - 1, lines, memo);
            }
            if col < next_line.len() - 1 {
                sum += count_paths(next_row, col + 1, lines, memo);
            }
            sum
        } else {
            count_paths(next_row, col, lines, memo)
        };

        memo.insert((row, col), total);
        total
    }

    count_paths(0, start_col, &lines, &mut memo) as u64
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
        assert_eq!(run_part1(path), 21)
    }

    #[test]
    fn test_run_part2() {
        let path = "inputs/test-input.txt".to_string();
        assert_eq!(run_part2(path), 40)
    }
}
