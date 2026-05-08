use thiserror::Error;

#[derive(Debug, Error)]
pub enum TopologyError {
    #[error("qubit {0} not found in topology")]
    QubitNotFound(usize),
    #[error("qubits {0} and {1} are not connected")]
    NotConnected(usize, usize),
    #[error("invalid lattice dimensions: {rows}x{cols}")]
    InvalidDimensions { rows: usize, cols: usize },
}
