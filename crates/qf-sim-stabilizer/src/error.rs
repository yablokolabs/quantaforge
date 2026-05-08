use thiserror::Error;

#[derive(Debug, Error)]
pub enum StabilizerError {
    #[error("gate {0} is not a Clifford gate")]
    NotClifford(String),
    #[error("qubit index {0} out of range")]
    QubitOutOfRange(usize),
    #[error(transparent)]
    CircuitError(#[from] qf_circuit::error::CircuitError),
}
