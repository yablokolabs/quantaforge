use num_complex::Complex64;
use qf_math::complex;
use qf_math::matrix::*;
use qf_math::sparse::SparseMatrix;
use qf_math::tensor::tensor_product;
use qf_math::vector::StateVector;

const EPS: f64 = 1e-10;

fn assert_complex_eq(a: Complex64, b: Complex64) {
    assert!(
        (a - b).norm() < EPS,
        "expected {b}, got {a}, diff = {}",
        (a - b).norm()
    );
}

// --- StateVector tests ---

#[test]
fn test_statevector_new_2_gives_00() {
    let sv = StateVector::new(2);
    assert_eq!(sv.num_qubits(), 2);
    assert_eq!(sv.dim(), 4);
    assert_complex_eq(sv.amplitude(0).unwrap(), Complex64::new(1.0, 0.0));
    for i in 1..4 {
        assert_complex_eq(sv.amplitude(i).unwrap(), Complex64::new(0.0, 0.0));
    }
}

// --- Unitary tests ---

#[test]
fn test_hadamard_is_unitary() {
    assert!(hadamard().is_unitary(EPS));
}

#[test]
fn test_pauli_matrices_unitary_and_self_inverse() {
    let paulis = [pauli_x(), pauli_y(), pauli_z()];
    let id = DenseMatrix::identity(2);
    for gate in &paulis {
        assert!(gate.is_unitary(EPS), "Pauli gate is not unitary");
        let sq = gate.multiply(gate).unwrap();
        for i in 0..2 {
            for j in 0..2 {
                assert_complex_eq(sq.get(i, j), id.get(i, j));
            }
        }
    }
}

// --- Tensor product tests ---

#[test]
fn test_tensor_product_identity() {
    let i2 = DenseMatrix::identity(2);
    let result = tensor_product(&i2, &i2);
    let i4 = DenseMatrix::identity(4);
    assert_eq!(result.rows(), 4);
    assert_eq!(result.cols(), 4);
    for r in 0..4 {
        for c in 0..4 {
            assert_complex_eq(result.get(r, c), i4.get(r, c));
        }
    }
}

// --- Matrix-vector multiply tests ---

#[test]
fn test_h_ket0_gives_plus() {
    let h = hadamard();
    let ket0 = vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)];
    let result = h.multiply_vec(&ket0).unwrap();
    let s = 1.0 / 2.0_f64.sqrt();
    assert_complex_eq(result[0], Complex64::new(s, 0.0));
    assert_complex_eq(result[1], Complex64::new(s, 0.0));
}

#[test]
fn test_cnot_10_gives_11() {
    let cx = cnot();
    // |10⟩ = [0, 0, 1, 0]
    let ket10 = vec![
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(1.0, 0.0),
        Complex64::new(0.0, 0.0),
    ];
    let result = cx.multiply_vec(&ket10).unwrap();
    // |11⟩ = [0, 0, 0, 1]
    assert_complex_eq(result[0], Complex64::new(0.0, 0.0));
    assert_complex_eq(result[1], Complex64::new(0.0, 0.0));
    assert_complex_eq(result[2], Complex64::new(0.0, 0.0));
    assert_complex_eq(result[3], Complex64::new(1.0, 0.0));
}

// --- Inner product tests ---

#[test]
fn test_inner_product_orthonormal() {
    let sv0 = StateVector::new(1); // |0⟩
    let mut sv1 = StateVector::new(1);
    sv1.set_amplitude(0, Complex64::new(0.0, 0.0)).unwrap();
    sv1.set_amplitude(1, Complex64::new(1.0, 0.0)).unwrap(); // |1⟩

    assert_complex_eq(sv0.inner_product(&sv0).unwrap(), Complex64::new(1.0, 0.0));
    assert_complex_eq(sv0.inner_product(&sv1).unwrap(), Complex64::new(0.0, 0.0));
}

// --- Sparse matrix tests ---

#[test]
fn test_sparse_matches_dense_multiply() {
    let h = hadamard();
    let sparse_h = SparseMatrix::from_dense(&h);
    let v = vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)];

    let dense_result = h.multiply_vec(&v).unwrap();
    let sparse_result = sparse_h.multiply_vec(&v).unwrap();

    for i in 0..2 {
        assert_complex_eq(dense_result[i], sparse_result[i]);
    }
}

// --- Additional gate tests ---

#[test]
fn test_all_single_qubit_gates_unitary() {
    assert!(phase_s().is_unitary(EPS));
    assert!(t_gate().is_unitary(EPS));
    assert!(rx(1.0).is_unitary(EPS));
    assert!(ry(2.0).is_unitary(EPS));
    assert!(rz(3.0).is_unitary(EPS));
}

#[test]
fn test_two_qubit_gates_unitary() {
    assert!(cnot().is_unitary(EPS));
    assert!(cz().is_unitary(EPS));
}

#[test]
fn test_complex_constants() {
    assert_complex_eq(complex::ZERO, Complex64::new(0.0, 0.0));
    assert_complex_eq(complex::ONE, Complex64::new(1.0, 0.0));
    assert_complex_eq(complex::I, Complex64::new(0.0, 1.0));
}

#[test]
fn test_approx_eq() {
    let a = Complex64::new(1.0, 0.0);
    let b = Complex64::new(1.0 + 1e-12, 0.0);
    assert!(complex::approx_eq(a, b, 1e-10));
    assert!(!complex::approx_eq(a, Complex64::new(2.0, 0.0), 0.5));
}

// --- Property-based tests ---

mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    fn arb_complex() -> impl Strategy<Value = Complex64> {
        (-10.0..10.0_f64, -10.0..10.0_f64).prop_map(|(re, im)| Complex64::new(re, im))
    }

    fn arb_matrix_2x2() -> impl Strategy<Value = DenseMatrix> {
        proptest::collection::vec(arb_complex(), 4)
            .prop_map(|data| DenseMatrix::from_row_major(2, 2, data).unwrap())
    }

    proptest! {
        #[test]
        fn test_conjugate_transpose_involution(m in arb_matrix_2x2()) {
            let mdd = m.conjugate_transpose().conjugate_transpose();
            for i in 0..4 {
                prop_assert!((mdd.data()[i] - m.data()[i]).norm() < 1e-10);
            }
        }

        #[test]
        fn test_multiply_associativity(
            a in arb_matrix_2x2(),
            b in arb_matrix_2x2(),
            c in arb_matrix_2x2(),
        ) {
            let ab_c = a.multiply(&b).unwrap().multiply(&c).unwrap();
            let a_bc = a.multiply(&b.multiply(&c).unwrap()).unwrap();
            for i in 0..4 {
                prop_assert!(
                    (ab_c.data()[i] - a_bc.data()[i]).norm() < 1e-6,
                    "Associativity failed at index {}", i
                );
            }
        }

        #[test]
        fn test_tensor_product_dimensions(
            ar in 1usize..=4,
            ac in 1usize..=4,
            br in 1usize..=4,
            bc in 1usize..=4,
        ) {
            let a = DenseMatrix::new(ar, ac);
            let b = DenseMatrix::new(br, bc);
            let result = tensor_product(&a, &b);
            prop_assert_eq!(result.rows(), ar * br);
            prop_assert_eq!(result.cols(), ac * bc);
        }
    }
}
