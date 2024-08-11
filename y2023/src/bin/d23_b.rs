use std::collections::HashSet;

use y2023::util::d23::{construct_graph, D23Error, Graph, Node};

fn get_max_dist(max_dist: Option<u32>, dist: u32) -> Option<u32> {
    match max_dist {
        Some(max_dist) => Some(max_dist.max(dist)),
        None => Some(dist),
    }
}

fn dfs_max(
    visited: &mut HashSet<Node>,
    graph: &Graph,
    v: Node,
    w: Node,
    take_from_end: bool,
) -> Option<u32> {
    let (node, other) = if take_from_end { (w, v) } else { (v, w) };

    let mut max_dist: Option<u32> = None;
    for (neighbor, edge_dist) in graph.get(&node).unwrap() {
        if visited.contains(neighbor) {
            if neighbor == &other {
                max_dist = get_max_dist(max_dist, *edge_dist);
            }
            continue;
        }
        visited.insert(*neighbor);
        let dist = if take_from_end {
            dfs_max(visited, graph, v, *neighbor, false)
        } else {
            dfs_max(visited, graph, *neighbor, w, true)
        };
        visited.remove(neighbor);
        if let Some(dist) = dist {
            max_dist = get_max_dist(max_dist, *edge_dist + dist);
        }
    }

    max_dist
}

fn solve(fp: &str) -> Result<u32, D23Error> {
    let (graph, source, sink) = construct_graph(fp)?;
    let mut undirected_graph = graph.clone();

    for (node, neighbors) in &graph {
        for (neighbor, edge_dist) in neighbors {
            undirected_graph
                .get_mut(neighbor)
                .unwrap()
                .push((*node, *edge_dist));
        }
    }
    let graph = undirected_graph;

    let mut visited = HashSet::from([source]);
    Ok(dfs_max(&mut visited, &graph, source, sink, false).unwrap_or(0))
}

fn main() {
    match solve("data/d23/a.txt") {
        Ok(sol) => println!("{}", sol),
        Err(e) => println!("Error: {}", e),
    }
}
