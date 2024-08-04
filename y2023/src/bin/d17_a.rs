use thiserror::Error;
use y2023::get_lines;

#[derive(Debug, Error)]
enum D17Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("error parsing char to digit: {0}")]
    Parse(char),
}

fn solve(fp: &str) -> Result<u64, D17Error> {
    let mut grid = vec![];
    for line in get_lines(fp)? {
        grid.push(
            line?
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or(D17Error::Parse(c))
                        .and_then(|d| Ok(d as u8))
                })
                .collect::<Result<Vec<u8>, _>>()?,
        );
    }

    let n = grid.len();
    let m = grid[0].len();
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut lowest_map = vec![vec![vec![vec![None; 3]; 4]; m]; n];
    let mut lowest_overall = u64::MAX;
    let mut stack = vec![((0, 1), (0, 0)), ((1, 0), (0, 1))];
    lowest_map[0][1][0][0] = Some(grid[0][1] as u64);
    lowest_map[1][0][0][1] = Some(grid[1][0] as u64);

    while !stack.is_empty() {
        let ((i, j), (d, l)) = stack.pop().unwrap();
        let Some(heat) = lowest_map[i][j][d][l] else {
            unreachable!()
        };

        if heat >= lowest_overall {
            continue;
        } else if i == n - 1 && j == m - 1 {
            lowest_overall = heat;
            continue;
        }

        for (k, (di, dj)) in dirs.iter().enumerate() {
            match (d, k) {
                (0, 2) | (2, 0) | (1, 3) | (3, 1) => continue,
                _ => {}
            }

            let new_l = if k == d { l + 1 } else { 0 };
            if new_l > 2 {
                continue;
            }

            let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
            if new_i < 0 || new_i >= n as i32 || new_j < 0 || new_j >= m as i32 {
                continue;
            }
            let (new_i, new_j) = (new_i as usize, new_j as usize);

            let new_heat = heat + grid[new_i][new_j] as u64;
            if let Some(old_heat) = lowest_map[new_i][new_j][k][new_l] {
                if new_heat >= old_heat {
                    continue;
                }
            }
            lowest_map[new_i][new_j][k][new_l] = Some(new_heat);
            stack.push(((new_i, new_j), (k, new_l)));
        }
    }

    Ok(lowest_overall)
}

fn main() {
    match solve("data/d17/small.txt") {
        Ok(sol) => println!("{}", sol),
        Err(e) => eprintln!("Error: {}", e),
    }
}
