# Intent language — execution summary

**Date:** 2026-09-28
**Status:** specified-only. Nothing in this concept is implemented, no milestone has accepted it, and no evidence here closes a U predicate. Read this document first when returning; it is self-contained.

**Pins.** Moriarty `8f73784042bd692733c296d0d49f5173be96725e` (the state everything was written against). defiformal `8c5dd103cd40369a763b02b1504441acce0ce3c2`, used only as a comparison system.

---

## 1. What was executed, in order

| Step | Output |
| --- | --- |
| Nine-reviewer U0 study (3 lenses × 3 model families) merged into one proposal | [`UNIFIED-PROPOSAL.md`](../../deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md) |
| Nine reviewers on DeFi kernel coverage, 8 categories + the kernel abstraction | [`DEFI-COVERAGE-REPORT.md`](../../deliverables/u0-study-2026-09-28/DEFI-COVERAGE-REPORT.md) |
| The same nine re-scored on the architecture axis — does the roadmap say *how* it will be built | [`ARCHITECTURE-COVERAGE-REPORT.md`](../../deliverables/u0-study-2026-09-28/ARCHITECTURE-COVERAGE-REPORT.md) |
| **Owner decision:** conditional settlement with programmable escrow belongs in the intent language | `wiki/moriarty-architecture.md`, CLM-0978 |
| MIL/1 designed and mapped to every DeFi and asset category | [`DESIGN.md`](DESIGN.md), [`CATEGORY-MAP.md`](CATEGORY-MAP.md) |
| Nine programming-language reviewers examined MIL/1 (types, expressiveness, compilation) | [`review/`](review/), [`review/REVIEW-REPORT.md`](review/REVIEW-REPORT.md) |
| MIL/2 cut against the review | [`DESIGN-MIL2.md`](DESIGN-MIL2.md) |

**The pivotal result.** Nine of nine reviewers endorsed MIL/1's shape and nine of nine refused to freeze it. Three load-bearing claims were false, not incomplete: the escrow meta-rule rejected the design's own examples and forbade the late-success race U3 requires; monotone completion was refuted three separate ways and was not a closed sentence; the anchoring rule — the design's central honesty claim — was erased by every eliminator, with a four-line laundering counterexample. Its surface examples were also not in its own grammar. MIL/2 exists because those repairs interact and could not be patched in place.

## 2. Current support checklist (MIL/2)

**Key:** ✅ in the frozen Φ₀ scope, design-complete · ◐ in the design, resting on an open obligation · ⏳ deferred to Φ₁ or a later milestone · ❌ deliberately excluded · ❓ open

### What an intent can say
- ✅ Outcome-first budgets: gross-debit cap, fee cap, minimum net outcome, named recipients, asset sets
- ✅ Validity window on a domain-qualified clock
- ✅ Signed conditions governing funding, release, refund and partial progress
- ✅ Declared solver degrees of freedom as typed holes
- ✅ Residue — a signed, digested record of what the signer knows the language cannot express
- ◐ Acceptance refinement: a completion can only narrow what the signer authorized

### Types
- ✅ Nominal asset identity with domain, issuer, representation, symbol, decimals
- ✅ `Qty<A>` unsigned u128, `Delta<A>` signed for balance/supply changes, `Position<I>` signed per instrument
- ✅ `Price<B,Q,s>` base-per-quote, scale 0–18
- ✅ Domain-qualified `Instant`, clock-indexed `Duration`, `Window`
- ✅ Obligations with principal/accrued/outstanding, roll-forward, claim class, debtor consent reference
- ✅ Encumbrances against a *set* of obligations with priority
- ✅ Linear receipts
- ⏳ `Share<P,C>` pool claims — typed, but the arithmetic that makes them useful is Φ₁

### Predicates
- ✅ Comparisons with formation rules (same asset, same clock, same scale)
- ✅ Addition, subtraction, min, max
- ✅ Cross-multiplication by literal coefficients, so `spent × 20 ≤ received × 11` needs no solver
- ✅ Bounded atomic negation
- ✅ `pre`/`post` state at stage boundaries
- ✅ Conjunction, disjunction, k-of-n threshold
- ✅ Freshness, finality, attestation, time predicates, delivery, discharge, seizure, consumption, funded/terminal/exercised/challenged
- ⏳ Φ₁: variable×variable products, `mulDiv`, `sharesFor`/`assetsFor` — vault and share math, deferred out of U0
- ◐ Totality: every partial operator rejects rather than lying

### Escrow and conditional settlement
- ✅ Custody by signer, program or k-of-n threshold
- ✅ Release and refund as guarded transitions, with a legitimate pending state
- ✅ Required `priority` field deciding the late-success/refund race
- ✅ `deadline none` — locked collateral discharged only by repayment
- ✅ Per-witness exclusivity: constrained branch bit plus ledger tombstone
- ✅ Authoring check `after(deadline) ⇒ release ∨ refund`, solver-free over Φ₀, failing closed on unknown
- ◐ Recovery viability as a named acceptance obligation under stated liveness assumptions

### Authority
- ✅ Eight rights: initiate, complete, reconcile, recover, disclose, amend, **issue**, **enforce**
- ✅ Linear and affine budgets, scope, window, delegability, revocation
- ✅ Signed total order on competing seizes of one encumbrance
- ❌ `unbounded_within` — deleted, contradicted bounded budgets

### Effects, conservation, evidence
- ✅ Domain on every effect line; a local effect must match the executing domain
- ✅ Per-`(domain, asset)` conservation: Σ balance deltas = declared supply delta
- ✅ Supply changes gated by the `issue` right
- ✅ Evidence as a type index: anchored, imported-with-policy, attested k-of-n
- ◐ Non-laundering by source-set propagation through every operator
- ✅ Imported facts cannot occupy an anchored position; no predicate asserts an anchored foreign fact

### Composition
- ✅ Episode as a ledger-linked sequence: predecessor head, intent digest, cumulative budgets, obligation roll-forward, tombstone
- ✅ Fork partitions the linear context; join fan-in 2
- ◐ Footprints derived from guards and effects, checked against declaration
- ⏳ In-circuit verification of parent proofs — stays U4

### Deliberately not supported
- ❌ Flash loans — restated as a product cut, not a consequence of atomicity; `trace` extension point reserved
- ❌ Concentrated liquidity — needs bounded iteration and a new sort; a future version, not a patch
- ❌ n-party clearing — stage arity *reserved*, but the U0 profile admits one signer
- ❌ In-circuit signature verification — ZKIR v3 has no Ed25519 verifier and no SHA-512; binding is a Poseidon digest plus a ledger-checked signature
- ❌ Global cross-domain rollback, in any form

### Still open
- ❓ Evidence predicates for recipient acceptance, document commitments, challenge records
- ❓ Bounded observation collections, without which TWAP and aggregation cannot be libraries
- ❓ The family→constructor map — MIL/2 adds constructors and makes it more necessary

## 3. Decisions taken, with reversal costs

| # | Decision | Reversal cost |
| --- | --- | --- |
| 1 | **Φ₀ frozen at U0, Φ₁ deferred.** Literal-coefficient cross-multiplication stays in Φ₀, so the U3 discriminator needs no solver. Settled by the repository's own measurement: one 128-bit division obligation cost bitvector `rlimit` 242,607,369 against 1,978 for unbounded `Int`; 256-bit exceeded 600 s. | Moderate — the grammar and caps already reserve Φ₁ |
| 2 | **Flash loans excluded as a product cut**, not an entailment of atomicity. `trace` extension point reserved. | Low now, high once the version header freezes |
| 3 | **Joins ledger-linked through U3.** In-circuit parent verification stays ZR09 in U4. | Low to defer further, high to pull forward |
| 4 | **Multi-signer stage arity reserved now**, U0 profile admits one signer. | **Very high if not reserved** — reviewers were unanimous that the single-signer relation cannot be retrofitted |
| 5 | **Concentrated liquidity out of scope.** | High — it is a second language, not an extension |

## 4. Obligations outstanding before any freeze

None is claimed proved. Each needs a proof or an executed K/TypeScript differential.

1. Φ totality — evaluation into `Value | Reject`, every partial operator naming its rejection
2. Source-set preservation — the non-laundering property, provable in K
3. The encumbrance sum rule — `Σ active locks ≤ balance` per owner and asset
4. Acceptance refinement under completion — `Accepted(I[σ]) ⊆ Accepted(I)`
5. Recovery viability as an acceptance predicate under named liveness assumptions
6. Derived-footprint containment — `declared ⊇ derived`

## 5. Corrections this work owes the U0 contract

Four findings are defects in `UNIFIED-PROPOSAL.md`, not in the intent language. They are recorded in the wiki and **not yet applied to that file**:

1. **E1** states conservation in the zero form; it must be the general law with S0 as its specialization (CLM-0973)
2. **T6** makes in-circuit intent authentication primary; ZKIR v3 has no Ed25519 verifier and no SHA-512
3. **The U3 discriminator's cross-multiplied UInt256 limits** do not fit a 255-bit field against a 256-bit `u128` product; they need the limb rule
4. **SMT stays out of U0** — a compile-time solver does not escape the trusted base unless the circuit re-establishes the property, because a wrong `unsat` admits a permanently stuck escrow

Separately, CLM-0969 corrects the U0 study's claim that declaring a protocol reserve would close all six numeric gaps: it closes one.

## 6. What to do when returning

In order, because each depends on the last:

1. **Take or overturn the five decisions in §3.** Everything downstream assumes them.
2. **Re-derive `CATEGORY-MAP.md` against MIL/2.** It was written against MIL/1 and the review found several "Expressible" rows false — the AMM invariant, vault deposit and redeem, the LTV example, periodic funding, the NFT and position rows. Re-asserting them without redoing the derivation repeats the mistake the review caught.
3. **Discharge or downgrade the six obligations in §4.** Anything still open at freeze time must be tagged, not assumed.
4. **Apply the §5 corrections to the U0 contract**, or record why not.
5. **Run a second review round on MIL/2.** The first paid for itself several times over; MIL/2 has not been reviewed by anyone.
6. **Measure before believing the caps in §12 of the design.** They are proposals, not measurements.

## 7. How the reviews were run

Nine reviewers per round, three per lens, one per model family: Claude Opus 5.5 as agents; GPT-6 Sol via `codex exec -m gpt-6-sol` at xhigh reasoning; Grok 4.7 via `grok -m grok-4.7 --reasoning-effort xhigh`, both read-only sandboxed. The harness is [`review/run_external.sh`](review/run_external.sh) and the briefs are in [`review/briefs/`](review/briefs/); both are reusable for the next round by swapping the lens briefs.

Every reviewer was bound to the non-negotiable constraints — bounded stages, no host-computed Booleans, checked finite-width arithmetic, no global cross-domain rollback, debt is not supply, the federated kernel stays optional — so a reviewer could not "fix" a design by breaking the product contract. Nothing was executed against a compiler, prover or ledger in any round.
