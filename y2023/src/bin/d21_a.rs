use std::collections::HashSet;

use thiserror::Error;
use y2023::get_lines;

#[derive(Debug, Error)]
enum D21Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("invalid character in input: {0}")]
    InvalidInputChar(char),
}

fn solve(fp: &str, steps: usize) -> Result<u64, D21Error> {
    let mut walkable = vec![];
    let mut start = (0, 0);

    for line in get_lines(fp)? {
        let mut new_row = vec![];
        for c in line?.chars() {
            let w = match c {
                '.' => true,
                'S' => {
                    start = (walkable.len(), new_row.len());
                    true
                }
                '#' => false,
                _ => return Err(D21Error::InvalidInputChar(c)),
            };
            new_row.push(w);
        }
        walkable.push(new_row);
    }

    let n = walkable.len();
    let m = walkable[0].len();
    let mut positions = HashSet::from([start]);

    for _ in 0..steps {
        let mut new_positions = HashSet::new();
        for (i, j) in &positions {
            for (di, dj) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_i = *i as i64 + di;
                let new_j = *j as i64 + dj;
                if new_i < 0 || new_j < 0 || new_i >= n as i64 || new_j >= m as i64 {
                    continue;
                }
                let (new_i, new_j) = (new_i as usize, new_j as usize);

                if walkable[new_i][new_j] {
                    new_positions.insert((new_i, new_j));
                }
            }
        }
        positions = new_positions;
    }

    Ok(positions.len() as u64)
}

fn main() {
    match solve("data/d21/test_1.txt", 3) {
        Ok(sol) => println!("{}", sol),
        Err(e) => eprintln!("{}", e),
    }
}
