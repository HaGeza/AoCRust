use crate::get_lines;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum D24Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("invalid line: {0}")]
    InvalidLine(String),
    #[error("parse error")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("part 2 unsolvable")]
    Part2Unsolvable,
}

pub fn str_to_triplet(s: &str) -> Result<[i64; 3], D24Error> {
    let nums = s
        .split(',')
        .map(|x| x.trim().parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()?;
    if nums.len() != 3 {
        return Err(D24Error::InvalidLine(s.to_string()));
    }
    Ok([nums[0], nums[1], nums[2]])
}

pub fn get_positions_and_velocities(fp: &str) -> Result<(Vec<[i64; 3]>, Vec<[i64; 3]>), D24Error> {
    let mut starts = vec![];
    let mut velocities = vec![];

    for line in get_lines(fp)? {
        let line = line?;
        let Some((pos_str, vel_str)) = line.split_once('@') else {
            return Err(D24Error::InvalidLine(line));
        };
        starts.push(str_to_triplet(pos_str)?);
        velocities.push(str_to_triplet(vel_str)?);
    }

    Ok((starts, velocities))
}
