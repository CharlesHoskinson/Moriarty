# s9-orchestration — first three experiments

**SOURCE — Scope/status:** These are future experiment instructions, not results. This wave ran source retrieval and `bound_audit.py` only. Existing command interfaces are captured in `raw/s9-orchestration/b3-command.md`, `b5-command.md` and `factory-contract.md`. The phase logger and integrated scheduler described below are **not implemented by this research pass**. No install, proof generation, ledger contact, commit or push was performed.

**ESTIMATE — Execution rule:** Perform heavy measurements in a separately authorized implementation/measurement run through one `scripts/doctorwatson heavy` invocation per experiment. That wrapper writes its normal factory evidence outside this research folder; consequently the commands below are not executed under this pass's research-only write rule. Put the experiment driver, public metadata and reports under `clusters/s9-orchestration/wave-1/measurements/`. Keep private inputs in their existing protected location. Preserve the stock verifier, official SRS, installed key identities and existing contract/wallet state.

**SOURCE —** The factory's exact invocation is `scripts/doctorwatson heavy --id ID --budget KEY [--hypothesis TEXT] -- CMD...`, with measured caps and an exclusive heavy lock (`raw/s9-orchestration/factory-contract.md`). **ESTIMATE —** Run concurrent workers *inside* its one child; separate heavy invocations serialize. After failure, inspect the trace and retry with a one-line hypothesis; after two same-kind failures, reduce to one shape/one batch. Respect `stop-rule`. Never overwrite an existing budget or report a wide-memory run as B7 compliant.

## E1. L0 service vector, memory lifetime and denominator audit

**OPEN QUESTION — Decision:** Can the selected unchanged relation possibly sustain 10× on the available CPU/GPU/memory, and which resource rules it out first? A parent-only wall-time profile is insufficient: collect one complete dependency chain and every distinct leaf/parent/decider shape, then warm repeats.

**ESTIMATE — Exact steps:**

1. Freeze one workload manifest containing relation/circuit identity, ordered child VK digests, SRS digest, public-instance digest, source head, Cargo.lock digest, features, binary digest, driver/runtime versions, CPU affinity and actual memory limits. Label it either narrow Sherlock A102/A80→P→D, Fenrir four-k17-leaf→N→D, or current B3 rollup. Do not relabel B3 as either other workload. Use existing valid fixture/command receipts for the first two. The concrete command below profiles the **B3/B5 family only**, whose command interface is available in this checkout.
2. Add instrumentation-only spans to the pinned prover: model/shape probing; key load/build/drop; witness assignment; rational conversion; lookup sort/scan/grand products; MSM; base/extended NTT; quotient; transcript/challenge boundaries; multiopen; serialization; independent verification. The earlier snapshot identifies `create_proof`, `compute_trace`, `finalise_proof`, `parse_advices`, KZG `commit`/`commit_lagrange`, and domain conversions as candidate hooks (`raw/s9-orchestration/prior-astra3.md`). Resolve them in the actual pinned source before editing; those historical locations are not a claim that an instrumentation API exists.
3. Implement a metadata-only JSONL logger using the event contract below. Bracket CPU work on **every executing Rayon/BLST worker**, not just the submitter. Per-thread CPU-clock deltas on non-overlapping leaf spans attribute core-seconds; process/cgroup deltas reconcile totals. A process-wide CPU delta during overlapping jobs cannot attribute one job's work. Label unattributed work rather than inventing its owner.
4. Add NVTX ranges for job/phase and CUDA events on the actual streams. Record enqueue, start and completion separately. Resolve event timestamps after completion; do **not** insert `cudaDeviceSynchronize` at every boundary. Preserve all algorithmically required event waits, including host transcript dependencies; avoid adding instrumentation-only global barriers. Flush/synchronize at the final measurement boundary. Nested CPU spans need exclusive accounting; concurrent GPU intervals need union accounting and stream identity. SOURCE: `raw/s9-orchestration/nsight-systems.md`, `cuda-streams.md`.
5. Sample, every 100 ms, dedicated run-cgroup `cpu.stat`, `memory.current`, `memory.peak`, `memory.swap.current`, `memory.events`, and CPU/memory/IO pressure when available. Record allocator live/reserved bytes at every allocation/free, including pinned memory and per-shape key residency. Also collect device telemetry, but do not infer allocation peaks from sparse `nvidia-smi` samples. A shared ambient cgroup is not an attributable run cgroup. If dedicated cgroups are unavailable, use per-process CPU/PSS/RSS plus allocator counters and state the measurement limitation. SOURCE interfaces: `raw/s9-orchestration/linux-cgroup-v2.md`.
6. First prove and stock-verify one instrumented batch. If valid, run a six-batch prefix in one process **per shape**, three independent process repetitions. This exposes cold initialization and five subsequent warm jobs without pretending separate CLI processes preserve decoded keys. Preserve phase durations by batch; if six repeats do not stabilize, label the result inconclusive. Run an equivalent uninstrumented reference once in the same session; if tracing perturbs wall time by more than an initially chosen 3%, reduce trace scope and report the perturbation. The 3% threshold is an experimental choice, not a published standard.

**ESTIMATE — Proposed event contract (no witness, scalar, proof randomness or environment contents):**

```text
run_id, workload_id, batch_id, proof_shape, job_id, attempt_id,
event_kind, phase, span_id, parent_span_id, dependency_ids,
host_monotonic_ns, thread_id, thread_cpu_ns,
device_id, stream_id, gpu_event_id,
host_live_bytes, host_reserved_bytes, pinned_bytes,
device_live_bytes, device_reserved_bytes, transfer_direction, transfer_bytes,
queue_reason, verified_artifact_digest
```

**ESTIMATE — Queue reasons:** dependency, key-load, host-budget, device-budget, worker, copy-engine, GPU-dispatch, ledger-order. Keep these separate from compute spans. Emit release events when last consumers finish, not at enqueue-time free requests. Use a common monotonic host clock; correlate device clocks through events/Nsight, not by subtracting unrelated clock epochs.

**ESTIMATE — Concrete B3/B5 command recipe:** The following uses the existing `prefix-*` interface, not a proposed scheduler CLI. Before running, set absolute `S9_REFERENCE` to a successfully checked B3 evidence directory, `S9_INSTRUMENTED` to its instrumentation-only rebuilt binary, `S9_SRS` to the existing official SRS, and `S9_OUTPUT` to a new experiment directory. The instrumentation implementation must honor `S9_TRACE_PATH`; this variable is a proposed logger interface, not an existing prover option. `S9_REFERENCE/fixture/corpus`, `rollup.bin` and `build.json` are artifacts described in the B3 source. Validate their digests against their receipt first. Put this body in a future `e1-driver.sh`, then run it once through the factory wrapper.

```bash
set -euo pipefail
: "${S9_REFERENCE:?approved B3 evidence directory}"
: "${S9_INSTRUMENTED:?instrumentation-only binary with phase logger}"
: "${S9_SRS:?existing official SRS}"
: "${S9_OUTPUT:?new output directory under this cluster}"
test -x "$S9_INSTRUMENTED"
test -x "$S9_REFERENCE/rollup.bin"
test -f "$S9_REFERENCE/build.json"
test -d "$S9_REFERENCE/fixture/corpus"
test ! -e "$S9_OUTPUT"
mkdir -p "$S9_OUTPUT"
export RAYON_NUM_THREADS=4
export MALLOC_MMAP_THRESHOLD_=131072
export MALLOC_TRIM_THRESHOLD_=131072
export MALLOC_ARENA_MAX=1
for repetition in 1 2 3; do
  case_dir="$S9_OUTPUT/repetition-$repetition"
  mkdir "$case_dir"
  for stage in leaf parent decider; do
    export S9_TRACE_PATH="$case_dir/$stage.events.jsonl"
    /usr/bin/time -v -o "$case_dir/$stage.time.txt" \
      "$S9_INSTRUMENTED" "prefix-$stage" \
      "$S9_REFERENCE/fixture/corpus" 6 "$case_dir" "$S9_SRS" \
      > "$case_dir/$stage.log" 2>&1
  done
  unset S9_TRACE_PATH
  /usr/bin/time -v -o "$case_dir/verify.time.txt" \
    "$S9_REFERENCE/rollup.bin" prefix-verify \
    "$S9_REFERENCE/fixture/corpus" 6 "$case_dir" "$S9_SRS" \
    > "$case_dir/verify.log" 2>&1
done
```

```bash
# From the repository root, after creating the driver and validating inputs:
scripts/doctorwatson heavy --id S9-L0 --budget s9-l0 -- \
  bash wiki-llm/recursion-perf/clusters/s9-orchestration/wave-1/e1-driver.sh
```

**SOURCE —** Nsight's documented CUDA/NVTX command supports `--trace=cuda,nvtx`, `--sample=none`, `--cpuctxsw=none` and `-o` (`raw/s9-orchestration/nsight-systems.md`). **ESTIMATE —** For one diagnostic repetition, insert this prefix before the instrumented command, inside the same heavy invocation:

```bash
nsys profile --trace=cuda,nvtx --sample=none --cpuctxsw=none \
  -o "$case_dir/$stage.nsys" \
  "$S9_INSTRUMENTED" "prefix-$stage" \
  "$S9_REFERENCE/fixture/corpus" 6 "$case_dir" "$S9_SRS"
```

**OPEN QUESTION —** `nsys` was not found on this research session's PATH; this is not proof it is absent everywhere. Do not install it as part of this pass. The application event logger plus CUDA event/allocator counters is the fallback. Without GPU events or an implemented GPU backend, report GPU service as **unmeasured**, not zero; a CPU profile only determines the next measurements.

**ESTIMATE — Deciding measurements:** Produce a per-shape/per-phase table of exclusive wall, core-seconds, GPU compute/copy demand, retained/temporary bytes and dependencies. Reconcile accounted time against whole-process totals and show unattributed residual. Calculate `Imin=max(C/c,G,Q,P,(L+F)/W)` under measured resource caps; keep network finality unknown in local runs. If any demand exceeds `Tmatched_CPU/10`, reject the 10× scheduler hypothesis under that configuration. A necessary bound passing is permission to test scheduling, not evidence that it works.

## E2. Bounded-window pipeline versus tuned CPU, with recovery

**OPEN QUESTION — Decision:** Does overlap produce useful verified completions, at acceptable latency and memory, beyond the best CPU schedule? Is the legal batch window large enough? This is the primary experiment; remote dispatch is initially represented by synthetic delays, not public-market jobs.

**ESTIMATE — Prerequisite:** Implement one persistent experiment driver with typed per-job dependencies, shared immutable keys where supported, separate CPU/control executors, event-completed GPU work, byte admission and stock verification. It must ingest the E1 trace schema and output a configuration manifest. It is not present in this research pass. Keep the narrow Sherlock, current B3 and Fenrir workloads separate. Never bypass a live circuit's thread-safety constraints to share its mutable witness state.

**ESTIMATE — Exact steps:**

1. Start with the existing sequential CPU path and fixed workload. Benchmark CPU-only configurations `(jobs, thread-cap/job)=(1,16),(2,8),(4,4)`, then `(8,2)` only if E1 shows enough memory and a benefit worth testing. Reserve four of the supplied cores for control/verification in this sweep; also compare the best schedule using the full permitted CPU allowance so the denominator is not weakened by that choice. Pin the total allowance and verify BLST and Rayon thread counts/CPU usage; `RAYON_NUM_THREADS` alone is not a BLST cap. Keep key-residency policy and memory ceiling matched.
2. For the best CPU settings, replay exactly the same arrival list and prove workload with the GPU backend. Start W=1, then W=2,4,8, allowing only lineage-valid batches. One heavy GPU workspace, two 64-MiB transfer buffers, host/device caps as in FINDINGS. Before every admission, require a safe completion path under the remaining byte budget. If the measured legal window is smaller, stop the sweep there; do not fabricate independence.
3. Compare three policies on the same traces/work: oldest-ready downstream priority; measured earliest-finish priority; key affinity plus an oldest-batch aging deadline. Measure GPU idle intervals with reason codes. Preserve original column/child/transcript order even if arithmetic requests finish out of order. Admit a second heavy device workspace only after its *joint live peak* is measured and fits; adding two independent maximum-RSS observations is not that measurement.
4. First use a six-batch smoke test. If it passes correctness/resources and the capacity bound still allows the target, extend to at least 20 counted completions per measured interval and three independent paired CPU/GPU repetitions. Exclude a documented warm-up segment from the steady interval while also reporting complete fill/drain elapsed time. Run CPU/GPU order alternately across repetitions; use the same power/clock policy, SRS/cache state and input identities. Stop further runs when a decisive failed capacity inequality makes the target impossible.
5. Emit both fixed-backlog throughput and arrival-to-completion latency. In a second arrival replay, cap offered load below the measured stable rate and report enqueue→admit, admit→locally-verified, and total p50/p95 latency. A growing queue is not sustainable throughput. Record ordered completed prefixes, not only out-of-order finished proofs.
6. Inject a slow oldest leaf, a worker exit after artifact creation but before acknowledgement, duplicate attempt delivery, stale predecessor state, altered public statement, wrong child order and corrupted proof bytes. Verify that invalid proofs/lineage are rejected, no parent starts early, duplicate completion/submission is suppressed, and progress resumes within budget. Return cancelled buffers only after their CUDA consumer events finish. Use synthetic witnesses only; do not log witness contents.
7. Emulate finality/backpressure delays of 0, 120 and 300 seconds in the scheduler's replay model, without sleeping or contacting a ledger. Run W=1,2,4,8 to see when the available window controls throughput. These are **sensitivity scenarios**, not measured stagenet latency. Real finality will replace them in a separately authorized acceptance run.

**ESTIMATE — Deciding measurements:** Report matched `CPU_interval/GPU_interval`, first-batch and p95 latency, completed-in-order rate, total core-seconds/batch, GPU active/idle service, host live/reserved and VRAM live/reserved high-water, swap delta, rejection/retry work and finalization backlog. A candidate succeeds only if every proof is accepted by the independent stock verifier, no resource cap/OOM/swap criterion fails, queues stay bounded, and the repeated paired results meet the stated target. Include the slowest paired repetition; three runs do not justify a precise statistical confidence interval.

**ESTIMATE — 10× gate:** Use ≥10× against the best matched CPU completion interval, not merely `1617.292/I`. Also publish the latter historical ratio as a separately labeled comparison if the workload is Fenrir. A six-batch B5 prefix is not the full 12-instance B5 suite. A local verifier result is not Finalized: exact ledger-9/stagenet proof acceptance, ordered state effects and finality/readback remain separate acceptance obligations, not actions performed here.

## E3. Trace-selected GPU allocation, launch and tail-latency ablations

**OPEN QUESTION — Decision:** Is GPU orchestration overhead large enough to merit pools, graphs or partitioning after E2? Skip this experiment if the trace attributes <1% of critical-path time to these mechanisms and no material tail-latency issue exists.

**ESTIMATE — Exact steps:**

1. Replay real measured k20 arithmetic shapes and dependency boundaries using public synthetic field inputs and enrolled official bases. Use the backend already validated in E1/E2; no replacement field/curve/transcript. Record driver/runtime/device capability queries, actual memory limits and clock/power state. Warm keys once, retaining cold setup numbers separately.
2. Pair the existing allocator with a bounded stream-ordered pool: set an explicit byte retention cap within the E2 device budget; record both used and reserved current/high-water counters. Use events to join all buffer consumers before free. Test one allocation failure/cancellation boundary and ensure a completion path retains its reservation. Compare three identical repeated runs; compute removed allocation/synchronization critical-path time.
3. If launch gaps remain significant, capture/replay a stable MSM/NTT region bounded by transcript challenges. Refresh pointers, scalar inputs and outputs each proof. Compare direct launch versus replay after including first graph upload/instantiation amortized over the measured batch count. Validate deterministic arithmetic outputs and complete stock-verifier acceptance; do not require byte-identical randomized whole proofs.
4. If terminal work waits behind running bulk kernels, compare serialized dispatch, priority streams, smaller algebraically equivalent chunks, and a small green-context SM/workqueue reservation. Query the actual API capabilities first. NVIDIA's Runtime green-context example requires CUDA 13.1 or later; Driver API availability is a separate capability. Do not assume a 148-SM example matches the 5090. Measure compute, bandwidth contention, reserved bytes and terminal p95 simultaneously.
5. Re-run the winning ablation on the complete E2 workload at the same arrival rate/window. Synthetic kernel improvement without better complete-chain behavior is not a result for this stage.

**SOURCE —** Required API/benchmark context is in `raw/s9-orchestration/cuda-async-allocator.md`, `cuda-graphs.md`, `cuda-streams.md`, `cuda-green-contexts.md`. NVIDIA's graph result uses RTX 3060/Xeon/CUDA 11.8 versus 12.6; its green-context result is a tiny integer kernel beside synthetic `sqrtf` work. Neither is a recursive proof benchmark.

**ESTIMATE — Deciding measurements:** If overhead fraction p is measured, the absolute elimination bound is `S≤1/(1-p)`. Keep an ablation only if it improves the target full-chain metric beyond run-to-run noise while preserving resource/acceptance checks. For a partition, report any throughput loss even if p95 improves. If p=.05, all launch/allocation/blocking elimination combined can save at most 1.053× before interacting bottlenecks; it cannot close a several-fold gap to 10×.
