use thiserror::Error;

use crate::get_lines;

#[derive(Debug, Error)]
pub enum D21Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("invalid character in input: {0}")]
    InvalidInputChar(char),
    #[error("part 2 error: {0}")]
    Part2Error(String),
    #[error("invalid distance matrix")]
    InvalidDistMatrix,
}

pub fn read_garden_matrix(fp: &str) -> Result<(Vec<Vec<bool>>, (usize, usize)), D21Error> {
    let mut walkable = vec![];
    let mut start = (0, 0);

    for line in get_lines(fp)? {
        let mut new_row = vec![];
        for c in line?.chars() {
            let w = match c {
                '.' => true,
                'S' => {
                    start = (walkable.len(), new_row.len());
                    true
                }
                '#' => false,
                _ => return Err(D21Error::InvalidInputChar(c)),
            };
            new_row.push(w);
        }
        walkable.push(new_row);
    }

    Ok((walkable, start))
}
