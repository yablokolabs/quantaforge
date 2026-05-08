use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use qf_circuit::serialization;
use qf_circuit::{CircuitBuilder, Gate};
use qf_scheduler::ResourceEstimate;
use qf_sim_statevec::StateVectorSimulator;

#[derive(Parser)]
#[command(name = "qf", about = "QuantaForge quantum simulation CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create and manage circuits
    Circuit {
        #[command(subcommand)]
        action: CircuitAction,
    },
    /// Run simulations
    Sim {
        #[command(subcommand)]
        action: SimAction,
    },
    /// Error correction experiments
    Qec {
        #[command(subcommand)]
        action: QecAction,
    },
    /// Resource estimation
    Estimate {
        /// Circuit file (JSON)
        #[arg(long)]
        circuit: String,
    },
}

#[derive(Subcommand)]
enum CircuitAction {
    /// Create a circuit from text format
    Create {
        #[arg(long)]
        qubits: usize,
        /// Semicolon-separated gates: "H 0; CNOT 0 1"
        #[arg(long)]
        gates: String,
        /// Output file (default: stdout)
        #[arg(long)]
        output: Option<String>,
    },
    /// Show circuit info
    Info {
        /// JSON file path
        #[arg(long)]
        circuit: String,
    },
}

#[derive(Subcommand)]
enum SimAction {
    /// Run a state-vector simulation
    Run {
        #[arg(long)]
        circuit: String,
        #[arg(long, default_value = "1024")]
        shots: usize,
        #[arg(long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
enum QecAction {
    /// Run a surface code experiment
    Surface {
        #[arg(long, default_value = "3")]
        distance: usize,
        #[arg(long, default_value = "0.01")]
        error_rate: f64,
        #[arg(long, default_value = "1000")]
        trials: usize,
    },
}

fn parse_gate_str(token: &str) -> Result<(Gate, Vec<usize>)> {
    let parts: Vec<&str> = token.split_whitespace().collect();
    if parts.is_empty() {
        anyhow::bail!("Empty gate specification");
    }
    let name = parts[0].to_uppercase();
    match name.as_str() {
        "H" | "X" | "Y" | "Z" | "S" | "T" | "MEASURE" => {
            let q: usize = parts
                .get(1)
                .context("Missing qubit index")?
                .parse()
                .context("Invalid qubit index")?;
            let gate = match name.as_str() {
                "H" => Gate::H,
                "X" => Gate::X,
                "Y" => Gate::Y,
                "Z" => Gate::Z,
                "S" => Gate::S,
                "T" => Gate::T,
                "MEASURE" => Gate::Measure,
                _ => unreachable!(),
            };
            Ok((gate, vec![q]))
        }
        "RX" | "RY" | "RZ" => {
            let q: usize = parts
                .get(1)
                .context("Missing qubit index")?
                .parse()
                .context("Invalid qubit index")?;
            let theta: f64 = parts
                .get(2)
                .context("Missing angle parameter")?
                .parse()
                .context("Invalid angle")?;
            let gate = match name.as_str() {
                "RX" => Gate::Rx(theta),
                "RY" => Gate::Ry(theta),
                "RZ" => Gate::Rz(theta),
                _ => unreachable!(),
            };
            Ok((gate, vec![q]))
        }
        "CNOT" | "CX" => {
            let c: usize = parts
                .get(1)
                .context("Missing control qubit")?
                .parse()
                .context("Invalid control qubit")?;
            let t: usize = parts
                .get(2)
                .context("Missing target qubit")?
                .parse()
                .context("Invalid target qubit")?;
            Ok((Gate::CNOT, vec![c, t]))
        }
        "CZ" => {
            let q1: usize = parts
                .get(1)
                .context("Missing first qubit")?
                .parse()
                .context("Invalid qubit")?;
            let q2: usize = parts
                .get(2)
                .context("Missing second qubit")?
                .parse()
                .context("Invalid qubit")?;
            Ok((Gate::CZ, vec![q1, q2]))
        }
        other => anyhow::bail!("Unknown gate: {other}"),
    }
}

fn add_to_builder(
    builder: CircuitBuilder,
    gate: &Gate,
    qubits: &[usize],
) -> Result<CircuitBuilder> {
    let b = match gate {
        Gate::H => builder.h(qubits[0])?,
        Gate::X => builder.x(qubits[0])?,
        Gate::Y => builder.y(qubits[0])?,
        Gate::Z => builder.z(qubits[0])?,
        Gate::S => builder.s(qubits[0])?,
        Gate::T => builder.t(qubits[0])?,
        Gate::Rx(t) => builder.rx(qubits[0], *t)?,
        Gate::Ry(t) => builder.ry(qubits[0], *t)?,
        Gate::Rz(t) => builder.rz(qubits[0], *t)?,
        Gate::CNOT => builder.cnot(qubits[0], qubits[1])?,
        Gate::CZ => builder.cz(qubits[0], qubits[1])?,
        Gate::Measure => builder.measure(qubits[0])?,
    };
    Ok(b)
}

fn cmd_circuit_create(qubits: usize, gates: &str, output: &Option<String>) -> Result<()> {
    let mut builder = CircuitBuilder::new(qubits)?;
    for token in gates.split(';') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        let (gate, qubit_indices) = parse_gate_str(token)?;
        builder = add_to_builder(builder, &gate, &qubit_indices)?;
    }
    let circuit = builder.build();
    let json = serialization::to_json(&circuit)?;

    if let Some(path) = output {
        std::fs::write(path, &json).context("Failed to write output file")?;
        println!("Circuit written to {path}");
    } else {
        println!("{json}");
    }
    Ok(())
}

fn cmd_circuit_info(path: &str) -> Result<()> {
    let json = std::fs::read_to_string(path).context("Failed to read circuit file")?;
    let circuit = serialization::from_json(&json)?;
    println!("Qubits:        {}", circuit.num_qubits());
    println!("Gate count:    {}", circuit.gate_count());
    println!("Depth:         {}", circuit.depth());
    println!("Clifford only: {}", circuit.is_clifford_only());
    println!("Measurements:  {}", circuit.has_measurements());
    Ok(())
}

fn cmd_sim_run(path: &str, shots: usize, output: &Option<String>) -> Result<()> {
    let json = std::fs::read_to_string(path).context("Failed to read circuit file")?;
    let circuit = serialization::from_json(&json)?;

    // Ensure circuit has measurements
    let circuit = if !circuit.has_measurements() {
        let mut builder = CircuitBuilder::new(circuit.num_qubits())?;
        for instr in circuit.instructions() {
            builder = add_to_builder(builder, &instr.gate, &instr.qubits)?;
        }
        builder = builder.measure_all()?;
        builder.build()
    } else {
        circuit
    };

    let sim = StateVectorSimulator::new();
    let result = sim.run_shots(&circuit, shots)?;

    let result_json = serde_json::to_string_pretty(&result.counts)?;

    if let Some(out_path) = output {
        std::fs::write(out_path, &result_json).context("Failed to write output")?;
        println!("Results written to {out_path}");
    } else {
        println!("Shots: {}", result.num_shots);
        println!("Qubits: {}", result.num_qubits);
        println!("Counts:");
        let mut sorted: Vec<_> = result.counts.iter().collect();
        sorted.sort_by_key(|(a, _)| *a);
        for (state, count) in &sorted {
            println!("  |{state}⟩: {count}");
        }
    }
    Ok(())
}

fn cmd_qec_surface(distance: usize, error_rate: f64, trials: usize) -> Result<()> {
    // qf-qec-surface is still a stub, so provide placeholder output
    println!("QEC Surface Code Experiment");
    println!("  Distance:   {distance}");
    println!("  Error rate: {error_rate}");
    println!("  Trials:     {trials}");
    println!();
    println!("  (qf-qec-surface is not yet fully implemented)");
    println!(
        "  Placeholder result: logical error rate ≈ {:.2e}",
        error_rate.powi(distance.div_ceil(2) as i32)
    );
    Ok(())
}

fn cmd_estimate(path: &str) -> Result<()> {
    let json = std::fs::read_to_string(path).context("Failed to read circuit file")?;
    let circuit = serialization::from_json(&json)?;
    let estimate = ResourceEstimate::from_circuit(&circuit);
    println!("Resource Estimate");
    println!("  Qubits:              {}", estimate.num_qubits);
    println!("  Gate count:          {}", estimate.gate_count);
    println!("  Circuit depth:       {}", estimate.circuit_depth);
    println!("  Est. memory (bytes): {}", estimate.estimated_memory_bytes);
    println!("  Recommended backend: {:?}", estimate.recommended_backend);
    println!(
        "  Runtime class:       {:?}",
        estimate.estimated_runtime_class
    );
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Circuit { action } => match action {
            CircuitAction::Create {
                qubits,
                gates,
                output,
            } => cmd_circuit_create(qubits, &gates, &output),
            CircuitAction::Info { circuit } => cmd_circuit_info(&circuit),
        },
        Commands::Sim { action } => match action {
            SimAction::Run {
                circuit,
                shots,
                output,
            } => cmd_sim_run(&circuit, shots, &output),
        },
        Commands::Qec { action } => match action {
            QecAction::Surface {
                distance,
                error_rate,
                trials,
            } => cmd_qec_surface(distance, error_rate, trials),
        },
        Commands::Estimate { circuit } => cmd_estimate(&circuit),
    }
}
