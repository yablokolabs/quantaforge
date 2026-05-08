//! Performance Benchmark
//!
//! Compares state-vector and stabilizer simulator performance across
//! different qubit counts. Demonstrates the exponential scaling of
//! state-vector simulation vs. polynomial scaling of the stabilizer formalism.

use qf_circuit::CircuitBuilder;
use qf_sim_stabilizer::StabilizerSimulator;
use qf_sim_statevec::StateVectorSimulator;
use std::time::Instant;

/// Build a Clifford circuit: layer of H gates, then a chain of CNOTs, then measure all.
fn build_clifford_circuit(n: usize) -> qf_circuit::Circuit {
    let mut builder = CircuitBuilder::new(n).unwrap();

    // Apply H to all qubits
    for q in 0..n {
        builder = builder.h(q).unwrap();
    }

    // Chain of CNOTs: 0→1, 1→2, ..., (n-2)→(n-1)
    for q in 0..n - 1 {
        builder = builder.cnot(q, q + 1).unwrap();
    }

    // Measure all
    builder = builder.measure_all().unwrap();

    builder.build()
}

fn bench_statevec(sim: &StateVectorSimulator, n: usize, runs: usize) -> f64 {
    let circuit = build_clifford_circuit(n);

    let start = Instant::now();
    for _ in 0..runs {
        let _ = sim.run(&circuit).unwrap();
    }
    let elapsed = start.elapsed();

    elapsed.as_secs_f64() / runs as f64
}

fn bench_stabilizer(sim: &StabilizerSimulator, n: usize, runs: usize) -> f64 {
    let circuit = build_clifford_circuit(n);

    let start = Instant::now();
    for _ in 0..runs {
        let _ = sim.run(&circuit).unwrap();
    }
    let elapsed = start.elapsed();

    elapsed.as_secs_f64() / runs as f64
}

fn format_time(seconds: f64) -> String {
    if seconds < 1e-6 {
        format!("{:.1} ns", seconds * 1e9)
    } else if seconds < 1e-3 {
        format!("{:.1} μs", seconds * 1e6)
    } else if seconds < 1.0 {
        format!("{:.1} ms", seconds * 1e3)
    } else {
        format!("{:.2} s", seconds)
    }
}

fn main() {
    println!("=== State-Vector vs Stabilizer Benchmark ===");
    println!();
    println!("Circuit: H⊗n → CNOT chain → Measure all (Clifford only)");
    println!();

    let sv_sim = StateVectorSimulator::new();
    let stab_sim = StabilizerSimulator::new();

    // State-vector: test small qubit counts (exponential scaling)
    let sv_qubits = [4, 8, 12, 16, 20];
    let sv_runs = [1000, 500, 100, 20, 5];

    println!("State-vector simulator (exponential in qubit count):");
    println!(
        "  {:>8}  {:>12}  {:>10}  {:>14}",
        "Qubits", "Avg Time", "Runs", "Memory"
    );
    println!("  {:->8}  {:->12}  {:->10}  {:->14}", "", "", "", "");

    for (&n, &runs) in sv_qubits.iter().zip(sv_runs.iter()) {
        let avg = bench_statevec(&sv_sim, n, runs);
        let memory = (1u64 << n) * 16; // 2^n * sizeof(Complex64)
        println!(
            "  {:>8}  {:>12}  {:>10}  {:>12} KB",
            n,
            format_time(avg),
            runs,
            memory / 1024
        );
    }

    println!();

    // Stabilizer: test larger qubit counts (polynomial scaling)
    let stab_qubits = [4, 16, 64, 128, 256, 512];
    let stab_runs = [1000, 500, 100, 50, 20, 10];

    println!("Stabilizer simulator (polynomial in qubit count):");
    println!(
        "  {:>8}  {:>12}  {:>10}  {:>14}",
        "Qubits", "Avg Time", "Runs", "Tableau Size"
    );
    println!("  {:->8}  {:->12}  {:->10}  {:->14}", "", "", "", "");

    for (&n, &runs) in stab_qubits.iter().zip(stab_runs.iter()) {
        let avg = bench_stabilizer(&stab_sim, n, runs);
        // Tableau is O(n^2) bits ≈ n^2 / 8 bytes
        let tableau_size = (n * n) / 8;
        println!(
            "  {:>8}  {:>12}  {:>10}  {:>12} KB",
            n,
            format_time(avg),
            runs,
            if tableau_size > 1024 {
                format!("{}", tableau_size / 1024)
            } else {
                format!("{:.1}", tableau_size as f64 / 1024.0)
            }
        );
    }

    println!();
    println!("Key takeaway:");
    println!("  State-vector: exact, universal gate set, but ~2x slower per added qubit");
    println!("  Stabilizer:   Clifford-only, but handles 500+ qubits easily");
    println!();
    println!("Choose state-vector when you need T gates, Rx/Ry/Rz, or exact amplitudes.");
    println!("Choose stabilizer when your circuit is Clifford-only (QEC, entanglement tests).");
}
