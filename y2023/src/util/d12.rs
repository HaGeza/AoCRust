use thiserror::Error;

#[derive(Debug, Error)]
pub enum D12Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("invalid line")]
    InvalidLine,
    #[error("invalid number: {0}")]
    InvalidNumber(#[from] std::num::ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spring {
    Broken,
    Working,
    Unknown,
}
use Spring::*;

pub fn get_springs_and_nums(line: String) -> Result<(Vec<Spring>, Vec<u32>), D12Error> {
    let Some((chars, nums)) = line.split_once(' ') else {
        return Err(D12Error::InvalidLine);
    };
    let springs = chars
        .chars()
        .map(|c| {
            Ok(match c {
                '.' => Broken,
                '#' => Working,
                '?' => Unknown,
                _ => return Err(D12Error::InvalidLine),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let nums = nums
        .split(',')
        .map(|n| n.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok((springs, nums))
}
