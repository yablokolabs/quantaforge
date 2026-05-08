//! Surface Code Error Correction Example
//!
//! Runs a Monte Carlo experiment to estimate the logical error rate of a
//! distance-3 surface code under depolarizing noise with a greedy decoder.

use qf_qec_surface::{GreedyDecoder, LogicalErrorTracker, SurfaceCode};
use rand::SeedableRng;

fn main() {
    println!("=== Surface Code Error Correction ===");
    println!();

    let distance = 3;
    let code = SurfaceCode::new(distance).unwrap();

    println!("Surface code parameters:");
    println!("  Distance:         {}", distance);
    println!("  Data qubits:      {}", code.num_data_qubits());
    println!("  Ancilla qubits:   {}", code.num_ancilla_qubits());
    println!("  Total qubits:     {}", code.total_qubits());
    println!("  X stabilizers:    {}", code.num_x_stabilizers());
    println!("  Z stabilizers:    {}", code.num_z_stabilizers());
    println!();

    let decoder = GreedyDecoder;
    let tracker = LogicalErrorTracker::new(distance);
    let num_trials = 10_000;

    println!(
        "Running Monte Carlo experiments ({} trials each)...",
        num_trials
    );
    println!();
    println!(
        "  {:>12}  {:>14}  {:>14}  {:>10}",
        "Phys. Error", "Logical Errors", "Logical Rate", "Suppressed"
    );
    println!("  {:->12}  {:->14}  {:->14}  {:->10}", "", "", "", "");

    let error_rates = [0.001, 0.005, 0.01, 0.02, 0.05, 0.1];

    for &physical_error_rate in &error_rates {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);

        let result =
            tracker.run_experiment(&code, &decoder, physical_error_rate, num_trials, &mut rng);

        let suppressed = if result.logical_error_rate > 0.0 {
            physical_error_rate / result.logical_error_rate
        } else {
            f64::INFINITY
        };

        println!(
            "  {:>12.4}  {:>14}  {:>14.6}  {:>10.1}x",
            physical_error_rate, result.logical_error_count, result.logical_error_rate, suppressed,
        );
    }

    println!();
    println!("Interpretation:");
    println!("  - Below the threshold (~1%), the surface code suppresses errors");
    println!("  - Above the threshold, error correction does more harm than good");
    println!("  - The greedy decoder is a heuristic; MWPM would give better results");
    println!(
        "  - Higher distance codes have lower logical error rates (at the cost of more qubits)"
    );
    println!();

    // Show what happens with distance 5
    let d5 = 5;
    let code5 = SurfaceCode::new(d5).unwrap();
    let tracker5 = LogicalErrorTracker::new(d5);
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    let result5 = tracker5.run_experiment(&code5, &decoder, 0.01, num_trials, &mut rng);

    println!("Distance-5 comparison at p=0.01:");
    println!(
        "  d=5: {} data qubits, logical error rate = {:.6}",
        code5.num_data_qubits(),
        result5.logical_error_rate
    );
}
