# astra-3-2: actual recursion DAG and throughput arithmetic

## Question
What does the current Sherlock chain verify, versus the hypothetical 16-transaction workload (4 leaves → 1 node → decider)? What are the measured stage times, row counts, k and key times (with provenance)? What is the DAG critical path versus service-demand throughput, and how much scheduling improvement is possible without extra cores or GPU? Does a zero-proof / fused-leaf or new-key option change this?

## Answer

- **CODE — Two different recursion lanes exist and must not be mixed.**
  - *Sherlock linked parent* (`recl9/src/sherlock_link.rs:1-20`): one parent circuit verifies two Moriarty Machine child proofs, A102 (Eval Literal → Return, line 102) and A80 (Return → StepDone, line 80), each under a fixed child key. It exposes 16 statement fields + a 12-field accumulator, then feeds the stock `DeciderRelation`. This is the "current Sherlock chain". **No measured run of this parent exists in the repo.** The only stats-writing paths for it are CLI `println!` JSON (`recl9/src/sherlock_cli.rs:234, 265, 291`), not `*.stats.json` files. The 2026-10-20 review README states it is "not an observed recursive proof" (`review/sherlock-accept-settle-20261020/README.md`).
  - *StrawAnchor 16-tx batch lane* (`recl9/REPORT.md` §1, §3): 4 verifier-free leaves (4 tx each) → one arity-4 node that verifies 4 leaf proofs in circuit (`recl9/src/node.rs:68-69` arity 2..=4; `node.rs:130` `verify_children_incircuit`) → decider. This is the only 16-tx chain with measured numbers. It is the workload the question calls hypothetical; it was proved end to end on 2026-10-14 and verified by the ledger-9 rc.6 path (REPORT §3, "Ledger-9 rc.6 verify").
- **CODE — Sherlock parent calls `setup_pk` on every proof.** `sherlock_link.rs:1041` `let pk = setup_pk(rel, vk);`, `:1044` `drop(pk);`, inside `prove_link_parent` (`:1017`). Confirmed. Keygen is `setup_vk` at `:973` (parent, cost-model k at `:971`) and `:1100-1102` (decider).
- **CODE — The recl9 CLI also rebuilds PK per invocation:** `recl9/src/main.rs:349` (leaf prove), `:462` (node), `:598` (decider). Each `prove` call in the 16-tx flow pays its own PK build (REPORT §6 commands).
- **CODE — Leaves are verifier-free.** `recl9/src/leaf.rs:1` module doc; `leaf.rs:42-43` n_tx in 1..=4. The verifier cost sits in the node: each extra child adds ~191,842 advice rows (`(957,724−574,040)/2`, REPORT §3 arity table).
- **SOURCE — `LOKI-DESIGN.md` L5 and `nodes/loki-08` (lines 31, 154–161)** use the 16-tx estimate of 370–640 s. Those inputs were not measured: leaf 60 s ref / 25 s accel, node P=500 s ref, decider 360 s ref (`loki-08:150-152`). Measured values below replace them.
- **ESTIMATE — Measured warm critical path (no keygen, sequential leaves, CPU-only, current code): 1,617 s ≈ 27 min.** Leaves 163.6 + node 867.3 + decider 586.4. Cold including PK builds: 1,775 s ≈ 29.6 min (adds leaf PK 6.4 + node PK 104.5 + decider PK 46.9).
- **ESTIMATE — loki-08's aP=aD=1 case (1,100 s) is ~1.47× too fast.** Measured node is 867.3 s = 1.73× its assumed 500 s; decider 586.4 s = 1.63× its assumed 360 s; leaves are faster than assumed (measured 35–49 s each vs 60 s). The observed case sits between loki-08's aP=aD=1 and aP=aD=2 rows.
- **ESTIMATE — Scheduling-only ceilings, same 20 cores, no GPU:**
  - **Measured CPU is far below capacity.** Node prove used 268% of CPU on average (`(2534.17+81.31)/973.7`), decider prove 401%. Over the node+decider serial wall (1,608.5 s), the average is ~3.2 of 20 cores (~16%). Total node+decider CPU-seconds 5,162.2 s.
  - **(a) Overlap the 4 leaves** (they are independent; `main.rs:354` runs them serially): critical path 1,617 → 1,503 s (−7%), bounded by the slowest leaf (49.4 s). Assumes leaves do not slow each other; leaf CPU% is not logged.
  - **(b) Two batches in flight** (node+decider of batch n concurrent with batch n+1): ~727 s per batch, i.e. (867.3+586.4)/2, ~2.2× throughput over 1,617 s. Assumes unchanged per-proof speed under contention. Peak RSS is ~15 GB per heavy proof (node 14,988,176 kB; decider 14,965,544 kB), so two in flight ≈ 30 GB of the 90 GB host. Host: 20 cores, 90 GB (verified with `nproc` and `free -g`).
  - **(c) Theoretical packing ceiling:** 5,162 CPU-s / 20 cores = **258 s per batch**, i.e. ~6.3×. Not reachable: node depends on all leaves, decider on node, and each proof uses only ~3–4 cores on average. Closing the gap needs intra-proof parallelism (the node proves at 268%), not just a scheduler.
- **ESTIMATE — Fused / zero-proof leaves and new keys.**
  - The current leaf is already verifier-free, so there is no zero-proof-leaf gain beyond what exists. The node verifier is the cost.
  - Fusing all 16 tx into one verifier-free leaf: linear extrapolation from measured leaf rows (1-tx 27,980; 4-tx 112,098; `REPORT.md` §3) gives ~447,680 rows (16 × 27,980). Table rows ~394k (4 × 98,563). Both fit k=19 (usable rows at k=19 = 2^19−7 = 524,281; the k=20 value 1,048,569 = 2^20−7 is in `node-keygen.log`). Prove time at k=19 is unmeasured; an extrapolation of ~4× the k=17 leaves gives ~140–220 s. This would remove the 867 s node from the path, but the ledger accepts only the decider's `acceptValidity` template proof (REPORT §5). A fused leaf therefore needs a new verifying key and template, i.e. **Track B (new keys)**, not a same-key change.
  - `mono/` note: `REPORT.md` §1 says mono keeps a 16-tx single-leaf fallback. The stats in the repo cover only 4-slot (`mono/keys/leaf4`) and 1-slot (`gate-jubjub`) keys. **No 16-slot measurement exists.** The mono leaf4 prove (61.9 s, `mono/out/leaf4/b1-g0/batch-1.stats.json`) is historical and must not be mixed with the recl9 leaves (34.8–49.4 s).

## Raw numbers

| Item | Value | Provenance |
|---|---|---|
| Leaf (4-tx) k / cost rows / table rows | 17 / 112,098 / 98,563 | `recl9/REPORT.md` §3; `recl9/keys/leaf4/keygen.stats.json` |
| Leaf (1-tx) k / cost rows / table rows | 15 / 27,980 / 26,883 | `REPORT.md` §3 |
| Leaf prove, 4 leaves (serial) | 34.78, 35.30, 44.19, 49.37 s; sum 163.63 s | `recl9/out/leaves-b1/leaves.stats.json` |
| Leaf pk (once) / keygen vk | 6.40 s / 31.0 s | same; REPORT §3 |
| Leaf proof bytes | 4,208 each | same |
| Node arity-4 k / rows / table rows | 20 / 958,095 / 1,032,465 (certified crate) | `recl9/keys/node4/keygen.stats.json`; `node-keygen.log` |
| Node keygen | vk 622.8 s; cost_model 335.25 s; wall 15:59.50; CPU 182%; peak RSS 4.50 GB | `node-keygen.log`; `keys/node4/keygen.stats.json` |
| Node prove | pk 104.46 s; prove 867.28 s; total 973.71 s; proof 6,432 B; RSS 14.99 GB | `recl9/out/node-b1/w.stats.json`; `node-prove.log` |
| Node time -v | user 2,534.17 s, sys 81.31 s, 268%, 16:13.73 | `node-prove.log` |
| Decider k / rows / table rows | 20 / 841,682 / 1,032,209 | `recl9/keys/decider/keygen.stats.json`; REPORT §3 |
| Decider keygen | 9:05.18 wall, 309% CPU, RSS 4.77 GB | `decider-keygen.log` |
| Decider prove | pk 46.89 s; prove 586.38 s; total 634.82 s; verify 0.0068 s; proof 6,432 B; RSS 14.97 GB | `recl9/out/decider-b1/batch-1.stats.json`; `decider-prove.log` |
| Decider time -v | user 2,486.95 s, sys 59.80 s, 401%, 10:34.85 | `decider-prove.log` |
| Sherlock parent | no measured run; k not recorded | `sherlock_cli.rs:265` prints JSON only; review README 2026-10-20 |
| Arity-2 node (interim crate) | advice 574,040, k 20 | REPORT §3 |

Derived (arithmetic):
- Warm serial: 163.63 + 867.28 + 586.38 = **1,617.29 s**.
- Cold: 1,617.29 + 6.40 + 104.46 + 46.89 = **1,775.04 s**.
- Leaves overlapped: 1,617.29 − 163.63 + 49.37 = **1,503.03 s**.
- Two-in-flight interval: (867.28 + 586.38)/2 = **726.83 s**.
- Packing ceiling: (2,615.48 + 2,546.75)/20 = **258.11 s**.
- Average cores in node+decider: 5,162.23 / 1,608.53 = **3.21**.
- loki-08 implied factors: node 867.28/500 = 1.73; decider 586.38/360 = 1.63.

## Caveats
- One run per stage; no repeats, so variance is unknown. `/usr/bin/time` CPU% is an average over the job, not a peak.
- Node and decider `threads: all` but CPU% (268%, 401%) shows the work was not saturating 20 cores. The leaf runs have no `time -v` log, so their CPU use is unknown.
- The stats files carry mtime 2026-10-17 (copy time). Measurement date is 2026-10-14 (REPORT header). Node and decider SRS: `bls_midnight_2p20`, pin match true.
- Sherlock child sizes (A102/A80 k and rows) and the parent k are not in any recl9 artifact I read. Treat the Sherlock parent cost as OPEN.
- Scheduling ceilings (a)–(c) and the fused-leaf k=19 timing are ESTIMATES from arithmetic, not measurements. The 258 s and 727 s figures assume no contention penalty and unchanged per-proof speed.
- Host figures (20 cores, 90 GB) come from `nproc` and `free -g` on this machine, which matches loki-08 but is not in the recl9 logs.
- Nothing was built or run; no repository was modified.

## Addendum (after bus FYI from astra-2)
- **CODE — Leaf trust caveat.** `recl9/src/leaf.rs:7-9`: the leaf prover holds every spender's `sk` (L0 computes `pk = sk*G`, `nk = H(DOM_NK, sk)` in circuit). Acceptable only for the synthetic strawman; production leaves verify user wallet proofs. So the 16-tx fused-leaf option cannot be presented as production wallet throughput.
- **CODE — Second cost_model call in the Sherlock parent.** `sherlock_link.rs:1029` calls `cost_model(rel, None)` on every `prove_link_parent` (inside the degree check), in addition to `setup_pk` at `:1041`. Its cost is not measured.

## CORRECTION (supersedes the "no measured run" statements above)
- **My earlier statement that no Sherlock parent run is measured was wrong.** A receipt exists: `docs/results/sherlock-native-observations-20261022/receipts/parent-proof.json` (Moriarty-factory). Observed fields: `circuit_k` 20; `proof_bytes` 6,432; `verifying_key_bytes` 2,189; `phases.setup_pk.duration_seconds` 62.328385; `phases.prove.duration_seconds` 882.589555 (scope: single CLI process phase); `phases.pairing_verify` 0.012975; `phases.stdlib_verify` 0.009061; child A102 verify 0.019319 s; child A80 verify 0.019102 s; accumulator fold 0.010638 s; folded pairing 0.000738 s; `nested_wall_observations.native_capture_seconds` 951.614; `prefinal_service_snapshot.memory_peak_bytes` 15,678,128,128 (mixed service counter, not isolated proof RSS); `cpu_seconds` 1,820.36 (same mixed counter; not usable for CPU%).
- **Sherlock parent warm/cold arithmetic (ESTIMATE from this receipt):** cold = 62.33 + 882.59 = 944.9 s; PK removal would save 62.33/944.92 = 6.6%. This is one run; the receipt does not say whether the prover was a cache-warm or cold build beyond what its own fields show.
- The receipt's k=20 and 6,432-byte proof match the recl9 node and decider sizes, consistent with a same-shape 2-child parent (not verified beyond this).
- Also cited as `wiki-llm/loki/nodes/loki-05-recursion-accumulation.md:129` (same numbers). I did not re-run anything.

## Addendum 2 (child sizes, verified from a Moriarty receipt)
- **SOURCE/CODE — Actual Sherlock children are small.** `docs/results/sherlock-native-a102-a80-child-producer-20261021/actual/RUN-REPORT.json` (Moriarty-factory): A102 `circuit_k` 13, `proof_bytes` 3,440, `instance_lanes` 253; A80 `circuit_k` 12, `proof_bytes` 3,440, `instance_lanes` 253. Phases: `a102-prove` 2,098 ms; `a80-prove` 1,156 ms; key reconstruct 2,699 ms and 843 ms; `verify-linked-children` 3,436 ms. Scope self-declared as "not parent recursion, terminal validity, ledger acceptance or full-39 coverage".
- **Caveat:** this run used `zkir_version` 3.0.0-rc.2 and ledger revision 04c9c5d9, a different toolchain from the recl9 midnight-proofs 0.8.2 measurements above. Its timings are not directly comparable to the recl9 numbers. astra-2 reported it as a debug build; I did not verify that.
- Consequence: the Sherlock parent at k20 (receipt) is about 7–8 k above its children, so the parent and terminal are the dominant cost. The "child sizes unknown" caveat is resolved for this toolchain only.

## CORRECTION 2 (supersedes the CPU, packing and "unreachable" wording above)
- **Whole-invocation CPU, not prove-only.** The `/usr/bin/time -v` blocks cover whole invocations, and PK construction runs inside each one: `recl9/logs/node-prove.log:3-9` (`[node] pk 104.5s` at line 3, prove at line 4, `User time` 2534.17 s line 6, `System time` 81.31 s line 7, `Percent of CPU` 268% line 8, `Elapsed` 16:13.73 line 9, max RSS 14,988,176 kB line 14); `recl9/logs/decider-prove.log:3-9` (`[decider] pk 46.9s` line 3, `User time` 2486.95 s line 6, `System time` 59.80 s line 7, 401% line 8, `Elapsed` 10:34.85 line 9, RSS 14,965,544 kB line 14). The 5,162 CPU-s therefore includes pk and cannot be split into prove-only CPU from these logs. "Warm 1,617 s over total CPU" is not an exact ceiling.
- **Packing bound is necessary, not proven.** 258 s = 5,162 CPU-s / 20 cores is a lower bound on the wall time these CPU-seconds could occupy, not evidence that 258 s is reachable. Several serial builders could in principle pack 20 cores unless memory limits bind; that is untested. Read "not reachable" above as "not established as achievable".
- **Thread setting.** `recl9/src/main.rs:171-173` records `threads` as `RAYON_NUM_THREADS` or `"all"`. The logs show `threads: all`, meaning the env var was unset, so Rayon used its default (logical cores). The logs do not record the logical core count or CPU model. The 20-core figure comes from `nproc` on the current host and cannot be attributed to the historical run.
- **Leaf PK.** `recl9/src/main.rs:349` runs `setup_pk` once for the leaf batch and reuses it across the four leaves, so the leaf PK is 6.40 s once (from `leaves.stats.json` `keygen_pk_s`), not per leaf. The 163.6 s leaf sum is prove-only, consistent with the leaves stats file.
- **Table and cost rows are not linear in transaction count.** The range-tag union changes the table with n_tx. The 16-tx row and table predictions in astra-3-5 and any "k19 fit" claim are unsupported and are retracted. The k20 table caps in the node are capacities, not row counts.

## CORRECTION 3 (midnight-proofs version of the historical timings)
- **The recl9 timings were produced on midnight-proofs 0.8.2, not 0.8.3.** `recl9/Cargo.lock` (line 647-648) pins `midnight-proofs` 0.8.2. The loki design text says Moriarty pins 0.8.3.
- **MSM dispatch differs.** 0.8.2 `src/poly/kzg/msm.rs:176-189` (registry `midnight-proofs-0.8.2`) uses blst's `multi_exp` only when `coeffs.len() <= (2 << 18)` = 524,288 and otherwise `msm_best`. The k20 node and decider MSMs (2^20 terms) therefore take the `msm_best` path. 0.8.3 `src/poly/kzg/msm.rs:176-192` always uses blst Pippenger for BLS12-381 G1, and its comment says `msm_best` pays a per-base subgroup check and is several times slower on large MSMs.
- **Consequence:** the historical 867 s node prove and 586 s decider prove are not the 0.8.3 baseline. The size of the difference on this host is unmeasured. Any "current baseline" must be re-measured on the pinned version.
- Actual parent receipt numbers (setup 62.33 s, prove 882.59 s) are also from the 0.8.2-era lane per their toolchain; do not mix with 0.8.3 claims.
