---
title: "Recursion structure and circuit size of the Sherlock chain"
project: loki
component: recursion-structure
date: 2026-10-24
sources:
  - /home/charles/repos/projectfenrir-main/recursion/stagenet-strawman/recl9/src/sherlock_link.rs
  - /home/charles/repos/projectfenrir-main/recursion/stagenet-strawman/recl9/REPORT.md
  - /home/charles/repos/projectfenrir-main/recursion/stagenet-strawman/recl9/DECIDER-CONFIG.json
  - /home/charles/repos/projectfenrir-main/recursion/stagenet-strawman/recl9/src/node.rs
  - /home/charles/repos/projectfenrir-main/recursion/pairing-gadget/lanes/opus/RESULTS.md
  - /home/charles/repos/Moriarty-sherlock/crates/mori-recursion/src/{parent,decider,pairing,inner,reference}.rs
  - /home/charles/repos/Moriarty-sherlock/crates/mori-backend/src/proving.rs (MAX_K = 19)
  - /home/charles/repos/Moriarty-sherlock/wiki-llm/loki/10x/astra-2/PROPOSAL.md
  - /home/charles/repos/Moriarty-factory/wiki-llm/loki/10x/LOKI-10X.md
  - https://eprint.iacr.org/2023/1192 (CycleFold)
  - https://eprint.iacr.org/2024/640 (Novakovic-Eagen, residue witness)
  - https://github.com/midnightntwrk/midnight-zk/pull/565 (fflonk)
  - https://eprint.iacr.org/2023/573 (HyperNova), https://eprint.iacr.org/2023/1106 (ProtoGalaxy), https://eprint.iacr.org/2023/1192 (CycleFold)
  - https://zkproof.org/2023/02/21/sangria-a-folding-scheme-for-plonk/ (Sangria)
  - https://eprint.iacr.org/2022/1662 (cycles of pairing-friendly curves)
---

# Recursion structure and circuit size of the Sherlock chain

## 0. Corrections to the brief (read first)

1. **Children are k13 (A102) and k12 (A80), not k15 and k12.** The child report
   `docs/results/sherlock-native-a102-a80-child-producer-20261021/actual/RUN-REPORT.json`
   records A102 k13, A80 k12, 3,440-byte proofs, 253 instance lanes (cited in
   `astra-2/PROPOSAL.md:51`). "k15" is the proposed *fused terminal* target, not a child.
   The child exact row counts are **unrecorded**. The only figures are domain capacities,
   8,185 and 4,089 usable rows (2^13 - 7 and 2^12 - 7), which bound the children but do not measure them.
2. **"Leaves 163.6 s" is a serial sum.** The four 4-tx leaves prove in 34.8, 35.3, 44.2 and
   49.4 s (`REPORT.md` section 3), which sum to 163.7 s. Wall time with four parallel leaves is
   roughly the slowest leaf, about 50 s, if cores allow. The 1,617.3 s "total" in LOKI-10X is
   also a sum (163.6 + 867.3 + 586.4).
3. **Sherlock parent timing is unreconciled.** The common brief says 220 s on 4 threads at
   16 GB. LOKI-10X and `loki-05` cite 882.590 s proving plus 62.328 s PK setup for k20 (6,432 B
   proof). These may be different runs or different thread counts. Do not mix them in a
   ratio until one receipt is identified.
4. **The residue gadget (316,833 rows) is a standalone measurement.** It is the KZG-decider
   pairing alone, at k19, `nr_pow2range_cols = 4`, with A and B subgroup-checked. It is not a
   per-child cost. It is measured in `pairing-gadget/lanes/opus/RESULTS.md:17`.
5. **`LINK_MAX_K = 19` (`sherlock_link.rs:656`) bounds child `circuit_k`, not the parent.** The
   parent is k20 by `cost_model`, and `MAX_K = 19` in `mori-backend/src/proving.rs:22` is the
   Moriarty producer's child cap.

## 1. The chain as it is built

**Sherlock link parent** (`mori-recursion/src/parent.rs`, mirrored in
`recl9/src/sherlock_link.rs:445-525`):

- Verifies the A102 child (k13) and the A80 child (k12) in circuit, each under a fixed VK
  (`assign_fixed_vk`), with `verifier.prepare` and one `accumulate` over both.
- Resolves the merged fixed bases once (`resolve_fixed_bases`), collapses once
  (`collapse`), and performs no pairing (deferred).
- Commits program, pre, mid, post and trace with pinned-domain Poseidon, then exposes
  16 statement fields plus the 12-field collapsed accumulator (28 public fields).
- Pins the envelopes (3,1) and (0,0), the contexts, and `work = 1` by equality.
- Parent `k = 20` by `cost_model` (`parent.rs:899`). Measured: 882.6 s prove, 62.3 s PK setup,
  6,432 B proof (`loki-05` line 129; the setup figure is recorded in LOKI-10X).

**Decider** (`mori-recursion/src/decider.rs`, `recl9/src/decider.rs`):

- Verifies W (the parent) in circuit, folds the carried accumulator, collapses, re-assigns A
  and B with the subgroup-checked assign, constrains both unit scalars, runs the residue
  pairing `e(A,[tau]2) e(B,-[1]2) = 1`, computes `stmt = H(DOM_STMT, ...)`, and lays out a
  15-slot `acceptValidity` template.
- Measured, recl9 (`REPORT.md` 3): `k = 20`, rows 841,682, table 1,032,209, prove 586.4 s,
  pk 46.9 s, vk setup 412 s, proof 6,432 B, peak RSS 14.97 GB.
- The ledger verifies the decider, not the parent.

**Fenrir recl9 16-tx chain** (`recl9/REPORT.md` 1, `DECIDER-CONFIG.json`):

- Leaves: four verifier-free 4-tx proofs, `k = 17`, 112,098 rows, 4,208 B each.
- Node N: one arity-4 node verifying all four leaves in circuit; `k = 20`; advice 958,095;
  lookup table 1,032,465; prove 867.3 s; proof 6,432 B; peak RSS 14.99 GB.
- Decider D: as above.

Structurally the Sherlock chain is "two small children, one k20 parent, one k20 decider". The
recl9 chain is "four leaves, one k20 node, one k20 decider". Both end in the same
decider template, so the decider is shared between the two lanes.

## 2. Where the circuit size comes from

### 2.1 In-circuit KZG and pairing deferral

- **Native field.** Midnight's native field is the BLS12-381 scalar field. The G1 group law and
  the Fq base-field arithmetic of a verifier are therefore emulated (non-native). This is the
  dominant cost of any in-circuit verifier, not the pairing itself.
- **Rows per emulated op at rc = 4** (`pairing-gadget/lanes/opus/RESULTS.md`, "Unit costs",
  measured at k17): Fp mul 15 rows (of which about 13 are range-check decomposition); Fp12 mul
  2,802; Fp12 squaring 1,812; Fp12 x line 1,152; line evaluation (fixed G2) 90.
- **Decider pairing breakdown** (RESULTS.md, "Breakdown", rc = 4): Miller loop about 283k rows
  (94%); residue check (c^-1 folds plus finish) about 18.8k rows (6%). A final exponentiation
  would cost "well over 100k rows" by the Opus estimate, so the residue method saves that.
- **Range checks dominate.** The Opus lane estimates about 85% of decider rows are range-check
  rows at rc = 4. rc is capped at 4 by the stdlib. The stock ledger rebuilds the constraint
  system from the arch, so changing rc is a new key (Track B).
- **Subgroup checks** cost 8.6k rows per G1 point (A and B), i.e. about 17k rows total
  (316,833 default vs 299,573 without them).
- **Accumulation** (`midnight-proofs` 0.8.x / stdlib `Accumulator`): each child contributes a
  dual MSM (`from_dual_msm`), accumulators are folded with `Accumulator::accumulate`, and the
  fixed bases are resolved once. The in-circuit cost of one child's verification is therefore
  the foreign-field MSM plus the transcript, not a pairing. The pairing is deferred to the
  decider, which is why the decider carries the residue gadget.
- **Historical split of the k20 decider** (astra-2 `PROPOSAL.md:90`): a node2 plus its
  pairing is about 891k rows (k20); a node4 plus pairing is about 1.275M (k21), before glue.

### 2.2 Why the children cost what they cost

- The parent's advice rows per arity are close to linear. Measured arity sweep
  (`REPORT.md`, "Node arity sizing"): arity 2 = 574,040; arity 3 = 765,907; arity 4 = 957,724
  (interim crate), and 958,095 on the certified crate. The fit `R(a) = 190,356 + 191,842 a`
  matches all three to within 25 rows (arity 3 is 765,882 by the fit; measured 765,907).
- Each child therefore costs about 192k advice rows in an arity-N node, regardless of child k.
  A k12 or k13 child is not cheap inside a k20 parent.

## 3. Ledger-side facts that constrain the design

- The decider template has 15 slots plus the 17-field statement (`acceptValidity`).
- Changing the decider or parent circuit changes the VK and the ledger verifier input. Track
  B therefore also means a contract or template change and a new deployment, with the
  fee and anchor migration that astra-2 costs at unmeasured cost.
- `DECIDER-CONFIG.json` pins `k = 20`, the decider VK sha256 and the W VK sha256. Any k change
  moves both.

## 4. Options, marked Track A (keeps VK, SRS, transcript, proof format) or Track B (new keys)

Each row is either a measured number from a source above or labelled ESTIMATE.

### 4.1 Reducing k from 20 to 19 (or lower)

The binding constraint is the **lookup table, not advice rows**, in both node and decider:

| Object | Advice rows | Table rows | usable at k19 | usable at k20 | Blocker at k19 |
|---|---|---|---|---|---|
| Node N (arity 4) | 958,095 | 1,032,465 | 524,281 | 1,048,569 | advice (-45% needed) and table (-49% needed) |
| Node N (arity 2) | 574,040 | 1,032,465 | 524,281 | 1,048,569 | advice (-9%) and table (-49%) |
| Node N (arity 1, est.) | about 382,198 | 1,032,465 | 524,281 | 1,048,569 | table only (-49%) |
| Decider D | 841,682 | 1,032,209 | 524,281 | 1,048,569 | table (-49%); rows also need -38% if residue is kept |

Notes:
- The table is fixed by the arch and does not shrink with arity. REPORT.md states that "all
  arities are bound to k = 20 by the fixed lookup table". The Opus note says the range-check
  table width is "k-1 bits", so the table may scale with k. The sources do not give a
  per-table breakdown. **A k19 claim needs one before anything else.**
- Removing the residue gadget alone gives D about 524,849 rows, which is 568 rows above the
  k19 usable capacity. That is close, but the table still blocks k19.
- Gain if k drops 20 to 19: the domain halves, so FFT and MSM work falls by about 2x to 4.4x
  (astra-2 uses 4.44x for FFT). The gain is ESTIMATE only. It is not realised unless advice and
  table both fit.

Track B for all of 4.1 (a new circuit and new keys).

### 4.2 Fused k15 terminal (the 10x verdict's top route)

- What it is: prove the A102 to A80 transition as one relation with no child proof verification,
  so the in-circuit pairing disappears. The ledger still checks the outer KZG pairing natively.
- Evidence: the children fit in domains of 8,192 and 4,096 rows, so their fragments are at most
  12,274 rows (`astra-2/PROPOSAL.md:65`). Budget: 8,192 extra rows for composition, giving 20,466
  of 32,761 usable k15 rows. **This is a design budget, not measured.** The 8,192 figure is an
  allocation, not a result.
- Evidence for "32x fewer rows than k20": 2^5 domain ratio (LOKI-10X). The row ratio is larger
  (841,682 / 20,466 is about 41x) but only if the fused terminal really has no residual
  verifier. That is unverified.
- Gain: LOKI-10X claims 10x on chain latency (medium confidence). Accelerating the parent alone
  caps the chain at 2.16x, because the leaves and decider dominate. Gain is ESTIMATE.
- Effort: 2 to 4 engineer-weeks plus a deployment (LOKI-10X, astra-2).
- Correctness risk: must reproduce all 16 statement fields and the Poseidon domain order
  (`sherlock_link.rs:519`). Equivalence to the Lean SPEC is the gate (`loki-lean/SPEC.md`).
  The children's own open obligations (`A102_OPEN`, `A80_OPEN`, `sherlock_link.rs:658-675`) are
  inherited, not closed, by a fused terminal.

Track B.

### 4.3 Defer the final pairing to the ledger (hypothesis, not verified)

- The residue gadget (316,833 rows) exists because D checks N's accumulator in circuit. If D
  instead exposed its folded accumulator as public inputs, the stock ledger verifier could check
  the pairing natively, as the leaf verification in recl9 already does ("stdlib and accumulator
  pairing both pass", `REPORT.md` 3).
- Gain if it works: about 38% of D's rows, and it removes the "k19 floor" that LOKI-10X names.
  It still needs a table cut to reach k19 (see 4.1).
- Blocker: the 15-slot `acceptValidity` template and the ledger contract shape would change.
  Whether ledger-9 `VerifierKey::verify` runs a pairing on an exposed accumulator is **not
  confirmed from the sources read**. It is the first thing to check.
- Effort: medium. Risk: a changed trust statement at the ledger, which needs review.

Track B.

### 4.4 Tree shape and fan-in

- Arity 4 with one node over four leaves is already the minimum-depth choice (depth 1).
- Arity 2 saves nothing at k20 (574k advice rows, same k, same table) and adds verifier instances
  (astra-2 `:88`).
- Fan-in 8 would need k21 (R(8) = 1,725,092 advice rows, above 1,048,569). Not a speed-up.
- Arity 1 (one child per node) fits advice at k19 (about 382k) but the table still blocks it, and
  every extra level adds a verifier.
- Gain from tree shape alone: none at k20. It matters only after 4.1 or 4.2.

Track B, low priority.

### 4.5 Key and PK caching

- Recorded: PK setup 62.3 s out of 944.9 s total (882.6 + 62.3), giving 1.07x
  (`LOKI-10X.md:28`, `astra-2:128`). The correction to LOKI-DESIGN L1 applies. That is not the
  "120 s per proof" the design once assumed.
- Track A (keys, proofs and format unchanged). Effort low. Already in the LOKI-DESIGN L1 list.

### 4.6 Folding: CycleFold, Sangria, ProtoGalaxy, HyperNova

- **CycleFold** (eprint 2023/1192): moves the verifier's EC scalar multiplications to a second
  curve. It reports about 1,000 to 1,500 multiplication gates for the secondary circuit,
  versus about 10,000 for a Nova-style setup. It needs a cycle, or at least a second curve
  whose base field is the first curve's scalar field.
- **Sangria** (zkSecurity/Geometry blog, 2023-02-21): a folding scheme for a PLONK variant with
  custom gates of degree 2. Sources describe it as a PLONK-based folding, not a halo2 port.
- **ProtoGalaxy** (eprint 2023/1106) and **HyperNova** (eprint 2023/573): both fold PLONKish-style
  instances. Both still need the verifier's EC operations, so both need a cycle or CycleFold.
- **BLS12-381 has no known 2-cycle.** This is a secondary-source claim (an Ethereum Magicians
  post and eprint 2022/1662, which says only that known cycles are sparse). Treat it as
  unconfirmed until checked against the cycle literature.
- **Verdict:** this is a new proof system and a new curve stack, so it is Track B and very large
  effort. astra-2 finds that folding loses at N = 16. It does not meet the 10x target. Keep out
  of the speed-up plan.

### 4.7 Aggregation schemes (snark-verifier, Nebra UniPlonK)

- snark-verifier (PSE/Axiom) aggregates Halo2 KZG proofs on BN254 for Ethereum. Its
  aggregation row counts were not found in this search. Its curve is not Midnight's.
- Nebra's UniPlonK (eprint 2023/869 search hit) gives a fixed-cost universal verifier, with
  marginal cost that scales with log of the maximum circuit size. It is a different protocol
  from Midnight's, so it needs new keys and a new verifier (Track B). No row numbers were found.
- Verdict: no measured number here transfers to Midnight's BLS12-381 stack. Track B and low
  priority.

### 4.8 fflonk t=4 (midnight-zk PR #565 and #575)

- PR #565 "fflonk optimizations" (merged 2026-10-14) on zswap k14: KZG prove 415 ms, fflonk t=4
  536 ms (1.29x slower); KZG verify 2.68 ms, fflonk t=4 2.28 ms. VK 578 B vs 2,258 B; proof
  3,840 B vs 4,384 B.
- So it shrinks the VK and proof and makes proving slower. It does not reduce k. It is not a
  speed-up for the parent, and it needs new keys. Its benefit is the VK and proof size.
- Track B. Low priority for speed.

## 5. Ranked table (top 5 by expected gain on the k20 parent divided by effort)

| Rank | Technique | Track | Gain (source) | Effort | Correctness risk | Pays off only if |
|---|---|---|---|---|---|---|
| 1 | Key and PK caching | A | 1.07x (measured setup share) | low | low | the parent is proved repeatedly with same keys |
| 2 | Fused k15 terminal | B | 10x on chain latency (LOKI-10X, ESTIMATE, medium confidence) | 2 to 4 eng-weeks plus deployment | high (replaces the proof and statement) | equivalence to SPEC holds, the 20,466-row budget survives measurement, ledger accepts the new VK |
| 3 | Table cut to reach k19 for N and D | B | 2x to 4.4x domain work (ESTIMATE) | medium to high, unknown | medium | a per-table breakdown shows the table can fall below 524,281 rows |
| 4 | Defer final pairing to native ledger (drop residue gadget) | B | about 38% of D's rows (measured gadget size, ESTIMATE of gain) | medium | high (ledger trust statement changes) | the ledger-9 verifier runs a pairing on an exposed accumulator (not yet confirmed) |
| 5 | fflonk t=4 | B | smaller VK and proof; prover 1.29x slower (measured on zswap k14) | medium | low to medium | the VK or proof size is the binding limit |

Not ranked (no gain at k20 or out of scope): tree shape and fan-in (4.4), folding (4.6),
UniPlonK and snark-verifier aggregation (4.7).

## 6. What to measure first

1. A per-table breakdown of the node's 1,032,465 and the decider's 1,032,209 lookup-table rows,
   and whether that table scales with k. This decides whether k19 is reachable at all.
2. Measured rows of the A102 and A80 children (the RUN-REPORT records only k and proof size).
3. The actual fused-terminal row count, not the 20,466 budget, at k15.
4. A reconciliation of the Sherlock parent timing (220 s vs 882.6 s) from the receipts.
5. The ledger-9 `VerifierKey::verify` source, to settle whether an exposed accumulator is
   pairing-checked natively (section 4.3).

All numbers above are from the cited files or labelled ESTIMATE. No code was edited and nothing
was committed.
