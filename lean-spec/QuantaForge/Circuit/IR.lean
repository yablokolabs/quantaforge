import QuantaForge.Gates.Defs

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

def has_measurements (_c : CircuitIR) : Bool :=
  false  -- simplified: our Gate type doesn't include Measure

def empty (n : Nat) : CircuitIR :=
  { numQubits := n, instructions := [] }

def addGate (c : CircuitIR) (g : Gate) (qs : List Nat) : CircuitIR :=
  { c with instructions := c.instructions ++ [{ gate := g, qubits := qs }] }

-- Helper: all predicate distributes over append
private theorem all_append_iff {α : Type} (p : α → Bool) (l1 l2 : List α) :
    (l1 ++ l2).all p = (l1.all p && l2.all p) := by
  induction l1 with
  | nil => simp [List.all]
  | cons h t ih => simp [List.all, List.cons_append, ih, Bool.and_assoc]

-- Properties

theorem empty_has_no_gates (n : Nat) : (empty n).gate_count = 0 := rfl

theorem empty_is_clifford (n : Nat) : (empty n).is_clifford = true := rfl

theorem add_gate_increments_count (c : CircuitIR) (g : Gate) (qs : List Nat) :
    (c.addGate g qs).gate_count = c.gate_count + 1 := by
  simp [addGate, gate_count, List.length_append]

theorem clifford_circuit_stays_clifford (c : CircuitIR) (g : Gate) (qs : List Nat)
    (hc : c.is_clifford = true) (hg : g.isClifford = true) :
    (c.addGate g qs).is_clifford = true := by
  unfold is_clifford at hc ⊢
  unfold addGate
  simp only [all_append_iff]
  simp [List.all, hc, hg]

theorem adding_t_makes_non_clifford (c : CircuitIR) (qs : List Nat)
    (_hc : c.is_clifford = true) :
    (c.addGate .T qs).is_clifford = false := by
  unfold is_clifford at *
  unfold addGate
  simp only [all_append_iff]
  simp [List.all, Gate.isClifford]

end CircuitIR
