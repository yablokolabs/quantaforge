use std::collections::HashMap;

use qf_circuit::{Circuit, Gate};
use rand::Rng;

use crate::error::{SimError, MAX_QUBITS};
use crate::gates::{apply_single_qubit_gate, apply_two_qubit_gate};
use crate::measurement::measure_qubit;
use crate::state::QuantumState;

#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub state: QuantumState,
    pub num_qubits: usize,
}

#[derive(Debug, Clone)]
pub struct ShotResult {
    pub counts: HashMap<String, usize>,
    pub num_shots: usize,
    pub num_qubits: usize,
}

pub struct StateVectorSimulator;

impl StateVectorSimulator {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, circuit: &Circuit) -> Result<SimulationResult, SimError> {
        let mut rng = rand::thread_rng();
        self.run_with_rng(circuit, &mut rng)
    }

    pub fn run_with_rng(
        &self,
        circuit: &Circuit,
        rng: &mut impl Rng,
    ) -> Result<SimulationResult, SimError> {
        let n = circuit.num_qubits();
        if n > MAX_QUBITS {
            return Err(SimError::TooManyQubits(n, MAX_QUBITS));
        }

        let mut qs = QuantumState::new(n);

        for instr in circuit.instructions() {
            match &instr.gate {
                Gate::Measure => {
                    let qubit = instr.qubits[0];
                    let result = measure_qubit(&mut qs.state_vector, qubit, rng)?;
                    qs.classical_bits[qubit] = Some(result);
                }
                gate => {
                    let mat = gate.matrix().ok_or(SimError::DimensionMismatch)?;
                    if gate.num_qubits() == 1 {
                        apply_single_qubit_gate(&mut qs.state_vector, &mat, instr.qubits[0])?;
                    } else {
                        apply_two_qubit_gate(
                            &mut qs.state_vector,
                            &mat,
                            instr.qubits[0],
                            instr.qubits[1],
                        )?;
                    }
                }
            }
        }

        Ok(SimulationResult {
            state: qs,
            num_qubits: n,
        })
    }

    pub fn run_shots(&self, circuit: &Circuit, shots: usize) -> Result<ShotResult, SimError> {
        let n = circuit.num_qubits();
        if n > MAX_QUBITS {
            return Err(SimError::TooManyQubits(n, MAX_QUBITS));
        }

        let mut counts: HashMap<String, usize> = HashMap::new();
        let mut rng = rand::thread_rng();

        for _ in 0..shots {
            let result = self.run_with_rng(circuit, &mut rng)?;
            let bitstring: String = (0..n)
                .rev()
                .map(|q| {
                    match result.state.get_measurement(q) {
                        Some(true) => '1',
                        Some(false) => '0',
                        None => {
                            // Qubit wasn't measured; sample from probabilities
                            let probs = result.state.probabilities();
                            // Determine the marginal probability of this qubit being |1⟩
                            let mut p1: f64 = 0.0;
                            for (i, &p) in probs.iter().enumerate() {
                                if (i >> q) & 1 == 1 {
                                    p1 += p;
                                }
                            }
                            if rng.gen::<f64>() < p1 {
                                '1'
                            } else {
                                '0'
                            }
                        }
                    }
                })
                .collect();
            *counts.entry(bitstring).or_insert(0) += 1;
        }

        Ok(ShotResult {
            counts,
            num_shots: shots,
            num_qubits: n,
        })
    }
}

impl Default for StateVectorSimulator {
    fn default() -> Self {
        Self::new()
    }
}
