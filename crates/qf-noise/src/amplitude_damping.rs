use qf_math::{Complex64, DenseMatrix};

use crate::channel::NoiseChannel;
use crate::error::NoiseError;

/// Amplitude damping channel modelling energy relaxation (T1 decay).
///
/// Kraus operators:
///   E0 = [[1, 0], [0, √(1-γ)]]
///   E1 = [[0, √γ], [0, 0]]
pub struct AmplitudeDampingChannel {
    gamma: f64,
}

impl AmplitudeDampingChannel {
    pub fn new(gamma: f64) -> Result<Self, NoiseError> {
        if !(0.0..=1.0).contains(&gamma) {
            return Err(NoiseError::InvalidParameter {
                param: "gamma",
                value: gamma,
            });
        }
        Ok(Self { gamma })
    }
}

impl NoiseChannel for AmplitudeDampingChannel {
    fn name(&self) -> &str {
        "amplitude_damping"
    }

    fn kraus_operators(&self, _qubit: usize) -> Vec<DenseMatrix> {
        let z = Complex64::new(0.0, 0.0);

        let e0 = DenseMatrix::from_row_major(
            2,
            2,
            vec![
                Complex64::new(1.0, 0.0),
                z,
                z,
                Complex64::new((1.0 - self.gamma).sqrt(), 0.0),
            ],
        )
        .unwrap();

        let e1 = DenseMatrix::from_row_major(
            2,
            2,
            vec![z, Complex64::new(self.gamma.sqrt(), 0.0), z, z],
        )
        .unwrap();

        vec![e0, e1]
    }
}
