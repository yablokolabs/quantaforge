use qf_logical::{
    apply_logical_cnot, apply_logical_gate, estimate_resources, LogicalGate, LogicalQubit,
    LogicalState,
};

#[test]
fn surface_code_d3_has_9_physical_qubits() {
    let q = LogicalQubit::from_surface_code(0, 3).unwrap();
    assert_eq!(q.physical_qubit_count(), 9);
}

#[test]
fn logical_qubit_starts_in_zero() {
    let q = LogicalQubit::from_surface_code(0, 3).unwrap();
    assert_eq!(q.state, LogicalState::Zero);
}

#[test]
fn apply_x_zero_to_one() {
    let mut q = LogicalQubit::from_surface_code(0, 3).unwrap();
    apply_logical_gate(&mut q, LogicalGate::X).unwrap();
    assert_eq!(q.state, LogicalState::One);
}

#[test]
fn apply_x_one_to_zero() {
    let mut q = LogicalQubit::from_surface_code(0, 3).unwrap();
    apply_logical_gate(&mut q, LogicalGate::X).unwrap();
    apply_logical_gate(&mut q, LogicalGate::X).unwrap();
    assert_eq!(q.state, LogicalState::Zero);
}

#[test]
fn apply_h_zero_to_plus() {
    let mut q = LogicalQubit::from_surface_code(0, 3).unwrap();
    apply_logical_gate(&mut q, LogicalGate::H).unwrap();
    assert_eq!(q.state, LogicalState::Plus);
}

#[test]
fn apply_h_plus_to_zero() {
    let mut q = LogicalQubit::from_surface_code(0, 3).unwrap();
    apply_logical_gate(&mut q, LogicalGate::H).unwrap();
    apply_logical_gate(&mut q, LogicalGate::H).unwrap();
    assert_eq!(q.state, LogicalState::Zero);
}

#[test]
fn apply_z_plus_to_minus() {
    let mut q = LogicalQubit::from_surface_code(0, 3).unwrap();
    apply_logical_gate(&mut q, LogicalGate::H).unwrap();
    apply_logical_gate(&mut q, LogicalGate::Z).unwrap();
    assert_eq!(q.state, LogicalState::Minus);
}

#[test]
fn cnot_zero_zero_unchanged() {
    let mut ctrl = LogicalQubit::from_surface_code(0, 3).unwrap();
    let mut tgt = LogicalQubit::from_surface_code(1, 3).unwrap();
    apply_logical_cnot(&mut ctrl, &mut tgt).unwrap();
    assert_eq!(ctrl.state, LogicalState::Zero);
    assert_eq!(tgt.state, LogicalState::Zero);
}

#[test]
fn cnot_one_zero_produces_one_one() {
    let mut ctrl = LogicalQubit::from_surface_code(0, 3).unwrap();
    let mut tgt = LogicalQubit::from_surface_code(1, 3).unwrap();
    apply_logical_gate(&mut ctrl, LogicalGate::X).unwrap();
    apply_logical_cnot(&mut ctrl, &mut tgt).unwrap();
    assert_eq!(ctrl.state, LogicalState::One);
    assert_eq!(tgt.state, LogicalState::One);
}

#[test]
fn cnot_control_one_flips_target() {
    let mut ctrl = LogicalQubit::from_surface_code(0, 3).unwrap();
    let mut tgt = LogicalQubit::from_surface_code(1, 3).unwrap();
    apply_logical_gate(&mut ctrl, LogicalGate::X).unwrap();
    apply_logical_gate(&mut tgt, LogicalGate::X).unwrap(); // target = One
    apply_logical_cnot(&mut ctrl, &mut tgt).unwrap();
    assert_eq!(tgt.state, LogicalState::Zero); // One flipped back to Zero
}

#[test]
fn cnot_single_qubit_returns_error() {
    let mut q = LogicalQubit::from_surface_code(0, 3).unwrap();
    let result = apply_logical_gate(&mut q, LogicalGate::CNOT);
    assert!(result.is_err());
}

#[test]
fn resource_estimate_two_logical_qubits_d3() {
    let est = estimate_resources(2, 3).unwrap();
    assert_eq!(est.num_logical_qubits, 2);
    assert_eq!(est.code_distance, 3);
    assert_eq!(
        est.total_physical_qubits,
        est.physical_qubits_per_logical * 2
    );
    assert!(est.physical_qubits_per_logical > 0);
}

#[test]
fn serialization_roundtrip() {
    let q = LogicalQubit::from_surface_code(42, 3).unwrap();
    let json = serde_json::to_string(&q).unwrap();
    let q2: LogicalQubit = serde_json::from_str(&json).unwrap();
    assert_eq!(q2.id, 42);
    assert_eq!(q2.state, LogicalState::Zero);
    assert_eq!(q2.num_physical_qubits, 9);
}
