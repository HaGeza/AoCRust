use thiserror::Error;

use crate::get_lines;

#[derive(Debug, Error)]
pub enum D14Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Empty,
    Round,
    Square,
}
use Cell::*;

pub fn get_grid(fp: &str) -> Result<Vec<Vec<Cell>>, D14Error> {
    let mut grid = vec![];

    for (i, line) in get_lines(fp)?.enumerate() {
        grid.push(vec![]);
        for c in line?.chars() {
            grid[i].push(match c {
                'O' => Round,
                '#' => Square,
                _ => Empty,
            });
        }
    }
    Ok(grid)
}
