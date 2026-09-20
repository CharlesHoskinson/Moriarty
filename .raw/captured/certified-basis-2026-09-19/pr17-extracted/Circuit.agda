{-# OPTIONS --safe #-}
open import zkir-v3.Assumptions

------------------------------------------------------------------------
-- In-circuit (constraint-system) semantics of zkir-v3
-- (ir_vm.rs: `impl Relation for IrSource`, the `circuit` method).
--
-- This module defines (spec §7):
--
--   • A closed vocabulary of arithmetization primitives (`Constraint`)
--     and the structural synthesis function (`synth-instr`, `synth`)
--     that lowers each source instruction to a *list of constraints*,
--     mirroring §7.2's emission contracts.  Synthesis is a deterministic
--     function of the source alone — independent of the prover's
--     preimage (§7.5).
--
--   • A witness model (`CircuitWitness`) and the satisfaction relation
--     (`holds`, `satisfies`).  `holds` is a single interpreter over the
--     closed `Constraint` vocabulary: every gadget atom requires the
--     resolved values to stand in the relation given by the
--     corresponding trust-base function (§7.0, the contract-level
--     reading).
--
-- MODEL.  Unlike zkir-v2 (a positional `List Fr` register file indexed
-- by `Index`), v3's value store is *named* and *typed*: a constraint
-- refers to its inputs by `Operand` and to its outputs by `Identifier`,
-- and resolution reuses the off-circuit modeling of `Semantics`
-- (`resolveᶜ` mirrors `resolve`).  The deterministic instructions emit a
-- constraint asserting the same functional relation the off-circuit
-- `step` computes; the type dispatch matches `Semantics`.
--
-- This module defines only the
-- constraint syntax, the witness, satisfaction, and synthesis; the
-- faithfulness theorems live in CircuitFaithfulness / CircuitProof.
------------------------------------------------------------------------

module zkir-v3.Circuit (⋯ : _) (open Assumptions ⋯) where

open import zkir-v3.Types ⋯
open import zkir-v3.Syntax ⋯
open import zkir-v3.Encoding ⋯ renaming (encode to encodeᵉ)
open import zkir-v3.Semantics ⋯ using (valEq?)
open import zkir-v3.Semantics ⋯ using (χ; pow2ᶠ) public

open import Data.Bool using (Bool; true; false; if_then_else_)
  renaming (not to bnot)
open import Data.List using (List; []; _∷_; _++_; _∷ʳ_; length; map; drop;
                             take)
open import Data.Maybe using (Maybe; just; nothing; _>>=_)
open import Data.Nat using (ℕ; zero; suc; _+_; _*_; _^_; _∸_; _<_; _<?_;
                            _≤_; _⊔_)
open import Data.Nat.DivMod using (_%_)
open import Data.Nat.Properties using (≤-trans; m≤m+n; m≤m⊔n; <-≤-trans;
                                       ^-monoʳ-≤)
open import Data.Product using (_×_; _,_; ∃-syntax)
open import Data.Vec using (reverse)
open import Data.Unit using (⊤; tt)
open import Data.Empty using (⊥)
open import Data.Sum using (_⊎_)
open import Relation.Binary.PropositionalEquality using (_≡_)
open import Relation.Nullary using (¬_)
open import Relation.Nullary.Decidable using (isYes)
open import Function using (case_of_)

------------------------------------------------------------------------
-- The circuit witness.
--
-- The in-circuit prover commits to:
--   • `assign` — the value of every named cell (the in-circuit `memory`).
--   • `pis`    — the public-input vector (binding input, optional
--                commitment, then the guarded `Impact` groups).
--   • `comm-rand` — the commitment randomness, present iff the circuit
--                has a communications commitment.
------------------------------------------------------------------------

record CircuitWitness : Set where
  constructor mk-witness
  field
    assign    : Identifier → Maybe IrValue
    pis       : List Fr
    comm-rand : Maybe Fr

------------------------------------------------------------------------
-- Resolution against the witness (mirrors `Semantics.resolve`).
------------------------------------------------------------------------

resolveᶜ : CircuitWitness → Operand → Maybe IrValue
resolveᶜ w (var id) = CircuitWitness.assign w id
resolveᶜ w (imm x)  = just (val-native x)

-- Resolve, expecting a Native value (an immediate, or a Native cell).
resolveᶜ-Fr : CircuitWitness → Operand → Maybe Fr
resolveᶜ-Fr w op =
  resolveᶜ w op >>= λ { (val-native x) → just x ; _ → nothing }

-- Resolve a list of operands to Native field values.
resolveᶜ-all-Fr : CircuitWitness → List Operand → Maybe (List Fr)
resolveᶜ-all-Fr w []         = just []
resolveᶜ-all-Fr w (op ∷ ops) =
  resolveᶜ-Fr w op        >>= λ x  →
  resolveᶜ-all-Fr w ops   >>= λ xs →
  just (x ∷ xs)

-- Resolve each operand and flatten its raw encoding (used by `comm`,
-- which commits to encode(inputs) ‖ encode(outputs), not to Native cells).
resolve-encode : CircuitWitness → List Operand → Maybe (List Fr)
resolve-encode w []         = just []
resolve-encode w (op ∷ ops) =
  resolveᶜ w op        >>= λ v    →
  resolve-encode w ops >>= λ rest →
  just (encodeᵉ v ++ rest)

-- Look up the n-th public-input entry.
pi-lookup : List Fr → ℕ → Maybe Fr
pi-lookup []       _       = nothing
pi-lookup (x ∷ _)  zero    = just x
pi-lookup (_ ∷ xs) (suc n) = pi-lookup xs n

-- The boolean embedding `χ` and `pow2ᶠ` are shared with `Semantics`
-- (re-exported at the top of this module).

-- `bits-to-ℕ` and `valFr` come from the trust base (`Assumptions`).

-- v ∈ {0, 1}, the booleanity predicate (an `AssignedBit` conversion).
is-bit : Fr → Set
is-bit v = (v ≡ 0ᶠ) ⊎ (v ≡ 1ᶠ)

-- "Operand `op` resolves to value `v`" / "cell `id` holds value `v`":
-- propositional equalities, for readability in the constraint relations.
_⊢_↦_ : CircuitWitness → Operand → IrValue → Set
w ⊢ op ↦ v = resolveᶜ w op ≡ just v

infix 1 _⊢_↦_

_⊢ᶠ_↦_ : CircuitWitness → Operand → Fr → Set
w ⊢ᶠ op ↦ x = resolveᶜ-Fr w op ≡ just x

infix 1 _⊢ᶠ_↦_

------------------------------------------------------------------------
-- The evened bit bound of the deployed range chip.
--
-- `ZkStdLib.lower_than` requires an even bit bound of at least 4, so the
-- Rust lowering evens the instruction's `bits` up
-- (`max(bits + bits % 2, 4)`, ir_vm.rs:966).
------------------------------------------------------------------------

even4 : ℕ → ℕ
even4 bits = (bits + bits % 2) ⊔ 4

bits≤even4 : ∀ bits → bits ≤ even4 bits
bits≤even4 bits =
  ≤-trans (m≤m+n bits (bits % 2)) (m≤m⊔n (bits + bits % 2) 4)

-- The exact off-circuit bound implies the chip's evened bound.
<-even4 : ∀ {v} bits → v < 2 ^ bits → v < 2 ^ even4 bits
<-even4 bits lt = <-≤-trans lt (^-monoʳ-≤ 2 (bits≤even4 bits))

------------------------------------------------------------------------
-- Constraints
--
-- The closed vocabulary of arithmetization primitives (§7.0).  Inputs
-- are referenced by `Operand`, outputs by `Identifier`.  A `gate-*`
-- constructor is a native polynomial relation between resolved values;
-- every other constructor is a gadget atom whose meaning (`holds`,
-- below) is fixed by the corresponding trust-base function.
--
-- The range/decomposition atoms (`div-mod`, `less-than`, `in-range`,
-- `reconstitute`) are contract-level like the rest: their `holds`
-- clauses state the canonical numeric answer directly, trusting the
-- lookup-based chips that implement them to pin it (spec §7.0).
-- Canonicity of these decompositions is part of the trusted chip
-- contract, not derived from gate equations in this development.
------------------------------------------------------------------------

data Constraint : Set where

  -- add: out = a ⊕ b, where ⊕ is the type-directed addition (Native,
  -- JubjubPoint, Secp256k1Point, Secp256k1Base, Secp256k1Scalar, Secp256r1Point,
  -- Secp256r1Base, Secp256r1Scalar, Curve25519Point, Curve25519Base, or
  -- Curve25519Scalar) selected by the operand types.
  gate-add
    : (out : Identifier) → (a b : Operand) → Constraint

  -- mul: out = a · b  (Native, Secp256k1Base, Secp256k1Scalar, Secp256r1Base,
  -- Secp256r1Scalar, Curve25519Base, or Curve25519Scalar).
  gate-mul
    : (out : Identifier) → (a b : Operand) → Constraint

  -- neg: out = ⊖ a  (Native, JubjubPoint, Secp256k1Point, Secp256k1Base,
  -- Secp256k1Scalar, Secp256r1Point, Secp256r1Base, Secp256r1Scalar,
  -- Curve25519Point, Curve25519Base, or Curve25519Scalar).
  gate-neg
    : (out : Identifier) → (a : Operand) → Constraint

  -- inv: out = a⁻¹  (Native, Secp256k1Base, Secp256k1Scalar, Secp256r1Base,
  -- Secp256r1Scalar, Curve25519Base, or Curve25519Scalar; unsatisfiable
  -- on a = 0).
  gate-inv
    : (out : Identifier) → (a : Operand) → Constraint

  -- copy / encode-component: out holds the resolved value of `a`.
  gate-copy
    : (out : Identifier) → (a : Operand) → Constraint

  -- encode(input): the `outputs` hold the raw field elements of `input`.
  encode-eq
    : (input : Operand) → (outputs : List Identifier) → Constraint

  -- constrain_eq(a, b): a and b resolve to the same value.
  eq
    : (a b : Operand) → Constraint

  -- constrain_to_boolean(v): ⟦v⟧ ∈ {0, 1}.
  boolean
    : (v : Operand) → Constraint

  -- assert(c): ⟦c⟧ ≠ 0.
  non-zero
    : (c : Operand) → Constraint

  -- constrain_bits(v, n): ⟦v⟧ < 2ⁿ.
  in-range
    : (v : Operand) → (bits : ℕ) → Constraint

  -- cond_select(bit, a, b): bit ∈ {0,1} ∧ out = (if bit then a else b).
  -- The bit-to-`AssignedBit` conversion adds the booleanity conjunct.
  select
    : (out : Identifier) → (bit a b : Operand) → Constraint

  -- test_eq(a, b): out = χ(a = b)  (a Native 0/1 flag).
  test-eq
    : (out : Identifier) → (a b : Operand) → Constraint

  -- not(a): a ∈ {0,1} ∧ out = χ(¬a)  (bit conversion adds booleanity).
  is-not
    : (out : Identifier) → (a : Operand) → Constraint

  -- less_than(a, b, n): a < 2ᵉ ∧ b < 2ᵉ ∧ out = χ(a < b), where
  -- e = even4 n.
  less-than
    : (out : Identifier) → (a b : Operand) → (bits : ℕ) → Constraint

  -- div_mod_power_of_two(v, n): out0 = v ≫ n, out1 = v mod 2ⁿ, via the
  -- canonical little-endian bit decomposition of v.
  div-mod
    : (q r : Identifier) → (v : Operand) → (bits : ℕ) → Constraint

  -- reconstitute_field(d, m, n): d < 2^(FR_BITS − n) ∧ m < 2ⁿ ∧
  -- out = 2ⁿ·d + m.
  reconstitute
    : (out : Identifier) → (d m : Operand) → (bits : ℕ) → Constraint

  -- jubjub_scalar_from_native(a): out = native→jubjubScalar a  (Native).
  scalar-from-native
    : (out : Identifier) → (a : Operand) → Constraint

  -- transient_hash(I): out = Poseidon(⟦I⟧)  (inputs Native).
  poseidon
    : (out : Identifier) → (inputs : List Operand) → Constraint

  -- persistent_hash(α, I): out = SHA-256 digest of ⟦I⟧ under α (Bytes32).
  sha256
    : (out : Identifier) → (alignment : Alignment)
    → (inputs : List Operand) → Constraint

  -- keccak256(α, I): out = Keccak-256 digest of ⟦I⟧ under α (Bytes32).
  keccak
    : (out : Identifier) → (alignment : Alignment)
    → (inputs : List Operand) → Constraint

  -- ec_mul(a, s): out = ⟦s⟧ · ⟦a⟧  (JubjubPoint × JubjubScalar,
  -- Secp256k1Point × Secp256k1Scalar, Secp256r1Point × Secp256r1Scalar, or
  -- Curve25519Point × Curve25519Scalar).
  ec-mul
    : (out : Identifier) → (a scalar : Operand) → Constraint

  -- ec_mul_generator(s): out = ⟦s⟧ · G  (JubjubScalar or Secp256k1Scalar,
  -- with the respective curve's generator; NOT extended to Secp256r1).
  ec-gen
    : (out : Identifier) → (scalar : Operand) → Constraint

  -- hash_to_curve(I): out = H2C(⟦I⟧)  (inputs Native → JubjubPoint).
  h2c
    : (out : Identifier) → (inputs : List Operand) → Constraint

  -- into_coordinates(p): (xo, yo) = affine coordinates of ⟦p⟧ (Jubjub or
  -- Curve25519, always satisfiable; Secp256k1/Secp256r1, where the
  -- identity is unsatisfiable).
  into-coords
    : (xo yo : Identifier) → (point : Operand) → Constraint

  -- from_coordinates(x, y): out = the curve point with coordinates
  -- (⟦x⟧, ⟦y⟧)  (Jubjub/Curve25519: fails if not in the prime-order
  -- subgroup — on-curve alone is not enough; the chip enforces this
  -- via in-circuit cofactor-clearing, verified; see the notes on
  -- `fromCoordsJ`/`fromCoordsC` in Assumptions and spec §9;
  -- Secp256k1/Secp256r1: fails if not on the curve).
  from-coords
    : (out : Identifier) → (x y : Operand) → Constraint

  -- into_bytes32(a): out = the little-endian bytes of ⟦a⟧  (Native,
  -- Secp256k1Base, Secp256k1Scalar, Secp256r1Base, Secp256r1Scalar, Curve25519Base,
  -- or Curve25519Scalar → Bytes32).
  into-bytes
    : (out : Identifier) → (a : Operand) → Constraint

  -- from_bytes32(b): out = ⟦b⟧ read as a field element  (Bytes32 →
  -- Native, Secp256k1Base, Secp256k1Scalar, Secp256r1Base, Secp256r1Scalar,
  -- Curve25519Base, or Curve25519Scalar; on the foreign fields
  -- non-canonical bytes reduce mod the order).
  from-bytes
    : (out : Identifier) → (b : Operand) → Constraint

  -- reverse_bytes(b): out = reverse ⟦b⟧  (Bytes32 → Bytes32).
  reverse-bytes
    : (out : Identifier) → (b : Operand) → Constraint

  -- bytes32_into_low_high(b): (lo, hi) = low/high split of ⟦b⟧.
  bytes-into-low-high
    : (lo hi : Identifier) → (b : Operand) → Constraint

  -- bytes32_from_low_high(lo, hi): out = the Bytes32 reassembled from
  -- (⟦lo⟧, ⟦hi⟧)  (asserts low < 2²⁴⁸, high < 256).
  bytes-from-low-high
    : (out : Identifier) → (lo hi : Operand) → Constraint

  -- Public-input binding: pis[entry] = the binding input the prover
  -- assigned (a free witness cell, π[0]).  Carries the value as a field.
  pi-binding
    : (entry : ℕ) → Constraint

  -- Guarded `Impact` public input: pis[entry] = select(g, ⟦x⟧, 0), where
  -- the guard `g` is converted to a bit (booleanity).
  pi-impact
    : (entry : ℕ) → (guard x : Operand) → Constraint

  -- Communications commitment (§7.4): pis[1] = Poseidon(comm-rand ‖
  -- encode(inputs) ‖ encode(outputs)), where the in-circuit Poseidon is
  -- assumed to agree with `transient-commit` (trust base).  `inputs` are
  -- the operands resolving to the declared circuit inputs, `outputs` the
  -- operands of the terminal `Output`.
  comm
    : (inputs outputs : List Operand) → Constraint

------------------------------------------------------------------------
-- The constraint system: structurally a list of constraints, plus the
-- structural shape (PI-vector length, comm-commitment flag).  Determined
-- by the source alone.
------------------------------------------------------------------------

record Circuit : Set where
  constructor mk-circuit
  field
    constraints : List Constraint
    pi-len      : ℕ          -- expected length of the verifier's PI vector
    has-comm    : Bool

------------------------------------------------------------------------
-- Satisfaction of a single constraint.
--
-- `holds w c` is the proposition "witness `w` satisfies constraint `c`".
-- Gadget atoms defer to the trust-base functions of `Assumptions`,
-- baking in the chips' soundness (the contract-level reading, §7.0).
------------------------------------------------------------------------

-- `bind-each w vs ids`: position-wise, each cell `ids` holds the value
-- `vs` (the output binding of the `encode-eq` clause of `holds`).  Kept
-- top-level so the faithfulness proofs can name and transport it.
bind-each : CircuitWitness → List IrValue → List Identifier → Set
bind-each w []       []         = ⊤
bind-each w (v ∷ vs) (id ∷ ids) =
  (CircuitWitness.assign w id ≡ just v) × bind-each w vs ids
bind-each w _        _          = ⊥

-- The two coordinate gates' per-family contracts, completing the
-- `ECFamily` table of `Types` with the columns that only the in-circuit
-- reading needs.
--
-- `Coordsᶠ` is a relation rather than a function because the underlying
-- operations differ in totality: the Weierstrass `coordsK1`/`coordsP`
-- return `nothing` exactly on the identity, which has no affine
-- coordinates, whereas the twisted-Edwards `coordsJ`/`coordsC` are
-- total.  That `just` is the in-circuit `assert_non_zero` — on the
-- Weierstrass families the relation is simply unsatisfiable at the
-- identity, and on the Edwards families every point relates.
Coordsᶠ : ∀ f → Pointᶠ f → Coordᶠ f → Coordᶠ f → Set
Coordsᶠ jubjub     p x y = coordsJ  p ≡ (x , y)
Coordsᶠ secp256k1  p x y = coordsK1 p ≡ just (x , y)
Coordsᶠ secp256r1  p x y = coordsP  p ≡ just (x , y)
Coordsᶠ curve25519 p x y = coordsC  p ≡ (x , y)

-- The reverse direction is uniformly partial: reconstruction carries an
-- on-curve check on every family, so a plain `Maybe` suffices.
fromCoordsᶠ : ∀ f → Coordᶠ f → Coordᶠ f → Maybe (Pointᶠ f)
fromCoordsᶠ jubjub     = fromCoordsJ
fromCoordsᶠ secp256k1  = fromCoordsK1
fromCoordsᶠ secp256r1  = fromCoordsP
fromCoordsᶠ curve25519 = fromCoordsC

holds : CircuitWitness → Constraint → Set

holds w (gate-add out a b) =
  ∃-syntax (λ av → ∃-syntax (λ bv → ∃-syntax (λ ov →
      (w ⊢ a ↦ av) × (w ⊢ b ↦ bv)
    × (CircuitWitness.assign w out ≡ just ov)
    × ( ∃-syntax (λ x → ∃-syntax (λ y →
          (av ≡ val-native x) × (bv ≡ val-native y)
        × (ov ≡ val-native (x +ᶠ y))))
      ⊎ ∃-syntax (λ p → ∃-syntax (λ q →
          (av ≡ val-jubjub-point p) × (bv ≡ val-jubjub-point q)
        × (ov ≡ val-jubjub-point (p +J q))))
      ⊎ ∃-syntax (λ p → ∃-syntax (λ q →
          (av ≡ val-secp256k1-point p) × (bv ≡ val-secp256k1-point q)
        × (ov ≡ val-secp256k1-point (p +K1 q))))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ y →
          (av ≡ val-secp256k1-base x) × (bv ≡ val-secp256k1-base y)
        × (ov ≡ val-secp256k1-base (x +K1ᵇ y))))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ y →
          (av ≡ val-secp256k1-scalar x) × (bv ≡ val-secp256k1-scalar y)
        × (ov ≡ val-secp256k1-scalar (x +K1ˢ y))))
      ⊎ ∃-syntax (λ p → ∃-syntax (λ q →
          (av ≡ val-secp256r1-point p) × (bv ≡ val-secp256r1-point q)
        × (ov ≡ val-secp256r1-point (p +P q))))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ y →
          (av ≡ val-secp256r1-base x) × (bv ≡ val-secp256r1-base y)
        × (ov ≡ val-secp256r1-base (x +Pᵇ y))))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ y →
          (av ≡ val-secp256r1-scalar x) × (bv ≡ val-secp256r1-scalar y)
        × (ov ≡ val-secp256r1-scalar (x +Pˢ y))))
      ⊎ ∃-syntax (λ p → ∃-syntax (λ q →
          (av ≡ val-curve25519-point p) × (bv ≡ val-curve25519-point q)
        × (ov ≡ val-curve25519-point (p +C q))))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ y →
          (av ≡ val-curve25519-base x) × (bv ≡ val-curve25519-base y)
        × (ov ≡ val-curve25519-base (x +Cᵇ y))))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ y →
          (av ≡ val-curve25519-scalar x) × (bv ≡ val-curve25519-scalar y)
        × (ov ≡ val-curve25519-scalar (x +Cˢ y))))))))

holds w (gate-mul out a b) =
  ∃-syntax (λ av → ∃-syntax (λ bv → ∃-syntax (λ ov →
      (w ⊢ a ↦ av) × (w ⊢ b ↦ bv)
    × (CircuitWitness.assign w out ≡ just ov)
    × ( ∃-syntax (λ x → ∃-syntax (λ y →
          (av ≡ val-native x) × (bv ≡ val-native y)
        × (ov ≡ val-native (x *ᶠ y))))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ y →
          (av ≡ val-secp256k1-base x) × (bv ≡ val-secp256k1-base y)
        × (ov ≡ val-secp256k1-base (x *K1ᵇ y))))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ y →
          (av ≡ val-secp256k1-scalar x) × (bv ≡ val-secp256k1-scalar y)
        × (ov ≡ val-secp256k1-scalar (x *K1ˢ y))))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ y →
          (av ≡ val-secp256r1-base x) × (bv ≡ val-secp256r1-base y)
        × (ov ≡ val-secp256r1-base (x *Pᵇ y))))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ y →
          (av ≡ val-secp256r1-scalar x) × (bv ≡ val-secp256r1-scalar y)
        × (ov ≡ val-secp256r1-scalar (x *Pˢ y))))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ y →
          (av ≡ val-curve25519-base x) × (bv ≡ val-curve25519-base y)
        × (ov ≡ val-curve25519-base (x *Cᵇ y))))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ y →
          (av ≡ val-curve25519-scalar x) × (bv ≡ val-curve25519-scalar y)
        × (ov ≡ val-curve25519-scalar (x *Cˢ y))))))))

holds w (gate-neg out a) =
  ∃-syntax (λ av → ∃-syntax (λ ov →
      (w ⊢ a ↦ av)
    × (CircuitWitness.assign w out ≡ just ov)
    × ( ∃-syntax (λ x →
          (av ≡ val-native x) × (ov ≡ val-native (-ᶠ x)))
      ⊎ ∃-syntax (λ p →
          (av ≡ val-jubjub-point p) × (ov ≡ val-jubjub-point (negJ p)))
      ⊎ ∃-syntax (λ p →
          (av ≡ val-secp256k1-point p) × (ov ≡ val-secp256k1-point (negK1 p)))
      ⊎ ∃-syntax (λ x →
          (av ≡ val-secp256k1-base x) × (ov ≡ val-secp256k1-base (-K1ᵇ x)))
      ⊎ ∃-syntax (λ x →
          (av ≡ val-secp256k1-scalar x) × (ov ≡ val-secp256k1-scalar (-K1ˢ x)))
      ⊎ ∃-syntax (λ p →
          (av ≡ val-secp256r1-point p) × (ov ≡ val-secp256r1-point (negP p)))
      ⊎ ∃-syntax (λ x →
          (av ≡ val-secp256r1-base x) × (ov ≡ val-secp256r1-base (-Pᵇ x)))
      ⊎ ∃-syntax (λ x →
          (av ≡ val-secp256r1-scalar x)
        × (ov ≡ val-secp256r1-scalar (-Pˢ x)))
      ⊎ ∃-syntax (λ p →
          (av ≡ val-curve25519-point p) × (ov ≡ val-curve25519-point (negC p)))
      ⊎ ∃-syntax (λ x →
          (av ≡ val-curve25519-base x) × (ov ≡ val-curve25519-base (-Cᵇ x)))
      ⊎ ∃-syntax (λ x →
          (av ≡ val-curve25519-scalar x)
        × (ov ≡ val-curve25519-scalar (-Cˢ x))))))

holds w (gate-inv out a) =
  ∃-syntax (λ av → ∃-syntax (λ ov →
      (w ⊢ a ↦ av)
    × (CircuitWitness.assign w out ≡ just ov)
    × ( ∃-syntax (λ x → ∃-syntax (λ xi →
          (av ≡ val-native x) × (invᶠ x ≡ just xi)
        × (ov ≡ val-native xi)))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ xi →
          (av ≡ val-secp256k1-base x) × (invK1ᵇ x ≡ just xi)
        × (ov ≡ val-secp256k1-base xi)))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ xi →
          (av ≡ val-secp256k1-scalar x) × (invK1ˢ x ≡ just xi)
        × (ov ≡ val-secp256k1-scalar xi)))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ xi →
          (av ≡ val-secp256r1-base x) × (invPᵇ x ≡ just xi)
        × (ov ≡ val-secp256r1-base xi)))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ xi →
          (av ≡ val-secp256r1-scalar x) × (invPˢ x ≡ just xi)
        × (ov ≡ val-secp256r1-scalar xi)))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ xi →
          (av ≡ val-curve25519-base x) × (invCᵇ x ≡ just xi)
        × (ov ≡ val-curve25519-base xi)))
      ⊎ ∃-syntax (λ x → ∃-syntax (λ xi →
          (av ≡ val-curve25519-scalar x) × (invCˢ x ≡ just xi)
        × (ov ≡ val-curve25519-scalar xi))))))

holds w (gate-copy out a) =
  ∃-syntax (λ av →
      (w ⊢ a ↦ av) × (CircuitWitness.assign w out ≡ just av))

holds w (encode-eq input outputs) =
  ∃-syntax (λ v →
      (w ⊢ input ↦ v)
    × bind-each w (map val-native (encodeᵉ v)) outputs)

holds w (eq a b) =
  ∃-syntax (λ v → (w ⊢ a ↦ v) × (w ⊢ b ↦ v))

holds w (boolean v) =
  ∃-syntax (λ x → (w ⊢ᶠ v ↦ x) × is-bit x)

holds w (non-zero c) =
  ∃-syntax (λ x → (w ⊢ᶠ c ↦ x) × ¬ (x ≡ 0ᶠ))

holds w (in-range v bits) =
  ∃-syntax (λ x → (w ⊢ᶠ v ↦ x) × (valFr x < 2 ^ bits))

holds w (select out bit a b) =
  ∃-syntax (λ bv → ∃-syntax (λ av → ∃-syntax (λ bvl → ∃-syntax (λ ov →
      (w ⊢ᶠ bit ↦ bv) × (w ⊢ a ↦ av) × (w ⊢ b ↦ bvl)
    × (CircuitWitness.assign w out ≡ just ov)
    × is-bit bv
    × ((bv ≡ 1ᶠ → ov ≡ av) × (bv ≡ 0ᶠ → ov ≡ bvl))))))

holds w (test-eq out a b) =
  ∃-syntax (λ av → ∃-syntax (λ bv → ∃-syntax (λ eq →
      (w ⊢ a ↦ av) × (w ⊢ b ↦ bv)
    × (valEq? av bv ≡ just eq)
    × (CircuitWitness.assign w out ≡ just (val-native (χ eq))))))

holds w (is-not out a) =
  ∃-syntax (λ x →
      (w ⊢ᶠ a ↦ x) × is-bit x
    × (CircuitWitness.assign w out
         ≡ just (val-native (χ (bnot (isYes (x ≟ᶠ 1ᶠ)))))))
  -- The booleanity conjunct restricts `x` to {0,1}; there `isYes (x ≟ᶠ
  -- 1ᶠ)` is the boolean reading of `x` (matches `Semantics.to𝔹`).

holds w (less-than out a b bits) =
  ∃-syntax (λ x → ∃-syntax (λ y →
      (w ⊢ᶠ a ↦ x) × (w ⊢ᶠ b ↦ y)
    × (valFr x < 2 ^ even4 bits) × (valFr y < 2 ^ even4 bits)
    × (CircuitWitness.assign w out
         ≡ just (val-native (χ (isYes (valFr x <? valFr y)))))))

holds w (div-mod q r v bits) =
  ∃-syntax (λ x →
      (w ⊢ᶠ v ↦ x)
    × (CircuitWitness.assign w q
         ≡ just (val-native (from-le-bits (drop bits (to-le-bits x)))))
    × (CircuitWitness.assign w r
         ≡ just (val-native (from-le-bits (take bits (to-le-bits x))))))

holds w (reconstitute out d m bits) =
  ∃-syntax (λ dv → ∃-syntax (λ mv →
      (w ⊢ᶠ d ↦ dv) × (w ⊢ᶠ m ↦ mv)
    × (valFr dv < 2 ^ (FR-BITS ∸ bits)) × (valFr mv < 2 ^ bits)
    × (CircuitWitness.assign w out
         ≡ just (val-native ((pow2ᶠ bits *ᶠ dv) +ᶠ mv)))))

holds w (scalar-from-native out a) =
  ∃-syntax (λ x →
      (w ⊢ᶠ a ↦ x)
    × (CircuitWitness.assign w out
         ≡ just (val-jubjub-scalar (native→jubjubScalar x))))

holds w (poseidon out inputs) =
  ∃-syntax (λ frs →
      (resolveᶜ-all-Fr w inputs ≡ just frs)
    × (CircuitWitness.assign w out
         ≡ just (val-native (transient-hash-fn frs))))

holds w (sha256 out alignment inputs) =
  ∃-syntax (λ frs → ∃-syntax (λ v →
      (resolveᶜ-all-Fr w inputs ≡ just frs)
    × (persistent-hash-fn alignment frs ≡ just v)
    × (CircuitWitness.assign w out ≡ just (val-bytes32 v))))

holds w (keccak out alignment inputs) =
  ∃-syntax (λ frs → ∃-syntax (λ v →
      (resolveᶜ-all-Fr w inputs ≡ just frs)
    × (keccak-fn alignment frs ≡ just v)
    × (CircuitWitness.assign w out ≡ just (val-bytes32 v))))

holds w (ec-mul out a scalar) =
    ∃-syntax (λ p → ∃-syntax (λ s →
      (w ⊢ a ↦ val-jubjub-point p) × (w ⊢ scalar ↦ val-jubjub-scalar s)
    × (CircuitWitness.assign w out ≡ just (val-jubjub-point (s ·J p)))))
  ⊎ ∃-syntax (λ p → ∃-syntax (λ s →
      (w ⊢ a ↦ val-secp256k1-point p) × (w ⊢ scalar ↦ val-secp256k1-scalar s)
    × (CircuitWitness.assign w out ≡ just (val-secp256k1-point (s ·K1 p)))))
  ⊎ ∃-syntax (λ p → ∃-syntax (λ s →
      (w ⊢ a ↦ val-secp256r1-point p) × (w ⊢ scalar ↦ val-secp256r1-scalar s)
    × (CircuitWitness.assign w out ≡ just (val-secp256r1-point (s ·P p)))))
  ⊎ ∃-syntax (λ p → ∃-syntax (λ s →
      (w ⊢ a ↦ val-curve25519-point p) × (w ⊢ scalar ↦ val-curve25519-scalar s)
    × (CircuitWitness.assign w out ≡ just (val-curve25519-point (s ·C p)))))

-- Unlike the other curve-dispatching gates, `EcMulGenerator` was never
-- extended past its original curves in the Rust: only Jubjub/Secp256k1
-- scalars are supported (`ir_vm.rs`'s `EcMulGenerator` arms) — neither
-- Secp256r1 nor Curve25519 dispatches here — everything else errors
-- in-circuit. Hence only a 2-way disjunction.
holds w (ec-gen out scalar) =
    ∃-syntax (λ s →
      (w ⊢ scalar ↦ val-jubjub-scalar s)
    × (CircuitWitness.assign w out ≡ just (val-jubjub-point (s ·J genJ))))
  ⊎ ∃-syntax (λ s →
      (w ⊢ scalar ↦ val-secp256k1-scalar s)
    × (CircuitWitness.assign w out ≡ just (val-secp256k1-point (s ·K1 genK1))))

holds w (h2c out inputs) =
  ∃-syntax (λ frs →
      (resolveᶜ-all-Fr w inputs ≡ just frs)
    × (CircuitWitness.assign w out
         ≡ just (val-jubjub-point (hash-to-curve-fn frs))))

holds w (into-coords xo yo point) =
  Anyᶠ (λ f → ∃-syntax (λ p → ∃-syntax (λ x → ∃-syntax (λ y →
      (w ⊢ point ↦ val-pointᶠ f p)
    × Coordsᶠ f p x y
    × (CircuitWitness.assign w xo ≡ just (val-coordᶠ f x))
    × (CircuitWitness.assign w yo ≡ just (val-coordᶠ f y))))))
    ec-families

holds w (from-coords out x y) =
  Anyᶠ (λ f → ∃-syntax (λ xv → ∃-syntax (λ yv → ∃-syntax (λ p →
      (w ⊢ x ↦ val-coordᶠ f xv) × (w ⊢ y ↦ val-coordᶠ f yv)
    × (fromCoordsᶠ f xv yv ≡ just p)
    × (CircuitWitness.assign w out ≡ just (val-pointᶠ f p))))))
    ec-families

holds w (into-bytes out a) =
  ∃-syntax (λ av → ∃-syntax (λ ov →
      (w ⊢ a ↦ av)
    × (CircuitWitness.assign w out ≡ just ov)
    × ( ∃-syntax (λ x →
          (av ≡ val-native x) × (ov ≡ val-bytes32 (nativeToBytes x)))
      ⊎ ∃-syntax (λ x →
          (av ≡ val-secp256k1-base x) × (ov ≡ val-bytes32 (secp256k1BaseToBytes x)))
      ⊎ ∃-syntax (λ s →
          (av ≡ val-secp256k1-scalar s)
        × (ov ≡ val-bytes32 (secp256k1ScalarToBytes s)))
      ⊎ ∃-syntax (λ x →
          (av ≡ val-secp256r1-base x)
        × (ov ≡ val-bytes32 (secp256r1BaseToBytes x)))
      ⊎ ∃-syntax (λ s →
          (av ≡ val-secp256r1-scalar s)
        × (ov ≡ val-bytes32 (secp256r1ScalarToBytes s)))
      ⊎ ∃-syntax (λ x →
          (av ≡ val-curve25519-base x)
        × (ov ≡ val-bytes32 (curve25519BaseToBytes x)))
      ⊎ ∃-syntax (λ s →
          (av ≡ val-curve25519-scalar s)
        × (ov ≡ val-bytes32 (curve25519ScalarToBytes s))))))

-- The target type is not recorded in the constraint (as in the Rust,
-- the chip is selected by the instruction's `val_t`; the executability
-- of the declared target is a witness-shape condition, `WShape`).
holds w (from-bytes out b) =
  ∃-syntax (λ bs →
      (w ⊢ b ↦ val-bytes32 bs)
    × ( (CircuitWitness.assign w out ≡ just (val-native (nativeFromBytes bs)))
      ⊎ (CircuitWitness.assign w out
           ≡ just (val-secp256k1-base (secp256k1BaseFromBytes bs)))
      ⊎ (CircuitWitness.assign w out
           ≡ just (val-secp256k1-scalar (secp256k1ScalarFromBytes bs)))
      ⊎ (CircuitWitness.assign w out
           ≡ just (val-secp256r1-base (secp256r1BaseFromBytes bs)))
      ⊎ (CircuitWitness.assign w out
           ≡ just (val-secp256r1-scalar (secp256r1ScalarFromBytes bs)))
      ⊎ (CircuitWitness.assign w out
           ≡ just (val-curve25519-base (curve25519BaseFromBytes bs)))
      ⊎ (CircuitWitness.assign w out
           ≡ just (val-curve25519-scalar (curve25519ScalarFromBytes bs)))))

holds w (reverse-bytes out b) =
  ∃-syntax (λ bs →
      (w ⊢ b ↦ val-bytes32 bs)
    × (CircuitWitness.assign w out ≡ just (val-bytes32 (reverse bs))))

holds w (bytes-into-low-high lo hi b) =
  ∃-syntax (λ bs → ∃-syntax (λ l → ∃-syntax (λ h →
      (w ⊢ b ↦ val-bytes32 bs)
    × (bytes32→low-high bs ≡ (l , h))
    × (CircuitWitness.assign w lo ≡ just (val-native l))
    × (CircuitWitness.assign w hi ≡ just (val-native h)))))

holds w (bytes-from-low-high out lo hi) =
  ∃-syntax (λ l → ∃-syntax (λ h → ∃-syntax (λ bs →
      (w ⊢ᶠ lo ↦ l) × (w ⊢ᶠ hi ↦ h)
    × (low-high→bytes32 l h ≡ just bs)
    × (CircuitWitness.assign w out ≡ just (val-bytes32 bs)))))

holds w (pi-binding entry) =
  ∃-syntax (λ bv → pi-lookup (CircuitWitness.pis w) entry ≡ just bv)

holds w (pi-impact entry guard x) =
  ∃-syntax (λ g → ∃-syntax (λ xv → ∃-syntax (λ pv →
      (w ⊢ᶠ guard ↦ g) × (w ⊢ᶠ x ↦ xv) × is-bit g
    × (pi-lookup (CircuitWitness.pis w) entry ≡ just pv)
    × ((g ≡ 1ᶠ → pv ≡ xv) × (g ≡ 0ᶠ → pv ≡ 0ᶠ)))))

holds w (comm inputs outputs) =
  ∃-syntax (λ ivs → ∃-syntax (λ ovs → ∃-syntax (λ rv → ∃-syntax (λ pv →
      (resolve-encode w inputs ≡ just ivs)
    × (resolve-encode w outputs ≡ just ovs)
    × (CircuitWitness.comm-rand w ≡ just rv)
    × (pi-lookup (CircuitWitness.pis w) 1 ≡ just pv)
    × (pv ≡ transient-commit (ivs ++ ovs) rv)))))

------------------------------------------------------------------------
-- Conjunctive satisfaction of the whole constraint list.
------------------------------------------------------------------------

satisfies-constraints : List Constraint → CircuitWitness → Set
satisfies-constraints []       _ = ⊤
satisfies-constraints (c ∷ cs) w = holds w c × satisfies-constraints cs w

-- The witness carries commitment randomness iff the circuit has-comm.
Maybe-shape : Bool → Maybe Fr → Set
Maybe-shape true  (just _) = ⊤
Maybe-shape true  nothing  = ⊥
Maybe-shape false _        = ⊤

-- Top-level satisfaction: all constraints hold, the randomness shape
-- matches the comm-commitment flag, and the PI vector has the recorded
-- structural length (= preamble + Σ Impact group sizes).
record satisfies (c : Circuit) (w : CircuitWitness) : Set where
  constructor mk-sat
  field
    pi-length     : length (CircuitWitness.pis w) ≡ Circuit.pi-len c
    rand-shape    : Maybe-shape (Circuit.has-comm c) (CircuitWitness.comm-rand w)
    constraint-ok : satisfies-constraints (Circuit.constraints c) w

------------------------------------------------------------------------
-- Synthesis
--
-- `synth-instr` processes one instruction, appending constraints and
-- threading the PI-entry cursor and the recorded inputs/outputs used by
-- the comm-commitment.  It is total (synthesis cannot fail; §7.5) and
-- preimage-independent.
------------------------------------------------------------------------

record SynthState : Set where
  constructor mk-synth
  field
    constraints  : List Constraint    -- in emission order
    next-pi      : ℕ                  -- next PI entry to be assigned
    output-ops   : List Operand       -- operands of `Output`, in order

private
  push : SynthState → Constraint → SynthState
  push st c = record st { constraints = SynthState.constraints st ∷ʳ c }

  push* : SynthState → List Constraint → SynthState
  push* st cs = record st { constraints = SynthState.constraints st ++ cs }

-- Number of "preamble" PI entries: binding input (+ optional comm).
preamble-pi-count : Bool → ℕ
preamble-pi-count true  = 2
preamble-pi-count false = 1

-- The guarded `Impact` constraints: one PI entry per input, each
-- `select(guard, x, 0)`, allocated consecutively from `start`.  (Public,
-- so the faithfulness proof can name and reason about it.)
impact-constraints : ℕ → Operand → List Operand → List Constraint
impact-constraints _     _     []         = []
impact-constraints start guard (x ∷ xs) =
  pi-impact start guard x ∷ impact-constraints (suc start) guard xs

synth-instr : Instruction → SynthState → SynthState

synth-instr (encode input outputs) st =
  push st (encode-eq input outputs)

synth-instr (assert cond) st =
  push st (non-zero cond)

synth-instr (cond-select bit a b output) st =
  push st (select output bit a b)

synth-instr (constrain-bits val bits) st =
  push st (in-range val bits)

synth-instr (constrain-eq a b) st =
  push st (eq a b)

synth-instr (constrain-to-boolean val) st =
  push st (boolean val)

synth-instr (copy val output) st =
  push st (gate-copy output val)

synth-instr (impact guard inputs) st =
  let start = SynthState.next-pi st
      cs    = impact-constraints start guard inputs
  in record (push* st cs) { next-pi = start + length inputs }

synth-instr (ec-mul a scalar output) st =
  push st (ec-mul output a scalar)

synth-instr (ec-mul-generator scalar output) st =
  push st (ec-gen output scalar)

synth-instr (hash-to-curve inputs output) st =
  push st (h2c output inputs)

synth-instr (into-coordinates point (xo , yo)) st =
  push st (into-coords xo yo point)

synth-instr (from-coordinates (x , y) output) st =
  push st (from-coords output x y)

synth-instr (into-bytes32 input output) st =
  push st (into-bytes output input)

synth-instr (from-bytes32 bytes val-t output) st =
  push st (from-bytes output bytes)

synth-instr (reverse-bytes bytes output) st =
  push st (reverse-bytes output bytes)

synth-instr (bytes32-into-low-high bytes (lo , hi)) st =
  push st (bytes-into-low-high lo hi bytes)

synth-instr (bytes32-from-low-high (lo , hi) output) st =
  push st (bytes-from-low-high output lo hi)

synth-instr (div-mod-power-of-two val bits outputs) st =
  case outputs of λ
    { (q ∷ r ∷ []) → push st (div-mod q r val bits)
    ; _            → st }
  -- WF3 fixes |outputs| = 2; other shapes emit no constraint.

synth-instr (reconstitute-field divisor modulus bits output) st =
  push st (reconstitute output divisor modulus bits)

synth-instr (transient-hash inputs output) st =
  push st (poseidon output inputs)

synth-instr (persistent-hash alignment inputs output) st =
  push st (sha256 output alignment inputs)

synth-instr (keccak256 alignment inputs output) st =
  push st (keccak output alignment inputs)

synth-instr (test-eq a b output) st =
  push st (test-eq output a b)

synth-instr (add a b output) st =
  push st (gate-add output a b)

synth-instr (mul a b output) st =
  push st (gate-mul output a b)

synth-instr (neg a output) st =
  push st (gate-neg output a)

synth-instr (inv a output) st =
  push st (gate-inv output a)

synth-instr (not a output) st =
  push st (is-not output a)

synth-instr (less-than a b bits output) st =
  push st (less-than output a b bits)

synth-instr (jubjub-scalar-from-native a output) st =
  push st (scalar-from-native output a)

synth-instr (public-input guard val-t output) st =
  st  -- a free witness cell of type `val-t`; the guard is ignored (§7.2).

synth-instr (private-input guard val-t output) st =
  st  -- as `public-input`: a free witness cell, guard ignored.

synth-instr (circuit-output vals) st =
  record st { output-ops = SynthState.output-ops st ++ vals }

-- Fold over an instruction list.
synth-instrs : List Instruction → SynthState → SynthState
synth-instrs []       st = st
synth-instrs (i ∷ is) st = synth-instrs is (synth-instr i st)

-- Resolve the declared inputs to operands (their names as variables),
-- in declaration order, for the comm-commitment preimage.
input-operands : List TypedIdentifier → List Operand
input-operands = map (λ ti → var (TypedIdentifier.name ti))

-- Top-level synthesis (§7.1, §7.3).  The PI preamble (binding input,
-- optional commitment) seeds `next-pi`; the instruction list emits the
-- per-instruction constraints (including the guarded `Impact` PI
-- entries); finally, if has-comm, the binding constraint for π[0] and
-- the comm-commitment over the inputs and outputs are appended.  The PI
-- length is the preamble plus the total `Impact` group sizes.
synth : IrSource → Circuit
synth src =
  let hc   = IrSource.do-communications-commitment src
      st₀  = mk-synth (pi-binding 0 ∷ []) (preamble-pi-count hc) []
      st   = synth-instrs (IrSource.instructions src) st₀
      ins  = input-operands (IrSource.inputs src)
      cs   = SynthState.constraints st
      cs′  = if hc
             then cs ∷ʳ comm ins (SynthState.output-ops st)
             else cs
  in mk-circuit cs′ (SynthState.next-pi st) hc
