use thiserror::Error;
use y2023::get_lines;

#[derive(Debug, Error)]
enum D15Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("empty input")]
    EmptyInput,
}

fn simple_hash(s: &str) -> u64 {
    let mut hashed = 0;
    for c in s.chars() {
        hashed += c as u64;
        hashed = (hashed * 17) % 256;
    }
    hashed
}

fn solve(fp: &str) -> Result<u64, D15Error> {
    let Some(line) = get_lines(fp)?.next() else {
        return Err(D15Error::EmptyInput);
    };

    let mut sum = 0;

    for s in line?.split(',') {
        sum += simple_hash(s);
    }

    Ok(sum)
}

fn main() {
    match solve("data/d15/a.txt") {
        Ok(sol) => println!("{}", sol),
        Err(e) => println!("Error: {}", e),
    }
}
