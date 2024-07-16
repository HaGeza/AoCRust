use std::{
    cmp::{max, min},
    io::BufRead,
};

use y2023::get_reader;

fn process_buffer(num_buffer: &mut String, adjacent_to_symbol: bool) -> u32 {
    let result = if !num_buffer.is_empty() && adjacent_to_symbol {
        num_buffer.parse::<u32>().unwrap()
    } else {
        0
    };
    num_buffer.clear();
    result
}

fn schematic_number_sum(fp: &str) -> Result<u32, std::io::Error> {
    let reader = get_reader(fp)?;

    let mut mat = vec![];
    let mut symbol_mat = vec![];

    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        mat.push(vec![]);
        symbol_mat.push(vec![]);
        for c in line.chars() {
            mat[i].push(c);
            let is_symbol = !".0123456789".contains(c);
            symbol_mat[i].push(is_symbol);
        }
    }

    let n = mat.len();
    let m = mat[0].len();

    let mut sum: u32 = 0;
    let mut num_buffer = String::new();
    let mut adjacent_to_symbol = false;

    for i in 0..n {
        for j in 0..m {
            let c = mat[i][j];
            if "0123456789".contains(c) {
                num_buffer.push(c);

                if !adjacent_to_symbol {
                    let top = max(i as i32 - 1, 0) as usize;
                    let bot = min(i + 2, n);
                    let left = max(j as i32 - 1, 0) as usize;
                    let right = min(j + 2, m);

                    'adjecency_check: for ii in top..bot {
                        for jj in left..right {
                            if symbol_mat[ii][jj] {
                                adjacent_to_symbol = true;
                                break 'adjecency_check;
                            }
                        }
                    }
                }
            } else {
                sum += process_buffer(&mut num_buffer, adjacent_to_symbol);
                adjacent_to_symbol = false;
            }
        }
        sum += process_buffer(&mut num_buffer, adjacent_to_symbol);
        adjacent_to_symbol = false;
    }
    sum += process_buffer(&mut num_buffer, adjacent_to_symbol);

    Ok(sum)
}

fn main() {
    match schematic_number_sum("data/d3/a.txt") {
        Ok(sum) => println!("{}", sum),
        Err(e) => eprintln!("{}", e),
    }
}
