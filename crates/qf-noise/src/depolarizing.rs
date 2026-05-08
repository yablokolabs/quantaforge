use qf_math::matrix::{pauli_x, pauli_y, pauli_z};
use qf_math::{Complex64, DenseMatrix};

use crate::channel::NoiseChannel;
use crate::error::NoiseError;

/// Depolarizing noise channel.
///
/// Maps ρ → (1-p)ρ + p·I/2, replacing the qubit state with the maximally
/// mixed state with probability p.
///
/// Kraus operators:
///   E0 = √(1 - 3p/4) I,  E1 = √(p/4) X,  E2 = √(p/4) Y,  E3 = √(p/4) Z
pub struct DepolarizingChannel {
    probability: f64,
}

impl DepolarizingChannel {
    pub fn new(probability: f64) -> Result<Self, NoiseError> {
        if !(0.0..=1.0).contains(&probability) {
            return Err(NoiseError::InvalidParameter {
                param: "probability",
                value: probability,
            });
        }
        Ok(Self { probability })
    }
}

impl NoiseChannel for DepolarizingChannel {
    fn name(&self) -> &str {
        "depolarizing"
    }

    fn kraus_operators(&self, _qubit: usize) -> Vec<DenseMatrix> {
        let p = self.probability;

        fn scale(mat: &DenseMatrix, s: f64) -> DenseMatrix {
            let c = Complex64::new(s, 0.0);
            let dim = mat.rows();
            let mut out = DenseMatrix::new(dim, dim);
            for i in 0..dim {
                for j in 0..dim {
                    out.set(i, j, mat.get(i, j) * c);
                }
            }
            out
        }

        let e0 = scale(&DenseMatrix::identity(2), (1.0 - 3.0 * p / 4.0).sqrt());
        let e1 = scale(&pauli_x(), (p / 4.0).sqrt());
        let e2 = scale(&pauli_y(), (p / 4.0).sqrt());
        let e3 = scale(&pauli_z(), (p / 4.0).sqrt());
        vec![e0, e1, e2, e3]
    }
}
