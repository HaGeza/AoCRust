use std::collections::{HashMap, HashSet, VecDeque};

use y2023::util::d23::{construct_graph, D23Error, Graph, Node};

fn sort_topologically(graph: &Graph, node: Node) -> Result<Vec<Node>, D23Error> {
    let mut visited = HashSet::new();
    let mut topological_ord = vec![];

    let mut dq = VecDeque::from([node]);
    while !dq.is_empty() {
        let node = dq.pop_front().unwrap();
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);

        for (neighbor, _) in graph.get(&node).unwrap() {
            dq.push_back(*neighbor);
        }
        topological_ord.push(node);
    }

    Ok(topological_ord)
}

fn solve(fp: &str) -> Result<u32, D23Error> {
    let (graph, source, sink) = construct_graph(fp)?;

    // Get the longest path from source to sink
    let topological_ord = sort_topologically(&graph, source)?;
    let mut dists = HashMap::from([(source, 0)]);
    for node in topological_ord {
        let dist = dists.get(&node).unwrap().clone();
        for (neighbor, edge_dist) in graph.get(&node).unwrap() {
            let new_dist = dist + edge_dist;
            let old_dist = dists.entry(*neighbor).or_insert(0);
            *old_dist = (*old_dist).max(new_dist);
        }
    }

    Ok(*dists.get(&sink).unwrap())
}

fn main() {
    match solve("data/d23/a.txt") {
        Ok(sol) => println!("{}", sol),
        Err(e) => println!("Error: {}", e),
    }
}
