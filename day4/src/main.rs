use clap::Parser;
use std::fs::read_to_string;

#[derive(Debug, Parser)]
struct Args {
    input_path: String,
}

fn remove_rolls(map: Vec<Vec<char>>) -> Option<(Vec<Vec<char>>, u32)> {
    let n = map.len() as isize;
    let m = map[0].len() as isize;
    let mut count = 0;
    let mut new_map = map.clone();

    for r in 0..n {
        for c in 0..m {
            if map[r as usize][c as usize] != '@' {
                continue;
            }
            let mut neighbors = 0;
            if r - 1 >= 0 {
                // (i-1, j)
                if map[(r - 1) as usize][c as usize] == '@' {
                    neighbors += 1;
                }
                // (i-1, j+1)
                if c + 1 < m {
                    if map[(r - 1) as usize][(c + 1) as usize] == '@' {
                        neighbors += 1;
                    }
                }
                // (i-1, j-1)
                if c - 1 >= 0 {
                    if map[(r - 1) as usize][(c - 1) as usize] == '@' {
                        neighbors += 1;
                    }
                }
            }
            if r + 1 < n {
                // (i+1, j)
                if map[(r + 1) as usize][c as usize] == '@' {
                    neighbors += 1;
                }
                // (i+1, j+1)
                if c + 1 < m {
                    if map[(r + 1) as usize][(c + 1) as usize] == '@' {
                        neighbors += 1;
                    }
                }
                // (i+1, j-1)
                if c - 1 >= 0 {
                    if map[(r + 1) as usize][(c - 1) as usize] == '@' {
                        neighbors += 1;
                    }
                }
            }
            if c - 1 >= 0 {
                // (i, j-1)
                if map[(r) as usize][(c - 1) as usize] == '@' {
                    neighbors += 1;
                }
            }
            if c + 1 < m {
                // (i, j+1)
                if map[(r) as usize][(c + 1) as usize] == '@' {
                    neighbors += 1;
                }
            }

            if neighbors < 4 {
                count += 1;
                new_map[r as usize][c as usize] = '.';
            }
        }
    }

    if count > 0 {
        return Some((new_map, count));
    }
    None
}

fn run_part2(input_path: String) -> u32 {
    let binding = read_to_string(&input_path).expect("Failed to read input file");
    let lines = binding.trim().lines().collect::<Vec<_>>();
    let mut map: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut total: u32 = 0;
    while let Some((new_map, count)) = remove_rolls(map) {
        total += count;
        map = new_map;
    }

    total
}

fn run_part1(input_path: String) -> u32 {
    let binding = read_to_string(&input_path).expect("Failed to read input file");
    let lines = binding.trim().lines().collect::<Vec<_>>();
    let map: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    remove_rolls(map).unwrap().1
}

fn main() {
    let args = Args::parse();
    let result_part1 = run_part1(args.input_path.clone());
    let result_part2 = run_part2(args.input_path);

    println!("Result part 1: {}", result_part1);
    println!("Result part 2: {}", result_part2);
}

#[cfg(test)]
mod tests {
    use crate::{run_part1, run_part2};

    #[test]
    fn test_run_part1() {
        let path = "inputs/test-input.txt".to_string();
        assert_eq!(run_part1(path), 13)
    }

    #[test]
    fn test_run_part2() {
        let path = "inputs/test-input.txt".to_string();
        assert_eq!(run_part2(path), 43)
    }
}
