#[derive(Debug, Error)]
pub enum D23Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("empty input")]
    EmptyInput,
    #[error("missing {0}")]
    MissingCell(String),
    #[error("invalid input char {0}")]
    InvalidInputChar(char),
}

#[derive(PartialEq, Eq)]
pub enum LabyrinthCell {
    Wall,
    Empty,
    Dir(i8, i8),
}
use std::collections::{HashMap, VecDeque};

use thiserror::Error;
use LabyrinthCell::*;

use crate::get_lines;

pub fn find_empty(line: String) -> Option<usize> {
    line.chars()
        .enumerate()
        .find(|(_, x)| *x == '.')
        .map(|(i, _)| i)
}

pub type Node = (usize, usize);
pub type Graph = HashMap<Node, Vec<(Node, u32)>>;

pub fn construct_graph(fp: &str) -> Result<(Graph, Node, Node), D23Error> {
    let mut graph = HashMap::new();

    let mut matrix = vec![];
    let mut visited = vec![];

    let mut first_line = None;
    let mut last_line = None;
    for line in get_lines(fp)? {
        let line = line?;
        if first_line.is_none() {
            first_line = Some(line.clone());
        }
        last_line = Some(line.clone());

        let mut visited_row = vec![];
        let mut matrix_row = vec![];
        for c in line.chars() {
            let cell = match c {
                '#' => Wall,
                '.' => Empty,
                '>' => Dir(0, 1),
                'v' => Dir(1, 0),
                '<' => Dir(0, -1),
                '^' => Dir(-1, 0),
                _ => return Err(D23Error::InvalidInputChar(c)),
            };
            visited_row.push(cell == Wall);
            matrix_row.push(cell);
        }
        visited.push(visited_row);
        matrix.push(matrix_row);
    }

    let n = matrix.len();
    let m = matrix[0].len();

    let first_line = first_line.ok_or(D23Error::EmptyInput)?;
    let last_line = last_line.ok_or(D23Error::EmptyInput)?;
    let source = (
        0,
        find_empty(first_line).ok_or(D23Error::MissingCell("source".to_string()))?,
    );
    let sink = (
        n - 1,
        find_empty(last_line).ok_or(D23Error::MissingCell("sink".to_string()))?,
    );

    graph.insert(source, vec![]);
    graph.insert(sink, vec![]);

    // Each element of the dq is a tuple of (current_node, current_dist, position)
    let mut dq = VecDeque::from([(source, 1, (source.0 + 1, source.1))]);

    while !dq.is_empty() {
        let mut buffer = vec![];
        let (node, dist, (i, j)) = dq.pop_front().unwrap();

        if visited[i][j] {
            if graph.contains_key(&(i, j)) {
                graph.get_mut(&node).unwrap().push(((i, j), dist));
            }
            continue;
        }
        visited[i][j] = true;

        let dirs = match matrix[i][j] {
            Empty => vec![(0, 1), (1, 0), (0, -1), (-1, 0)],
            Dir(di, dj) => vec![(di, dj)],
            Wall => unreachable!(),
        };

        let mut non_wall_neighbors = 0;
        for (di, dj) in dirs {
            let ii = i as i32 + di as i32;
            let jj = j as i32 + dj as i32;
            if ii < 0 || ii >= n as i32 || jj < 0 || jj >= m as i32 {
                continue;
            }
            let (ii, jj) = (ii as usize, jj as usize);
            non_wall_neighbors += if matrix[ii][jj] == Wall { 0 } else { 1 };
            if (ii, jj) == node {
                continue;
            }

            if graph.contains_key(&(ii, jj)) {
                graph.get_mut(&node).unwrap().push(((ii, jj), dist + 1));
            } else if !visited[ii][jj] {
                match matrix[ii][jj] {
                    Empty => buffer.push((ii, jj)),
                    Dir(dii, djj) => {
                        if (di + dii, dj + djj) != (0, 0) {
                            buffer.push((ii, jj));
                        }
                    }
                    Wall => (),
                }
            }
        }

        let (act_node, act_dist) = if non_wall_neighbors > 2 {
            graph.entry((i, j)).or_insert(vec![]);
            graph.get_mut(&node).unwrap().push(((i, j), dist));
            ((i, j), 1)
        } else {
            (node, dist + 1)
        };
        for (ii, jj) in buffer {
            dq.push_back((act_node, act_dist, (ii, jj)));
        }
    }

    Ok((graph, source, sink))
}
