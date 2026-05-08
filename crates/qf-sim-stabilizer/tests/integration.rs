use qf_circuit::{Circuit, Gate, Instruction};
use qf_sim_stabilizer::{is_deterministic, StabilizerError, StabilizerSimulator, Tableau};
use rand::rngs::StdRng;
use rand::SeedableRng;

fn make_circuit(num_qubits: usize, ops: &[(Gate, &[usize])]) -> Circuit {
    let mut c = Circuit::new(num_qubits).unwrap();
    for (gate, qubits) in ops {
        c.add_instruction(Instruction::new(gate.clone(), qubits.to_vec()).unwrap())
            .unwrap();
    }
    c
}

#[test]
fn z_ket0_measurement() {
    // No gates, measure qubit 0 → always 0
    let circuit = make_circuit(1, &[(Gate::Measure, &[0])]);
    let sim = StabilizerSimulator::new();
    for seed in 0..20 {
        let mut rng = StdRng::seed_from_u64(seed);
        let result = sim.run_with_rng(&circuit, &mut rng).unwrap();
        assert_eq!(result.measurements[0], Some(false));
    }
}

#[test]
fn x_ket0_measurement() {
    // X|0⟩ = |1⟩, measure → always 1
    let circuit = make_circuit(1, &[(Gate::X, &[0]), (Gate::Measure, &[0])]);
    let sim = StabilizerSimulator::new();
    for seed in 0..20 {
        let mut rng = StdRng::seed_from_u64(seed);
        let result = sim.run_with_rng(&circuit, &mut rng).unwrap();
        assert_eq!(result.measurements[0], Some(true));
    }
}

#[test]
fn h_ket0_measurement_random() {
    // H|0⟩ = |+⟩, measure → random
    let circuit = make_circuit(1, &[(Gate::H, &[0]), (Gate::Measure, &[0])]);
    let sim = StabilizerSimulator::new();
    let mut seen_false = false;
    let mut seen_true = false;
    for seed in 0..100 {
        let mut rng = StdRng::seed_from_u64(seed);
        let result = sim.run_with_rng(&circuit, &mut rng).unwrap();
        match result.measurements[0] {
            Some(false) => seen_false = true,
            Some(true) => seen_true = true,
            None => panic!("expected measurement"),
        }
    }
    assert!(seen_false, "expected to see outcome 0");
    assert!(seen_true, "expected to see outcome 1");
}

#[test]
fn bell_state() {
    // H q0, CNOT q0→q1, measure both → always same result
    let circuit = make_circuit(
        2,
        &[
            (Gate::H, &[0]),
            (Gate::CNOT, &[0, 1]),
            (Gate::Measure, &[0]),
            (Gate::Measure, &[1]),
        ],
    );
    let sim = StabilizerSimulator::new();
    for seed in 0..50 {
        let mut rng = StdRng::seed_from_u64(seed);
        let result = sim.run_with_rng(&circuit, &mut rng).unwrap();
        assert_eq!(
            result.measurements[0], result.measurements[1],
            "Bell state qubits must agree (seed={})",
            seed
        );
    }
}

#[test]
fn ghz_state() {
    // H q0, CNOT q0→q1, CNOT q0→q2, measure all → all agree
    let circuit = make_circuit(
        3,
        &[
            (Gate::H, &[0]),
            (Gate::CNOT, &[0, 1]),
            (Gate::CNOT, &[0, 2]),
            (Gate::Measure, &[0]),
            (Gate::Measure, &[1]),
            (Gate::Measure, &[2]),
        ],
    );
    let sim = StabilizerSimulator::new();
    for seed in 0..50 {
        let mut rng = StdRng::seed_from_u64(seed);
        let result = sim.run_with_rng(&circuit, &mut rng).unwrap();
        let m0 = result.measurements[0];
        assert_eq!(m0, result.measurements[1], "GHZ q0==q1 (seed={})", seed);
        assert_eq!(m0, result.measurements[2], "GHZ q0==q2 (seed={})", seed);
    }
}

#[test]
fn hh_identity() {
    // H twice = identity, measure → always 0
    let circuit = make_circuit(
        1,
        &[(Gate::H, &[0]), (Gate::H, &[0]), (Gate::Measure, &[0])],
    );
    let sim = StabilizerSimulator::new();
    for seed in 0..20 {
        let mut rng = StdRng::seed_from_u64(seed);
        let result = sim.run_with_rng(&circuit, &mut rng).unwrap();
        assert_eq!(result.measurements[0], Some(false));
    }
}

#[test]
fn ss_equals_z() {
    // SS|0⟩ = Z|0⟩ = |0⟩, measure → always 0
    let circuit = make_circuit(
        1,
        &[(Gate::S, &[0]), (Gate::S, &[0]), (Gate::Measure, &[0])],
    );
    let sim = StabilizerSimulator::new();
    for seed in 0..20 {
        let mut rng = StdRng::seed_from_u64(seed);
        let result = sim.run_with_rng(&circuit, &mut rng).unwrap();
        assert_eq!(result.measurements[0], Some(false));
    }
}

#[test]
fn non_clifford_rejection_t() {
    let circuit = make_circuit(1, &[(Gate::T, &[0])]);
    let sim = StabilizerSimulator::new();
    let mut rng = StdRng::seed_from_u64(0);
    let err = sim.run_with_rng(&circuit, &mut rng).unwrap_err();
    assert!(matches!(err, StabilizerError::NotClifford(_)));
}

#[test]
fn non_clifford_rejection_rx() {
    let circuit = make_circuit(1, &[(Gate::Rx(1.0), &[0])]);
    let sim = StabilizerSimulator::new();
    let mut rng = StdRng::seed_from_u64(0);
    let err = sim.run_with_rng(&circuit, &mut rng).unwrap_err();
    assert!(matches!(err, StabilizerError::NotClifford(_)));
}

#[test]
fn non_clifford_rejection_ry() {
    let circuit = make_circuit(1, &[(Gate::Ry(1.0), &[0])]);
    let sim = StabilizerSimulator::new();
    let mut rng = StdRng::seed_from_u64(0);
    let err = sim.run_with_rng(&circuit, &mut rng).unwrap_err();
    assert!(matches!(err, StabilizerError::NotClifford(_)));
}

#[test]
fn non_clifford_rejection_rz() {
    let circuit = make_circuit(1, &[(Gate::Rz(1.0), &[0])]);
    let sim = StabilizerSimulator::new();
    let mut rng = StdRng::seed_from_u64(0);
    let err = sim.run_with_rng(&circuit, &mut rng).unwrap_err();
    assert!(matches!(err, StabilizerError::NotClifford(_)));
}

#[test]
fn cz_test() {
    // CZ behaves like H-on-target, CNOT, H-on-target
    // Test: H q0, CZ q0 q1, measure both
    // Compare with H q0, H q1, CNOT q0 q1, H q1, measure both
    let circuit_cz = make_circuit(
        2,
        &[
            (Gate::H, &[0]),
            (Gate::CZ, &[0, 1]),
            (Gate::Measure, &[0]),
            (Gate::Measure, &[1]),
        ],
    );
    let circuit_cnot = make_circuit(
        2,
        &[
            (Gate::H, &[0]),
            (Gate::H, &[1]),
            (Gate::CNOT, &[0, 1]),
            (Gate::H, &[1]),
            (Gate::Measure, &[0]),
            (Gate::Measure, &[1]),
        ],
    );
    let sim = StabilizerSimulator::new();
    for seed in 0..50 {
        let mut rng1 = StdRng::seed_from_u64(seed);
        let mut rng2 = StdRng::seed_from_u64(seed);
        let r1 = sim.run_with_rng(&circuit_cz, &mut rng1).unwrap();
        let r2 = sim.run_with_rng(&circuit_cnot, &mut rng2).unwrap();
        assert_eq!(
            r1.measurements, r2.measurements,
            "CZ vs CNOT (seed={})",
            seed
        );
    }
}

#[test]
fn large_qubit_count() {
    // 100-qubit circuit with H on each qubit, then measure all
    let n = 100;
    let mut ops: Vec<(Gate, Vec<usize>)> = Vec::new();
    for i in 0..n {
        ops.push((Gate::H, vec![i]));
    }
    for i in 0..n {
        ops.push((Gate::Measure, vec![i]));
    }
    let mut circuit = Circuit::new(n).unwrap();
    for (gate, qubits) in &ops {
        circuit
            .add_instruction(Instruction::new(gate.clone(), qubits.clone()).unwrap())
            .unwrap();
    }
    let sim = StabilizerSimulator::new();
    let mut rng = StdRng::seed_from_u64(42);
    let start = std::time::Instant::now();
    let result = sim.run_with_rng(&circuit, &mut rng).unwrap();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 1,
        "100-qubit simulation took too long: {:?}",
        elapsed
    );
    assert_eq!(result.num_qubits, n);
    // All qubits should have measurements
    for i in 0..n {
        assert!(result.measurements[i].is_some());
    }
}

#[test]
fn deterministic_check_ket0() {
    // |0⟩ state: measurement is deterministic
    let tableau = Tableau::new(1);
    assert!(is_deterministic(&tableau, 0));
}

#[test]
fn deterministic_check_after_h() {
    // After H: measurement is non-deterministic
    let mut tableau = Tableau::new(1);
    tableau.hadamard(0);
    assert!(!is_deterministic(&tableau, 0));
}

#[test]
fn y_gate_measurement() {
    // Y|0⟩ = i|1⟩, measure → always 1
    let circuit = make_circuit(1, &[(Gate::Y, &[0]), (Gate::Measure, &[0])]);
    let sim = StabilizerSimulator::new();
    for seed in 0..20 {
        let mut rng = StdRng::seed_from_u64(seed);
        let result = sim.run_with_rng(&circuit, &mut rng).unwrap();
        assert_eq!(result.measurements[0], Some(true));
    }
}

#[test]
fn z_gate_ket0() {
    // Z|0⟩ = |0⟩, measure → always 0
    let circuit = make_circuit(1, &[(Gate::Z, &[0]), (Gate::Measure, &[0])]);
    let sim = StabilizerSimulator::new();
    for seed in 0..20 {
        let mut rng = StdRng::seed_from_u64(seed);
        let result = sim.run_with_rng(&circuit, &mut rng).unwrap();
        assert_eq!(result.measurements[0], Some(false));
    }
}
