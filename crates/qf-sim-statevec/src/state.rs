use qf_math::StateVector;

#[derive(Debug, Clone)]
pub struct QuantumState {
    pub state_vector: StateVector,
    pub classical_bits: Vec<Option<bool>>,
}

impl QuantumState {
    pub fn new(num_qubits: usize) -> Self {
        Self {
            state_vector: StateVector::new(num_qubits),
            classical_bits: vec![None; num_qubits],
        }
    }

    pub fn num_qubits(&self) -> usize {
        self.state_vector.num_qubits()
    }

    pub fn probabilities(&self) -> Vec<f64> {
        self.state_vector.probabilities()
    }

    pub fn get_measurement(&self, qubit: usize) -> Option<bool> {
        self.classical_bits.get(qubit).copied().flatten()
    }
}
