use crate::code::SurfaceCode;
use crate::error::QecError;
use crate::syndrome::Syndrome;

/// Result of decoding.
#[derive(Debug, Clone)]
pub struct DecoderResult {
    pub x_correction: Vec<bool>,
    pub z_correction: Vec<bool>,
    pub success: bool,
}

/// Decoder trait.
pub trait Decoder: Send + Sync {
    fn decode(&self, code: &SurfaceCode, syndrome: &Syndrome) -> Result<DecoderResult, QecError>;

    fn name(&self) -> &str;
}

/// Greedy peeling decoder.
///
/// For each syndrome type the decoder repeatedly picks the data qubit
/// that clears the most flagged stabilizers, toggles the correction there,
/// and updates the residual syndrome.  This is *not* a production MWPM
/// decoder – it is a simple heuristic suitable for research prototyping.
pub struct GreedyDecoder;

impl GreedyDecoder {
    fn correct_with_syndrome(
        syndrome_bits: &[bool],
        stabilizer_table: &[Vec<usize>],
        num_data_qubits: usize,
    ) -> Vec<bool> {
        let mut correction = vec![false; num_data_qubits];
        let mut remaining: Vec<bool> = syndrome_bits.to_vec();

        loop {
            let flagged: Vec<usize> = remaining
                .iter()
                .enumerate()
                .filter(|(_, &f)| f)
                .map(|(i, _)| i)
                .collect();

            if flagged.is_empty() {
                break;
            }

            // Among data qubits in the support of any flagged stabilizer,
            // pick the one with the best *net* improvement:
            //   net = flags_cleared − flags_created = 2·flagged_count − total_count
            // Break ties by preferring qubits in fewer total stabilizers.
            let mut best_qubit: Option<usize> = None;
            let mut best_net: isize = isize::MIN;
            let mut best_total: usize = usize::MAX;

            for &si in &flagged {
                for &q in &stabilizer_table[si] {
                    let flagged_count = stabilizer_table
                        .iter()
                        .enumerate()
                        .filter(|(idx, stab)| remaining[*idx] && stab.contains(&q))
                        .count();
                    let total_count = stabilizer_table
                        .iter()
                        .filter(|stab| stab.contains(&q))
                        .count();
                    let net = 2 * flagged_count as isize - total_count as isize;

                    if net > best_net || (net == best_net && total_count < best_total) {
                        best_net = net;
                        best_total = total_count;
                        best_qubit = Some(q);
                    }
                }
            }

            match best_qubit {
                Some(q) => {
                    correction[q] ^= true;
                    for (idx, stab) in stabilizer_table.iter().enumerate() {
                        if stab.contains(&q) {
                            remaining[idx] ^= true;
                        }
                    }
                }
                None => break,
            }
        }

        correction
    }
}

impl Decoder for GreedyDecoder {
    fn decode(&self, code: &SurfaceCode, syndrome: &Syndrome) -> Result<DecoderResult, QecError> {
        let n = code.num_data_qubits();

        // Z syndrome detects X errors → x_correction
        let x_correction =
            Self::correct_with_syndrome(&syndrome.z_syndrome, code.z_stabilizer_table(), n);

        // X syndrome detects Z errors → z_correction
        let z_correction =
            Self::correct_with_syndrome(&syndrome.x_syndrome, code.x_stabilizer_table(), n);

        Ok(DecoderResult {
            x_correction,
            z_correction,
            success: true,
        })
    }

    fn name(&self) -> &str {
        "greedy"
    }
}
