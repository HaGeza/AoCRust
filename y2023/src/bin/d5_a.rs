use thiserror::Error;
use y2023::get_lines;

#[derive(Error, Debug)]
enum D5Error {
    #[error("File not found: {0}")]
    FileNotFound(#[from] std::io::Error),
    #[error("No seeds line")]
    NoSeedsLine,
    #[error("Invalid seeds line")]
    InvalidSeedsLine,
    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

fn min_location(fp: &str) -> Result<u64, D5Error> {
    let mut lines = get_lines(fp)?;

    let Some(Ok(seeds_line)) = lines.next() else {
        return Err(D5Error::NoSeedsLine);
    };
    let Some((_, seeds_part)) = seeds_line.split_once(":") else {
        return Err(D5Error::InvalidSeedsLine);
    };

    let mut seeds = seeds_part
        .split_whitespace()
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()?;
    let mut mapped = vec![false; seeds.len()];

    for line in lines {
        let line = line?;

        if line.is_empty() {
            continue;
        } else if line.contains("map:") {
            mapped = vec![false; seeds.len()];
        } else {
            let nums = line
                .split_whitespace()
                .map(|x| x.parse::<u64>())
                .collect::<Result<Vec<u64>, _>>()?;
            let (to_start, from_start, range) = (nums[0], nums[1], nums[2]);

            for j in 0..seeds.len() {
                if mapped[j] {
                    continue;
                }
                if seeds[j] >= from_start && seeds[j] < from_start + range {
                    seeds[j] = seeds[j] - from_start + to_start;
                    mapped[j] = true;
                }
            }
        }
    }

    let Some(&min_loc) = seeds.iter().min() else {
        return Err(D5Error::InvalidSeedsLine);
    };
    Ok(min_loc)
}

fn main() {
    match min_location("data/d5/a.txt") {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Error: {}", e),
    }
}
