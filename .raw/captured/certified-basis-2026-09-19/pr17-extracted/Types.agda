{-# OPTIONS --safe #-}
open import zkir-v3.Assumptions

------------------------------------------------------------------------
-- Types and values of zkir-v3  (ir_types.rs)
--
-- The IR's type set, the off-circuit value domain `IrValue`, the
-- per-type encoding width, and the value→type and default-value maps.
------------------------------------------------------------------------

module zkir-v3.Types (⋯ : _) (open Assumptions ⋯) where

open import Data.Empty using (⊥)
open import Data.List using (List; []; _∷_)
open import Data.Maybe using (Maybe; just; nothing)
open import Data.Nat using (ℕ)
open import Data.Product using (_×_; _,_)
open import Data.Sum using (_⊎_)
open import Relation.Binary.Definitions using (DecidableEquality)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Relation.Nullary using (yes; no)

------------------------------------------------------------------------
-- Types  (ir_types.rs: IrType)
------------------------------------------------------------------------

data IrType : Set where
  native            : IrType   -- Scalar<BLS12-381>
  bytes32           : IrType   -- Bytes<32>
  jubjub-point      : IrType   -- Point<Jubjub>
  jubjub-scalar     : IrType   -- Scalar<Jubjub>
  secp256k1-point        : IrType   -- Point<Secp256k1>
  secp256k1-base         : IrType   -- Base<Secp256k1>
  secp256k1-scalar       : IrType   -- Scalar<Secp256k1>
  secp256r1-point   : IrType   -- Point<Secp256r1>
  secp256r1-base    : IrType   -- Base<Secp256r1>
  secp256r1-scalar  : IrType   -- Scalar<Secp256r1>
  curve25519-point  : IrType   -- Point<Curve25519>
  curve25519-base   : IrType   -- Base<Curve25519>
  curve25519-scalar : IrType   -- Scalar<Curve25519>

-- Number of raw Fr elements needed to encode a value of this type
-- (ir_types.rs: IrType::encoded_len).
encoded-len : IrType → ℕ
encoded-len native            = 1
encoded-len bytes32           = 2
encoded-len jubjub-point      = 2
encoded-len jubjub-scalar     = 1
encoded-len secp256k1-point        = 5
encoded-len secp256k1-base         = 2
encoded-len secp256k1-scalar       = 2
encoded-len secp256r1-point   = 5
encoded-len secp256r1-base    = 2
encoded-len secp256r1-scalar  = 2
encoded-len curve25519-point  = 4
encoded-len curve25519-base   = 2
encoded-len curve25519-scalar = 2

------------------------------------------------------------------------
-- Off-circuit values  (ir_types.rs: IrValue)
--
-- Constructors carry the concrete carrier; they are named distinctly
-- from the `IrType` tags to avoid overloading.
------------------------------------------------------------------------

data IrValue : Set where
  val-native            : Fr               → IrValue
  val-bytes32           : Bytes32          → IrValue
  val-jubjub-point      : JubjubPoint      → IrValue
  val-jubjub-scalar     : JubjubScalar     → IrValue
  val-secp256k1-point        : Secp256k1Point        → IrValue
  val-secp256k1-base         : Secp256k1Base         → IrValue
  val-secp256k1-scalar       : Secp256k1Scalar       → IrValue
  val-secp256r1-point   : Secp256r1Point   → IrValue
  val-secp256r1-base    : Secp256r1Base    → IrValue
  val-secp256r1-scalar  : Secp256r1Scalar  → IrValue
  val-curve25519-point  : Curve25519Point  → IrValue
  val-curve25519-base   : Curve25519Base   → IrValue
  val-curve25519-scalar : Curve25519Scalar → IrValue

-- The type of a value  (ir_types.rs: IrValue::get_type).
typeof : IrValue → IrType
typeof (val-native            _) = native
typeof (val-bytes32           _) = bytes32
typeof (val-jubjub-point      _) = jubjub-point
typeof (val-jubjub-scalar     _) = jubjub-scalar
typeof (val-secp256k1-point        _) = secp256k1-point
typeof (val-secp256k1-base         _) = secp256k1-base
typeof (val-secp256k1-scalar       _) = secp256k1-scalar
typeof (val-secp256r1-point   _) = secp256r1-point
typeof (val-secp256r1-base    _) = secp256r1-base
typeof (val-secp256r1-scalar  _) = secp256r1-scalar
typeof (val-curve25519-point  _) = curve25519-point
typeof (val-curve25519-base   _) = curve25519-base
typeof (val-curve25519-scalar _) = curve25519-scalar

-- Decidable equality of types (used by the type checks of `cond-select`
-- and the `output` terminator).
_≟T_ : DecidableEquality IrType
native        ≟T native        = yes refl
native        ≟T bytes32       = no (λ ())
native        ≟T jubjub-point  = no (λ ())
native        ≟T jubjub-scalar = no (λ ())
native        ≟T secp256k1-point    = no (λ ())
native        ≟T secp256k1-base     = no (λ ())
native        ≟T secp256k1-scalar   = no (λ ())
native        ≟T secp256r1-point  = no (λ ())
native        ≟T secp256r1-base   = no (λ ())
native        ≟T secp256r1-scalar = no (λ ())
native        ≟T curve25519-point   = no (λ ())
native        ≟T curve25519-base    = no (λ ())
native        ≟T curve25519-scalar  = no (λ ())
bytes32       ≟T native        = no (λ ())
bytes32       ≟T bytes32       = yes refl
bytes32       ≟T jubjub-point  = no (λ ())
bytes32       ≟T jubjub-scalar = no (λ ())
bytes32       ≟T secp256k1-point    = no (λ ())
bytes32       ≟T secp256k1-base     = no (λ ())
bytes32       ≟T secp256k1-scalar   = no (λ ())
bytes32       ≟T secp256r1-point  = no (λ ())
bytes32       ≟T secp256r1-base   = no (λ ())
bytes32       ≟T secp256r1-scalar = no (λ ())
bytes32       ≟T curve25519-point   = no (λ ())
bytes32       ≟T curve25519-base    = no (λ ())
bytes32       ≟T curve25519-scalar  = no (λ ())
jubjub-point  ≟T native        = no (λ ())
jubjub-point  ≟T bytes32       = no (λ ())
jubjub-point  ≟T jubjub-point  = yes refl
jubjub-point  ≟T jubjub-scalar = no (λ ())
jubjub-point  ≟T secp256k1-point    = no (λ ())
jubjub-point  ≟T secp256k1-base     = no (λ ())
jubjub-point  ≟T secp256k1-scalar   = no (λ ())
jubjub-point  ≟T secp256r1-point  = no (λ ())
jubjub-point  ≟T secp256r1-base   = no (λ ())
jubjub-point  ≟T secp256r1-scalar = no (λ ())
jubjub-point  ≟T curve25519-point   = no (λ ())
jubjub-point  ≟T curve25519-base    = no (λ ())
jubjub-point  ≟T curve25519-scalar  = no (λ ())
jubjub-scalar ≟T native        = no (λ ())
jubjub-scalar ≟T bytes32       = no (λ ())
jubjub-scalar ≟T jubjub-point  = no (λ ())
jubjub-scalar ≟T jubjub-scalar = yes refl
jubjub-scalar ≟T secp256k1-point    = no (λ ())
jubjub-scalar ≟T secp256k1-base     = no (λ ())
jubjub-scalar ≟T secp256k1-scalar   = no (λ ())
jubjub-scalar ≟T secp256r1-point  = no (λ ())
jubjub-scalar ≟T secp256r1-base   = no (λ ())
jubjub-scalar ≟T secp256r1-scalar = no (λ ())
jubjub-scalar ≟T curve25519-point   = no (λ ())
jubjub-scalar ≟T curve25519-base    = no (λ ())
jubjub-scalar ≟T curve25519-scalar  = no (λ ())
secp256k1-point    ≟T native        = no (λ ())
secp256k1-point    ≟T bytes32       = no (λ ())
secp256k1-point    ≟T jubjub-point  = no (λ ())
secp256k1-point    ≟T jubjub-scalar = no (λ ())
secp256k1-point    ≟T secp256k1-point    = yes refl
secp256k1-point    ≟T secp256k1-base     = no (λ ())
secp256k1-point    ≟T secp256k1-scalar   = no (λ ())
secp256k1-point    ≟T secp256r1-point  = no (λ ())
secp256k1-point    ≟T secp256r1-base   = no (λ ())
secp256k1-point    ≟T secp256r1-scalar = no (λ ())
secp256k1-point    ≟T curve25519-point   = no (λ ())
secp256k1-point    ≟T curve25519-base    = no (λ ())
secp256k1-point    ≟T curve25519-scalar  = no (λ ())
secp256k1-base     ≟T native        = no (λ ())
secp256k1-base     ≟T bytes32       = no (λ ())
secp256k1-base     ≟T jubjub-point  = no (λ ())
secp256k1-base     ≟T jubjub-scalar = no (λ ())
secp256k1-base     ≟T secp256k1-point    = no (λ ())
secp256k1-base     ≟T secp256k1-base     = yes refl
secp256k1-base     ≟T secp256k1-scalar   = no (λ ())
secp256k1-base     ≟T secp256r1-point  = no (λ ())
secp256k1-base     ≟T secp256r1-base   = no (λ ())
secp256k1-base     ≟T secp256r1-scalar = no (λ ())
secp256k1-base     ≟T curve25519-point   = no (λ ())
secp256k1-base     ≟T curve25519-base    = no (λ ())
secp256k1-base     ≟T curve25519-scalar  = no (λ ())
secp256k1-scalar   ≟T native        = no (λ ())
secp256k1-scalar   ≟T bytes32       = no (λ ())
secp256k1-scalar   ≟T jubjub-point  = no (λ ())
secp256k1-scalar   ≟T jubjub-scalar = no (λ ())
secp256k1-scalar   ≟T secp256k1-point    = no (λ ())
secp256k1-scalar   ≟T secp256k1-base     = no (λ ())
secp256k1-scalar   ≟T secp256k1-scalar   = yes refl
secp256k1-scalar   ≟T secp256r1-point  = no (λ ())
secp256k1-scalar   ≟T secp256r1-base   = no (λ ())
secp256k1-scalar   ≟T secp256r1-scalar = no (λ ())
secp256k1-scalar   ≟T curve25519-point   = no (λ ())
secp256k1-scalar   ≟T curve25519-base    = no (λ ())
secp256k1-scalar   ≟T curve25519-scalar  = no (λ ())

secp256r1-point  ≟T native           = no (λ ())
secp256r1-point  ≟T bytes32          = no (λ ())
secp256r1-point  ≟T jubjub-point     = no (λ ())
secp256r1-point  ≟T jubjub-scalar    = no (λ ())
secp256r1-point  ≟T secp256k1-point       = no (λ ())
secp256r1-point  ≟T secp256k1-base        = no (λ ())
secp256r1-point  ≟T secp256k1-scalar      = no (λ ())
secp256r1-point  ≟T secp256r1-point  = yes refl
secp256r1-point  ≟T secp256r1-base   = no (λ ())
secp256r1-point  ≟T secp256r1-scalar = no (λ ())
secp256r1-point   ≟T curve25519-point   = no (λ ())
secp256r1-point   ≟T curve25519-base    = no (λ ())
secp256r1-point   ≟T curve25519-scalar  = no (λ ())

secp256r1-base   ≟T native           = no (λ ())
secp256r1-base   ≟T bytes32          = no (λ ())
secp256r1-base   ≟T jubjub-point     = no (λ ())
secp256r1-base   ≟T jubjub-scalar    = no (λ ())
secp256r1-base   ≟T secp256k1-point       = no (λ ())
secp256r1-base   ≟T secp256k1-base        = no (λ ())
secp256r1-base   ≟T secp256k1-scalar      = no (λ ())
secp256r1-base   ≟T secp256r1-point  = no (λ ())
secp256r1-base   ≟T secp256r1-base   = yes refl
secp256r1-base   ≟T secp256r1-scalar = no (λ ())
secp256r1-base    ≟T curve25519-point   = no (λ ())
secp256r1-base    ≟T curve25519-base    = no (λ ())
secp256r1-base    ≟T curve25519-scalar  = no (λ ())

secp256r1-scalar ≟T native           = no (λ ())
secp256r1-scalar ≟T bytes32          = no (λ ())
secp256r1-scalar ≟T jubjub-point     = no (λ ())
secp256r1-scalar ≟T jubjub-scalar    = no (λ ())
secp256r1-scalar ≟T secp256k1-point       = no (λ ())
secp256r1-scalar ≟T secp256k1-base        = no (λ ())
secp256r1-scalar ≟T secp256k1-scalar      = no (λ ())
secp256r1-scalar ≟T secp256r1-point  = no (λ ())
secp256r1-scalar ≟T secp256r1-base   = no (λ ())
secp256r1-scalar ≟T secp256r1-scalar = yes refl
secp256r1-scalar  ≟T curve25519-point   = no (λ ())
secp256r1-scalar  ≟T curve25519-base    = no (λ ())
secp256r1-scalar  ≟T curve25519-scalar  = no (λ ())

curve25519-point   ≟T native             = no (λ ())
curve25519-point   ≟T bytes32            = no (λ ())
curve25519-point   ≟T jubjub-point       = no (λ ())
curve25519-point   ≟T jubjub-scalar      = no (λ ())
curve25519-point   ≟T secp256k1-point         = no (λ ())
curve25519-point   ≟T secp256k1-base          = no (λ ())
curve25519-point   ≟T secp256k1-scalar        = no (λ ())
curve25519-point   ≟T secp256r1-point    = no (λ ())
curve25519-point   ≟T secp256r1-base     = no (λ ())
curve25519-point   ≟T secp256r1-scalar   = no (λ ())
curve25519-point   ≟T curve25519-point   = yes refl
curve25519-point   ≟T curve25519-base    = no (λ ())
curve25519-point   ≟T curve25519-scalar  = no (λ ())

curve25519-base    ≟T native             = no (λ ())
curve25519-base    ≟T bytes32            = no (λ ())
curve25519-base    ≟T jubjub-point       = no (λ ())
curve25519-base    ≟T jubjub-scalar      = no (λ ())
curve25519-base    ≟T secp256k1-point         = no (λ ())
curve25519-base    ≟T secp256k1-base          = no (λ ())
curve25519-base    ≟T secp256k1-scalar        = no (λ ())
curve25519-base    ≟T secp256r1-point    = no (λ ())
curve25519-base    ≟T secp256r1-base     = no (λ ())
curve25519-base    ≟T secp256r1-scalar   = no (λ ())
curve25519-base    ≟T curve25519-point   = no (λ ())
curve25519-base    ≟T curve25519-base    = yes refl
curve25519-base    ≟T curve25519-scalar  = no (λ ())

curve25519-scalar  ≟T native             = no (λ ())
curve25519-scalar  ≟T bytes32            = no (λ ())
curve25519-scalar  ≟T jubjub-point       = no (λ ())
curve25519-scalar  ≟T jubjub-scalar      = no (λ ())
curve25519-scalar  ≟T secp256k1-point         = no (λ ())
curve25519-scalar  ≟T secp256k1-base          = no (λ ())
curve25519-scalar  ≟T secp256k1-scalar        = no (λ ())
curve25519-scalar  ≟T secp256r1-point    = no (λ ())
curve25519-scalar  ≟T secp256r1-base     = no (λ ())
curve25519-scalar  ≟T secp256r1-scalar   = no (λ ())
curve25519-scalar  ≟T curve25519-point   = no (λ ())
curve25519-scalar  ≟T curve25519-base    = no (λ ())
curve25519-scalar  ≟T curve25519-scalar  = yes refl

------------------------------------------------------------------------
-- The elliptic-curve families that `ec-mul-generator`,
-- `into-coordinates` and `from-coordinates` dispatch on.
--
-- Each of those three instructions picks a curve from an operand's type
-- and produces a result in a matching type of the same curve.
-- `ECFamily` names the curve; the columns give its point type, the type
-- of a point's x/y coordinates, its scalar type, the carriers behind the
-- point and coordinate types, and the `IrValue` constructors injecting
-- those carriers.  The recognizers invert the type columns, reading the
-- family off an operand's type.  So the point ↔ coordinate ↔ scalar
-- pairing lives in one table instead of being re-derived at each
-- dispatch site.
--
-- `ec-families` is the dispatch table of `into-coordinates` and
-- `from-coordinates`.  `ec-mul-generator` uses `gen-families`, which the
-- Rust supports only for Jubjub and Secp256k1.
------------------------------------------------------------------------

data ECFamily : Set where
  jubjub secp256k1 secp256r1 curve25519 : ECFamily

pointTy : ECFamily → IrType
pointTy jubjub     = jubjub-point
pointTy secp256k1  = secp256k1-point
pointTy secp256r1  = secp256r1-point
pointTy curve25519 = curve25519-point

coordTy : ECFamily → IrType
coordTy jubjub     = native
coordTy secp256k1  = secp256k1-base
coordTy secp256r1  = secp256r1-base
coordTy curve25519 = curve25519-base

scalarTy : ECFamily → IrType
scalarTy jubjub     = jubjub-scalar
scalarTy secp256k1  = secp256k1-scalar
scalarTy secp256r1  = secp256r1-scalar
scalarTy curve25519 = curve25519-scalar

Pointᶠ : ECFamily → Set
Pointᶠ jubjub     = JubjubPoint
Pointᶠ secp256k1  = Secp256k1Point
Pointᶠ secp256r1  = Secp256r1Point
Pointᶠ curve25519 = Curve25519Point

-- Jubjub's coordinates are plain `Fr`: on the Edwards model its base
-- field is the circuit's own field, which is why `coordTy jubjub` is
-- `native` rather than a curve-specific base type.
Coordᶠ : ECFamily → Set
Coordᶠ jubjub     = Fr
Coordᶠ secp256k1  = Secp256k1Base
Coordᶠ secp256r1  = Secp256r1Base
Coordᶠ curve25519 = Curve25519Base

val-pointᶠ : ∀ f → Pointᶠ f → IrValue
val-pointᶠ jubjub     = val-jubjub-point
val-pointᶠ secp256k1  = val-secp256k1-point
val-pointᶠ secp256r1  = val-secp256r1-point
val-pointᶠ curve25519 = val-curve25519-point

val-coordᶠ : ∀ f → Coordᶠ f → IrValue
val-coordᶠ jubjub     = val-native
val-coordᶠ secp256k1  = val-secp256k1-base
val-coordᶠ secp256r1  = val-secp256r1-base
val-coordᶠ curve25519 = val-curve25519-base

pointFamily : IrType → Maybe ECFamily
pointFamily jubjub-point     = just jubjub
pointFamily secp256k1-point  = just secp256k1
pointFamily secp256r1-point  = just secp256r1
pointFamily curve25519-point = just curve25519
pointFamily _                = nothing

coordFamily : IrType → Maybe ECFamily
coordFamily native          = just jubjub
coordFamily secp256k1-base  = just secp256k1
coordFamily secp256r1-base  = just secp256r1
coordFamily curve25519-base = just curve25519
coordFamily _               = nothing

-- Jubjub and Secp256k1 only: `EcMulGenerator` is not extended to the
-- other two curves, so their scalars fall to the catch-all.
genScalarFamily : IrType → Maybe ECFamily
genScalarFamily jubjub-scalar    = just jubjub
genScalarFamily secp256k1-scalar = just secp256k1
genScalarFamily _                = nothing

-- Every recognizer inverts its type column, and every value column
-- lands in the type column it is paired with.
_ : ∀ f → pointFamily (pointTy f) ≡ just f
        × coordFamily (coordTy f) ≡ just f
        × (∀ p → typeof (val-pointᶠ f p) ≡ pointTy f)
        × (∀ x → typeof (val-coordᶠ f x) ≡ coordTy f)
_ = λ { jubjub     → refl , refl , (λ _ → refl) , (λ _ → refl)
      ; secp256k1  → refl , refl , (λ _ → refl) , (λ _ → refl)
      ; secp256r1  → refl , refl , (λ _ → refl) , (λ _ → refl)
      ; curve25519 → refl , refl , (λ _ → refl) , (λ _ → refl) }

ec-families : List ECFamily
ec-families = jubjub ∷ secp256k1 ∷ secp256r1 ∷ curve25519 ∷ []

gen-families : List ECFamily
gen-families = jubjub ∷ secp256k1 ∷ []

-- "`P` holds of one of `fs`", laid out as the right-nested `⊎` that the
-- dispatching instructions' per-family disjuncts are consumed as.
Anyᶠ : (ECFamily → Set) → List ECFamily → Set
Anyᶠ P []           = ⊥
Anyᶠ P (f ∷ [])     = P f
Anyᶠ P (f ∷ g ∷ fs) = P f ⊎ Anyᶠ P (g ∷ fs)
