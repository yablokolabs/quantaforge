use qf_math::{Complex64, DenseMatrix, StateVector};

use crate::error::SimError;

/// Apply a single-qubit gate to the state vector.
/// Qubit 0 is the least significant bit.
pub fn apply_single_qubit_gate(
    sv: &mut StateVector,
    gate: &DenseMatrix,
    target: usize,
) -> Result<(), SimError> {
    let n = sv.num_qubits();
    if target >= n {
        return Err(SimError::QubitOutOfRange {
            index: target,
            num_qubits: n,
        });
    }
    if gate.rows() != 2 || gate.cols() != 2 {
        return Err(SimError::InvalidGateSize {
            expected: 2,
            got: gate.rows(),
        });
    }

    let dim = sv.dim();
    let data = sv.data_mut();
    let g00 = gate.get(0, 0);
    let g01 = gate.get(0, 1);
    let g10 = gate.get(1, 0);
    let g11 = gate.get(1, 1);

    for i in 0..dim {
        if (i >> target) & 1 == 0 {
            let j = i | (1 << target);
            let a0 = data[i];
            let a1 = data[j];
            data[i] = g00 * a0 + g01 * a1;
            data[j] = g10 * a0 + g11 * a1;
        }
    }

    Ok(())
}

/// Apply a two-qubit gate to the state vector.
/// qubit1 and qubit2 correspond to the first and second qubit operands of the gate.
/// The 4×4 gate matrix is indexed as |q1 q2⟩ with q1 as the high bit.
pub fn apply_two_qubit_gate(
    sv: &mut StateVector,
    gate: &DenseMatrix,
    qubit1: usize,
    qubit2: usize,
) -> Result<(), SimError> {
    let n = sv.num_qubits();
    if qubit1 >= n {
        return Err(SimError::QubitOutOfRange {
            index: qubit1,
            num_qubits: n,
        });
    }
    if qubit2 >= n {
        return Err(SimError::QubitOutOfRange {
            index: qubit2,
            num_qubits: n,
        });
    }
    if gate.rows() != 4 || gate.cols() != 4 {
        return Err(SimError::InvalidGateSize {
            expected: 4,
            got: gate.rows(),
        });
    }

    let dim = sv.dim();
    let data = sv.data_mut();

    let high = qubit1.max(qubit2);
    let low = qubit1.min(qubit2);

    for i in 0..dim {
        // Only process basis states where both qubit bits are 0
        if (i >> low) & 1 != 0 || (i >> high) & 1 != 0 {
            continue;
        }

        let idx00 = i;
        let idx01 = i | (1 << low);
        let idx10 = i | (1 << high);
        let idx11 = i | (1 << high) | (1 << low);

        // Map physical indices to gate matrix indices based on qubit ordering.
        // Gate matrix is indexed as |q1 q2⟩ where q1 is the "high" bit in the matrix.
        // If qubit1 > qubit2: qubit1 maps to physical high bit → direct mapping
        // If qubit1 < qubit2: qubit1 maps to physical low bit → need to swap 01↔10
        let (i00, i01, i10, i11) = if qubit1 > qubit2 {
            (idx00, idx01, idx10, idx11)
        } else {
            // qubit1 is the low bit physically, but high bit in gate matrix
            // |q1=0,q2=0⟩ → idx00, |q1=0,q2=1⟩ → idx10, |q1=1,q2=0⟩ → idx01, |q1=1,q2=1⟩ → idx11
            (idx00, idx10, idx01, idx11)
        };

        let a = [data[i00], data[i01], data[i10], data[i11]];
        let mut b = [Complex64::new(0.0, 0.0); 4];

        for (r, b_r) in b.iter_mut().enumerate() {
            for (c, a_c) in a.iter().enumerate() {
                *b_r += gate.get(r, c) * a_c;
            }
        }

        data[i00] = b[0];
        data[i01] = b[1];
        data[i10] = b[2];
        data[i11] = b[3];
    }

    Ok(())
}
