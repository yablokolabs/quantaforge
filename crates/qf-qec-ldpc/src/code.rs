use serde::{Deserialize, Serialize};

use crate::error::LdpcError;
use crate::parity_check::ParityCheckMatrix;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassicalLdpcCode {
    pub parity_check: ParityCheckMatrix,
    pub name: String,
}

/// CSS (Calderbank-Shor-Steane) code from two classical codes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssCode {
    pub hx: ParityCheckMatrix,
    pub hz: ParityCheckMatrix,
    pub name: String,
}

impl CssCode {
    /// Validates CSS orthogonality: Hx * Hz^T = 0 (mod 2)
    pub fn new(
        hx: ParityCheckMatrix,
        hz: ParityCheckMatrix,
        name: &str,
    ) -> Result<Self, LdpcError> {
        if hx.cols() != hz.cols() {
            return Err(LdpcError::CssDimensionMismatch {
                hx_cols: hx.cols(),
                hz_cols: hz.cols(),
            });
        }
        // Check Hx * Hz^T = 0 (mod 2)
        let hzt = hz.transpose();
        for r in 0..hx.rows() {
            for c in 0..hzt.cols() {
                let mut dot = 0u8;
                for k in 0..hx.cols() {
                    dot ^= hx.get(r, k) & hzt.get(k, c);
                }
                if dot != 0 {
                    return Err(LdpcError::CssOrthogonalityViolated);
                }
            }
        }
        Ok(Self {
            hx,
            hz,
            name: name.to_string(),
        })
    }

    pub fn num_physical_qubits(&self) -> usize {
        self.hx.cols()
    }

    pub fn num_x_checks(&self) -> usize {
        self.hx.rows()
    }

    pub fn num_z_checks(&self) -> usize {
        self.hz.rows()
    }

    /// n - rank(Hx) - rank(Hz)
    pub fn num_logical_qubits(&self) -> usize {
        let n = self.num_physical_qubits();
        let rx = gf2_rank(&self.hx);
        let rz = gf2_rank(&self.hz);
        n.saturating_sub(rx + rz)
    }
}

/// Q-LDPC code wrapper — same as CSS but with explicit LDPC constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QLdpcCode {
    pub css: CssCode,
    pub max_weight: usize,
}

impl QLdpcCode {
    pub fn new(css: CssCode, max_weight: usize) -> Result<Self, LdpcError> {
        if !css.hx.is_ldpc(max_weight) || !css.hz.is_ldpc(max_weight) {
            return Err(LdpcError::WeightConstraintViolated { max_weight });
        }
        Ok(Self { css, max_weight })
    }
}

/// Gaussian elimination over GF(2) to compute rank
pub fn gf2_rank(matrix: &ParityCheckMatrix) -> usize {
    let rows = matrix.rows();
    let cols = matrix.cols();
    let mut m: Vec<Vec<u8>> = matrix.data().clone();

    let mut rank = 0;
    for col in 0..cols {
        // Find pivot row
        let pivot = (rank..rows).find(|&row| m[row][col] == 1);
        let pivot = match pivot {
            Some(p) => p,
            None => continue,
        };

        m.swap(rank, pivot);

        // Eliminate all other rows
        let pivot_row = m[rank].clone();
        for (row, m_row) in m.iter_mut().enumerate() {
            if row != rank && m_row[col] == 1 {
                for (c, cell) in m_row.iter_mut().enumerate() {
                    *cell ^= pivot_row[c];
                }
            }
        }
        rank += 1;
    }
    rank
}
