use num_complex::Complex64;
use serde::{Deserialize, Serialize};

use crate::error::MathError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateVector {
    data: Vec<Complex64>,
    num_qubits: usize,
}

impl StateVector {
    /// Initialize to |0...0⟩ state.
    pub fn new(num_qubits: usize) -> Self {
        let dim = 1usize << num_qubits;
        let mut data = vec![Complex64::new(0.0, 0.0); dim];
        data[0] = Complex64::new(1.0, 0.0);
        Self { data, num_qubits }
    }

    /// Create from raw amplitudes. Length must be a power of 2.
    pub fn from_data(data: Vec<Complex64>) -> Result<Self, MathError> {
        let len = data.len();
        if len == 0 || (len & (len - 1)) != 0 {
            return Err(MathError::InvalidQubitCount(len));
        }
        let num_qubits = len.trailing_zeros() as usize;
        Ok(Self { data, num_qubits })
    }

    #[inline]
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    #[inline]
    pub fn dim(&self) -> usize {
        1usize << self.num_qubits
    }

    #[inline]
    pub fn amplitude(&self, index: usize) -> Result<Complex64, MathError> {
        if index >= self.data.len() {
            return Err(MathError::IndexOutOfBounds {
                index,
                size: self.data.len(),
            });
        }
        Ok(self.data[index])
    }

    #[inline]
    pub fn set_amplitude(&mut self, index: usize, val: Complex64) -> Result<(), MathError> {
        if index >= self.data.len() {
            return Err(MathError::IndexOutOfBounds {
                index,
                size: self.data.len(),
            });
        }
        self.data[index] = val;
        Ok(())
    }

    /// Returns |amplitude|^2 for each basis state.
    pub fn probabilities(&self) -> Vec<f64> {
        self.data.iter().map(|a| a.norm_sqr()).collect()
    }

    /// Euclidean norm: sqrt(sum |a_i|^2).
    pub fn norm(&self) -> f64 {
        self.data.iter().map(|a| a.norm_sqr()).sum::<f64>().sqrt()
    }

    /// Normalize in-place.
    pub fn normalize(&mut self) {
        let n = self.norm();
        if n > 0.0 {
            for a in &mut self.data {
                *a /= n;
            }
        }
    }

    /// Inner product ⟨self|other⟩ = Σ conj(self_i) * other_i.
    pub fn inner_product(&self, other: &StateVector) -> Result<Complex64, MathError> {
        if self.data.len() != other.data.len() {
            return Err(MathError::DimensionMismatch {
                expected: self.data.len(),
                got: other.data.len(),
            });
        }
        let result = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a.conj() * b)
            .sum();
        Ok(result)
    }

    #[inline]
    pub fn data(&self) -> &[Complex64] {
        &self.data
    }

    #[inline]
    pub fn data_mut(&mut self) -> &mut [Complex64] {
        &mut self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_single_qubit() {
        let sv = StateVector::new(1);
        assert_eq!(sv.num_qubits(), 1);
        assert_eq!(sv.dim(), 2);
        assert_eq!(sv.amplitude(0).unwrap(), Complex64::new(1.0, 0.0));
        assert_eq!(sv.amplitude(1).unwrap(), Complex64::new(0.0, 0.0));
    }

    #[test]
    fn test_new_two_qubits() {
        let sv = StateVector::new(2);
        assert_eq!(sv.dim(), 4);
        assert_eq!(sv.amplitude(0).unwrap(), Complex64::new(1.0, 0.0));
        for i in 1..4 {
            assert_eq!(sv.amplitude(i).unwrap(), Complex64::new(0.0, 0.0));
        }
    }

    #[test]
    fn test_from_data_valid() {
        let data = vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)];
        let sv = StateVector::from_data(data).unwrap();
        assert_eq!(sv.num_qubits(), 1);
    }

    #[test]
    fn test_from_data_invalid() {
        let data = vec![Complex64::new(1.0, 0.0); 3];
        assert!(StateVector::from_data(data).is_err());
    }

    #[test]
    fn test_norm_and_normalize() {
        let data = vec![Complex64::new(3.0, 0.0), Complex64::new(4.0, 0.0)];
        let mut sv = StateVector::from_data(data).unwrap();
        assert!((sv.norm() - 5.0).abs() < 1e-10);
        sv.normalize();
        assert!((sv.norm() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_inner_product() {
        let sv0 = StateVector::new(1);
        let mut sv1 = StateVector::new(1);
        sv1.set_amplitude(0, Complex64::new(0.0, 0.0)).unwrap();
        sv1.set_amplitude(1, Complex64::new(1.0, 0.0)).unwrap();

        let ip00 = sv0.inner_product(&sv0).unwrap();
        assert!((ip00 - Complex64::new(1.0, 0.0)).norm() < 1e-10);

        let ip01 = sv0.inner_product(&sv1).unwrap();
        assert!((ip01 - Complex64::new(0.0, 0.0)).norm() < 1e-10);
    }

    #[test]
    fn test_probabilities() {
        let data = vec![
            Complex64::new(1.0 / 2.0_f64.sqrt(), 0.0),
            Complex64::new(1.0 / 2.0_f64.sqrt(), 0.0),
        ];
        let sv = StateVector::from_data(data).unwrap();
        let probs = sv.probabilities();
        assert!((probs[0] - 0.5).abs() < 1e-10);
        assert!((probs[1] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_index_out_of_bounds() {
        let sv = StateVector::new(1);
        assert!(sv.amplitude(2).is_err());
    }
}
