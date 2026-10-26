# s6-opening — first three experiments

**SOURCE — Status:** only the small algebra experiment has run; its output is [raw/s6-opening/algebra-check.md](../../../raw/s6-opening/algebra-check.md). No CUDA kernel, Rust proof, recursive row count or ledger transaction was benchmarked. The target hardware is owner-provided: 20 cores, 90 GB RAM, RTX5090 32 GB, CUDA13.x. [Baseline](../../../raw/s6-opening/task-baseline.md).

**ESTIMATE — Execution scope:** the steps below are an implementation-phase protocol. A benchmark adapter is required and is **not an existing executable** in this research delivery. The exact CLI below specifies what that adapter must implement; it is not a claim that an existing repository command accepts these flags. Keep its source and results under `clusters/s6-opening/`. Running real heavy jobs later also makes factory traces outside this research directory; therefore none are run under this task's write-only-two-directories restriction. Do not change evaluator-owned holdouts.

**SOURCE — Factory wrapper syntax:** `scripts/doctorwatson heavy --id ID [--budget KEY] [--hypothesis TEXT] -- CMD…`. It runs preflight, measured caps and trace capture; retries of the same failed command/head require a hypothesis. [Contract excerpt](../../../raw/s6-opening/factory-cli.md). **ESTIMATE:** register these experiment IDs/budgets in the implementation phase before the commands, honor measured caps, and shrink to one small case after two failures of the same kind. Do not bypass preflight or grow caps to force a desired result.

## Common adapter and measurement contract

**ESTIMATE — Required adapter:** implement `clusters/s6-opening/wave-1/bin/s6-opening-bench` against the actual resolved proving dependency, preserving the owner's existing fixture generators and terminal verifier. It must expose `profile`, `opening`, and `pcs-sweep` with the arguments below. These are explicit future implementation requirements, not code added by this research.

1. **ESTIMATE:** resolve and record Cargo.lock hashes, dependency paths/revisions/features, local arithmetic patches, Rust compiler, CUDA toolkit/driver and GPU model. Record circuit/PK/VK hashes and official SRS provenance/digest/degree; separate 0.8.2 and 0.8.3 cohorts until verified equivalent. Do not download alternate ceremonies or call unsafe setup for acceptance fixtures.
2. **ESTIMATE:** use public synthetic fixture inputs and seeded test randomness `0..9`; never serialize private witnesses into profiling output. Seeds are for reproducibility only, not a production RNG policy. Use identical inputs and randomness schedules across arithmetic variants. Keep production challenge widths, transcript choices, public inputs and blinding unchanged.
3. **ESTIMATE:** case names `sherlock-a102`, `sherlock-a80`, `sherlock-parent`, `sherlock-decider`, `fenrir-leaf`, `fenrir-node`, `fenrir-decider` map to the supplied k13/k12/k20/k20/k17/k20/k20 workloads. Add `sherlock-chain` and `fenrir-chain` to measure their actual dependencies and scheduling. For Fenrir's full chain generate four distinct valid leaf instances, not four references to one proof.
4. **ESTIMATE:** set `RAYON_NUM_THREADS=20`, one admitted heavy process, one GPU. Pin the same available 20 CPUs if host policy permits and report the affinity. Use one untimed warmup, then ten timed warm repetitions; collect three fresh-process cold repetitions separately. Cold means empty process-level PK/SRS/device caches, **not** forcibly dropping the host's system page cache. Randomize the order of comparable variants. Stop if factory caps are exceeded.
5. **ESTIMATE:** output JSONL, one record per sample, containing `case, cohort, variant, seed, k, pcs, srs_digest, vk_digest, cache_mode, total_ms, s6_ms, phase_ms, peak_rss_bytes, peak_vram_bytes, h2d_bytes, d2h_bytes, query_count, point_set_sizes, msm_lengths, proof_bytes_equal, transcript_equal, reference_verify, exit`. Add active row/column/domain counts for E3. Use a monotonic CPU timer; synchronize GPU work at timer end. Capture CUDA event times separately from wall time. Report medians, nearest-rank p95, all raw samples and failures; ten samples do not establish a stable tail-latency distribution.
6. **ESTIMATE:** for fixed-circuit arithmetic, require exact canonical commitments, scalar evaluation messages, squeezed challenges and full proof bytes under fixed RNG. Also verify using a separately built unmodified reference verifier. For changed inner PCS, byte equality is not expected: require the same intended statement, honest acceptance and rejection of modified inputs, child commitments, evaluation claims, accumulator bindings, wrong VK/SRS, malformed/truncated proofs and trailing bytes where the interface forbids them.
7. **ESTIMATE:** report both proving-chain completion and eventual prove-to-ledger/settle latency. A stagenet acceptance run is a later execution step through the existing owner-approved path; this research contacts no ledger endpoint. Never declare ledger-9 acceptance from native verification alone. If an existing contract VK cannot change, E3 is exploratory and cannot become an acceptance-preserving deployment.

## E1 — Measure exact s6 coverage and the cheapest byte-identical changes

**OPEN QUESTION:** Is the stage large enough to merit a GPU project, and does the redundant final evaluation matter at all after Rayon and BLST optimizations?

**SOURCE — Exact anchors:** `KZGCommitmentScheme::multi_open` and `multi_prepare` in [0.8.3 KZG](../../../raw/s6-opening/midnight-kzg-0.8.3.md); `kate_division`, `eval_polynomial`, `inner_product` in [arithmetic](../../../raw/s6-opening/midnight-arithmetic-0.8.3.md). [PR565 patches](../../../raw/s6-opening/pr565-files.md) provide `divide_by_roots` and `eval_interpolant` references, not a drop-in dependency upgrade.

**ESTIMATE — Exact steps:**

1. Instrument entry/exit and disjoint spans: query grouping/order; q_i combinations; point-set division; F fold; F MSM; F transcript write/x3; each q_i(x3) evaluation plus ordered writes/x4; final fold; final evaluation/subtraction; final division; π MSM; final serialization. Record calls, lengths, allocation bytes and point-set membership cardinalities without polynomial contents. Use wall-clock trace nesting to avoid double-counting Rayon worker times.
2. Run baseline cases at their supplied k values and full chains. Measure synthesis, quotient and post-opening time outside s6 too. Report `q=T_s6_on_critical_path/T_chain`, not total s6 CPU-seconds divided by elapsed chain time.
3. Variant A: replace only final `kate_division(final_poly-v,x3)` by `kate_division(final_poly.values,x3)` and remove the now-unused v evaluation. Do not remove the q_i evaluation writes before x4. Variant B: remove avoidable coefficient clones and fold into reused buffers without changing polynomial/set order. Variant C: replace repeated root division with `divide_by_roots`; record the actual vanishing coefficient sparsity. Keep the three variants separate before testing their combination.
4. Run each variant on the same seeds, assert identical transcript/proof bytes and reference verification. Include zero/constant polynomials and one-element/tied-size point sets in arithmetic-level fixtures. Any mismatch blocks the variant regardless of speed.
5. For the verifier's barycentric change, record native timing separately; a changed recursive gadget belongs in E3 because key/circuit identity may change. Do not attribute PR565's total k14 speedup to the divider.

**ESTIMATE — Exact future invocation, after implementing and registering the adapter:**

```bash
cd /home/charl/code/Moriarty-perf
export S6_DRIVER="$PWD/wiki-llm/recursion-perf/clusters/s6-opening/wave-1/bin/s6-opening-bench"
export RAYON_NUM_THREADS=20
scripts/doctorwatson heavy --id S6OPEN-E1 -- "$S6_DRIVER" profile \
  --cases sherlock-a102,sherlock-a80,sherlock-parent,sherlock-decider,fenrir-leaf,fenrir-node,fenrir-decider,sherlock-chain,fenrir-chain \
  --variants reference,no-final-eval,reuse-fold-buffers,whole-divisor \
  --seeds 0:10 --warmup 1 --warm-repeats 10 --cold-repeats 3 \
  --out wiki-llm/recursion-perf/clusters/s6-opening/wave-1/results/e1.jsonl
```

**ESTIMATE — Deciding measurement:** retain a change only after correctness passes and paired wall-time samples show a repeatable reduction larger than observed run-to-run noise. If q<.10, the stage-only infinite-speed ceiling is `<1.111×`; this refutes s6 alone as the 10× path. If a candidate saves <1% of total chain time, keep it only if its implementation/maintenance cost is correspondingly small. Send measured q and phase costs to the neighboring stage leads.

**SOURCE — Already runnable algebra check:**

```bash
cd /home/charl/code/Moriarty-perf/wiki-llm/recursion-perf
python3 clusters/s6-opening/wave-1/check_opening_algebra.py
```

**SOURCE:** this passes 432 scan/constant-elision cases, 96 evaluation-form cases and 48 sparse-divisor cases. [Run](../../../raw/s6-opening/algebra-check.md). **OPEN QUESTION:** integration and proof-byte identity remain untested.

## E2 — Exact-Midnight GPU opening, including batching and transfer costs

**OPEN QUESTION:** Can a resident implementation accelerate the whole opening stage ≥4× on this RTX5090 while retaining exact proof bytes and fitting memory? Does batching improve complete sibling latency after shared-base caches are warm?

**SOURCE — Reusable mechanism:** the Axiom scan and device reductions are pinned to `b2d4bcfd4261cc506b5412fb04fe631f028849d7`; their field specialization is BN254. [Division scan](../../../raw/s6-opening/axiom-kate-division-header.md), [launcher](../../../raw/s6-opening/axiom-kate-division.md), [multiopen kernels](../../../raw/s6-opening/axiom-multiopen-cuda.md). [ICICLE docs](../../../raw/s6-opening/ingonyama-msm-api.md) supply a shared-base batch API precedent, not host performance evidence.

**ESTIMATE — Exact steps:**

1. Start with nonsecret captured-shape arithmetic fixtures: lengths 2¹²,2¹³,2¹⁷,2²⁰ and root sets matching E1; add length 0/1 and non-power-of-two boundaries for helper functions that support them. Port the scan's affine composition to the existing BLS12-381 scalar representation. Match canonical/Montgomery conventions and infinity handling to the reference backend; no unsafe BN254 casts.
2. Differential-check every scalar quotient and canonical MSM point versus the CPU path. For n=0 explicitly use existing helper preconditions rather than silently extending an API that rejects empty inputs. Test z=0, z=1, arbitrary full-width z, sparse/dense coefficients, all-zero coefficients, leading zeros and partial GPU tiles. For sparse t4 division compare four residue scans with the one-pass CPU divisor and sequential linear divisions.
3. Implement three paths: CPU reference; GPU MSM-only with all transfer costs; GPU resident combinations+evaluations+division+MSMs. A fourth `resident-fused` variant removes redundant copies/launches. Keep the exact Midnight F→x3→evaluations→x4→π schedule. Return only commitments and required evaluation scalars to the host; the host continues the unchanged transcript.
4. Replay the actual k20 P/N/D openings. Separately batch two ragged Sherlock children and four k17 Fenrir children. Use independent transcript states and independent seeded randomness for every child. Run batch sizes 1,2,4 with precompute factors 1,2,4; zero-pad shorter coefficient vectors only when the common official SRS/basis makes that mathematically identical. Compare against independently scheduled serial calls using already-warm bases, not just cold repeated uploads.
5. Sweep admission until measured memory approaches the smaller of factory caps and the provisional targets 28 GiB VRAM/80 GiB host RSS. Track all live buffers, base/precompute tables and MSM buckets. Prefer one resident k20 opening at a time until traces establish spare memory. A GPU failure must restart/fall back before any inconsistent result reaches the transcript.
6. Optionally test evaluation-basis quotients only if E1 shows conversion or serial-division cost worth removing and evaluation buffers already exist. Use Lagrange bases derived from the official SRS, correct bit-reversal/root order, degree bounds and q(z)=p′(z) at domain collisions; for multi-root divisors use the correct removable-singularity formula. Include every extra NTT, inverse and interpolation in wall time. C-KZG's blob workload is a mechanism reference, not a target-size benchmark. [Source](../../../raw/s6-opening/ckzg-opening.md).
7. Only after primitive and complete-stage checks pass, run whole Sherlock/Fenrir pipelines. Compare fixed test RNG proof bytes, then native reference verification. Arrange eventual terminal acceptance using unchanged ledger protocol and the applicable registered VK.

**ESTIMATE — Exact future invocation:**

```bash
cd /home/charl/code/Moriarty-perf
export S6_DRIVER="$PWD/wiki-llm/recursion-perf/clusters/s6-opening/wave-1/bin/s6-opening-bench"
export RAYON_NUM_THREADS=20
scripts/doctorwatson heavy --id S6OPEN-E2-GPU -- "$S6_DRIVER" opening \
  --cases sherlock-a102,sherlock-a80,sherlock-parent,sherlock-decider,fenrir-leaf,fenrir-node,fenrir-decider \
  --variants reference,gpu-msm-only,gpu-resident,gpu-resident-fused \
  --batch-sizes 1,2,4 --precompute-factors 1,2,4 \
  --seeds 0:10 --warmup 1 --warm-repeats 10 --cold-repeats 3 \
  --vram-target-gib 28 --rss-target-gib 80 \
  --out wiki-llm/recursion-perf/clusters/s6-opening/wave-1/results/e2.jsonl
```

**ESTIMATE — Deciding measurement:** all equivalence checks pass; synchronized median resident `s6_ms` is at most CPU `s6_ms/4`, and whole-chain latency improves without a material failure/p95 regression. The 4× threshold is a project prioritization target, not a forecast. Report `T_new=T_old−T_s6+T_s6_gpu+added_nonoverlapped_overhead` alongside actual end-to-end timing; reject any discrepancy caused by excluded uploads or hidden CPU work. Retain batching only if the entire ready sibling batch completes faster with warm caches and measured memory stays within admission targets.

## E3 — Inner-only fflonk sweep and recursive-domain break-even

**OPEN QUESTION:** Does commitment packing reduce recursive rows/domains enough to outweigh extra inner proving and SRS/key cost while D still emits the exact accepted terminal PCS?

**SOURCE — Requirements:** PR565's benchmark, PR575's verifier trait and the fflonk parameter implementation are at [PR565](../../../raw/s6-opening/fflonk-pr565.md), [PR575](../../../raw/s6-opening/fflonk-pr575.md), [fflonk code](../../../raw/s6-opening/midnight-fflonk-pr565.md). `Fflonk<Kzg,1>` means t≤2; `Fflonk<Kzg,2>` means t≤4. Loading a k20 t≤4 circuit requests inner capacity k22. Neither PR proves a recursive k reduction.

**ESTIMATE — Exact steps:**

1. Freeze baseline circuits, full statements, child order, transcripts/features, VK/PK and official SRS hashes. Implement matching native and recursive PCS support in an isolated adapter. Do not merge upstream wholesale: its grouped commitment API and other changes are not established as wire-compatible with 0.8.2/0.8.3.
2. Make five configurations per topology: baseline KZG throughout; t2 children only; t4 children only; t2 children and P/N; t4 children and P/N. D's externally verified proof always remains original KZG+Blake2b. If the implementation cannot mix inner and terminal PCS without broader redesign, report that as a failed feasibility gate rather than silently switching the terminal DefaultPCS.
3. Verify official SRS capacity before keygen: a switched child needs k+1 or k+2; switched k20 P/N needs k21 or k22. Use the existing authenticated official material. Do not generate substitute parameters or assert a smaller supported degree from circuit k alone. Measure lazy residue-basis creation separately in cold/warm runs.
4. Synthesize honest and adversarial verifier circuits; record usable rows (including required blinding rows), advice/fixed columns, lookup/permutation counts and max domain. Try k20, then k19 and k18 only if constraints/row allocation fit. Report failures; never trim verifier obligations or change arity to make k fit. Include a native-only barycentric backport control and, if warranted, a separate recursive-gadget-only control so its row effects are not credited to fflonk.
5. Regenerate affected keys. Time keygen once per configuration, then the ten paired warm proving runs and three cold runs. Measure each layer and full chain, not just proof/VK bytes. Preserve public statement semantics and accumulator bindings; run the adversarial checks from the common contract. New proofs will differ, so use independent native verification plus the unchanged external terminal verifier and intended statement binding.
6. Compute `extra=L*(p−1)+K/R` and `saved=U*d` from observed timings. Report R=1 and R=100 as explicit deployment scenarios. Treat the k14 p values only as starting sensitivity inputs. If a new D VK is needed, establish whether the existing contract can register/redeploy it; actual deployment remains outside this research task.

**ESTIMATE — Exact future invocation:**

```bash
cd /home/charl/code/Moriarty-perf
export S6_DRIVER="$PWD/wiki-llm/recursion-perf/clusters/s6-opening/wave-1/bin/s6-opening-bench"
export RAYON_NUM_THREADS=20
scripts/doctorwatson heavy --id S6OPEN-E3 -- "$S6_DRIVER" pcs-sweep \
  --topologies sherlock,fenrir \
  --configs baseline,children-t2,children-t4,children-parent-t2,children-parent-t4 \
  --terminal-pcs original-kzg --terminal-transcript original-blake2b \
  --outer-k-candidates 20,19,18 --require-official-srs \
  --seeds 0:10 --warmup 1 --warm-repeats 10 --cold-repeats 3 \
  --amortization-runs 1,100 \
  --out wiki-llm/recursion-perf/clusters/s6-opening/wave-1/results/e3.jsonl
```

**ESTIMATE — Deciding measurement:** promote only a configuration that preserves the full intended statement, passes independent verification and eventual ledger acceptance, and has `saved>extra` with repeatable complete-chain improvement. A smaller VK alone fails this criterion. A measured k20→k19 drop is compelling evidence but still needs whole-pipeline timing; unchanged k/column counts plus slower inner proofs can reject fflonk even when native verification improves. If a protocol migration cannot pass an immutable existing contract VK, retain it as a future deployment option rather than claiming current acceptance.
