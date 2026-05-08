use thiserror::Error;

#[derive(Debug, Error)]
pub enum MathError {
    #[error("dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },
    #[error("matrix is not square: {rows}x{cols}")]
    NotSquare { rows: usize, cols: usize },
    #[error("index out of bounds: {index} >= {size}")]
    IndexOutOfBounds { index: usize, size: usize },
    #[error("invalid qubit count: {0}")]
    InvalidQubitCount(usize),
}
