/-!
# Circuit Intermediate Representation

Abstract circuit IR matching the Rust `qf_circuit::Circuit` type.

## Correspondence to Rust
| Lean type        | Rust type                    |
|------------------|------------------------------|
| `CircuitIR`      | `qf_circuit::Circuit`        |
| `Instruction`    | `qf_circuit::Instruction`    |
| `CircuitIR.gate_count` | `Circuit::gate_count()` |
| `CircuitIR.is_clifford` | `Circuit::is_clifford_only()` |
-/

import QuantaForge.Gates.Defs

structure Instruction where
  gate : Gate
  qubits : List Nat
deriving Repr, BEq

structure CircuitIR where
  numQubits : Nat
  instructions : List Instruction
deriving Repr

namespace CircuitIR

def gate_count (c : CircuitIR) : Nat :=
  c.instructions.length

def is_clifford (c : CircuitIR) : Bool :=
  c.instructions.all (fun i => i.gate.isClifford)

def has_measurements (c : CircuitIR) : Bool :=
  false  -- simplified: our Gate type doesn't include Measure

def empty (n : Nat) : CircuitIR :=
  { numQubits := n, instructions := [] }

def addGate (c : CircuitIR) (g : Gate) (qs : List Nat) : CircuitIR :=
  { c with instructions := c.instructions ++ [{ gate := g, qubits := qs }] }

-- Properties

theorem empty_has_no_gates (n : Nat) : (empty n).gate_count = 0 := rfl

theorem empty_is_clifford (n : Nat) : (empty n).is_clifford = true := rfl

theorem add_gate_increments_count (c : CircuitIR) (g : Gate) (qs : List Nat) :
    (c.addGate g qs).gate_count = c.gate_count + 1 := by
  simp [addGate, gate_count, List.length_append]

theorem clifford_circuit_stays_clifford (c : CircuitIR) (g : Gate) (qs : List Nat)
    (hc : c.is_clifford = true) (hg : g.isClifford = true) :
    (c.addGate g qs).is_clifford = true := by
  simp [addGate, is_clifford, List.all_append]
  constructor
  · exact hc
  · simp [hg]

theorem adding_t_makes_non_clifford (c : CircuitIR) (qs : List Nat)
    (hc : c.is_clifford = true) :
    (c.addGate .T qs).is_clifford = false := by
  simp [addGate, is_clifford, List.all_append]

end CircuitIR
