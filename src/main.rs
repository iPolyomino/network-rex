mod graph;

fn main() {
    let g = graph::Graph {
        name: String::from("sample"),
        nodes: vec![1, 2, 3],
    };
    print!("{}", g);
}
