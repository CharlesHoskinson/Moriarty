# s3-commit-msm — first three experiments

**SOURCE:** This wave did not build or benchmark software. Existing A3/A4 runner commands were inspected read-only and archived in `raw/s3-commit-msm/{factory-a3,factory-a4}.md`. The plans below require an implementation/harness phase; the instrumentation and backend-selection seam do **not** already exist because they are described here. No install, deployment, ledger submission or contact outside public web/AgentBus was performed.

**OPEN QUESTION:** Required local inputs are the approved A2 child bundle, official k20 SRS, pinned source/binary revisions, existing PK/VK and frozen public deployed-contract state. Their paths/digests must be supplied by the implementation checkout, not guessed or regenerated with test SRS. A4's current wrapper fetches live contract state and selects the newest parent directory, so it is unsuitable for a controlled frozen-input comparison as written.

**ESTIMATE (experimental protocol):** Run one GPU job at a time. Start with four CPU workers to match runner defaults, then compare 4/10/20 workers within the factory's measured cap; freeze the best permissible CPU setting for GPU comparisons. Use no private witness dumps, secret seeds or environment dumps. Record aggregate counters and public input/SRS/VK digests. Limit active device allocation to `min(24 GiB, .8*cudaMemGetInfo.free)` until a whole-prover memory trace justifies a different policy.

## E1 — establish the actual commitment budget and CPU dispatch baseline

**OPEN QUESTION:** Is remaining commitment work large enough to make 10× possible, and is the current CPU comparison accidentally using the 0.8.2 dispatch cliff?

**ESTIMATE (exact steps):**

1. Pin and hash the current binary, source revision, Cargo.lock, official SRS, public inputs and keys. Read actual `msm_specific` dispatch and log the selected implementation; do not infer it merely from a package name. Obtain five warm paired CPU/GPU runs initially only if an existing GPU route is available; otherwise collect five CPU runs and postpone the measured Nocy comparison. Record cold initialization separately. Preserve the same A2 children and same PK lifetime.
2. At `commit` and `commit_lagrange`, add monotonic begin/end spans around **the whole call**, with child spans for encoding, normalization, filtering, allocation, H2D, digit decomposition/sort, accumulation, reduction, D2H, normalization/serialization and waiting. Add phase and proof identifiers. Do not log scalar values. Count logical length and nonzero count in memory; only publish distributions for synthetic/public fixtures. At each transcript barrier record which commitments it waits for and their output order.
3. Record one row per request with fields `proof_id, role, phase, basis_digest, basis_kind, k, n, backend, begin_ns, end_ns, conversion_ns, h2d_ns, device_ms, wait_ns, d2h_ns, peak_vram_bytes, setup_ns, verify_ok`. Record whole-proof wall time separately. Nsight/CUDA-event profiling is a diagnostic run; use host monotonic wall time without tracing for final latency. Concurrent MSM intervals require interval-union/critical-path analysis, not summing inclusive times.
4. In an in-memory replay harness, probe **N = 2^19−1, 2^19, 2^19+1, 2^20**, using both dense and zero-heavy vectors. Separately vary nonzero counts through that boundary at fixed N to locate where filtering affects dispatch. Compare original dispatch and always-BLST at identical toolchain, optimization and CPU budget. Never upgrade the whole proof SDK just to obtain the latter.
5. Apply the same counters to A102 k13, A80 k12, P k20, D k20, and, in a separate cohort, four k17 leaves, N k20 and D k20. Keep total chain latency, per-proof latency and steady-state throughput distinct. When stage invocations appear in later opening rounds, tag them so s6 does not double-count them.

**SOURCE:** The existing parent test command is the one below (`raw/s3-commit-msm/factory-a3.md`). It requires frozen `MORIARTY_A2_LINK_DIR`, `MORIARTY_PARENT_SRS`, `MORIARTY_PARENT_OUTPUT_DIR` and parameter directory. For a later implementation run, invoke it through the existing factory from the repository root; this wave does not execute it:

```bash
scripts/doctorwatson heavy --id s3-msm-e1 --budget s3-msm-e1 -- \
  cargo test --release --locked -p mori-recursion --features sherlock-recursion \
  --test sherlock_parent -- --include-ignored --nocapture --test-threads=1
```

**OPEN QUESTION:** `s3-msm-e1` is a **proposed experiment ID/budget**, not a claimed registered predicate. Bind its resource budget through the factory before execution. The test needs the instrumentation in steps 2–3; running the unmodified command alone does not yield phase shares. Parent/terminal tests may synthesize witnesses internally, so record the pre-Witnessed interval separately.

**ESTIMATE (deciding measurement):** With measured current same-host baseline `T0`, measured accelerated-stage share `f`, service speed `s` from E2, and overhead `h`, calculate `Tpred=T0*(1−f+f/s+h)`. Reject an MSM-only 10× plan if `T0/Tpred<10`, including uncertainty. Against the **recorded Nocy CPU baseline only**, 20× residual MSM needs `f≥.758373` with zero new overhead. Any materially faster always-BLST CPU run replaces the old CPU denominator. If no same-workload Nocy replay exists, report the formula as an estimate and the actual measured same-host ratio separately.

## E2 — arithmetic bakeoff with real bases, including forced chunking

**OPEN QUESTION:** Which backend gives the lowest correct **complete commitment service time**, and does memory pressure expose errors missed by an isolated benchmark?

**SOURCE:** WLC's existing driver is `381_xyzz_bal/benches/msm.rs`, variable `BENCH_NPOW`, fixed `batches=16`; initialization is outside its timed loop and inputs repeat 2,048 bases. Existing command: `BENCH_NPOW=20 cargo bench --bench msm` from that subdirectory, assuming its dependencies are already available. Evidence: `raw/s3-commit-msm/{wlc-readme,wlc-bench,wlc-util}.md`. **That unmodified benchmark is only a source-baseline reproduction, not the acceptance experiment.**

**ESTIMATE (exact steps):**

1. Implement one shared harness that constructs canonical field scalars, obtains the official monomial and domain-correct Lagrange bases, and sends the **same** arrays to current BLST, sppark and each available challenger. Include Nocy's pinned MSM only if its build is already reproducible. Build native `sm_120` plus compatible PTX with the installed CUDA 13.x; use the repository's pinned Rust toolchain. Record compile failures rather than installing/changing toolchains. Pin every candidate source revision and record object/binary hashes.
2. Use small correctness lengths `0,1,2,31,32,33,255,256,257`, then throughput lengths `2^12,2^13,2^17,2^18,2^19±1,2^20`. On synthetic fixtures also test identity bases, repeated points, inverse pairs, all-zero scalars, all-one scalars, alternating 0/1, random full-field scalars, canonical `r−1`, powers near each window boundary and a valid scalar with bit 254 set. Reject noncanonical inputs at the adapter boundary. For actual SRS cases, test sparse **indexed** scalars, zero padding and random full-width blinding tails.
3. For each case compute BLST's result and an independent straightforward sum for small N. Require equal canonical compressed G1 bytes after normalization. Cross-check CPU/GPU, prepared/unprepared, single/batched and canonical/Montgomery-input modes. Test proof A followed by a different proof B reusing the same buffers, including cache eviction and changed domain/SRS keys. Distinct projective coordinate encodings are not an error if normalized points match.
4. Explicitly force the backend's chunk count to **1,2,4** using a test-only exposed configuration seam and log actual chunks; do not merely hope k20 triggers chunking. Repeat with additional allocations leaving **8,4,2 GiB free**, but only when the harness's measured working set fits. Stop cleanly on OOM. Under the initial 24-GiB cap, low-free-memory cases may need the explicit chunk seam instead of a memory hog; do not violate the cap to reproduce a published 29.9-GB allocation. Repeat the Montgomery tests with `precompute_factor={1,4}`, batch `{1,4}`, shared bases true/false, and one vs several streams.
5. For the open-ICICLE path, first normalize/convert bases once and scalars explicitly, set both `are_*_montgomery_form=false`, and treat this as the initial production-candidate path. Test the internal-conversion mode separately. A reported `SUCCESS` with a wrong point is a failure; rerun a shrunk case and preserve public/synthetic inputs. Do not interpret agreement of two GPU modes as proof of correctness without BLST.
6. Time initialization/base upload/precompute separately. For each configuration perform three untimed warm-ups and thirty measured samples, alternating backend order. Report median and empirical p95, total API wall time, device-only time, H2D/D2H and maximum VRAM. Check correctness on every output. WLC must also run with `batches=1` and actual SRS arrays; report 16-batch duration and throughput separately. For scalars originating on device, report a separate resident-input experiment, without silently dropping the host conversion cost from a host-input comparison.

**ESTIMATE (deciding measurement):** Any wrong point disqualifies that configuration. Promote a backend only if its full-call median is at least **1.20× faster** than the current correct comparator at k20 and its preliminary p95 is no worse; this is an engineering decision threshold, not a claimed speedup. Compare the required stage speed from E1 to the **measured** service speed. A 5× kernel and 1.05× service result is 1.05× for budgeting. Preserve a CPU path for smaller sizes where GPU service loses.

**SOURCE:** The Montgomery/chunking regression and explicit-conversion workaround are documented in `raw/s3-commit-msm/gnark-montgomery-bug.md`; candidate source and adapter are `open-icicle-cuda-msm.md` and `gnark-api.md`. Current-fix status remains an OPEN QUESTION. No assertion of full constant-time behavior follows from this test matrix.

## E3 — phase batching, precompute economics and unchanged-verifier proof replay

**OPEN QUESTION:** Do isolated gains survive transcript dependencies, blinding and the polynomial stages' live VRAM requirements?

**ESTIMATE (exact steps):**

1. Take E2's fastest correct backend. Compare four modes with otherwise identical inputs: sequential calls; async calls with per-job scratch; shared-base batched calls returning distinct points; and scalar-resident batched calls. Begin with a single compute stream and batch width one. Then test widths **2,4,8**, keeping batches within the exact existing transcript phase. Deliberately scramble completion order and assert absorption still follows the original commitment order.
2. Sweep `precompute_factor={1,2,4,8}` at each supported batch width. Start `c=16`, compare `c={14,16,18}` on k20, and log the actual backend-selected c/chunks if the setting is automatic or overridden. Use one GPU admission budget for MSM plus live polynomial buffers; at the measured NTT/quotient peak, repeat the winner and its nearest lower-memory configuration. Share an authenticated basis handle across P/D/N only if actual SRS and domain conventions match; allocate separate Lagrange handles for different k.
3. Measure preparation and replay for **1,16,64,256 uses** of each base table. Compute `break_even=ceil(prep_ms/(base_ms−prepared_ms))` only when the denominator is positive. Separate first-proof wall latency from amortized throughput. Do not count existing Nocy base caching as new savings. Instrument overlap rather than assuming more streams means more useful parallelism.
4. If the timeline actually identifies launch-bound gaps, capture a stable graph inside one phase after allocations and preparation; measure graph construction/instantiation separately and replay it. Compare against the same uncaptured kernels. Drop graph work when whole-proof median improvement is under **1%**. Do not capture transcript challenge-dependent input generation across barriers.
5. Run five initial paired complete proofs for P and D with frozen public inputs/keys; extend the winning configuration to twenty paired runs for a preliminary tail comparison. Use a deterministic RNG **only for synthetic differential tests**, preserving consumption/order, and compare commitment bytes plus transcript challenge sequences. If other nondeterminism prevents full proof-byte equality, identify it; unchanged-verifier acceptance remains required. Production comparisons retain fresh blinding.
6. Replay GPU-produced children→parent→terminal through the unchanged CPU proof verifier and the pinned **local ledger-9 acceptance path** with the frozen deployed public state. Also verify CPU-produced proofs as a control. Mutate proof bytes and public statements and require rejection. Include both Sherlock and Fenrir cohorts separately. No wallet access, live transaction, new key deployment or SRS substitution is part of this experiment plan.

**SOURCE:** The existing local terminal test command is below, from `raw/s3-commit-msm/factory-a4.md`. The future operator must freeze `MORIARTY_A3_PARENT_DIR`, `MORIARTY_PARENT_SRS`, `MORIARTY_DEPLOYED_CONTRACT_STATE` and `MORIARTY_TERMINAL_OUTPUT_DIR` and use an implementation checkout/budget. The wrapper's live-fetch setup is intentionally not invoked:

```bash
scripts/doctorwatson heavy --id s3-msm-e3-terminal --budget s3-msm-e3-terminal --gpu -- \
  cargo test --release --locked -p mori-recursion --features sherlock-ledger \
  --test sherlock_terminal terminal_proof_matches_the_installed_stagenet_verifier \
  -- --exact --include-ignored --nocapture --test-threads=1
```

**OPEN QUESTION:** As in E1, this ID/budget and the instrumented backend seam require implementation; this document does not claim they are registered. Fenrir's corresponding approved replay runner still needs binding to its frozen fixture. A local ledger-9 acceptance result does not create a new stagenet receipt.

**ESTIMATE (deciding measurement):** Adopt only when exact commitment/statement checks pass, local ledger-9 accepts, the measured memory cap holds, and whole-chain **wall latency** improves against the equally cached current CPU baseline. Report both warm and cold ratios. Call the 10× target met only when the measured chain ratio and its uncertainty support it; a per-MSM or per-parent ratio is insufficient. Record the measured residual budget even on failure so the witness/quotient stages can act on it.

**SOURCE:** Heavy work is to use the factory's `heavy --id ... [--budget ...] [--hypothesis ...] -- CMD` interface, inspected in the local factory contract/CLI (`raw/s3-commit-msm/factory-contract.md`). On failure, preserve trace, state a one-line hypothesis, retry with `--hypothesis`; after two like failures shrink the reproducer and obey `stop-rule`. These are later execution requirements, not operations run during this source-only wave.
