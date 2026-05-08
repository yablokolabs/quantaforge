use thiserror::Error;

#[derive(Debug, Error)]
pub enum QecError {
    #[error("code distance {0} must be >= 3 and odd")]
    InvalidDistance(usize),
    #[error("decoder failed: {0}")]
    DecoderFailure(String),
    #[error(transparent)]
    TopologyError(#[from] qf_topology::error::TopologyError),
}
