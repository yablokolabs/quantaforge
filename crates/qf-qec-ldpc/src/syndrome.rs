use rand::Rng;

use crate::error::LdpcError;
use crate::parity_check::ParityCheckMatrix;

pub fn compute_syndrome(h: &ParityCheckMatrix, error: &[u8]) -> Result<Vec<u8>, LdpcError> {
    h.syndrome(error)
}

pub fn random_error(n: usize, error_rate: f64, rng: &mut impl Rng) -> Vec<u8> {
    (0..n)
        .map(|_| if rng.gen::<f64>() < error_rate { 1 } else { 0 })
        .collect()
}
