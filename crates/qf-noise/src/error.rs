use thiserror::Error;

#[derive(Debug, Error)]
pub enum NoiseError {
    #[error("noise parameter {param} = {value} out of valid range [0, 1]")]
    InvalidParameter { param: &'static str, value: f64 },
    #[error(transparent)]
    MathError(#[from] qf_math::error::MathError),
}
