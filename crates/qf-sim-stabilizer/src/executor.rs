use qf_circuit::{Circuit, Gate};
use rand::Rng;

use crate::error::StabilizerError;
use crate::tableau::Tableau;

#[derive(Debug, Clone)]
pub struct StabilizerResult {
    pub measurements: Vec<Option<bool>>,
    pub num_qubits: usize,
    pub tableau: Tableau,
}

pub struct StabilizerSimulator;

impl StabilizerSimulator {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, circuit: &Circuit) -> Result<StabilizerResult, StabilizerError> {
        let mut rng = rand::thread_rng();
        self.run_with_rng(circuit, &mut rng)
    }

    pub fn run_with_rng(
        &self,
        circuit: &Circuit,
        rng: &mut impl Rng,
    ) -> Result<StabilizerResult, StabilizerError> {
        let n = circuit.num_qubits();
        let mut tableau = Tableau::new(n);
        let mut measurements: Vec<Option<bool>> = vec![None; n];

        for instr in circuit.instructions() {
            match &instr.gate {
                Gate::H => tableau.hadamard(instr.qubits[0]),
                Gate::X => tableau.pauli_x(instr.qubits[0]),
                Gate::Y => tableau.pauli_y(instr.qubits[0]),
                Gate::Z => tableau.pauli_z(instr.qubits[0]),
                Gate::S => tableau.phase_gate(instr.qubits[0]),
                Gate::CNOT => tableau.cnot(instr.qubits[0], instr.qubits[1]),
                Gate::CZ => tableau.cz(instr.qubits[0], instr.qubits[1]),
                Gate::Measure => {
                    let q = instr.qubits[0];
                    let result = tableau.measure(q, rng);
                    measurements[q] = Some(result);
                }
                Gate::T => {
                    return Err(StabilizerError::NotClifford("T".to_string()));
                }
                Gate::Rx(_) => {
                    return Err(StabilizerError::NotClifford("Rx".to_string()));
                }
                Gate::Ry(_) => {
                    return Err(StabilizerError::NotClifford("Ry".to_string()));
                }
                Gate::Rz(_) => {
                    return Err(StabilizerError::NotClifford("Rz".to_string()));
                }
            }
        }

        Ok(StabilizerResult {
            measurements,
            num_qubits: n,
            tableau,
        })
    }
}

impl Default for StabilizerSimulator {
    fn default() -> Self {
        Self::new()
    }
}
