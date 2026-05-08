use crate::circuit::Circuit;
use crate::error::CircuitError;
use crate::gate::Gate;
use crate::instruction::Instruction;

pub struct CircuitBuilder {
    circuit: Circuit,
}

impl CircuitBuilder {
    pub fn new(num_qubits: usize) -> Result<Self, CircuitError> {
        Ok(Self {
            circuit: Circuit::new(num_qubits)?,
        })
    }

    fn add(mut self, gate: Gate, qubits: Vec<usize>) -> Result<Self, CircuitError> {
        let instr = Instruction::new(gate, qubits)?;
        self.circuit.add_instruction(instr)?;
        Ok(self)
    }

    pub fn h(self, qubit: usize) -> Result<Self, CircuitError> {
        self.add(Gate::H, vec![qubit])
    }

    pub fn x(self, qubit: usize) -> Result<Self, CircuitError> {
        self.add(Gate::X, vec![qubit])
    }

    pub fn y(self, qubit: usize) -> Result<Self, CircuitError> {
        self.add(Gate::Y, vec![qubit])
    }

    pub fn z(self, qubit: usize) -> Result<Self, CircuitError> {
        self.add(Gate::Z, vec![qubit])
    }

    pub fn s(self, qubit: usize) -> Result<Self, CircuitError> {
        self.add(Gate::S, vec![qubit])
    }

    pub fn t(self, qubit: usize) -> Result<Self, CircuitError> {
        self.add(Gate::T, vec![qubit])
    }

    pub fn rx(self, qubit: usize, theta: f64) -> Result<Self, CircuitError> {
        self.add(Gate::Rx(theta), vec![qubit])
    }

    pub fn ry(self, qubit: usize, theta: f64) -> Result<Self, CircuitError> {
        self.add(Gate::Ry(theta), vec![qubit])
    }

    pub fn rz(self, qubit: usize, theta: f64) -> Result<Self, CircuitError> {
        self.add(Gate::Rz(theta), vec![qubit])
    }

    pub fn cnot(self, control: usize, target: usize) -> Result<Self, CircuitError> {
        self.add(Gate::CNOT, vec![control, target])
    }

    pub fn cz(self, q1: usize, q2: usize) -> Result<Self, CircuitError> {
        self.add(Gate::CZ, vec![q1, q2])
    }

    pub fn measure(self, qubit: usize) -> Result<Self, CircuitError> {
        self.add(Gate::Measure, vec![qubit])
    }

    pub fn measure_all(mut self) -> Result<Self, CircuitError> {
        let n = self.circuit.num_qubits();
        for q in 0..n {
            let instr = Instruction::new(Gate::Measure, vec![q])?;
            self.circuit.add_instruction(instr)?;
        }
        Ok(self)
    }

    pub fn build(self) -> Circuit {
        self.circuit
    }
}
