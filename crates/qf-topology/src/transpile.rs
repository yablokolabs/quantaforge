use crate::error::TopologyError;
use crate::graph::QubitGraph;

/// A sequence of SWAP gates and the resulting qubit mapping after routing.
#[derive(Debug, Clone)]
pub struct SwapRoute {
    pub swaps: Vec<(usize, usize)>,
    pub final_mapping: Vec<usize>,
}

/// Greedy routing: find the shortest path between the physical qubits that
/// currently hold `logical_q1` and `logical_q2`, then insert SWAP gates along
/// the path to bring them adjacent.
///
/// `current_mapping[logical] = physical` — maps logical qubit indices to
/// physical qubit ids in the topology.
pub fn route_gate(
    graph: &QubitGraph,
    logical_q1: usize,
    logical_q2: usize,
    current_mapping: &[usize],
) -> Result<SwapRoute, TopologyError> {
    if logical_q1 >= current_mapping.len() {
        return Err(TopologyError::QubitNotFound(logical_q1));
    }
    if logical_q2 >= current_mapping.len() {
        return Err(TopologyError::QubitNotFound(logical_q2));
    }

    let phys1 = current_mapping[logical_q1];
    let phys2 = current_mapping[logical_q2];

    // Already adjacent — no swaps needed
    if graph.are_connected(phys1, phys2) {
        return Ok(SwapRoute {
            swaps: vec![],
            final_mapping: current_mapping.to_vec(),
        });
    }

    let path = graph.shortest_path(phys1, phys2)?;

    // Walk along the path, swapping the first qubit toward the second.
    // We need to move phys1 along until it is adjacent to phys2,
    // i.e., perform swaps for path[0]-path[1], path[1]-path[2], ..., path[n-3]-path[n-2].
    let mut mapping = current_mapping.to_vec();
    let mut swaps = Vec::new();

    // Build a reverse map: physical -> logical
    let mut phys_to_logical: std::collections::HashMap<usize, usize> =
        mapping.iter().enumerate().map(|(l, &p)| (p, l)).collect();

    for i in 0..path.len() - 2 {
        let a = path[i];
        let b = path[i + 1];
        swaps.push((a, b));

        // Update mapping: swap the logical assignments of physical qubits a and b
        let log_a = phys_to_logical.get(&a).copied();
        let log_b = phys_to_logical.get(&b).copied();

        if let Some(la) = log_a {
            mapping[la] = b;
        }
        if let Some(lb) = log_b {
            mapping[lb] = a;
        }

        // Update reverse map
        if let Some(la) = log_a {
            phys_to_logical.insert(b, la);
        } else {
            phys_to_logical.remove(&b);
        }
        if let Some(lb) = log_b {
            phys_to_logical.insert(a, lb);
        } else {
            phys_to_logical.remove(&a);
        }
    }

    Ok(SwapRoute {
        swaps,
        final_mapping: mapping,
    })
}
