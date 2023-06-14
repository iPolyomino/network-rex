use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Graph {
    pub name: String,
    pub nodes: Vec<u32>,
    pub edges: Vec<(u32, u32)>,
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Graph with {} nodes and {} edges",
            self.nodes.len(),
            self.edges.len()
        )
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            name: String::from(""),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn edges_list(&self, edge: u32) -> Vec<(u32, u32)> {
        self.edges
            .iter()
            .filter(|&&(x, y)| x == edge || y == edge)
            .copied()
            .collect()
    }

    pub fn has_node(&self, node: &u32) -> bool {
        self.nodes.contains(node)
    }

    pub fn add_node(&mut self, node: u32) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, source: u32, target: u32) {
        if !self.nodes.contains(&source) {
            self.add_node(source);
        }
        if !self.nodes.contains(&target) {
            self.add_node(target);
        }
        self.edges.push((source, target));
    }

    pub fn degree(&self) -> Vec<(u32, usize)> {
        self.nodes
            .iter()
            .map(|&n| (n, self.edges_list(n).len()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_list() {
        let g = Graph {
            name: String::from("sample"),
            nodes: vec![1, 2, 3],
            edges: vec![(1, 2), (2, 3)],
        };
        assert_eq!(g.edges_list(1), vec![(1, 2)]);
        assert_eq!(g.edges_list(2), vec![(1, 2), (2, 3)]);
        assert_eq!(g.edges_list(3), vec![(2, 3)]);
    }

    #[test]
    fn has_node() {
        let g = Graph {
            name: String::from("sample"),
            nodes: vec![1, 2, 3],
            edges: vec![],
        };
        assert_eq!(g.has_node(&1), true);
    }

    #[test]
    fn add_edge() {
        let mut g = Graph::new();
        g.add_edge(1, 2);

        assert_eq!(g.edges, vec![(1, 2)]);
        assert_eq!(g.nodes, vec![1, 2]);
    }
}
