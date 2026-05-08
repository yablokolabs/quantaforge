# QuantaForge

A production-grade quantum simulation and verification platform built in Rust with Lean 4 formal specifications.

## Overview

QuantaForge is a simulation-first software platform that models a QPU stack, error correction, cloud orchestration, and AI-driven job coordination. This is a **software simulation platform**, not a hardware control system.

### Key Features

- **State-vector simulator** — exact simulation up to ~30 qubits
- **Stabilizer simulator** — polynomial-time Clifford circuit simulation (1000+ qubits)
- **Noise models** — depolarizing, amplitude damping, phase damping, readout noise
- **Surface code QEC** — syndrome extraction, greedy decoder, logical error tracking
- **LDPC/Q-LDPC** — parity check matrices, CSS codes, bit-flip decoder
- **Multi-agent orchestration** — rules-based workload classification and routing
- **REST API** — axum-based API for job submission and management
- **CLI** — full command-line interface
- **Lean 4 proofs** — formal verification of gate identities, Pauli algebra, circuit properties

### Architecture

```
                            ┌───────────┐   ┌──────────┐
                            │  qf-cli   │   │  qf-api  │
                            └─────┬─────┘   └────┬─────┘
                                  │               │
                   ┌──────────────┼───────────────┤
                   │              │               │
              ┌────▼────┐   ┌────▼─────┐   ┌─────▼──────┐
              │qf-agent │   │qf-sched- │   │ qf-ui-spec │
              │         │   │  uler    │   │            │
              └────┬────┘   └────┬─────┘   └────────────┘
                   │             │
          ┌────────┼─────────────┤
          │        │             │
   ┌──────▼──────┐ │   ┌────────▼────────┐
   │qf-sim-      │ │   │qf-sim-          │
   │  statevec   │ │   │  stabilizer     │
   └──────┬──────┘ │   └────────┬────────┘
          │        │            │
          │   ┌────▼────┐      │       ┌──────────┐
          │   │qf-noise │      │       │qf-logical│
          │   └────┬────┘      │       └──┬───┬───┘
          │        │           │          │   │
          │   ┌────▼─────┐    │   ┌──────▼┐ ┌▼────────┐
          │   │qf-circuit│    │   │qf-qec-│ │qf-qec-  │
          │   └────┬─────┘    │   │surface│ │  ldpc   │
          │        │          │   └───┬───┘ └──┬──────┘
          │        │          │       │        │
          └────────┼──────────┘   ┌───▼────┐   │
                   │              │qf-topo- │   │
                   │              │  logy   │   │
                   │              └───┬─────┘   │
                   │                  │         │
                   └──────────┬───────┘         │
                              │                 │
                         ┌────▼─────┐           │
                         │ qf-math  │◄──────────┘
                         └──────────┘
```

### Crate Overview

| Crate | Description |
|-------|-------------|
| `qf-math` | Complex numbers, vectors, matrices, tensor products, sparse operators |
| `qf-circuit` | Quantum gates, circuit IR, builder, parser, JSON/YAML serialization |
| `qf-sim-statevec` | Dense state-vector simulator with measurement |
| `qf-sim-stabilizer` | Aaronson-Gottesman CHP stabilizer simulator |
| `qf-noise` | Quantum noise channels (Kraus operator formalism) |
| `qf-topology` | Qubit connectivity graphs, 2D lattice, routing |
| `qf-qec-surface` | Surface code, syndrome extraction, greedy decoder |
| `qf-qec-ldpc` | LDPC/Q-LDPC codes, parity checks, bit-flip decoder |
| `qf-logical` | Logical qubit abstractions, encoded operations |
| `qf-scheduler` | Resource estimation, job planning, batch execution |
| `qf-agent` | Multi-agent orchestration engine |
| `qf-api` | REST API (axum) |
| `qf-cli` | Command-line tool (clap) |
| `qf-ui-spec` | Shared API types and OpenAPI schemas |

## Quick Start

```bash
# Build everything
cargo build --workspace

# Run all tests
cargo test --workspace

# CLI usage
cargo run --bin qf-cli -- circuit create --qubits 2 --gates "H 0; CNOT 0 1"
cargo run --bin qf-cli -- sim run --circuit circuit.json --shots 1000
cargo run --bin qf-cli -- qec surface --distance 3 --error-rate 0.01 --trials 1000

# Start the API server
cargo run --bin qf-api
# Then: curl http://localhost:3000/health
```

## Usage Examples

### Build and simulate a Bell state (Rust API)

```rust
use qf_circuit::CircuitBuilder;
use qf_sim_statevec::StateVectorSimulator;

// Build |Φ+⟩ = (|00⟩ + |11⟩)/√2
let circuit = CircuitBuilder::new(2).unwrap()
    .h(0).unwrap()
    .cnot(0, 1).unwrap()
    .measure(0).unwrap()
    .measure(1).unwrap()
    .build();

let sim = StateVectorSimulator::new();
let result = sim.run_shots(&circuit, 1000).unwrap();

// Result: ~50% |00⟩, ~50% |11⟩ — qubits are perfectly correlated
for (state, count) in &result.counts {
    println!("|{}⟩: {} shots", state, count);
}
```

### Run a noisy simulation

```rust
use qf_noise::{DensityMatrix, DepolarizingChannel, NoiseChannel};
use qf_math::gates;

// Start with |0⟩, apply H gate, then depolarizing noise
let h = gates::hadamard();
let state = DensityMatrix::from_statevec(&[
    num_complex::Complex64::new(1.0, 0.0),
    num_complex::Complex64::new(0.0, 0.0),
]);
let after_h = state.apply_unitary(&h);

let noise = DepolarizingChannel::new(0.05).unwrap(); // 5% error rate
let noisy = noise.apply(&after_h).unwrap();

println!("Purity after noise: {:.4}", noisy.purity());
// Purity < 1.0 indicates mixed state (decoherence)
```

### Surface code error correction experiment

```rust
use qf_qec_surface::{SurfaceCode, GreedyDecoder, LogicalErrorTracker};

let code = SurfaceCode::new(3).unwrap();  // distance-3 code
let decoder = GreedyDecoder;
let mut tracker = LogicalErrorTracker::new(&code, &decoder);

let result = tracker.run_experiment(0.01, 10_000); // p=1%, 10k trials
println!("Logical error rate: {:.6}", result.logical_error_rate);
// Below threshold (~1%), logical rate < physical rate = error suppression
```

### Multi-agent workload orchestration

```rust
use qf_agent::{OrchestrationEngine, WorkloadRequest};

let engine = OrchestrationEngine::new();

let request = WorkloadRequest {
    description: "Simulate a 20-qubit VQE circuit with noise".to_string(),
    qubit_count: Some(20),
    gate_count: Some(500),
    domain_hint: None,
};

let result = engine.process(request);
println!("Routed to: {:?}", result.solver_used);
println!("Backend: {:?}", result.recommended_backend);
```

### CLI usage

```bash
# Create a circuit from gate description
cargo run --bin qf-cli -- circuit create --qubits 3 --gates "H 0; CNOT 0 1; CNOT 1 2; M 0; M 1; M 2"

# Run a simulation with 1000 measurement shots
cargo run --bin qf-cli -- sim run --circuit circuit.json --shots 1000

# Run surface code QEC experiment
cargo run --bin qf-cli -- qec surface --distance 3 --error-rate 0.01 --trials 10000

# Estimate resources for a circuit
cargo run --bin qf-cli -- estimate --qubits 20 --depth 100
```

### REST API usage

```bash
# Start the server
cargo run --bin qf-api &

# Health check
curl http://localhost:3000/health

# Create a circuit
curl -X POST http://localhost:3000/circuits \
  -H "Content-Type: application/json" \
  -d '{"num_qubits": 2, "gates": [{"gate": "H", "targets": [0]}, {"gate": "CNOT", "targets": [0, 1]}]}'

# Submit a simulation job
curl -X POST http://localhost:3000/jobs \
  -H "Content-Type: application/json" \
  -d '{"circuit_id": "<id-from-above>", "shots": 1000, "backend": "statevec"}'

# Check job status
curl http://localhost:3000/jobs/<job-id>
```

### Runnable examples

```bash
# Bell state: create |Φ+⟩ and measure correlations
cargo run --example bell_state

# GHZ state: 3-qubit entanglement with stabilizer comparison
cargo run --example ghz_state

# Noisy simulation: ideal vs depolarizing noise at various error rates
cargo run --example noisy_sim

# Surface code QEC: Monte Carlo error rate estimation at multiple distances
cargo run --example surface_code

# Performance benchmark: state-vector vs stabilizer scaling comparison
cargo run --example benchmark
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| POST | `/circuits` | Create a circuit |
| GET | `/circuits/{id}` | Get circuit details |
| POST | `/jobs` | Submit a simulation job |
| GET | `/jobs/{id}` | Get job status/results |
| GET | `/jobs` | List all jobs |

Swagger UI is available at `/swagger-ui/` when the API server is running.

## Simulation Limits

| Backend | Max Qubits | Gate Set | Memory |
|---------|-----------|----------|--------|
| State-vector | ~30 | Universal | 2^n × 16 bytes |
| Stabilizer | 1000+ | Clifford only | O(n²) |

**Important**: Dense state-vector simulation is exponential in qubit count. 30 qubits require ~16 GB of RAM. This is a fundamental limit of classical simulation, not a bug.

| Qubits | State-vector memory |
|--------|-------------------|
| 10 | 16 KB |
| 20 | 16 MB |
| 25 | 512 MB |
| 30 | 16 GB |

## Lean 4 Formal Specifications

The `lean-spec/` directory contains formal proofs of:

- **Pauli group algebra** — associativity, self-inverse, commutativity relations
- **Gate properties** — Clifford membership, qubit counts, parameterization
- **Circuit IR properties** — gate counting, Clifford detection, depth analysis
- **GF(2) parity check** — CSS code orthogonality conditions
- **Correspondence** — mapping between Lean specifications and Rust types

These proofs cover the mathematical specification. The Rust implementation is a computational realization of these specifications.

```bash
# Build proofs (requires Lean 4 / elan)
cd lean-spec && lake build
```

## Honest Limitations

- Surface code decoder is a greedy heuristic, **not** production MWPM (minimum-weight perfect matching)
- No tensor-network backend (planned)
- No pulse-level simulation — gate-level abstraction only
- Lean proofs cover exact math, not floating-point accuracy of the Rust implementation
- Agent system is rules-based keyword matching, no LLM integration by default
- This is simulation software, not quantum hardware control
- State-vector simulator caps at 30 qubits (configurable, but memory is the real limit)
- No GPU acceleration — CPU only with Rayon parallelism

## Building from Source

### Prerequisites

- Rust 1.75+ (stable)
- For Lean proofs: [elan](https://github.com/leanprover/elan) with Lean 4

```bash
git clone https://github.com/yablokolabs/quantaforge.git
cd quantaforge
cargo build --workspace
cargo test --workspace
```

## License

Apache-2.0
