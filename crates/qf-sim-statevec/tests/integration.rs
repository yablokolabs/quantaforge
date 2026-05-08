use qf_circuit::{Circuit, Gate, Instruction};
use qf_math::complex::approx_eq;
use qf_math::Complex64;
use qf_sim_statevec::{SimError, StateVectorSimulator};
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::f64::consts::PI;

const EPS: f64 = 1e-10;

fn seeded_rng() -> StdRng {
    StdRng::seed_from_u64(42)
}

#[test]
fn test_h_on_zero_gives_plus() {
    let mut circuit = Circuit::new(1).unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::H, vec![0]).unwrap())
        .unwrap();

    let sim = StateVectorSimulator::new();
    let mut rng = seeded_rng();
    let result = sim.run_with_rng(&circuit, &mut rng).unwrap();

    let inv_sqrt2 = 1.0 / 2.0_f64.sqrt();
    let sv = &result.state.state_vector;
    assert!(approx_eq(
        sv.amplitude(0).unwrap(),
        Complex64::new(inv_sqrt2, 0.0),
        EPS
    ));
    assert!(approx_eq(
        sv.amplitude(1).unwrap(),
        Complex64::new(inv_sqrt2, 0.0),
        EPS
    ));
}

#[test]
fn test_x_on_zero_gives_one() {
    let mut circuit = Circuit::new(1).unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::X, vec![0]).unwrap())
        .unwrap();

    let sim = StateVectorSimulator::new();
    let mut rng = seeded_rng();
    let result = sim.run_with_rng(&circuit, &mut rng).unwrap();

    let sv = &result.state.state_vector;
    assert!(approx_eq(
        sv.amplitude(0).unwrap(),
        Complex64::new(0.0, 0.0),
        EPS
    ));
    assert!(approx_eq(
        sv.amplitude(1).unwrap(),
        Complex64::new(1.0, 0.0),
        EPS
    ));
}

#[test]
fn test_cnot_10_gives_11() {
    // Start with |00⟩, apply X to qubit 1 to get |10⟩, then CNOT(1→0) to get |11⟩
    let mut circuit = Circuit::new(2).unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::X, vec![1]).unwrap())
        .unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::CNOT, vec![1, 0]).unwrap())
        .unwrap();

    let sim = StateVectorSimulator::new();
    let mut rng = seeded_rng();
    let result = sim.run_with_rng(&circuit, &mut rng).unwrap();

    let sv = &result.state.state_vector;
    // |11⟩ = index 3
    assert!(approx_eq(
        sv.amplitude(0).unwrap(),
        Complex64::new(0.0, 0.0),
        EPS
    ));
    assert!(approx_eq(
        sv.amplitude(1).unwrap(),
        Complex64::new(0.0, 0.0),
        EPS
    ));
    assert!(approx_eq(
        sv.amplitude(2).unwrap(),
        Complex64::new(0.0, 0.0),
        EPS
    ));
    assert!(approx_eq(
        sv.amplitude(3).unwrap(),
        Complex64::new(1.0, 0.0),
        EPS
    ));
}

#[test]
fn test_bell_state() {
    // H on q0, CNOT q0→q1 → (|00⟩ + |11⟩)/√2
    let mut circuit = Circuit::new(2).unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::H, vec![0]).unwrap())
        .unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::CNOT, vec![0, 1]).unwrap())
        .unwrap();

    let sim = StateVectorSimulator::new();
    let mut rng = seeded_rng();
    let result = sim.run_with_rng(&circuit, &mut rng).unwrap();

    let inv_sqrt2 = 1.0 / 2.0_f64.sqrt();
    let sv = &result.state.state_vector;
    assert!(approx_eq(
        sv.amplitude(0).unwrap(),
        Complex64::new(inv_sqrt2, 0.0),
        EPS
    ));
    assert!(approx_eq(
        sv.amplitude(1).unwrap(),
        Complex64::new(0.0, 0.0),
        EPS
    ));
    assert!(approx_eq(
        sv.amplitude(2).unwrap(),
        Complex64::new(0.0, 0.0),
        EPS
    ));
    assert!(approx_eq(
        sv.amplitude(3).unwrap(),
        Complex64::new(inv_sqrt2, 0.0),
        EPS
    ));
}

#[test]
fn test_identity_circuit() {
    let circuit = Circuit::new(2).unwrap();
    let sim = StateVectorSimulator::new();
    let mut rng = seeded_rng();
    let result = sim.run_with_rng(&circuit, &mut rng).unwrap();

    let sv = &result.state.state_vector;
    assert!(approx_eq(
        sv.amplitude(0).unwrap(),
        Complex64::new(1.0, 0.0),
        EPS
    ));
    assert!(approx_eq(
        sv.amplitude(1).unwrap(),
        Complex64::new(0.0, 0.0),
        EPS
    ));
    assert!(approx_eq(
        sv.amplitude(2).unwrap(),
        Complex64::new(0.0, 0.0),
        EPS
    ));
    assert!(approx_eq(
        sv.amplitude(3).unwrap(),
        Complex64::new(0.0, 0.0),
        EPS
    ));
}

#[test]
fn test_3_qubit_ghz() {
    // H on q0, CNOT q0→q1, CNOT q0→q2 → (|000⟩ + |111⟩)/√2
    let mut circuit = Circuit::new(3).unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::H, vec![0]).unwrap())
        .unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::CNOT, vec![0, 1]).unwrap())
        .unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::CNOT, vec![0, 2]).unwrap())
        .unwrap();

    let sim = StateVectorSimulator::new();
    let mut rng = seeded_rng();
    let result = sim.run_with_rng(&circuit, &mut rng).unwrap();

    let inv_sqrt2 = 1.0 / 2.0_f64.sqrt();
    let sv = &result.state.state_vector;
    assert!(approx_eq(
        sv.amplitude(0).unwrap(),
        Complex64::new(inv_sqrt2, 0.0),
        EPS
    ));
    for i in 1..7 {
        assert!(approx_eq(
            sv.amplitude(i).unwrap(),
            Complex64::new(0.0, 0.0),
            EPS
        ));
    }
    assert!(approx_eq(
        sv.amplitude(7).unwrap(),
        Complex64::new(inv_sqrt2, 0.0),
        EPS
    ));
}

#[test]
fn test_measurement_collapses_state() {
    // H on q0, then measure → should collapse to |0⟩ or |1⟩
    let mut circuit = Circuit::new(1).unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::H, vec![0]).unwrap())
        .unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::Measure, vec![0]).unwrap())
        .unwrap();

    let sim = StateVectorSimulator::new();
    let mut rng = seeded_rng();
    let result = sim.run_with_rng(&circuit, &mut rng).unwrap();

    let sv = &result.state.state_vector;
    let a0 = sv.amplitude(0).unwrap();
    let a1 = sv.amplitude(1).unwrap();

    // One amplitude must be ~1 and the other ~0
    let p0 = a0.norm_sqr();
    let p1 = a1.norm_sqr();
    assert!(
        (p0 - 1.0).abs() < EPS || (p1 - 1.0).abs() < EPS,
        "State should have collapsed: p0={p0}, p1={p1}"
    );

    // Measurement result should be stored
    let meas = result.state.get_measurement(0);
    assert!(meas.is_some());
}

#[test]
fn test_shot_statistics_bell() {
    // Bell circuit with measurements
    let mut circuit = Circuit::new(2).unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::H, vec![0]).unwrap())
        .unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::CNOT, vec![0, 1]).unwrap())
        .unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::Measure, vec![0]).unwrap())
        .unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::Measure, vec![1]).unwrap())
        .unwrap();

    let sim = StateVectorSimulator::new();
    let result = sim.run_shots(&circuit, 1000).unwrap();

    let count_00 = result.counts.get("00").copied().unwrap_or(0);
    let count_11 = result.counts.get("11").copied().unwrap_or(0);
    let count_01 = result.counts.get("01").copied().unwrap_or(0);
    let count_10 = result.counts.get("10").copied().unwrap_or(0);

    // Bell state should only give "00" and "11"
    assert_eq!(count_01, 0, "Should not see |01⟩ in Bell state");
    assert_eq!(count_10, 0, "Should not see |10⟩ in Bell state");
    assert!(
        count_00 >= 400 && count_00 <= 600,
        "Expected ~500 for |00⟩, got {count_00}"
    );
    assert!(
        count_11 >= 400 && count_11 <= 600,
        "Expected ~500 for |11⟩, got {count_11}"
    );
    assert_eq!(count_00 + count_11, 1000);
}

#[test]
fn test_multiple_measurements() {
    // Create a 2-qubit circuit, measure each qubit independently
    let mut circuit = Circuit::new(2).unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::X, vec![0]).unwrap())
        .unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::Measure, vec![0]).unwrap())
        .unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::Measure, vec![1]).unwrap())
        .unwrap();

    let sim = StateVectorSimulator::new();
    let mut rng = seeded_rng();
    let result = sim.run_with_rng(&circuit, &mut rng).unwrap();

    // q0 was flipped to |1⟩, q1 stays |0⟩
    assert_eq!(result.state.get_measurement(0), Some(true));
    assert_eq!(result.state.get_measurement(1), Some(false));
}

#[test]
fn test_rx_pi_equals_minus_i_x() {
    // Rx(π)|0⟩ = -i|1⟩
    let mut circuit = Circuit::new(1).unwrap();
    circuit
        .add_instruction(Instruction::new(Gate::Rx(PI), vec![0]).unwrap())
        .unwrap();

    let sim = StateVectorSimulator::new();
    let mut rng = seeded_rng();
    let result = sim.run_with_rng(&circuit, &mut rng).unwrap();

    let sv = &result.state.state_vector;
    assert!(approx_eq(
        sv.amplitude(0).unwrap(),
        Complex64::new(0.0, 0.0),
        EPS
    ));
    assert!(approx_eq(
        sv.amplitude(1).unwrap(),
        Complex64::new(0.0, -1.0),
        EPS
    ));
}

#[test]
fn test_too_many_qubits_error() {
    let circuit = Circuit::new(31).unwrap();
    let sim = StateVectorSimulator::new();
    let result = sim.run(&circuit);
    assert!(result.is_err());
    match result.unwrap_err() {
        SimError::TooManyQubits(n, max) => {
            assert_eq!(n, 31);
            assert_eq!(max, 30);
        }
        e => panic!("Expected TooManyQubits error, got: {e:?}"),
    }
}
