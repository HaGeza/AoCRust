use std::{
    collections::{HashMap, HashSet},
    u16,
};

use thiserror::Error;
use y2023::get_lines;

#[derive(Debug, Error)]
enum D25Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("invalid line in input: {0}")]
    InvalidLine(String),
    #[error("no cut of size 3 found")]
    NoCutOf3Found,
}

fn insert_neighbors(
    source: String,
    neighbors: HashMap<String, u16>,
    graph: &mut HashMap<String, HashMap<String, u16>>,
) {
    if let Some(node) = graph.get_mut(&source) {
        node.extend(neighbors);
    } else {
        graph.insert(source, neighbors);
    }
}

fn get_tightest_node(weights: &HashMap<String, u16>) -> (String, u16) {
    weights.iter().fold(("".to_string(), 0), |acc, (node, w)| {
        if *w > acc.1 {
            (node.clone(), *w)
        } else {
            acc
        }
    })
}

fn insert_or_increase_weight(map: &mut HashMap<String, u16>, key: String, weight: u16) {
    let node_w = map.entry(key).or_insert(0);
    *node_w += weight;
}

fn solve(fp: &str) -> Result<usize, D25Error> {
    let mut graph = HashMap::new();

    // Construct graph
    for line in get_lines(fp)? {
        let line = line?;
        let Some((source, rest)) = line.split_once(':') else {
            return Err(D25Error::InvalidLine(line.to_string()));
        };
        let source = source.trim().to_string();

        let others = rest
            .trim()
            .split_whitespace()
            .map(|s| (s.to_string(), 1))
            .collect::<HashMap<_, _>>();
        insert_neighbors(source.clone(), others.clone(), &mut graph);
        for other in others {
            insert_neighbors(other.0, HashMap::from([(source.clone(), 1)]), &mut graph);
        }
    }

    let start_node = graph.keys().next().unwrap().clone();

    let mut graph_size = graph.keys().len();
    let original_graph_size = graph_size;
    let mut cut_size = 0;

    while graph_size > 1 {
        let mut weights = HashMap::new();
        for (node, w) in graph.get(&start_node).unwrap() {
            insert_or_increase_weight(&mut weights, node.clone(), *w);
        }
        let mut seen = HashSet::from([start_node.clone()]);

        let mut prev_node = start_node.clone();
        let mut current_node = start_node.clone();

        while !weights.is_empty() {
            prev_node = current_node;
            (current_node, cut_size) = get_tightest_node(&weights);

            for (node, w) in graph.get(&current_node).unwrap() {
                if !seen.contains(node) {
                    insert_or_increase_weight(&mut weights, node.clone(), *w);
                }
            }
            weights.remove(&current_node);
            seen.insert(current_node.clone());
        }

        if cut_size <= 3 {
            let cut_off_size = current_node.split_whitespace().count();
            return Ok((original_graph_size - cut_off_size) * cut_off_size);
        }

        // merge prev_node with current_node
        let new_node = [prev_node.clone(), current_node.clone()].join(" ");

        let prev_neighbors = graph.remove(&prev_node).unwrap();
        let current_neighbors = graph.remove(&current_node).unwrap();
        let mut new_neighbors = HashMap::new();
        for (node, w) in prev_neighbors.into_iter().chain(current_neighbors) {
            if node == prev_node || node == current_node {
                continue;
            }
            insert_or_increase_weight(&mut new_neighbors, node, w);
        }

        for (node, w) in new_neighbors.iter() {
            let node_neighbors = graph.get_mut(node).unwrap();
            node_neighbors.remove(&prev_node);
            node_neighbors.remove(&current_node);
            node_neighbors.insert(new_node.clone(), *w);
        }

        graph.insert(new_node, new_neighbors);
        graph_size -= 1;
    }

    Err(D25Error::NoCutOf3Found)
}

fn main() {
    match solve("data/d25/a.txt") {
        Ok(sol) => println!("{}", sol),
        Err(e) => eprintln!("Error: {}", e),
    }
}
