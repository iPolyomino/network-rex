use std::fmt::{Display, Formatter};

pub struct Graph {
    pub name: String,
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Graph: {}", self.name)
    }
}
