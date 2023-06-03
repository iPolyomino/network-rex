use std::fmt::{Display, Formatter};

fn main() {
    let g = Graph {
        name: String::from("sample"),
    };
    print!("{}", g);
}

struct Graph {
    name: String,
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Graph: {}", self.name)
    }
}
