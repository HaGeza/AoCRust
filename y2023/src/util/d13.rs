use thiserror::Error;

use crate::get_lines;

#[derive(Debug, Error)]
pub enum D13Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("parse error")]
    ParseInt(#[from] std::num::ParseIntError),
}

pub fn solve_d13<F>(fp: &str, summary_fn: F) -> Result<u64, D13Error>
where
    F: Fn(&Vec<Vec<bool>>) -> u64,
{
    let mut sum = 0;

    let mut matrix = vec![];
    for line in get_lines(fp)? {
        let line = line?;
        if line.trim().is_empty() {
            sum += summary_fn(&matrix);
            matrix.clear();
        } else {
            matrix.push(line.chars().map(|c| c == '#').collect::<Vec<_>>());
        }
    }
    sum += summary_fn(&matrix);

    Ok(sum)
}
