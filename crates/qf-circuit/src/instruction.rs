use serde::{Deserialize, Serialize};

use crate::error::CircuitError;
use crate::gate::Gate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instruction {
    pub gate: Gate,
    pub qubits: Vec<usize>,
}

impl Instruction {
    pub fn new(gate: Gate, qubits: Vec<usize>) -> Result<Self, CircuitError> {
        let expected = gate.num_qubits();
        if qubits.len() != expected {
            return Err(CircuitError::WrongQubitCount {
                expected,
                got: qubits.len(),
            });
        }
        Ok(Self { gate, qubits })
    }
}
