use itertools::Itertools;
use rand::random;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::iter::repeat;

use crate::exception::NetworkRexError;
use crate::generators::classic::{complete_graph, empty_graph, star_graph};
use crate::graph::Graph;

pub fn gnp_random_graph(n: u32, p: f32) -> Graph {
    let graph = Graph {
        name: String::from("random graph"),
        nodes: (0..n).collect(),
        edges: vec![],
    };

    if p <= 0.0 {
        return graph;
    }
    if p >= 1.0 {
        return complete_graph(n);
    }

    let edges: Vec<(u32, u32)> = (0..n)
        .tuple_combinations()
        .filter(|_vec| random::<f32>() < p)
        .collect();

    return Graph { edges, ..graph };
}

pub fn random_regular_graph(d: u32, n: u32) -> Result<Graph, NetworkRexError> {
    if (d * n) % 2 != 0 {
        return Err(NetworkRexError {
            message: String::from("n * d must be even"),
        });
    }
    if !(d < n) {
        return Err(NetworkRexError {
            message: String::from("the 0 <= d < n inequality must be satisfied"),
        });
    }

    if d == 0 {
        return Ok(empty_graph(n));
    }

    fn suitable(edges: &HashSet<(u32, u32)>, potential_edges: &HashMap<u32, u32>) -> bool {
        // Helper subroutine to check if there are suitable edges remaining
        // If False, the generation of the graph has failed
        if potential_edges.is_empty() {
            return true;
        }

        for &(mut s1) in potential_edges.keys() {
            for &(mut s2) in potential_edges.keys() {
                if s1 == s2 {
                    break;
                }
                if s1 > s2 {
                    (s1, s2) = (s2, s1)
                }
                if !edges.contains(&(s1, s2)) {
                    return true;
                }
            }
        }
        return false;
    }

    fn try_creation(n: u32, d: u32) -> Option<HashSet<(u32, u32)>> {
        // Attempt to create an edge set

        let mut edges: HashSet<(u32, u32)> = HashSet::new();
        let mut stubs: Vec<u32> = (0..n).cycle().take((n * d) as usize).collect();
        let mut rng = rand::thread_rng();

        while !stubs.is_empty() {
            let mut potential_edges: HashMap<u32, u32> = HashMap::new();

            stubs.shuffle(&mut rng);

            let mut stubiter = stubs.iter();
            while let (Some(&(mut s1)), Some(&(mut s2))) = (stubiter.next(), stubiter.next()) {
                if s1 > s2 {
                    (s1, s2) = (s2, s1)
                }
                if s1 != s2 && !edges.contains(&(s1, s2)) {
                    edges.insert((s1, s2));
                } else {
                    *potential_edges.entry(s1).or_insert(0) += 1;
                    *potential_edges.entry(s2).or_insert(0) += 1;
                }
            }

            if !suitable(&edges, &potential_edges) {
                return None;
            }

            stubs = potential_edges
                .iter()
                .flat_map(|(&node, &potential)| repeat(node).take(potential as usize))
                .collect();
        }
        return Some(edges);
    }

    // Even though a suitable edge set exists,
    // the generation of such a set is not guaranteed.
    // Try repeatedly to find one.
    let edges = loop {
        if let Some(edges) = try_creation(n, d) {
            break edges;
        }
    };

    let g = Graph {
        name: String::from("random graph"),
        nodes: (0..n).collect(),
        edges: edges.into_iter().collect(),
    };

    return Ok(g);
}

fn random_subset(seq: &Vec<u32>, m: u32) -> HashSet<u32> {
    let mut targets = HashSet::new();
    let mut rng = rand::thread_rng();
    while targets.len() < m as usize {
        if let Some(&x) = seq.choose(&mut rng) {
            targets.insert(x);
        }
    }
    targets
}

pub fn barabasi_albert_graph(n: u32, m: u32) -> Result<Graph, NetworkRexError> {
    if m < 1 || m >= n {
        return Err(NetworkRexError {
            message: format!(
                "Barabási–Albert network must have m >= 1 and m < n, m = {}, n = {}",
                m, n
            ),
        });
    }
    let mut graph = star_graph(m);
    let mut repeated_nodes: Vec<u32> = graph
        .degree()
        .into_iter()
        .flat_map(|(n, d)| repeat(n).take(d))
        .collect();

    let mut source = graph.nodes.len() as u32;
    while source < n {
        let targets = random_subset(&repeated_nodes, m);

        graph.add_node(source);
        targets
            .iter()
            .for_each(|&target| graph.add_edge(source, target));

        repeated_nodes.extend(targets);
        repeated_nodes.extend(repeat(source).take(m as usize));

        source += 1;
    }

    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_regular_graph() {
        // Valid parameters
        let result = random_regular_graph(2, 4);
        let expected_graph = Graph {
            name: String::from("random graph"),
            nodes: vec![0, 1, 2, 3],
            edges: vec![(0, 1), (0, 2), (1, 3), (2, 3)],
        };
        assert!(result.is_ok());
        let graph = result.unwrap();
        assert_eq!(graph.name, expected_graph.name);
        assert_eq!(graph.nodes, expected_graph.nodes);
        assert_eq!(graph.edges.len(), expected_graph.edges.len());

        let zero_d = random_regular_graph(0, 5).unwrap();
        assert_eq!(zero_d.edges.len(), 0);
        assert_eq!(zero_d.nodes, vec![0, 1, 2, 3, 4]);

        let odd_parameter = random_regular_graph(3, 5);
        let expected_odd_parameter_err = Err(NetworkRexError {
            message: String::from("n * d must be even"),
        });
        assert_eq!(odd_parameter, expected_odd_parameter_err);

        let large_d = random_regular_graph(5, 4);
        let expected_large_d_err = Err(NetworkRexError {
            message: String::from("the 0 <= d < n inequality must be satisfied"),
        });
        assert_eq!(large_d, expected_large_d_err);
    }
}
