use thiserror::Error;

pub const MAX_QUBITS: usize = 30;

#[derive(Debug, Error)]
pub enum SimError {
    #[error("qubit count {0} exceeds maximum supported ({1})")]
    TooManyQubits(usize, usize),
    #[error("qubit index {index} out of range for {num_qubits}-qubit system")]
    QubitOutOfRange { index: usize, num_qubits: usize },
    #[error("gate matrix has wrong dimensions: expected {expected}x{expected}, got {got}x{got}")]
    InvalidGateSize { expected: usize, got: usize },
    #[error("state vector dimension mismatch")]
    DimensionMismatch,
    #[error(transparent)]
    CircuitError(#[from] qf_circuit::error::CircuitError),
    #[error(transparent)]
    MathError(#[from] qf_math::error::MathError),
}
