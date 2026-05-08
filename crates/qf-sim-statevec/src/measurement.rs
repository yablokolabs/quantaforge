use qf_math::StateVector;
use rand::Rng;

use crate::error::SimError;

/// Measure a single qubit, collapsing the state vector.
/// Returns the measurement outcome (false=|0⟩, true=|1⟩).
pub fn measure_qubit(
    sv: &mut StateVector,
    qubit: usize,
    rng: &mut impl Rng,
) -> Result<bool, SimError> {
    let n = sv.num_qubits();
    if qubit >= n {
        return Err(SimError::QubitOutOfRange {
            index: qubit,
            num_qubits: n,
        });
    }

    let data = sv.data_mut();

    // Compute probability of measuring |0⟩
    let mut p0: f64 = 0.0;
    for (i, amp) in data.iter().enumerate() {
        if (i >> qubit) & 1 == 0 {
            p0 += amp.norm_sqr();
        }
    }

    let outcome = rng.gen::<f64>() >= p0; // false = |0⟩, true = |1⟩

    // Collapse: zero out amplitudes inconsistent with outcome
    let outcome_bit = if outcome { 1 } else { 0 };
    for (i, amp) in data.iter_mut().enumerate() {
        if (i >> qubit) & 1 != outcome_bit {
            *amp = qf_math::Complex64::new(0.0, 0.0);
        }
    }

    // Renormalize
    let norm_sq: f64 = data.iter().map(|a| a.norm_sqr()).sum();
    let norm = norm_sq.sqrt();
    if norm > 0.0 {
        for amp in data.iter_mut() {
            *amp /= norm;
        }
    }

    Ok(outcome)
}
