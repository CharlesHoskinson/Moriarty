# s5-quotient-ntt — first experiments

**SOURCE — Status:** This wave ran only the small algebra check in E2, recorded in [coset-tiles-check.md](../../../raw/s5-quotient-ntt/coset-tiles-check.md). It fetched public sources and wrote research artifacts. No GPU build, real proof, key change or ledger call ran. Host specifications are owner-supplied in [task-context.md](../../../raw/s5-quotient-ntt/task-context.md).

**OPEN QUESTION — Execution prerequisites:** E1 and E3 require an instrumented copy of the actual pinned Sherlock/Fenrir prover and existing approved fixtures; no such executable/fixture command was supplied in this research folder. The named hooks and measurement contract below specify what to implement, not an existing CLI. E2 supplies real upstream commands for its operator component, plus a required Midnight adapter. Keep new drivers/builds under `clusters/s5-quotient-ntt/` and evidence under `raw/s5-quotient-ntt/`. Do not install missing dependencies. In a factory checkout, route heavy builds/proofs through its `scripts/doctorwatson heavy` interface after inspecting that checkout's help; do not invent a wrapper syntax or run it from this research-only directory.

**ESTIMATE — Common experimental policy:** One proof job/GPU at a time; cap host compute to 20 cores, build parallelism to 8, RSS to 72 GiB (leaving roughly 18 GB from the supplied 90 GB), and GPU allocations to `min(24 GiB, 0.8×currently_free_bytes)`. These are conservative starting budgets, not measured required memory. Start with 5 timed warm repeats after one warm-up and 3 cold process starts; increase samples only when variance prevents a decision. Report completed samples, median, range, failures and timeouts, never a timeout as a finished baseline. Use a monotonic host clock around the entire function and CUDA events only for subordinate kernel measurements. Every result file gets the required title/Source/Collected/Published header. Never log private witness values; compare them in memory and output counts/status only.

## E1 — Determine whether s5 and degree reduction can matter

**OPEN QUESTION — Hypothesis:** P/D/N are spending enough time inside the actual quotient transition to justify a resident backend, and at least one circuit may sit just above a degree threshold.

**SOURCE — Instrumentation anchors:** [0.8.3 domain source](../../../raw/s5-quotient-ntt/midnight-083-domain.md) provides `coeff_to_extended`, `divide_by_vanishing_poly`, `extended_to_coeff`, `k()` and `extended_k()`. [0.8.3 evaluation](../../../raw/s5-quotient-ntt/midnight-083-evaluation.md) provides the numerator and argument-family loops. [Earlier pinned-hook audit](../../../raw/s5-quotient-ntt/earlier-loki-03.md) identifies `compute_nu_poly`, `Evaluator::evaluate_numerator` and `Committed::construct`; confirm these in the actual dependency version before patching. [Constraint degree](../../../raw/s5-quotient-ntt/midnight-083-circuit.md) and [lookup degree](../../../raw/s5-quotient-ntt/midnight-083-lookup.md) specify the maxima to report.

**OPEN QUESTION — Exact steps:**

1. Record the real dependency lock hash, executable hash, feature flags, circuit/VK identifier, official SRS identifier, k, and terminal transcript configuration. Keep Sherlock and Fenrir records separate; do not assume 0.8.2 and 0.8.3 are interchangeable. Record `nvidia-smi` model/driver/free-memory output and `nvcc --version`; do not dump the environment.
2. In a research copy, add monotonic spans at `LookupsArgued` and `Quotiented`; add nested spans for coefficient extension, numerator gates, permutation, each lookup family, trash, vanishing multiplication and inverse transform. Record quotient splitting/commitment separately if the state boundary includes it. Measure the complete proof and recursive chain too. Avoid double-counting nested spans and parallel child tasks.
3. Before proving, report `n`, `N`, `b=N/n`, `cs.degree()`, maximum gate degree, each argument's required degree, `minimum_degree`, counts of advice/fixed/sigma/instance/helper columns, and an allocation-lifetime inventory. Report the name/identifier of the expression setting the degree. Count operations/rotations and number of forward/inverse transforms; distinguish allocated bytes from peak concurrently live bytes.
4. Run CPU reference P, D and N individually with `RAYON_NUM_THREADS=20`; use approved in-memory fixtures. Run at least one complete Sherlock chain and one Fenrir chain to establish the critical path; time child/leaf jobs and chain makespan separately. No transaction submission is needed for this performance profile.
5. Produce one row per circuit: `k,d,b,N,columns,lookup_count,forward_NTT_count,inverse_NTT_count,T_s5,T_proof,T_chain,peak_RSS,peak_VRAM,cold_or_warm,status`. Include actual state-boundary semantics. Compute `f=T_s5/T_proof` only from matching runs; for chain forecasts use the measured schedule rather than summed parallel leaf times.
6. Evaluate a degree candidate without changing the circuit: calculate its proposed d/b/k/column counts. If the degree-setting constraint is still ≥6, record why. If existing lookups remain, reject a claimed b<4 for this 0.8.3 protocol. Price any k increase before proposing a circuit edit.

**ESTIMATE — Deciding measurements:** A 30.82× s5 speedup could reach 10× total only if `f≥0.930181`, even with zero added overhead. If `f≤0.6`, cap the end-to-end claim near `1/(0.4+0.6/30.82)=2.384×`. Promote degree reduction only if the actual computed b falls (e.g. 8→4) and neither k nor column growth cancels the work reduction. A 5→4 degree reduction fails the NTT-size test. E1 is successful even if it rejects the 10× claim.

## E2 — Validate exact coset tiling and choose the NTT/LDE backend

**OPEN QUESTION — Hypothesis:** Coset tiling buys useful memory headroom with exact rotations, while fully resident transforms beat host-staged segmentation when the workload fits.

**SOURCE — Immediately runnable check (already passed):** This command uses only Python's standard library and synthetic scalars. It verifies all tile entries against a direct DFT, signed/wrapped rotations and reconstruction of a known quotient over the BLS12-381 scalar modulus. It is not a benchmark.

```bash
python3 clusters/s5-quotient-ntt/wave-1/check_coset_tiles.py
```

**OPEN QUESTION — Exact GPU steps:**

1. Prepare a source snapshot of public `https://github.com/zlyber/pipelonk`, recording its revision, under `clusters/s5-quotient-ntt/wave-1/work/pipelonk/`. The captured `main` source and archived `ae` differ: use the `main` files matching [README](../../../raw/s5-quotient-ntt/pipelonk-readme.md), [lde.cu](../../../raw/s5-quotient-ntt/pipelonk-lde.md) and [CMake](../../../raw/s5-quotient-ntt/pipelonk-lde-cmake.md), and retain original source hashes. Use already available CUDA/CMake/compiler; record a blocked dependency rather than installing it.
2. Audit the emitted build command. The captured CMake adds `-O0 -g`; build an unmodified version and a separately labelled host-optimized version with that explicit `-O0` replaced by `-O3`. Do not describe this as device-optimizer improvement without inspecting the actual flags. Confirm SM120/CUDA 13 compilation; the source's H100/4090 CUDA 12.8 claim is not that check. Resolve legacy FindCUDA issues only inside the research copy and record changes.
3. Cap C++ host workers explicitly: for the local LDE adaptation use `step1_thread=20` and `step2_thread=6` at benchmark/check call sites, so the three stage pools have `3×6=18` workers. Verify concurrent worker count including helper/driver threads. `RAYON_NUM_THREADS` does not control these C++ pools. Benchmark unmodified thread tuning separately only within host caps.
4. Provide `input.bin` at the artifact root as required by the source's relative path `../../input.bin`. Use public/synthetic scalars in the artifact's verified limb/Montgomery representation, with sufficient bytes for each selected input size. Do not reuse private witness dumps. `--check` below only compares packing implementations: also compare against the independent Midnight/reference NTT described in step 6.
5. From the research copy, use these upstream interfaces; build within its `LDE/build` so the relative input path is correct. Wrap the commands with the factory heavy runner where applicable. Start at k20, not the much larger default suite.

```bash
cd clusters/s5-quotient-ntt/wave-1/work/pipelonk
cmake -S LDE -B LDE/build
cmake --build LDE/build --parallel 8
cd LDE/build
./LDE --check 20 2
./LDE --check 20 3
./LDE --bench-sweep-range 20 2 1 8 5
./LDE --bench-naive-sweep-range 20 2 1 8 5
./LDE --bench-sweep-range 20 3 1 8 5
./LDE --bench-naive-sweep-range 20 3 1 8 5
```

6. Implement a research adapter using the pinned Midnight scalar modulus, actual `ω_N`, `ζ`, inverse normalization and Montgomery conversion. Compare (a) padded length-N reference, (b) resident ICICLE/sppark length-N extension, (c) b length-n tiles with `s_t=ζω_N^t`, (d) Pipelonk staged LDE **after** proving its roots/coset/order mapping. For each, compare full output values against CPU, not just a forward/inverse round trip. Test n=2^12, 2^13, 2^17, 2^20, b=4/8, batches 1/4/16 and the actual live counts from E1. For small sizes use zeros, unit vectors at first/last coefficient, p−1 and deterministic random vectors; test rotations −n−1, −2, −1, 0, 1, 2, n+1. Report mismatch counts without scalar contents. The small Python script uses a derived root; it does not supply the production Midnight ABI/root mapping.
7. Time both the entire cold call (plan construction, allocation, conversion, transfer, transform, completion) and persistent-plan warm call, plus subspans. Pipelonk's current CUDA-event interval excludes workspace/index-map construction. Measure peak VRAM and actual host/device bytes. Compare radix-two/mixed-radix and fast-twiddle settings on the same data; retain natural order as the correctness baseline.
8. For coset tiles, measure extra shift-power generation/scaling and scatter explicitly. Do not use period-three ζ scaling for general `s_t`. Keep all b output tiles, then the same final inverse NTT; there is no sound shortcut that simply drops cosets.

**ESTIMATE — Deciding measurements:** Require zero mismatches in all tested forward outputs and exact inverse coefficients. Prefer a configuration only if full-call median improves ≥10% with repeat ranges showing a stable advantage, or it enables a previously over-budget workload within 24 GiB with acceptable latency. For the hypothetical 128-column b=8 case, verify the predicted `32 GiB→4 GiB` evaluation-column reduction; measure coefficient/output/workspace bytes separately. If memory already fits and tiling adds ≥10% time, keep full-domain residency. No ratio to 1.13 ms alone decides the winner.

## E3 — Whole-quotient ablation and acceptance preservation

**OPEN QUESTION — Hypothesis:** The improvement comes from residency and bounded graph execution, rather than an isolated fast transform or a changed proof relation.

**OPEN QUESTION — Exact steps:**

1. Using E1's pinned circuit and E2's validated backend, implement three research variants at the same `compute_nu_poly`/quotient boundary: **A** CPU reference; **B** GPU NTT with host numerator materialization; **C** resident quotient evaluator including gates, permutation, every lookup, trash, vanishing multiplication and inverse NTT. Add **C-tiled** only if E2 justifies it. Keep the original CPU challenge derivation, random blinding, quotient splitting/commitment and proof serialization.
2. Start C with a generic opcode interpreter and last-use temporary allocation. Then specialize only measured hot partitions; compare partition sizes 32/64/128 arithmetic operations, reporting registers/thread, local-memory spills, compile time and peak live buffers. Preserve constraint-family and y-Horner accumulation order. Fuse the b-periodic vanishing multiply into the final numerator store only after standalone equality passes. Do not regenerate keys for A/B/C.
3. Compare in memory after gates, permutation, each lookup family, trash, vanishing division and inverse NTT. Require equality of all evaluations and unblinded quotient coefficients. Test large/negative rotations, boundary rows, multiple permutation chunks and empty optional families. Run CPU and GPU with the same deterministic test-only randomness schedule; compare transcript bytes/challenges and proofs where reproducibility permits. Production randomness stays unchanged.
4. Run local unchanged native verification with the same VK/SRS/public inputs. Then test accelerated A102/A80 inside P, accelerated P inside D, and Fenrir leaf/N/D verification, including accumulator/pairing checks. Negative controls: mutate a quotient coefficient or resulting commitment, a public input and a transcript message; require rejection or differential failure at the corresponding boundary. Treat byte equality as additional evidence, not a substitute for acceptance.
5. Apply common cold/warm sampling to A/B/C and C-tiled. Compare spans on the same host and fixtures. Hold MSM caching and proof concurrency fixed. Report `T_s5`, complete proof latency, chain makespan, transfers, peak RSS/VRAM, kernel versus wall time, and all failure counts. Benchmark four-leaf throughput separately; parents cannot consume absent child proofs and terminal transcript challenges cannot be speculated.
6. Return the measured ratios and successful local acceptance artifacts to the owner. The final ledger-9/StrawAnchor stagenet replay belongs to the authorized integration run using its preserved state and official SRS; **do not contact stagenet during this public-web-only research task**. Until that replay succeeds, report ledger acceptance as untested.

**ESTIMATE — Deciding measurements:** Promote C only with exact differential success, unchanged local verification and ≥20% complete-stage improvement over B without exceeding budgets. A target 8× CPU-stage improvement is met only if `T_s5(A)/T_s5(C)≥8`, including all transfers and initialization for the declared cold/warm mode. Claim 10× end to end only from `T_chain(A)/T_chain(C)≥10` on completed comparable runs and after the required acceptance checks; kernel or throughput ratios do not qualify. If C misses, use its profile to decide whether to optimize arithmetic, graph partitions or other stages.
