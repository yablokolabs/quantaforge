use qf_circuit::{Circuit, CircuitBuilder, CircuitError, Gate};

#[test]
fn bell_state_circuit() {
    let circuit = CircuitBuilder::new(2)
        .unwrap()
        .h(0)
        .unwrap()
        .cnot(0, 1)
        .unwrap()
        .build();
    assert_eq!(circuit.instructions().len(), 2);
    assert_eq!(circuit.num_qubits(), 2);
}

#[test]
fn all_gate_types() {
    let circuit = CircuitBuilder::new(3)
        .unwrap()
        .h(0)
        .unwrap()
        .x(0)
        .unwrap()
        .y(0)
        .unwrap()
        .z(0)
        .unwrap()
        .s(0)
        .unwrap()
        .t(0)
        .unwrap()
        .rx(1, 1.0)
        .unwrap()
        .ry(1, 2.0)
        .unwrap()
        .rz(1, 3.0)
        .unwrap()
        .cnot(0, 1)
        .unwrap()
        .cz(1, 2)
        .unwrap()
        .measure(0)
        .unwrap()
        .build();
    assert_eq!(circuit.gate_count(), 12);
}

#[test]
fn clifford_only_true() {
    let circuit = CircuitBuilder::new(2)
        .unwrap()
        .h(0)
        .unwrap()
        .cnot(0, 1)
        .unwrap()
        .build();
    assert!(circuit.is_clifford_only());
}

#[test]
fn clifford_only_false_with_t() {
    let circuit = CircuitBuilder::new(2)
        .unwrap()
        .h(0)
        .unwrap()
        .cnot(0, 1)
        .unwrap()
        .t(0)
        .unwrap()
        .build();
    assert!(!circuit.is_clifford_only());
}

#[test]
fn circuit_depth() {
    // H on q0 (depth 1), then CNOT 0,1 (depth 2 for both q0 and q1)
    let circuit = CircuitBuilder::new(2)
        .unwrap()
        .h(0)
        .unwrap()
        .cnot(0, 1)
        .unwrap()
        .build();
    assert_eq!(circuit.depth(), 2);
}

#[test]
fn parse_circuit_valid() {
    let input = "H 0\nCNOT 0 1";
    let circuit = qf_circuit::parser::parse_circuit(input, 2).unwrap();
    assert_eq!(circuit.instructions().len(), 2);
    assert_eq!(circuit.instructions()[0].gate, Gate::H);
    assert_eq!(circuit.instructions()[1].gate, Gate::CNOT);
}

#[test]
fn serialize_json_roundtrip() {
    let circuit = CircuitBuilder::new(2)
        .unwrap()
        .h(0)
        .unwrap()
        .cnot(0, 1)
        .unwrap()
        .build();
    let json = qf_circuit::serialization::to_json(&circuit).unwrap();
    let restored = qf_circuit::serialization::from_json(&json).unwrap();
    assert_eq!(circuit.num_qubits(), restored.num_qubits());
    assert_eq!(circuit.gate_count(), restored.gate_count());
    for (a, b) in circuit
        .instructions()
        .iter()
        .zip(restored.instructions().iter())
    {
        assert_eq!(a, b);
    }
}

#[test]
fn serialize_yaml_roundtrip() {
    let circuit = CircuitBuilder::new(2)
        .unwrap()
        .h(0)
        .unwrap()
        .cnot(0, 1)
        .unwrap()
        .build();
    let yaml = qf_circuit::serialization::to_yaml(&circuit).unwrap();
    let restored = qf_circuit::serialization::from_yaml(&yaml).unwrap();
    assert_eq!(circuit.num_qubits(), restored.num_qubits());
    assert_eq!(circuit.gate_count(), restored.gate_count());
    for (a, b) in circuit
        .instructions()
        .iter()
        .zip(restored.instructions().iter())
    {
        assert_eq!(a, b);
    }
}

#[test]
fn qubit_out_of_range() {
    let result = CircuitBuilder::new(2).unwrap().h(5);
    match result {
        Err(CircuitError::QubitOutOfRange { index, num_qubits }) => {
            assert_eq!(index, 5);
            assert_eq!(num_qubits, 2);
        }
        Err(other) => panic!("expected QubitOutOfRange, got {:?}", other),
        Ok(_) => panic!("expected error"),
    }
}

#[test]
fn zero_qubit_circuit() {
    let result = Circuit::new(0);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CircuitError::ZeroQubits));
}

#[test]
fn measure_returns_none_matrix() {
    assert!(Gate::Measure.matrix().is_none());
}

#[test]
fn hadamard_matrix_matches_qf_math() {
    let gate_matrix = Gate::H.matrix().unwrap();
    let math_matrix = qf_math::matrix::hadamard();
    assert_eq!(gate_matrix.rows(), math_matrix.rows());
    assert_eq!(gate_matrix.cols(), math_matrix.cols());
    for r in 0..gate_matrix.rows() {
        for c in 0..gate_matrix.cols() {
            let diff = (gate_matrix.get(r, c) - math_matrix.get(r, c)).norm();
            assert!(diff < 1e-10, "mismatch at ({}, {})", r, c);
        }
    }
}
