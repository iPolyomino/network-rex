mod graph;

fn main() {
    let g = graph::Graph {
        name: String::from("sample"),
    };
    print!("{}", g);
}
