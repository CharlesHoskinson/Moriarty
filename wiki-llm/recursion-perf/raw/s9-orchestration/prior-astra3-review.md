# Earlier queue-model review: secondary analysis
> Source: file:///home/charl/code/Moriarty-perf/wiki-llm/loki/10x/helpers/haiku-08/astra-3-7.md
> Collected: 2026-10-26
> Published: Unknown

# astra-3-7: adversarial review of queue_model.py and queue_model_results.json

## Question
Check denominator fairness, event math, concurrency semantics, finite buffers, and which assumptions make the 10.45x result fail. Files reviewed (read only): `/home/charles/repos/Moriarty-factory/wiki-llm/loki/10x/astra-3/queue_model.py`, `queue_model_results.json`, and `PROPOSAL.md` (sections 1, speedup budget, memory). I did not edit parent files. The sensitivity grid below was run in-process with `python3 -B` and `exec` of the script's `simulate` (no files written, no `__pycache__`, no proofs run).

## Verdict
- **The event loop's arithmetic is correct** (checked below). The 10.45x is what the code computes for its inputs.
- **The inputs do not survive the review.** The result is driven by two unsupported assumptions: (a) the CPU phase is charged as a wall fraction of 0.25 on a worker that is assumed to be "2 effective cores", which the measured baseline contradicts; (b) the GPU bulk speed of 10x, which the design's own estimates do not support (design: about 1.6x MSM-only, about 3x with the quotient). At the design-supported bulk speed of 3x the pipeline gives about 3.8x to 4.5x.
- **Memory is not modeled.** `host_slots=5` is a count, not a byte budget. Measured per-process RSS is about 15 GB, so 5 slots exceed the 64 GiB experiment budget unless the proving key is shared.
- **The 10.45x margin is small in any case.** The PROPOSAL states about 4.5% slowdown erases it. My runs show that corrected CPU fractions alone move it to 9.7x and 7.4x, and the bulk speed moves it below 5x.

## 1. Denominator fairness
- **Numerator** `sum(TIMES)` = 1617.29 s is prove-only: leaves 163.63 s (`recl9/out/leaves-b1/leaves.stats.json` `prove_s`, CODE), node 867.28 s (`out/node-b1/w.stats.json` `prove_s`, CODE), decider 586.38 s (`out/decider-b1/batch-1.stats.json` `prove_s`, CODE). The values match. PK setup is excluded from both sides of the ratio, so the ratio is a "warm" comparison. That is acceptable only if the pipeline also gets PK caching for free. PK setup in these runs is 104.46 s (node), 46.89 s (decider) and about 6.4 s for leaves (PROPOSAL section 1 gives 157.75 s total). The pipeline cannot run warm without the L1 PK cache, which my astra-3-1 shows is worth only 1.07x to 1.12x per stage.
- **The baseline is not one measured serial batch.** It is a sum of three separate process runs, each with its own SRS read and RSS. Those runs are on recl9 (midnight-proofs 0.8.2, `Cargo.lock` in recl9), but the candidate is pinned to Moriarty's 0.8.3 (`/home/charles/repos/Moriarty-factory/Cargo.toml:40`). In 0.8.3 the MSM at k20 always uses blst Pippenger (`midnight-proofs-0.8.3/src/poly/kzg/msm.rs:181`), while 0.8.2 switches to `msm_best` above 2^19 terms (`0.8.2 msm.rs:178`). The baseline binary is therefore not the candidate binary, and the direction of the error is unknown.
- **One CPU fraction for all stages.** The model applies 0.25 to a k17 leaf (35 s) and to a k20 node (867 s) alike. Leaves are small and their CPU share is unmeasured. The assumption is stated in the model's docstring but not justified.
- **Denominator arithmetic is fine.** `speedup = sum(TIMES)/interval` uses the same sum as the baseline, so the ratio is consistent within the model.

## 2. Event math (checked line by line against queue_model.py)
- **Steady interval** `(completed[batches-21] - completed[19]) / (batches-40)`: indices 179 and 19, 160 intervals, divisor 160. Correct.
- **Eligibility:** leaves (s<4) need only the window check. The node (s=4) needs leaves 0..3 done (control complete). The decider (s=5) needs the node done. Both dependencies are real. Requiring the leaf's control phase to finish before the node starts is conservative, not wrong.
- **Lock-step artifact:** every batch has identical durations (`TIMES[s]` times fractions), no variance, no jitter. Identical phases in identical windows can make the scheduler look smoother than real. The sensitivity runs do not vary durations.
- **Per-job resource:** a job holds a host slot from CPU start until control completion (`slots` incremented at CPU start, decremented at control end). Slot pressure is therefore counted correctly. Memory is not held per slot (see section 4).
- **Priority:** within a batch the scheduler serves downstream stages first (`range(5,-1,-1)`), and oldest batch first. This is a policy choice; it is not tested against alternatives.
- **Asserts** on `slots`, `cpu_busy`, `gpu_busy`, `control_busy` hold by construction.
- **No code bug found** in the event loop. The one issue is that the stated assumption "2 effective cores per CPU worker" is not enforced anywhere in the code: `cpu_workers` is a count of slots whose duration is `cpu_fraction * TIMES`, independent of cores. Neither the 20-core host limit nor per-core contention is modeled.

## 3. Concurrency semantics
- **CPU lane:** `cpu_workers` concurrent CPU phases, each of duration `cpu_fraction * T_stage`. This is a wall-time share, not core-seconds. The implied core demand is not checked against the 20 logical CPUs.
- **Baseline utilization is measured, and it contradicts the model.** The node invocation log `recl9/logs/node-prove.log` (lines 5-9, GNU `time -v`) gives user 2534.17 s + system 81.31 s over elapsed 16:13.73 (973.73 s), 268% CPU. The decider invocation `recl9/logs/decider-prove.log` (lines 5-9) gives user 2486.95 s + system 59.80 s over 10:34.85 (634.85 s), 401% CPU. These are per-process numbers including PK setup, so they are attributable. The 2.69 and 4.01 effective-core figures in PROPOSAL section 1 are confirmed. Both figures are on the 20-core host.
- **Consequence:** if the CPU phase is 25% of wall at the baseline's 2.69 to 4.01 effective cores, its core-seconds are 0.25 T x u. On a 2-core worker that takes 0.125 T x u of wall time, so the effective CPU fraction is 0.336 (node) to 0.50 (decider), not 0.25. The 0.25 setting therefore understates CPU time by 1.34x to 2x.
- **GPU lane:** one lane, service `(1-cpu_fraction)/bulk_speed * T`. The 75% remainder is assumed to be fully GPU-accelerable at 10x with no CPU residue. The design's own estimate is about 1.6x MSM-only and about 3x with the quotient (LOKI-DESIGN L3/L4). Nothing in the model justifies 10x.
- **Involuntary context switches:** the node run shows 4,599,052 involuntary context switches (node-prove.log, GNU `time -v`, line 19; decider-prove.log line 19 gives 1,255,045). That is heavy preemption at 268% CPU on 20 logical CPUs. It is a sign of oversubscription between rayon and blst's own 20-thread pool (astra-3-4 section 3), which the model does not represent.

## 4. Finite buffers
- The only buffer is `host_slots` (a count). The model has no byte budget.
- **Measured per-process peak RSS:** node 14,988,176 kB; decider 14,965,544 kB (GNU time logs above). Also `[node] pk 104.5s rss 11799136 kB` (node-prove.log line 3), i.e. about 11.25 GiB resident after PK setup. That supports the PK-resident estimate in astra-3-4 section 5 (about 11.6 GiB).
- **Slots against memory:** with duplicated PKs, each slot is about 15 GB, so 4 slots is about 60 GB and 5 slots is about 75 GB. The PROPOSAL's experiment budget is 64 GiB and B7 requires peak below 16 GB (PROPOSAL section 4). 5 slots fail the 64 GiB budget unless the PK is shared.
- **With an Arc-shared PK** (astra-3-4 section 5, ESTIMATE): about 11.6 GiB shared plus about 4.5 GiB per slot (pause state about 1.1 GiB plus finalise transient about 3.4 GiB), so 5 slots is about 34 GiB. That fits 64 GiB. The 5-slot case is therefore feasible only with a shared PK, which the model does not know about.
- **VRAM:** the GPU lane is serialized, so at most one heavy device proof is modeled. Device-memory admission (the design's 19 GiB at N=2^22, PROPOSAL section 4) is not in the model.

## 5. Sensitivity (run here; cores_per_worker = 2, 4 workers, 6 batches, same TIMES)

| cpu_fraction | bulk_speed | host_slots | steady interval (s) | speedup vs 1617.29 s |
|---|---|---|---|---|
| 0.25 (as submitted) | 10 | 5 | 154.7 | 10.45 |
| 0.25 | 10 | 4 | 157.5 | 10.27 |
| 0.25 | 10 | 3 | 191.2 | 8.46 |
| 0.25 | 10 | 2 | 283.6 | 5.70 |
| 0.336 (node-measured u=2.69) | 10 | 5 | 166.5 | 9.71 |
| 0.336 | 10 | 4 | 182.3 | 8.87 |
| 0.336 | 10 | 3 | 238.0 | 6.80 |
| 0.5 (decider-measured u=4.01) | 10 | 5 | 219.0 | 7.39 |
| 0.5 | 10 | 4 | 232.6 | 6.95 |
| 0.5 | 10 | 3 | 304.9 | 5.30 |
| 0.25 | 3 (design-supported order) | 5 | 424.8 | 3.81 |
| 0.25 | 3 | 4 | 427.9 | 3.78 |
| 0.336 | 3 | 5 | 398.1 | 4.06 |
| 0.5 | 3 | 5 | 355.6 | 4.55 |
| 0.5 | 3 | 2 | 578.9 | 2.79 |

Reading: the 10x result needs both the submitted CPU fraction and a 10x GPU bulk speed. Each corrected input alone drops it below 10x. Together with a design-supported bulk speed of 3x, the model gives 3.8x to 4.6x.

## 6. What makes 10.45x fail (refutation conditions)
1. **CPU fraction** above about 0.34 at 2-core workers (node-measured effective cores): 9.7x, and falling.
2. **GPU bulk speed** below about 8x: 8.9x at 8x in the submitted sensitivity; about 4x at the design's 3x.
3. **Slots limited by memory** to 3 or fewer with duplicated PKs: 8.5x or less at submitted fractions, 5.3x or less at corrected fractions.
4. **Oversubscription** (node: 4.6M involuntary context switches at 268% CPU): not modeled; any slowdown above about 4.5% erases the margin (PROPOSAL section 3).
5. **Baseline binary mismatch** (0.8.2 vs 0.8.3 MSM dispatch at k20): unknown direction; not modeled.
6. **Stage-specific fractions** (k17 leaves vs k20 node/decider): unmeasured; not modeled.

## 7. Recommendations (for the model, not the claim)
- Replace `cpu_fraction` with core-seconds per stage from the GNU time logs (node 2615.48 CPU-s, decider 2546.75 CPU-s, both including PK setup; split setup from prove before using them).
- Charge CPU phases to a shared 20-core pool (rayon and blst share it), not to independent workers.
- Add per-slot byte size (shared PK + pause + finalise transient) and a host-memory budget.
- Use bulk speeds from measured GPU kernels, not 10x. The design's own range is about 1.6x to 3x.
- Add duration variance (e.g. ±10% lognormal per phase) to remove the lock-step artifact.
- Measure one real serial batch and one real pipelined batch on the same binary before any claim. The model does not supply a measurement.

## Caveats
- The sensitivity grid runs the submitted `simulate` function as written. The "corrected" fractions are my arithmetic from the measured effective cores, not an independent measurement of CPU fractions per stage.
- Leaves' CPU time is not located in any log, so the CPU fractions for k17 leaves are unknown.
- No GPU or prover benchmark was run; the design's bulk-speed ranges are cited from LOKI-DESIGN and PROPOSAL and not re-measured.
- The Arc-shared PK memory figure is an ESTIMATE (astra-3-4 section 5).
