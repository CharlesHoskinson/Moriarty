{-# OPTIONS --safe #-}
open import zkir-v3.Assumptions

------------------------------------------------------------------------
-- Static single-assignment well-formedness for a zkir-v3 `IrSource`.
--
-- The forward faithfulness theorem `forward` (in `CircuitFaithfulness`)
-- has two producer-side hypotheses that are not intrinsic to a run:
--
--   • `WF-run P S (instructions S) st0`  — at every step the outputs the
--     instruction binds are fresh (unbound in the memory it sees) and, for
--     multi-output instructions, pairwise distinct; and
--   • `NoDup (map name (inputs S))`      — the declared input names are
--     distinct.
--
-- Both are consequences of a purely static property of the source: it is
-- in single-assignment form.  This module states that property as a
-- decidable predicate `producer-SA` — a linear scan tracking the set of
-- already-bound names — and proves `producer-safe→WF`, discharging the
-- semantic `WF-run` hypothesis from it.  The clean corollary `forward-sa`
-- then depends only on the static check (and the run's success).
--
-- The invariant that connects the static "bound name set" to the dynamic
-- memory is `DomEq m bound`: the memory's domain is exactly `bound`.  It
-- is seeded from `init` (`init-dom`: the initial memory binds exactly the
-- input names) and maintained across each step (`step-dom`: a step extends
-- the domain by exactly the names `outs-of i`).
------------------------------------------------------------------------

module zkir-v3.Obligations (⋯ : _) (open Assumptions ⋯) where

open import zkir-v3.Types ⋯
open import zkir-v3.Syntax ⋯
open import zkir-v3.Semantics ⋯
  using ( Mem; State; ProofPreimage; ins; step; init; out1; run
        ; insertMany; decode-inputs; resolve; resolveᶠ; resolve𝔹
        ; resolve-all-Fr; eval-guard; collectOutputs; valEq?; _≟LFr_
        ; default-val; _≟B_; to𝔹; preprocess; guardD )
open import zkir-v3.SemanticsProperties ⋯
  using ( AllFresh; NotIn; NoDup; out-fresh; WF-run
        ; ins-here; ins-other; resolveᶠ-intro
        ; _⊑_; _≼_; ⊑-refl; ⊑-trans; ≼-refl; ≼-trans
        ; run-cons; run-shaped; mk-run-shaped; Consumed
        ; run-extends; init-outs; init-decode )
open import zkir-v3.Encoding ⋯ renaming (encode to encodeᵉ)

open import Data.Bool    using (Bool; true; false; if_then_else_; T)
open import Data.Bool.Properties using (T?)
open import Data.List    using (List; []; _∷_; _++_; _∷ʳ_; map; take; drop; length; concatMap)
open import Data.List.Properties using (++-identityʳ)
open import Data.Maybe   using (Maybe; just; nothing; _>>=_)
open import Data.Nat     using (ℕ; _^_; _+_; _*_; _∸_; _<_; _<?_; _<ᵇ_; _≤ᵇ_)
open import Data.Nat     using () renaming (_≟_ to _≟ℕ_)
open import Data.Product using (_×_; _,_; ∃; ∃₂; proj₁; proj₂)
open import Function.Bundles using (_⇔_; mk⇔)
open import Data.Unit    using (⊤; tt)
open import Function     using (case_of_)
open import Relation.Binary.PropositionalEquality
  using (_≡_; refl; sym; trans; cong; cong₂; subst)
open import Relation.Nullary using (¬_; yes; no; Dec)
open import Relation.Nullary.Decidable
  using (¬?; _×-dec_; _⊎-dec_; map′; isYes≗does; dec-true)
open import Data.String using () renaming (_≟_ to _≟str_)

open import Data.List.Relation.Unary.All using (All; []; _∷_; all?)
open import Data.List.Membership.DecPropositional _≟str_ using (_∈?_)
open import Data.List.Relation.Unary.Any using (Any; here; there)
open import Data.List.Membership.Propositional using (_∈_)
open import Data.List.Membership.Propositional.Properties
  using (∈-++⁻; ∈-++⁺ˡ; ∈-++⁺ʳ)
open import Data.Sum using (_⊎_; inj₁; inj₂)
open import Data.Empty using (⊥)
open import Data.Maybe using () renaming (map to mapᵐ)
open import Data.Maybe.Properties using (just-injective; ≡-dec)

------------------------------------------------------------------------
-- The output identifiers each instruction binds.
--
-- Exactly the names appearing as memory-write targets in `step`; mirrors
-- the shape of `out-fresh`.  No-output instructions bind nothing.
------------------------------------------------------------------------

outs-of : Instruction → List Identifier
outs-of (encode _ outputs)                 = outputs
outs-of (assert _)                         = []
outs-of (cond-select _ _ _ o)              = o ∷ []
outs-of (constrain-bits _ _)               = []
outs-of (constrain-eq _ _)                 = []
outs-of (constrain-to-boolean _)           = []
outs-of (copy _ o)                         = o ∷ []
outs-of (impact _ _)                       = []
outs-of (ec-mul _ _ o)                     = o ∷ []
outs-of (ec-mul-generator _ o)             = o ∷ []
outs-of (hash-to-curve _ o)                = o ∷ []
outs-of (into-coordinates _ (xo , yo))     = xo ∷ yo ∷ []
outs-of (from-coordinates _ o)             = o ∷ []
outs-of (into-bytes32 _ o)                 = o ∷ []
outs-of (from-bytes32 _ _ o)               = o ∷ []
outs-of (reverse-bytes _ o)                = o ∷ []
outs-of (bytes32-into-low-high _ (lo , hi)) = lo ∷ hi ∷ []
outs-of (bytes32-from-low-high _ o)        = o ∷ []
outs-of (div-mod-power-of-two _ _ outs)    = outs
outs-of (reconstitute-field _ _ _ o)       = o ∷ []
outs-of (transient-hash _ o)               = o ∷ []
outs-of (persistent-hash _ _ o)            = o ∷ []
outs-of (keccak256 _ _ o)                  = o ∷ []
outs-of (test-eq _ _ o)                    = o ∷ []
outs-of (add _ _ o)                        = o ∷ []
outs-of (mul _ _ o)                        = o ∷ []
outs-of (neg _ o)                          = o ∷ []
outs-of (inv _ o)                          = o ∷ []
outs-of (not _ o)                          = o ∷ []
outs-of (less-than _ _ _ o)                = o ∷ []
outs-of (jubjub-scalar-from-native _ o)    = o ∷ []
outs-of (public-input _ _ o)               = o ∷ []
outs-of (private-input _ _ o)              = o ∷ []
outs-of (circuit-output _)                 = []

------------------------------------------------------------------------
-- Domain-tracking invariant: the memory's domain is exactly `bound`.
------------------------------------------------------------------------

-- The memory's domain is contained in `bound`: every identifier outside
-- `bound` is unbound.  Only this "off" direction is consumed downstream
-- (`out-fresh-of`/`allFresh-of` turn "not bound ⇒ unbound in memory" into
-- freshness); it is self-contained (needs no key-freshness to maintain).
DomEq : Mem → List Identifier → Set
DomEq m bound = ∀ {id} → ¬ (id ∈ bound) → m id ≡ nothing

-- A member of a singleton is the element.
∈-[x] : ∀ {id o : Identifier} → id ∈ (o ∷ []) → id ≡ o
∈-[x] (here p) = p

-- The empty domain matches the empty memory.
domEq-∅ : ∀ {m} → (∀ id → m id ≡ nothing) → DomEq m []
domEq-∅ empty {id} _ = empty id

------------------------------------------------------------------------
-- Extending the domain by one fresh binding.
------------------------------------------------------------------------

domEq-ins : ∀ {m bound} (o : Identifier) v
  → DomEq m bound → DomEq (ins o v m) (bound ++ (o ∷ []))
domEq-ins {m} {bound} o v d {id} id∉ =
  let id∉bound = λ p → id∉ (∈-++⁺ˡ p)
      id≢o     = λ id≡o → id∉ (∈-++⁺ʳ bound (here id≡o))
  in trans (ins-other v m id≢o) (d id∉bound)

-- Transport a `DomEq` along an equality of the bound-name list.
domEq-cast : ∀ {m bs cs} → bs ≡ cs → DomEq m bs → DomEq m cs
domEq-cast refl d = d

-- Prepending one binding to the domain.  Unlike `domEq-ins` (which
-- appends), this frames the fresh cell at the *front* of the name list —
-- the shape `decode-inputs` produces.  No freshness is required: binding
-- `o` covers it whether or not `o` was already in the tail's domain.
domEq-cons : ∀ {m bound} (o : Identifier) v
  → DomEq m bound → DomEq (ins o v m) (o ∷ bound)
domEq-cons {m} {bound} o v d {id} id∉ =
  let id≢o     = λ id≡o → id∉ (here id≡o)
      id∉bound = λ p → id∉ (there p)
  in trans (ins-other v m id≢o) (d id∉bound)

-- `NotIn` read as plain non-membership.
notIn-∉ : ∀ {id} ks → NotIn id ks → ¬ (id ∈ ks)
notIn-∉ (k ∷ ks) (id≢k , nin) (here id≡k) = id≢k id≡k
notIn-∉ (k ∷ ks) (id≢k , nin) (there p)   = notIn-∉ ks nin p

open import Data.List.Properties using (++-assoc)

-- Extending the domain by a whole `insertMany`.  Mirrors `insertMany`
-- (and `insertMany-⊑`); the "off" direction needs no key-freshness (an
-- id outside `bound ++ ids` avoids every inserted key and stays unbound).
domEq-insertMany : ∀ {bound} st ids vs {st'}
  → insertMany st ids vs ≡ just st'
  → DomEq (State.mem st) bound
  → DomEq (State.mem st') (bound ++ ids)
domEq-insertMany {bound} st []         []         refl d =
  domEq-cast (sym (++-identityʳ bound)) d
domEq-insertMany {bound} st (id ∷ ids) (v ∷ vs)   e    d =
  domEq-cast (++-assoc bound (id ∷ []) ids)
    (domEq-insertMany {bound ++ (id ∷ [])} (out1 st id v) ids vs e
      (domEq-ins id v d))
domEq-insertMany st []        (_ ∷ _)  ()  _
domEq-insertMany st (_ ∷ _)   []       ()  _

-- Extending the domain by two bindings (the `out1 ∘ out1` shape of the
-- two-output instructions).  A specialisation of `domEq-insertMany` at a
-- two-element list (the states coincide definitionally).
domEq-ins2 : ∀ {bound} st (i₁ : Identifier) v₁ (i₂ : Identifier) v₂
  → DomEq (State.mem st) bound
  → DomEq (State.mem (out1 (out1 st i₁ v₁) i₂ v₂)) (bound ++ (i₁ ∷ i₂ ∷ []))
domEq-ins2 {bound} st i₁ v₁ i₂ v₂ d =
  domEq-insertMany st (i₁ ∷ i₂ ∷ []) (v₁ ∷ v₂ ∷ []) refl d

------------------------------------------------------------------------
-- Seeding the invariant from `init`.
--
-- `decode-inputs` binds exactly the declared input names, so the initial
-- memory's domain is `map name (inputs S)`.  `init-decode` (from
-- `SemanticsProperties`) says the initial memory is that decoding.
------------------------------------------------------------------------

decode-inputs-dom : ∀ tis raw {m}
  → decode-inputs tis raw ≡ just m
  → DomEq m (map TypedIdentifier.name tis)
decode-inputs-dom []         []      refl = domEq-∅ (λ _ → refl)
decode-inputs-dom []         (_ ∷ _) ()
decode-inputs-dom (ti ∷ tis) raw ieq
  with decode (TypedIdentifier.val-t ti)
              (take (encoded-len (TypedIdentifier.val-t ti)) raw)
... | just v
  with decode-inputs tis (drop (encoded-len (TypedIdentifier.val-t ti)) raw)
         in erest
...   | just m' with refl ← ieq =
        domEq-cons (TypedIdentifier.name ti) v
          (decode-inputs-dom tis _ erest)

init-dom : ∀ {S P st0}
  → init S P ≡ just st0
  → DomEq (State.mem st0) (map TypedIdentifier.name (IrSource.inputs S))
init-dom {S} {P} {st0} ieq =
  decode-inputs-dom (IrSource.inputs S) (ProofPreimage.inputs P)
    (init-decode {S} {P} {st0} ieq)

------------------------------------------------------------------------
-- Converting static output freshness (w.r.t. `bound`) into the semantic
-- `out-fresh` (freshness w.r.t. the memory).
--
-- The `DomEq`'s second component turns "not in the bound set" into
-- "unbound in memory"; the static `NoDup (outs-of i)` supplies the
-- within-instruction distinctness the multi-output cases ask for.
------------------------------------------------------------------------

-- Every statically-fresh name is unbound in a domain-matching memory.
allFresh-of : ∀ {m bound} ids
  → DomEq m bound → All (λ o → ¬ (o ∈ bound)) ids → AllFresh ids m
allFresh-of []         _ _          = tt
allFresh-of (o ∷ ids)  d (o∉ ∷ frs) = d o∉ , allFresh-of ids d frs

out-fresh-of : ∀ {m bound} (i : Instruction)
  → DomEq m bound
  → All (λ o → ¬ (o ∈ bound)) (outs-of i)
  → NoDup (outs-of i)
  → out-fresh i m
out-fresh-of (encode _ outputs)  d af nd = allFresh-of outputs d af , nd
out-fresh-of (assert _)                    d af nd = tt
out-fresh-of (cond-select _ _ _ o)         d        (o∉ ∷ _) _ = d o∉
out-fresh-of (constrain-bits _ _)          d af nd = tt
out-fresh-of (constrain-eq _ _)            d af nd = tt
out-fresh-of (constrain-to-boolean _)      d af nd = tt
out-fresh-of (copy _ o)                    d        (o∉ ∷ _) _ = d o∉
out-fresh-of (impact _ _)                  d af nd = tt
out-fresh-of (ec-mul _ _ o)                d        (o∉ ∷ _) _ = d o∉
out-fresh-of (ec-mul-generator _ o)        d        (o∉ ∷ _) _ = d o∉
out-fresh-of (hash-to-curve _ o)           d        (o∉ ∷ _) _ = d o∉
out-fresh-of (into-coordinates _ (xo , yo)) d
  (xo∉ ∷ yo∉ ∷ []) ((xo≢yo , _) , _) = d xo∉ , d yo∉ , xo≢yo
out-fresh-of (from-coordinates _ o)        d        (o∉ ∷ _) _ = d o∉
out-fresh-of (into-bytes32 _ o)            d        (o∉ ∷ _) _ = d o∉
out-fresh-of (from-bytes32 _ _ o)          d        (o∉ ∷ _) _ = d o∉
out-fresh-of (reverse-bytes _ o)           d        (o∉ ∷ _) _ = d o∉
out-fresh-of (bytes32-into-low-high _ (lo , hi)) d
  (lo∉ ∷ hi∉ ∷ []) ((lo≢hi , _) , _) = d lo∉ , d hi∉ , lo≢hi
out-fresh-of (bytes32-from-low-high _ o)   d        (o∉ ∷ _) _ = d o∉
out-fresh-of (div-mod-power-of-two _ _ outs) d af nd = allFresh-of outs d af , nd
out-fresh-of (reconstitute-field _ _ _ o)  d        (o∉ ∷ _) _ = d o∉
out-fresh-of (transient-hash _ o)          d        (o∉ ∷ _) _ = d o∉
out-fresh-of (persistent-hash _ _ o)       d        (o∉ ∷ _) _ = d o∉
out-fresh-of (keccak256 _ _ o)             d        (o∉ ∷ _) _ = d o∉
out-fresh-of (test-eq _ _ o)               d        (o∉ ∷ _) _ = d o∉
out-fresh-of (add _ _ o)                   d        (o∉ ∷ _) _ = d o∉
out-fresh-of (mul _ _ o)                   d        (o∉ ∷ _) _ = d o∉
out-fresh-of (neg _ o)                     d        (o∉ ∷ _) _ = d o∉
out-fresh-of (inv _ o)                     d        (o∉ ∷ _) _ = d o∉
out-fresh-of (not _ o)                     d        (o∉ ∷ _) _ = d o∉
out-fresh-of (less-than _ _ _ o)           d        (o∉ ∷ _) _ = d o∉
out-fresh-of (jubjub-scalar-from-native _ o) d        (o∉ ∷ _) _ = d o∉
out-fresh-of (public-input _ _ o)          d        (o∉ ∷ _) _ = d o∉
out-fresh-of (private-input _ _ o)         d        (o∉ ∷ _) _ = d o∉
out-fresh-of (circuit-output _)            d af nd = tt

------------------------------------------------------------------------
-- Domain maintenance across one step.
--
-- Inverting `step` per instruction (mirroring `step-mem-⊑`), the memory
-- grows by exactly the names `outs-of i`: no-output instructions leave it
-- unchanged (`bound ++ [] = bound`), single-output instructions bind one
-- fresh cell (`domEq-ins`), two-output ones two (`domEq-ins2`), and the
-- `insertMany` instructions a whole fresh, distinct list
-- (`domEq-insertMany`).
------------------------------------------------------------------------

-- The no-output shape: memory and domain both unchanged.
domEq-nop : ∀ {m bound} → DomEq m bound → DomEq m (bound ++ [])
domEq-nop {bound = bound} d = domEq-cast (sym (++-identityʳ bound)) d

step-dom : ∀ {P S st st' bound} (i : Instruction)
  → step P S st i ≡ just st'
  → DomEq (State.mem st) bound
  → DomEq (State.mem st') (bound ++ outs-of i)
step-dom {st = st} (encode input outputs) e d
  with resolve (State.mem st) input
... | just v = domEq-insertMany st outputs (map val-native (encodeᵉ v)) e d
step-dom {st = st} (assert cond) e d
  with resolve𝔹 (State.mem st) cond
... | just true with refl ← e = domEq-nop d
step-dom {st = st} (cond-select bit a b output) e d
  with resolve𝔹 (State.mem st) bit
... | just bv with resolve (State.mem st) a | resolve (State.mem st) b
...   | just av | just bvl with typeof av ≟T typeof bvl
...     | yes _ with refl ← e = domEq-ins output _ d
step-dom {st = st} (constrain-bits val bits) e d
  with resolveᶠ (State.mem st) val
... | just x with valFr x <? 2 ^ bits
...   | yes _ with refl ← e = domEq-nop d
step-dom {st = st} (constrain-eq a b) e d
  with resolve (State.mem st) a | resolve (State.mem st) b
... | just av | just bv with valEq? av bv
...   | just true with refl ← e = domEq-nop d
step-dom {st = st} (constrain-to-boolean val) e d
  with resolve𝔹 (State.mem st) val
... | just _ with refl ← e = domEq-nop d
step-dom {st = st} (copy val output) e d
  with resolve (State.mem st) val
... | just v with refl ← e = domEq-ins output _ d
step-dom {P = P} {st = st} (impact guard inputs) e d
  with resolve-all-Fr (State.mem st) inputs
... | just vals with resolve𝔹 (State.mem st) guard
...   | just g with g
...     | true
          with take (length vals)
                 (drop (State.pti-idx st)
                   (ProofPreimage.pub-transcript-inputs P))
               ≟LFr vals
...       | yes _ with refl ← e = domEq-nop d
step-dom {st = st} (impact guard inputs) e d | just vals | just g | false
  with refl ← e = domEq-nop d
step-dom {st = st} (ec-mul a scalar output) e d
  with resolve (State.mem st) a | resolve (State.mem st) scalar
... | just (val-jubjub-point p) | just (val-jubjub-scalar s)
        with refl ← e = domEq-ins output _ d
step-dom {st = st} (ec-mul a scalar output) e d
  | just (val-secp256k1-point p) | just (val-secp256k1-scalar s)
        with refl ← e = domEq-ins output _ d
step-dom {st = st} (ec-mul a scalar output) e d
  | just (val-secp256r1-point p) | just (val-secp256r1-scalar s)
        with refl ← e = domEq-ins output _ d
step-dom {st = st} (ec-mul a scalar output) e d
  | just (val-curve25519-point p) | just (val-curve25519-scalar s)
        with refl ← e = domEq-ins output _ d
step-dom {st = st} (ec-mul-generator scalar output) e d
  with resolve (State.mem st) scalar
... | just (val-jubjub-scalar s) with refl ← e = domEq-ins output _ d
step-dom {st = st} (ec-mul-generator scalar output) e d
  | just (val-secp256k1-scalar s) with refl ← e = domEq-ins output _ d
step-dom {st = st} (hash-to-curve inputs output) e d
  with resolve-all-Fr (State.mem st) inputs
... | just frs with refl ← e = domEq-ins output _ d
step-dom {st = st} (into-coordinates point (xid , yid)) e d
  with resolve (State.mem st) point
... | just (val-jubjub-point p) with coordsJ p
...   | (x , y) with refl ← e =
          domEq-ins2 st xid (val-native x) yid (val-native y) d
step-dom {st = st} (into-coordinates point (xid , yid)) e d
  | just (val-secp256k1-point p) with coordsK1 p
...   | just (x , y) with refl ← e =
          domEq-ins2 st xid (val-secp256k1-base x) yid (val-secp256k1-base y) d
step-dom {st = st} (into-coordinates point (xid , yid)) e d
  | just (val-secp256r1-point p) with coordsP p
...   | just (x , y) with refl ← e =
          domEq-ins2 st xid (val-secp256r1-base x) yid (val-secp256r1-base y) d
step-dom {st = st} (into-coordinates point (xid , yid)) e d
  | just (val-curve25519-point p) with coordsC p
...   | (x , y) with refl ← e =
          domEq-ins2 st xid (val-curve25519-base x)
                        yid (val-curve25519-base y) d
step-dom {st = st} (from-coordinates (xop , yop) output) e d
  with resolve (State.mem st) xop | resolve (State.mem st) yop
... | just (val-native x) | just (val-native y) with fromCoordsJ x y
...   | just p with refl ← e = domEq-ins output _ d
step-dom {st = st} (from-coordinates (xop , yop) output) e d
  | just (val-secp256k1-base x) | just (val-secp256k1-base y) with fromCoordsK1 x y
...   | just p with refl ← e = domEq-ins output _ d
step-dom {st = st} (from-coordinates (xop , yop) output) e d
  | just (val-secp256r1-base x) | just (val-secp256r1-base y)
        with fromCoordsP x y
...   | just p with refl ← e = domEq-ins output _ d
step-dom {st = st} (from-coordinates (xop , yop) output) e d
  | just (val-curve25519-base x) | just (val-curve25519-base y)
        with fromCoordsC x y
...   | just p with refl ← e = domEq-ins output _ d
step-dom {st = st} (into-bytes32 input output) e d
  with resolve (State.mem st) input
... | just (val-native x) with refl ← e = domEq-ins output _ d
... | just (val-secp256k1-base x) with refl ← e = domEq-ins output _ d
... | just (val-secp256k1-scalar s) with refl ← e = domEq-ins output _ d
... | just (val-secp256r1-base x) with refl ← e = domEq-ins output _ d
... | just (val-secp256r1-scalar s) with refl ← e = domEq-ins output _ d
... | just (val-curve25519-base x) with refl ← e = domEq-ins output _ d
... | just (val-curve25519-scalar s) with refl ← e = domEq-ins output _ d
step-dom {st = st} (from-bytes32 bytes native output) e d
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) with refl ← e = domEq-ins output _ d
step-dom {st = st} (from-bytes32 bytes secp256k1-base output) e d
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) with refl ← e = domEq-ins output _ d
step-dom {st = st} (from-bytes32 bytes secp256k1-scalar output) e d
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) with refl ← e = domEq-ins output _ d
step-dom {st = st} (from-bytes32 bytes secp256r1-base output) e d
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) with refl ← e = domEq-ins output _ d
step-dom {st = st} (from-bytes32 bytes secp256r1-scalar output) e d
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) with refl ← e = domEq-ins output _ d
step-dom {st = st} (from-bytes32 bytes curve25519-base output) e d
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) with refl ← e = domEq-ins output _ d
step-dom {st = st} (from-bytes32 bytes curve25519-scalar output) e d
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) with refl ← e = domEq-ins output _ d
step-dom {st = st} (from-bytes32 bytes bytes32 output) e d
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) = case e of λ ()
step-dom {st = st} (from-bytes32 bytes jubjub-point output) e d
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) = case e of λ ()
step-dom {st = st} (from-bytes32 bytes jubjub-scalar output) e d
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) = case e of λ ()
step-dom {st = st} (from-bytes32 bytes secp256k1-point output) e d
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) = case e of λ ()
step-dom {st = st} (from-bytes32 bytes secp256r1-point output) e d
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) = case e of λ ()
step-dom {st = st} (from-bytes32 bytes curve25519-point output) e d
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) = case e of λ ()
step-dom {st = st} (reverse-bytes bytes output) e d
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) with refl ← e = domEq-ins output _ d
step-dom {st = st} (bytes32-into-low-high bytes (loid , hiid)) e d
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) with bytes32→low-high b
...   | (lo , hi) with refl ← e =
          domEq-ins2 st loid (val-native lo) hiid (val-native hi) d
step-dom {st = st} (bytes32-from-low-high (loop , hiop) output) e d
  with resolveᶠ (State.mem st) loop | resolveᶠ (State.mem st) hiop
... | just lo | just hi with low-high→bytes32 lo hi
...   | just b with refl ← e = domEq-ins output _ d
step-dom {st = st} (div-mod-power-of-two val bits outs) e d
  with resolveᶠ (State.mem st) val
... | just x =
        domEq-insertMany st outs
          ( val-native (from-le-bits (drop bits (to-le-bits x)))
          ∷ val-native (from-le-bits (take bits (to-le-bits x))) ∷ [])
          e d
step-dom {st = st} (reconstitute-field divisor modulus bits output) e d
  with resolveᶠ (State.mem st) divisor | resolveᶠ (State.mem st) modulus
... | just dd | just mo with valFr mo <? 2 ^ bits
...   | yes _ with valFr dd <? 2 ^ (FR-BITS ∸ bits)
...     | yes _ with valFr mo + 2 ^ bits * valFr dd <? FR-ORDER
...       | yes _ with refl ← e = domEq-ins output _ d
step-dom {st = st} (transient-hash inputs output) e d
  with resolve-all-Fr (State.mem st) inputs
... | just frs with refl ← e = domEq-ins output _ d
step-dom {st = st} (persistent-hash alignment inputs output) e d
  with resolve-all-Fr (State.mem st) inputs
... | just frs with persistent-hash-fn alignment frs
...   | just h with refl ← e = domEq-ins output _ d
step-dom {st = st} (keccak256 alignment inputs output) e d
  with resolve-all-Fr (State.mem st) inputs
... | just frs with keccak-fn alignment frs
...   | just h with refl ← e = domEq-ins output _ d
step-dom {st = st} (test-eq a b output) e d
  with resolve (State.mem st) a | resolve (State.mem st) b
... | just av | just bv with valEq? av bv
...   | just eqv with refl ← e = domEq-ins output _ d
step-dom {st = st} (add a b output) e d
  with resolve (State.mem st) a | resolve (State.mem st) b
... | just (val-native x) | just (val-native y)
        with refl ← e = domEq-ins output _ d
... | just (val-jubjub-point p) | just (val-jubjub-point q)
        with refl ← e = domEq-ins output _ d
... | just (val-secp256k1-point p) | just (val-secp256k1-point q)
        with refl ← e = domEq-ins output _ d
... | just (val-secp256k1-base x) | just (val-secp256k1-base y)
        with refl ← e = domEq-ins output _ d
... | just (val-secp256k1-scalar x) | just (val-secp256k1-scalar y)
        with refl ← e = domEq-ins output _ d
... | just (val-secp256r1-point p) | just (val-secp256r1-point q)
        with refl ← e = domEq-ins output _ d
... | just (val-secp256r1-base x) | just (val-secp256r1-base y)
        with refl ← e = domEq-ins output _ d
... | just (val-secp256r1-scalar x) | just (val-secp256r1-scalar y)
        with refl ← e = domEq-ins output _ d
... | just (val-curve25519-point p) | just (val-curve25519-point q)
        with refl ← e = domEq-ins output _ d
... | just (val-curve25519-base x) | just (val-curve25519-base y)
        with refl ← e = domEq-ins output _ d
... | just (val-curve25519-scalar x) | just (val-curve25519-scalar y)
        with refl ← e = domEq-ins output _ d
step-dom {st = st} (mul a b output) e d
  with resolve (State.mem st) a | resolve (State.mem st) b
... | just (val-native x) | just (val-native y)
        with refl ← e = domEq-ins output _ d
... | just (val-secp256k1-base x) | just (val-secp256k1-base y)
        with refl ← e = domEq-ins output _ d
... | just (val-secp256k1-scalar x) | just (val-secp256k1-scalar y)
        with refl ← e = domEq-ins output _ d
... | just (val-secp256r1-base x) | just (val-secp256r1-base y)
        with refl ← e = domEq-ins output _ d
... | just (val-secp256r1-scalar x) | just (val-secp256r1-scalar y)
        with refl ← e = domEq-ins output _ d
... | just (val-curve25519-base x) | just (val-curve25519-base y)
        with refl ← e = domEq-ins output _ d
... | just (val-curve25519-scalar x) | just (val-curve25519-scalar y)
        with refl ← e = domEq-ins output _ d
step-dom {st = st} (neg a output) e d
  with resolve (State.mem st) a
... | just (val-native x) with refl ← e = domEq-ins output _ d
... | just (val-jubjub-point p) with refl ← e = domEq-ins output _ d
... | just (val-secp256k1-point p) with refl ← e = domEq-ins output _ d
... | just (val-secp256k1-base x) with refl ← e = domEq-ins output _ d
... | just (val-secp256k1-scalar x) with refl ← e = domEq-ins output _ d
... | just (val-secp256r1-point p) with refl ← e = domEq-ins output _ d
... | just (val-secp256r1-base x) with refl ← e = domEq-ins output _ d
... | just (val-secp256r1-scalar x) with refl ← e = domEq-ins output _ d
... | just (val-curve25519-point p) with refl ← e = domEq-ins output _ d
... | just (val-curve25519-base x) with refl ← e = domEq-ins output _ d
... | just (val-curve25519-scalar x) with refl ← e = domEq-ins output _ d
step-dom {st = st} (inv a output) e d
  with resolve (State.mem st) a
... | just (val-native x) with invᶠ x
...   | just xi with refl ← e = domEq-ins output _ d
step-dom {st = st} (inv a output) e d
  | just (val-secp256k1-base x) with invK1ᵇ x
...   | just xi with refl ← e = domEq-ins output _ d
step-dom {st = st} (inv a output) e d
  | just (val-secp256k1-scalar x) with invK1ˢ x
...   | just xi with refl ← e = domEq-ins output _ d
step-dom {st = st} (inv a output) e d
  | just (val-secp256r1-base x) with invPᵇ x
...   | just xi with refl ← e = domEq-ins output _ d
step-dom {st = st} (inv a output) e d
  | just (val-secp256r1-scalar x) with invPˢ x
...   | just xi with refl ← e = domEq-ins output _ d
step-dom {st = st} (inv a output) e d
  | just (val-curve25519-base x) with invCᵇ x
...   | just xi with refl ← e = domEq-ins output _ d
step-dom {st = st} (inv a output) e d
  | just (val-curve25519-scalar x) with invCˢ x
...   | just xi with refl ← e = domEq-ins output _ d
step-dom {st = st} (not a output) e d
  with resolve𝔹 (State.mem st) a
... | just b with refl ← e = domEq-ins output _ d
step-dom {st = st} (less-than a b bits output) e d
  with resolveᶠ (State.mem st) a | resolveᶠ (State.mem st) b
... | just x | just y with valFr x <? 2 ^ bits
...   | yes _ with valFr y <? 2 ^ bits
...     | yes _ with refl ← e = domEq-ins output _ d
step-dom {st = st} (jubjub-scalar-from-native a output) e d
  with resolveᶠ (State.mem st) a
... | just x with refl ← e = domEq-ins output _ d
step-dom {st = st} (public-input guard val-t output) e d
  with eval-guard (State.mem st) guard
... | just g with g
...   | true with decode val-t
                    (take (encoded-len val-t) (State.pto-rem st))
...     | just v with refl ← e = domEq-ins output _ d
step-dom {st = st} (public-input guard val-t output) e d
    | just g | false with refl ← e = domEq-ins output _ d
step-dom {st = st} (private-input guard val-t output) e d
  with eval-guard (State.mem st) guard
... | just g with g
...   | true with decode val-t
                    (take (encoded-len val-t) (State.priv-rem st))
...     | just v with refl ← e = domEq-ins output _ d
step-dom {st = st} (private-input guard val-t output) e d
    | just g | false with refl ← e = domEq-ins output _ d
step-dom {S = S} {st = st} (circuit-output vals) e d
  with collectOutputs (State.mem st) (IrSource.outputs S) vals
... | just vs with refl ← e = domEq-nop d

------------------------------------------------------------------------
-- The static single-assignment predicate and its soundness.
--
-- `SA bound is` scans `is` from an already-bound name set `bound`: each
-- instruction's outputs must be fresh against `bound` and pairwise
-- distinct, and the scan continues with `bound` extended by those
-- outputs.  It is decidable (a nesting of `_∈?_`/`all?` decisions), though
-- only the `Set` form is needed downstream.
------------------------------------------------------------------------

SA : List Identifier → List Instruction → Set
SA bound []       = ⊤
SA bound (i ∷ is) = All (λ o → ¬ (o ∈ bound)) (outs-of i)
                  × NoDup (outs-of i)
                  × SA (bound ++ outs-of i) is

producer-SA : IrSource → Set
producer-SA S = NoDup (map TypedIdentifier.name (IrSource.inputs S))
              × SA (map TypedIdentifier.name (IrSource.inputs S))
                   (IrSource.instructions S)

-- The domain invariant + `SA` imply `WF-run`: at the head, `out-fresh-of`
-- turns the static freshness into the semantic one; along the tail,
-- `step-dom` re-establishes the invariant at the enlarged bound.
sa-wf : ∀ {P S} is st bound
  → SA bound is → DomEq (State.mem st) bound → WF-run P S is st
sa-wf []       st bound _              _ = tt
sa-wf {P} {S} (i ∷ is) st bound (af , nd , sa-rest) d =
    out-fresh-of i d af nd
  , λ {st'} step-eq →
      sa-wf {P} {S} is st' (bound ++ outs-of i) sa-rest
        (step-dom {P} {S} i step-eq d)

producer-safe→WF : ∀ {S P st0}
  → producer-SA S
  → init S P ≡ just st0
  → WF-run P S (IrSource.instructions S) st0
producer-safe→WF {S} {P} {st0} (_ , sa) ieq =
  sa-wf (IrSource.instructions S) st0
    (map TypedIdentifier.name (IrSource.inputs S)) sa
    (init-dom {S} {P} {st0} ieq)

------------------------------------------------------------------------
-- The single-assignment check is decidable: `producer-SA` is a runnable
-- static well-formedness check.
------------------------------------------------------------------------

NotIn? : (id : Identifier) (ks : List Identifier) → Dec (NotIn id ks)
NotIn? id []       = yes tt
NotIn? id (k ∷ ks) = ¬? (id ≟str k) ×-dec NotIn? id ks

NoDup? : (ids : List Identifier) → Dec (NoDup ids)
NoDup? []         = yes tt
NoDup? (id ∷ ids) = NotIn? id ids ×-dec NoDup? ids

SA? : (bound : List Identifier) (is : List Instruction) → Dec (SA bound is)
SA? bound []       = yes tt
SA? bound (i ∷ is) =
      all? (λ o → ¬? (o ∈? bound)) (outs-of i)
  ×-dec NoDup? (outs-of i)
  ×-dec SA? (bound ++ outs-of i) is

producer-SA? : (S : IrSource) → Dec (producer-SA S)
producer-SA? S =
      NoDup? (map TypedIdentifier.name (IrSource.inputs S))
  ×-dec SA? (map TypedIdentifier.name (IrSource.inputs S))
            (IrSource.instructions S)

------------------------------------------------------------------------
-- WF2 — the structural bit-count bounds (spec §5.3).
--
-- The Rust preprocess rejects these dynamically ("Excessive bit
-- bound" / "Excessive bit count", ir_vm.rs); the mechanized `step`
-- deliberately does not re-check them (Semantics, header note).  The
-- transfer theorem below (`preprocessʳ-agree`) makes the fidelity claim
-- precise: on every WF2-conforming source the Rust-faithful semantics
-- `preprocessʳ` — whose step re-checks the bounds — agrees with
-- `preprocess`, so the development's theorems apply verbatim.
-- 248 = FR_BYTES_STORED · 8.
------------------------------------------------------------------------

wf2-ok : Instruction → Bool
wf2-ok (constrain-bits _ bits)         = bits <ᵇ FR-BITS
wf2-ok (less-than _ _ bits _)          = bits <ᵇ FR-BITS
wf2-ok (div-mod-power-of-two _ bits _) = bits ≤ᵇ 248
wf2-ok (reconstitute-field _ _ bits _) = bits ≤ᵇ 248
wf2-ok _                               = true

producer-WF2 : IrSource → Set
producer-WF2 S = All (λ i → T (wf2-ok i)) (IrSource.instructions S)

producer-WF2? : (S : IrSource) → Dec (producer-WF2 S)
producer-WF2? S = all? (λ i → T? (wf2-ok i)) (IrSource.instructions S)

------------------------------------------------------------------------
-- The Rust-faithful semantics and the WF2 transfer theorem.
--
-- `stepʳ` guards each instruction with the WF2 bit-count check the Rust
-- preprocess performs dynamically; `runʳ`/`preprocessʳ` are the
-- corresponding Rust-faithful run and preprocess (the terminal side
-- conditions are those of `preprocess`, unchanged).  On a
-- WF2-conforming source every guard passes, so the two semantics agree
-- outright.
------------------------------------------------------------------------

stepʳ : ProofPreimage → IrSource → State → Instruction → Maybe State
stepʳ P S st i = if wf2-ok i then step P S st i else nothing

runʳ : ProofPreimage → IrSource → State → List Instruction → Maybe State
runʳ P S st []       = just st
runʳ P S st (i ∷ is) = stepʳ P S st i >>= λ st′ → runʳ P S st′ is

preprocessʳ : IrSource → ProofPreimage → Maybe State
preprocessʳ S P =
  init S P >>= λ st0 →
  runʳ P S st0 (IrSource.instructions S) >>= λ stf →
  guardD (State.pti-idx stf
           ≟ℕ length (ProofPreimage.pub-transcript-inputs P))
   (case State.pto-rem stf of λ
     { (_ ∷ _) → nothing
     ; []      → case State.priv-rem stf of λ
       { (_ ∷ _) → nothing
       ; []      →
           if IrSource.do-communications-commitment S
           then (case ProofPreimage.comm-commitment P of λ
                  { (just (c , r)) →
                      guardD (c ≟ᶠ transient-commit
                                    (ProofPreimage.inputs P
                                      ++ concatMap encodeᵉ (State.outs stf)) r)
                        (just stf)
                  ; nothing → nothing })
           else just stf } })

-- On WF2-conforming instructions the guarded run is the plain run.
runʳ-agree : ∀ {P S st} is → All (λ i → T (wf2-ok i)) is
  → runʳ P S st is ≡ run P S st is
runʳ-agree []       _          = refl
runʳ-agree {P} {S} {st} (i ∷ is) (px ∷ pxs) with wf2-ok i
... | false = case px of λ ()
... | true with step P S st i
...   | nothing  = refl
...   | just st′ = runʳ-agree {P} {S} {st′} is pxs

-- The transfer theorem: for a WF2-conforming source the Rust-faithful
-- preprocess is the mechanized one.
preprocessʳ-agree : ∀ S P → producer-WF2 S
  → preprocessʳ S P ≡ preprocess S P
preprocessʳ-agree S P wf2
  with init S P
... | nothing  = refl
... | just st0
  rewrite runʳ-agree {P} {S} {st0} (IrSource.instructions S) wf2
  with run P S st0 (IrSource.instructions S)
...   | nothing  = refl
...   | just stf
  with State.pti-idx stf ≟ℕ length (ProofPreimage.pub-transcript-inputs P)
...     | no _  = refl
...     | yes _
  with State.pto-rem stf
...       | (_ ∷ _) = refl
...       | []
  with State.priv-rem stf
...         | (_ ∷ _) = refl
...         | []
  with IrSource.do-communications-commitment S
...           | false = refl
...           | true
  with ProofPreimage.comm-commitment P
...             | nothing = refl
...             | just (c , r)
  with c ≟ᶠ transient-commit
             (ProofPreimage.inputs P ++ concatMap encodeᵉ (State.outs stf)) r
...               | yes _ = refl
...               | no _  = refl

------------------------------------------------------------------------
-- Static well-typedness for a zkir-v3 `IrSource`.
--
-- The backward per-instruction lemmas (`*-bwd` in `CircuitBackward`)
-- ask for *type-directed* facts about their operands — that `add`'s
-- operands are both Native or both JubjubPoint, that `cond-select`'s two
-- branches share a type, etc.  Off-circuit these hold because a
-- well-typed producer never feeds an operand the wrong type; zkir-v3 is
-- untyped at the wire level, so we recover them from a static typing
-- pass, exactly parallel to the single-assignment pass above.
--
-- The type context `TyCtx` assigns each bound identifier an `IrType`.
-- The value-typing invariant `TyEq m Γ` says every entry of `Γ` names a
-- cell of `m` holding a value of that type.  It is seeded from `init`
-- (declared input types) and maintained by `step` (each instruction's
-- output type rule, `outtys`).  The decidable `producer-WT` scans the
-- source, computing the output types and checking each instruction's
-- operand types against what its off-circuit `step` demands.
------------------------------------------------------------------------

TyCtx : Set
TyCtx = List (Identifier × IrType)

-- The identifiers a context binds.
dom-ty : TyCtx → List Identifier
dom-ty = map proj₁

-- Look up an identifier's declared type (first match).
lookup-ty : TyCtx → Identifier → Maybe IrType
lookup-ty []             _  = nothing
lookup-ty ((k , t) ∷ Γ) id  =
  case id ≟str k of λ { (yes _) → just t ; (no _) → lookup-ty Γ id }

-- Operand type: an immediate is Native; a variable's is its context type.
optype : TyCtx → Operand → Maybe IrType
optype Γ (imm _)  = just native
optype Γ (var id) = lookup-ty Γ id

------------------------------------------------------------------------
-- The value-typing invariant.
------------------------------------------------------------------------

-- Each identifier the context types names a cell holding a value of that
-- type.  Stated via `lookup-ty` (first-match), so a fresh append never
-- perturbs an earlier entry.  Wrapped in a record (not a bare Π) so the
-- combinators' Π-typed results are not eagerly instantiated at call
-- sites (a bare `∀{id t}→…` result stalls the unifier with fresh `{id}`
-- metas, exactly the `_⊑_` issue from `holds-lower`).
record TyEq (m : Mem) (Γ : TyCtx) : Set where
  constructor tyEq
  field
    at : ∀ {id t} → lookup-ty Γ id ≡ just t
       → ∃ λ v → m id ≡ just v × typeof v ≡ t
open TyEq public

-- `decode` yields a value of the type it was asked to read.
decode-typeof : ∀ t raw {v} → decode t raw ≡ just v → typeof v ≡ t
decode-typeof native (x ∷ []) refl = refl
decode-typeof jubjub-scalar (s ∷ []) e
  with jubjubScalarFromFr s
... | just _ with refl ← e = refl
decode-typeof bytes32 (lo ∷ hi ∷ []) e
  with low-high→bytes32 lo hi
... | just _ with refl ← e = refl
decode-typeof jubjub-point (x ∷ y ∷ []) e
  with fromCoordsJ x y
... | just _ with refl ← e = refl
decode-typeof secp256k1-base (l ∷ h ∷ []) e
  with limbs→secp256k1Base l h
... | just _ with refl ← e = refl
decode-typeof secp256k1-scalar (l ∷ h ∷ []) e
  with limbs→secp256k1Scalar l h
... | just _ with refl ← e = refl
decode-typeof secp256k1-point (a ∷ b ∷ c ∷ d ∷ f ∷ []) e
  with limbs→secp256k1Point a b c d f
... | just _ with refl ← e = refl
decode-typeof secp256r1-base (l ∷ h ∷ []) e
  with limbs→secp256r1Base l h
... | just _ with refl ← e = refl
decode-typeof secp256r1-scalar (l ∷ h ∷ []) e
  with limbs→secp256r1Scalar l h
... | just _ with refl ← e = refl
decode-typeof secp256r1-point (a ∷ b ∷ c ∷ d ∷ f ∷ []) e
  with limbs→secp256r1Point a b c d f
... | just _ with refl ← e = refl
decode-typeof curve25519-base (l ∷ h ∷ []) e
  with limbs→curve25519Base l h
... | just _ with refl ← e = refl
decode-typeof curve25519-scalar (l ∷ h ∷ []) e
  with limbs→curve25519Scalar l h
... | just _ with refl ← e = refl
decode-typeof curve25519-point (a ∷ b ∷ c ∷ d ∷ []) e
  with limbs→curve25519Point a b c d
... | just _ with refl ← e = refl

-- The default value of a type has that type.
default-typeof : ∀ t → typeof (default-val t) ≡ t
default-typeof native            = refl
default-typeof bytes32           = refl
default-typeof jubjub-point      = refl
default-typeof jubjub-scalar     = refl
default-typeof secp256k1-point        = refl
default-typeof secp256k1-base         = refl
default-typeof secp256k1-scalar       = refl
default-typeof secp256r1-point   = refl
default-typeof secp256r1-base    = refl
default-typeof secp256r1-scalar  = refl
default-typeof curve25519-point  = refl
default-typeof curve25519-base   = refl
default-typeof curve25519-scalar = refl

------------------------------------------------------------------------
-- Lookup lemmas for context extension.
------------------------------------------------------------------------

-- A lookup found in a prefix is unchanged by appending a suffix.
lookup-ty-++ˡ : ∀ Γ Δ {id t}
  → lookup-ty Γ id ≡ just t → lookup-ty (Γ ++ Δ) id ≡ just t
lookup-ty-++ˡ ((k , s) ∷ Γ) Δ {id} e with id ≟str k
... | yes _ = e
... | no  _ = lookup-ty-++ˡ Γ Δ e

-- Looking up in `Γ ++ Δ` an id absent from `Γ` falls through to `Δ`.
lookup-ty-++ʳ : ∀ Γ Δ {id}
  → ¬ (id ∈ dom-ty Γ)
  → lookup-ty (Γ ++ Δ) id ≡ lookup-ty Δ id
lookup-ty-++ʳ []            Δ _   = refl
lookup-ty-++ʳ ((k , s) ∷ Γ) Δ {id} id∉ with id ≟str k
... | yes id≡k = case id∉ (here id≡k) of λ ()
... | no  _    = lookup-ty-++ʳ Γ Δ (λ p → id∉ (there p))

-- The head of a context is found under its own key.
lookup-ty-here : ∀ {Γ} (o : Identifier) t → lookup-ty ((o , t) ∷ Γ) o ≡ just t
lookup-ty-here {Γ} o t with o ≟str o
... | yes _  = refl
... | no ¬p  = case ¬p refl of λ ()

-- A successful lookup in `Γ ++ Δ` either came from `Γ` or (if `Γ` misses)
-- from `Δ`.  Splits an append lookup for the reverse (`step`) direction.
lookup-ty-++⁻ : ∀ Γ Δ {id t}
  → lookup-ty (Γ ++ Δ) id ≡ just t
  → (lookup-ty Γ id ≡ just t) ⊎ (lookup-ty Δ id ≡ just t)
lookup-ty-++⁻ []            Δ e = inj₂ e
lookup-ty-++⁻ ((k , s) ∷ Γ) Δ {id} e with id ≟str k
... | yes _ = inj₁ e
... | no  _ = lookup-ty-++⁻ Γ Δ e

-- The singleton context resolves only its own key.
lookup-ty-[x]≢ : ∀ {id o t s} → ¬ (id ≡ o)
  → lookup-ty ((o , t) ∷ []) id ≡ just s → ⊥
lookup-ty-[x]≢ {id} {o} id≢o e with id ≟str o
... | yes id≡o = id≢o id≡o
... | no  _    = case e of λ ()

------------------------------------------------------------------------
-- Seeding and maintaining `TyEq`.
------------------------------------------------------------------------

-- The empty context is satisfied by any memory (no lookups succeed).
tyEq-∅ : ∀ {m} → TyEq m []
tyEq-∅ = tyEq λ ()

-- Prepend a fresh binding whose value has the declared type.  The shape
-- `decode-inputs` produces (front-extension); `lookup-ty` sees the new
-- head first, and earlier keys are untouched.
tyEq-cons : ∀ (m : Mem) (Γ : TyCtx) (o : Identifier) v t
  → typeof v ≡ t → TyEq m Γ → TyEq (ins o v m) ((o , t) ∷ Γ)
tyEq-cons m Γ o v t tv teq = tyEq λ {id} {s} e → go id s e
  where
  go : ∀ id s → lookup-ty ((o , t) ∷ Γ) id ≡ just s
     → ∃ λ w → ins o v m id ≡ just w × typeof w ≡ s
  go id s e with id ≟str o
  ... | yes refl with refl ← just-injective e = v , refl , tv
  ... | no id≢o = let (w , mw , tw) = at teq e in w , mw , tw

-- Append a fresh binding at the end (step's single-output shape).  The
-- new key `o` must be absent from `dom-ty Γ` so the append does not
-- shadow it; earlier lookups fall through the append unchanged.
tyEq-ins : ∀ (m : Mem) (Γ : TyCtx) (o : Identifier) v t
  → typeof v ≡ t → ¬ (o ∈ dom-ty Γ)
  → TyEq m Γ → TyEq (ins o v m) (Γ ++ (o , t) ∷ [])
tyEq-ins m Γ o v t tv o∉ teq = tyEq λ {id} {s} e → go id s e
  where
  go : ∀ id s → lookup-ty (Γ ++ (o , t) ∷ []) id ≡ just s
     → ∃ λ w → ins o v m id ≡ just w × typeof w ≡ s
  go id s e with id ≟str o
  ... | yes refl
        with refl ← just-injective
               (trans (sym (trans (lookup-ty-++ʳ Γ ((o , t) ∷ []) o∉)
                                  (lookup-ty-here o t))) e) =
          v , refl , tv
  ... | no id≢o =
        let (w , mw , tw) = at teq (from-Γ (lookup-ty-++⁻ Γ ((o , t) ∷ []) e))
        in w , mw , tw
    where
    -- The lookup came from `Γ` (the singleton only resolves `o`, id≢o).
    from-Γ : (lookup-ty Γ id ≡ just s) ⊎ (lookup-ty ((o , t) ∷ []) id ≡ just s)
           → lookup-ty Γ id ≡ just s
    from-Γ (inj₁ p) = p
    from-Γ (inj₂ q) = case lookup-ty-[x]≢ id≢o q of λ ()

------------------------------------------------------------------------
-- Seeding `TyEq` from `init`.
--
-- `decode-inputs` binds each declared input to a value of its declared
-- type (`decode-typeof`), so the initial memory satisfies the input
-- context `input-ctx S` (the declared `(name , type)` pairs).
------------------------------------------------------------------------

input-ctx : List TypedIdentifier → TyCtx
input-ctx = map (λ ti → TypedIdentifier.name ti , TypedIdentifier.val-t ti)

decode-inputs-ty : ∀ tis raw {m}
  → decode-inputs tis raw ≡ just m
  → TyEq m (input-ctx tis)
decode-inputs-ty []         []      refl = tyEq-∅
decode-inputs-ty []         (_ ∷ _) ()
decode-inputs-ty (ti ∷ tis) raw ieq
  with decode (TypedIdentifier.val-t ti)
              (take (encoded-len (TypedIdentifier.val-t ti)) raw)
         in edec
... | just v
  with decode-inputs tis (drop (encoded-len (TypedIdentifier.val-t ti)) raw)
         in erest
...   | just m' with refl ← ieq =
        tyEq-cons m' (input-ctx tis)
          (TypedIdentifier.name ti) v (TypedIdentifier.val-t ti)
          (decode-typeof (TypedIdentifier.val-t ti) _ edec)
          (decode-inputs-ty tis (drop (encoded-len (TypedIdentifier.val-t ti))
                                   raw) erest)

init-ty : ∀ {S P st0}
  → init S P ≡ just st0
  → TyEq (State.mem st0) (input-ctx (IrSource.inputs S))
init-ty {S} {P} {st0} ieq =
  decode-inputs-ty (IrSource.inputs S) (ProofPreimage.inputs P)
    (init-decode {S} {P} {st0} ieq)

------------------------------------------------------------------------
-- Reading an operand's type off the invariant.
--
-- Under `TyEq m Γ`, an operand with a declared type `t` (`optype Γ`) that
-- resolves to `av` in `m` has `typeof av ≡ t`.  This is the bridge the
-- discharge lemmas use to turn a static operand type into a runtime one.
------------------------------------------------------------------------

tyEq-optype : ∀ {m Γ op t av}
  → TyEq m Γ → optype Γ op ≡ just t → resolve m op ≡ just av
  → typeof av ≡ t
tyEq-optype {op = imm x} teq ot rv
  with refl ← ot with refl ← rv = refl
tyEq-optype {m} {Γ} {op = var id} {t} {av} teq ot rv
  with at teq ot
... | (w , mw , tw) with refl ← just-injective (trans (sym mw) rv) = tw

-- A value whose type is `native` is a `val-native`.
typeof-native : ∀ {v} → typeof v ≡ native → ∃ λ x → v ≡ val-native x
typeof-native {val-native x} refl = x , refl

-- A value whose type is `jubjub-point` is a `val-jubjub-point`.
typeof-point : ∀ {v} → typeof v ≡ jubjub-point → ∃ λ p → v ≡ val-jubjub-point p
typeof-point {val-jubjub-point p} refl = p , refl

-- A value whose type is `bytes32` is a `val-bytes32`.
typeof-bytes32 : ∀ {v} → typeof v ≡ bytes32 → ∃ λ b → v ≡ val-bytes32 b
typeof-bytes32 {val-bytes32 b} refl = b , refl

-- A value whose type is `secp256k1-point` is a `val-secp256k1-point`.
typeof-secp256k1-point : ∀ {v} → typeof v ≡ secp256k1-point
  → ∃ λ p → v ≡ val-secp256k1-point p
typeof-secp256k1-point {val-secp256k1-point p} refl = p , refl

-- A value whose type is `secp256k1-base` is a `val-secp256k1-base`.
typeof-secp256k1-base : ∀ {v} → typeof v ≡ secp256k1-base
  → ∃ λ x → v ≡ val-secp256k1-base x
typeof-secp256k1-base {val-secp256k1-base x} refl = x , refl

-- A value whose type is `secp256k1-scalar` is a `val-secp256k1-scalar`.
typeof-secp256k1-scalar : ∀ {v} → typeof v ≡ secp256k1-scalar
  → ∃ λ s → v ≡ val-secp256k1-scalar s
typeof-secp256k1-scalar {val-secp256k1-scalar s} refl = s , refl

-- A value whose type is `secp256r1-point` is a `val-secp256r1-point`.
typeof-secp256r1-point : ∀ {v} → typeof v ≡ secp256r1-point
  → ∃ λ p → v ≡ val-secp256r1-point p
typeof-secp256r1-point {val-secp256r1-point p} refl = p , refl

-- A value whose type is `secp256r1-base` is a `val-secp256r1-base`.
typeof-secp256r1-base : ∀ {v} → typeof v ≡ secp256r1-base
  → ∃ λ x → v ≡ val-secp256r1-base x
typeof-secp256r1-base {val-secp256r1-base x} refl = x , refl

-- A value whose type is `secp256r1-scalar` is a `val-secp256r1-scalar`.
typeof-secp256r1-scalar : ∀ {v} → typeof v ≡ secp256r1-scalar
  → ∃ λ s → v ≡ val-secp256r1-scalar s
typeof-secp256r1-scalar {val-secp256r1-scalar s} refl = s , refl

-- A value whose type is `curve25519-point` is a `val-curve25519-point`.
typeof-curve25519-point : ∀ {v} → typeof v ≡ curve25519-point
  → ∃ λ p → v ≡ val-curve25519-point p
typeof-curve25519-point {val-curve25519-point p} refl = p , refl

-- A value whose type is `curve25519-base` is a `val-curve25519-base`.
typeof-curve25519-base : ∀ {v} → typeof v ≡ curve25519-base
  → ∃ λ x → v ≡ val-curve25519-base x
typeof-curve25519-base {val-curve25519-base x} refl = x , refl

-- A value whose type is `curve25519-scalar` is a `val-curve25519-scalar`.
typeof-curve25519-scalar : ∀ {v} → typeof v ≡ curve25519-scalar
  → ∃ λ s → v ≡ val-curve25519-scalar s
typeof-curve25519-scalar {val-curve25519-scalar s} refl = s , refl

------------------------------------------------------------------------
-- Discharging the type-directed `*-bwd` obligations.
--
-- Given the typing invariant at the pre-step context, the operands'
-- static types (`optype`), and their runtime resolutions, produce exactly
-- the type-support premise the corresponding backward lemma asks for.
------------------------------------------------------------------------

-- A field-or-group operand type: the eleven types `add`/`neg` accept
-- (native, JubjubPoint, Secp256k1Point, Secp256k1Base, Secp256k1Scalar, Secp256r1Point,
-- Secp256r1Base, Secp256r1Scalar, Curve25519Point, Curve25519Base,
-- Curve25519Scalar).  The disjunct order matches `Circuit.holds`'
-- `gate-add`/`gate-neg`.
FN : IrType → Set
FN t = (t ≡ native) ⊎ (t ≡ jubjub-point) ⊎ (t ≡ secp256k1-point)
     ⊎ (t ≡ secp256k1-base) ⊎ (t ≡ secp256k1-scalar)
     ⊎ (t ≡ secp256r1-point) ⊎ (t ≡ secp256r1-base) ⊎ (t ≡ secp256r1-scalar)
     ⊎ (t ≡ curve25519-point) ⊎ (t ≡ curve25519-base)
     ⊎ (t ≡ curve25519-scalar)

-- A field-multiplication operand type: the seven types `mul`/`inv`
-- accept (native, Secp256k1Base, Secp256k1Scalar, Secp256r1Base, Secp256r1Scalar,
-- Curve25519Base, Curve25519Scalar).  Order matches `Circuit.holds`'
-- `gate-mul`/`gate-inv`.
FM : IrType → Set
FM t = (t ≡ native) ⊎ (t ≡ secp256k1-base) ⊎ (t ≡ secp256k1-scalar)
     ⊎ (t ≡ secp256r1-base) ⊎ (t ≡ secp256r1-scalar)
     ⊎ (t ≡ curve25519-base) ⊎ (t ≡ curve25519-scalar)

-- `add-bwd`'s eleven-way disjunction, from a shared operand type that is
-- one of the field/group types.  The disjunct order is `FN`'s, and matches
-- `gate-add`.
add-support : ∀ {m Γ a b t av bv}
  → TyEq m Γ
  → optype Γ a ≡ just t → optype Γ b ≡ just t
  → FN t
  → resolve m a ≡ just av → resolve m b ≡ just bv
  → ( (∃ λ x → ∃ λ y →
         resolve m a ≡ just (val-native x) × resolve m b ≡ just (val-native y))
    ⊎ (∃ λ p → ∃ λ q →
         resolve m a ≡ just (val-jubjub-point p)
       × resolve m b ≡ just (val-jubjub-point q))
    ⊎ (∃ λ p → ∃ λ q →
         resolve m a ≡ just (val-secp256k1-point p)
       × resolve m b ≡ just (val-secp256k1-point q))
    ⊎ (∃ λ x → ∃ λ y →
         resolve m a ≡ just (val-secp256k1-base x)
       × resolve m b ≡ just (val-secp256k1-base y))
    ⊎ (∃ λ x → ∃ λ y →
         resolve m a ≡ just (val-secp256k1-scalar x)
       × resolve m b ≡ just (val-secp256k1-scalar y))
    ⊎ (∃ λ p → ∃ λ q →
         resolve m a ≡ just (val-secp256r1-point p)
       × resolve m b ≡ just (val-secp256r1-point q))
    ⊎ (∃ λ x → ∃ λ y →
         resolve m a ≡ just (val-secp256r1-base x)
       × resolve m b ≡ just (val-secp256r1-base y))
    ⊎ (∃ λ x → ∃ λ y →
         resolve m a ≡ just (val-secp256r1-scalar x)
       × resolve m b ≡ just (val-secp256r1-scalar y))
    ⊎ (∃ λ p → ∃ λ q →
         resolve m a ≡ just (val-curve25519-point p)
       × resolve m b ≡ just (val-curve25519-point q))
    ⊎ (∃ λ x → ∃ λ y →
         resolve m a ≡ just (val-curve25519-base x)
       × resolve m b ≡ just (val-curve25519-base y))
    ⊎ (∃ λ x → ∃ λ y →
         resolve m a ≡ just (val-curve25519-scalar x)
       × resolve m b ≡ just (val-curve25519-scalar y)))
add-support {a = a} {b} teq ota otb (inj₁ refl) ra rb
  with typeof-native (tyEq-optype {op = a} teq ota ra)
     | typeof-native (tyEq-optype {op = b} teq otb rb)
... | (x , refl) | (y , refl) = inj₁ (x , y , ra , rb)
add-support {a = a} {b} teq ota otb (inj₂ (inj₁ refl)) ra rb
  with typeof-point (tyEq-optype {op = a} teq ota ra)
     | typeof-point (tyEq-optype {op = b} teq otb rb)
... | (p , refl) | (q , refl) = inj₂ (inj₁ (p , q , ra , rb))
add-support {a = a} {b} teq ota otb (inj₂ (inj₂ (inj₁ refl))) ra rb
  with typeof-secp256k1-point (tyEq-optype {op = a} teq ota ra)
     | typeof-secp256k1-point (tyEq-optype {op = b} teq otb rb)
... | (p , refl) | (q , refl) = inj₂ (inj₂ (inj₁ (p , q , ra , rb)))
add-support {a = a} {b} teq ota otb (inj₂ (inj₂ (inj₂ (inj₁ refl)))) ra rb
  with typeof-secp256k1-base (tyEq-optype {op = a} teq ota ra)
     | typeof-secp256k1-base (tyEq-optype {op = b} teq otb rb)
... | (x , refl) | (y , refl) = inj₂ (inj₂ (inj₂ (inj₁ (x , y , ra , rb))))
add-support {a = a} {b} teq ota otb (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl))))) ra rb
  with typeof-secp256k1-scalar (tyEq-optype {op = a} teq ota ra)
     | typeof-secp256k1-scalar (tyEq-optype {op = b} teq otb rb)
... | (x , refl) | (y , refl) =
      inj₂ (inj₂ (inj₂ (inj₂ (inj₁ (x , y , ra , rb)))))
add-support {a = a} {b} teq ota otb
  (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl)))))) ra rb
  with typeof-secp256r1-point (tyEq-optype {op = a} teq ota ra)
     | typeof-secp256r1-point (tyEq-optype {op = b} teq otb rb)
... | (p , refl) | (q , refl) =
      inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ (p , q , ra , rb))))))
add-support {a = a} {b} teq ota otb
  (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl))))))) ra rb
  with typeof-secp256r1-base (tyEq-optype {op = a} teq ota ra)
     | typeof-secp256r1-base (tyEq-optype {op = b} teq otb rb)
... | (x , refl) | (y , refl) =
      inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ (x , y , ra , rb)))))))
add-support {a = a} {b} teq ota otb
  (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl)))))))) ra rb
  with typeof-secp256r1-scalar (tyEq-optype {op = a} teq ota ra)
     | typeof-secp256r1-scalar (tyEq-optype {op = b} teq otb rb)
... | (x , refl) | (y , refl) =
      inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ (x , y , ra , rb))))))))
add-support {a = a} {b} teq ota otb
  (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl))))))))) ra rb
  with typeof-curve25519-point (tyEq-optype {op = a} teq ota ra)
     | typeof-curve25519-point (tyEq-optype {op = b} teq otb rb)
... | (p , refl) | (q , refl) =
      inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂
        (inj₁ (p , q , ra , rb)))))))))
add-support {a = a} {b} teq ota otb
  (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl))))))))))
  ra rb
  with typeof-curve25519-base (tyEq-optype {op = a} teq ota ra)
     | typeof-curve25519-base (tyEq-optype {op = b} teq otb rb)
... | (x , refl) | (y , refl) =
      inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂
        (inj₁ (x , y , ra , rb))))))))))
add-support {a = a} {b} teq ota otb
  (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ refl))))))))))
  ra rb
  with typeof-curve25519-scalar (tyEq-optype {op = a} teq ota ra)
     | typeof-curve25519-scalar (tyEq-optype {op = b} teq otb rb)
... | (x , refl) | (y , refl) =
      inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂
        (x , y , ra , rb))))))))))

-- `cond-select-bwd`'s `typeof av ≡ typeof bvl`, from a shared operand type.
cond-select-support : ∀ {m Γ a b t av bv}
  → TyEq m Γ
  → optype Γ a ≡ just t → optype Γ b ≡ just t
  → resolve m a ≡ just av → resolve m b ≡ just bv
  → typeof av ≡ typeof bv
cond-select-support {a = a} {b} teq ota otb ra rb =
  trans (tyEq-optype {op = a} teq ota ra)
        (sym (tyEq-optype {op = b} teq otb rb))

-- Extracts a `val-native` resolution from an operand statically typed
-- Native.  Used wherever a backward lemma reads a Native operand
-- (`into-bytes32`, the coordinate splits, the native `mul` arm, …).
mul-support-l : ∀ {m Γ a av}
  → TyEq m Γ → optype Γ a ≡ just native → resolve m a ≡ just av
  → ∃ λ x → resolve m a ≡ just (val-native x)
mul-support-l {a = a} {av} teq ota ra
  with typeof-native (tyEq-optype {op = a} teq ota ra)
... | (x , refl) = x , ra

-- `mul-bwd`'s seven-way disjunction, from a shared operand type that is
-- one of the field-multiplication types.  Order matches `gate-mul`.
mul-support : ∀ {m Γ a b t av bv}
  → TyEq m Γ
  → optype Γ a ≡ just t → optype Γ b ≡ just t
  → FM t
  → resolve m a ≡ just av → resolve m b ≡ just bv
  → ( (∃ λ x → ∃ λ y →
         resolve m a ≡ just (val-native x) × resolve m b ≡ just (val-native y))
    ⊎ (∃ λ x → ∃ λ y →
         resolve m a ≡ just (val-secp256k1-base x)
       × resolve m b ≡ just (val-secp256k1-base y))
    ⊎ (∃ λ x → ∃ λ y →
         resolve m a ≡ just (val-secp256k1-scalar x)
       × resolve m b ≡ just (val-secp256k1-scalar y))
    ⊎ (∃ λ x → ∃ λ y →
         resolve m a ≡ just (val-secp256r1-base x)
       × resolve m b ≡ just (val-secp256r1-base y))
    ⊎ (∃ λ x → ∃ λ y →
         resolve m a ≡ just (val-secp256r1-scalar x)
       × resolve m b ≡ just (val-secp256r1-scalar y))
    ⊎ (∃ λ x → ∃ λ y →
         resolve m a ≡ just (val-curve25519-base x)
       × resolve m b ≡ just (val-curve25519-base y))
    ⊎ (∃ λ x → ∃ λ y →
         resolve m a ≡ just (val-curve25519-scalar x)
       × resolve m b ≡ just (val-curve25519-scalar y)))
mul-support {a = a} {b} teq ota otb (inj₁ refl) ra rb
  with typeof-native (tyEq-optype {op = a} teq ota ra)
     | typeof-native (tyEq-optype {op = b} teq otb rb)
... | (x , refl) | (y , refl) = inj₁ (x , y , ra , rb)
mul-support {a = a} {b} teq ota otb (inj₂ (inj₁ refl)) ra rb
  with typeof-secp256k1-base (tyEq-optype {op = a} teq ota ra)
     | typeof-secp256k1-base (tyEq-optype {op = b} teq otb rb)
... | (x , refl) | (y , refl) = inj₂ (inj₁ (x , y , ra , rb))
mul-support {a = a} {b} teq ota otb (inj₂ (inj₂ (inj₁ refl))) ra rb
  with typeof-secp256k1-scalar (tyEq-optype {op = a} teq ota ra)
     | typeof-secp256k1-scalar (tyEq-optype {op = b} teq otb rb)
... | (x , refl) | (y , refl) = inj₂ (inj₂ (inj₁ (x , y , ra , rb)))
mul-support {a = a} {b} teq ota otb (inj₂ (inj₂ (inj₂ (inj₁ refl)))) ra rb
  with typeof-secp256r1-base (tyEq-optype {op = a} teq ota ra)
     | typeof-secp256r1-base (tyEq-optype {op = b} teq otb rb)
... | (x , refl) | (y , refl) = inj₂ (inj₂ (inj₂ (inj₁ (x , y , ra , rb))))
mul-support {a = a} {b} teq ota otb
  (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl))))) ra rb
  with typeof-secp256r1-scalar (tyEq-optype {op = a} teq ota ra)
     | typeof-secp256r1-scalar (tyEq-optype {op = b} teq otb rb)
... | (x , refl) | (y , refl) =
      inj₂ (inj₂ (inj₂ (inj₂ (inj₁ (x , y , ra , rb)))))
mul-support {a = a} {b} teq ota otb
  (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl)))))) ra rb
  with typeof-curve25519-base (tyEq-optype {op = a} teq ota ra)
     | typeof-curve25519-base (tyEq-optype {op = b} teq otb rb)
... | (x , refl) | (y , refl) =
      inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ (x , y , ra , rb))))))
mul-support {a = a} {b} teq ota otb
  (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ refl)))))) ra rb
  with typeof-curve25519-scalar (tyEq-optype {op = a} teq ota ra)
     | typeof-curve25519-scalar (tyEq-optype {op = b} teq otb rb)
... | (x , refl) | (y , refl) =
      inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (x , y , ra , rb))))))

-- `inv-bwd`'s seven-way disjunction (single operand).  Order matches
-- `gate-inv`.
inv-support : ∀ {m Γ a t av}
  → TyEq m Γ → optype Γ a ≡ just t → FM t
  → resolve m a ≡ just av
  → (∃ λ x → resolve m a ≡ just (val-native x))
  ⊎ (∃ λ x → resolve m a ≡ just (val-secp256k1-base x))
  ⊎ (∃ λ x → resolve m a ≡ just (val-secp256k1-scalar x))
  ⊎ (∃ λ x → resolve m a ≡ just (val-secp256r1-base x))
  ⊎ (∃ λ x → resolve m a ≡ just (val-secp256r1-scalar x))
  ⊎ (∃ λ x → resolve m a ≡ just (val-curve25519-base x))
  ⊎ (∃ λ x → resolve m a ≡ just (val-curve25519-scalar x))
inv-support {a = a} teq ota (inj₁ refl) ra
  with typeof-native (tyEq-optype {op = a} teq ota ra)
... | (x , refl) = inj₁ (x , ra)
inv-support {a = a} teq ota (inj₂ (inj₁ refl)) ra
  with typeof-secp256k1-base (tyEq-optype {op = a} teq ota ra)
... | (x , refl) = inj₂ (inj₁ (x , ra))
inv-support {a = a} teq ota (inj₂ (inj₂ (inj₁ refl))) ra
  with typeof-secp256k1-scalar (tyEq-optype {op = a} teq ota ra)
... | (x , refl) = inj₂ (inj₂ (inj₁ (x , ra)))
inv-support {a = a} teq ota (inj₂ (inj₂ (inj₂ (inj₁ refl)))) ra
  with typeof-secp256r1-base (tyEq-optype {op = a} teq ota ra)
... | (x , refl) = inj₂ (inj₂ (inj₂ (inj₁ (x , ra))))
inv-support {a = a} teq ota (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl))))) ra
  with typeof-secp256r1-scalar (tyEq-optype {op = a} teq ota ra)
... | (x , refl) = inj₂ (inj₂ (inj₂ (inj₂ (inj₁ (x , ra)))))
inv-support {a = a} teq ota (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl)))))) ra
  with typeof-curve25519-base (tyEq-optype {op = a} teq ota ra)
... | (x , refl) = inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ (x , ra))))))
inv-support {a = a} teq ota (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ refl)))))) ra
  with typeof-curve25519-scalar (tyEq-optype {op = a} teq ota ra)
... | (x , refl) = inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (x , ra))))))

-- `neg-bwd`'s eleven-way disjunction (single operand).  Order matches
-- `gate-neg`.
neg-support : ∀ {m Γ a t av}
  → TyEq m Γ → optype Γ a ≡ just t → FN t
  → resolve m a ≡ just av
  → (∃ λ x → resolve m a ≡ just (val-native x))
  ⊎ (∃ λ p → resolve m a ≡ just (val-jubjub-point p))
  ⊎ (∃ λ p → resolve m a ≡ just (val-secp256k1-point p))
  ⊎ (∃ λ x → resolve m a ≡ just (val-secp256k1-base x))
  ⊎ (∃ λ x → resolve m a ≡ just (val-secp256k1-scalar x))
  ⊎ (∃ λ p → resolve m a ≡ just (val-secp256r1-point p))
  ⊎ (∃ λ x → resolve m a ≡ just (val-secp256r1-base x))
  ⊎ (∃ λ x → resolve m a ≡ just (val-secp256r1-scalar x))
  ⊎ (∃ λ p → resolve m a ≡ just (val-curve25519-point p))
  ⊎ (∃ λ x → resolve m a ≡ just (val-curve25519-base x))
  ⊎ (∃ λ x → resolve m a ≡ just (val-curve25519-scalar x))
neg-support {a = a} teq ota (inj₁ refl) ra
  with typeof-native (tyEq-optype {op = a} teq ota ra)
... | (x , refl) = inj₁ (x , ra)
neg-support {a = a} teq ota (inj₂ (inj₁ refl)) ra
  with typeof-point (tyEq-optype {op = a} teq ota ra)
... | (p , refl) = inj₂ (inj₁ (p , ra))
neg-support {a = a} teq ota (inj₂ (inj₂ (inj₁ refl))) ra
  with typeof-secp256k1-point (tyEq-optype {op = a} teq ota ra)
... | (p , refl) = inj₂ (inj₂ (inj₁ (p , ra)))
neg-support {a = a} teq ota (inj₂ (inj₂ (inj₂ (inj₁ refl)))) ra
  with typeof-secp256k1-base (tyEq-optype {op = a} teq ota ra)
... | (x , refl) = inj₂ (inj₂ (inj₂ (inj₁ (x , ra))))
neg-support {a = a} teq ota (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl))))) ra
  with typeof-secp256k1-scalar (tyEq-optype {op = a} teq ota ra)
... | (x , refl) = inj₂ (inj₂ (inj₂ (inj₂ (inj₁ (x , ra)))))
neg-support {a = a} teq ota (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl)))))) ra
  with typeof-secp256r1-point (tyEq-optype {op = a} teq ota ra)
... | (p , refl) = inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ (p , ra))))))
neg-support {a = a} teq ota
  (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl))))))) ra
  with typeof-secp256r1-base (tyEq-optype {op = a} teq ota ra)
... | (x , refl) = inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ (x , ra)))))))
neg-support {a = a} teq ota
  (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl)))))))) ra
  with typeof-secp256r1-scalar (tyEq-optype {op = a} teq ota ra)
... | (x , refl) =
      inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ (x , ra))))))))
neg-support {a = a} teq ota
  (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl))))))))) ra
  with typeof-curve25519-point (tyEq-optype {op = a} teq ota ra)
... | (p , refl) =
      inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ (p , ra)))))))))
neg-support {a = a} teq ota
  (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl)))))))))) ra
  with typeof-curve25519-base (tyEq-optype {op = a} teq ota ra)
... | (x , refl) =
      inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂
        (inj₁ (x , ra))))))))))
neg-support {a = a} teq ota
  (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ refl)))))))))) ra
  with typeof-curve25519-scalar (tyEq-optype {op = a} teq ota ra)
... | (x , refl) =
      inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂
        (x , ra))))))))))

-- The reflexive equality reads `true` at every `valEq?`-supported type
-- (every value constructor except JubjubScalar, where `valEq?` is
-- `nothing`).
valEq?-refl-native  : ∀ x → valEq? (val-native x) (val-native x) ≡ just true
valEq?-refl-native  x =
  cong just (trans (isYes≗does (x ≟ᶠ x)) (dec-true (x ≟ᶠ x) refl))

valEq?-refl-bytes32 : ∀ b → valEq? (val-bytes32 b) (val-bytes32 b) ≡ just true
valEq?-refl-bytes32 b =
  cong just (trans (isYes≗does (b ≟B b)) (dec-true (b ≟B b) refl))

valEq?-refl-point   : ∀ p
  → valEq? (val-jubjub-point p) (val-jubjub-point p) ≡ just true
valEq?-refl-point   p =
  cong just (trans (isYes≗does (p ≟J p)) (dec-true (p ≟J p) refl))

valEq?-refl-secp256k1-point : ∀ p
  → valEq? (val-secp256k1-point p) (val-secp256k1-point p) ≡ just true
valEq?-refl-secp256k1-point p =
  cong just (trans (isYes≗does (p ≟K1 p)) (dec-true (p ≟K1 p) refl))

valEq?-refl-secp256k1-base : ∀ x
  → valEq? (val-secp256k1-base x) (val-secp256k1-base x) ≡ just true
valEq?-refl-secp256k1-base x =
  cong just (trans (isYes≗does (x ≟K1ᵇ x)) (dec-true (x ≟K1ᵇ x) refl))

valEq?-refl-secp256k1-scalar : ∀ s
  → valEq? (val-secp256k1-scalar s) (val-secp256k1-scalar s) ≡ just true
valEq?-refl-secp256k1-scalar s =
  cong just (trans (isYes≗does (s ≟K1ˢ s)) (dec-true (s ≟K1ˢ s) refl))

valEq?-refl-secp256r1-point : ∀ p
  → valEq? (val-secp256r1-point p) (val-secp256r1-point p) ≡ just true
valEq?-refl-secp256r1-point p =
  cong just (trans (isYes≗does (p ≟P p)) (dec-true (p ≟P p) refl))

valEq?-refl-secp256r1-base : ∀ x
  → valEq? (val-secp256r1-base x) (val-secp256r1-base x) ≡ just true
valEq?-refl-secp256r1-base x =
  cong just (trans (isYes≗does (x ≟Pᵇ x)) (dec-true (x ≟Pᵇ x) refl))

valEq?-refl-secp256r1-scalar : ∀ s
  → valEq? (val-secp256r1-scalar s) (val-secp256r1-scalar s) ≡ just true
valEq?-refl-secp256r1-scalar s =
  cong just (trans (isYes≗does (s ≟Pˢ s)) (dec-true (s ≟Pˢ s) refl))

valEq?-refl-curve25519-point : ∀ p
  → valEq? (val-curve25519-point p) (val-curve25519-point p) ≡ just true
valEq?-refl-curve25519-point p =
  cong just (trans (isYes≗does (p ≟C p)) (dec-true (p ≟C p) refl))

valEq?-refl-curve25519-base : ∀ x
  → valEq? (val-curve25519-base x) (val-curve25519-base x) ≡ just true
valEq?-refl-curve25519-base x =
  cong just (trans (isYes≗does (x ≟Cᵇ x)) (dec-true (x ≟Cᵇ x) refl))

valEq?-refl-curve25519-scalar : ∀ s
  → valEq? (val-curve25519-scalar s) (val-curve25519-scalar s) ≡ just true
valEq?-refl-curve25519-scalar s =
  cong just (trans (isYes≗does (s ≟Cˢ s)) (dec-true (s ≟Cˢ s) refl))

-- The `valEq?`-supported operand types of `constrain-eq` (every type
-- except JubjubScalar, whose `valEq?` is `nothing`).
SuppEq : IrType → Set
SuppEq t = (t ≡ native) ⊎ (t ≡ bytes32) ⊎ (t ≡ jubjub-point)
         ⊎ (t ≡ secp256k1-point) ⊎ (t ≡ secp256k1-base) ⊎ (t ≡ secp256k1-scalar)
         ⊎ (t ≡ secp256r1-point) ⊎ (t ≡ secp256r1-base) ⊎ (t ≡ secp256r1-scalar)
         ⊎ (t ≡ curve25519-point) ⊎ (t ≡ curve25519-base)
         ⊎ (t ≡ curve25519-scalar)

-- `constrain-eq-bwd`'s reflexive-`valEq?` support, given the operand's
-- type is `valEq?`-supported.
constrain-eq-support : ∀ {m Γ a t av}
  → TyEq m Γ → optype Γ a ≡ just t
  → SuppEq t
  → resolve m a ≡ just av
  → resolve m a ≡ just av × valEq? av av ≡ just true
constrain-eq-support {a = a} {av = av} teq ota tsup ra = ra , goal tsup
  where
  goal : _ → valEq? av av ≡ just true
  goal (inj₁ refl)
    with x , eqx ← typeof-native (tyEq-optype {op = a} teq ota ra) =
      trans (cong (λ w → valEq? w w) eqx) (valEq?-refl-native x)
  goal (inj₂ (inj₁ refl))
    with b , eqb ← typeof-bytes32 (tyEq-optype {op = a} teq ota ra) =
      trans (cong (λ w → valEq? w w) eqb) (valEq?-refl-bytes32 b)
  goal (inj₂ (inj₂ (inj₁ refl)))
    with p , eqp ← typeof-point (tyEq-optype {op = a} teq ota ra) =
      trans (cong (λ w → valEq? w w) eqp) (valEq?-refl-point p)
  goal (inj₂ (inj₂ (inj₂ (inj₁ refl))))
    with p , eqp ← typeof-secp256k1-point (tyEq-optype {op = a} teq ota ra) =
      trans (cong (λ w → valEq? w w) eqp) (valEq?-refl-secp256k1-point p)
  goal (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl)))))
    with x , eqx ← typeof-secp256k1-base (tyEq-optype {op = a} teq ota ra) =
      trans (cong (λ w → valEq? w w) eqx) (valEq?-refl-secp256k1-base x)
  goal (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl))))))
    with s , eqs ← typeof-secp256k1-scalar (tyEq-optype {op = a} teq ota ra) =
      trans (cong (λ w → valEq? w w) eqs) (valEq?-refl-secp256k1-scalar s)
  goal (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl)))))))
    with p , eqp ← typeof-secp256r1-point (tyEq-optype {op = a} teq ota ra) =
      trans (cong (λ w → valEq? w w) eqp) (valEq?-refl-secp256r1-point p)
  goal (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl))))))))
    with x , eqx ← typeof-secp256r1-base (tyEq-optype {op = a} teq ota ra) =
      trans (cong (λ w → valEq? w w) eqx) (valEq?-refl-secp256r1-base x)
  goal (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl)))))))))
    with s , eqs ← typeof-secp256r1-scalar (tyEq-optype {op = a} teq ota ra) =
      trans (cong (λ w → valEq? w w) eqs) (valEq?-refl-secp256r1-scalar s)
  goal (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂
         (inj₂ (inj₂ (inj₂ (inj₁ refl))))))))))
    with p , eqp ← typeof-curve25519-point (tyEq-optype {op = a} teq ota ra) =
      trans (cong (λ w → valEq? w w) eqp) (valEq?-refl-curve25519-point p)
  goal (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂
         (inj₂ (inj₂ (inj₂ (inj₂ (inj₁ refl)))))))))))
    with x , eqx ← typeof-curve25519-base (tyEq-optype {op = a} teq ota ra) =
      trans (cong (λ w → valEq? w w) eqx) (valEq?-refl-curve25519-base x)
  goal (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ (inj₂
         (inj₂ (inj₂ (inj₂ (inj₂ (inj₂ refl)))))))))))
    with s , eqs ← typeof-curve25519-scalar (tyEq-optype {op = a} teq ota ra) =
      trans (cong (λ w → valEq? w w) eqs) (valEq?-refl-curve25519-scalar s)

------------------------------------------------------------------------
-- Operand resolution from the typing invariant.
--
-- The backward driver must, at each step, *produce* the resolution of
-- every operand the reconstructed `step`/constraint reads.  `TyEq` alone
-- supplies it: a typed operand (`optype Γ op ≡ just t`) names a bound
-- cell (immediates resolve to a `val-native` directly).  These are the
-- witnesses `defd-of` shifts to the post-step witness (via `⊢-pres`) and
-- the backward lemmas take as their operand premises.
------------------------------------------------------------------------

optype-resolve : ∀ {m Γ op t} → TyEq m Γ → optype Γ op ≡ just t
  → ∃ λ v → resolve m op ≡ just v
optype-resolve {op = imm x}  teq ot = val-native x , refl
optype-resolve {op = var id} teq ot with at teq ot
... | (v , mv , _) = v , mv

optype-resolveᶠ : ∀ {m Γ op} → TyEq m Γ → optype Γ op ≡ just native
  → ∃ λ x → resolveᶠ m op ≡ just x
optype-resolveᶠ {m} {op = op} teq ot with optype-resolve {op = op} teq ot
... | (v , rv) with typeof-native (tyEq-optype {op = op} teq ot rv)
...   | (x , refl) = x , resolveᶠ-intro m op rv

------------------------------------------------------------------------
-- Static output-typing rules.
--
-- `outtys Γ i` extends the context `Γ` with the type of each output wire
-- of `i`, or fails (`nothing`) when `i`'s operands are ill-typed for what
-- its off-circuit `step` demands.  It parallels `outs-of` / `step-dom`:
-- no-output instructions leave `Γ` unchanged; single-output ones append
-- one typed entry; the two-/list-output ones append the corresponding
-- entries.  Only the type-directed instructions (`add`/`mul`/`neg`/
-- `cond-select`/`copy`) inspect operand types; the rest have a fixed
-- output type.
------------------------------------------------------------------------

-- Same operand type on both sides, if defined and equal (`add`,
-- `cond-select`).
same-ty : TyCtx → Operand → Operand → Maybe IrType
same-ty Γ a b with optype Γ a | optype Γ b
... | just ta | just tb =
      case ta ≟T tb of λ { (yes _) → just ta ; (no _) → nothing }
... | _       | _       = nothing

-- `same-ty` yields a type that both operands share.
same-ty-l : ∀ {Γ a b t} → same-ty Γ a b ≡ just t → optype Γ a ≡ just t
same-ty-l {Γ} {a} {b} e with optype Γ a | optype Γ b
... | just ta | just tb with ta ≟T tb
...   | yes _ with refl ← e = refl

same-ty-r : ∀ {Γ a b t} → same-ty Γ a b ≡ just t → optype Γ b ≡ just t
same-ty-r {Γ} {a} {b} e with optype Γ a | optype Γ b
... | just ta | just tb with ta ≟T tb
...   | yes ta≡tb with refl ← e = sym (cong just ta≡tb)

-- Append one typed output.
_◂_ : TyCtx → (Identifier × IrType) → TyCtx
Γ ◂ e = Γ ++ e ∷ []

-- Append a list of outputs all of the same type (`encode` → native;
-- `div-mod`/`persistent`/`keccak` → native; two-output lists as well).
extN : TyCtx → List Identifier → IrType → TyCtx
extN Γ []         t = Γ
extN Γ (o ∷ os)   t = extN (Γ ◂ (o , t)) os t

-- The static output typing.  `nothing` = ill-typed (an operand's type is
-- undefined or fails the instruction's demand).
outtys : TyCtx → Instruction → Maybe TyCtx
outtys Γ (encode input outputs)        = just (extN Γ outputs native)
outtys Γ (assert _)                    = just Γ
outtys Γ (cond-select bit a b o)       =
  mapᵐ (λ t → Γ ◂ (o , t)) (same-ty Γ a b)
outtys Γ (constrain-bits _ _)          = just Γ
outtys Γ (constrain-eq _ _)            = just Γ
outtys Γ (constrain-to-boolean _)      = just Γ
outtys Γ (copy val o)                  = mapᵐ (λ t → Γ ◂ (o , t)) (optype Γ val)
outtys Γ (impact _ _)                  = just Γ
outtys Γ (ec-mul a _ o)                = mapᵐ (λ t → Γ ◂ (o , t)) (optype Γ a)
-- `genScalarFamily` covers only Jubjub and Secp256k1, unlike every other
-- foreign-curve-dispatching instruction: a `secp256r1-scalar` or
-- `curve25519-scalar` operand is rejected, matching the real Rust's
-- `EcMulGenerator`, which errors on both.
outtys Γ (ec-mul-generator scalar o)   =
  mapᵐ (λ f → Γ ◂ (o , pointTy f)) (optype Γ scalar >>= genScalarFamily)
outtys Γ (hash-to-curve _ o)           = just (Γ ◂ (o , jubjub-point))
outtys Γ (into-coordinates point (xo , yo)) =
  mapᵐ (λ f → extN Γ (xo ∷ yo ∷ []) (coordTy f))
       (optype Γ point >>= pointFamily)
outtys Γ (from-coordinates (xop , yop) o) =
  mapᵐ (λ f → Γ ◂ (o , pointTy f)) (optype Γ xop >>= coordFamily)
outtys Γ (into-bytes32 _ o)            = just (Γ ◂ (o , bytes32))
outtys Γ (from-bytes32 _ val-t o)      = just (Γ ◂ (o , val-t))
outtys Γ (reverse-bytes _ o)           = just (Γ ◂ (o , bytes32))
outtys Γ (bytes32-into-low-high _ (lo , hi)) =
  just (extN Γ (lo ∷ hi ∷ []) native)
outtys Γ (bytes32-from-low-high _ o)   = just (Γ ◂ (o , bytes32))
outtys Γ (div-mod-power-of-two _ _ outs) = just (extN Γ outs native)
outtys Γ (reconstitute-field _ _ _ o)  = just (Γ ◂ (o , native))
outtys Γ (transient-hash _ o)          = just (Γ ◂ (o , native))
outtys Γ (persistent-hash _ _ o)       = just (Γ ◂ (o , bytes32))
outtys Γ (keccak256 _ _ o)             = just (Γ ◂ (o , bytes32))
outtys Γ (test-eq _ _ o)               = just (Γ ◂ (o , native))
outtys Γ (add a b o)                   = mapᵐ (λ t → Γ ◂ (o , t)) (same-ty Γ a b)
outtys Γ (mul a b o)                   = mapᵐ (λ t → Γ ◂ (o , t)) (same-ty Γ a b)
outtys Γ (neg a o)                     = mapᵐ (λ t → Γ ◂ (o , t)) (optype Γ a)
outtys Γ (inv a o)                     = mapᵐ (λ t → Γ ◂ (o , t)) (optype Γ a)
outtys Γ (not _ o)                     = just (Γ ◂ (o , native))
outtys Γ (less-than _ _ _ o)           = just (Γ ◂ (o , native))
outtys Γ (jubjub-scalar-from-native _ o) = just (Γ ◂ (o , jubjub-scalar))
outtys Γ (public-input _ val-t o)      = just (Γ ◂ (o , val-t))
outtys Γ (private-input _ val-t o)     = just (Γ ◂ (o , val-t))
outtys Γ (circuit-output _)            = just Γ

------------------------------------------------------------------------
-- `dom-ty` of an extended context (mirrors `outs-of`).
------------------------------------------------------------------------

-- Appending one entry appends its key.
dom-ty-◂ : ∀ Γ o t → dom-ty (Γ ◂ (o , t)) ≡ dom-ty Γ ++ (o ∷ [])
dom-ty-◂ Γ o t = map-++ proj₁ Γ ((o , t) ∷ [])
  where open import Data.List.Properties using (map-++)

-- Appending a same-typed list appends the list of keys.
dom-ty-extN : ∀ Γ ids t → dom-ty (extN Γ ids t) ≡ dom-ty Γ ++ ids
dom-ty-extN Γ []         t = sym (++-identityʳ (dom-ty Γ))
dom-ty-extN Γ (o ∷ ids)  t =
  trans (dom-ty-extN (Γ ◂ (o , t)) ids t)
    (trans (cong (_++ ids) (dom-ty-◂ Γ o t)) (++-assoc (dom-ty Γ) (o ∷ []) ids))

------------------------------------------------------------------------
-- Two-output and list `TyEq` extension (native values).
------------------------------------------------------------------------

-- Not a member of `Γ`'s extended domain: absent from `Γ` and ≠ the new key.
∉-◂ : ∀ {Γ o t id} → ¬ (id ∈ dom-ty Γ) → ¬ (id ≡ o)
  → ¬ (id ∈ dom-ty (Γ ◂ (o , t)))
∉-◂ {Γ} {o} {t} {id} id∉ id≢o id∈
  with ∈-++⁻ (dom-ty Γ) (subst (id ∈_) (dom-ty-◂ Γ o t) id∈)
... | inj₁ p = id∉ p
... | inj₂ q = id≢o (∈-[x] q)

-- Two distinct fresh native outputs.
tyEq-ins2 : ∀ (m : Mem) (Γ : TyCtx) (i₁ : Identifier) v₁ t₁
              (i₂ : Identifier) v₂ t₂
  → typeof v₁ ≡ t₁ → typeof v₂ ≡ t₂
  → ¬ (i₁ ∈ dom-ty Γ) → ¬ (i₂ ∈ dom-ty Γ) → ¬ (i₂ ≡ i₁)
  → TyEq m Γ
  → TyEq (ins i₂ v₂ (ins i₁ v₁ m)) ((Γ ◂ (i₁ , t₁)) ◂ (i₂ , t₂))
tyEq-ins2 m Γ i₁ v₁ t₁ i₂ v₂ t₂ tv₁ tv₂ i₁∉ i₂∉ i₂≢i₁ teq =
  tyEq-ins (ins i₁ v₁ m) (Γ ◂ (i₁ , t₁)) i₂ v₂ t₂ tv₂
    (∉-◂ i₂∉ i₂≢i₁)
    (tyEq-ins m Γ i₁ v₁ t₁ tv₁ i₁∉ teq)

-- A list of fresh, distinct native outputs (`insertMany`).  Mirrors
-- `domEq-insertMany`: the context grows by the keys (`extN … native`),
-- each cell holding a `val-native` (so `typeof ≡ native` by `refl`).
tyEq-insertManyⁿ : ∀ {Γ} st ids frs {st'}
  → insertMany st ids (map val-native frs) ≡ just st'
  → TyEq (State.mem st) Γ
  → All (λ o → ¬ (o ∈ dom-ty Γ)) ids → NoDup ids
  → TyEq (State.mem st') (extN Γ ids native)
tyEq-insertManyⁿ {Γ} st []         []         refl teq _          _ = teq
tyEq-insertManyⁿ {Γ} st (id ∷ ids) (fr ∷ frs) e    teq (o∉ ∷ afr) (nin , ndup) =
  tyEq-insertManyⁿ {Γ ◂ (id , native)} (out1 st id (val-native fr)) ids frs e
    (tyEq-ins (State.mem st) Γ id (val-native fr) native refl o∉ teq)
    (all-shift ids afr nin) ndup
  where
  all-shift : ∀ ids → All (λ o → ¬ (o ∈ dom-ty Γ)) ids → NotIn id ids
    → All (λ o → ¬ (o ∈ dom-ty (Γ ◂ (id , native)))) ids
  all-shift []         []          _            = []
  all-shift (k ∷ ids)  (k∉ ∷ afr') (id≢k , nin') =
    ∉-◂ k∉ (λ k≡id → id≢k (sym k≡id)) ∷ all-shift ids afr' nin'
tyEq-insertManyⁿ st []        (_ ∷ _)  () _ _ _
tyEq-insertManyⁿ st (_ ∷ _)   []       () _ _ _

------------------------------------------------------------------------
-- Type maintenance across one step.
--
-- Inverting `step` per instruction (mirroring `step-dom`), the memory's
-- typing extends from `Γ` to `Γ'` (the `outtys` result): no-output
-- instructions leave `Γ` unchanged; each output cell holds a value of the
-- statically-predicted type.  For fixed-type outputs (`mul`, hashes, the
-- byte/coordinate splits, …) the value is a literal `val-native`/
-- `val-bytes32`/… so its `typeof` is `refl`; for the type-directed ones
-- (`add`/`neg`/`cond-select`/`copy`) the output type is read off the
-- operands via `tyEq-optype`/`same-ty`.
------------------------------------------------------------------------

-- A no-output step leaves the typing unchanged.
tyEq-nop : ∀ {m Γ Γ'} → just Γ ≡ just Γ' → TyEq m Γ → TyEq m Γ'
tyEq-nop refl teq = teq

step-ty : ∀ {P S st st' Γ Γ'} (i : Instruction)
  → outtys Γ i ≡ just Γ'
  → step P S st i ≡ just st'
  → TyEq (State.mem st) Γ
  → All (λ o → ¬ (o ∈ dom-ty Γ)) (outs-of i)
  → NoDup (outs-of i)
  → TyEq (State.mem st') Γ'
step-ty {st = st} (assert cond) oty e teq _ _
  with resolve𝔹 (State.mem st) cond
... | just true with refl ← e = tyEq-nop oty teq
step-ty {st = st} (constrain-bits val bits) oty e teq _ _
  with resolveᶠ (State.mem st) val
... | just x with valFr x <? 2 ^ bits
...   | yes _ with refl ← e = tyEq-nop oty teq
step-ty {st = st} (constrain-eq a b) oty e teq _ _
  with resolve (State.mem st) a | resolve (State.mem st) b
... | just av | just bv with valEq? av bv
...   | just true with refl ← e = tyEq-nop oty teq
step-ty {st = st} (constrain-to-boolean val) oty e teq _ _
  with resolve𝔹 (State.mem st) val
... | just _ with refl ← e = tyEq-nop oty teq
step-ty {P = P} {st = st} (impact guard inputs) oty e teq _ _
  with resolve-all-Fr (State.mem st) inputs
... | just vals with resolve𝔹 (State.mem st) guard
...   | just g with g
...     | true
          with take (length vals)
                 (drop (State.pti-idx st)
                   (ProofPreimage.pub-transcript-inputs P))
               ≟LFr vals
...       | yes _ with refl ← e = tyEq-nop oty teq
step-ty {st = st} (impact guard inputs) oty e teq _ _
  | just vals | just g | false with refl ← e = tyEq-nop oty teq
step-ty {S = S} {st = st} (circuit-output vals) oty e teq _ _
  with collectOutputs (State.mem st) (IrSource.outputs S) vals
... | just vs with refl ← e = tyEq-nop oty teq
-- `ec-mul`'s output shares the operand point's type (like `add`/`neg`).
step-ty {st = st} {Γ = Γ} (ec-mul a scalar output) oty e teq (o∉ ∷ _) _
  with optype Γ a in oat
... | just t with refl ← oty
  with resolve (State.mem st) a in ra | resolve (State.mem st) scalar
...   | just (val-jubjub-point p) | just (val-jubjub-scalar s) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
...   | just (val-secp256k1-point p) | just (val-secp256k1-scalar s) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
...   | just (val-secp256r1-point p) | just (val-secp256r1-scalar s)
        with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
...   | just (val-curve25519-point p) | just (val-curve25519-scalar s)
        with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
-- `ec-mul-generator`'s output is the point type matching the scalar's
-- curve.  The operand's static type need not be matched on: `tyEq-optype`
-- identifies it with the resolved scalar's own type, which is the type
-- `outtys` reads the curve off, so `oty` then fixes `Γ'` outright.
step-ty {st = st} {Γ = Γ} (ec-mul-generator scalar output) oty e teq (o∉ ∷ _) _
  with resolve (State.mem st) scalar in rs
... | just (val-jubjub-scalar s) with optype Γ scalar in ost
...   | just _ with refl ← tyEq-optype {op = scalar} teq ost rs
                 with refl ← oty with refl ← e =
          tyEq-ins (State.mem st) Γ output _ jubjub-point refl o∉ teq
step-ty {st = st} {Γ = Γ} (ec-mul-generator scalar output) oty e teq (o∉ ∷ _) _
  | just (val-secp256k1-scalar s) with optype Γ scalar in ost
...   | just _ with refl ← tyEq-optype {op = scalar} teq ost rs
                 with refl ← oty with refl ← e =
          tyEq-ins (State.mem st) Γ output _ secp256k1-point refl o∉ teq
step-ty {st = st} (hash-to-curve inputs output) oty e teq (o∉ ∷ _) _
  with refl ← oty with resolve-all-Fr (State.mem st) inputs
... | just frs with refl ← e =
        tyEq-ins (State.mem st) _ output _ jubjub-point refl o∉ teq
step-ty {st = st} {Γ = Γ} (from-coordinates (xop , yop) output) oty e teq
  (o∉ ∷ _) _
  with resolve (State.mem st) xop in rx | resolve (State.mem st) yop
... | just (val-native x) | just (val-native y) with fromCoordsJ x y
...   | just p with optype Γ xop in oxt
...     | just _ with refl ← tyEq-optype {op = xop} teq oxt rx
                   with refl ← oty with refl ← e =
          tyEq-ins (State.mem st) Γ output _ jubjub-point refl o∉ teq
step-ty {st = st} {Γ = Γ} (from-coordinates (xop , yop) output) oty e teq
  (o∉ ∷ _) _
  | just (val-secp256k1-base x) | just (val-secp256k1-base y) with fromCoordsK1 x y
...   | just p with optype Γ xop in oxt
...     | just _ with refl ← tyEq-optype {op = xop} teq oxt rx
                   with refl ← oty with refl ← e =
          tyEq-ins (State.mem st) Γ output _ secp256k1-point refl o∉ teq
step-ty {st = st} {Γ = Γ} (from-coordinates (xop , yop) output) oty e teq
  (o∉ ∷ _) _
  | just (val-secp256r1-base x) | just (val-secp256r1-base y)
      with fromCoordsP x y
...   | just p with optype Γ xop in oxt
...     | just _ with refl ← tyEq-optype {op = xop} teq oxt rx
                   with refl ← oty with refl ← e =
          tyEq-ins (State.mem st) Γ output _ secp256r1-point refl o∉ teq
step-ty {st = st} {Γ = Γ} (from-coordinates (xop , yop) output) oty e teq
  (o∉ ∷ _) _
  | just (val-curve25519-base x) | just (val-curve25519-base y)
      with fromCoordsC x y
...   | just p with optype Γ xop in oxt
...     | just _ with refl ← tyEq-optype {op = xop} teq oxt rx
                   with refl ← oty with refl ← e =
          tyEq-ins (State.mem st) Γ output _ curve25519-point refl o∉ teq
step-ty {st = st} (into-bytes32 input output) oty e teq (o∉ ∷ _) _
  with refl ← oty with resolve (State.mem st) input
... | just (val-native x) with refl ← e =
        tyEq-ins (State.mem st) _ output _ bytes32 refl o∉ teq
... | just (val-secp256k1-base x) with refl ← e =
        tyEq-ins (State.mem st) _ output _ bytes32 refl o∉ teq
... | just (val-secp256k1-scalar s) with refl ← e =
        tyEq-ins (State.mem st) _ output _ bytes32 refl o∉ teq
... | just (val-secp256r1-base x) with refl ← e =
        tyEq-ins (State.mem st) _ output _ bytes32 refl o∉ teq
... | just (val-secp256r1-scalar s) with refl ← e =
        tyEq-ins (State.mem st) _ output _ bytes32 refl o∉ teq
... | just (val-curve25519-base x) with refl ← e =
        tyEq-ins (State.mem st) _ output _ bytes32 refl o∉ teq
... | just (val-curve25519-scalar s) with refl ← e =
        tyEq-ins (State.mem st) _ output _ bytes32 refl o∉ teq
step-ty {st = st} (bytes32-from-low-high (loop , hiop) output) oty e teq (o∉ ∷ _) _
  with refl ← oty
  with resolveᶠ (State.mem st) loop | resolveᶠ (State.mem st) hiop
... | just lo | just hi with low-high→bytes32 lo hi
...   | just b with refl ← e =
        tyEq-ins (State.mem st) _ output _ bytes32 refl o∉ teq
step-ty {st = st} (reconstitute-field divisor modulus bits output) oty e teq
  (o∉ ∷ _) _
  with refl ← oty
  with resolveᶠ (State.mem st) divisor | resolveᶠ (State.mem st) modulus
... | just dd | just mo with valFr mo <? 2 ^ bits
...   | yes _ with valFr dd <? 2 ^ (FR-BITS ∸ bits)
...     | yes _ with valFr mo + 2 ^ bits * valFr dd <? FR-ORDER
...       | yes _ with refl ← e =
            tyEq-ins (State.mem st) _ output _ native refl o∉ teq
step-ty {st = st} (transient-hash inputs output) oty e teq (o∉ ∷ _) _
  with refl ← oty with resolve-all-Fr (State.mem st) inputs
... | just frs with refl ← e =
        tyEq-ins (State.mem st) _ output _ native refl o∉ teq
step-ty {st = st} (test-eq a b output) oty e teq (o∉ ∷ _) _
  with refl ← oty with resolve (State.mem st) a | resolve (State.mem st) b
... | just av | just bv with valEq? av bv
...   | just eqv with refl ← e =
        tyEq-ins (State.mem st) _ output _ native refl o∉ teq
step-ty {st = st} {Γ = Γ} (mul a b output) oty e teq (o∉ ∷ _) _
  with same-ty Γ a b in sty
... | just t with refl ← oty
  with resolve (State.mem st) a in ra | resolve (State.mem st) b in rb
...   | just (val-native x) | just (val-native y) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
...   | just (val-secp256k1-base x) | just (val-secp256k1-base y) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
...   | just (val-secp256k1-scalar x) | just (val-secp256k1-scalar y) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
...   | just (val-secp256r1-base x) | just (val-secp256r1-base y)
        with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
...   | just (val-secp256r1-scalar x) | just (val-secp256r1-scalar y)
        with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
...   | just (val-curve25519-base x) | just (val-curve25519-base y)
        with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
...   | just (val-curve25519-scalar x) | just (val-curve25519-scalar y)
        with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
step-ty {st = st} {Γ = Γ} (inv a output) oty e teq (o∉ ∷ _) _
  with resolve (State.mem st) a in ra
... | just (val-native x) with invᶠ x
...   | just xi with optype Γ a in oat
...     | just t with refl ← oty with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
step-ty {st = st} {Γ = Γ} (inv a output) oty e teq (o∉ ∷ _) _
  | just (val-secp256k1-base x) with invK1ᵇ x
...   | just xi with optype Γ a in oat
...     | just t with refl ← oty with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
step-ty {st = st} {Γ = Γ} (inv a output) oty e teq (o∉ ∷ _) _
  | just (val-secp256k1-scalar x) with invK1ˢ x
...   | just xi with optype Γ a in oat
...     | just t with refl ← oty with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
step-ty {st = st} {Γ = Γ} (inv a output) oty e teq (o∉ ∷ _) _
  | just (val-secp256r1-base x) with invPᵇ x
...   | just xi with optype Γ a in oat
...     | just t with refl ← oty with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
step-ty {st = st} {Γ = Γ} (inv a output) oty e teq (o∉ ∷ _) _
  | just (val-secp256r1-scalar x) with invPˢ x
...   | just xi with optype Γ a in oat
...     | just t with refl ← oty with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
step-ty {st = st} {Γ = Γ} (inv a output) oty e teq (o∉ ∷ _) _
  | just (val-curve25519-base x) with invCᵇ x
...   | just xi with optype Γ a in oat
...     | just t with refl ← oty with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
step-ty {st = st} {Γ = Γ} (inv a output) oty e teq (o∉ ∷ _) _
  | just (val-curve25519-scalar x) with invCˢ x
...   | just xi with optype Γ a in oat
...     | just t with refl ← oty with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
step-ty {st = st} (not a output) oty e teq (o∉ ∷ _) _
  with refl ← oty with resolve𝔹 (State.mem st) a
... | just b with refl ← e =
        tyEq-ins (State.mem st) _ output _ native refl o∉ teq
step-ty {st = st} (less-than a b bits output) oty e teq (o∉ ∷ _) _
  with refl ← oty
  with resolveᶠ (State.mem st) a | resolveᶠ (State.mem st) b
... | just x | just y with valFr x <? 2 ^ bits
...   | yes _ with valFr y <? 2 ^ bits
...     | yes _ with refl ← e =
          tyEq-ins (State.mem st) _ output _ native refl o∉ teq
step-ty {st = st} (jubjub-scalar-from-native a output) oty e teq (o∉ ∷ _) _
  with refl ← oty with resolveᶠ (State.mem st) a
... | just x with refl ← e =
        tyEq-ins (State.mem st) _ output _ jubjub-scalar refl o∉ teq
step-ty {st = st} (from-bytes32 bytes native output) oty e teq (o∉ ∷ _) _
  with refl ← oty with resolve (State.mem st) bytes
... | just (val-bytes32 b) with refl ← e =
        tyEq-ins (State.mem st) _ output _ native refl o∉ teq
step-ty {st = st} (from-bytes32 bytes secp256k1-base output) oty e teq (o∉ ∷ _) _
  with refl ← oty with resolve (State.mem st) bytes
... | just (val-bytes32 b) with refl ← e =
        tyEq-ins (State.mem st) _ output _ secp256k1-base refl o∉ teq
step-ty {st = st} (from-bytes32 bytes secp256k1-scalar output) oty e teq (o∉ ∷ _) _
  with refl ← oty with resolve (State.mem st) bytes
... | just (val-bytes32 b) with refl ← e =
        tyEq-ins (State.mem st) _ output _ secp256k1-scalar refl o∉ teq
step-ty {st = st} (from-bytes32 bytes secp256r1-base output) oty e teq
  (o∉ ∷ _) _
  with refl ← oty with resolve (State.mem st) bytes
... | just (val-bytes32 b) with refl ← e =
        tyEq-ins (State.mem st) _ output _ secp256r1-base refl o∉ teq
step-ty {st = st} (from-bytes32 bytes secp256r1-scalar output) oty e teq
  (o∉ ∷ _) _
  with refl ← oty with resolve (State.mem st) bytes
... | just (val-bytes32 b) with refl ← e =
        tyEq-ins (State.mem st) _ output _ secp256r1-scalar refl o∉ teq
step-ty {st = st} (from-bytes32 bytes curve25519-base output) oty e teq
  (o∉ ∷ _) _
  with refl ← oty with resolve (State.mem st) bytes
... | just (val-bytes32 b) with refl ← e =
        tyEq-ins (State.mem st) _ output _ curve25519-base refl o∉ teq
step-ty {st = st} (from-bytes32 bytes curve25519-scalar output) oty e teq
  (o∉ ∷ _) _
  with refl ← oty with resolve (State.mem st) bytes
... | just (val-bytes32 b) with refl ← e =
        tyEq-ins (State.mem st) _ output _ curve25519-scalar refl o∉ teq
step-ty {st = st} (from-bytes32 bytes secp256k1-point output) oty e teq _ _
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) = case e of λ ()
step-ty {st = st} (from-bytes32 bytes bytes32 output) oty e teq _ _
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) = case e of λ ()
step-ty {st = st} (from-bytes32 bytes jubjub-point output) oty e teq _ _
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) = case e of λ ()
step-ty {st = st} (from-bytes32 bytes jubjub-scalar output) oty e teq _ _
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) = case e of λ ()
step-ty {st = st} (from-bytes32 bytes secp256r1-point output) oty e teq _ _
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) = case e of λ ()
step-ty {st = st} (from-bytes32 bytes curve25519-point output) oty e teq _ _
  with resolve (State.mem st) bytes
... | just (val-bytes32 b) = case e of λ ()
step-ty {st = st} (reverse-bytes bytes output) oty e teq (o∉ ∷ _) _
  with refl ← oty with resolve (State.mem st) bytes
... | just (val-bytes32 b) with refl ← e =
        tyEq-ins (State.mem st) _ output _ bytes32 refl o∉ teq
-- Transcript inputs: the bound value has the declared type `val-t` by
-- `decode-typeof` (active) or `default-typeof` (inactive).
step-ty {st = st} (public-input guard val-t output) oty e teq (o∉ ∷ _) _
  with refl ← oty with eval-guard (State.mem st) guard
... | just true with decode val-t (take (encoded-len val-t) (State.pto-rem st))
                     in edec
...   | just v with refl ← e =
        tyEq-ins (State.mem st) _ output v val-t
          (decode-typeof val-t _ edec) o∉ teq
step-ty {st = st} (public-input guard val-t output) oty e teq (o∉ ∷ _) _
    | just false with refl ← e =
        tyEq-ins (State.mem st) _ output _ val-t (default-typeof val-t) o∉ teq
step-ty {st = st} (private-input guard val-t output) oty e teq (o∉ ∷ _) _
  with refl ← oty with eval-guard (State.mem st) guard
... | just true with decode val-t (take (encoded-len val-t) (State.priv-rem st))
                     in edec
...   | just v with refl ← e =
        tyEq-ins (State.mem st) _ output v val-t
          (decode-typeof val-t _ edec) o∉ teq
step-ty {st = st} (private-input guard val-t output) oty e teq (o∉ ∷ _) _
    | just false with refl ← e =
        tyEq-ins (State.mem st) _ output _ val-t (default-typeof val-t) o∉ teq
-- Type-directed single outputs: read the output type off the operands.
step-ty {st = st} {Γ = Γ} (copy val output) oty e teq (o∉ ∷ _) _
  with optype Γ val in ovt
... | just t with refl ← oty with resolve (State.mem st) val in rv
...   | just v with refl ← e =
        tyEq-ins (State.mem st) Γ output v t
          (tyEq-optype {op = val} teq ovt rv) o∉ teq
step-ty {st = st} {Γ = Γ} (add a b output) oty e teq (o∉ ∷ _) _
  with same-ty Γ a b in sty
... | just t with refl ← oty
  with resolve (State.mem st) a in ra | resolve (State.mem st) b in rb
...   | just (val-native x) | just (val-native y) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
...   | just (val-jubjub-point p) | just (val-jubjub-point q) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
...   | just (val-secp256k1-point p) | just (val-secp256k1-point q) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
...   | just (val-secp256k1-base x) | just (val-secp256k1-base y) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
...   | just (val-secp256k1-scalar x) | just (val-secp256k1-scalar y) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
...   | just (val-secp256r1-point p) | just (val-secp256r1-point q)
        with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
...   | just (val-secp256r1-base x) | just (val-secp256r1-base y)
        with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
...   | just (val-secp256r1-scalar x) | just (val-secp256r1-scalar y)
        with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
...   | just (val-curve25519-point p) | just (val-curve25519-point q)
        with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
...   | just (val-curve25519-base x) | just (val-curve25519-base y)
        with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
...   | just (val-curve25519-scalar x) | just (val-curve25519-scalar y)
        with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra) o∉ teq
step-ty {st = st} {Γ = Γ} (neg a output) oty e teq (o∉ ∷ _) _
  with optype Γ a in oat
... | just t with refl ← oty with resolve (State.mem st) a in ra
...   | just (val-native x) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
...   | just (val-jubjub-point p) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
...   | just (val-secp256k1-point p) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
...   | just (val-secp256k1-base x) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
...   | just (val-secp256k1-scalar x) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
...   | just (val-secp256r1-point p) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
...   | just (val-secp256r1-base x) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
...   | just (val-secp256r1-scalar x) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
...   | just (val-curve25519-point p) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
...   | just (val-curve25519-base x) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
...   | just (val-curve25519-scalar x) with refl ← e =
          tyEq-ins (State.mem st) Γ output _ t
            (tyEq-optype {op = a} teq oat ra) o∉ teq
step-ty {st = st} {Γ = Γ} (cond-select bit a b output) oty e teq (o∉ ∷ _) _
  with same-ty Γ a b in sty
... | just t with refl ← oty with resolve𝔹 (State.mem st) bit
...   | just bv with resolve (State.mem st) a in ra
                   | resolve (State.mem st) b in rb
...     | just av | just bvl with typeof av ≟T typeof bvl
...       | yes _ with refl ← e =
            tyEq-ins (State.mem st) Γ output _ t (tyof-if bv) o∉ teq
  where
  -- The selected value has type `t` whichever branch is taken (both `a`
  -- and `b` share the type `t`).
  tyof-if : ∀ bv → typeof (if bv then av else bvl) ≡ t
  tyof-if true  = tyEq-optype {op = a} teq (same-ty-l {Γ} {a} {b} sty) ra
  tyof-if false = tyEq-optype {op = b} teq (same-ty-r {Γ} {a} {b} sty) rb
-- Two distinct outputs whose type is fixed by the point operand's curve.
step-ty {st = st} {Γ = Γ} (into-coordinates point (xid , yid)) oty e teq
  (x∉ ∷ y∉ ∷ []) ((x≢y , _) , _)
  with resolve (State.mem st) point in rp
... | just (val-jubjub-point p) with coordsJ p
...   | (x , y) with optype Γ point in opt
...     | just _ with refl ← tyEq-optype {op = point} teq opt rp
                    with refl ← oty with refl ← e =
          tyEq-ins2 (State.mem st) Γ xid (val-native x) native
            yid (val-native y) native refl refl x∉ y∉
            (λ y≡x → x≢y (sym y≡x)) teq
step-ty {st = st} {Γ = Γ} (into-coordinates point (xid , yid)) oty e teq
  (x∉ ∷ y∉ ∷ []) ((x≢y , _) , _)
  | just (val-secp256k1-point p) with coordsK1 p
...   | just (x , y) with optype Γ point in opt
...     | just _ with refl ← tyEq-optype {op = point} teq opt rp
                    with refl ← oty with refl ← e =
          tyEq-ins2 (State.mem st) Γ xid (val-secp256k1-base x) secp256k1-base
            yid (val-secp256k1-base y) secp256k1-base refl refl x∉ y∉
            (λ y≡x → x≢y (sym y≡x)) teq
step-ty {st = st} {Γ = Γ} (into-coordinates point (xid , yid)) oty e teq
  (x∉ ∷ y∉ ∷ []) ((x≢y , _) , _)
  | just (val-secp256r1-point p) with coordsP p
...   | just (x , y) with optype Γ point in opt
...     | just _ with refl ← tyEq-optype {op = point} teq opt rp
                    with refl ← oty with refl ← e =
          tyEq-ins2 (State.mem st) Γ xid (val-secp256r1-base x) secp256r1-base
            yid (val-secp256r1-base y) secp256r1-base refl refl x∉ y∉
            (λ y≡x → x≢y (sym y≡x)) teq
-- Total coordinate split: the twisted-Edwards identity has real affine
-- coordinates, so `coordsC` yields a pair outright (no `just`).
step-ty {st = st} {Γ = Γ} (into-coordinates point (xid , yid)) oty e teq
  (x∉ ∷ y∉ ∷ []) ((x≢y , _) , _)
  | just (val-curve25519-point p) with coordsC p
...   | (x , y) with optype Γ point in opt
...     | just _ with refl ← tyEq-optype {op = point} teq opt rp
                    with refl ← oty with refl ← e =
          tyEq-ins2 (State.mem st) Γ xid (val-curve25519-base x)
            curve25519-base
            yid (val-curve25519-base y) curve25519-base refl refl x∉ y∉
            (λ y≡x → x≢y (sym y≡x)) teq
step-ty {st = st} (bytes32-into-low-high bytes (loid , hiid)) oty e teq
  (l∉ ∷ h∉ ∷ []) ((l≢h , _) , _)
  with refl ← oty with resolve (State.mem st) bytes
... | just (val-bytes32 b) with bytes32→low-high b
...   | (lo , hi) with refl ← e =
          tyEq-ins2 (State.mem st) _ loid (val-native lo) native
            hiid (val-native hi) native refl refl l∉ h∉
            (λ h≡l → l≢h (sym h≡l)) teq
-- List native outputs (`insertMany`).
step-ty {st = st} (encode input outputs) oty e teq af nd
  with refl ← oty with resolve (State.mem st) input
... | just v =
      tyEq-insertManyⁿ st outputs (encodeᵉ v) e teq af nd
step-ty {st = st} (div-mod-power-of-two val bits outs) oty e teq af nd
  with refl ← oty with resolveᶠ (State.mem st) val
... | just x =
      tyEq-insertManyⁿ st outs
        ( from-le-bits (drop bits (to-le-bits x))
        ∷ from-le-bits (take bits (to-le-bits x)) ∷ [])
        e teq af nd
step-ty {st = st} (persistent-hash alignment inputs output) oty e teq (o∉ ∷ _) _
  with refl ← oty with resolve-all-Fr (State.mem st) inputs
... | just frs with persistent-hash-fn alignment frs
...   | just h with refl ← e =
        tyEq-ins (State.mem st) _ output _ bytes32 refl o∉ teq
step-ty {st = st} (keccak256 alignment inputs output) oty e teq (o∉ ∷ _) _
  with refl ← oty with resolve-all-Fr (State.mem st) inputs
... | just frs with keccak-fn alignment frs
...   | just h with refl ← e =
        tyEq-ins (State.mem st) _ output _ bytes32 refl o∉ teq

------------------------------------------------------------------------
-- `outtys` grows the typed domain by exactly `outs-of i`.
--
-- So the single-assignment freshness (`¬∈ bound`, over the SA name set)
-- and the typing freshness (`¬∈ dom-ty Γ`) stay in lock-step: whenever
-- `dom-ty Γ ≡ bound`, the successor context's domain is `bound ++
-- outs-of i` — exactly the SA scan's next bound.
------------------------------------------------------------------------

outtys-dom : ∀ Γ (i : Instruction) {Γ'}
  → outtys Γ i ≡ just Γ'
  → dom-ty Γ' ≡ dom-ty Γ ++ outs-of i
outtys-dom Γ (encode input outputs)  refl = dom-ty-extN Γ outputs native
outtys-dom Γ (assert _)              refl = sym (++-identityʳ (dom-ty Γ))
outtys-dom Γ (cond-select bit a b o) oty with same-ty Γ a b
... | just t with refl ← oty = dom-ty-◂ Γ o t
outtys-dom Γ (constrain-bits _ _)    refl = sym (++-identityʳ (dom-ty Γ))
outtys-dom Γ (constrain-eq _ _)      refl = sym (++-identityʳ (dom-ty Γ))
outtys-dom Γ (constrain-to-boolean _) refl = sym (++-identityʳ (dom-ty Γ))
outtys-dom Γ (copy val o)            oty with optype Γ val
... | just t with refl ← oty = dom-ty-◂ Γ o t
outtys-dom Γ (impact _ _)            refl = sym (++-identityʳ (dom-ty Γ))
outtys-dom Γ (ec-mul a _ o)          oty with optype Γ a
... | just t with refl ← oty = dom-ty-◂ Γ o t
outtys-dom Γ (ec-mul-generator scalar o) oty
  with optype Γ scalar >>= genScalarFamily
... | just f with refl ← oty = dom-ty-◂ Γ o (pointTy f)
outtys-dom Γ (hash-to-curve _ o)     refl = dom-ty-◂ Γ o jubjub-point
outtys-dom Γ (into-coordinates point (xo , yo)) oty
  with optype Γ point >>= pointFamily
... | just f with refl ← oty = dom-ty-extN Γ (xo ∷ yo ∷ []) (coordTy f)
outtys-dom Γ (from-coordinates (xop , yop) o) oty
  with optype Γ xop >>= coordFamily
... | just f with refl ← oty = dom-ty-◂ Γ o (pointTy f)
outtys-dom Γ (into-bytes32 _ o)      refl = dom-ty-◂ Γ o bytes32
outtys-dom Γ (from-bytes32 _ vt o)   refl = dom-ty-◂ Γ o vt
outtys-dom Γ (reverse-bytes _ o)     refl = dom-ty-◂ Γ o bytes32
outtys-dom Γ (bytes32-into-low-high _ (lo , hi)) refl =
  dom-ty-extN Γ (lo ∷ hi ∷ []) native
outtys-dom Γ (bytes32-from-low-high _ o) refl = dom-ty-◂ Γ o bytes32
outtys-dom Γ (div-mod-power-of-two _ _ outs) refl = dom-ty-extN Γ outs native
outtys-dom Γ (reconstitute-field _ _ _ o) refl = dom-ty-◂ Γ o native
outtys-dom Γ (transient-hash _ o)    refl = dom-ty-◂ Γ o native
outtys-dom Γ (persistent-hash _ _ o) refl = dom-ty-◂ Γ o bytes32
outtys-dom Γ (keccak256 _ _ o)       refl = dom-ty-◂ Γ o bytes32
outtys-dom Γ (test-eq _ _ o)         refl = dom-ty-◂ Γ o native
outtys-dom Γ (add a b o)             oty with same-ty Γ a b
... | just t with refl ← oty = dom-ty-◂ Γ o t
outtys-dom Γ (mul a b o)             oty with same-ty Γ a b
... | just t with refl ← oty = dom-ty-◂ Γ o t
outtys-dom Γ (neg a o)               oty with optype Γ a
... | just t with refl ← oty = dom-ty-◂ Γ o t
outtys-dom Γ (inv a o)               oty with optype Γ a
... | just t with refl ← oty = dom-ty-◂ Γ o t
outtys-dom Γ (not _ o)               refl = dom-ty-◂ Γ o native
outtys-dom Γ (less-than _ _ _ o)     refl = dom-ty-◂ Γ o native
outtys-dom Γ (jubjub-scalar-from-native _ o) refl = dom-ty-◂ Γ o jubjub-scalar
outtys-dom Γ (public-input _ vt o)   refl = dom-ty-◂ Γ o vt
outtys-dom Γ (private-input _ vt o)  refl = dom-ty-◂ Γ o vt
outtys-dom Γ (circuit-output _)      refl = sym (++-identityʳ (dom-ty Γ))

------------------------------------------------------------------------
-- Operand typing precondition.
--
-- `outtys` fixes each instruction's OUTPUT types but, for the fixed-type
-- instructions, does not constrain the operands.  `OpTy Γ i` records the
-- operand types the off-circuit `step` and the backward lemma of `i`
-- demand: which operands must be Native, which carry a specific type,
-- and — for `add`/`neg` — that the shared operand type is a field or a
-- point, or — for `constrain-eq` — that the compared operand's type
-- supports the reflexive equality test (Native / Bytes32 / JubjubPoint,
-- never JubjubScalar).  Bundled with `WT`, it makes `producer-WT`
-- guarantee every operand reference is bound to a value of the required
-- type — the fact the backward driver needs to resolve each operand at
-- the pre-step memory.
------------------------------------------------------------------------

-- An operand carries type `t` under `Γ`.
Opᵗ : TyCtx → Operand → IrType → Set
Opᵗ Γ op t = optype Γ op ≡ just t

-- An operand is typed (bound) under `Γ`.
OpDefd : TyCtx → Operand → Set
OpDefd Γ op = ∃ λ t → optype Γ op ≡ just t

-- Every operand of a list is Native.
AllNatᵒ : TyCtx → List Operand → Set
AllNatᵒ Γ = All (λ op → Opᵗ Γ op native)

-- Every operand of a list is defined (typed at some type).  Used for
-- `circuit-output`, whose operands carry no other constraint but must be
-- bound so the off-circuit `collectOutputs` resolves them.
AllDefdᵒ : TyCtx → List Operand → Set
AllDefdᵒ Γ = All (OpDefd Γ)

-- An all-Native operand list resolves to a field-element list.
allNat-resolve : ∀ {m Γ} ops → TyEq m Γ → AllNatᵒ Γ ops
  → ∃ λ frs → resolve-all-Fr m ops ≡ just frs
allNat-resolve []         teq []         = [] , refl
allNat-resolve {m} (op ∷ ops) teq (oN ∷ aN)
  with optype-resolveᶠ {op = op} teq oN | allNat-resolve ops teq aN
... | (x , rx) | (frs , rfrs) = x ∷ frs , cons
  where
  cons : resolve-all-Fr m (op ∷ ops) ≡ just (x ∷ frs)
  cons rewrite rx | rfrs = refl

-- An optional guard, if present, is Native.
GuardNat : TyCtx → Maybe Operand → Set
GuardNat Γ nothing   = ⊤
GuardNat Γ (just op) = Opᵗ Γ op native

OpTy : TyCtx → Instruction → Set
OpTy Γ (encode input _)              = OpDefd Γ input
OpTy Γ (assert cond)                 = Opᵗ Γ cond native
OpTy Γ (cond-select bit _ _ _)       = Opᵗ Γ bit native
OpTy Γ (constrain-bits val _)        = Opᵗ Γ val native
OpTy Γ (constrain-eq a b)            =
  (∃ λ t → Opᵗ Γ a t × SuppEq t) × OpDefd Γ b
OpTy Γ (constrain-to-boolean val)    = Opᵗ Γ val native
OpTy Γ (copy val _)                  = OpDefd Γ val
OpTy Γ (impact guard inputs)         = Opᵗ Γ guard native × AllNatᵒ Γ inputs
OpTy Γ (ec-mul a scalar _)           =
    (Opᵗ Γ a jubjub-point × Opᵗ Γ scalar jubjub-scalar)
  ⊎ (Opᵗ Γ a secp256k1-point × Opᵗ Γ scalar secp256k1-scalar)
  ⊎ (Opᵗ Γ a secp256r1-point × Opᵗ Γ scalar secp256r1-scalar)
  ⊎ (Opᵗ Γ a curve25519-point × Opᵗ Γ scalar curve25519-scalar)
-- `EcMulGenerator` is NOT extended to Secp256r1 or Curve25519 in the Rust
-- (unlike every other foreign-curve-dispatching instruction) — only
-- Jubjub/Secp256k1 scalars are supported.
OpTy Γ (ec-mul-generator scalar _)   =
  Anyᶠ (λ f → Opᵗ Γ scalar (scalarTy f)) gen-families
OpTy Γ (hash-to-curve inputs _)      = AllNatᵒ Γ inputs
OpTy Γ (into-coordinates point _)    =
  Anyᶠ (λ f → Opᵗ Γ point (pointTy f)) ec-families
OpTy Γ (from-coordinates (x , y) _)  =
  Anyᶠ (λ f → Opᵗ Γ x (coordTy f) × Opᵗ Γ y (coordTy f)) ec-families
OpTy Γ (into-bytes32 input _)        = ∃ λ t → Opᵗ Γ input t × FM t
OpTy Γ (from-bytes32 bytes _ _)      = Opᵗ Γ bytes bytes32
OpTy Γ (reverse-bytes bytes _)       = Opᵗ Γ bytes bytes32
OpTy Γ (bytes32-into-low-high b _)   = Opᵗ Γ b bytes32
OpTy Γ (bytes32-from-low-high (lo , hi) _) =
  Opᵗ Γ lo native × Opᵗ Γ hi native
OpTy Γ (div-mod-power-of-two val _ _) = Opᵗ Γ val native
OpTy Γ (reconstitute-field d m _ _)  = Opᵗ Γ d native × Opᵗ Γ m native
OpTy Γ (transient-hash inputs _)     = AllNatᵒ Γ inputs
OpTy Γ (persistent-hash _ inputs _)  = AllNatᵒ Γ inputs
OpTy Γ (keccak256 _ inputs _)        = AllNatᵒ Γ inputs
OpTy Γ (test-eq a b _)               = OpDefd Γ a × OpDefd Γ b
OpTy Γ (add a b _)                   =
  ∃ λ t → Opᵗ Γ a t × Opᵗ Γ b t × FN t
OpTy Γ (mul a b _)                   =
  ∃ λ t → Opᵗ Γ a t × Opᵗ Γ b t × FM t
OpTy Γ (neg a _)                     = ∃ λ t → Opᵗ Γ a t × FN t
OpTy Γ (inv a _)                     = ∃ λ t → Opᵗ Γ a t × FM t
OpTy Γ (not a _)                     = Opᵗ Γ a native
OpTy Γ (less-than a b _ _)           = Opᵗ Γ a native × Opᵗ Γ b native
OpTy Γ (jubjub-scalar-from-native a _) = Opᵗ Γ a native
OpTy Γ (public-input guard _ _)      = GuardNat Γ guard
OpTy Γ (private-input guard _ _)     = GuardNat Γ guard
OpTy Γ (circuit-output vals)         = AllDefdᵒ Γ vals

------------------------------------------------------------------------
-- `OpTy` is decidable (a runnable operand-typing check).
------------------------------------------------------------------------

opᵗ? : ∀ Γ op t → Dec (Opᵗ Γ op t)
opᵗ? Γ op t = ≡-dec _≟T_ (optype Γ op) (just t)

opDefd? : ∀ Γ op → Dec (OpDefd Γ op)
opDefd? Γ op with optype Γ op
... | just t  = yes (t , refl)
... | nothing = no λ { (_ , ()) }

allNatᵒ? : ∀ Γ ops → Dec (AllNatᵒ Γ ops)
allNatᵒ? Γ = all? (λ op → opᵗ? Γ op native)

allDefdᵒ? : ∀ Γ ops → Dec (AllDefdᵒ Γ ops)
allDefdᵒ? Γ = all? (opDefd? Γ)

FN? : ∀ t → Dec (FN t)
FN? t = (t ≟T native) ⊎-dec ((t ≟T jubjub-point) ⊎-dec ((t ≟T secp256k1-point)
      ⊎-dec ((t ≟T secp256k1-base) ⊎-dec ((t ≟T secp256k1-scalar)
      ⊎-dec ((t ≟T secp256r1-point) ⊎-dec ((t ≟T secp256r1-base)
      ⊎-dec ((t ≟T secp256r1-scalar) ⊎-dec ((t ≟T curve25519-point)
      ⊎-dec ((t ≟T curve25519-base)
      ⊎-dec (t ≟T curve25519-scalar))))))))))

FM? : ∀ t → Dec (FM t)
FM? t = (t ≟T native) ⊎-dec ((t ≟T secp256k1-base) ⊎-dec ((t ≟T secp256k1-scalar)
      ⊎-dec ((t ≟T secp256r1-base) ⊎-dec ((t ≟T secp256r1-scalar)
      ⊎-dec ((t ≟T curve25519-base) ⊎-dec (t ≟T curve25519-scalar))))))

SuppEq? : ∀ t → Dec (SuppEq t)
SuppEq? t = (t ≟T native) ⊎-dec ((t ≟T bytes32) ⊎-dec ((t ≟T jubjub-point)
        ⊎-dec ((t ≟T secp256k1-point) ⊎-dec ((t ≟T secp256k1-base)
        ⊎-dec ((t ≟T secp256k1-scalar) ⊎-dec ((t ≟T secp256r1-point)
        ⊎-dec ((t ≟T secp256r1-base) ⊎-dec ((t ≟T secp256r1-scalar)
        ⊎-dec ((t ≟T curve25519-point) ⊎-dec ((t ≟T curve25519-base)
        ⊎-dec (t ≟T curve25519-scalar)))))))))))

guardNat? : ∀ Γ g → Dec (GuardNat Γ g)
guardNat? Γ nothing   = yes tt
guardNat? Γ (just op) = opᵗ? Γ op native

-- Single-operand refinement (`neg`, `inv`, `constrain-eq`, `into-bytes32`):
-- an operand carrying a type in a decidable predicate.
opp? : ∀ {P : IrType → Set} → (∀ t → Dec (P t)) → ∀ Γ a
  → Dec (∃ λ t → Opᵗ Γ a t × P t)
opp? P? Γ a with optype Γ a
... | just t  = map′ (λ p → t , refl , p) (λ { (_ , refl , p) → p }) (P? t)
... | nothing = no λ { (_ , () , _) }

-- Two-operand refinement (`add`, `mul`): both operands share a type in a
-- decidable predicate.
opp2? : ∀ {P : IrType → Set} → (∀ t → Dec (P t)) → ∀ Γ a b
  → Dec (∃ λ t → Opᵗ Γ a t × Opᵗ Γ b t × P t)
opp2? P? Γ a b with optype Γ a | optype Γ b
... | nothing | _       = no λ { (_ , () , _ , _) }
... | just _  | nothing = no λ { (_ , _ , () , _) }
... | just ta | just tb with ta ≟T tb
...   | no ta≢tb = no λ { (_ , refl , e2 , _) → ta≢tb (sym (just-injective e2)) }
...   | yes refl = map′ (λ p → ta , refl , refl , p)
                        (λ { (_ , refl , refl , p) → p }) (P? ta)

-- Curve-family dispatch (`ec-mul-generator`, `into-coordinates`,
-- `from-coordinates`): decide the per-family demand at each family of the
-- instruction's table.
anyᶠ? : ∀ {P : ECFamily → Set} → (∀ f → Dec (P f)) → ∀ fs → Dec (Anyᶠ P fs)
anyᶠ? P? []           = no λ ()
anyᶠ? P? (f ∷ [])     = P? f
anyᶠ? P? (f ∷ g ∷ fs) = P? f ⊎-dec anyᶠ? P? (g ∷ fs)

OpTy? : ∀ Γ i → Dec (OpTy Γ i)
OpTy? Γ (encode input _)              = opDefd? Γ input
OpTy? Γ (assert cond)                 = opᵗ? Γ cond native
OpTy? Γ (cond-select bit _ _ _)       = opᵗ? Γ bit native
OpTy? Γ (constrain-bits val _)        = opᵗ? Γ val native
OpTy? Γ (constrain-eq a b)            = opp? SuppEq? Γ a ×-dec opDefd? Γ b
OpTy? Γ (constrain-to-boolean val)    = opᵗ? Γ val native
OpTy? Γ (copy val _)                  = opDefd? Γ val
OpTy? Γ (impact guard inputs)         =
  opᵗ? Γ guard native ×-dec allNatᵒ? Γ inputs
OpTy? Γ (ec-mul a scalar _)           =
      (opᵗ? Γ a jubjub-point ×-dec opᵗ? Γ scalar jubjub-scalar)
  ⊎-dec ((opᵗ? Γ a secp256k1-point ×-dec opᵗ? Γ scalar secp256k1-scalar)
  ⊎-dec ((opᵗ? Γ a secp256r1-point ×-dec opᵗ? Γ scalar secp256r1-scalar)
  ⊎-dec (opᵗ? Γ a curve25519-point ×-dec opᵗ? Γ scalar curve25519-scalar)))
OpTy? Γ (ec-mul-generator scalar _)   =
  anyᶠ? (λ f → opᵗ? Γ scalar (scalarTy f)) gen-families
OpTy? Γ (hash-to-curve inputs _)      = allNatᵒ? Γ inputs
OpTy? Γ (into-coordinates point _)    =
  anyᶠ? (λ f → opᵗ? Γ point (pointTy f)) ec-families
OpTy? Γ (from-coordinates (x , y) _)  =
  anyᶠ? (λ f → opᵗ? Γ x (coordTy f) ×-dec opᵗ? Γ y (coordTy f)) ec-families
OpTy? Γ (into-bytes32 input _)        = opp? FM? Γ input
OpTy? Γ (from-bytes32 bytes _ _)      = opᵗ? Γ bytes bytes32
OpTy? Γ (reverse-bytes bytes _)       = opᵗ? Γ bytes bytes32
OpTy? Γ (bytes32-into-low-high b _)   = opᵗ? Γ b bytes32
OpTy? Γ (bytes32-from-low-high (lo , hi) _) =
  opᵗ? Γ lo native ×-dec opᵗ? Γ hi native
OpTy? Γ (div-mod-power-of-two val _ _) = opᵗ? Γ val native
OpTy? Γ (reconstitute-field d m _ _)  = opᵗ? Γ d native ×-dec opᵗ? Γ m native
OpTy? Γ (transient-hash inputs _)     = allNatᵒ? Γ inputs
OpTy? Γ (persistent-hash _ inputs _)  = allNatᵒ? Γ inputs
OpTy? Γ (keccak256 _ inputs _)        = allNatᵒ? Γ inputs
OpTy? Γ (test-eq a b _)               = opDefd? Γ a ×-dec opDefd? Γ b
OpTy? Γ (add a b _)                   = opp2? FN? Γ a b
OpTy? Γ (mul a b _)                   = opp2? FM? Γ a b
OpTy? Γ (neg a _)                     = opp? FN? Γ a
OpTy? Γ (inv a _)                     = opp? FM? Γ a
OpTy? Γ (not a _)                     = opᵗ? Γ a native
OpTy? Γ (less-than a b _ _)           = opᵗ? Γ a native ×-dec opᵗ? Γ b native
OpTy? Γ (jubjub-scalar-from-native a _) = opᵗ? Γ a native
OpTy? Γ (public-input guard _ _)      = guardNat? Γ guard
OpTy? Γ (private-input guard _ _)     = guardNat? Γ guard
OpTy? Γ (circuit-output vals)         = allDefdᵒ? Γ vals

------------------------------------------------------------------------
-- The static well-typedness predicate and its soundness.
--
-- `WT Γ is` threads `outtys` through the instruction list: each
-- instruction is well-typed at the current context (its outputs typed by
-- `outtys`, its operands typed by `OpTy`), yielding the next context.
-- `producer-WT` adds it to `producer-SA` (which already gives freshness
-- and input-name distinctness).  Its soundness is consumed per-step by
-- the backward driver (`bwd-go`) and the build fold, via `step-ty`.
------------------------------------------------------------------------

WT : TyCtx → List Instruction → Set
WT Γ []       = ⊤
WT Γ (i ∷ is) = ∃ λ Γ' → outtys Γ i ≡ just Γ' × OpTy Γ i × WT Γ' is

producer-WT : IrSource → Set
producer-WT S = producer-SA S
              × WT (input-ctx (IrSource.inputs S)) (IrSource.instructions S)

-- Freshness transport: an `SA`-fresh output list (over `bound`) is fresh
-- over `dom-ty Γ` when the two coincide.
all∉-cast : ∀ {Γ bound} ids → dom-ty Γ ≡ bound
  → All (λ o → ¬ (o ∈ bound)) ids → All (λ o → ¬ (o ∈ dom-ty Γ)) ids
all∉-cast ids refl af = af

-- The typed domain of the input context is the SA input-name set.
dom-ty-input-ctx : ∀ tis
  → dom-ty (input-ctx tis) ≡ map TypedIdentifier.name tis
dom-ty-input-ctx []         = refl
dom-ty-input-ctx (ti ∷ tis) = cong (TypedIdentifier.name ti ∷_)
                                (dom-ty-input-ctx tis)

------------------------------------------------------------------------
-- `producer-WT` is decidable: a runnable static well-typedness check.
------------------------------------------------------------------------

WT? : (Γ : TyCtx) (is : List Instruction) → Dec (WT Γ is)
WT? Γ []       = yes tt
WT? Γ (i ∷ is) with outtys Γ i in oty
... | nothing = no λ { (_ , () , _ , _) }
... | just Γ' =
      map′ (λ { (op , wt) → Γ' , refl , op , wt })
           (λ { (_ , refl , op , wt) → op , wt })
           (OpTy? Γ i ×-dec WT? Γ' is)

producer-WT? : (S : IrSource) → Dec (producer-WT S)
producer-WT? S =
      producer-SA? S
  ×-dec WT? (input-ctx (IrSource.inputs S)) (IrSource.instructions S)




