//! GHZ State Example
//!
//! Creates a 3-qubit GHZ state |GHZ⟩ = (|000⟩ + |111⟩)/√2.
//! This is the maximally entangled state of three qubits.

use qf_circuit::CircuitBuilder;
use qf_sim_statevec::StateVectorSimulator;

fn main() {
    // Build the GHZ circuit: H on qubit 0, then CNOT(0,1), CNOT(0,2)
    let circuit = CircuitBuilder::new(3)
        .unwrap()
        .h(0)
        .unwrap()
        .cnot(0, 1)
        .unwrap()
        .cnot(0, 2)
        .unwrap()
        .measure(0)
        .unwrap()
        .measure(1)
        .unwrap()
        .measure(2)
        .unwrap()
        .build();

    println!("=== 3-Qubit GHZ State ===");
    println!();
    println!("Circuit:");
    println!("  Qubits: {}", circuit.num_qubits());
    println!("  Gates:  {}", circuit.gate_count());
    println!("  Depth:  {}", circuit.depth());
    println!("  Clifford-only: {}", circuit.is_clifford_only());
    println!();

    let sim = StateVectorSimulator::new();

    // Show state vector amplitudes
    let single = sim.run(&circuit).unwrap();
    println!("State vector probabilities:");
    for (i, p) in single.state.probabilities().iter().enumerate() {
        if *p > 1e-10 {
            println!("  |{:03b}⟩: {:.4}", i, p);
        }
    }
    println!();

    // Run shots
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
    println!("Expected: ~50% |000⟩, ~50% |111⟩");
    println!("All three qubits are perfectly correlated in the GHZ state.");

    // Also demonstrate the stabilizer simulator on this Clifford circuit
    println!();
    println!("--- Stabilizer simulator comparison ---");
    let stab_sim = qf_sim_stabilizer::StabilizerSimulator::new();
    let stab_result = stab_sim.run(&circuit).unwrap();
    println!("Stabilizer measurements: {:?}", stab_result.measurements);
    println!("(Single-shot only; stabilizer simulator gives one sample per run)");
}
