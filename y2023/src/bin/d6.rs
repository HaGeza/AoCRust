use std::{
    cmp::{max, min},
    env,
    fs::File,
    io::{BufReader, Lines},
};

use thiserror::Error;
use y2023::get_lines;

#[derive(Error, Debug)]
enum D6Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("parse error")]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid input")]
    InvalidInput,
}

fn get_nums_from_next_line(
    lines: &mut Lines<BufReader<File>>,
    merge_nums: bool,
) -> Result<Vec<u64>, D6Error> {
    let binding = lines.next().ok_or(D6Error::InvalidInput)??;
    let mut strings: Vec<&str> = binding.split_whitespace().skip(1).collect();

    let binding = strings.join("");
    let joined_string = binding.as_str();
    strings = if merge_nums {
        vec![joined_string]
    } else {
        strings
    };

    strings
        .iter()
        .map(|x| x.parse::<u64>().map_err(D6Error::from))
        .collect::<Result<Vec<u64>, D6Error>>()
}

fn num_solutions_prod(fp: &str, merge_nums: bool) -> Result<u64, D6Error> {
    let mut lines = get_lines(fp)?;

    let times = get_nums_from_next_line(&mut lines, merge_nums)?;
    let distances = get_nums_from_next_line(&mut lines, merge_nums)?;

    let mut possibilities = vec![];
    for (&t, &d) in times.iter().zip(distances.iter()) {
        let delta = f64::sqrt((t * t) as f64 - (4 * d) as f64);
        let low = (-(t as f64) + delta) / (-2.0);
        let high = (-(t as f64) - delta) / (-2.0);

        let low_int = (low + 1e-10).ceil() as i64;
        let high_int = (high - 1e-10).floor() as i64;
        possibilities.push(min(d as i64, high_int) as u64 - max(0, low_int) as u64 + 1);
    }

    println!("{:?}", possibilities);

    Ok(possibilities.iter().product())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if args.len() < 2 {
        println!("No input provided, running for a");
        "a"
    } else {
        args[1].as_str()
    };

    let merge_nums = match input {
        "a" => false,
        "b" => true,
        _ => {
            eprintln!("Invalid input");
            return;
        }
    };

    match num_solutions_prod("data/d6/a.txt", merge_nums) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
