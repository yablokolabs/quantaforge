use num_complex::Complex64;

use crate::error::MathError;
use crate::matrix::DenseMatrix;

#[derive(Debug, Clone)]
pub struct SparseMatrix {
    rows: usize,
    cols: usize,
    row_ptrs: Vec<usize>,
    col_indices: Vec<usize>,
    values: Vec<Complex64>,
}

impl SparseMatrix {
    /// Empty sparse matrix.
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            row_ptrs: vec![0; rows + 1],
            col_indices: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Convert a dense matrix to CSR sparse format.
    pub fn from_dense(matrix: &DenseMatrix) -> Self {
        let rows = matrix.rows();
        let cols = matrix.cols();
        let mut row_ptrs = Vec::with_capacity(rows + 1);
        let mut col_indices = Vec::new();
        let mut values = Vec::new();

        row_ptrs.push(0);
        for i in 0..rows {
            for j in 0..cols {
                let val = matrix.get(i, j);
                if val.norm() > 1e-15 {
                    col_indices.push(j);
                    values.push(val);
                }
            }
            row_ptrs.push(col_indices.len());
        }

        Self {
            rows,
            cols,
            row_ptrs,
            col_indices,
            values,
        }
    }

    /// Convert back to dense.
    pub fn to_dense(&self) -> DenseMatrix {
        let mut result = DenseMatrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for idx in self.row_ptrs[i]..self.row_ptrs[i + 1] {
                let j = self.col_indices[idx];
                result.set(i, j, self.values[idx]);
            }
        }
        result
    }

    /// Sparse matrix-vector multiply.
    pub fn multiply_vec(&self, vec: &[Complex64]) -> Result<Vec<Complex64>, MathError> {
        if self.cols != vec.len() {
            return Err(MathError::DimensionMismatch {
                expected: self.cols,
                got: vec.len(),
            });
        }
        let mut result = vec![Complex64::new(0.0, 0.0); self.rows];
        for (i, res) in result.iter_mut().enumerate() {
            for idx in self.row_ptrs[i]..self.row_ptrs[i + 1] {
                *res += self.values[idx] * vec[self.col_indices[idx]];
            }
        }
        Ok(result)
    }

    /// Number of non-zero entries.
    #[inline]
    pub fn nnz(&self) -> usize {
        self.values.len()
    }

    #[inline]
    pub fn rows(&self) -> usize {
        self.rows
    }

    #[inline]
    pub fn cols(&self) -> usize {
        self.cols
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let mut m = DenseMatrix::new(3, 3);
        m.set(0, 0, Complex64::new(1.0, 0.0));
        m.set(1, 2, Complex64::new(2.0, 1.0));
        m.set(2, 1, Complex64::new(-1.0, 0.0));

        let sparse = SparseMatrix::from_dense(&m);
        assert_eq!(sparse.nnz(), 3);

        let dense = sparse.to_dense();
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (dense.get(i, j) - m.get(i, j)).norm() < 1e-10,
                    "Mismatch at ({i}, {j})"
                );
            }
        }
    }

    #[test]
    fn test_sparse_multiply_vec() {
        let id = DenseMatrix::identity(3);
        let sparse = SparseMatrix::from_dense(&id);
        let v = vec![
            Complex64::new(1.0, 0.0),
            Complex64::new(2.0, 0.0),
            Complex64::new(3.0, 0.0),
        ];
        let result = sparse.multiply_vec(&v).unwrap();
        for i in 0..3 {
            assert!((result[i] - v[i]).norm() < 1e-10);
        }
    }

    #[test]
    fn test_empty_sparse() {
        let s = SparseMatrix::new(3, 3);
        assert_eq!(s.nnz(), 0);
        let v = vec![Complex64::new(1.0, 0.0); 3];
        let result = s.multiply_vec(&v).unwrap();
        for r in &result {
            assert!(r.norm() < 1e-10);
        }
    }
}
