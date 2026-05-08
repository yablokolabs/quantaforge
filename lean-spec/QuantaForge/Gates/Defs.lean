/-!
# Gate Definitions

We define quantum gates as abstract objects with specified algebraic properties.
Rather than encoding complex matrix entries (which requires real number formalization),
we define gates by their algebraic relations and prove those relations hold.

This corresponds to the Rust `qf_circuit::Gate` enum.
-/

-- Abstract gate type
inductive Gate where
  | I    : Gate   -- Identity
  | X    : Gate   -- Pauli X
  | Y    : Gate   -- Pauli Y
  | Z    : Gate   -- Pauli Z
  | H    : Gate   -- Hadamard
  | S    : Gate   -- Phase gate
  | T    : Gate   -- T gate
  | CNOT : Gate   -- Controlled-NOT (2-qubit)
  | CZ   : Gate   -- Controlled-Z (2-qubit)
deriving Repr, BEq, DecidableEq

-- Gate composition (abstract multiplication)
-- We axiomatize the algebraic structure rather than computing with matrices
class GateAlgebra (G : Type) where
  compose : G → G → G
  id : G
  inv : G → G

-- Is a gate its own inverse?
def Gate.isSelfInverse : Gate → Bool
  | .I => true
  | .X => true
  | .Y => true
  | .Z => true
  | .H => true
  | _ => false

-- Number of qubits a gate acts on
def Gate.numQubits : Gate → Nat
  | .CNOT => 2
  | .CZ => 2
  | _ => 1

-- Is the gate Clifford?
def Gate.isClifford : Gate → Bool
  | .I => true
  | .X => true
  | .Y => true
  | .Z => true
  | .H => true
  | .S => true
  | .CNOT => true
  | .CZ => true
  | .T => false
