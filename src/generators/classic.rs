use itertools::Itertools;

use crate::graph::Graph;

pub fn complete_graph(n: u32) -> Graph {
    if n <= 1 {
        return empty_graph(n);
    }

    let edges: Vec<(u32, u32)> = (0..n).tuple_combinations().collect();

    return Graph {
        name: String::from("complete graph"),
        nodes: (0..n).collect(),
        edges,
    };
}

pub fn empty_graph(n: u32) -> Graph {
    // Returns the empty graph with n nodes and zero edges
    Graph {
        name: String::from("empty graph"),
        nodes: (0..n).collect(),
        edges: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_graph() {
        let cg = complete_graph(5);
        assert_eq!(cg.nodes.len(), 5);
        assert_eq!(cg.edges.len(), 10);
    }

    #[test]
    fn test_empty_graph() {
        let eg = empty_graph(7);
        assert_eq!(eg.nodes.len(), 7);
        assert_eq!(eg.edges.len(), 0);
    }
}
