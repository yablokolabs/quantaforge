use qf_qec_surface::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

// ── Construction ──────────────────────────────────────────────────

#[test]
fn distance3_has_9_data_qubits() {
    let code = SurfaceCode::new(3).unwrap();
    assert_eq!(code.num_data_qubits(), 9);
}

#[test]
fn distance3_stabilizer_counts() {
    let code = SurfaceCode::new(3).unwrap();
    // (d²-1)/2 = 4 of each type
    assert_eq!(code.num_x_stabilizers(), 4);
    assert_eq!(code.num_z_stabilizers(), 4);
    assert_eq!(code.num_ancilla_qubits(), 8);
}

#[test]
fn distance5_has_25_data_qubits() {
    let code = SurfaceCode::new(5).unwrap();
    assert_eq!(code.num_data_qubits(), 25);
    assert_eq!(code.num_x_stabilizers(), 12);
    assert_eq!(code.num_z_stabilizers(), 12);
    assert_eq!(code.total_qubits(), 25 + 24);
}

#[test]
fn invalid_distance_too_small() {
    assert!(SurfaceCode::new(1).is_err());
    assert!(SurfaceCode::new(2).is_err());
}

#[test]
fn invalid_distance_even() {
    assert!(SurfaceCode::new(4).is_err());
    assert!(SurfaceCode::new(6).is_err());
}

// ── Stabilizer commutativity ─────────────────────────────────────

#[test]
fn stabilizers_commute_d3() {
    stabilizers_commute(3);
}

#[test]
fn stabilizers_commute_d5() {
    stabilizers_commute(5);
}

fn stabilizers_commute(d: usize) {
    let code = SurfaceCode::new(d).unwrap();
    // Every X stabilizer must overlap with every Z stabilizer
    // on an EVEN number of qubits (so they commute).
    for xi in 0..code.num_x_stabilizers() {
        let x_sup = code.x_stabilizer_qubits(xi);
        for zi in 0..code.num_z_stabilizers() {
            let z_sup = code.z_stabilizer_qubits(zi);
            let overlap = x_sup.iter().filter(|q| z_sup.contains(q)).count();
            assert!(
                overlap % 2 == 0,
                "X-stab {xi} and Z-stab {zi} overlap on {overlap} qubits (must be even)"
            );
        }
    }
}

// ── Syndrome extraction ──────────────────────────────────────────

#[test]
fn no_errors_gives_zero_syndrome() {
    let code = SurfaceCode::new(3).unwrap();
    let n = code.num_data_qubits();
    let x_err = vec![false; n];
    let z_err = vec![false; n];
    let syn = extract_syndrome(&code, &x_err, &z_err);

    assert!(syn.x_syndrome.iter().all(|&b| !b));
    assert!(syn.z_syndrome.iter().all(|&b| !b));
}

#[test]
fn single_x_error_triggers_z_syndrome() {
    let code = SurfaceCode::new(3).unwrap();
    let n = code.num_data_qubits();
    // X error on qubit 4 (centre)
    let mut x_err = vec![false; n];
    x_err[4] = true;
    let z_err = vec![false; n];

    let syn = extract_syndrome(&code, &x_err, &z_err);
    // X syndrome should be all zero (X errors don't trigger X stabilizers)
    assert!(syn.x_syndrome.iter().all(|&b| !b));
    // At least one Z stabilizer should flag
    assert!(syn.z_syndrome.iter().any(|&b| b));
}

#[test]
fn single_z_error_triggers_x_syndrome() {
    let code = SurfaceCode::new(3).unwrap();
    let n = code.num_data_qubits();
    let x_err = vec![false; n];
    let mut z_err = vec![false; n];
    z_err[4] = true;

    let syn = extract_syndrome(&code, &x_err, &z_err);
    assert!(syn.x_syndrome.iter().any(|&b| b));
    assert!(syn.z_syndrome.iter().all(|&b| !b));
}

// ── Greedy decoder ───────────────────────────────────────────────

#[test]
fn greedy_corrects_single_x_error_d3() {
    let code = SurfaceCode::new(3).unwrap();
    let decoder = GreedyDecoder;
    let n = code.num_data_qubits();

    for q in 0..n {
        let mut x_err = vec![false; n];
        x_err[q] = true;
        let z_err = vec![false; n];
        let syn = extract_syndrome(&code, &x_err, &z_err);
        let res = decoder.decode(&code, &syn).unwrap();

        // After correction the residual should not be a logical error
        let tracker = LogicalErrorTracker::new(3);
        assert!(
            !tracker.has_logical_x_error(&x_err, &res.x_correction),
            "logical X error after correcting single X on qubit {q}"
        );
    }
}

#[test]
fn greedy_corrects_single_z_error_d3() {
    let code = SurfaceCode::new(3).unwrap();
    let decoder = GreedyDecoder;
    let n = code.num_data_qubits();

    for q in 0..n {
        let x_err = vec![false; n];
        let mut z_err = vec![false; n];
        z_err[q] = true;
        let syn = extract_syndrome(&code, &x_err, &z_err);
        let res = decoder.decode(&code, &syn).unwrap();

        let tracker = LogicalErrorTracker::new(3);
        assert!(
            !tracker.has_logical_z_error(&z_err, &res.z_correction),
            "logical Z error after correcting single Z on qubit {q}"
        );
    }
}

// ── Logical error tracker ────────────────────────────────────────

#[test]
fn no_error_no_logical_error() {
    let n = 9;
    let tracker = LogicalErrorTracker::new(3);
    let zeros = vec![false; n];
    assert!(!tracker.has_logical_x_error(&zeros, &zeros));
    assert!(!tracker.has_logical_z_error(&zeros, &zeros));
}

// ── Small experiment ─────────────────────────────────────────────

#[test]
fn low_error_rate_experiment() {
    let code = SurfaceCode::new(3).unwrap();
    let decoder = GreedyDecoder;
    let tracker = LogicalErrorTracker::new(3);
    let mut rng = StdRng::seed_from_u64(42);

    let result = tracker.run_experiment(&code, &decoder, 0.01, 200, &mut rng);

    assert_eq!(result.num_trials, 200);
    assert_eq!(result.code_distance, 3);
    // At 1% physical error rate and distance 3, logical error rate should be low
    assert!(
        result.logical_error_rate < 0.15,
        "logical error rate {} too high at p=0.01",
        result.logical_error_rate
    );
}
