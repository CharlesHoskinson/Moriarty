# astra-3-11: review of astra-3 PROPOSAL.md (DAG and new-key semantics, 0.8.2 vs new baseline, 16 GB scope, predicates)

## Question
Review `wiki-llm/loki/10x/astra-3/PROPOSAL.md` for: DAG and new-key semantics; old 0.8.2 versus new baseline; the 16 GB B7 scope; predicates; and whether the "direct terminal ≤ 88.259 s" wording claims a measured speed. Extrapolations must be conditional. No edits to the lead's files.

## Answer

### Verified (CODE / SOURCE, checked by me)
- **Version split is real.** `recl9/Cargo.lock:647-648` pins `midnight-proofs` 0.8.2. Moriarty `Cargo.toml:40` and `crates/mori-backend/Cargo.toml:15` pin `=0.8.3`; `Cargo.lock:2030-2031` has 0.8.3 (also a 0.7.3 entry at `:2010-2011`).
- **MSM dispatch differs.** 0.8.2 `src/poly/kzg/msm.rs:176-189` uses blst `multi_exp` only for `len <= 2<<18` (524,288) and `msm_best` otherwise. 0.8.3 `src/poly/kzg/msm.rs:176-192` always uses blst Pippenger for G1. The proposal's citation of 0.8.3 `:176` is correct.
- **Lookup HashMap randomness is in both versions.** `src/plonk/lookup/prover.rs:399` (`HashMap::with_capacity`) and `:430` (iteration) are identical in 0.8.2 and 0.8.3. The proposal's `P/src/plonk/lookup/prover.rs:399,430` is correct.
- **Setup and prove arithmetic.** `receipts/parent-proof.json`: `setup_pk` 62.328385318 s; `prove` 882.589555363 s, and the receipt README (`docs/results/sherlock-native-observations-20261022/README.md:5`) says this prove interval includes explicit PK drop. `(62.328+882.590)/882.590 = 1.07062` is correct. `(500+120)/500 = 1.24` and `500/(500-120) = 1.316` are correct.
- **Fenrir historical sums.** Leaves 163.630576714 s, node 867.279750364 s, decider 586.381689547 s, total 1,617.292016625 s; PK sum 157.751410713 s; setup-inclusive 1,775.043 s; ratio 1.09754. All correct.
- **CPU effective cores.** Node 2,615.48 CPU-s over 973.73 s = 2.686; decider 2,546.75 over 634.85 = 4.012. Correct. These are whole-invocation values, including PK (`node-prove.log:3-9`, `decider-prove.log:3-9`).
- **Sherlock children.** `RUN-REPORT.json` shows A102 `circuit_k` 13 and A80 `circuit_k` 12 (3,440 B each). Correct.
- **Speedup budget arithmetic.** `0.25 + 0.75/10 + 0.01 = 0.335`, so 1/0.335 = 2.98507; 541.793/154.705 = 3.50211; product 10.454; 3600/154.705 = 23.27 batches/h. Correct, as is 1/0.26 = 3.846.
- **Predicates.** `qa/sherlock/predicates.yaml:12-20` (A2), `:115-121` (B3), `:154-160` (B7) are `unimplemented.sh` checks. B7 is "peak under decimal 16 GB with swap 0". The proposal's citation of line 154 is correct.
- **Swap is already in use on this host.** `free -g` at 22:51 UTC shows Swap 32 GiB total, 8 GiB used. So swap 0 is not satisfied on the current host, whatever the experiment.

### Problems to fix in the proposal

1. **The ≤ 88.259 s target wording.** 88.259 = 882.590/10, so the target is exactly "at most one tenth" of the parent alone. The proposal says "would be below one tenth", which should read "at most one tenth". It also reads as if a 10× result is in hand. It is a target only. Logic check: the old complete chain is at least the parent alone (882.59 s) plus children plus the old decider, which is unmeasured for this pair, so a new terminal at or below 88.259 s, under the same witness-input-to-verified-terminal scope and ledger statement, would be at least 10× against the old chain. That implication is valid only if the new terminal includes every obligation the old ledger-bound terminal covers. The proposal should state this scope condition explicitly.

2. **"Below 6.88×" ceiling is mis-scoped.** `258.11 s` = (node+decider CPU-s)/20 excludes leaf CPU, while the denominator `1,775.04 s` includes leaf work. Leaf CPU is not measured (no `time -v` for the leaf batch). The ratio therefore overstates the ceiling. Either add leaf CPU (unknown) or label the result "node+decider only, leaves excluded". As written it should not be quoted.

3. **Memory estimate uses the wrong fixed-column count.** §4 uses "historical 27 fixed plus 18 permutation columns". Measured node and decider are `fixed_columns` 45 with 18 permutations (`recl9/keys/decider/keygen.stats.json:6,10`; node keygen 45 in `recl9/logs/node-keygen.log`). The leaf is 32 fixed (REPORT §3). With 45+18 under the proposal's own formula, `32 MiB × [(45+18)×6+12] = 32 × 390 MiB = 12,480 MiB = 12.19 GiB` per key shape, and two keys ≈ 24.38 GiB, not 17.625 GiB. The ×6 and +12 factors are the proposal's assumptions and need a measured decoded-key size.

4. **Baseline version.** The proposal's "old" numbers are 0.8.2 (recl9). Its "new" baseline must be 0.8.3 (Moriarty's pin), so the 867.28 s and 586.38 s values are not a current baseline. The 0.8.3 path is faster or differently scheduled for MSM sizes above 2^19, but the size of the difference on this host is unmeasured. The parent receipt's midnight-proofs version is not stated in the fields I read, so it cannot yet be placed on either version. Before the 882.59 s figure is used as the denominator for (b), its version must be confirmed.

5. **Warm vs cold.** The "warm caching factor 1.000×" row and the "setup-inclusive 1.0975×" row are consistent. The pure-scheduling ceiling (item 2) mixes them and should not.

6. **Modelled fractions.** The 25% CPU, 75% bulk at 10×, 1% overhead and "four workers" inputs are unmeasured, as the proposal says. The 10.45× result should keep its "conditional on assumed fractions" label wherever it is quoted, including the verdict paragraph.

### Predicates
- A2, B3 and B7 are unimplemented (verified above). The proposal's F301-F307 do not exist yet and are labeled PROPOSAL, which is correct.
- The claim that `scripts/sherlock-next --json` named A2 first failing was **not verified by me** (I did not run repo scripts).

### Not checked by me (cited in the proposal; treat as unverified here)
- `P/src/plonk/mod.rs:416`, `P/src/poly/kzg/params.rs:40`, `L/transient-crypto/src/proofs.rs:250,545,64`, `L/proof-server/src/worker_pool.rs:236`, `Z/src/lib.rs:1529,1593`, `R/blst-0.3.17/src/lib.rs:41`, `M/models/doctorwatson/factory/heavy.py:225`.
- Nocy timings (12.3→4.4 s at k17; 24.6→8.8 s at k18), ICICLE and NVIDIA captures, PR #480 at `31435e9`.
- `queue_model.py` results (I did not re-run the simulation).

## Raw numbers (checked)

| Item | Value | Source |
|---|---|---|
| Parent setup / prove | 62.328385318 s / 882.589555363 s | `receipts/parent-proof.json` |
| 882.590 / 10 | 88.259 s | arithmetic |
| Fenrir sum (prove only) | 1,617.292016625 s | leaves.stats, w.stats, batch-1.stats |
| Setup-inclusive | 1,775.043 s; ratio 1.09754 | same |
| Node / decider CPU-s | 2,615.48 / 2,546.75 | `node-prove.log:6-7`, `decider-prove.log:6-7` |
| Packing bound (node+decider only) | 258.11 s; 6.88× vs 1,775.04 (mis-scoped) | arithmetic |
| Decider / node fixed columns | 45 / 45 (18 permutations) | `keygen.stats.json:6,10`; node-keygen.log |
| Corrected per-key decoded estimate | 12.19 GiB (assumed factors) | arithmetic |
| Host swap now | 8 GiB used of 32 GiB | `free -g`, 22:51 UTC |
| B7 cap | 16,000,000,000 B, swap 0 | `predicates.yaml:156-160` |

## Caveats
- All extrapolations are conditional on the assumed fractions and the 0.8.2/0.8.3 gap.
- Host swap 8 GiB in use now means any B7 run on this host needs the swap state recorded for that run, not inferred.
- No repository was edited; no build, proof, or key generation was run.

## CORRECTION to my item 3 and item 2 (after astra-3's source pointer)
- **Item 3 is retracted.** `midnight-proofs-0.8.3/src/dev/cost_model.rs:244` sets `fixed_columns: self.fixed.len() + self.permutation.columns`. The reported 45 therefore already includes the 18 permutation columns, and the actual fixed count is 27. The proposal's "27 fixed plus 18 permutation" memory estimate is correct as written. The leaf's reported 32 fixed likewise means 22 fixed plus 10 permutations (REPORT §3). My "12.19 GiB per key" figure is withdrawn.
- **Item 2 is downgraded.** The 258.11 s figure covers only node and decider CPU-seconds, so it is a valid lower bound on the ideal wall time for all work, which includes leaves. Therefore 1,775.04 / 258.11 = 6.88 is a loose upper bound on the ceiling, not an invalid one. Adding leaf CPU lowers it. It should be quoted only as an upper bound.
- The other items stand: the 0.8.2/0.8.3 split, the 88.259 s scope wording, the B7 and swap facts, and the predicates.
