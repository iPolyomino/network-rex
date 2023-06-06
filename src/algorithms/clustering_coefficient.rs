use itertools::Itertools;
use rand::random;

use crate::graph::Graph;

pub fn gnp_random_graph(n: u32, p: f32) -> Graph {
    let graph = Graph {
        name: String::from("random graph"),
        nodes: (0..n).collect(),
        edges: vec![],
    };

    if p <= 0.0 {
        return graph;
    }
    if p >= 1.0 {
        // TODO: complete_graph
    }

    let edges: Vec<(u32, u32)> = (0..n)
        .tuple_combinations()
        .filter(|_vec| random::<f32>() < p)
        .collect();

    return Graph {
        edges: edges,
        ..graph
    };
}
