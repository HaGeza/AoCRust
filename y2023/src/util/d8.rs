use std::collections::HashMap;

use crate::get_lines;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum D8Error {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("regex error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("line does not match pattern: {0}")]
    NonMatchingLine(String),
    #[error("unreachable")]
    Unreachable,
    #[error("invalid input")]
    InvalidInput,
}

pub const START_NODE: &str = "AAA";
pub const END_NODE: &str = "ZZZ";

#[derive(Debug, Clone, Default)]
pub struct Node {
    left: Option<String>,
    right: Option<String>,
}

pub fn read_environment(fp: &str) -> Result<(Vec<u8>, HashMap<String, Node>), D8Error> {
    let mut nodes = HashMap::new();
    let line_pattern =
        regex::Regex::new(r"(?<node>\w+) *= *\( *(?<left>\w+) *, *(?<right>\w+) *\) *")?;

    let mut lines = get_lines(fp)?;
    let moves = lines
        .next()
        .ok_or(D8Error::InvalidInput)??
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Vec<u8>>();

    for line in lines.skip(1) {
        let line = line?;

        let caps = line_pattern
            .captures(&line)
            .ok_or(D8Error::NonMatchingLine(line.to_string()))?;

        for i in ["node", "left", "right"] {
            if !nodes.contains_key(&caps[i].to_string()) {
                nodes.insert(caps[i].to_string(), Node::default());
            }
        }

        let node = nodes
            .get_mut(&caps["node"].to_string())
            .ok_or(D8Error::Unreachable)?;
        node.left = Some(caps["left"].to_string());
        node.right = Some(caps["right"].to_string());
    }

    Ok((moves, nodes))
}

pub fn steps_from_a_to_b(
    moves: &Vec<u8>,
    nodes: &HashMap<String, Node>,
    a: String,
    zs: &Vec<String>,
) -> Result<u64, D8Error> {
    let mut current_node = a;
    let mut move_ind = 0;
    let mut steps = 0;

    while !zs.contains(&current_node) {
        let node = nodes.get(&current_node).ok_or(D8Error::Unreachable)?;
        current_node = if moves[move_ind] == 0 {
            node.left.as_ref().ok_or(D8Error::Unreachable)?
        } else {
            node.right.as_ref().ok_or(D8Error::Unreachable)?
        }
        .to_string();

        steps += 1;
        move_ind = (move_ind + 1) % moves.len();
    }

    Ok(steps)
}
