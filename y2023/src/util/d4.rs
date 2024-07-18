use std::{collections::HashSet, num::ParseIntError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum D4Error {
    #[error("File not found: {0}")]
    FileNotFound(#[from] std::io::Error),
    #[error("Regex error: {0}")]
    InvalidRegex(#[from] regex::Error),
    #[error("Regex does not match")]
    NoRegexMatch,
    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Number of extra cards not provided")]
    MissingExtraCardsNumber,
}

pub fn parse_numbers(cap: &str) -> Result<Vec<u32>, ParseIntError> {
    cap.split_whitespace()
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()
}

pub fn get_winning_set_and_own_array(line: &str) -> Result<(HashSet<u32>, Vec<u32>), D4Error> {
    let re = regex::Regex::new(r"^Card +\d+:([^|]*)\|(.*)$")?;
    let Some(caps) = re.captures(&line) else {
        return Err(D4Error::NoRegexMatch);
    };

    let winning = parse_numbers(&caps[1])?;
    let own = parse_numbers(&caps[2])?;

    let mut winning_set = HashSet::new();
    for w in winning {
        winning_set.insert(w);
    }

    Ok((winning_set, own))
}
