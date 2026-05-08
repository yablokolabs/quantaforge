/-!
# QuantaForge Basic Definitions

Core mathematical objects for quantum computing proofs.
We work with a simplified model using `Fin n → Fin n → ℂ` for matrices
since we don't depend on Mathlib.
-/

-- Use built-in Complex if available, otherwise define a minimal version
structure C where
  re : Float
  im : Float
deriving Repr, BEq

-- For proof purposes, we define a "symbolic" complex number type
-- that supports exact arithmetic (not floating point)
inductive QBit where
  | zero : QBit
  | one : QBit
deriving Repr, BEq, DecidableEq

-- 2x2 matrix over a general type
structure Mat2 (α : Type) where
  a00 : α
  a01 : α
  a10 : α
  a11 : α
deriving Repr, BEq

-- 4x4 matrix
structure Mat4 (α : Type) where
  entries : Fin 4 → Fin 4 → α

-- For proofs we work over ℤ or rationals where possible,
-- or use abstract algebraic properties

-- Boolean matrix (for stabilizer/parity check proofs)
abbrev BoolMat (m n : Nat) := Fin m → Fin n → Bool

-- GF(2) arithmetic
def xorBool (a b : Bool) : Bool := a != b
def andBool (a b : Bool) : Bool := a && b
