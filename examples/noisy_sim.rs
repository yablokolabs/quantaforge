//! Noisy Simulation Example
//!
//! Demonstrates the effect of depolarizing noise on a quantum circuit.
//! Compares ideal state-vector simulation against a noisy density matrix.

use qf_circuit::CircuitBuilder;
use qf_math::StateVector;
use qf_noise::{DensityMatrix, DepolarizingChannel, NoiseChannel};
use qf_sim_statevec::StateVectorSimulator;

fn main() {
    println!("=== Noisy Simulation: Ideal vs Depolarizing Noise ===");
    println!();

    // Build a simple circuit: put qubit 0 in superposition
    let circuit = CircuitBuilder::new(1).unwrap().h(0).unwrap().build();

    // --- Ideal simulation ---
    let sim = StateVectorSimulator::new();
    let ideal_result = sim.run(&circuit).unwrap();
    let ideal_probs = ideal_result.state.probabilities();

    println!("Circuit: H|0⟩ → |+⟩ = (|0⟩ + |1⟩)/√2");
    println!();
    println!("Ideal state probabilities:");
    println!("  P(|0⟩) = {:.4}", ideal_probs[0]);
    println!("  P(|1⟩) = {:.4}", ideal_probs[1]);
    println!();

    // --- Noisy simulation using density matrix + Kraus operators ---
    println!("Applying depolarizing noise at various error rates:");
    println!();
    println!(
        "  {:>8}  {:>8}  {:>8}  {:>8}",
        "Error p", "P(|0⟩)", "P(|1⟩)", "Purity"
    );
    println!("  {:->8}  {:->8}  {:->8}  {:->8}", "", "", "", "");

    for &p in &[0.0, 0.01, 0.05, 0.1, 0.2, 0.5, 1.0] {
        // Start from the ideal |+⟩ state
        let sv = {
            let mut sv = StateVector::new(1);
            let sqrt2_inv = 1.0 / f64::sqrt(2.0);
            let c = qf_math::Complex64::new(sqrt2_inv, 0.0);
            sv.set_amplitude(0, c).unwrap();
            sv.set_amplitude(1, c).unwrap();
            sv
        };

        let mut rho = DensityMatrix::from_statevector(&sv);

        // Apply depolarizing channel
        if p > 0.0 {
            let channel = DepolarizingChannel::new(p).unwrap();
            channel.apply(&mut rho, 0).unwrap();
        }

        let matrix = rho.matrix();
        let p0 = matrix.get(0, 0).re;
        let p1 = matrix.get(1, 1).re;
        let purity = rho.purity();

        println!("  {:>8.3}  {:>8.4}  {:>8.4}  {:>8.4}", p, p0, p1, purity);
    }

    println!();
    println!("Notes:");
    println!("  - Purity = 1.0 means a pure state (no noise)");
    println!("  - Purity = 0.5 means the maximally mixed state (1 qubit)");
    println!("  - Depolarizing noise: ρ → (1-p)ρ + p·I/2");
    println!("  - At p=1.0, the state is completely depolarized (maximally mixed)");

    // --- Multi-qubit noisy example ---
    println!();
    println!("--- Two-qubit Bell state with noise ---");
    println!();

    let bell_circuit = CircuitBuilder::new(2)
        .unwrap()
        .h(0)
        .unwrap()
        .cnot(0, 1)
        .unwrap()
        .measure(0)
        .unwrap()
        .measure(1)
        .unwrap()
        .build();

    let shots = 2000;
    let ideal_shots = sim.run_shots(&bell_circuit, shots).unwrap();
    println!("Ideal Bell state ({} shots):", shots);
    let mut counts: Vec<_> = ideal_shots.counts.iter().collect();
    counts.sort_by_key(|(k, _)| (*k).clone());
    for (state, count) in &counts {
        println!(
            "  |{}⟩: {} ({:.1}%)",
            state,
            count,
            *(*count) as f64 / shots as f64 * 100.0
        );
    }
    println!();
    println!("Note: For multi-qubit noisy simulation, apply noise channels to the");
    println!("density matrix of each qubit independently using the Kraus formalism.");
}
