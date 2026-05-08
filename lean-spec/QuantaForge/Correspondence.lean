/-!
# Correspondence to Rust Implementation

This file documents the semantic connection between Lean specifications
and Rust implementation types. No FFI is involved — the correspondence
is established by documentation and shared algebraic structure.

## Type Mapping

| Lean type              | Rust type                          | Module              |
|------------------------|------------------------------------|---------------------|
| `Gate`                 | `qf_circuit::Gate`                 | `qf-circuit`        |
| `Gate.isClifford`      | `Gate::is_clifford()` (method)     | `qf-circuit`        |
| `Gate.numQubits`       | `Gate::num_qubits()`               | `qf-circuit`        |
| `Gate.isSelfInverse`   | (optimization pass check)          | `qf-circuit`        |
| `CircuitIR`            | `qf_circuit::Circuit`              | `qf-circuit`        |
| `CircuitIR.gate_count` | `Circuit::gate_count()`            | `qf-circuit`        |
| `CircuitIR.is_clifford`| `Circuit::is_clifford_only()`      | `qf-circuit`        |
| `Instruction`          | `qf_circuit::Instruction`          | `qf-circuit`        |
| `Pauli`                | (internal to stabilizer sim)       | `qf-sim-stabilizer` |
| `Pauli.mul`            | Tableau row multiplication         | `qf-sim-stabilizer` |
| `Pauli.commutes`       | Stabilizer commutation checks      | `qf-qec-surface`    |
| `GF2.dot`              | `ParityCheckMatrix::syndrome()`    | `qf-qec-ldpc`       |

## Proof Strategy

Properties proved in Lean:
1. Gate algebraic identities (XX=I, HH=I, etc.) — justifies optimization passes
2. Pauli group structure — justifies stabilizer simulation correctness
3. Clifford detection — justifies backend routing in scheduler
4. GF(2) parity properties — justifies syndrome computation

These proofs cover the *mathematical specification*. The Rust implementation
is a *computational realization* of these specifications. The correspondence
is asserted by documentation — not by verified compilation.

## What Is NOT Proved

- Floating-point accuracy of gate matrix entries
- Correct indexing in state-vector manipulation
- Memory safety (handled by Rust's type system)
- Performance characteristics
- Concurrency correctness in the API layer
-/
