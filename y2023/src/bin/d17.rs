use std::{
    cmp::{max, min, Reverse},
    collections::BinaryHeap,
};

use thiserror::Error;
use y2023::{get_lines, get_subquestion_arg};

#[derive(Debug, Error)]
enum D17Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("error parsing char to digit: {0}")]
    Parse(char),
}

fn grid_section_sum(grid: &Vec<Vec<u8>>, i: usize, j: usize, new_i: usize, new_j: usize) -> u32 {
    let mut sum = 0;
    let (si, ei) = (min(i, new_i), (max(i, new_i)));
    let (sj, ej) = (min(j, new_j), (max(j, new_j)));

    for ii in si..=ei {
        for jj in sj..=ej {
            sum += grid[ii][jj] as u32;
        }
    }
    sum - grid[i][j] as u32
}

fn solve(fp: &str, min_steps: usize, max_steps: usize) -> Result<u32, D17Error> {
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
    let n_steps = (max_steps - min_steps) as usize + 1;
    let min_si = min_steps as i32;

    let mut lowest_map = vec![vec![vec![vec![None; n_steps]; 4]; m]; n];
    let mut lowest_overall = u32::MAX;
    // let mut stack = vec![((0, min_steps), (0, 0)), ((min_steps, 0), (0, 1))];
    let heat_0 = grid_section_sum(&grid, 0, 0, 0, min_steps);
    let heat_1 = grid_section_sum(&grid, 0, 0, min_steps, 0);
    lowest_map[0][min_steps][0][0] = Some(heat_0);
    lowest_map[min_steps][0][0][1] = Some(heat_1);
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((heat_0, (0, min_steps), (0, 0))));
    heap.push(Reverse((heat_1, (min_steps, 0), (0, 1))));

    while let Some(Reverse((heat, (i, j), (d, l)))) = heap.pop() {
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
            if new_l >= n_steps {
                continue;
            }

            let (new_i, new_j) = if k == d {
                (i as i32 + di, j as i32 + dj)
            } else {
                (i as i32 + min_si * di, j as i32 + min_si * dj)
            };
            if new_i < 0 || new_i >= n as i32 || new_j < 0 || new_j >= m as i32 {
                continue;
            }
            let (new_i, new_j) = (new_i as usize, new_j as usize);

            let new_heat = heat + grid_section_sum(&grid, i, j, new_i, new_j);
            if let Some(old_heat) = lowest_map[new_i][new_j][k][new_l] {
                if new_heat >= old_heat {
                    continue;
                }
            }
            lowest_map[new_i][new_j][k][new_l] = Some(new_heat);
            heap.push(Reverse((new_heat, (new_i, new_j), (k, new_l))));
        }
    }

    Ok(lowest_overall)
}

fn main() {
    let (min_steps, max_steps) = match get_subquestion_arg().as_str() {
        "a" => (1, 3),
        "b" => (4, 10),
        x => {
            eprintln!("Invalid argument: {}", x);
            return;
        }
    };

    match solve("data/d17/a.txt", min_steps, max_steps) {
        Ok(sol) => println!("{}", sol),
        Err(e) => eprintln!("Error: {}", e),
    }
}
