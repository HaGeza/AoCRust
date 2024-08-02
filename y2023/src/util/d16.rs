use thiserror::Error;

use crate::get_lines;

#[derive(Debug, Error)]
pub enum D16Error {
    #[error("io error")]
    IoError(#[from] std::io::Error),
}

pub fn read_grid(fp: &str) -> Result<Vec<Vec<char>>, D16Error> {
    let mut grid = vec![];
    for line in get_lines(fp)? {
        grid.push(vec![]);
        for c in line?.chars() {
            grid.last_mut().unwrap().push(c);
        }
    }
    Ok(grid)
}

pub fn get_number_of_energized(
    grid: &Vec<Vec<char>>,
    start_pos: (i32, i32),
    start_dir: usize,
) -> u64 {
    let n = grid.len();
    let m = grid[0].len();

    let mut traveled = vec![vec![vec![false; 4]; m]; n];

    let mut stack: Vec<((i32, i32), usize)> = vec![(start_pos, start_dir)];
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut sum = 0;
    while !stack.is_empty() {
        let (pos, dir) = stack.pop().unwrap();

        if pos.0 < 0 || pos.0 >= n as i32 || pos.1 < 0 || pos.1 >= m as i32 {
            continue;
        }

        let (i, j) = (pos.0 as usize, pos.1 as usize);
        if traveled[i][j][dir] {
            continue;
        }
        sum += traveled[i][j].iter().all(|&x| !x) as u64;
        traveled[i][j][dir] = true;

        match grid[i][j] {
            '|' => match dir {
                0 | 2 => {
                    stack.push(((pos.0 + dirs[1].0, pos.1), 1));
                    stack.push(((pos.0 + dirs[3].0, pos.1), 3));
                }
                1 | 3 => stack.push(((pos.0 + dirs[dir].0, pos.1), dir)),
                _ => unreachable!(),
            },
            '-' => match dir {
                1 | 3 => {
                    stack.push(((pos.0, pos.1 + dirs[0].1), 0));
                    stack.push(((pos.0, pos.1 + dirs[2].1), 2));
                }
                0 | 2 => stack.push(((pos.0, pos.1 + dirs[dir].1), dir)),
                _ => unreachable!(),
            },
            '\\' => {
                let new_dir = match dir {
                    0 => 1,
                    1 => 0,
                    2 => 3,
                    3 => 2,
                    _ => unreachable!(),
                };
                stack.push(((pos.0 + dirs[new_dir].0, pos.1 + dirs[new_dir].1), new_dir));
            }
            '/' => {
                let new_dir = match dir {
                    0 => 3,
                    1 => 2,
                    2 => 1,
                    3 => 0,
                    _ => unreachable!(),
                };
                stack.push(((pos.0 + dirs[new_dir].0, pos.1 + dirs[new_dir].1), new_dir));
            }
            _ => stack.push(((pos.0 + dirs[dir].0, pos.1 + dirs[dir].1), dir)),
        }
    }
    sum
}
