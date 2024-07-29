use std::vec;

use thiserror::Error;
use y2023::{get_lines, get_subquestion_arg};

#[derive(Debug, Error)]
pub enum D11Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("incorrect matrix")]
    MatrixError,
}

fn solve(fp: &str, empty_multiplier: i64) -> Result<u64, D11Error> {
    let mut are_rows_empty = vec![];
    let mut are_cols_empty = vec![];
    let mut star_positions = vec![];

    for (i, line) in get_lines(fp)?.enumerate() {
        are_rows_empty.push(true);
        for (j, c) in line?.chars().enumerate() {
            if are_cols_empty.len() <= j {
                are_cols_empty.push(true);
            }
            if c == '#' {
                star_positions.push((i, j));
                are_rows_empty[i] = false;
                are_cols_empty[j] = false;
            }
        }
    }

    let mut empty_row_count = 0;
    let mut empty_row_until = vec![];
    for is_row_empty in are_rows_empty {
        if is_row_empty {
            empty_row_count += 1;
        }
        empty_row_until.push(empty_row_count);
    }

    let mut empty_col_count = 0;
    let mut empty_col_until = vec![];
    for is_col_empty in are_cols_empty {
        if is_col_empty {
            empty_col_count += 1;
        }
        empty_col_until.push(empty_col_count);
    }

    let mut distance_sum = 0;
    for (k, (i1, j1)) in star_positions.iter().enumerate() {
        for (i2, j2) in star_positions.iter().skip(k + 1) {
            let top = *i1.min(i2);
            let bot = *i1.max(i2);
            let left = *j1.min(j2);
            let right = *j1.max(j2);

            distance_sum += (bot - top)
                + (right - left)
                + ((empty_row_until[bot] - empty_row_until[top])
                    + (empty_col_until[right] - empty_col_until[left]))
                    * (empty_multiplier - 1) as usize;
        }
    }

    Ok(distance_sum as u64)
}

fn main() {
    let empty_multiplier = match get_subquestion_arg().as_str() {
        "a" => 2,
        "b" => 1000000,
        x => {
            eprintln!("Invalid argument: {}", x);
            return;
        }
    };

    match solve("data/d11/a.txt", empty_multiplier) {
        Ok(solution) => println!("{}", solution),
        Err(e) => eprintln!("Error: {}", e),
    }
}
