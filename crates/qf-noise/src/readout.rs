use rand::Rng;

use crate::error::NoiseError;

/// Classical readout noise: probabilistic bit-flip of measurement outcomes.
pub struct ReadoutNoise {
    /// Probability of reading 1 when the true state is 0.
    p0_to_1: f64,
    /// Probability of reading 0 when the true state is 1.
    p1_to_0: f64,
}

impl ReadoutNoise {
    pub fn new(p0_to_1: f64, p1_to_0: f64) -> Result<Self, NoiseError> {
        if !(0.0..=1.0).contains(&p0_to_1) {
            return Err(NoiseError::InvalidParameter {
                param: "p0_to_1",
                value: p0_to_1,
            });
        }
        if !(0.0..=1.0).contains(&p1_to_0) {
            return Err(NoiseError::InvalidParameter {
                param: "p1_to_0",
                value: p1_to_0,
            });
        }
        Ok(Self { p0_to_1, p1_to_0 })
    }

    /// Symmetric readout noise with the same error rate in both directions.
    pub fn symmetric(p: f64) -> Result<Self, NoiseError> {
        Self::new(p, p)
    }

    /// Apply readout noise to a single measurement result.
    pub fn apply_to_measurement(&self, measured_bit: bool, rng: &mut impl Rng) -> bool {
        let flip_prob = if measured_bit {
            self.p1_to_0
        } else {
            self.p0_to_1
        };
        if rng.gen::<f64>() < flip_prob {
            !measured_bit
        } else {
            measured_bit
        }
    }
}
