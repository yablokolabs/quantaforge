use thiserror::Error;

#[derive(Debug, Error)]
pub enum LogicalError {
    #[error("logical operation not supported for this code: {0}")]
    UnsupportedOperation(String),
    #[error("invalid logical qubit index: {0}")]
    InvalidIndex(usize),
    #[error(transparent)]
    QecError(#[from] qf_qec_surface::error::QecError),
    #[error(transparent)]
    LdpcError(#[from] qf_qec_ldpc::error::LdpcError),
}
