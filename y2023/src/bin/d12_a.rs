use thiserror::Error;
use y2023::{get_lines, get_subquestion_arg};

#[derive(Debug, Error)]
enum D12Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("invalid line")]
    InvalidLine,
    #[error("invalid number: {0}")]
    InvalidNumber(#[from] std::num::ParseIntError),
}

#[derive(Debug, Clone, Copy)]
enum Component {
    Empty,
    Spring,
    Unknown,
}
use Component::*;

fn num_arrangements_rec(
    components: Vec<Component>,
    nums: Vec<u32>,
    next_component: Component,
) -> u64 {
    if components.is_empty() {
        return nums.is_empty() as u64;
    }
    if (components.len() as i64) < (nums.iter().sum::<u32>() as i64 + nums.len() as i64 - 1) {
        return 0;
    }

    match (components[0], next_component) {
        (Empty, Empty) | (Unknown, Empty) | (Empty, Unknown) => {
            num_arrangements_rec(components[1..].to_vec(), nums, Unknown)
        }
        (Spring, Spring) | (Unknown, Spring) | (Spring, Unknown) => {
            if nums.is_empty() {
                0
            } else {
                let (remaining_nums, new_next_char) = match nums[0] {
                    1 => (nums[1..].to_vec(), Empty),
                    num => ([vec![num - 1], nums[1..].to_vec()].concat(), Spring),
                };
                num_arrangements_rec(components[1..].to_vec(), remaining_nums, new_next_char)
            }
        }
        (Unknown, Unknown) => {
            num_arrangements_rec(components.clone(), nums.clone(), Spring)
                + num_arrangements_rec(components, nums, Empty)
        }
        _ => 0,
    }
}

fn num_arrangements(line: String, multiplier: u32) -> Result<u64, D12Error> {
    let Some((chars, nums)) = line.split_once(' ') else {
        return Err(D12Error::InvalidLine);
    };
    let components = chars
        .chars()
        .map(|c| {
            Ok(match c {
                '.' => Empty,
                '#' => Spring,
                '?' => Unknown,
                _ => return Err(D12Error::InvalidLine),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let nums = nums
        .split(',')
        .map(|n| n.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    let simple_sol = num_arrangements_rec(components.clone(), nums.clone(), Unknown);

    if multiplier == 1 {
        Ok(simple_sol)
    } else {
        let components = [components.clone(), vec![Unknown], components].concat();
        let nums = [nums.clone(), nums].concat();
        let double_sol = num_arrangements_rec(components, nums, Unknown);
        // if double_sol % simple_sol != 0 {
        //     println!(
        //         "Warning: {} % {} == {}",
        //         double_sol,
        //         simple_sol,
        //         double_sol % simple_sol
        //     );
        // }
        let scaler = double_sol / simple_sol;
        let shift = double_sol % simple_sol;
        let mut sol = simple_sol;
        for _ in 1..multiplier {
            sol *= scaler;
            sol += shift;
        }
        Ok(sol)
    }
}

fn solve(fp: &str, multiplier: u32) -> Result<u64, D12Error> {
    let mut sum = 0;
    for line in get_lines(fp)? {
        sum += num_arrangements(line?, multiplier)?;
    }
    Ok(sum)
}

fn main() {
    let multiplier = match get_subquestion_arg().as_str() {
        "a" => 1,
        "b" => 5,
        x => {
            eprintln!("Invalid argument: {}", x);
            return;
        }
    };

    match solve("data/d12/a.txt", multiplier) {
        Ok(solution) => println!("{}", solution),
        Err(e) => eprintln!("{}", e),
    }
}
