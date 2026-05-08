use serde::{Deserialize, Serialize};

use crate::error::QecError;

/// Qubit role in the surface code lattice
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QubitRole {
    Data,
    MeasureX,
    MeasureZ,
}

/// A qubit in the surface code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceQubit {
    pub id: usize,
    pub role: QubitRole,
    pub row: usize,
    pub col: usize,
}

/// Rotated surface code with a given code distance.
///
/// Layout (distance d, d odd, d ≥ 3):
/// - d² data qubits on a d×d grid, ID = row * d + col
/// - (d²−1)/2 X stabilizers and (d²−1)/2 Z stabilizers
///
/// Interior stabilizers (weight 4) sit on faces of the grid:
///   face (r,c) acts on data qubits (r,c), (r,c+1), (r+1,c), (r+1,c+1)
///   X-type when (r+c) is odd, Z-type when (r+c) is even
///
/// Boundary stabilizers (weight 2):
///   X-type on left  (col 0,   odd rows) and right (col d−1, even rows)
///   Z-type on top   (row 0,   even cols) and bottom (row d−1, odd cols)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceCode {
    pub distance: usize,
    pub data_qubits: Vec<SurfaceQubit>,
    pub measure_x_qubits: Vec<SurfaceQubit>,
    pub measure_z_qubits: Vec<SurfaceQubit>,
    x_stabilizers: Vec<Vec<usize>>,
    z_stabilizers: Vec<Vec<usize>>,
}

impl SurfaceCode {
    /// Build a rotated surface code of the given distance.
    /// Distance must be odd and ≥ 3.
    pub fn new(distance: usize) -> Result<Self, QecError> {
        if distance < 3 || distance.is_multiple_of(2) {
            return Err(QecError::InvalidDistance(distance));
        }

        let d = distance;
        let mut data_qubits = Vec::with_capacity(d * d);
        let mut measure_x_qubits = Vec::new();
        let mut measure_z_qubits = Vec::new();
        let mut x_stabilizers: Vec<Vec<usize>> = Vec::new();
        let mut z_stabilizers: Vec<Vec<usize>> = Vec::new();

        // Data qubits
        for r in 0..d {
            for c in 0..d {
                data_qubits.push(SurfaceQubit {
                    id: r * d + c,
                    role: QubitRole::Data,
                    row: r,
                    col: c,
                });
            }
        }

        let mut ancilla_id = d * d;

        // --- X stabilizers ---

        // Interior X: faces where (r+c) is even (weight 4)
        for r in 0..d - 1 {
            for c in 0..d - 1 {
                if (r + c) % 2 == 0 {
                    x_stabilizers.push(vec![
                        r * d + c,
                        r * d + c + 1,
                        (r + 1) * d + c,
                        (r + 1) * d + c + 1,
                    ]);
                    measure_x_qubits.push(SurfaceQubit {
                        id: ancilla_id,
                        role: QubitRole::MeasureX,
                        row: r,
                        col: c,
                    });
                    ancilla_id += 1;
                }
            }
        }

        // Left boundary X: column 0, odd rows (weight 2)
        for r in (1..d - 1).step_by(2) {
            x_stabilizers.push(vec![r * d, (r + 1) * d]);
            measure_x_qubits.push(SurfaceQubit {
                id: ancilla_id,
                role: QubitRole::MeasureX,
                row: r,
                col: 0,
            });
            ancilla_id += 1;
        }

        // Right boundary X: column d-1, even rows (weight 2)
        for r in (0..d - 1).step_by(2) {
            x_stabilizers.push(vec![r * d + (d - 1), (r + 1) * d + (d - 1)]);
            measure_x_qubits.push(SurfaceQubit {
                id: ancilla_id,
                role: QubitRole::MeasureX,
                row: r,
                col: d - 1,
            });
            ancilla_id += 1;
        }

        // --- Z stabilizers ---

        // Interior Z: faces where (r+c) is odd (weight 4)
        for r in 0..d - 1 {
            for c in 0..d - 1 {
                if (r + c) % 2 == 1 {
                    z_stabilizers.push(vec![
                        r * d + c,
                        r * d + c + 1,
                        (r + 1) * d + c,
                        (r + 1) * d + c + 1,
                    ]);
                    measure_z_qubits.push(SurfaceQubit {
                        id: ancilla_id,
                        role: QubitRole::MeasureZ,
                        row: r,
                        col: c,
                    });
                    ancilla_id += 1;
                }
            }
        }

        // Top boundary Z: row 0, even columns (weight 2)
        for c in (0..d - 1).step_by(2) {
            z_stabilizers.push(vec![c, c + 1]);
            measure_z_qubits.push(SurfaceQubit {
                id: ancilla_id,
                role: QubitRole::MeasureZ,
                row: 0,
                col: c,
            });
            ancilla_id += 1;
        }

        // Bottom boundary Z: row d-1, odd columns (weight 2)
        for c in (1..d - 1).step_by(2) {
            z_stabilizers.push(vec![(d - 1) * d + c, (d - 1) * d + c + 1]);
            measure_z_qubits.push(SurfaceQubit {
                id: ancilla_id,
                role: QubitRole::MeasureZ,
                row: d - 1,
                col: c,
            });
            ancilla_id += 1;
        }

        Ok(SurfaceCode {
            distance: d,
            data_qubits,
            measure_x_qubits,
            measure_z_qubits,
            x_stabilizers,
            z_stabilizers,
        })
    }

    pub fn num_data_qubits(&self) -> usize {
        self.data_qubits.len()
    }

    pub fn num_ancilla_qubits(&self) -> usize {
        self.measure_x_qubits.len() + self.measure_z_qubits.len()
    }

    pub fn total_qubits(&self) -> usize {
        self.num_data_qubits() + self.num_ancilla_qubits()
    }

    pub fn num_x_stabilizers(&self) -> usize {
        self.x_stabilizers.len()
    }

    pub fn num_z_stabilizers(&self) -> usize {
        self.z_stabilizers.len()
    }

    /// Data-qubit IDs acted on by the given X stabilizer (weight 2 or 4).
    pub fn x_stabilizer_qubits(&self, stabilizer_index: usize) -> Vec<usize> {
        self.x_stabilizers[stabilizer_index].clone()
    }

    /// Data-qubit IDs acted on by the given Z stabilizer (weight 2 or 4).
    pub fn z_stabilizer_qubits(&self, stabilizer_index: usize) -> Vec<usize> {
        self.z_stabilizers[stabilizer_index].clone()
    }

    /// Borrow the full table of X-stabilizer supports.
    pub fn x_stabilizer_table(&self) -> &[Vec<usize>] {
        &self.x_stabilizers
    }

    /// Borrow the full table of Z-stabilizer supports.
    pub fn z_stabilizer_table(&self) -> &[Vec<usize>] {
        &self.z_stabilizers
    }
}
