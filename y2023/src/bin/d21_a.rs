use std::collections::HashSet;

use y2023::util::d21::{read_garden_matrix, D21Error};

fn solve(fp: &str, steps: usize) -> Result<u64, D21Error> {
    let (walkable, start) = read_garden_matrix(fp)?;

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
    for steps in 1..20 {
        match solve("data/d21/test_2_large.txt", steps) {
            Ok(sol) => println!("{}: {}", steps, sol),
            Err(e) => eprintln!("{}", e),
        }
    }
}
