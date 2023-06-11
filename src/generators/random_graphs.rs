use itertools::Itertools;
use rand::random;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::iter::repeat;

use crate::exception::NetworkRexError;
use crate::generators::classic::{complete_graph, empty_graph};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_regular_graph() {
        let rrg_zero_d = random_regular_graph(0, 5).unwrap();
        assert_eq!(rrg_zero_d.edges.len(), 0);
        assert_eq!(rrg_zero_d.nodes, vec![0, 1, 2, 3, 4]);
    }
}
