{-# OPTIONS --safe #-}

------------------------------------------------------------------------
-- Assumptions of the zkir-v3 formalization
--
-- The trust base of the development, collected into a single flat record
-- `Assumptions`, in the style of zkir-v2.  Downstream modules take an
-- `Assumptions` value as a module parameter
-- (`module M (⋯ : _) (open Assumptions ⋯) where`), so the development
-- typechecks under `--safe` with no `postulate`s.
--
-- The assumed laws are only those the faithfulness proofs consume: the
-- field non-triviality `1ᶠ≢0ᶠ` (Group A) and the typed-encoding
-- round-trips (Group C).  Laws are admitted only when a proof needs
-- them.
--
-- Chips are modeled at the level of their *functional contract*: each
-- field below is the relation a `midnight-zk-stdlib` operation
-- guarantees, not its gate-level lowering (see docs/zkir-v3-spec.md §7.0).
------------------------------------------------------------------------

module zkir-v3.Assumptions where

open import Data.Bool    using (Bool; if_then_else_)
open import Data.Fin     using (Fin)
open import Data.List    using (List; []; _∷_)
open import Data.Maybe   using (Maybe; just)
open import Data.Nat     using (ℕ; _+_; _*_)
open import Data.Product using (_×_; _,_; uncurry)
open import Data.Vec     using (Vec)
open import Relation.Binary.Definitions using (DecidableEquality)
open import Relation.Binary.PropositionalEquality using (_≡_)
open import Relation.Nullary using (¬_)

-- Integer value of a little-endian bit list (used by the valuation `valFr`).
bits-to-ℕ : List Bool → ℕ
bits-to-ℕ []       = 0
bits-to-ℕ (b ∷ bs) = (if b then 1 else 0) + 2 * bits-to-ℕ bs

------------------------------------------------------------------------
-- Concrete byte types (ir_types.rs: Bytes32 = [u8; 32]).
--
-- A byte is an element of Fin 256; a `Bytes32` is a length-32 vector of
-- bytes.  These are concrete (they do not depend on the trust base) and
-- are shared by the record fields below and by downstream modules.
------------------------------------------------------------------------

Byte : Set
Byte = Fin 256

Bytes32 : Set
Bytes32 = Vec Byte 32

record Assumptions : Set₁ where

  ----------------------------------------------------------------------
  -- Carrier types.
  ----------------------------------------------------------------------

  field
    Fr               : Set
    -- ^ BLS12-381 scalar field element; the native field and the base
    --   field of Jubjub (transient_crypto::curve::Fr).
    Alignment        : Set
    -- ^ Byte-alignment descriptor (base_crypto::fab::Alignment).
    JubjubPoint      : Set
    -- ^ Point of the Jubjub curve (in its prime-order subgroup).
    JubjubScalar     : Set
    -- ^ Element of the Jubjub scalar field.
    Secp256k1Point        : Set
    -- ^ Point of the Secp256k1 curve (midnight_curves::k256::K256, a
    --   Weierstrass curve of cofactor 1 over a foreign field), including
    --   the identity.
    Secp256k1Base         : Set
    -- ^ Secp256k1 base field element (k256::Fp).
    Secp256k1Scalar       : Set
    -- ^ Secp256k1 scalar field element (k256::Fq).
    Secp256r1Point   : Set
    -- ^ Point of the Secp256r1 curve (midnight_curves::p256::P256, a
    --   Weierstrass curve — NIST P-256 — over a foreign field), including
    --   the identity.
    Secp256r1Base    : Set
    -- ^ Secp256r1 base field element (p256::Fp).
    Secp256r1Scalar  : Set
    -- ^ Secp256r1 scalar field element (p256::Fq).
    Curve25519Point  : Set
    -- ^ Point of the Curve25519 curve (midnight_curves::curve25519::
    --   Curve25519Subgroup, a twisted Edwards curve over a foreign
    --   field), including the identity — which, unlike the Weierstrass
    --   curves above, has real affine coordinates ((0, 1)) rather than
    --   being a special-cased sentinel.
    Curve25519Base   : Set
    -- ^ Curve25519 base field element (curve25519::Fp, the 2²⁵⁵-19 field).
    Curve25519Scalar : Set
    -- ^ Curve25519 scalar field element (curve25519::Scalar, the
    --   standard Ed25519 subgroup order).

  ----------------------------------------------------------------------
  -- Native field (ZkStdLib arithmetic over Fr).
  ----------------------------------------------------------------------

  field
    0ᶠ 1ᶠ : Fr
    _+ᶠ_ _*ᶠ_ : Fr → Fr → Fr
    -ᶠ_       : Fr → Fr
    -- Inversion is partial: `nothing` exactly on 0ᶠ (Inv errors on zero).
    invᶠ      : Fr → Maybe Fr
    _≟ᶠ_      : DecidableEquality Fr

    -- Bit decomposition.  `FR-BITS` is the number of bits (255), and
    -- `FR-ORDER` the field order; `to-le-bits` yields the canonical
    -- little-endian bits and `from-le-bits` reads bits as an integer
    -- reduced modulo the order.  Used by the bit/field instructions.
    FR-BITS FR-ORDER : ℕ
    to-le-bits       : Fr → List Bool
    from-le-bits     : List Bool → Fr

  ----------------------------------------------------------------------
  -- Jubjub (native embedded curve) gadget contracts.
  ----------------------------------------------------------------------

  field
    _+J_ : JubjubPoint → JubjubPoint → JubjubPoint   -- point addition
    _·J_ : JubjubScalar → JubjubPoint → JubjubPoint  -- scalar mult.
    genJ : JubjubPoint                               -- group generator
    idJ  : JubjubPoint                               -- identity (default)
    negJ : JubjubPoint → JubjubPoint                 -- point negation
    _≟J_ : DecidableEquality JubjubPoint

    -- Affine coordinates.  `coordsJ` is total (every subgroup point has
    -- affine coordinates on the Edwards model); `fromCoordsJ` is partial
    -- (the coordinates must denote a point in the prime-order subgroup).
    --
    -- `fromCoordsJ` is also the *in-circuit* contract of
    -- `from-coordinates` (`Circuit.from-coords`), and this is VERIFIED
    -- faithful (2026-07-24, midnight-circuits 7.2.2): the chip's
    -- `point_from_coordinates` enforces subgroup membership in-circuit by
    -- cofactor-clearing — a fresh point under the curve-equation gate,
    -- constrained multiply-by-8, input coordinates pinned to the result
    -- (edwards_chip.rs:783; spec §9, "Contract strength").
    coordsJ     : JubjubPoint → Fr × Fr
    fromCoordsJ : Fr → Fr → Maybe JubjubPoint

    -- Scalar encoding (canonical 252-bit) and the Native → JubjubScalar
    -- reduction (JubjubScalarFromNative).
    jubjubScalarToFr   : JubjubScalar → Fr
    jubjubScalarFromFr : Fr → Maybe JubjubScalar
    native→jubjubScalar : Fr → JubjubScalar

    -- Hash a sequence of native elements to a Jubjub point.
    hash-to-curve-fn : List Fr → JubjubPoint

  ----------------------------------------------------------------------
  -- Secp256k1 (foreign-field emulated curve) gadget contracts.
  --
  -- The in-circuit side is the `midnight-circuits` foreign-field /
  -- foreign-ECC chips (`AssignedField`, `AssignedForeignPoint`), modeled
  -- here by their functional contracts; the CRT/limb internals stay
  -- inside the trust base (spec §7.0, decision record).
  ----------------------------------------------------------------------

  field
    -- Base field Fp (k256::Fp).  Inversion is partial: `nothing`
    -- exactly on 0 (Inv errors on zero).
    _+K1ᵇ_ _*K1ᵇ_ : Secp256k1Base → Secp256k1Base → Secp256k1Base
    -K1ᵇ_       : Secp256k1Base → Secp256k1Base
    invK1ᵇ      : Secp256k1Base → Maybe Secp256k1Base
    _≟K1ᵇ_      : DecidableEquality Secp256k1Base

    -- Scalar field Fq (k256::Fq).
    _+K1ˢ_ _*K1ˢ_ : Secp256k1Scalar → Secp256k1Scalar → Secp256k1Scalar
    -K1ˢ_       : Secp256k1Scalar → Secp256k1Scalar
    invK1ˢ      : Secp256k1Scalar → Maybe Secp256k1Scalar
    _≟K1ˢ_      : DecidableEquality Secp256k1Scalar

    -- Curve group.
    _+K1_ : Secp256k1Point → Secp256k1Point → Secp256k1Point    -- point addition
    negK1 : Secp256k1Point → Secp256k1Point                -- point negation
    _·K1_ : Secp256k1Scalar → Secp256k1Point → Secp256k1Point   -- scalar mult. (msm)
    genK1 : Secp256k1Point                            -- group generator
    idK1  : Secp256k1Point                            -- identity (default)
    _≟K1_ : DecidableEquality Secp256k1Point

    -- Affine coordinates.  `coordsK1` is partial: `nothing` exactly on
    -- the Weierstrass identity, which has no affine coordinates
    -- (IntoCoordinates errors off-circuit / asserts non-zero
    -- in-circuit).  `fromCoordsK1` is partial (on-curve check); the
    -- identity is not constructible from coordinates.
    coordsK1     : Secp256k1Point → Maybe (Secp256k1Base × Secp256k1Base)
    fromCoordsK1 : Secp256k1Base → Secp256k1Base → Maybe Secp256k1Point

    -- Limb encodings (`as_public_input` / `from_public_input` of the
    -- assigned foreign types): a 256-bit field element is two native
    -- limbs; a point is x-limbs ++ y-limbs ++ an identity flag (widths
    -- 2 / 2 / 5, ir_types.rs encoded_len).
    --
    -- TRUST NOTE: the decoders modeled here are the *canonical*
    -- partial inverses — they reject any limb vector the encoder
    -- cannot emit, as the Group C `secp*-sound` laws demand.
    secp256k1Base→limbs   : Secp256k1Base → Fr × Fr
    limbs→secp256k1Base   : Fr → Fr → Maybe Secp256k1Base
    secp256k1Scalar→limbs : Secp256k1Scalar → Fr × Fr
    limbs→secp256k1Scalar : Fr → Fr → Maybe Secp256k1Scalar
    secp256k1Point→limbs  : Secp256k1Point → Fr × Fr × Fr × Fr × Fr
    limbs→secp256k1Point  : Fr → Fr → Fr → Fr → Fr → Maybe Secp256k1Point

    -- Bytes32 conversions (IntoBytes32 / FromBytes32 on the foreign
    -- fields): `to_bytes_le` and the *total* little-endian reduction
    -- `from_le_bytes_with_reduction` (non-canonical bytes reduce mod
    -- the field order).
    secp256k1BaseToBytes     : Secp256k1Base → Bytes32
    secp256k1BaseFromBytes   : Bytes32 → Secp256k1Base
    secp256k1ScalarToBytes   : Secp256k1Scalar → Bytes32
    secp256k1ScalarFromBytes : Bytes32 → Secp256k1Scalar

  ----------------------------------------------------------------------
  -- Secp256r1 (foreign-field emulated curve) gadget contracts.
  --
  -- Structurally identical to the Secp256k1 block above (also a
  -- Weierstrass foreign curve); the `P` tag mirrors `K`, both derived
  -- from the underlying midnight_curves module name (p256 / k256).
  ----------------------------------------------------------------------

  field
    -- Base field Fp (p256::Fp).  Inversion is partial: `nothing`
    -- exactly on 0 (Inv errors on zero).
    _+Pᵇ_ _*Pᵇ_ : Secp256r1Base → Secp256r1Base → Secp256r1Base
    -Pᵇ_        : Secp256r1Base → Secp256r1Base
    invPᵇ       : Secp256r1Base → Maybe Secp256r1Base
    _≟Pᵇ_       : DecidableEquality Secp256r1Base

    -- Scalar field Fq (p256::Fq).
    _+Pˢ_ _*Pˢ_ : Secp256r1Scalar → Secp256r1Scalar → Secp256r1Scalar
    -Pˢ_        : Secp256r1Scalar → Secp256r1Scalar
    invPˢ       : Secp256r1Scalar → Maybe Secp256r1Scalar
    _≟Pˢ_       : DecidableEquality Secp256r1Scalar

    -- Curve group.  No generator constant: unlike Jubjub/Secp256k1,
    -- `EcMulGenerator` does not dispatch on Secp256r1, so no proof
    -- consumes one.
    _+P_ : Secp256r1Point → Secp256r1Point → Secp256r1Point   -- point add.
    negP : Secp256r1Point → Secp256r1Point                    -- point neg.
    _·P_ : Secp256r1Scalar → Secp256r1Point → Secp256r1Point  -- scalar mult.
    idP  : Secp256r1Point                                     -- identity
    _≟P_ : DecidableEquality Secp256r1Point

    -- Affine coordinates.  `coordsP` is partial: `nothing` exactly on
    -- the Weierstrass identity, which has no affine coordinates
    -- (IntoCoordinates errors off-circuit / asserts non-zero
    -- in-circuit).  `fromCoordsP` is partial (on-curve check); the
    -- identity is not constructible from coordinates.
    coordsP     : Secp256r1Point → Maybe (Secp256r1Base × Secp256r1Base)
    fromCoordsP : Secp256r1Base → Secp256r1Base → Maybe Secp256r1Point

    -- Limb encodings (`as_public_input` / `from_public_input` of the
    -- assigned foreign types): a 256-bit field element is two native
    -- limbs; a point is x-limbs ++ y-limbs ++ an identity flag (widths
    -- 2 / 2 / 5, matching Secp256k1's — ir_types.rs encoded_len).
    --
    -- TRUST NOTE: the decoders modeled here are the canonical partial
    -- inverses, matching the same modeling approach used for
    -- Secp256k1 above.
    secp256r1Base→limbs   : Secp256r1Base → Fr × Fr
    limbs→secp256r1Base   : Fr → Fr → Maybe Secp256r1Base
    secp256r1Scalar→limbs : Secp256r1Scalar → Fr × Fr
    limbs→secp256r1Scalar : Fr → Fr → Maybe Secp256r1Scalar
    secp256r1Point→limbs  : Secp256r1Point → Fr × Fr × Fr × Fr × Fr
    limbs→secp256r1Point  : Fr → Fr → Fr → Fr → Fr → Maybe Secp256r1Point

    -- Bytes32 conversions (IntoBytes32 / FromBytes32 on the foreign
    -- fields): `to_bytes_le` and the *total* little-endian reduction
    -- `from_le_bytes_with_reduction` (non-canonical bytes reduce mod
    -- the field order).
    secp256r1BaseToBytes     : Secp256r1Base → Bytes32
    secp256r1BaseFromBytes   : Bytes32 → Secp256r1Base
    secp256r1ScalarToBytes   : Secp256r1Scalar → Bytes32
    secp256r1ScalarFromBytes : Bytes32 → Secp256r1Scalar

  ----------------------------------------------------------------------
  -- Curve25519 (foreign-field emulated Edwards curve) gadget contracts.
  --
  -- Unlike Secp256k1/Secp256r1 (foreign Weierstrass curves) and unlike
  -- Jubjub (a native Edwards curve), Curve25519 is a foreign Edwards
  -- curve — the one combination that needs the identity to be a real
  -- point rather than a special-cased sentinel: `coordsC` below is
  -- TOTAL, mirroring `coordsJ`'s shape, not `coordsK1`/`coordsP`'s
  -- `Maybe`-wrapped shape. Everything else (field arithmetic, limb
  -- encoding, Bytes32 conversion, curve-group operations) mirrors the
  -- Secp256k1/Secp256r1 block's shape (foreign-field carriers); the `C`
  -- tag is derived from the underlying midnight_curves module name
  -- (curve25519), following the same convention as K/k256, P/p256.
  ----------------------------------------------------------------------

  field
    -- Base field Fp (curve25519::Fp).  Inversion is partial: `nothing`
    -- exactly on 0 (Inv errors on zero).
    _+Cᵇ_ _*Cᵇ_ : Curve25519Base → Curve25519Base → Curve25519Base
    -Cᵇ_        : Curve25519Base → Curve25519Base
    invCᵇ       : Curve25519Base → Maybe Curve25519Base
    _≟Cᵇ_       : DecidableEquality Curve25519Base

    -- Scalar field (curve25519::Scalar).
    _+Cˢ_ _*Cˢ_ : Curve25519Scalar → Curve25519Scalar → Curve25519Scalar
    -Cˢ_        : Curve25519Scalar → Curve25519Scalar
    invCˢ       : Curve25519Scalar → Maybe Curve25519Scalar
    _≟Cˢ_       : DecidableEquality Curve25519Scalar

    -- Curve group.  No generator constant: unlike Jubjub/Secp256k1,
    -- `EcMulGenerator` does not dispatch on Curve25519, so no proof
    -- consumes one.
    _+C_ : Curve25519Point → Curve25519Point → Curve25519Point  -- point add.
    negC : Curve25519Point → Curve25519Point                    -- point neg.
    _·C_ : Curve25519Scalar → Curve25519Point → Curve25519Point -- scalar mult.
    idC  : Curve25519Point                                      -- identity
    _≟C_ : DecidableEquality Curve25519Point

    -- Affine coordinates. UNLIKE the Weierstrass curves, `coordsC` is
    -- TOTAL: the twisted-Edwards identity (0, 1) is a real point on the
    -- curve, not a sentinel — IntoCoordinates never errors/asserts on
    -- the identity for this curve (no such check exists in the Rust).
    -- `fromCoordsC` is partial like `fromCoordsJ`: the coordinates
    -- must denote a point in the prime-order subgroup (on-curve alone
    -- is not enough — the cofactor is 8; the chip enforces membership
    -- in-circuit by cofactor-clearing, exactly as for Jubjub — see the
    -- note on `fromCoordsJ` above and spec §9). Unlike the Weierstrass
    -- curves, the identity IS constructible from its coordinates
    -- (0, 1).
    coordsC     : Curve25519Point → Curve25519Base × Curve25519Base
    fromCoordsC : Curve25519Base → Curve25519Base → Maybe Curve25519Point

    -- Limb encodings (`as_public_input` / `from_public_input`): a
    -- 256-bit field element is two native limbs (same widths as
    -- Secp256k1/Secp256r1); a point is x-limbs ++ y-limbs, with NO
    -- identity flag (widths 2 / 2 / 4, ir_types.rs encoded_len — one
    -- element fewer than the Weierstrass curves' 5, since there's no
    -- flag element).
    --
    -- TRUST NOTE: the decoders modeled here are the canonical partial
    -- inverses (they reject what the encoder cannot emit and any
    -- non-torsion-free point).
    curve25519Base→limbs   : Curve25519Base → Fr × Fr
    limbs→curve25519Base   : Fr → Fr → Maybe Curve25519Base
    curve25519Scalar→limbs : Curve25519Scalar → Fr × Fr
    limbs→curve25519Scalar : Fr → Fr → Maybe Curve25519Scalar
    curve25519Point→limbs  : Curve25519Point → Fr × Fr × Fr × Fr
    limbs→curve25519Point  : Fr → Fr → Fr → Fr → Maybe Curve25519Point

    -- Bytes32 conversions (IntoBytes32 / FromBytes32 on the foreign
    -- fields): `to_bytes_le` and the *total* little-endian reduction
    -- `from_le_bytes_with_reduction` (non-canonical bytes reduce mod
    -- the field order).
    curve25519BaseToBytes     : Curve25519Base → Bytes32
    curve25519BaseFromBytes   : Bytes32 → Curve25519Base
    curve25519ScalarToBytes   : Curve25519Scalar → Bytes32
    curve25519ScalarFromBytes : Bytes32 → Curve25519Scalar

  ----------------------------------------------------------------------
  -- Bytes32 ↔ field conversions.
  ----------------------------------------------------------------------

  field
    -- IntoBytes32 / FromBytes32 on Native.  `nativeFromBytes` is total
    -- (non-canonical inputs are reduced modulo the field order).
    nativeToBytes   : Fr → Bytes32
    nativeFromBytes : Bytes32 → Fr

    -- The (low, high) decomposition shared by the Bytes32 encoding and
    -- the Bytes32IntoLowHigh / Bytes32FromLowHigh instructions: `low` is
    -- the first 31 bytes as a field element, `high` the 32nd byte.
    -- `low-high→bytes32` is partial (requires low < 2²⁴⁸ and high < 256).
    bytes32→low-high : Bytes32 → Fr × Fr
    low-high→bytes32 : Fr → Fr → Maybe Bytes32

  ----------------------------------------------------------------------
  -- Hashing and the communications commitment.
  ----------------------------------------------------------------------

  field
    transient-hash-fn  : List Fr → Fr
    -- Persistent (SHA-256) and Keccak-256 hashes parse their inputs under
    -- the alignment, hence partial; each yields a 32-byte digest.
    -- `persistent-hash-fn` models SHA-256 (`Sha256::digest`); `keccak-fn`
    -- models Keccak-256.  Both remain opaque.
    persistent-hash-fn : Alignment → List Fr → Maybe Bytes32
    keccak-fn          : Alignment → List Fr → Maybe Bytes32
    -- Communications commitment: transient_commit(values, randomness).
    transient-commit   : List Fr → Fr → Fr

  ----------------------------------------------------------------------
  -- Field non-triviality (faithfulness Group A).
  --
  -- The only algebraic law assumed about the field operations: `1ᶠ ≢ 0ᶠ`
  -- bridges `assert` and the boolean reasoning of `not` / `cond-select` /
  -- `impact` guards.
  -- The gate atoms' `holds` clauses relate resolved values through the
  -- operations themselves, so no further field laws are consumed.
  ----------------------------------------------------------------------

  field
    1ᶠ≢0ᶠ : ¬ (1ᶠ ≡ 0ᶠ)

  ----------------------------------------------------------------------
  -- Canonical valuation (faithfulness Group B).
  --
  -- The integer value of a field element, `valFr = bits-to-ℕ ∘
  -- to-le-bits`.  The range/decomposition contracts (`in-range`,
  -- `less-than`, `div-mod`, `reconstitute`) and the reconstitute
  -- no-overflow guard state their numeric bounds directly in terms of
  -- it; no laws about the valuation are assumed — canonicity of the
  -- underlying decompositions is part of the trusted chip contracts
  -- (spec §7.0, decision record).
  ----------------------------------------------------------------------

  valFr : Fr → ℕ
  valFr x = bits-to-ℕ (to-le-bits x)

  ----------------------------------------------------------------------
  -- Typed-encoding round-trips (faithfulness Group C).
  --
  -- The decode/encode primitives for the non-native types are mutually
  -- inverse on valid data: the `*-round` laws give `decode ∘ encode = id`
  -- (for the backward direction / statement-soundness); the `*-sound`
  -- laws give `encode ∘ decode = id` on raw data (used by the
  -- communications-commitment faithfulness case, where the in-circuit
  -- preimage re-encodes the decoded inputs and must recover the raw
  -- input stream).  Native carries no law — its encoding is the identity.
  --
  -- The `secp*-sound` laws pin the decoders to the canonical partial
  -- inverses — see the TRUST NOTE at the limb-encoding fields above.
  ----------------------------------------------------------------------

  field
    coordsJ-fromCoordsJ : ∀ p → uncurry fromCoordsJ (coordsJ p) ≡ just p
    fromCoordsJ-coordsJ : ∀ {x y p}
      → fromCoordsJ x y ≡ just p → coordsJ p ≡ (x , y)

    jubjubScalar-round : ∀ s
      → jubjubScalarFromFr (jubjubScalarToFr s) ≡ just s
    jubjubScalar-sound : ∀ {f s}
      → jubjubScalarFromFr f ≡ just s → jubjubScalarToFr s ≡ f

    bytes32-round : ∀ b
      → uncurry low-high→bytes32 (bytes32→low-high b) ≡ just b
    bytes32-sound : ∀ {lo hi b}
      → low-high→bytes32 lo hi ≡ just b → bytes32→low-high b ≡ (lo , hi)

    secp256k1Base-round : ∀ x
      → uncurry limbs→secp256k1Base (secp256k1Base→limbs x) ≡ just x
    secp256k1Base-sound : ∀ {l h x}
      → limbs→secp256k1Base l h ≡ just x → secp256k1Base→limbs x ≡ (l , h)

    secp256k1Scalar-round : ∀ s
      → uncurry limbs→secp256k1Scalar (secp256k1Scalar→limbs s) ≡ just s
    secp256k1Scalar-sound : ∀ {l h s}
      → limbs→secp256k1Scalar l h ≡ just s → secp256k1Scalar→limbs s ≡ (l , h)

    secp256k1Point-round : ∀ {p a b c d e}
      → secp256k1Point→limbs p ≡ (a , b , c , d , e)
      → limbs→secp256k1Point a b c d e ≡ just p
    secp256k1Point-sound : ∀ {a b c d e p}
      → limbs→secp256k1Point a b c d e ≡ just p
      → secp256k1Point→limbs p ≡ (a , b , c , d , e)

    -- Coordinate round-trips (both directions partial: `coordsK1` on the
    -- identity, `fromCoordsK1` off-curve).
    coordsK1-fromCoordsK1 : ∀ {p x y}
      → coordsK1 p ≡ just (x , y) → fromCoordsK1 x y ≡ just p
    fromCoordsK1-coordsK1 : ∀ {x y p}
      → fromCoordsK1 x y ≡ just p → coordsK1 p ≡ just (x , y)

    secp256r1Base-round : ∀ x
      → uncurry limbs→secp256r1Base (secp256r1Base→limbs x) ≡ just x
    secp256r1Base-sound : ∀ {l h x}
      → limbs→secp256r1Base l h ≡ just x → secp256r1Base→limbs x ≡ (l , h)

    secp256r1Scalar-round : ∀ s
      → uncurry limbs→secp256r1Scalar (secp256r1Scalar→limbs s) ≡ just s
    secp256r1Scalar-sound : ∀ {l h s}
      → limbs→secp256r1Scalar l h ≡ just s
      → secp256r1Scalar→limbs s ≡ (l , h)

    secp256r1Point-round : ∀ {p a b c d e}
      → secp256r1Point→limbs p ≡ (a , b , c , d , e)
      → limbs→secp256r1Point a b c d e ≡ just p
    secp256r1Point-sound : ∀ {a b c d e p}
      → limbs→secp256r1Point a b c d e ≡ just p
      → secp256r1Point→limbs p ≡ (a , b , c , d , e)

    curve25519Base-round : ∀ x
      → uncurry limbs→curve25519Base (curve25519Base→limbs x) ≡ just x
    curve25519Base-sound : ∀ {l h x}
      → limbs→curve25519Base l h ≡ just x → curve25519Base→limbs x ≡ (l , h)

    curve25519Scalar-round : ∀ s
      → uncurry limbs→curve25519Scalar (curve25519Scalar→limbs s) ≡ just s
    curve25519Scalar-sound : ∀ {l h s}
      → limbs→curve25519Scalar l h ≡ just s
      → curve25519Scalar→limbs s ≡ (l , h)

    curve25519Point-round : ∀ {p a b c d}
      → curve25519Point→limbs p ≡ (a , b , c , d)
      → limbs→curve25519Point a b c d ≡ just p
    curve25519Point-sound : ∀ {a b c d p}
      → limbs→curve25519Point a b c d ≡ just p
      → curve25519Point→limbs p ≡ (a , b , c , d)

    -- No coordinate round-trip laws: no proof consumes them (the point
    -- wire-encoding goes through limbs, with its own round-trip laws
    -- above, so — unlike Jubjub, whose point encoding IS its coordinate
    -- pair — the coordinate functions never need to compose).
