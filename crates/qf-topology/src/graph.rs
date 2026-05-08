use std::collections::HashMap;

use petgraph::algo::astar;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};

use crate::error::TopologyError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QubitNode {
    pub id: usize,
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub struct QubitGraph {
    graph: UnGraph<QubitNode, ()>,
    id_to_index: HashMap<usize, NodeIndex>,
}

impl QubitGraph {
    pub fn new() -> Self {
        Self {
            graph: UnGraph::new_undirected(),
            id_to_index: HashMap::new(),
        }
    }

    pub fn add_qubit(&mut self, id: usize, row: usize, col: usize) -> NodeIndex {
        let idx = self.graph.add_node(QubitNode { id, row, col });
        self.id_to_index.insert(id, idx);
        idx
    }

    pub fn add_connection(&mut self, q1: usize, q2: usize) -> Result<(), TopologyError> {
        let &idx1 = self
            .id_to_index
            .get(&q1)
            .ok_or(TopologyError::QubitNotFound(q1))?;
        let &idx2 = self
            .id_to_index
            .get(&q2)
            .ok_or(TopologyError::QubitNotFound(q2))?;
        self.graph.add_edge(idx1, idx2, ());
        Ok(())
    }

    pub fn num_qubits(&self) -> usize {
        self.graph.node_count()
    }

    pub fn num_connections(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn are_connected(&self, q1: usize, q2: usize) -> bool {
        let (Some(&idx1), Some(&idx2)) = (self.id_to_index.get(&q1), self.id_to_index.get(&q2))
        else {
            return false;
        };
        self.graph.find_edge(idx1, idx2).is_some()
    }

    pub fn neighbors(&self, qubit: usize) -> Result<Vec<usize>, TopologyError> {
        let &idx = self
            .id_to_index
            .get(&qubit)
            .ok_or(TopologyError::QubitNotFound(qubit))?;
        let ids = self
            .graph
            .edges(idx)
            .map(|e| {
                let other = if e.source() == idx {
                    e.target()
                } else {
                    e.source()
                };
                self.graph[other].id
            })
            .collect();
        Ok(ids)
    }

    pub fn degree(&self, qubit: usize) -> Result<usize, TopologyError> {
        let &idx = self
            .id_to_index
            .get(&qubit)
            .ok_or(TopologyError::QubitNotFound(qubit))?;
        Ok(self.graph.edges(idx).count())
    }

    pub fn shortest_path(&self, from: usize, to: usize) -> Result<Vec<usize>, TopologyError> {
        let &start = self
            .id_to_index
            .get(&from)
            .ok_or(TopologyError::QubitNotFound(from))?;
        let &goal = self
            .id_to_index
            .get(&to)
            .ok_or(TopologyError::QubitNotFound(to))?;

        let result = astar(&self.graph, start, |n| n == goal, |_| 1u32, |_| 0u32);

        match result {
            Some((_cost, path)) => Ok(path.into_iter().map(|idx| self.graph[idx].id).collect()),
            None => Err(TopologyError::NotConnected(from, to)),
        }
    }

    pub fn all_qubits(&self) -> Vec<usize> {
        self.graph
            .node_indices()
            .map(|idx| self.graph[idx].id)
            .collect()
    }

    pub(crate) fn inner(&self) -> &UnGraph<QubitNode, ()> {
        &self.graph
    }
}

impl Default for QubitGraph {
    fn default() -> Self {
        Self::new()
    }
}
