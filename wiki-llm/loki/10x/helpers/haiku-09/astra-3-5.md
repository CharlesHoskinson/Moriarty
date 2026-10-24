# astra-3-5: fused 16-tx relation, k19 fit, domain budget, and B3/B7 memory compliance

## Question
(1) Can a stock-ledger-9-compatible, verifier-free 16-transaction leaf replace the 4-leaf → node → decider chain? Does the leaf max-4 restriction bind, and do 16 native transitions fit k19 or do tables force k20? (2) Is it semantically the Sherlock full machine? (3) Domain-work and witness-row budget: 4·k17 + 2·k20 versus 1·k19, including retained recursion. (4) Does the factory's B3/B7 memory cap (decimal 16 GB) comply with a 90 GB host scheduling proposal? No benchmarks; no guaranteed row extrapolation.

## Answer

- **CODE — The max-4 restriction is in three places, not just one.**
  - `recl9/src/leaf.rs:42-43` `validate()`: `n_tx` must be 1..=4.
  - `recl9/src/compose.rs:53` `compose_assigned` rejects any arity outside 2..=4, and `compose.rs:61-62` picks the arity-4 domain `dom::in4()`.
  - `recl9/src/node.rs:68-69` node arity 2..=4.
  - `recl9/src/batch.rs:34-37` chunks slots by `n_tx`.
  - Lifting to 16 is therefore a code change (a depth-2 composition or a new fixed-width combine with new domain constants), not a parameter change. The 16-field output is a fixed-width aggregate, and whether it generalizes without changing its statement semantics is **unverified (OPEN)**.
- **CODE — The leaf is verifier-free and Poseidon-transcript.** `leaf.rs:1-9` (no in-circuit verifier, no BLS point). Its circuit (`leaf.rs:77-96`) runs `slot_rules` per slot, then `compose_assigned` for n>1 (`leaf.rs:93`). The leaf transcript is Poseidon (`recl9/out/leaves-b1/leaves.stats.json:56`). The decider's transcript is Blake2b (`recl9/out/decider-b1/batch-1.stats.json:37`).
- **CODE — The leaf holds every spender's secret key.** `leaf.rs:7-9` says this is acceptable only for the synthetic strawman, because production leaves verify user wallet proofs. A 16-tx fused leaf inherits this, so it is not a production wallet-throughput number.
- **SOURCE — Ledger acceptance.** The ledger accepts the decider's `acceptValidity` template proof (REPORT §5). `midnight-zk-stdlib-2.3.5/src/lib.rs:2017` `verify<R, H: TranscriptHash>` is generic over the transcript, so the transcript is a proof-side choice in stdlib. Whether the ledger-9 rc.6 key reader accepts a Poseidon-transcript leaf VK is **not verified**: the ledger verify tool source is not in the readable tree (`decider/tools` absent in this checkout).
- **ESTIMATE — Fit: 16 transactions fit k19 (not k20), with the table not binding.**
  - Cost rows: 1-tx 27,980 and 4-tx 112,098 (REPORT §3). Linear fit: 28,039.3 rows per tx, intercept −59. Predicted 16-tx = 448,570 rows.
  - Usable rows at k19 = 2^19 − 7 = 524,281 (same blinding offset as the measured k20 value 1,048,569 = 2^20 − 7). Predicted load 85.6% of k19. k18 usable = 262,137, too small.
  - Table rows: 1-tx 26,883, 4-tx 98,563. Linear: 23,893.3 per tx, intercept 2,990. Predicted 16-tx = 385,283, below the k19 usable limit. The table is **not** the binding constraint for the leaf (in contrast to the node, where the 1,032,465-row fixed table forces k20 for all arities).
  - Caveat: 14% headroom on a linear extrapolation. A k20 leaf is needed only if the true count exceeds 524,281 (about 17% above the prediction).
- **ESTIMATE — Semantics vs Sherlock.** The leaf proves the strawman I1 wallet rules and transition rules (REPORT §1, leaf.rs module doc). The Sherlock linked parent (`recl9/src/sherlock_link.rs:1-20`) proves a Moriarty Machine run: A102 line 102 and A80 line 80 children, with program, pre/mid/post roots and trace commitments. These are different programs. A fused 16-tx leaf is **not** semantically equivalent to the Sherlock full machine. It is equivalent only to the synthetic Fenrir-derived batch rules.
- **ESTIMATE — Domain-work and witness-row budget.** Measured/estimated numbers:
  - **Current chain** (4·k17 leaves, 1·k20 node, 1·k20 decider): domain 4·2^17 + 2·2^20 = **2,621,440**. Witness rows 4·112,098 + 958,095 + 841,682 = **2,248,169**.
  - **Fused 16-tx leaf alone** (k19): domain 2^19 = **524,288** (5.0× less domain). Rows 448,570 (ESTIMATE). **But** it is not ledger-acceptable by itself (see SOURCE above), so it needs new keys and a new acceptance path (Track B).
  - **Fused leaf + wrapper + decider**, which keeps the existing ledger form: the wrapper must verify the fused leaf in circuit. A node of arity 1 is not allowed (`node.rs:68`). Arity 2 is the minimum, k20 (table-bound). Domain: 2^19 + 2^20 + 2^20 = **2,621,440**, the same as the current chain. Rows ≈ 448,570 + 382,198 (arity-1 estimate: 574,040 − 191,842) + 841,682 = **1,672,450** (ESTIMATE). The domain saving is zero; only rows drop by ~26%.
  - **Retained recursion, generic chunk fusion:** the node is table-bound at k20 for every arity 2–4 (REPORT §3 arity table), so each recursion level costs a full k20 node (measured prove 867.3 s). Fan-in 4 is already the cheapest node count (1 node per 16 tx). Fusing into 16-tx leaves removes the node level but not the decider level.
- **ESTIMATE — Decider stays in the stock path.** The decider verifies a W with a BLS12-381 accumulator. The fused leaf (Poseidon, verifier-free) has no accumulator, so the decider cannot consume it. Either a new decider relation or a new wrapper is needed, which means new keys. Track B.
- **CODE — B3/B7 compliance requirement is distinct from host RAM.**
  - `qa/sherlock/predicates.yaml:115-121` (B3: one 16-tx full batch proved and verified locally, heavy, budget `b3-full-batch-local`, depends on B2 and A5).
  - `qa/sherlock/predicates.yaml:154-160` (B7: "Time, RSS and bytes in separate scopes, peak under decimal 16 GB with swap 0", T2, depends on B3).
  - `qa/sherlock/budgets.json:56` `b3-full-batch-local`: `maxRssBytes: null`, `measured` all null. No cap is declared in the budget file. The 16 GB cap is only in the predicate title.
  - Both predicates run `scripts/qa/unimplemented.sh`, which exits 2 "UNIMPLEMENTED". **B3 and B7 are not implemented.**
  - Decimal 16 GB = 16,000,000,000 bytes (14.90 GiB).
  - Current measured per-process peaks (`/usr/bin/time` kB, KiB units): node 14,988,176 kB = **15,347,892,224 B** (15.35 GB, 0.65 GB or 4.1% under the cap); decider 14,965,544 kB = **15,324,717,056 B** (15.32 GB); leaves 1,451,204 kB = 1.49 GB. Each passes, but only one at a time.
  - Two heavy proofs in flight (loki-08 scheduling proposal): 30,672,609,280 B ≈ **30.7 GB, 1.92× the cap**. It fails B7 if the measured unit is the scheduler (which is the natural reading of "peak" for a single run).
  - The 90 GB host (`nproc` 20, `free -g` total 90, available 74) does not change B7. The cap is a fixed per-run peak with swap 0, independent of host RAM. A 64 GB host would not help compliance either; the 16 GB cap is the constraint.
  - Swap 0 is **not recorded** in the recl9 logs I read; B7 cannot pass on those logs alone.
  - Sherlock parent receipt (`docs/results/sherlock-native-observations-20261022/receipts/parent-proof.json`): `memory_peak_bytes` 15,678,128,128 (15.68 GB) under the cap, but it is a mixed service counter, not isolated RSS. Not B7 evidence.
  - Compliance conclusion: **B7 cannot be claimed for any schedule that holds two heavy proofs resident**, and nothing in the current logs establishes swap 0. A single-heavy schedule passes RSS arithmetic (node 15.35 GB < 16.00 GB) but still needs its own measured B7 run.

## Raw numbers

| Item | Value | Source |
|---|---|---|
| Leaf cost rows 1-tx / 4-tx | 27,980 / 112,098 | `recl9/REPORT.md` §3 |
| Leaf table rows 1-tx / 4-tx | 26,883 / 98,563 | REPORT §3 |
| Predicted 16-tx cost rows (linear) | 448,570 | ESTIMATE, this file |
| Predicted 16-tx table rows (linear) | 385,283 | ESTIMATE |
| Usable rows k19 / k18 | 524,281 / 262,137 | 2^k − 7 (inferred from `node-keygen.log` usable 1,048,569 at k20) |
| Node table rows (any arity 2–4) | 1,032,465 (k20 fixed) | REPORT §3 |
| Node advice rows arity 2 / 3 / 4 | 574,040 / 765,907 / 957,724 (interim); 958,095 certified | REPORT §3 |
| Per-child increment | 191,842 rows | derived |
| Current domain / rows | 2,621,440 / 2,248,169 | derived |
| Fused leaf-only domain / rows | 524,288 / 448,570 | derived, ESTIMATE |
| Fused + wrapper + decider domain / rows | 2,621,440 / 1,672,450 | derived, ESTIMATE |
| Node peak RSS | 14,988,176 kB = 15,347,892,224 B | `recl9/logs/node-prove.log` |
| Decider peak RSS | 14,965,544 kB = 15,324,717,056 B | `recl9/logs/decider-prove.log` |
| Two heavy in flight | 30,672,609,280 B | derived |
| B7 cap | 16,000,000,000 B (decimal) | `qa/sherlock/predicates.yaml:156` |
| Sherlock receipt memory counter | 15,678,128,128 B (mixed) | `docs/results/sherlock-native-observations-20261022/receipts/parent-proof.json` |

## Caveats
- The 16-tx row and table counts are linear extrapolations from two points (1-tx and 4-tx). No 16-slot circuit was built. Nonlinearity in composition could move the count by more than the 14% k19 headroom.
- The fused leaf's k19 prove time is not estimated here. Domain-work is a proxy for prover work, not measured time.
- Ledger-9 key-reader transcript compatibility is OPEN; the ledger tool source is not available in this checkout.
- The "64 GB" figure from the question is not in any source I read. The measured host is 90 GB.
- `mono/` 16-tx single-leaf fallback (REPORT §1) has no stats file in the repo; not used.
- Nothing was built, run, or modified in any repository.

## CORRECTION (retracts the fit claim)
- **Retracted: "16-tx fits k19".** Table row counts are not linear in transaction count because the range-tag union changes with n_tx, and the same caveat applies to cost rows. The linear predictions (448,570 cost rows; 385,283 table rows) and the 85.6% k19 load are unsupported sensitivity numbers only. The k19 fit is **unproven**; the k20 outcome is equally open.
- **Still valid (CODE):** the leaf max-4 restriction at `leaf.rs:42-43`, `compose.rs:53` (arity 2..=4), `node.rs:68-69`; lifting to 16 is a code change.
- **Still valid (SOURCE/CODE):** the decider transcript is Blake2b and the leaf is Poseidon, and ledger acceptance of a new VK is unverified. The B7 numbers are measured per-process peaks; the two-in-flight failure is arithmetic on those peaks.
- **Domain accounting** (2^19+2^20+2^20 = 2^21 for a fused leaf with wrapper and decider) depends on the assumed k19 and k20 sizes, so treat it as conditional on the unproven fit.

## CORRECTION 3 (topology wording)
- The wrapper-plus-decider accounting (2^21 domain) is one topology, not the only one. A direct fused terminal that binds the ledger statement inside the leaf, with no recursive wrapper and no decider, is also a candidate when the same party holds all witnesses. That is a new-key change, and it must still pass the stock ledger key-reader and architecture limits (unverified).
- Domain ratio for that direct path: current 4·2^17 + 2·2^20 = 2.5·2^20 = 2,621,440 versus 2^19 = 524,288, a 5× domain reduction. This is unweighted domain work. Column weighting and overhead are separate and unmeasured.
- The k19 fit remains unproven (see the CORRECTION above).

## CORRECTION 4 (a measured 16-slot cost exists; it is not a proof)
- `projectfenrir/recursion/stagenet-strawman/mono/logs/cost.log:10` records a 16-slot mono leaf cost model: `n_slots` 16, `k` 19, `rows` 449,169, `table_rows` 360,707, `advice_columns` 9, `fixed_columns` 32 (including 10 permutations, per cost_model.rs:244), `lookups` 4, `permutations` 10, `usable_rows_at_k` 524,281, `seconds` 88.816 (cost-model time, not a prover time). Source: `mono/logs/cost.log:10`.
- This is the first measured row count at 16 slots. It supports the k19 fit for cost rows (449,169 of 524,281 = 85.7%). My linear prediction was 448,570 rows and 385,283 table rows; the advice count matches within 0.13%, but the table prediction (385,283) is 6.8% below the measured 360,707. Table counts are not linear, as the range-tag correction said. So the fit is now measured for cost rows only, and the table fit is unproven at 16 slots but at 360,707 is below the k19 limit.
- No k19 **proof** was found (astra-2): `mono/logs/mock-batch.log` is MockProver only. `mono/logs/cost.log` is the only 16-slot record I read. My earlier "no 16-slot measurement exists" applied to proofs and is retained for them.
- The mono 16-slot relation is not the Sherlock terminal (mono/src/mono.rs:273 leaves CommunicationCommitment free; astra-2), so it is not a qualified terminal.
