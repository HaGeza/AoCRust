use thiserror::Error;

#[derive(Error, Debug)]
pub enum D5Error {
    #[error("File not found: {0}")]
    FileNotFound(#[from] std::io::Error),
    #[error("No seeds line")]
    NoSeedsLine,
    #[error("Invalid seeds line")]
    InvalidSeedsLine,
    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}
