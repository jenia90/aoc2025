Advent of Code 2025

This repository is organized as a Cargo workspace with one crate per puzzle day.

Workspace layout

- Each day is a separate crate named day1, day2, day3, etc.
- Each crate is a binary crate with its main binary source at src/main.rs (crate root).
- Puzzle input files for a crate live inside that crate under inputs/ as inputs/input.txt and inputs/test-input.txt.

Example crate structure (for day1):

day1/
  Cargo.toml
  src/
    main.rs
  inputs/
    input.txt        # real puzzle input
    test-input.txt   # small test input used during development

Root workspace Cargo.toml (example)

[workspace]
members = [
  "day1",
  "day2",
  # add new day crates here, e.g. "day3"
]

How to run

- From the repository root run a single day's binary with:
  cargo run -p day1

- To pass arguments to the program (after the binary) use -- to separate cargo flags, e.g.:
  cargo run -p day1 -- --example-arg value

- Run tests for a single crate:
  cargo test -p day1

Reading inputs in code

Input files are located relative to the crate root. For example, to read the real input in Rust:

let input = std::fs::read_to_string("inputs/input.txt")?;

Adding a new day

1. Create a new binary crate in the repository root:
   cargo new --bin day3
2. Add "day3" to the [workspace].members array in the root Cargo.toml.
3. Add inputs/ and put input.txt and test-input.txt there.

Notes

- Crate naming uses day1, day2, ... (no leading zeros).
- Keep each crate self-contained: code under src/, inputs under inputs/.

If you'd like I can also add a small CONTRIBUTING section or CI badge — tell me what to include.
