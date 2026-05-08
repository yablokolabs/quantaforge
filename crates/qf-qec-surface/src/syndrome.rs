use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::code::SurfaceCode;

/// Syndrome: result of measuring all stabilizers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Syndrome {
    /// One bit per X stabilizer (flags when an odd number of Z errors hit its support).
    pub x_syndrome: Vec<bool>,
    /// One bit per Z stabilizer (flags when an odd number of X errors hit its support).
    pub z_syndrome: Vec<bool>,
}

/// Compute the syndrome produced by a given Pauli error pattern.
///
/// * X errors are detected by Z stabilizers (`z_syndrome`).
/// * Z errors are detected by X stabilizers (`x_syndrome`).
pub fn extract_syndrome(code: &SurfaceCode, x_errors: &[bool], z_errors: &[bool]) -> Syndrome {
    // X stabilizers flag on Z errors
    let x_syndrome: Vec<bool> = code
        .x_stabilizer_table()
        .iter()
        .map(|support| support.iter().filter(|&&q| z_errors[q]).count() % 2 == 1)
        .collect();

    // Z stabilizers flag on X errors
    let z_syndrome: Vec<bool> = code
        .z_stabilizer_table()
        .iter()
        .map(|support| support.iter().filter(|&&q| x_errors[q]).count() % 2 == 1)
        .collect();

    Syndrome {
        x_syndrome,
        z_syndrome,
    }
}

/// Generate independent X and Z errors with the given per-qubit rate.
pub fn random_errors(
    num_qubits: usize,
    error_rate: f64,
    rng: &mut impl Rng,
) -> (Vec<bool>, Vec<bool>) {
    let rate = error_rate.clamp(0.0, 1.0);
    let x_errors: Vec<bool> = (0..num_qubits).map(|_| rng.gen_bool(rate)).collect();
    let z_errors: Vec<bool> = (0..num_qubits).map(|_| rng.gen_bool(rate)).collect();
    (x_errors, z_errors)
}
