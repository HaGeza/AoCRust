use std::collections::VecDeque;

use y2023::util::d21::{read_garden_matrix, D21Error};

enum RepeatType {
    Center,
    Straight,
    Corner,
}

fn fill_dist_matrix(
    walkable: &Vec<Vec<bool>>,
    start: (usize, usize),
    steps: u64,
    repeat_type: RepeatType,
) -> Result<u64, D21Error> {
    let n = walkable.len();
    let m = walkable[0].len();
    let mut positions = VecDeque::from([start]);
    let mut sum = 0;
    let mut dist = vec![vec![None; n]; m];
    dist[start.0][start.1] = Some(0);

    while !positions.is_empty() {
        let (i, j) = positions.pop_front().unwrap();

        let Some(d) = dist[i][j] else {
            return Err(D21Error::InvalidDistMatrix);
        };

        sum += match repeat_type {
            RepeatType::Center => (d <= steps && d % 2 == steps % 2) as u64,
            RepeatType::Straight => {
                let steps_needed = (n / 2) as u64 + 1 + d;
                if steps_needed > steps {
                    0
                } else {
                    let parity_match = (steps_needed % 2 == steps % 2) as u64;
                    ((steps - steps_needed) / n as u64 + 1 + parity_match) / 2
                }
            }
            RepeatType::Corner => {
                let steps_needed = n as u64 + 1 + d;
                if steps_needed > steps {
                    0
                } else {
                    let parity_match = steps_needed % 2 == steps % 2;
                    let reachable_count = ((steps - steps_needed) / n as u64) + 1;
                    if parity_match {
                        let multiplier = (reachable_count + 1) / 2;
                        multiplier * multiplier
                    } else if reachable_count > 1 {
                        let multiplier = reachable_count / 2;
                        multiplier * (multiplier + 1)
                    } else {
                        0
                    }
                }
            }
        };

        for (di, dj) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_i = i as i64 + di;
            let new_j = j as i64 + dj;
            if new_i < 0 || new_j < 0 || new_i >= n as i64 || new_j >= m as i64 {
                continue;
            }
            let (new_i, new_j) = (new_i as usize, new_j as usize);

            if walkable[new_i][new_j] {
                if dist[new_i][new_j].is_none() {
                    dist[new_i][new_j] = Some(d + 1);
                    positions.push_back((new_i, new_j));
                }
            }
        }
    }
    Ok(sum)
}

fn solve(fp: &str, steps: u64) -> Result<u64, D21Error> {
    let (walkable, start) = read_garden_matrix(fp)?;

    let n = walkable.len();
    if n != walkable[0].len() {
        return Err(D21Error::Part2Error("Non-square matrix".to_string()));
    }
    if n % 2 != 1 {
        return Err(D21Error::Part2Error("Even side length matrix".to_string()));
    }
    if start != (n / 2, n / 2) {
        return Err(D21Error::Part2Error(
            "Starting point not in the center".to_string(),
        ));
    }

    let start_is = [0, start.0, n - 1];
    let start_js = [0, start.1, n - 1];

    let mut sum = 0;
    for i in 0..3 {
        for j in 0..3 {
            let (si, sj) = (start_is[i], start_js[j]);
            let repeat_type = match (i, j) {
                (1, 1) => RepeatType::Center,
                (1, _) | (_, 1) => RepeatType::Straight,
                _ => RepeatType::Corner,
            };

            sum += fill_dist_matrix(&walkable, (si, sj), steps, repeat_type)?;
        }
    }

    Ok(sum)
}

fn main() {
    let steps = 26501365;
    match solve("data/d21/a.txt", steps) {
        Ok(sol) => println!("{}: {}", steps, sol),
        Err(e) => eprintln!("Error: {}", e),
    }
}
