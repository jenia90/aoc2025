[![Rust](https://github.com/jenia90/aoc2025/actions/workflows/rust.yml/badge.svg)](https://github.com/jenia90/aoc2025/actions/workflows/rust.yml)

# Advent of Code 2025

This repository is organized as a Cargo workspace with one crate per puzzle day.

#### Workspace layout

- Each day is a separate crate named day1, day2, day3, etc.
- Each crate is a binary crate with its main binary source at src/main.rs (crate root).
- Puzzle input files for a crate live inside that crate under inputs/ as inputs/input.txt and inputs/test-input.txt.

#### Example crate structure (for day1):

```plaintesxt
day1/
  Cargo.toml
  src/
    main.rs
  inputs/
    input.txt        # real puzzle input
    test-input.txt   # small test input used during development
```

#### Root workspace Cargo.toml

```toml
[workspace]
members = [
  "day1",
  "day2",
  # add new day crates here, e.g. "day3"
]
```

#### How to run

- From the repository root run a single day's binary with:
  ```bash
  cargo run -p day1
  ```
- To pass arguments to the program (after the binary) use -- to separate cargo flags, e.g.:
  ```bash
  cargo run -p day1 <input_file> [<addition_args>...]
  ```

#### Adding a new day

1. Create a new binary crate in the repository root:
   ```bash
   cargo new --bin dayN
   ```
3. Add "day3" to the `[workspace].members` array in the root Cargo.toml, if it hasn't been added automatically.
4. Add `inputs/` and put `input.txt` and `test-input.txt` there.

#### Notes

- Crate naming uses day1, day2, ... (no leading zeros).
- Keep each crate self-contained: code under `src/`, `inputs` under `inputs/`.
