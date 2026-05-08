use thiserror::Error;

#[derive(Debug, Error)]
pub enum LdpcError {
    #[error("parity check matrix dimensions invalid: {rows}x{cols}")]
    InvalidDimensions { rows: usize, cols: usize },
    #[error("syndrome length mismatch: expected {expected}, got {got}")]
    SyndromeMismatch { expected: usize, got: usize },
    #[error("decoder did not converge after {0} iterations")]
    DidNotConverge(usize),
    #[error("inconsistent row lengths in parity check matrix")]
    InconsistentRowLengths,
    #[error("invalid GF(2) value: {0} (must be 0 or 1)")]
    InvalidValue(u8),
    #[error("CSS orthogonality violated: Hx * Hz^T != 0 (mod 2)")]
    CssOrthogonalityViolated,
    #[error("CSS dimension mismatch: Hx has {hx_cols} cols but Hz has {hz_cols} cols")]
    CssDimensionMismatch { hx_cols: usize, hz_cols: usize },
    #[error("LDPC weight constraint violated: max weight {max_weight} exceeded")]
    WeightConstraintViolated { max_weight: usize },
}
