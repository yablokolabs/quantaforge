use serde::{Deserialize, Serialize};

use crate::error::CircuitError;
use crate::gate::Gate;
use crate::instruction::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circuit {
    num_qubits: usize,
    instructions: Vec<Instruction>,
    name: Option<String>,
}

impl Circuit {
    pub fn new(num_qubits: usize) -> Result<Self, CircuitError> {
        if num_qubits == 0 {
            return Err(CircuitError::ZeroQubits);
        }
        Ok(Self {
            num_qubits,
            instructions: Vec::new(),
            name: None,
        })
    }

    pub fn with_name(num_qubits: usize, name: &str) -> Result<Self, CircuitError> {
        let mut c = Self::new(num_qubits)?;
        c.name = Some(name.to_string());
        Ok(c)
    }

    pub fn add_instruction(&mut self, instr: Instruction) -> Result<(), CircuitError> {
        for &q in &instr.qubits {
            if q >= self.num_qubits {
                return Err(CircuitError::QubitOutOfRange {
                    index: q,
                    num_qubits: self.num_qubits,
                });
            }
        }
        self.instructions.push(instr);
        Ok(())
    }

    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    /// Circuit depth: the maximum number of time-steps (layers) any qubit participates in.
    pub fn depth(&self) -> usize {
        let mut qubit_depth = vec![0usize; self.num_qubits];
        for instr in &self.instructions {
            let layer = instr
                .qubits
                .iter()
                .map(|&q| qubit_depth[q])
                .max()
                .unwrap_or(0)
                + 1;
            for &q in &instr.qubits {
                qubit_depth[q] = layer;
            }
        }
        qubit_depth.into_iter().max().unwrap_or(0)
    }

    pub fn gate_count(&self) -> usize {
        self.instructions.len()
    }

    pub fn has_measurements(&self) -> bool {
        self.instructions.iter().any(|i| i.gate.is_measurement())
    }

    pub fn is_clifford_only(&self) -> bool {
        self.instructions.iter().all(|i| {
            matches!(
                i.gate,
                Gate::H
                    | Gate::S
                    | Gate::X
                    | Gate::Y
                    | Gate::Z
                    | Gate::CNOT
                    | Gate::CZ
                    | Gate::Measure
            )
        })
    }
}
