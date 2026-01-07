use clap::Parser;
use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, Parser)]
struct Args {
    input_path: String,
    num_connections: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn d2(&self, other: &Self) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }

        true
    }

    fn get_circuit_sizes(&mut self) -> Vec<usize> {
        let mut sizes = HashMap::new();

        let roots: Vec<usize> = (0..self.parent.len()).map(|i| self.find(i)).collect();

        for root in roots {
            *sizes.entry(root).or_insert(0) += 1;
        }

        let mut result: Vec<usize> = sizes.values().copied().collect();
        result.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending

        result
    }
}
fn run_part1(input_path: String, num_connections: usize) -> usize {
    let input = read_to_string(input_path).expect("Failed to read input file");

    let jboxes: Vec<Point> = input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|coord| coord.parse::<i64>().unwrap());
            Point::new(
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect();

    let num_boxes = jboxes.len();
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for (i, p1) in jboxes.iter().enumerate() {
        for (j, p2) in jboxes.iter().enumerate().skip(i + 1) {
            edges.push((p1.d2(p2), i, j));
        }
    }

    edges.sort_unstable_by_key(|&(dist, _, _)| dist);

    let mut connections = 0;
    let mut uf = UnionFind::new(num_boxes);
    for (_, i, j) in edges {
        connections += 1;
        uf.union(i, j);
        if connections == num_connections {
            break;
        }
    }

    let sizes = uf.get_circuit_sizes();

    sizes[0] * sizes[1] * sizes[2]
}

fn run_part2(input_path: String) -> u64 {
    let input = read_to_string(input_path).expect("Failed to read input file");

    let jboxes: Vec<Point> = input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|coord| coord.parse::<i64>().unwrap());
            Point::new(
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect();

    let num_boxes = jboxes.len();
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for (i, p1) in jboxes.iter().enumerate() {
        for (j, p2) in jboxes.iter().enumerate().skip(i + 1) {
            edges.push((p1.d2(p2), i, j));
        }
    }

    edges.sort_unstable_by_key(|&(dist, _, _)| dist);

    let mut connections = num_boxes;
    let mut uf = UnionFind::new(num_boxes);
    let mut last_i = 0usize;
    let mut last_j = 0usize;

    for &(_dist, i, j) in &edges {
        if uf.union(i, j) {
            connections -= 1;
            last_i = i;
            last_j = j;
            if connections == 1 {
                break;
            }
        }
    }

    let xi = jboxes[last_i].x as u64;
    let xj = jboxes[last_j].x as u64;
    xi * xj
}

fn main() {
    let args = Args::parse();
    let result_part1 = run_part1(args.input_path.clone(), args.num_connections);
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
        assert_eq!(run_part1(path, 10), 40)
    }

    #[test]
    fn test_run_part2() {
        let path = "inputs/test-input.txt".to_string();
        assert_eq!(run_part2(path), 25272)
    }
}
