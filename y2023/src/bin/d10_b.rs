use std::collections::HashMap;

use y2023::util::d10::{get_filled_matrix, in_bounds, Cell, D10Error};

#[derive(Clone, PartialEq, Eq)]
enum Containment {
    Inside,
    Outside,
}

fn fill_with(
    fill_value: i32,
    i: usize,
    j: usize,
    matrix: &Vec<Vec<Cell>>,
    fill_matrix: &mut Vec<Vec<Option<i32>>>,
) -> Containment {
    let mut containment = Containment::Inside;

    match &matrix[i as usize][j as usize] {
        Cell::Pipe(_, _, Some(_)) | Cell::Start => return containment,
        _ => (),
    }

    fill_matrix[i][j] = Some(fill_value);

    for (di, dj) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let ii = i as i32 + di;
        let jj = j as i32 + dj;

        if !in_bounds(ii, jj, matrix.len() as i32, matrix[0].len() as i32) {
            containment = Containment::Outside;
            continue;
        }

        let (ui, uj) = (ii as usize, jj as usize);
        if let Some(_) = fill_matrix[ui][uj] {
            continue;
        }
        match fill_with(fill_value, ui, uj, matrix, fill_matrix) {
            Containment::Outside => containment = Containment::Outside,
            _ => (),
        }
    }

    containment
}

fn solve(fp: &str) -> Result<i32, D10Error> {
    let matrix = get_filled_matrix(fp)?;

    let mut fill_matrix = vec![vec![None; matrix[0].len()]; matrix.len()];

    let mut inside_count = 0;
    let mut section_count = 0;
    let mut sections = HashMap::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            match &fill_matrix[i][j] {
                None => {
                    sections.insert(
                        section_count,
                        fill_with(section_count, i, j, &matrix, &mut fill_matrix),
                    );
                    section_count += 1;
                }
                _ => (),
            }
        }
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if let Some(section) = fill_matrix[i][j] {
                if sections[&section] == Containment::Inside {
                    inside_count += 1;
                }
                print!(
                    "{}",
                    match sections[&section] {
                        Containment::Inside => 'I',
                        Containment::Outside => '.',
                    }
                );
            } else {
                print!(
                    "{}",
                    match &matrix[i][j] {
                        Cell::Pipe(_, _, Some(_)) => '.',
                        Cell::Start => '.',
                        _ => '.',
                    }
                );
            }
        }
        println!();
    }

    Ok(inside_count)
}

fn main() {
    match solve("data/d10/test_b.txt") {
        Ok(inside) => println!("{}", inside),
        Err(e) => eprintln!("{}", e),
    }
}
