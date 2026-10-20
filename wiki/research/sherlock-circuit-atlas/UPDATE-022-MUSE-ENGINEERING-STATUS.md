---
id: moriarty.research.sherlock.atlas.update022.status
title: UPDATE022 Muse engineering status snapshot
type: overview
status: active
created: 2026-10-20
updated: 2026-10-20
updated_at: 2026-10-20T16:37:22.922562+00:00
tags:
  - moriarty
  - sherlock
  - circuit-concept
sources:
  - src-d23ee2de2634fede6675
---

# UPDATE-022 — internal engineering status

**Date:** 2026-10-20. Internal only, kept apart from the finished visitor copy. The website is UNPUBLISHED and goes live only when the full Sherlock goal is finished and accepted. No site-live, circuit, native, proof, target or EARS qualification is claimed, and this asynchronous log never gates native or product work.

## Merged C4 arithmetic source — bounded approval, tests UNRUN

PR 353 is merged as an exact-12 remote readback. The bounded five-mode ScalarArithmetic emitter, its real consumer and registration carry two fresh non-author bounded reviews approving design, security relation under retained premises, static source API and delivered test source, with no mandatory bounded-source fix; coverage and delivery scope remain changes-required for any stronger claim. This is static source review, not compilation or a test pass. The delivered source contains 10 positive and 12 raw test functions, 22 in total, all UNRUN and uncompiled. Source approval by hash alone is not asserted; the opinions above rest on the source read in full. No degree k, row count, proof size, latency, throughput, speedup, synthesis, MockProver, native proof, stagenet or EARS result is stated or follows.

## What the bounded relation does and does not do

The emitter selects exactly (First, Not) and (Second, Add|Sub|Mul|Lt). Numeric modes discharge the selected register binding in-circuit: operand A must be UInt and the pre register must equal its checked reader payload; Not requires the register at zero. That is a selected-step predicate, not a global prior-state, entry or full invariant theorem. Checked multiplication splits each 128-bit operand into 64-bit limbs, forms four partial products with cross carries c0 and c1, retains the complete high half x1·y1 + c1, and stages error 10 when it is non-zero rather than a wrapped low half; ADD retains its 128-bit carry and SUB its borrow, and LT and NOT stage Booleans or error 11, never integers. Geometry and field names in source ground the concept's shapes only; they are not an executed native circuit layout.

The consumer's LocalPublic and LedgerCall modes bind the same bounded relation; LedgerCall output is the constrained supplied post image, DATA and helper lanes, with the communications commitment recomputed from the current statement and post lanes on every check. The emitter's exported post_phase, post_reg_a and post_staged operands are ungated derived receipts: post_phase is Done even under refused helper work or a disabled statement, and the consumer does not bind circuit post-state or LedgerCall output to them. The host-pair route is an independent-post route: a semantically wrong post that encodes reaches the checker and is not generated, repaired or regenerated from pre by the emitter. The owner-record route validates internal event and snapshot consistency only; the record is publicly constructible, so consistency is not authenticated provenance. The selected-step wrapper calls the owner recorder exactly once and does not roll back: an Err return may follow one already-taken step. Staging is not allocation and there is no HelperExit result or clearing; an applied step is zero allocation but charges one work unit and one helper visit. Under refused helper work the outcome changes to Exhausted HelperWork and the other specified lanes, counters and helper are preserved with no charge; the looser historical every-state-preserved header wording is not repeated as the operative statement.

## Open, plainly

Six coverage classes remain required: coherent phase/mode and register pairing-predicate isolation; direct host producer refusals for prefix, retained config, geometry, length and cap boundaries; evaluation of the disabled exported overflow receipt; systematic genuine all-mode Const/DATA operand-A/B crosses; true refused-work coverage with the retained admission maximum at the 128-bit maximum; and broader individual preserved-section lane mutations or a justified lane-equivalence account. Entry and top-frame coherence, marker placement, the full entry/StepInv/program obligations, P1 native issuance and source authentication, and the full 39-selector interpreter scope all remain open. The 19 public mathematical branch examples are hand-derived illustrations frozen before the producer source; they are not independent complete-state goldens and do not qualify the circuit.

## Separate observations, not arithmetic qualification

Three actual observations are recorded separately and are not merged into the qualification above. An ordinary new Core traversal link/capture run passed for its two EnvGet capture operations only, with no independent comparison performed and no capture semantics included; it is not an arithmetic SDK run or proof. A synthetic twelve-unit process classified 12 of 12 runtime methods as passing, with all six K laws unrun and no genuine runtime bound; it is not a K law evaluation. A Clean rectangular kernel passed its strict typed Nat/Fin address roundtrip and flattening scope with eight manual examples; Rust extraction, column binding, SDK, refinement and native obligations there remain open. None of these is an arithmetic SDK, proof or end-to-end pipeline acceptance, and together they are not one accepted pipeline.