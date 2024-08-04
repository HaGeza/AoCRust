# Advent of Code 2023 in Rust

This repository contains solutions for [Advent of Code 2023](https://adventofcode.com/2023) written in Rust.
The solutions were written during the summer of 2024.

## Usage

**The solutions require data to be stored under `y2023/data/d<day_number>` (see the relevant scripts for expected input)**.

Most solution can be run with:
```sh
cargo run --bin d<day_number>[_a|_b]
```
For example:
```sh
# Run day 13, first question:
cargo run --bin d13_a
# Run day 8, second question:
cargo run --bin d8_b
```
For certain days, due to similarity of solutions, they are written in one shared script, hence they can be run by passing `a` or `b` as a parameter:
```sh
cargo run --bin d<day_number> [a|b]
```
For example:
```sh
# Run day 9, first question
cargo run --bin d9 a
# Run day 17, second question
cargo run --bin d17 b
```
