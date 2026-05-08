/-!
# Pauli Group and Stabilizer Properties

Algebraic properties of the Pauli group relevant to error correction.
Proofs over GF(2) arithmetic for syndrome/parity check operations.
-/

-- Pauli operators as an enum
inductive Pauli where
  | I : Pauli
  | X : Pauli
  | Y : Pauli
  | Z : Pauli
deriving Repr, BEq, DecidableEq

namespace Pauli

/-- Pauli multiplication (ignoring phases) in the quotient group -/
def mul : Pauli → Pauli → Pauli
  | I, p => p
  | p, I => p
  | X, X => I
  | Y, Y => I
  | Z, Z => I
  | X, Y => Z
  | Y, X => Z
  | X, Z => Y
  | Z, X => Y
  | Y, Z => X
  | Z, Y => X

instance : Mul Pauli := ⟨mul⟩

/-- Do two Paulis commute? (true) or anticommute? (false) -/
def commutes : Pauli → Pauli → Bool
  | I, _ => true
  | _, I => true
  | X, X => true
  | Y, Y => true
  | Z, Z => true
  | _, _ => false  -- X,Y / X,Z / Y,Z all anticommute

-- Pauli group axioms

theorem mul_assoc (a b c : Pauli) : (a * b) * c = a * (b * c) := by
  cases a <;> cases b <;> cases c <;> rfl

theorem mul_id_right (a : Pauli) : a * I = a := by
  cases a <;> rfl

theorem mul_id_left (a : Pauli) : I * a = a := by
  cases a <;> rfl

theorem mul_self_eq_id (a : Pauli) : a * a = I := by
  cases a <;> rfl

-- Commutativity proofs

theorem ii_commute : commutes I I = true := rfl
theorem xx_commute : commutes X X = true := rfl
theorem xz_anticommute : commutes X Z = false := rfl
theorem yz_anticommute : commutes Y Z = false := rfl
theorem xy_anticommute : commutes X Y = false := rfl

-- Self-inverse
theorem x_inv : X * X = I := rfl
theorem y_inv : Y * Y = I := rfl
theorem z_inv : Z * Z = I := rfl

-- Stabilizer property: a stabilizer is its own inverse
theorem stabilizer_self_inverse (s : Pauli) : s * s = I := mul_self_eq_id s

end Pauli

/-!
## GF(2) Parity Check Properties

Properties of binary parity check operations used in LDPC and surface codes.
-/
namespace GF2

def dot (v w : List Bool) : Bool :=
  (v.zip w).foldl (fun acc (a, b) => xor acc (a && b)) false

-- Dot product with zero vector is zero
theorem dot_zero (v : List Bool) (_h : v.length = 0) : dot v [] = false := by
  simp [dot, List.zip]

-- Self-dot of zero vector
theorem dot_nil_nil : dot [] [] = false := rfl

-- Syndrome of zero error is zero
theorem zero_syndrome (row : List Bool) :
    dot row (row.map (fun _ => false)) = false := by
  simp [dot]
  induction row with
  | nil => rfl
  | cons _ _ ih =>
    simp [List.zip, List.map, List.foldl]
    exact ih

end GF2
