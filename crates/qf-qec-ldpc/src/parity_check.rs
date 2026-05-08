use serde::{Deserialize, Serialize};

use crate::error::LdpcError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParityCheckMatrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<u8>>,
}

impl ParityCheckMatrix {
    pub fn new(rows: usize, cols: usize) -> Result<Self, LdpcError> {
        if rows == 0 || cols == 0 {
            return Err(LdpcError::InvalidDimensions { rows, cols });
        }
        Ok(Self {
            rows,
            cols,
            data: vec![vec![0u8; cols]; rows],
        })
    }

    pub fn from_dense(data: Vec<Vec<u8>>) -> Result<Self, LdpcError> {
        if data.is_empty() {
            return Err(LdpcError::InvalidDimensions { rows: 0, cols: 0 });
        }
        let cols = data[0].len();
        if cols == 0 {
            return Err(LdpcError::InvalidDimensions {
                rows: data.len(),
                cols: 0,
            });
        }
        for (i, row) in data.iter().enumerate() {
            if row.len() != cols {
                return Err(LdpcError::InconsistentRowLengths);
            }
            for &val in row {
                if val > 1 {
                    return Err(LdpcError::InvalidValue(val));
                }
            }
            let _ = i;
        }
        Ok(Self {
            rows: data.len(),
            cols,
            data,
        })
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.data[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, val: u8) {
        self.data[row][col] = val & 1;
    }

    pub fn row_weight(&self, row: usize) -> usize {
        self.data[row].iter().filter(|&&v| v == 1).count()
    }

    pub fn col_weight(&self, col: usize) -> usize {
        self.data.iter().filter(|row| row[col] == 1).count()
    }

    pub fn max_row_weight(&self) -> usize {
        (0..self.rows)
            .map(|r| self.row_weight(r))
            .max()
            .unwrap_or(0)
    }

    pub fn max_col_weight(&self) -> usize {
        (0..self.cols)
            .map(|c| self.col_weight(c))
            .max()
            .unwrap_or(0)
    }

    pub fn is_ldpc(&self, max_weight: usize) -> bool {
        self.max_row_weight() <= max_weight && self.max_col_weight() <= max_weight
    }

    /// Compute syndrome = H * error (mod 2)
    pub fn syndrome(&self, error: &[u8]) -> Result<Vec<u8>, LdpcError> {
        if error.len() != self.cols {
            return Err(LdpcError::SyndromeMismatch {
                expected: self.cols,
                got: error.len(),
            });
        }
        let mut syn = vec![0u8; self.rows];
        for (r, row) in self.data.iter().enumerate() {
            let mut acc = 0u8;
            for (c, &h) in row.iter().enumerate() {
                acc ^= h & error[c];
            }
            syn[r] = acc;
        }
        Ok(syn)
    }

    pub fn transpose(&self) -> ParityCheckMatrix {
        let mut t_data = vec![vec![0u8; self.rows]; self.cols];
        for (r, row) in self.data.iter().enumerate() {
            for (c, &val) in row.iter().enumerate() {
                t_data[c][r] = val;
            }
        }
        ParityCheckMatrix {
            rows: self.cols,
            cols: self.rows,
            data: t_data,
        }
    }

    /// Access underlying data for GF(2) rank computation
    pub fn data(&self) -> &Vec<Vec<u8>> {
        &self.data
    }
}

/// Repetition code parity check matrix: (n-1) × n
pub fn repetition_code(n: usize) -> ParityCheckMatrix {
    assert!(n >= 2, "Repetition code requires n >= 2");
    let rows = n - 1;
    let mut data = vec![vec![0u8; n]; rows];
    for r in 0..rows {
        data[r][r] = 1;
        data[r][r + 1] = 1;
    }
    ParityCheckMatrix {
        rows,
        cols: n,
        data,
    }
}

/// Hamming code [2^r - 1, 2^r - 1 - r] parity check matrix
pub fn hamming_code(r: usize) -> ParityCheckMatrix {
    assert!(r >= 2, "Hamming code requires r >= 2");
    let n = (1usize << r) - 1; // 2^r - 1
                               // Columns are binary representations of 1..=n
    let data: Vec<Vec<u8>> = (0..r)
        .map(|row| {
            (0..n)
                .map(|col| {
                    let val = col + 1;
                    let bit = r - 1 - row;
                    ((val >> bit) & 1) as u8
                })
                .collect()
        })
        .collect();
    ParityCheckMatrix {
        rows: r,
        cols: n,
        data,
    }
}
