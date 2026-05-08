use qf_math::{Complex64, StateVector};
use qf_noise::{
    AmplitudeDampingChannel, DensityMatrix, DepolarizingChannel, NoiseChannel, NoiseModel,
    PhaseDampingChannel, ReadoutNoise,
};
use rand::SeedableRng;

const EPS: f64 = 1e-10;

// ── helpers ──────────────────────────────────────────────────────────────────

fn state_zero() -> StateVector {
    StateVector::new(1) // |0⟩
}

fn state_one() -> StateVector {
    let mut sv = StateVector::new(1);
    sv.set_amplitude(0, Complex64::new(0.0, 0.0)).unwrap();
    sv.set_amplitude(1, Complex64::new(1.0, 0.0)).unwrap();
    sv
}

fn state_plus() -> StateVector {
    let s = 1.0 / 2.0_f64.sqrt();
    StateVector::from_data(vec![Complex64::new(s, 0.0), Complex64::new(s, 0.0)]).unwrap()
}

fn assert_trace_one(rho: &DensityMatrix) {
    let tr = rho.trace();
    assert!(
        (tr.re - 1.0).abs() < 1e-8 && tr.im.abs() < 1e-8,
        "Trace should be 1, got {tr}"
    );
}

// ── Depolarizing ─────────────────────────────────────────────────────────────

#[test]
fn depolarizing_p0_identity() {
    let ch = DepolarizingChannel::new(0.0).unwrap();
    let sv = state_plus();
    let mut rho = DensityMatrix::from_statevector(&sv);
    let before = rho.matrix().get(0, 1);
    ch.apply(&mut rho, 0).unwrap();
    let after = rho.matrix().get(0, 1);
    assert!(
        (before - after).norm() < EPS,
        "p=0 should leave state unchanged"
    );
    assert_trace_one(&rho);
}

#[test]
fn depolarizing_p1_maximally_mixed() {
    let ch = DepolarizingChannel::new(1.0).unwrap();
    let sv = state_zero();
    let mut rho = DensityMatrix::from_statevector(&sv);
    ch.apply(&mut rho, 0).unwrap();
    // maximally mixed state: ρ = I/2
    assert!(
        (rho.matrix().get(0, 0).re - 0.5).abs() < EPS,
        "ρ[0,0] should be 0.5"
    );
    assert!(
        (rho.matrix().get(1, 1).re - 0.5).abs() < EPS,
        "ρ[1,1] should be 0.5"
    );
    assert!(
        rho.matrix().get(0, 1).norm() < EPS,
        "off-diagonals should be 0"
    );
    assert_trace_one(&rho);
}

// ── Amplitude damping ────────────────────────────────────────────────────────

#[test]
fn amplitude_damping_one_drives_to_ground() {
    let ch = AmplitudeDampingChannel::new(1.0).unwrap();
    let sv = state_one();
    let mut rho = DensityMatrix::from_statevector(&sv);
    ch.apply(&mut rho, 0).unwrap();
    // |1⟩ → |0⟩
    assert!(
        (rho.matrix().get(0, 0).re - 1.0).abs() < EPS,
        "should be in |0⟩"
    );
    assert!(
        rho.matrix().get(1, 1).re.abs() < EPS,
        "|1⟩ population should be 0"
    );
    assert_trace_one(&rho);
}

#[test]
fn amplitude_damping_preserves_ground() {
    let ch = AmplitudeDampingChannel::new(0.7).unwrap();
    let sv = state_zero();
    let mut rho = DensityMatrix::from_statevector(&sv);
    ch.apply(&mut rho, 0).unwrap();
    assert!(
        (rho.matrix().get(0, 0).re - 1.0).abs() < EPS,
        "|0⟩ should remain |0⟩"
    );
    assert_trace_one(&rho);
}

// ── Phase damping ────────────────────────────────────────────────────────────

#[test]
fn phase_damping_preserves_populations() {
    let ch = PhaseDampingChannel::new(0.5).unwrap();
    let sv = state_plus();
    let mut rho = DensityMatrix::from_statevector(&sv);
    let diag0_before = rho.matrix().get(0, 0);
    let diag1_before = rho.matrix().get(1, 1);
    ch.apply(&mut rho, 0).unwrap();
    assert!(
        (rho.matrix().get(0, 0) - diag0_before).norm() < EPS,
        "diagonal unchanged"
    );
    assert!(
        (rho.matrix().get(1, 1) - diag1_before).norm() < EPS,
        "diagonal unchanged"
    );
    assert_trace_one(&rho);
}

#[test]
fn phase_damping_kills_coherences() {
    let ch = PhaseDampingChannel::new(1.0).unwrap();
    let sv = state_plus();
    let mut rho = DensityMatrix::from_statevector(&sv);
    ch.apply(&mut rho, 0).unwrap();
    assert!(
        rho.matrix().get(0, 1).norm() < EPS,
        "off-diagonal should vanish for γ=1"
    );
    assert!(
        rho.matrix().get(1, 0).norm() < EPS,
        "off-diagonal should vanish for γ=1"
    );
    assert_trace_one(&rho);
}

// ── Readout noise ────────────────────────────────────────────────────────────

#[test]
fn readout_noise_p0_no_flip() {
    let rn = ReadoutNoise::new(0.0, 0.0).unwrap();
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    for _ in 0..100 {
        assert!(!rn.apply_to_measurement(false, &mut rng));
        assert!(rn.apply_to_measurement(true, &mut rng));
    }
}

#[test]
fn readout_symmetric_half_is_random() {
    let rn = ReadoutNoise::symmetric(0.5).unwrap();
    let mut rng = rand::rngs::StdRng::seed_from_u64(123);
    let mut ones = 0u32;
    let n = 10_000;
    for _ in 0..n {
        if rn.apply_to_measurement(false, &mut rng) {
            ones += 1;
        }
    }
    let ratio = ones as f64 / n as f64;
    assert!(
        (ratio - 0.5).abs() < 0.05,
        "symmetric(0.5) should give ~50% flips, got {ratio}"
    );
}

// ── Trace preservation ──────────────────────────────────────────────────────

#[test]
fn all_channels_preserve_trace() {
    let channels: Vec<Box<dyn NoiseChannel>> = vec![
        Box::new(DepolarizingChannel::new(0.3).unwrap()),
        Box::new(AmplitudeDampingChannel::new(0.4).unwrap()),
        Box::new(PhaseDampingChannel::new(0.6).unwrap()),
    ];

    let sv = state_plus();
    for ch in &channels {
        let mut rho = DensityMatrix::from_statevector(&sv);
        ch.apply(&mut rho, 0).unwrap();
        assert_trace_one(&rho);
    }
}

// ── NoiseModel serialization ────────────────────────────────────────────────

#[test]
fn noise_model_serde_roundtrip() {
    let model = NoiseModel::with_depolarizing(0.05).unwrap();
    let json = serde_json::to_string(&model).unwrap();
    let restored: NoiseModel = serde_json::from_str(&json).unwrap();
    // gate_noise should survive round-trip
    assert!(restored.gate_noise.is_some());
    assert!(restored.readout_noise.is_none());
}

// ── Invalid parameter ───────────────────────────────────────────────────────

#[test]
fn invalid_parameter_errors() {
    assert!(DepolarizingChannel::new(1.5).is_err());
    assert!(DepolarizingChannel::new(-0.1).is_err());
    assert!(AmplitudeDampingChannel::new(2.0).is_err());
    assert!(PhaseDampingChannel::new(-0.01).is_err());
    assert!(ReadoutNoise::new(0.0, 1.1).is_err());
}

// ── Purity ──────────────────────────────────────────────────────────────────

#[test]
fn pure_state_has_purity_one() {
    let rho = DensityMatrix::from_statevector(&state_zero());
    assert!(rho.is_pure(1e-8));
}

#[test]
fn depolarized_state_has_lower_purity() {
    let ch = DepolarizingChannel::new(0.5).unwrap();
    let mut rho = DensityMatrix::from_statevector(&state_zero());
    ch.apply(&mut rho, 0).unwrap();
    assert!(rho.purity() < 1.0 - 1e-8, "purity should be < 1");
}

// ── Multi-qubit ─────────────────────────────────────────────────────────────

#[test]
fn depolarizing_on_two_qubit_system() {
    let sv = StateVector::new(2); // |00⟩
    let mut rho = DensityMatrix::from_statevector(&sv);
    let ch = DepolarizingChannel::new(0.3).unwrap();

    // Apply to qubit 0
    ch.apply(&mut rho, 0).unwrap();
    assert_trace_one(&rho);

    // Apply to qubit 1
    ch.apply(&mut rho, 1).unwrap();
    assert_trace_one(&rho);
}
