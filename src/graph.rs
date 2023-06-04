use std::fmt::{Display, Formatter};

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_node() {
        let g = Graph {
            name: String::from("sample"),
            nodes: vec![1, 2, 3],
            edges: vec![],
        };
        assert_eq!(g.has_node(&1), true);
    }
}
