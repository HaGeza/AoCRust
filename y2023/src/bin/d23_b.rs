use std::collections::HashSet;

use y2023::util::d23::{construct_graph, D23Error, Graph, Node};

fn dfs_max(visited: &mut HashSet<Node>, graph: &Graph, node: Node, sink: Node) -> Option<u32> {
    if node == sink {
        return Some(0);
    }

    let mut max_dist: Option<u32> = None;
    for (neighbor, edge_dist) in graph.get(&node).unwrap() {
        if visited.contains(neighbor) {
            continue;
        }
        visited.insert(*neighbor);
        let dist = dfs_max(visited, graph, *neighbor, sink);
        visited.remove(neighbor);
        if let Some(dist) = dist {
            max_dist = match max_dist {
                Some(max_dist) => Some(max_dist.max(dist + edge_dist)),
                None => Some(dist + edge_dist),
            };
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
    Ok(dfs_max(&mut visited, &graph, source, sink).unwrap_or(0))
}

fn main() {
    match solve("data/d23/a.txt") {
        Ok(sol) => println!("{}", sol),
        Err(e) => println!("Error: {}", e),
    }
}
