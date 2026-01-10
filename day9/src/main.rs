use clap::Parser;
use std::{
    cmp::{max, min},
    collections::HashSet,
    fs::read_to_string,
};

#[derive(Debug, Parser)]
struct Args {
    input_path: String,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn run_part1(input_path: String) -> i64 {
    let input = read_to_string(input_path).expect("Failed to read input file");

    let points: HashSet<(i64, i64)> = input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|coord| coord.parse::<i64>().unwrap());
            (coords.next().unwrap(), coords.next().unwrap())
        })
        .collect();

    let mut max_area = 0;
    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i + 1) {
            let area = (1 + (p1.0 - p2.0).abs()) * (1 + (p1.1 - p2.1).abs());
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn run_part2(input_path: String) -> i64 {
    // ----- read input: lines "x,y" -----
    let input = std::fs::read_to_string(input_path).expect("Failed to read input file");

    let poly: Vec<Point> = input
        .lines()
        .map(|line| {
            let mut it = line.split(',').map(|c| c.trim().parse::<i64>().unwrap());
            Point {
                x: it.next().unwrap(),
                y: it.next().unwrap(),
            }
        })
        .collect();

    let mut max_area = 0;
    for (i, &p1) in poly.iter().enumerate() {
        for &p2 in poly.iter().skip(i + 1) {
            let area = (1 + (p1.x - p2.x).abs()) * (1 + (p1.y - p2.y).abs());
            if rectangle_in_polygon(p1, p2, &poly) {
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }

    max_area
}

fn assert_polygon_is_valid(poly: &[Point]) {
    let n = poly.len();
    assert!(n >= 4);

    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        let c = poly[(i + 2) % n];

        // Edges must be axis-aligned.
        assert!(a.x == b.x || a.y == b.y);

        let ab_vertical = a.x == b.x;
        let bc_vertical = b.x == c.x;

        // No two adjacent edges both vertical or both horizontal.
        assert!(ab_vertical != bc_vertical);
    }
}

/// Check whether the axis-aligned rectangle with opposite corners a,b
/// is considered "within" the polygon per the author’s three conditions. [page:1]
fn rectangle_in_polygon(a: Point, b: Point, poly: &[Point]) -> bool {
    let (x1, x2) = (min(a.x, b.x), max(a.x, b.x));
    let (y1, y2) = (min(a.y, b.y), max(a.y, b.y));

    // 1) All four corners inside/on polygon. [page:1]
    let corners = [
        Point { x: x1, y: y1 },
        Point { x: x1, y: y2 },
        Point { x: x2, y: y1 },
        Point { x: x2, y: y2 },
    ];
    if corners.iter().any(|&p| !point_in_polygon(p, poly)) {
        return false;
    }

    // 2) No polygon edge has any point strictly inside the rectangle. [page:1]
    let n = poly.len();
    for i in 0..n {
        let p = poly[i];
        let q = poly[(i + 1) % n];

        // Axis-aligned edges only.
        if p.x == q.x {
            // vertical edge at x = p.x, y in [yp1, yp2]
            let ex = p.x;
            let (ey1, ey2) = (min(p.y, q.y), max(p.y, q.y));
            // Check if any point on this segment is strictly inside rect:
            // x1 < ex < x2 and (∃ y with y1 < y < y2 and ey1 <= y <= ey2)
            if ex > x1 && ex < x2 {
                let seg_low = max(ey1, y1 + 1);
                let seg_high = min(ey2, y2 - 1);
                if seg_low <= seg_high {
                    return false;
                }
            }
        } else if p.y == q.y {
            // horizontal edge at y = p.y, x in [xp1, xp2]
            let ey = p.y;
            let (ex1, ex2) = (min(p.x, q.x), max(p.x, q.x));
            // x in [ex1, ex2], check if strictly inside rect:
            // y1 < ey < y2 and (∃ x with x1 < x < x2 and ex1 <= x <= ex2)
            if ey > y1 && ey < y2 {
                let seg_low = max(ex1, x1 + 1);
                let seg_high = min(ex2, x2 - 1);
                if seg_low <= seg_high {
                    return false;
                }
            }
        }
    }

    // 3) Shrink rectangle by one tile on each side; inner corners must be inside. [page:1]
    let inner_x1 = x1 + 1;
    let inner_x2 = x2 - 1;
    let inner_y1 = y1 + 1;
    let inner_y2 = y2 - 1;

    if inner_x1 <= inner_x2 && inner_y1 <= inner_y2 {
        let inner_corners = [
            Point {
                x: inner_x1,
                y: inner_y1,
            },
            Point {
                x: inner_x1,
                y: inner_y2,
            },
            Point {
                x: inner_x2,
                y: inner_y1,
            },
            Point {
                x: inner_x2,
                y: inner_y2,
            },
        ];
        if inner_corners.iter().any(|&p| !point_in_polygon(p, poly)) {
            return false;
        }
    }

    true
}

/// Even–odd rule point-in-polygon with explicit boundary check,
/// adapted to rectilinear polygons. [web:23][page:1]
fn point_in_polygon(p: Point, poly: &[Point]) -> bool {
    let n = poly.len();

    // First: boundary check (on any axis-aligned edge). [page:1]
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];

        if a.x == b.x {
            // vertical edge at x = a.x
            if p.x == a.x && between(p.y, a.y, b.y) {
                return true;
            }
        } else if a.y == b.y {
            // horizontal edge at y = a.y
            if p.y == a.y && between(p.x, a.x, b.x) {
                return true;
            }
        }
    }

    // Even-odd ray cast to the right. [web:23][web:28][page:1]
    let mut crossings = 0;
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];

        if a.x == b.x {
            // vertical edge
            let (xv, y1, y2) = (a.x, min(a.y, b.y), max(a.y, b.y));
            // Count edge if:
            // - edge is strictly to the right of point
            // - ray at y = p.y passes through segment (y1 <= p.y < y2)
            if xv > p.x && p.y >= y1 && p.y < y2 {
                crossings += 1;
            }
        } else {
            // horizontal edge: ray parallel, ignore
            continue;
        }
    }

    (crossings % 2) == 1
}

fn between(v: i64, a: i64, b: i64) -> bool {
    let (lo, hi) = (min(a, b), max(a, b));
    v >= lo && v <= hi
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
        assert_eq!(run_part1(path), 50)
    }

    #[test]
    fn test_run_part2() {
        let path = "inputs/test-input.txt".to_string();
        assert_eq!(run_part2(path), 24)
    }
}
