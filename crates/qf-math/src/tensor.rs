use num_complex::Complex64;

use crate::matrix::DenseMatrix;

/// Kronecker (tensor) product of two matrices: A ⊗ B.
pub fn tensor_product(a: &DenseMatrix, b: &DenseMatrix) -> DenseMatrix {
    let (ar, ac) = (a.rows(), a.cols());
    let (br, bc) = (b.rows(), b.cols());
    let mut result = DenseMatrix::new(ar * br, ac * bc);
    for i in 0..ar {
        for j in 0..ac {
            let a_ij = a.get(i, j);
            for k in 0..br {
                for l in 0..bc {
                    result.set(i * br + k, j * bc + l, a_ij * b.get(k, l));
                }
            }
        }
    }
    result
}

/// Kronecker (tensor) product of two vectors.
pub fn tensor_product_vec(a: &[Complex64], b: &[Complex64]) -> Vec<Complex64> {
    let mut result = Vec::with_capacity(a.len() * b.len());
    for &ai in a {
        for &bj in b {
            result.push(ai * bj);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_identity() {
        let i2 = DenseMatrix::identity(2);
        let result = tensor_product(&i2, &i2);
        assert_eq!(result.rows(), 4);
        assert_eq!(result.cols(), 4);
        let i4 = DenseMatrix::identity(4);
        for r in 0..4 {
            for c in 0..4 {
                assert!(
                    (result.get(r, c) - i4.get(r, c)).norm() < 1e-10,
                    "Mismatch at ({r}, {c})"
                );
            }
        }
    }

    #[test]
    fn test_tensor_product_vec() {
        let a = vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)];
        let b = vec![Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)];
        let result = tensor_product_vec(&a, &b);
        assert_eq!(result.len(), 4);
        // |0⟩ ⊗ |1⟩ = |01⟩ = [0, 1, 0, 0]
        assert!((result[0] - Complex64::new(0.0, 0.0)).norm() < 1e-10);
        assert!((result[1] - Complex64::new(1.0, 0.0)).norm() < 1e-10);
        assert!((result[2] - Complex64::new(0.0, 0.0)).norm() < 1e-10);
        assert!((result[3] - Complex64::new(0.0, 0.0)).norm() < 1e-10);
    }

    #[test]
    fn test_tensor_dimensions() {
        let a = DenseMatrix::new(2, 3);
        let b = DenseMatrix::new(4, 5);
        let result = tensor_product(&a, &b);
        assert_eq!(result.rows(), 8);
        assert_eq!(result.cols(), 15);
    }
}
