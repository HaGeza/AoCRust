use thiserror::Error;
use y2023::get_lines;

#[derive(Debug, Error)]
enum D14Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Round,
    Square,
}
use Cell::*;

fn solve(fp: &str) -> Result<u64, D14Error> {
    let mut grid = vec![];

    for (i, line) in get_lines(fp)?.enumerate() {
        grid.push(vec![]);
        for c in line?.chars() {
            grid[i].push(match c {
                'O' => Round,
                '#' => Square,
                _ => Empty,
            });
        }
    }

    let n = grid.len();
    let m = grid[0].len();
    let mut fall = vec![vec![0; m]; n];

    for i in 1..n {
        for j in 0..m {
            fall[i][j] = match &grid[i - 1][j] {
                Square => 0,
                Round => fall[i - 1][j],
                Empty => 1 + fall[i - 1][j],
            }
        }
    }

    let mut sum = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == Round {
                sum += (n - i + fall[i][j]) as u64;
            }
        }
    }
    Ok(sum)
}

fn main() {
    match solve("data/d14/a.txt") {
        Ok(result) => println!("{}", result),
        Err(error) => println!("Error: {}", error),
    }
}
