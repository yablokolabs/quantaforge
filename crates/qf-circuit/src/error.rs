use thiserror::Error;

#[derive(Debug, Error)]
pub enum CircuitError {
    #[error("qubit index {index} out of range for {num_qubits}-qubit circuit")]
    QubitOutOfRange { index: usize, num_qubits: usize },
    #[error("gate requires {expected} qubits, got {got}")]
    WrongQubitCount { expected: usize, got: usize },
    #[error("circuit must have at least 1 qubit")]
    ZeroQubits,
    #[error("parse error: {0}")]
    ParseError(String),
    #[error("serialization error: {0}")]
    SerializationError(String),
}
