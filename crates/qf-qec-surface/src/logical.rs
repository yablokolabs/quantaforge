use serde::{Deserialize, Serialize};

use crate::code::SurfaceCode;
use crate::decoder::Decoder;
use crate::syndrome::{extract_syndrome, random_errors};

/// Track whether a logical error has occurred after error + correction.
///
/// Logical X = X on an entire row  (left↔right chain, weight d).
/// Logical Z = Z on an entire column (top↔bottom chain, weight d).
///
/// Because every stabilizer has even overlap with every row / column,
/// the parity of the residual on any single row (or column) tells us
/// whether a logical error occurred. We check row 0 / column 0.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalErrorTracker {
    distance: usize,
}

impl LogicalErrorTracker {
    pub fn new(distance: usize) -> Self {
        Self { distance }
    }

    /// True when the combined X-error + X-correction is a non-trivial
    /// logical X operator (odd parity on column 0).
    pub fn has_logical_x_error(&self, x_errors: &[bool], x_correction: &[bool]) -> bool {
        let d = self.distance;
        let mut parity = false;
        for r in 0..d {
            parity ^= x_errors[r * d] ^ x_correction[r * d];
        }
        parity
    }

    /// True when the combined Z-error + Z-correction is a non-trivial
    /// logical Z operator (odd parity on row 0).
    pub fn has_logical_z_error(&self, z_errors: &[bool], z_correction: &[bool]) -> bool {
        let d = self.distance;
        let mut parity = false;
        for c in 0..d {
            parity ^= z_errors[c] ^ z_correction[c];
        }
        parity
    }

    /// Run a Monte-Carlo experiment: inject random errors, decode, and
    /// count logical failures.
    pub fn run_experiment(
        &self,
        code: &SurfaceCode,
        decoder: &dyn Decoder,
        error_rate: f64,
        num_trials: usize,
        rng: &mut impl rand::Rng,
    ) -> ExperimentResult {
        let mut logical_error_count: usize = 0;

        for _ in 0..num_trials {
            let (x_errors, z_errors) = random_errors(code.num_data_qubits(), error_rate, rng);
            let syndrome = extract_syndrome(code, &x_errors, &z_errors);

            match decoder.decode(code, &syndrome) {
                Ok(result) => {
                    if self.has_logical_x_error(&x_errors, &result.x_correction)
                        || self.has_logical_z_error(&z_errors, &result.z_correction)
                    {
                        logical_error_count += 1;
                    }
                }
                Err(_) => {
                    logical_error_count += 1;
                }
            }
        }

        ExperimentResult {
            num_trials,
            logical_error_count,
            logical_error_rate: logical_error_count as f64 / num_trials as f64,
            physical_error_rate: error_rate,
            code_distance: self.distance,
        }
    }
}

/// Summary of a decoding experiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResult {
    pub num_trials: usize,
    pub logical_error_count: usize,
    pub logical_error_rate: f64,
    pub physical_error_rate: f64,
    pub code_distance: usize,
}
