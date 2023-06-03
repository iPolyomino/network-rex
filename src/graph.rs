use std::fmt::{Display, Formatter};

pub struct Graph {
    pub name: String,
    pub nodes: Vec<u32>,
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Graph: {}", self.name)
    }
}
