use num_complex::Complex64;
use serde::{Deserialize, Serialize};

use crate::error::MathError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DenseMatrix {
    data: Vec<Complex64>,
    rows: usize,
    cols: usize,
}

impl DenseMatrix {
    /// Zero matrix of given dimensions.
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![Complex64::new(0.0, 0.0); rows * cols],
            rows,
            cols,
        }
    }

    /// Identity matrix of size n×n.
    pub fn identity(n: usize) -> Self {
        let mut m = Self::new(n, n);
        for i in 0..n {
            m.data[i * n + i] = Complex64::new(1.0, 0.0);
        }
        m
    }

    /// Construct from row-major data.
    pub fn from_row_major(
        rows: usize,
        cols: usize,
        data: Vec<Complex64>,
    ) -> Result<Self, MathError> {
        if data.len() != rows * cols {
            return Err(MathError::DimensionMismatch {
                expected: rows * cols,
                got: data.len(),
            });
        }
        Ok(Self { data, rows, cols })
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> Complex64 {
        self.data[row * self.cols + col]
    }

    #[inline]
    pub fn set(&mut self, row: usize, col: usize, val: Complex64) {
        self.data[row * self.cols + col] = val;
    }

    #[inline]
    pub fn rows(&self) -> usize {
        self.rows
    }

    #[inline]
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Matrix-matrix multiply.
    pub fn multiply(&self, other: &DenseMatrix) -> Result<DenseMatrix, MathError> {
        if self.cols != other.rows {
            return Err(MathError::DimensionMismatch {
                expected: self.cols,
                got: other.rows,
            });
        }
        let mut result = DenseMatrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for k in 0..self.cols {
                let a_ik = self.data[i * self.cols + k];
                for j in 0..other.cols {
                    result.data[i * other.cols + j] += a_ik * other.data[k * other.cols + j];
                }
            }
        }
        Ok(result)
    }

    /// Matrix-vector multiply.
    pub fn multiply_vec(&self, vec: &[Complex64]) -> Result<Vec<Complex64>, MathError> {
        if self.cols != vec.len() {
            return Err(MathError::DimensionMismatch {
                expected: self.cols,
                got: vec.len(),
            });
        }
        let mut result = vec![Complex64::new(0.0, 0.0); self.rows];
        for (i, res) in result.iter_mut().enumerate() {
            let row_start = i * self.cols;
            for (j, &v) in vec.iter().enumerate() {
                *res += self.data[row_start + j] * v;
            }
        }
        Ok(result)
    }

    /// Conjugate transpose (dagger).
    pub fn conjugate_transpose(&self) -> DenseMatrix {
        let mut result = DenseMatrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j * self.rows + i] = self.data[i * self.cols + j].conj();
            }
        }
        result
    }

    /// Check if U†U ≈ I within tolerance eps.
    pub fn is_unitary(&self, eps: f64) -> bool {
        if self.rows != self.cols {
            return false;
        }
        let dag = self.conjugate_transpose();
        let product = match dag.multiply(self) {
            Ok(p) => p,
            Err(_) => return false,
        };
        let id = DenseMatrix::identity(self.rows);
        for i in 0..product.data.len() {
            if (product.data[i] - id.data[i]).norm() > eps {
                return false;
            }
        }
        true
    }

    /// Matrix trace (sum of diagonal elements).
    pub fn trace(&self) -> Complex64 {
        let n = self.rows.min(self.cols);
        let mut sum = Complex64::new(0.0, 0.0);
        for i in 0..n {
            sum += self.data[i * self.cols + i];
        }
        sum
    }

    /// Raw data accessor.
    #[inline]
    pub fn data(&self) -> &[Complex64] {
        &self.data
    }
}

// --- Standard quantum gate matrices ---

pub fn hadamard() -> DenseMatrix {
    let s = 1.0 / 2.0_f64.sqrt();
    DenseMatrix::from_row_major(
        2,
        2,
        vec![
            Complex64::new(s, 0.0),
            Complex64::new(s, 0.0),
            Complex64::new(s, 0.0),
            Complex64::new(-s, 0.0),
        ],
    )
    .unwrap()
}

pub fn pauli_x() -> DenseMatrix {
    let z = Complex64::new(0.0, 0.0);
    let o = Complex64::new(1.0, 0.0);
    DenseMatrix::from_row_major(2, 2, vec![z, o, o, z]).unwrap()
}

pub fn pauli_y() -> DenseMatrix {
    let z = Complex64::new(0.0, 0.0);
    let ni = Complex64::new(0.0, -1.0);
    let pi = Complex64::new(0.0, 1.0);
    DenseMatrix::from_row_major(2, 2, vec![z, ni, pi, z]).unwrap()
}

pub fn pauli_z() -> DenseMatrix {
    let z = Complex64::new(0.0, 0.0);
    let o = Complex64::new(1.0, 0.0);
    let mo = Complex64::new(-1.0, 0.0);
    DenseMatrix::from_row_major(2, 2, vec![o, z, z, mo]).unwrap()
}

pub fn phase_s() -> DenseMatrix {
    let z = Complex64::new(0.0, 0.0);
    let o = Complex64::new(1.0, 0.0);
    let i = Complex64::new(0.0, 1.0);
    DenseMatrix::from_row_major(2, 2, vec![o, z, z, i]).unwrap()
}

pub fn t_gate() -> DenseMatrix {
    let z = Complex64::new(0.0, 0.0);
    let o = Complex64::new(1.0, 0.0);
    let angle = std::f64::consts::FRAC_PI_4;
    let t = Complex64::new(angle.cos(), angle.sin());
    DenseMatrix::from_row_major(2, 2, vec![o, z, z, t]).unwrap()
}

pub fn cnot() -> DenseMatrix {
    let z = Complex64::new(0.0, 0.0);
    let o = Complex64::new(1.0, 0.0);
    #[rustfmt::skip]
    let data = vec![
        o, z, z, z,
        z, o, z, z,
        z, z, z, o,
        z, z, o, z,
    ];
    DenseMatrix::from_row_major(4, 4, data).unwrap()
}

pub fn cz() -> DenseMatrix {
    let z = Complex64::new(0.0, 0.0);
    let o = Complex64::new(1.0, 0.0);
    let mo = Complex64::new(-1.0, 0.0);
    #[rustfmt::skip]
    let data = vec![
        o, z, z,  z,
        z, o, z,  z,
        z, z, o,  z,
        z, z, z, mo,
    ];
    DenseMatrix::from_row_major(4, 4, data).unwrap()
}

/// Rotation around X axis: e^{-i θ/2 X}
pub fn rx(theta: f64) -> DenseMatrix {
    let c = Complex64::new((theta / 2.0).cos(), 0.0);
    let s = Complex64::new(0.0, -(theta / 2.0).sin());
    DenseMatrix::from_row_major(2, 2, vec![c, s, s, c]).unwrap()
}

/// Rotation around Y axis: e^{-i θ/2 Y}
pub fn ry(theta: f64) -> DenseMatrix {
    let c = Complex64::new((theta / 2.0).cos(), 0.0);
    let s = Complex64::new((theta / 2.0).sin(), 0.0);
    let ns = Complex64::new(-(theta / 2.0).sin(), 0.0);
    DenseMatrix::from_row_major(2, 2, vec![c, ns, s, c]).unwrap()
}

/// Rotation around Z axis: e^{-i θ/2 Z}
pub fn rz(theta: f64) -> DenseMatrix {
    let e_neg = Complex64::new(0.0, -theta / 2.0).exp();
    let e_pos = Complex64::new(0.0, theta / 2.0).exp();
    let z = Complex64::new(0.0, 0.0);
    DenseMatrix::from_row_major(2, 2, vec![e_neg, z, z, e_pos]).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let id = DenseMatrix::identity(3);
        assert_eq!(id.get(0, 0), Complex64::new(1.0, 0.0));
        assert_eq!(id.get(0, 1), Complex64::new(0.0, 0.0));
        assert_eq!(id.get(1, 1), Complex64::new(1.0, 0.0));
        assert_eq!(id.get(2, 2), Complex64::new(1.0, 0.0));
    }

    #[test]
    fn test_multiply() {
        let id = DenseMatrix::identity(2);
        let h = hadamard();
        let result = id.multiply(&h).unwrap();
        for i in 0..4 {
            assert!((result.data[i] - h.data[i]).norm() < 1e-10);
        }
    }

    #[test]
    fn test_conjugate_transpose() {
        let h = hadamard();
        let hd = h.conjugate_transpose();
        // H is Hermitian, so H† = H
        for i in 0..4 {
            assert!((hd.data[i] - h.data[i]).norm() < 1e-10);
        }
    }

    #[test]
    fn test_hadamard_unitary() {
        assert!(hadamard().is_unitary(1e-10));
    }

    #[test]
    fn test_trace() {
        let id = DenseMatrix::identity(3);
        let tr = id.trace();
        assert!((tr - Complex64::new(3.0, 0.0)).norm() < 1e-10);
    }

    #[test]
    fn test_pauli_self_inverse() {
        for gate in [pauli_x(), pauli_y(), pauli_z()] {
            let sq = gate.multiply(&gate).unwrap();
            let id = DenseMatrix::identity(2);
            for i in 0..4 {
                assert!(
                    (sq.data[i] - id.data[i]).norm() < 1e-10,
                    "Pauli gate is not self-inverse"
                );
            }
        }
    }

    #[test]
    fn test_rotations_unitary() {
        assert!(rx(0.5).is_unitary(1e-10));
        assert!(ry(1.2).is_unitary(1e-10));
        assert!(rz(2.7).is_unitary(1e-10));
    }

    #[test]
    fn test_multiply_dimension_mismatch() {
        let a = DenseMatrix::new(2, 3);
        let b = DenseMatrix::new(2, 2);
        assert!(a.multiply(&b).is_err());
    }
}
