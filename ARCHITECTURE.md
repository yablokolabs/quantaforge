# QuantaForge Architecture

This document describes the internal architecture of the QuantaForge quantum simulation platform. It is intended for contributors and anyone looking to understand, extend, or integrate with the system.

## Crate Dependency Graph

```
                            ┌───────────┐   ┌──────────┐
                   User     │  qf-cli   │   │  qf-api  │   User
                  facing    └─────┬─────┘   └────┬─────┘   facing
                                  │               │
                   ┌──────────────┼───────────────┤
                   │              │               │
              ┌────▼────┐   ┌────▼─────┐   ┌─────▼──────┐
              │qf-agent │   │qf-sched- │   │ qf-ui-spec │
              │         │   │  uler    │   │  (types)   │
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
                        Foundation layer
```

### Dependency Rules

1. **`qf-math`** is the foundation — no internal dependencies.
2. **`qf-circuit`** depends only on `qf-math` for gate matrices.
3. **Simulators** (`qf-sim-statevec`, `qf-sim-stabilizer`) depend on `qf-math` + `qf-circuit`.
4. **QEC crates** (`qf-qec-surface`, `qf-qec-ldpc`) depend on `qf-math` and optionally `qf-topology`.
5. **`qf-logical`** bridges QEC crates to provide logical-qubit abstractions.
6. **Orchestration** (`qf-scheduler`, `qf-agent`) depends on `qf-circuit` for circuit analysis.
7. **User-facing binaries** (`qf-api`, `qf-cli`) are leaf nodes that compose everything.
8. **`qf-ui-spec`** is a standalone types crate with no internal dependencies (only serde/utoipa).

## Data Flow: Circuit Simulation

```
  User Input           Circuit IR          Simulation            Results
 ┌──────────┐       ┌─────────────┐     ┌──────────────┐     ┌──────────┐
 │ Text DSL │──────▶│             │     │ state-vector │────▶│ ShotRe-  │
 │ "H 0;    │ parse │  Circuit    │────▶│ simulator    │     │ sult     │
 │  CNOT 0 1│       │             │     │ (exact)      │     │ {counts} │
 │  M 0; M 1│       │             │     └──────────────┘     └──────────┘
 └──────────┘       │ - gates     │
                    │ - qubits    │     ┌──────────────┐     ┌──────────┐
 ┌──────────┐       │ - depth     │     │ stabilizer   │────▶│Stabilizer│
 │ Builder  │──────▶│ - metadata  │────▶│ simulator    │     │ Result   │
 │ API      │ build │             │     │ (Clifford)   │     │{measure} │
 └──────────┘       │             │     └──────────────┘     └──────────┘
                    └─────────────┘
 ┌──────────┐            │
 │ JSON /   │────────────┘
 │ YAML     │ deserialize
 └──────────┘
```

### Step-by-step

1. **Circuit Creation**: Users build circuits via `CircuitBuilder` (fluent API), text DSL parsing (`parse_circuit`), or JSON/YAML deserialization.
2. **Circuit IR**: A `Circuit` contains an ordered list of `Instruction`s, each pairing a `Gate` enum variant with target qubit indices. The circuit tracks metadata: depth, gate count, Clifford-only flag.
3. **Backend Selection**: If the circuit is Clifford-only (`circuit.is_clifford_only()`), the stabilizer backend can handle it in O(n²) per gate. Otherwise, state-vector simulation is required.
4. **State-vector Simulation**: Initializes a `StateVector` to |0…0⟩, then applies each gate as a unitary matrix (with Kronecker expansion for multi-qubit systems). Measurement collapses the state and records classical bits.
5. **Shot Execution**: `run_shots(circuit, n)` repeats the simulation n times, accumulating a histogram of measurement outcomes in `ShotResult.counts`.
6. **Stabilizer Simulation**: Uses the Aaronson-Gottesman `Tableau` representation. Each Clifford gate updates the tableau in O(n) time. Measurement uses the stabilizer formalism — outcome is either deterministic or random with equal probability.

### State-vector Memory Model

The simulator stores a dense complex vector of dimension 2^n:

```
Memory = 2^n × 16 bytes (Complex64 = two f64)

 n=10:      16 KB   — instant
 n=20:      16 MB   — sub-second
 n=25:     512 MB   — seconds
 n=30:      16 GB   — minutes, needs large-memory machine
 n=40:      16 TB   — infeasible on any single machine
```

Gate application computes `U ⊗ I` (Kronecker product with identity) and multiplies against the state vector. For single-qubit gates, this is optimized to iterate over pairs of amplitudes rather than constructing the full 2^n × 2^n matrix.

## Error Correction Pipeline

```
 ┌─────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────┐
 │ Code        │    │ Error        │    │ Syndrome     │    │ Decoder  │
 │ Construction│───▶│ Injection    │───▶│ Extraction   │───▶│          │
 │             │    │              │    │              │    │          │
 │ SurfaceCode │    │ random_      │    │ extract_     │    │ Greedy-  │
 │ ::new(d)    │    │ errors(n,p)  │    │ syndrome()   │    │ Decoder  │
 └─────────────┘    └──────────────┘    └──────────────┘    └────┬─────┘
                                                                 │
 ┌─────────────────────────────────────────────────────────┐     │
 │                   Logical Error Check                    │◄────┘
 │                                                         │
 │  Compare (errors ⊕ correction) against logical operators │
 │  → has_logical_x_error() / has_logical_z_error()        │
 └─────────────────────────────────────────────────────────┘
```

### Surface Code

1. **Code Construction**: `SurfaceCode::new(d)` creates a distance-d surface code on a d×d grid. Data qubits sit on vertices; ancilla qubits (X and Z stabilizers) sit on plaquettes and vertices of the dual lattice. For distance 3: 9 data qubits, 8 ancilla qubits.

2. **Error Injection**: `random_errors(n, p, rng)` generates independent X and Z errors on each data qubit with probability p. Returns `(x_errors, z_errors)` as boolean vectors.

3. **Syndrome Extraction**: `extract_syndrome(code, x_errors, z_errors)` computes which stabilizers are violated. An X error on qubit q triggers the Z-stabilizers that include q, and vice versa.

4. **Decoding**: The `GreedyDecoder` examines the syndrome and produces a correction — a set of Pauli corrections that would produce the same syndrome. The decoder is a heuristic; it matches nearby syndrome bits greedily.

5. **Logical Error Check**: `LogicalErrorTracker` checks whether `errors ⊕ correction` forms a non-trivial logical operator (a chain spanning the code). If so, the logical qubit has been corrupted.

6. **Monte Carlo Experiment**: `LogicalErrorTracker::run_experiment()` repeats this pipeline for many trials, producing an `ExperimentResult` with the logical error rate.

### LDPC Codes

The LDPC pipeline follows a similar pattern:

1. **Parity Check Matrix**: `ParityCheckMatrix` stores an m×n binary matrix over GF(2). Built-in constructors: `repetition_code(n)`, `hamming_code(r)`.
2. **CSS Code Construction**: `CssCode::new(hx, hz)` validates the CSS condition (Hx · Hz^T = 0 mod 2) and computes the number of logical qubits.
3. **Syndrome Computation**: `compute_syndrome(h, error)` multiplies H × e over GF(2).
4. **Bit-flip Decoder**: Iterative Gallager Algorithm A — flips bits that participate in the most unsatisfied checks, up to `max_iterations`.

## Agent Orchestration

```
 ┌───────────┐     ┌─────────────┐     ┌───────────┐     ┌────────────┐
 │  TaskSpec  │────▶│  Workload   │────▶│  Task     │────▶│  Solver    │
 │            │     │  Classifier │     │  Router   │     │            │
 │ description│     │             │     │           │     │ Simulation │
 │ domain?    │     │ keyword     │     │ domain →  │     │ Optimiz.   │
 │ circuit?   │     │ matching    │     │ solver    │     │ Verific.   │
 │ params     │     │             │     │           │     │            │
 └───────────┘     └─────────────┘     └───────────┘     └─────┬──────┘
                                                               │
                   ┌──────────────────────────────────────┐    │
                   │         Result Aggregator             │◄───┘
                   │                                      │
                   │ Collects SolverResults, counts        │
                   │ successes/failures, produces summary  │
                   └──────────────────────────────────────┘
```

### Pipeline

1. **Task Specification**: A `TaskSpec` describes work to be done — a natural-language description, an optional domain hint, optional circuit JSON, and key-value parameters.

2. **Classification**: `WorkloadClassifier` assigns a `TaskDomain` (Simulation, Optimization, Verification, ErrorCorrection, ResourceEstimation). If the task already has a domain, it passes through. Otherwise, keyword-based heuristics classify it.

3. **Routing**: `TaskRouter` maps each domain to a registered `Solver`. Three built-in solvers handle the five domains:
   - `SimulationSolver` → Simulation, ResourceEstimation
   - `OptimizationSolver` → Optimization
   - `VerificationSolver` → Verification, ErrorCorrection

4. **Solving**: Each solver processes the task and returns a `SolverResult` with status (Success/Partial/Failed), output text, and metadata.

5. **Aggregation**: `ResultAggregator` collects results from one or more solver invocations, tallies success/failure counts, and produces an `AggregatedResult` with a summary.

6. **Workflows**: `OrchestrationEngine::process_workflow()` handles multi-step task sequences, feeding each through the classify → route → solve → aggregate pipeline.

## API Request Lifecycle

```
  HTTP Request
       │
       ▼
  ┌──────────┐     ┌──────────────┐     ┌─────────────┐
  │  axum    │────▶│  Handler     │────▶│  Domain     │
  │  Router  │     │  Function    │     │  Logic      │
  │          │     │              │     │             │
  │ /health  │     │ Deserialize  │     │ Build       │
  │ /circuits│     │ request body │     │ circuit,    │
  │ /jobs    │     │ via serde    │     │ run sim,    │
  └──────────┘     └──────────────┘     │ plan job    │
                                        └──────┬──────┘
                                               │
                   ┌──────────────┐             │
                   │  JSON        │◄────────────┘
                   │  Response    │
                   │              │
                   │ CircuitResp  │
                   │ JobResponse  │
                   │ ErrorResponse│
                   └──────────────┘
```

### Request Types

- **`POST /circuits`**: Receives `CreateCircuitRequest` with qubit count and gate specs. Builds a `Circuit` via `CircuitBuilder`, serializes it, returns `CircuitResponse` with ID, metadata, and the circuit JSON.

- **`POST /jobs`**: Receives `SubmitJobRequest` with circuit ID and shot count. Looks up the circuit, creates a `Job` in the `JobPlanner`, dispatches simulation, returns `JobResponse`.

- **`GET /jobs/{id}`**: Looks up job status. If completed, includes `JobResultResponse` with measurement counts.

The API uses `Arc<Mutex<AppState>>` for in-memory state (circuits and jobs). There is no persistent storage — state is lost on restart. This is appropriate for a simulation platform.

### Shared Types

`qf-ui-spec` defines all request/response types with `serde` and `utoipa` derives for automatic OpenAPI schema generation. This keeps the API contract in one place, separate from the handler logic.

## Key Design Decisions

### Consuming Builder Pattern

`CircuitBuilder` methods take `self` by value and return `Result<Self, CircuitError>`. This enables method chaining with `?` propagation:

```rust
let circuit = CircuitBuilder::new(2)?
    .h(0)?
    .cnot(0, 1)?
    .measure_all()?
    .build();
```

The tradeoff: you cannot reuse a builder after an error. This was chosen for ergonomics in the common case.

### Stateless Simulators

Both `StateVectorSimulator` and `StabilizerSimulator` are unit structs with no internal state. All state (the quantum state vector or tableau) is created fresh per simulation run. This makes simulators trivially `Send + Sync` and eliminates any risk of state leakage between runs.

### Trait-based Extensibility

Both the decoder (`qec::Decoder`) and solver (`agent::Solver`) systems use trait objects. New decoders (e.g., MWPM, union-find) or solvers can be added by implementing the trait and registering with the router.

### Separation of Noise from Simulation

Noise channels operate on `DensityMatrix`, not `StateVector`. This is intentional — noise is a non-unitary process that mixes pure states. The `DensityMatrix::from_statevector()` bridge converts between representations. This keeps the state-vector simulator clean and exact.

### GF(2) Arithmetic for QEC

LDPC and surface codes work with binary matrices over GF(2). Rather than using a field abstraction, the code uses `u8` values (0 or 1) with XOR for addition and AND for multiplication. This is simpler and faster for the binary case.

## Performance Characteristics

| Operation | Complexity | Typical Timing |
|-----------|-----------|----------------|
| Single-qubit gate (statevec, n qubits) | O(2^n) | ~1ms at n=20 |
| Two-qubit gate (statevec, n qubits) | O(2^n) | ~2ms at n=20 |
| Clifford gate (stabilizer, n qubits) | O(n) | <1μs at n=100 |
| Measurement (statevec) | O(2^n) | ~1ms at n=20 |
| Measurement (stabilizer) | O(n²) | <10μs at n=100 |
| Surface code syndrome extraction (distance d) | O(d²) | <1ms |
| Greedy decoder | O(s²) where s = syndrome size | <1ms |
| LDPC bit-flip decoder (m×n matrix, k iterations) | O(m·n·k) | <10ms |

### Parallelism

- `qf-math` uses Rayon for parallel matrix operations on large state vectors
- `qf-sim-statevec` uses Rayon for parallel amplitude updates during gate application
- Shot execution is sequential (each shot depends on RNG state), but could be parallelized with independent RNG seeds

## Future Extension Points

### New Simulator Backends

Define a common `Simulator` trait (not yet extracted) and implement:
- Tensor network contraction for intermediate qubit counts (30-100)
- GPU-accelerated state-vector simulation
- MPS (matrix product state) for low-entanglement circuits

### Better Decoders

The `Decoder` trait in `qf-qec-surface` is ready for:
- MWPM (minimum-weight perfect matching) via Blossom V
- Union-find decoder (nearly linear time)
- Neural network decoders

### Persistent Storage

Replace in-memory `AppState` with a database backend (SQLite/Postgres) for the API server. The `Job` and circuit types already have serde support.

### LLM Agent Integration

The `Solver` trait in `qf-agent` can be extended with LLM-backed solvers. The current rules-based classifier could be replaced with or augmented by an LLM for natural-language task understanding.

### Pulse-level Simulation

Add a `qf-pulse` crate for Hamiltonian simulation, enabling:
- Custom gate calibration
- Crosstalk modeling
- Realistic noise from pulse imperfections

### Distributed Simulation

State-vector simulation can be distributed across nodes by partitioning the amplitude vector. This would require an MPI or gRPC communication layer and careful management of two-qubit gates that span partitions.
