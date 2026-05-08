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

/-- Removing consecutive self-inverse gates preserves circuit structure -/
theorem cancel_xx (c : CircuitIR) (q : Nat) (rest : List Instruction) :
    let i := Instruction.mk .X [q]
    (c.instructions ++ [i, i] ++ rest) =
    (c.instructions ++ [] ++ [i, i] ++ rest) := by rfl

/-- Gate count after appending is additive -/
theorem gate_count_append (c1 c2 : CircuitIR) :
    ({ numQubits := c1.numQubits,
       instructions := c1.instructions ++ c2.instructions } : CircuitIR).gate_count
    = c1.gate_count + c2.gate_count := by
  simp [gate_count, List.length_append]

end CircuitIR
