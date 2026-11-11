---
title: "Run decider"
type: entity
tags: [recursion, decider, on-chain]
sources: [c6-outcome-ledger9-review, chip-spike-experiments, e2-e4-experiments, design-2-7-chip-leaf, council-findings, council-revisions, cost-lane-fable, cost-lane-gemini, elastic-chip-gpu-fable]
last_updated: 2026-11-11
---

# Run decider

The terminal proof of a run (`crates/mori-recursion/src/run_decider.rs`). It verifies the last node proof and is the proof Midnight verifies on chain, with a Blake2b transcript ([[c6-outcome-ledger9-review]]).

- 929,166 rows, k = 20, over the chip-leaf node key: "cost model over the chip-leaf node key, unsafe SRS" ([[c6-outcome-ledger9-review]]); also 929,166 over the C6 digest leaf node, unchanged across leaf k, (unlabeled) in [[e2-e4-experiments]] and "Measured model rows" in [[gp-architecture]]. Gate: at most 1,000,000 rows ([[design-2-7-chip-leaf]]).
- The same log line gives 1,032,467 table rows of the 1,048,576 in the domain; whether the tables grow with the circuit was not checked ([[c6-outcome-ledger9-review]]).
- The decider does not depend on the leaf; it consumes only the node key ([[design-2-7-chip-leaf]]).
- Under C6 (SRC-006) it exposes exactly `[prog_d, input_d, outcome_d, n_steps]` and no value that depends on a memory address ([[C6OutcomeOnlyPublication]]).
- Proof size: 6,432 bytes for the k = 20 terminal decider on stagenet; ESTIMATE 6,320 bytes for the chip-key decider ([[c6-outcome-ledger9-review]]).

## Council revisions (branch `pipeline-anim`)

- C4 found two seal paths: the M4 run decider opened the root SEG's header and skipped the `deps_d` check, and the GP-path run entry had no specified head opening ([[council-findings]]).
- P-3, one seal path: the run entry accepts only a SET root sealed by NODE, recomputes `claim_d` and opens the root as `LEAF` or `DEP` with one hash; `open_terminal` is removed; on the M4 path one SEAL node runs first, about 65 s more (ESTIMATE). The run entry refuses `ADM` and `VIEW` roots. DEC-L2 (settle at most once) moves to the LEDGER CONTRACT kit ([[council-revisions]]).

## Cost round (proposals, not merged)

- The decider is a fixed floor of 70 s per run (measured on the stand-in) and about 41.6 native `spend` circuits at the 1,000,000-row gate ([[cost-lane-fable]], [[cost-lane-astra]]). Proposed levers: batching under one decider, a LEAF key kind inside the decider (L3a, L4c), single-collapse accumulation to k = 19 (ESTIMATE) ([[cost-lane-fable]], [[cost-lane-gemini]]). L3a and L4c stand against the one seal path of P-3.
- [[cost-lane-gemini]] calls the 929,166 rows a measured spike; this page records them as a `cost_model` figure.
- A k = 20 decider verified under the stock and installed ledger verifier in a prior run, as recorded; confirmed from sources by [[elastic-chip-gpu-fable]], open in [[cost-lane-fable]] ([[MidnightLedger9]]).

See also [[IvcNode]], [[AcceptRunContract]], [[MidnightLedger9]].
