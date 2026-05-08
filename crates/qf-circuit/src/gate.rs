use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Gate {
    H,
    X,
    Y,
    Z,
    S,
    T,
    Rx(f64),
    Ry(f64),
    Rz(f64),
    CNOT,
    CZ,
    Measure,
}

impl Gate {
    pub fn num_qubits(&self) -> usize {
        match self {
            Gate::CNOT | Gate::CZ => 2,
            _ => 1,
        }
    }

    pub fn is_parameterized(&self) -> bool {
        matches!(self, Gate::Rx(_) | Gate::Ry(_) | Gate::Rz(_))
    }

    pub fn is_measurement(&self) -> bool {
        matches!(self, Gate::Measure)
    }

    pub fn matrix(&self) -> Option<qf_math::DenseMatrix> {
        match self {
            Gate::H => Some(qf_math::matrix::hadamard()),
            Gate::X => Some(qf_math::matrix::pauli_x()),
            Gate::Y => Some(qf_math::matrix::pauli_y()),
            Gate::Z => Some(qf_math::matrix::pauli_z()),
            Gate::S => Some(qf_math::matrix::phase_s()),
            Gate::T => Some(qf_math::matrix::t_gate()),
            Gate::Rx(theta) => Some(qf_math::matrix::rx(*theta)),
            Gate::Ry(theta) => Some(qf_math::matrix::ry(*theta)),
            Gate::Rz(theta) => Some(qf_math::matrix::rz(*theta)),
            Gate::CNOT => Some(qf_math::matrix::cnot()),
            Gate::CZ => Some(qf_math::matrix::cz()),
            Gate::Measure => None,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Gate::H => "H",
            Gate::X => "X",
            Gate::Y => "Y",
            Gate::Z => "Z",
            Gate::S => "S",
            Gate::T => "T",
            Gate::Rx(_) => "RX",
            Gate::Ry(_) => "RY",
            Gate::Rz(_) => "RZ",
            Gate::CNOT => "CNOT",
            Gate::CZ => "CZ",
            Gate::Measure => "MEASURE",
        }
    }
}
