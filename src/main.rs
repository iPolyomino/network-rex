mod graph;

fn main() {
    let g = graph::Graph {
        name: String::from("sample"),
        nodes: vec![1, 2, 3],
        edges: vec![(1, 2), (2, 3)],
    };
    print!("{}", g);
    for (x, y) in g.edges_list(2) {
        println!("{} - {}", x, y);
    }
}
