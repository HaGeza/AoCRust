use thiserror::Error;
use y2023::get_lines;

#[derive(Debug, Error)]
enum D13Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("parse error")]
    ParseInt(#[from] std::num::ParseIntError),
}

fn get_col_symmetries(matrix: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let n = matrix.len();
    let m = matrix[0].len();

    let mut symmetries = vec![vec![false; m]; m];
    for j in 0..m {
        symmetries[j][j] = true;
        'compare_cols: for k in j + 1..m {
            for i in 0..n {
                if matrix[i][j] != matrix[i][k] {
                    continue 'compare_cols;
                }
            }
            symmetries[j][k] = true;
            symmetries[k][j] = true;
        }
    }
    symmetries
}

fn get_row_symmetries(matrix: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let n = matrix.len();
    let m = matrix[0].len();

    let mut symmetries = vec![vec![false; n]; n];
    for i in 0..n {
        symmetries[i][i] = true;
        'compare_rows: for k in i + 1..n {
            for j in 0..m {
                if matrix[i][j] != matrix[k][j] {
                    continue 'compare_rows;
                }
            }
            symmetries[i][k] = true;
            symmetries[k][i] = true;
        }
    }
    symmetries
}

fn get_pattern(symmetries: &Vec<Vec<bool>>) -> Option<(usize, usize)> {
    let n = symmetries.len();
    let mut centers = vec![];

    for i in 0..n - 1 {
        for j in 0..n - 1 {
            if symmetries[i][j]
                && symmetries[i][j + 1]
                && symmetries[i + 1][j]
                && symmetries[i + 1][j + 1]
            {
                centers.push((i, j));
            }
        }
    }

    'centers: for center in centers {
        // Check main diagonal
        let tl = center.0.min(center.1);
        let (mut si, mut sj) = (center.0 - tl, center.1 - tl);
        while si < n && sj < n {
            if !symmetries[si][sj] {
                continue 'centers;
            }
            si += 1;
            sj += 1;
        }

        // Check secondary diagonal
        let bl = (n - center.0 - 2).min(center.1);
        let (mut si, mut sj) = ((center.0 + 1 + bl) as i32, (center.1 - bl) as i32);
        while si >= 0 && sj < n as i32 {
            if !symmetries[si as usize][sj as usize] {
                continue 'centers;
            }
            si -= 1;
            sj += 1;
        }
        return Some(center);
    }
    None
}

fn summarize_pattern(matrix: &Vec<Vec<bool>>) -> u64 {
    let col_symmetries = get_col_symmetries(matrix);
    if let Some((_, j)) = get_pattern(&col_symmetries) {
        return (j + 1) as u64;
    }
    let row_symmetries = get_row_symmetries(matrix);
    if let Some((i, _)) = get_pattern(&row_symmetries) {
        return (i + 1) as u64 * 100;
    }
    0
}

fn solve(fp: &str) -> Result<u64, D13Error> {
    let mut sum = 0;

    let mut matrix = vec![];
    for line in get_lines(fp)? {
        let line = line?;
        if line.trim().is_empty() {
            sum += summarize_pattern(&matrix);
            matrix.clear();
        } else {
            matrix.push(line.chars().map(|c| c == '#').collect::<Vec<_>>());
        }
    }
    sum += summarize_pattern(&matrix);

    Ok(sum)
}

fn main() {
    match solve("data/d13/a.txt") {
        Ok(solution) => println!("{}", solution),
        Err(e) => eprintln!("Error: {}", e),
    }
}
