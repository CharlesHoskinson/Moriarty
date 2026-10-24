# astra-3-8: baseline and acceptance experiment design for 16-tx throughput (same-key and new-key)

## Question
Design a real baseline and acceptance experiment for same-key and new-key throughput on the 16-transaction workload on this host. Specify metrics, denominators (cold/warm, concurrency), DAG and state-dependency handling, rekey deploy cost (one-time vs per-batch), and the acceptance gates. Design only: no repo edits, no proofs run, no fees invented.

## Answer

### 1. Status of the gates this experiment must respect (CODE)
- `qa/sherlock/predicates.yaml:12-20`: A2 (two linked child proofs A102 to A80 compiled, proved and accepted) is `scripts/qa/unimplemented.sh A2`, heavy, budget `a2-linked-children`. `qa/sherlock/budgets.json:4-11` has all `measured` fields null.
- `predicates.yaml:115-121` B3 (one 16-tx full batch, proved and verified locally) and `:154-160` B7 (peak under decimal 16 GB, swap 0, time/RSS/bytes in separate scopes) are unimplemented (exit 2). Budget `b3-full-batch-local` has `maxRssBytes: null`.
- The claim "A2 first failing" is not verified here: I found no result file recording an A2 run. Treat it as a claim from astra-3 until a result file is cited.
- Consequence: any run of the arms below is a declared resource experiment, not B3 or B7 evidence, unless it meets the B7 scope rules.

### 2. Arms (each arm is a separate, named configuration; none is relabeled)
- **S0 same-key serial baseline (existing code).** recl9 4 leaves → arity-4 node → decider, stock keys (`recl9/src/main.rs` CLI commands per `recl9/REPORT.md` §6). Measured warm 1,617.3 s (sum of stage prove times, keygen excluded). This is the denominator for any "Nx" claim, and must be re-run on this host under the same protocol before any comparison.
- **S1 same-key Sherlock linked path.** A102 (k13) and A80 (k12) children (`docs/results/sherlock-native-a102-a80-child-producer-20261021/actual/RUN-REPORT.json`), parent k20 (`docs/results/sherlock-native-observations-20261022/receipts/parent-proof.json`: setup_pk 62.33 s, prove 882.59 s), decider. Different toolchain (zkir 3.0.0-rc.2) from S0, so it is not a comparison arm against S0 without a same-toolchain rerun.
- **N1 new-key fused 16-tx leaf (k19 candidate).** Needs new key, new template, new acceptance path (`astra-3-5.md`). Arm only if the acceptance route exists; otherwise design-only.
- **N2 new-key fused A102+A80 relation (k14–15 candidate, astra-2 hypothesis).** Not built. Design-only until a relation exists and its own constraint count is measured.

### 3. Metrics (per arm, per stage; each in its own scope per SRPOC-005 and B7)
- **Wall time** per stage: keygen (vk, pk) reported separately; prove; verify (local stdlib and ledger-verify); write. Use `/usr/bin/time -v` as in `recl9/logs/*.log` for process wall, user, sys, and percent CPU.
- **Core-seconds** = user + sys per process (from `/usr/bin/time`), plus whole-run core-seconds from cgroup `cpu.stat` `usage_usec`, so that overlap between concurrent processes is counted once.
- **CPU RSS**: `%M` (max RSS) per process. **PSS**: sum of `Pss` from `/proc/<pid>/smaps_rollup` sampled at 1 Hz across the process tree. Report the peak of the summed PSS and the peak of summed RSS separately.
- **Swap**: `vmstat` si/so columns during the run, and `/proc/meminfo` SwapUsed before and after. B7 requires swap 0; record it.
- **VRAM peak** (GPU arms only): `nvidia-smi --query-compute-apps=pid,used_memory --format=csv -l 1`, sampled peak for all processes of the arm. Record the sampled method.
- **GPU timing** (GPU arms only): CUDA events around each kernel stage, synchronized, reported as device time separate from host wall. Without events, GPU stage timings are not reported.
- **Per-stage counts**: k, cost rows, table rows, column counts, domain capacity (2^k), proof bytes, VK bytes, SRS sha256 (per `recl9/REPORT.md` §3 and SRPOC-005). Domain capacity is not used rows.
- **Failures and censored runs** kept with the same schema (SRPOC-004). A run killed by a cap is recorded as censored, not dropped (astra-1 noted the same).

### 4. Denominators
- **Count only complete ledger-9-verified deciders.** A batch counts toward throughput only when its decider proof is accepted by the ledger verify step (`ledger-verify` returning `honest: true`, as in `recl9/REPORT.md` §3 "Ledger-9 rc.6 verify") with all negatives rejecting. Partial proofs, a parent without a verified decider, and runs cut by a cap do not count.
- **Throughput** = verified batches / (last verified decider time − first start time), including all stages, for a fixed number of batches (for example 2, 4, 6 per SRPOC-004 prefixes).
- **Per-batch latency** = submit-to-verified time for each batch, reported as a distribution, not a mean alone.
- **Cold** = first run including keygen and PK construction (for S0: leaf pk 6.40 s, node pk 104.46 s, decider pk 46.89 s; node keygen vk 622.8 s, decider keygen 412 s per REPORT §3). **Warm** = after PK/VK cache, with keygen excluded and stated. Both denominators are reported; the 16-tx warm figure is 1,617.3 s and cold is 1,775.0 s for S0 (existing data).
- **Concurrency denominator.** The serial baseline is the best single-process configuration measured on this host, not an arbitrary one. Concurrency is swept (for example 1, 2, 4 workers) on the same batches, and the best measured concurrency is reported next to serial. A concurrency result is valid only if the serial baseline was tuned with the same effort.

### 5. DAG, speculation and state dependencies
- **Leaves depend on pre-state.** Each batch's leaves take a pre-root from the previous batch's post-state (`recl9/REPORT.md` §4: the chain lane sets the real binding per submission). Speculative leaves for batch n+1 need a predicted root. If batch n fails verification or the predicted root differs, every speculative artifact for n+1 is discarded and counted as wasted core-seconds, not as throughput.
- **Parent after children.** `recl9/src/sherlock_link.rs` `prove_link_parent` (`:1017`) verifies both children off-circuit before its prove; `setup_pk` is at `:1041`. A parent proof must not start until both children are verified. Log the child-verified timestamp per batch.
- **Node after leaves** (S0): the node takes all four leaf proofs (`recl9/src/node.rs:130`), so it is blocked until all leaves for that batch are verified.
- **Decider after node.** The decider's pinned W vk must match the node's (`REPORT.md` §3: `w_vk sha256 ed68cf2d...`). A key change in the node changes the decider template; it cannot be overlapped with a stale decider.
- **Determinism for GPU equivalence.** Fixed RNG and fixed thread count do not give byte-identical proofs by themselves: `prover.rs` (midnight-proofs 0.8.3) uses a random-seeded leftover table map and iterates a HashMap (astra-1 report, `prover.rs:399`, `:430`). Equivalence must use lookup-multiset and adjacency equations, or a test-only deterministic map injected in both fixtures.

### 6. Rekey: one-time vs per-batch cost
- **One-time (new key path, N1/N2):** new keygen (vk and pk), new verifier and anchor deployment on stagenet, new template capture and pins (SHA-256 of template, VK, relation), and ledger acceptance of the new verifier. Record each as a separate line with its own tx hash; do not amortize it into a per-batch number until the count of batches to amortize over is fixed in the design.
- **Per-batch:** proof generation, ledger verify, submission and settlement readback. Submission fees are **UNKNOWN in the sources read**; they must be measured on the target network and reported as separate fields (SRPOC-006). No fee number is assumed here.
- **Same-key arms** (S0, S1) have no rekey cost; their deploy cost is the existing anchor/verifier deployment, reported once.

### 7. Acceptance gates
- **B3:** one full 16-tx batch proved and verified locally, using the exact workload expectations frozen before execution (`tasks.md` item "Freeze independent minimal recursive ... expectations", `openspec/changes/sherlock-stagenet-recursive-rollup/tasks.md`).
- **B7:** time, RSS, bytes in separate scopes; peak under decimal 16 GB (16,000,000,000 B) with swap 0. Measured per-process peaks for S0: node 15.35 GB, decider 15.32 GB, leaves 1.49 GB; two heavy proofs resident would be about 30.7 GB and fail. A run holding two heavy proofs is a declared resource experiment only, and it does not pass B7 regardless of host RAM (the host has 90 GB; the cap is not host-relative).
- **B6:** per-layer k, domain capacity, used rows, lookup rows, columns, with unavailable values explicit.
- **B4/B5:** stagenet acceptance and 2/4/6-batch prefixes, with failures kept.
- **Negative controls** for each arm: the public-input and proof-byte tamper sets from `recl9/REPORT.md` §3 (node and decider tables), all must reject.

## Raw numbers used (all from files cited above)

| Item | Value | Source |
|---|---|---|
| S0 warm (leaves sum, node prove, decider prove) | 163.63 s, 867.28 s, 586.38 s = 1,617.29 s | `recl9/out/*/stats.json`, `recl9/logs/*.log` |
| S0 cold adds pk | 6.40 s, 104.46 s, 46.89 s = total 1,775.04 s | same |
| S0 node CPU | 268% (user 2,534.17 s, sys 81.31 s) | `recl9/logs/node-prove.log` |
| S0 decider CPU | 401% (user 2,486.95 s, sys 59.80 s) | `recl9/logs/decider-prove.log` |
| S1 parent (receipt) | k20, setup_pk 62.33 s, prove 882.59 s, proof 6,432 B | `receipts/parent-proof.json` |
| S1 children | A102 k13 3,440 B; A80 k12 3,440 B; 253 instance lanes | `RUN-REPORT.json` |
| Node/decider peak RSS | 15,347,892,224 B / 15,324,717,056 B | `recl9/logs/*.log` (kB ×1024) |
| B7 cap | 16,000,000,000 B | `qa/sherlock/predicates.yaml:156` |

## Caveats
- Every number in the S1 row comes from a different toolchain (zkir 3.0.0-rc.2) than S0 (midnight-proofs 0.8.2). They are not comparable without a same-toolchain rerun.
- No measurement of GPU or VRAM exists in these sources for this host. GPU arms are design-only here.
- The 16-tx N1 and N2 arms are unbuilt; their costs are estimates (`astra-3-5.md`, `astra-3-2.md`).
- Submission fees, finalization and settlement latency are not in the sources read and are left as UNKNOWN fields.
- Nothing was run. No repository was edited. No proofs were generated.
