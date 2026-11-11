---
title: "Review: C6 outcome-only publication and ledger 9 fit"
type: source
tags: [c6, review, ledger9, chip-leaf]
sources: [c6-outcome-ledger9-review]
last_updated: 2026-11-10
date: 2026-11-10
source_file: ~/Projects/Moriarty/research/sherlock-c6-outcome-ledger9-review-20261110/review.md
source_url: local
---

# Review: C6 outcome-only publication and ledger 9 fit

## Summary

Run `sherlock-c6-outcome-ledger9-review-20261110`. Three members (Fable as lead, Astra, Carbon) each answered two questions before seeing another answer. Q1: should C6 publish only the outcome? Verdict **adopt with changes**, 3 of 3. Q2: does the chip leaf design run on Midnight ledger 9 (tag `ledger-9.1.0.1`)? Verdict **yes with changes**, 3 of 3. No build, test, keygen or proof was run in the review; every row count is quoted from the repository.

## Key Claims

Q1, agreements:
- `fin_d` in `stmt` breaks replay refusal under D3. `acceptRun` refuses a replay by `stmt`, `stmt` hashes `fin_d`, and `fin_d` hashes `lanes_d`, a free witness over the raw carrier. With partition-dependent addresses one run has many valid `stmt` values, so the same run can be accepted again. All three members found this independently.
- Resource accounting must be logical. Physical address exhaustion must never become `Exhausted(kind)`.
- The D3 address space is a completeness limit that this decision does not repair: a run has fewer than 2^17 leaves and 2^20 cells, while GP-1 states 2^64 steps (`review.md:40`). See [[RunLengthCeiling]].
- The value digest is a recursive Merkle digest over the value as a tree; sharing is erased; raw `TypeIdC` and `CtorIdC` must not be hashed.

Q1 verdict, conditions of adoption ([[C6OutcomeOnlyPublication]]):
1. Public values `[prog_d, input_d, outcome_d, n_steps]`. Replay key `claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)`. `n_steps` is published beside the claim, not in it. `fin_d` and `init_d` leave the public values.
2. `Rejected(code)` and `Exhausted(kind)` are decided on logical counters only.
3. One value digest, `sherlock-value/1`, for `input_d` and `outcome_d` ([[ValueDigest]]).
4. Sealed claims only across runs.
5. A refinement model whose reference does not allocate per leaf.
6. Open and named: SI-12 under C = 8 (D-1), the 2^17 leaf ceiling against GP-1, the type identity (D-2), the Init mechanism for an input.

Q1 dissent kept: D-1 to D-6 (see [[dissent-c6-section]]).

Value digest cost, all ESTIMATE in the source, on 70 rows per Poseidon permutation:
- UInt, Bool, Leaf: 210 rows. Nil: 140 rows. Cons, Branch: 210 rows.
- Worst leaf, 8 data appends: about +1,680 rows, 6,511 to about 8,191, so k = 14 if the usable rows of k = 13 are exceeded.
- Average: at most +11% leaf rows on the census average.
- Node (823,466 rows): unchanged if no column is added. Decider (929,166 rows, k = 20): under 1,000 rows of change.

Q2 evidence and verdict:
- The decider size 929,166 rows at k = 20 is labelled "cost model over the chip-leaf node key, unsafe SRS". The same log line gives 1,032,467 table rows of the 1,048,576 in the domain.
- The ledger and node tags resolve `midnight-proofs` 0.8.2; the workspace proves with 0.8.3. A 0.8.3 proof against a 0.8.2 verifier is not yet qualified.
- The ledger has no maximum-k check. The data catalogue lists ceremony files through `bls_midnight_2p25`.
- Proof size 6,432 bytes for the k = 20 terminal decider on stagenet; ESTIMATE 6,320 bytes for the chip-key decider.
- No blocker in ledger 9. Open work: new contract, template and keys; no `acceptRun` transaction validated by a 9.1.0.1 node; the chip leaf is a spike with no proof; qualify 0.8.3 against 0.8.2; read the target `LedgerParameters`; one contract for all programs needs `prog_d` and `input_d` as call data; transaction construction items.

## Key Quotes

- Owner decision quoted in the design text: "go with publishing only the outcome for C6 after gemini carbon and fable and astra review that".
- "The prover can only fail to prove a long run (address ceiling); it cannot change the outcome."
- "'Yes' is not yet 'accepted on chain': items 1 to 5 above are open."

## Connections

- [[C6OutcomeOnlyPublication]], [[ValueDigest]], [[RunLengthCeiling]], [[ChipLeaf]].
- [[AcceptRunContract]], [[RunDecider]], [[MidnightLedger9]].
- The design text it proposed is now in [[design-2-7-chip-leaf]]; its dissent is recorded in [[dissent-c6-section]].

## Contradictions

- Agreement 7 lead note: the GP architecture study's public `shape_d` binds segment bracketing ([[gp-architecture]]). The review asks that `shape_d` take heights and claim content, not state digests. Open as D-6.
- D-3: [[gp-architecture]] puts `n_steps` inside `out_d` (`architecture.md:98`); the verdict takes it out of the claim.
- Exhaust kind 7 (`LogicalCap`) in the `Exhausted(kind)` encoding conflicts with SVM-009 ([[dissent-c6-section]]).
- Carbon's SRS ceiling at k = 20 is recorded as wrong: the catalogue lists files through `2p25`.
