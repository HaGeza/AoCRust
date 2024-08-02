use thiserror::Error;

#[derive(Debug, Error)]
pub enum D15Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("empty input")]
    EmptyInput,
    #[error("invalid operation: {0}")]
    InvalidOperation(String),
    #[error("parse error")]
    ParseError(#[from] std::num::ParseIntError),
}

pub fn simple_hash(s: &str) -> u8 {
    let mut hashed: u16 = 0;
    for c in s.chars() {
        hashed += c as u16;
        hashed = (hashed * 17) % 256;
    }
    hashed as u8
}
