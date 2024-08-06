use thiserror::Error;

#[derive(Debug, Error)]
pub enum D18Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("parse error")]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid input")]
    InvalidInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Dir {
    R,
    D,
    L,
    U,
}

pub fn get_movement(dir: Dir, dist: i64) -> Result<(i64, i64), D18Error> {
    match dir {
        Dir::R => Ok((0, dist)),
        Dir::D => Ok((dist, 0)),
        Dir::L => Ok((0, -dist)),
        Dir::U => Ok((-dist, 0)),
    }
}

pub fn parse_line(line: &str) -> Result<(Dir, i64, String), D18Error> {
    let Some((dir, rest)) = line.split_once(' ') else {
        return Err(D18Error::InvalidInput);
    };
    let Some((dist, col)) = rest.split_once(' ') else {
        return Err(D18Error::InvalidInput);
    };

    let dir = match dir.chars().next() {
        Some('R') => Dir::R,
        Some('D') => Dir::D,
        Some('L') => Dir::L,
        Some('U') => Dir::U,
        _ => return Err(D18Error::InvalidInput),
    };
    let dist = dist.parse::<i64>()?;

    Ok((dir, dist, col.to_string()))
}
