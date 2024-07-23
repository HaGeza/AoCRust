use thiserror::Error;
use y2023::{get_lines, get_subquestion_arg};

#[derive(Debug, Error)]
enum D9Error {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
}

enum Direction {
    Next,
    Previous,
}

fn predict_next(line: String) -> Result<i64, D9Error> {
    let mut nums: Vec<i64> = line
        .split_whitespace()
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()?;
    let mut ends = vec![];

    while nums.iter().any(|&n| n != 0) && nums.len() > 1 {
        for i in 0..nums.len() - 1 {
            nums[i] = nums[i + 1] - nums[i];
        }
        ends.push(nums.pop().unwrap());
    }

    let prediction = ends.iter().fold(*nums.last().unwrap(), |acc, &n| acc + n);
    Ok(prediction)
}

fn predict_prev(line: String) -> Result<i64, D9Error> {
    let mut nums: Vec<i64> = line
        .split_whitespace()
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()?;
    let mut starts = vec![];

    while nums.iter().any(|&n| n != 0) && nums.len() > 1 {
        starts.push(nums[0]);
        for i in 0..nums.len() - 1 {
            nums[i] = nums[i + 1] - nums[i];
        }
        nums.pop();
    }

    let prediction = starts
        .iter()
        .rev()
        .fold(*nums.first().unwrap(), |acc, &n| n - acc);

    Ok(prediction)
}

fn prediction_sum(fp: &str, direction: Direction) -> Result<i64, D9Error> {
    let mut sum = 0;

    for line in get_lines(fp)? {
        sum += match direction {
            Direction::Next => predict_next(line?)?,
            Direction::Previous => predict_prev(line?)?,
        };
    }
    Ok(sum)
}

fn main() {
    let direction = match get_subquestion_arg().as_str() {
        "a" => Direction::Next,
        "b" => Direction::Previous,
        x => {
            eprintln!("Invalid argument: {}", x);
            return;
        }
    };

    match prediction_sum("data/d9/a.txt", direction) {
        Ok(solution) => println!("{}", solution),
        Err(e) => eprintln!("Error: {}", e),
    }
}
