use petgraph::visit::Bfs;

use crate::error::TopologyError;
use crate::graph::QubitGraph;

/// Validate that a two-qubit gate can be applied directly (qubits are adjacent).
pub fn validate_two_qubit_gate(
    graph: &QubitGraph,
    q1: usize,
    q2: usize,
) -> Result<(), TopologyError> {
    if graph.are_connected(q1, q2) {
        Ok(())
    } else {
        Err(TopologyError::NotConnected(q1, q2))
    }
}

/// Return the adjacency matrix for the graph (indexed by qubit id order).
pub fn adjacency_matrix(graph: &QubitGraph) -> Vec<Vec<bool>> {
    let mut ids = graph.all_qubits();
    ids.sort();
    let n = ids.len();
    let pos: std::collections::HashMap<usize, usize> =
        ids.iter().enumerate().map(|(i, &id)| (id, i)).collect();

    let mut mat = vec![vec![false; n]; n];
    for &id in &ids {
        if let Ok(nbrs) = graph.neighbors(id) {
            for nb in nbrs {
                if let (Some(&a), Some(&b)) = (pos.get(&id), pos.get(&nb)) {
                    mat[a][b] = true;
                    mat[b][a] = true;
                }
            }
        }
    }
    mat
}

/// Return the maximum degree across all qubits in the graph.
pub fn max_degree(graph: &QubitGraph) -> usize {
    graph
        .all_qubits()
        .iter()
        .filter_map(|&q| graph.degree(q).ok())
        .max()
        .unwrap_or(0)
}

/// Check if the graph is connected (all qubits reachable from any starting qubit).
pub fn is_connected(graph: &QubitGraph) -> bool {
    let inner = graph.inner();
    let total = inner.node_count();
    if total <= 1 {
        return true;
    }

    let start = inner.node_indices().next().unwrap();
    let mut bfs = Bfs::new(inner, start);
    let mut visited = 0usize;
    while bfs.next(inner).is_some() {
        visited += 1;
    }
    visited == total
}
