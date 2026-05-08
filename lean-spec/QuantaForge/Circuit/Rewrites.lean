import QuantaForge.Circuit.IR

/-!
# Circuit Rewrite Rules

Prove that certain circuit transformations preserve semantics.
Working at the instruction-list level.
-/

namespace CircuitIR

/-- Two circuits are equivalent if they have the same gate sequence -/
def equiv (c1 c2 : CircuitIR) : Prop :=
  c1.numQubits = c2.numQubits ∧ c1.instructions = c2.instructions

/-- Appending an empty list of instructions does not change the circuit -/
theorem append_nil (c : CircuitIR) :
    equiv { c with instructions := c.instructions ++ [] } c := by
  constructor
  · rfl
  · simp [List.append_nil]

/-- Gate count after appending is additive -/
theorem gate_count_append (c1 c2 : CircuitIR) :
    ({ numQubits := c1.numQubits,
       instructions := c1.instructions ++ c2.instructions } : CircuitIR).gate_count
    = c1.gate_count + c2.gate_count := by
  simp [gate_count, List.length_append]

end CircuitIR
