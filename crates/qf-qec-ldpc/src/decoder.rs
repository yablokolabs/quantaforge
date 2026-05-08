use crate::error::LdpcError;
use crate::parity_check::ParityCheckMatrix;

pub trait Decoder: Send + Sync {
    fn decode(
        &self,
        parity_check: &ParityCheckMatrix,
        syndrome: &[u8],
    ) -> Result<Vec<u8>, LdpcError>;

    fn name(&self) -> &str;
}

/// Simple bit-flip decoder (Gallager's Algorithm A)
pub struct BitFlipDecoder {
    max_iterations: usize,
}

impl BitFlipDecoder {
    pub fn new(max_iterations: usize) -> Self {
        Self { max_iterations }
    }
}

impl Decoder for BitFlipDecoder {
    fn decode(
        &self,
        parity_check: &ParityCheckMatrix,
        syndrome: &[u8],
    ) -> Result<Vec<u8>, LdpcError> {
        let rows = parity_check.rows();
        let cols = parity_check.cols();
        let mut estimate = vec![0u8; cols];

        for _iter in 0..self.max_iterations {
            // Compute current syndrome
            let current_syn = parity_check.syndrome(&estimate)?;

            // Check if syndrome matches target
            if current_syn == syndrome {
                return Ok(estimate);
            }

            // Count unsatisfied checks per bit
            let mut unsatisfied = vec![0usize; cols];
            for r in 0..rows {
                if current_syn[r] != syndrome[r] {
                    for (c, count) in unsatisfied.iter_mut().enumerate() {
                        if parity_check.get(r, c) == 1 {
                            *count += 1;
                        }
                    }
                }
            }

            // Flip the bit with the most unsatisfied checks
            let max_unsat = *unsatisfied.iter().max().unwrap_or(&0);
            if max_unsat == 0 {
                return Ok(estimate);
            }

            // Find first bit with max unsatisfied count and flip it
            for (c, &u) in unsatisfied.iter().enumerate() {
                if u == max_unsat {
                    estimate[c] ^= 1;
                    break;
                }
            }
        }

        // Final check
        let final_syn = parity_check.syndrome(&estimate)?;
        if final_syn == syndrome {
            Ok(estimate)
        } else {
            Err(LdpcError::DidNotConverge(self.max_iterations))
        }
    }

    fn name(&self) -> &str {
        "BitFlipDecoder"
    }
}
