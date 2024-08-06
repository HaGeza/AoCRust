use y2023::{
    get_lines,
    util::d18::{get_movement, parse_line, D18Error},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cube {
    Border,
    Unknown,
    Section(usize),
}
use Cube::*;

fn solve(fp: &str) -> Result<u64, D18Error> {
    let (mut x, mut y, mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0, 0, 0);
    let mut steps = vec![];

    for line in get_lines(fp)? {
        let (dir, dist, col) = parse_line(&line?)?;
        steps.push((dir, dist, col.to_string()));

        let (dy, dx) = get_movement(dir, dist)?;
        y += dy;
        x += dx;

        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    let n = (max_y - min_y + 1) as usize;
    let m = (max_x - min_x + 1) as usize;
    let mut matrix = vec![vec![Unknown; m]; n];

    y = -min_y;
    x = -min_x;
    for (dir, dist, _) in steps {
        let (dy, dx) = get_movement(dir, dist)?;
        let (new_x, new_y) = (x + dx, y + dy);
        let l = x.min(new_x);
        let r = x.max(new_x);
        let t = y.min(new_y);
        let b = y.max(new_y);

        for i in t..=b {
            for j in l..=r {
                matrix[i as usize][j as usize] = Border;
            }
        }
        x = new_x;
        y = new_y;
    }

    let mut sol = 0;
    let mut sections = vec![];
    for i in 0..n {
        for j in 0..m {
            sol += match matrix[i][j] {
                Section(ind) => sections[ind] as u64,
                Border => 1,
                Unknown => {
                    let mut q = vec![(i as i32, j as i32)];
                    let ind = sections.len();
                    let mut inside = true;

                    while !q.is_empty() {
                        let (i, j) = q.pop().unwrap();
                        if i < 0 || i as usize >= n || j < 0 || j as usize >= m {
                            inside = false;
                            continue;
                        }
                        if matrix[i as usize][j as usize] != Unknown {
                            continue;
                        }
                        matrix[i as usize][j as usize] = Section(ind);
                        q.push((i - 1, j));
                        q.push((i + 1, j));
                        q.push((i, j - 1));
                        q.push((i, j + 1));
                    }

                    sections.push(inside);
                    sections[ind] as u64
                }
            };
        }
    }

    Ok(sol)
}

fn main() {
    match solve("data/d18/a.txt") {
        Ok(sol) => println!("{}", sol),
        Err(err) => println!("Error: {}", err),
    }
}
