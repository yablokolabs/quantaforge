use qf_math::tensor::tensor_product;
use qf_math::{Complex64, DenseMatrix, StateVector};

use crate::error::NoiseError;

/// Density matrix representation for mixed quantum states.
#[derive(Debug, Clone)]
pub struct DensityMatrix {
    matrix: DenseMatrix,
    num_qubits: usize,
}

impl DensityMatrix {
    /// Construct ρ = |ψ⟩⟨ψ| from a state vector (outer product).
    pub fn from_statevector(sv: &StateVector) -> Self {
        let dim = sv.dim();
        let mut matrix = DenseMatrix::new(dim, dim);
        for i in 0..dim {
            let ai = sv.data()[i];
            for j in 0..dim {
                let aj = sv.data()[j];
                matrix.set(i, j, ai * aj.conj());
            }
        }
        DensityMatrix {
            matrix,
            num_qubits: sv.num_qubits(),
        }
    }

    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    pub fn dim(&self) -> usize {
        1usize << self.num_qubits
    }

    pub fn trace(&self) -> Complex64 {
        self.matrix.trace()
    }

    pub fn matrix(&self) -> &DenseMatrix {
        &self.matrix
    }

    /// Purity = Tr(ρ²). Equals 1 for pure states, 1/d for maximally mixed.
    pub fn purity(&self) -> f64 {
        let rho2 = self.matrix.multiply(&self.matrix).expect("square matrix");
        rho2.trace().re
    }

    /// Check if the state is approximately pure (purity ≈ 1).
    pub fn is_pure(&self, eps: f64) -> bool {
        (self.purity() - 1.0).abs() < eps
    }
}

/// Kraus operator representation of a quantum channel.
pub trait NoiseChannel: Send + Sync {
    fn name(&self) -> &str;

    /// Return the 2×2 Kraus operators for this single-qubit channel.
    fn kraus_operators(&self, qubit: usize) -> Vec<DenseMatrix>;

    /// Apply the channel to the density matrix on the given qubit.
    fn apply(&self, state: &mut DensityMatrix, qubit: usize) -> Result<(), NoiseError> {
        let kraus = self.kraus_operators(qubit);
        apply_single_qubit_channel(state, &kraus, qubit)
    }

    fn num_qubits(&self) -> usize {
        1
    }
}

/// Embed a single-qubit (2×2) operator into the full n-qubit Hilbert space.
///
/// For an n-qubit system with target qubit `t` (0-indexed from MSB):
///   E_full = I_{2^t} ⊗ E ⊗ I_{2^{n-t-1}}
fn embed_single_qubit_op(op: &DenseMatrix, num_qubits: usize, target: usize) -> DenseMatrix {
    let left = DenseMatrix::identity(1usize << target);
    let right = DenseMatrix::identity(1usize << (num_qubits - target - 1));
    let tmp = tensor_product(&left, op);
    tensor_product(&tmp, &right)
}

/// Apply a single-qubit channel (given as 2×2 Kraus operators) to a density matrix.
///
/// ρ' = Σ_k E_k ρ E_k†
pub fn apply_single_qubit_channel(
    rho: &mut DensityMatrix,
    kraus_ops: &[DenseMatrix],
    target_qubit: usize,
) -> Result<(), NoiseError> {
    if target_qubit >= rho.num_qubits() {
        return Err(NoiseError::InvalidParameter {
            param: "target_qubit",
            value: target_qubit as f64,
        });
    }

    let n = rho.num_qubits();
    let dim = rho.dim();
    let mut new_matrix = DenseMatrix::new(dim, dim);

    for op in kraus_ops {
        let e_full = embed_single_qubit_op(op, n, target_qubit);
        let e_dag = e_full.conjugate_transpose();
        // E_k ρ E_k†
        let tmp = e_full
            .multiply(rho.matrix())
            .map_err(NoiseError::MathError)?;
        let contribution = tmp.multiply(&e_dag).map_err(NoiseError::MathError)?;

        for i in 0..dim {
            for j in 0..dim {
                let cur = new_matrix.get(i, j);
                new_matrix.set(i, j, cur + contribution.get(i, j));
            }
        }
    }

    rho.matrix = new_matrix;
    Ok(())
}
