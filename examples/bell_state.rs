//! Bell State Example
//!
//! Creates a Bell state |Φ+⟩ = (|00⟩ + |11⟩)/√2 and measures it.
//! Expected output: ~50% |00⟩ and ~50% |11⟩, demonstrating perfect correlations.

use qf_circuit::CircuitBuilder;
use qf_sim_statevec::StateVectorSimulator;

fn main() {
    // Build the Bell state circuit: H on qubit 0, then CNOT(0,1), then measure both
    let circuit = CircuitBuilder::new(2)
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

    println!("=== Bell State |Φ+⟩ ===");
    println!();
    println!("Circuit:");
    println!("  Qubits: {}", circuit.num_qubits());
    println!("  Gates:  {}", circuit.gate_count());
    println!("  Depth:  {}", circuit.depth());
    println!();

    // Run a single simulation to inspect the state vector before measurement
    let sim = StateVectorSimulator::new();
    let single = sim.run(&circuit).unwrap();
    println!("State vector probabilities:");
    for (i, p) in single.state.probabilities().iter().enumerate() {
        if *p > 1e-10 {
            println!("  |{:02b}⟩: {:.4}", i, p);
        }
    }
    println!();

    // Run 1000 shots and show the measurement histogram
    let shots = 1000;
    let result = sim.run_shots(&circuit, shots).unwrap();

    println!("Measurement results ({} shots):", shots);
    let mut counts: Vec<_> = result.counts.iter().collect();
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
    println!("Expected: ~50% |00⟩, ~50% |11⟩ (Bell state correlations)");
    println!("The qubits are perfectly correlated — measuring one determines the other.");
}
