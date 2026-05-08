use qf_circuit::Circuit;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceEstimate {
    pub num_qubits: usize,
    pub gate_count: usize,
    pub circuit_depth: usize,
    pub estimated_memory_bytes: u64,
    pub recommended_backend: Backend,
    pub estimated_runtime_class: RuntimeClass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Backend {
    StateVector,
    Stabilizer,
    TensorNetwork,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeClass {
    Instant,
    Fast,
    Moderate,
    Heavy,
    Infeasible,
}

impl ResourceEstimate {
    pub fn from_circuit(circuit: &Circuit) -> Self {
        let n = circuit.num_qubits();
        let memory = if n <= 30 { (1u64 << n) * 16 } else { u64::MAX };
        let backend = if circuit.is_clifford_only() {
            Backend::Stabilizer
        } else if n <= 30 {
            Backend::StateVector
        } else {
            Backend::TensorNetwork
        };
        let runtime = match n {
            0..=20 => RuntimeClass::Instant,
            21..=25 => RuntimeClass::Fast,
            26..=28 => RuntimeClass::Moderate,
            29..=30 => RuntimeClass::Heavy,
            _ => {
                if circuit.is_clifford_only() {
                    RuntimeClass::Fast
                } else {
                    RuntimeClass::Infeasible
                }
            }
        };
        Self {
            num_qubits: n,
            gate_count: circuit.gate_count(),
            circuit_depth: circuit.depth(),
            estimated_memory_bytes: memory,
            recommended_backend: backend,
            estimated_runtime_class: runtime,
        }
    }
}
