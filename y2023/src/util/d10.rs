use std::collections::VecDeque;

use crate::get_lines;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum D10Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("incorrect matrix")]
    MatrixError,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }
}

pub enum Cell {
    Empty,
    Start,
    Pipe(Direction, Direction, Option<i32>),
}

pub fn in_bounds(i: i32, j: i32, n: i32, m: i32) -> bool {
    i < n && i >= 0 && j < m && j >= 0
}

pub fn get_filled_matrix(fp: &str) -> Result<Vec<Vec<Cell>>, D10Error> {
    let mut matrix = Vec::new();
    let mut start_pos = (0, 0);

    for (i, line) in get_lines(fp)?.enumerate() {
        let mut row = Vec::new();
        for (j, c) in line?.chars().enumerate() {
            row.push(match c {
                '|' => Cell::Pipe(Direction::N, Direction::S, None),
                '-' => Cell::Pipe(Direction::E, Direction::W, None),
                'L' => Cell::Pipe(Direction::N, Direction::E, None),
                'J' => Cell::Pipe(Direction::N, Direction::W, None),
                '7' => Cell::Pipe(Direction::S, Direction::W, None),
                'F' => Cell::Pipe(Direction::S, Direction::E, None),
                'S' => {
                    start_pos = (i as i32, j as i32);
                    Cell::Start
                }
                _ => Cell::Empty,
            })
        }
        matrix.push(row);
    }

    let mut stack = VecDeque::new();
    let mut start_directions = Vec::new();
    let n = matrix.len() as i32;
    let m = matrix[0].len() as i32;

    let (si, sj) = start_pos;
    for ((i, j), d) in [
        ((si - 1, sj), Direction::S),
        ((si, sj + 1), Direction::W),
        ((si + 1, sj), Direction::N),
        ((si, sj - 1), Direction::E),
    ]
    .iter()
    {
        if in_bounds(*i, *j, n, m) {
            match &matrix[*i as usize][*j as usize] {
                Cell::Pipe(d1, d2, None) => {
                    if *d1 == *d || *d2 == *d {
                        stack.push_front(((*i, *j), 1));
                        start_directions.push(d.clone());
                    }
                }
                _ => continue,
            }
        }
    }

    if start_directions.len() != 2 {
        return Err(D10Error::MatrixError);
    } else {
        matrix[si as usize][sj as usize] = Cell::Pipe(
            start_directions[0].opposite(),
            start_directions[1].opposite(),
            Some(0),
        );
    }

    while !stack.is_empty() {
        let ((i, j), distance) = stack.pop_back().unwrap();
        let cell = &matrix[i as usize][j as usize];

        matrix[i as usize][j as usize] = match cell {
            Cell::Pipe(d1, d2, None) => {
                for d in [d1, d2].iter() {
                    let (ii, jj) = match d {
                        Direction::N => (i - 1, j),
                        Direction::E => (i, j + 1),
                        Direction::S => (i + 1, j),
                        Direction::W => (i, j - 1),
                    };
                    if in_bounds(ii, jj, n, m) {
                        stack.push_front(((ii, jj), distance + 1))
                    }
                }

                Cell::Pipe(d1.clone(), d2.clone(), Some(distance))
            }
            _ => continue,
        }
    }

    Ok(matrix)
}
