use y2023::util::d13::solve_d13;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Difference {
    Same,
    One,
    Many,
}
use Difference::*;

fn get_col_symmetries(matrix: &Vec<Vec<bool>>) -> Vec<Vec<Difference>> {
    let n = matrix.len();
    let m = matrix[0].len();

    let mut symmetries = vec![vec![Same; m]; m];
    for j in 0..m {
        for k in j + 1..m {
            for i in 0..n {
                if matrix[i][j] != matrix[i][k] {
                    match symmetries[j][k] {
                        Same => {
                            symmetries[j][k] = One;
                            symmetries[k][j] = One;
                        }
                        _ => {
                            symmetries[j][k] = Many;
                            symmetries[k][j] = Many;
                        }
                    }
                }
            }
        }
    }
    symmetries
}

fn get_row_symmetries(matrix: &Vec<Vec<bool>>) -> Vec<Vec<Difference>> {
    let n = matrix.len();
    let m = matrix[0].len();

    let mut symmetries = vec![vec![Same; n]; n];
    for i in 0..n {
        for k in i + 1..n {
            for j in 0..m {
                if matrix[i][j] != matrix[k][j] {
                    match symmetries[i][k] {
                        Same => {
                            symmetries[i][k] = One;
                            symmetries[k][i] = One;
                        }
                        _ => {
                            symmetries[i][k] = Many;
                            symmetries[k][i] = Many;
                        }
                    }
                }
            }
        }
    }
    symmetries
}

fn get_smudged_pattern(symmetries: &Vec<Vec<Difference>>) -> Option<(usize, usize)> {
    let n = symmetries.len();
    let mut centers = vec![];

    for i in 0..n - 1 {
        for j in 0..n - 1 {
            let part_of_center = [
                symmetries[i][j],
                symmetries[i][j + 1],
                symmetries[i + 1][j],
                symmetries[i + 1][j + 1],
            ];

            let same_count = part_of_center.iter().filter(|&&x| x == Same).count();
            let one_count = part_of_center.iter().filter(|&&x| matches!(x, One)).count();

            if same_count == 4 || one_count == 2 {
                centers.push((i, j));
            }
        }
    }

    'centers: for center in centers {
        let mut diff_count = 0;

        // Check main diagonal
        let tl = center.0.min(center.1);
        let (mut si, mut sj) = (center.0 - tl, center.1 - tl);
        while si < n && sj < n {
            match symmetries[si][sj] {
                Same => {}
                One => {
                    diff_count += 1;
                    if diff_count > 2 {
                        continue 'centers;
                    }
                }
                Many => continue 'centers,
            }
            si += 1;
            sj += 1;
        }
        if diff_count != 0 && diff_count != 2 {
            continue 'centers;
        }

        // Check secondary diagonal
        let bl = (n - center.0 - 2).min(center.1);
        let (mut si, mut sj) = ((center.0 + 1 + bl) as i32, (center.1 - bl) as i32);
        while si >= 0 && sj < n as i32 {
            match symmetries[si as usize][sj as usize] {
                Same => {}
                One => {
                    diff_count += 1;
                    if diff_count > 2 {
                        continue 'centers;
                    }
                }
                Many => continue 'centers,
            }
            si -= 1;
            sj += 1;
        }
        if diff_count != 2 {
            continue 'centers;
        }

        return Some(center);
    }
    None
}

fn summarize_pattern(matrix: &Vec<Vec<bool>>) -> u64 {
    let col_symmetries = get_col_symmetries(matrix);
    if let Some((_, j)) = get_smudged_pattern(&col_symmetries) {
        return (j + 1) as u64;
    }
    let row_symmetries = get_row_symmetries(matrix);
    if let Some((i, _)) = get_smudged_pattern(&row_symmetries) {
        return (i + 1) as u64 * 100;
    }
    0
}

fn main() {
    match solve_d13("data/d13/a.txt", summarize_pattern) {
        Ok(solution) => println!("{}", solution),
        Err(e) => eprintln!("Error: {}", e),
    }
}
