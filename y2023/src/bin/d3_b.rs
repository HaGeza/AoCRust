use std::{
    cmp::{max, min},
    io::BufRead,
};

use y2023::get_reader;

enum Cell {
    NonSymbol,
    MainGear(Option<u32>, Option<u32>),
    OtherSymbol,
}

fn process_num_buffer(
    num_buffer: &mut String,
    symbol_mat: &mut Vec<Vec<Cell>>,
    i: usize,
    j: usize,
) {
    let n = symbol_mat.len();
    let m = symbol_mat[0].len();

    let top = max(i as i32 - 1, 0) as usize;
    let bot = min(i + 2, n);
    let left = max(j as i32 - num_buffer.len() as i32 - 1, 0) as usize;
    let right = min(j + 1, m);

    let num = num_buffer.parse::<u32>().unwrap();

    for ii in top..bot {
        for jj in left..right {
            match symbol_mat[ii][jj] {
                Cell::MainGear(None, _) => {
                    symbol_mat[ii][jj] = Cell::MainGear(Some(num), None);
                }
                Cell::MainGear(Some(other), None) => {
                    symbol_mat[ii][jj] = Cell::MainGear(Some(other), Some(num));
                }
                Cell::MainGear(Some(_), Some(_)) => {
                    symbol_mat[ii][jj] = Cell::OtherSymbol;
                }
                _ => {}
            }
        }
    }

    num_buffer.clear();
}

fn gear_ratio_sum(fp: &str) -> Result<u32, std::io::Error> {
    let reader = get_reader(fp)?;

    let mut mat = vec![];
    let mut symbol_mat = vec![];

    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        mat.push(vec![]);
        symbol_mat.push(vec![]);
        for c in line.chars() {
            mat[i].push(c);

            if ".0123456789".contains(c) {
                symbol_mat[i].push(Cell::NonSymbol);
            } else if c != '*' {
                symbol_mat[i].push(Cell::OtherSymbol);
            } else {
                symbol_mat[i].push(Cell::MainGear(None, None));
            }
        }
    }

    let n = mat.len();
    let m = mat[0].len();

    let mut sum: u32 = 0;
    let mut num_buffer = String::new();

    for i in 0..n {
        for j in 0..m {
            let c = mat[i][j];
            if "0123456789".contains(c) {
                num_buffer.push(c);
            } else if num_buffer.len() > 0 {
                process_num_buffer(&mut num_buffer, &mut symbol_mat, i, j);
            }
        }
        if num_buffer.len() > 0 {
            process_num_buffer(&mut num_buffer, &mut symbol_mat, i, m);
        }
    }

    for i in 0..n {
        for j in 0..m {
            match symbol_mat[i][j] {
                Cell::MainGear(Some(a), Some(b)) => {
                    sum += a * b;
                }
                _ => {}
            }
        }
    }

    Ok(sum)
}

fn main() {
    match gear_ratio_sum("data/d3/a.txt") {
        Ok(sum) => println!("{}", sum),
        Err(e) => eprintln!("{}", e),
    }
}
