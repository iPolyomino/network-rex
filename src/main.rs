mod graph;

fn main() {
    let g = graph::Graph {
        name: String::from("sample"),
        nodes: vec![1, 2, 3, 4, 5, 6, 7],
        edges: vec![(1, 2), (1, 4), (1, 5), (2, 3), (2, 4), (3, 7), (5, 6)],
    };
    print!("{}", g);
    println!("{:?}", g.edges_list(2));
}
