# s4-lookups — first three experiments

**SOURCE — Status:** This is an experiment specification, not a report of prover benchmarks. Only the small algebra/row model has run. Reproduce that completed check now with:

```bash
python3 clusters/s4-lookups/wave-1/check_models.py
```

**SOURCE — Inputs:** Pinned source and recorded baselines are documented in [FINDINGS.md](FINDINGS.md). Evidence paths below are relative to the research root. The target host supplied in the brief is 20 cores, 90 GB RAM, RTX5090 32 GB, CUDA13.x; hardware/toolchain availability must be recorded by the experiment runner rather than assumed from this brief (`raw/s4-lookups/stage-catalog.md` and the owner brief).

**OPEN QUESTION — Required implementation before running E1/E2:** Build a small release-mode `lookup-lab` harness against the exact target's dependency lockfile. The CLI below is a **proposed interface**, not an existing executable. Wrap the actual `commit_permuted` and `commit_product` internals in the selected proofs version rather than benchmarking a Python reimplementation. The existing 0.8.3 `bench-internal` module provides phase entry points, but its whole `Commit lookup permuted`/`Commit lookup products` samples include commitments and transforms, so add the finer spans specified below. Source: `raw/s4-lookups/midnight-bench.md`.

**OPEN QUESTION — Common measurement contract:**

1. Record git revision, complete Cargo lock hash, Rust and CUDA compiler/driver versions, CPU affinity, thread count, GPU model, official SRS digest, circuit/VK hash, k, lane count, active query count and table shape. Run Sherlock and recl9 separately with their own pins. Do not compare 0.8.3 microbenchmarks to 0.8.2 receipts as a measured acceleration.
2. Keep production witness data in memory. Use public synthetic inputs for saved replay artifacts; save only timings, aggregate counts, digests and acceptance results for real circuits. Never publish witness vectors, randomness or keys containing private material.
3. Use warm, prebuilt keys for the main comparison. Report key setup/cold-start separately; benchmark one proof at a time. Use one warm-up and five alternating baseline/candidate repetitions for complete-stage/proof timing, and 20 repetitions for small isolated kernels. Report median, min/max, CPU time, peak RSS, peak device allocation and H2D/D2H bytes/times. Bootstrap or other uncertainty intervals may supplement these, but five runs do not establish a reliable p95.
4. Run heavy builds/proofs under the factory wrapper with explicit caps. The commands below are for a later authorized implementation run; this research task does not write factory traces outside its two permitted directories. On failure read the trace, record a one-line hypothesis and retry with `--hypothesis`; after two same-kind failures shrink to k12/one lane. No installations or on-chain submissions are part of these experiments.
5. For each candidate report both s4 speedup and the actual complete proof/chain delta, using `G(p,g)=1/(1−p+p/g)`. Separate stage-local MSM/NTT costs from scans so s3/s5 do not count the same savings twice. Use the original transcript order even if arithmetic runs concurrently.

## E1 — Replace sorting and field hashing with structured table IDs

**OPEN QUESTION — Decision:** Does table-index grouping improve the exact k20 lookup preparation enough to matter while preserving existing-verifier acceptance? This is the first implementation experiment because it addresses work that MSM/NTT acceleration leaves behind.

**SOURCE — Targets:** `midnight-proofs-0.8.3/src/plonk/lookup/prover.rs::{commit_permuted,permute_expression_pair}` and `midnight-circuits-7.2.4/src/field/decomposition/pow2range.rs::{configure,load_table}`. Public captures: `raw/s4-lookups/midnight-lookup-prover.md`, `midnight-pow2range.md`. Adapt to the exact recl9 pin independently.

**OPEN QUESTION — Exact steps:**

1. Instrument compression of input and table separately; input sorting; table counting/hash allocation; group-head assignment; leftover scatter; blinding; Lagrange→coefficient conversion; and the two commitments. Log spans by lookup argument and table-expression identity. Measure time from the existing `Committed` boundary through all lookup-product commitments, with copy-permutation product time separately identified.
2. Recognize only the fixed range-table pattern actually configured by `Pow2RangeChip`. Build a fixed `(tag,value)→ID` mapping using loaded tags and their offsets. Normalize table padding into explicit multiplicities. For each input evaluate the **original** selector/tag/value expressions, validate the tuple's membership, and count its ID. Reject a malformed candidate or fall back to the original generic implementation; never weaken the lookup relation.
3. In increasing ID order, emit each used tuple count times in A′. Put one table copy at each group's head in S′. Collect repeat-position indices, then scatter every remaining table copy, including padding duplicates, into those positions. Preserve the original `b+1` random suffix entries of A′/S′ and random-number consumption order. Keep four lookup outputs independent; share only immutable table data. Add a second candidate which compresses an identical fixed table once per theta/proof rather than once per lane.
4. Test n=2^12,2^17,2^19,2^20 and lanes=1,4. For every size test: all disabled/zero inputs; all one repeated valid value; uniform valid IDs; all distinct IDs when table size permits; skew with 90% zero; observed tag-frequency distributions with public synthetic values. Include duplicate table padding, smallest/largest valid values and invalid values exactly 2^tag. Include a generic non-range lookup to confirm fallback.
5. Validate raw and compressed multisets, A′[0]=S′[0], and `(A′[i]−S′[i])(A′[i]−A′[i−1])=0` for every usable row. Inject small-field compression collisions in the toy model; test real BLS scalar computations separately. No monotonic sorting condition should be added to the verifier.
6. Run complete proofs on public fixtures with the original VK, official SRS and transcript implementation. Verify them with the unmodified CPU verifier and the available local ledger-9 adapter. Mutated input/proof cases must fail at the appropriate final verifier; a prepare-only recursive parent can defer rejection to D. Repeat the standard existing end-to-end rejection fixtures. Do not demand baseline proof byte identity: HashMap iteration differs. With **identical already-constructed A′/S′ and blinds**, later arithmetic comparisons can and should be exact.
7. Repeat complete Sherlock P/D and recl9 N/D timings after differential checks pass. Include any new preprocessing/cache initialization on a cold run and amortize it explicitly on warm runs. Report which stages/lanes actually took the specialization.

**OPEN QUESTION — Proposed harness invocation after implementing that interface:**

```bash
S4_LAB=/home/charl/code/Moriarty-perf/wiki-llm/recursion-perf/clusters/s4-lookups/wave-1/lookup-lab
/home/charl/code/Moriarty-perf/scripts/doctorwatson heavy --id S4-E1 --max-wall 1800 -- \
  "$S4_LAB" prepare --k 12,17,19,20 --lanes 1,4 --threads 1,20 \
  --variants stock,indexed,indexed-shared-table --warmup 1 --repetitions 5 \
  --cases zero,repeated,uniform,distinct,skew,observed-tags,invalid,generic \
  --output /home/charl/code/Moriarty-perf/wiki-llm/recursion-perf/clusters/s4-lookups/wave-1/e1.jsonl
```

**ESTIMATE — Keep/stop measurement:** Promote the candidate if correctness checks pass, preparation is ≥2× faster, and full proof time improves ≥5% reproducibly. These are prioritization thresholds, not cryptographic criteria. If preparation is tiny, retain a simple safe optimization only if maintenance cost is justified and redirect effort. Any acceptance mismatch fails the candidate regardless of speed. The 39.32→3.94 s cuSnark k24/one-lookup result is a comparison of mechanisms, not an expected k20 result (`raw/s4-lookups/cusnark-permutation.md`).

## E2 — CPU versus GPU inversion and product scans

**OPEN QUESTION — Decision:** Does a resident GPU implementation beat the best 20-core CPU implementation on complete grand-product construction, including transfers and integration?

**SOURCE — Targets:** Existing denominator/numerator loops, `BatchInvert`, serial lookup `scan`, and copy-permutation `last_z` propagation. Candidate primitives: sppark `polynomial/prefix_op.cuh` with `Multiply<bls12_381::fr_t>`, `ff/batch_inversion.hpp`, and CUB scans as a control. Captures: `raw/s4-lookups/{midnight-lookup-prover,midnight-permutation-prover,ff-batch,sppark-prefix,sppark-bls381,helper2-batch-inversion,cub-scan}.md`. The sppark local inversion template alone does not parallelize an entire vector.

**OPEN QUESTION — Exact steps:**

1. Use identical public, valid A/S/A′/S′ buffers and beta/gamma for all implementations. For k=12,17,19,20 and 1/4 lanes, produce CPU reference denominators `d[i]=(A′[i]+beta)(S′[i]+gamma)` and numerators `a[i]=(A[i]+beta)(S[i]+gamma)`. Separately include arbitrary ratio vectors with zeros to test kernel arithmetic; do not label those arbitrary vectors valid lookup witnesses.
2. Implement CPU control: parallel chunks of batch inversion, preserving inverse(0)=0; parallel local exclusive products; scan chunk totals; multiply each chunk by its carry. Sweep 1,2,4,8,16,20 workers and block lengths 256,1024,4096,16384. Avoid nesting 20 lane workers inside 20 arithmetic workers. For copy permutations apply the carried `last_z` at each set boundary; for lookups start each lane at one.
3. Implement GPU A: denominator/numerator formation → batched inversion → ratio → multiplicative scan. Sweep per-thread inversion groups 8/16/32 and launch blocks 128/256/512 where legal. Benchmark CUB custom finite-field scan against sppark's multiplicative prefix implementation. Confirm whether each scan is inclusive; if it is, shift by one and inject z[0]=1 to match Midnight's exclusive recurrence.
4. For every variant compare canonical BLS scalar outputs at **every** index against CPU. Include lengths 1,31,32,33,255,256,257,2^k−b−1 and full 2^k to expose partial blocks and the last usable row. Test zero at index 0, at a chunk boundary and at the final element; test p−1, all ones and random full-width values. Preserve zero masking in inversion. Check z's usable recurrence and terminal value for nonexceptional valid lookups; preserve baseline behavior on deliberately zero denominators rather than inventing an acceptance rule.
5. Implement GPU B only after A is correct: keep input/compressed/permuted vectors resident from E1; fuse numerator/denominator operations where useful; hand z to existing commitment/NTT interfaces without a host round trip. Keep blinding randomness fresh and controlled by the original proof generator; do not reuse blinds to gain residency. Keep transcript writes on the original dependency path.
6. Record three timings: kernel arithmetic only with CUDA events; H2D+kernel+D2H with synchronization; and complete product stage including commitments/transforms. Record total allocated VRAM with real SRS/PK/NTT/MSM allocations present. Admit one heavy proof at a time and require ≤28 GiB device allocation as an initial engineering margin on the 32 GB card; revise only from measured peak data. Enforce a host RSS cap below the supplied 90 GB budget.
7. Run the same unmodified CPU/local-ledger acceptance and terminal rejection fixtures as E1. Only then measure full proof and chain latency with GPU B. Record CPU-only, CPU-optimized and GPU-optimized results side by side.

**OPEN QUESTION — Proposed invocation; not run in this wave:**

```bash
S4_LAB=/home/charl/code/Moriarty-perf/wiki-llm/recursion-perf/clusters/s4-lookups/wave-1/lookup-lab
/home/charl/code/Moriarty-perf/scripts/doctorwatson heavy --id S4-E2 --gpu --max-wall 1800 -- \
  "$S4_LAB" products --k 12,17,19,20 --lanes 1,4 --threads 1,2,4,8,16,20 \
  --variants stock,cpu-blocked,gpu-roundtrip,gpu-resident \
  --warmup 1 --kernel-repetitions 20 --stage-repetitions 5 \
  --output /home/charl/code/Moriarty-perf/wiki-llm/recursion-perf/clusters/s4-lookups/wave-1/e2.jsonl
```

**ESTIMATE — Keep/stop measurement:** Require ≥2× transfer-inclusive arithmetic gain over the **best CPU control**, plus ≥5% complete-proof improvement, to prioritize GPU integration. If the complete-stage gain vanishes after commitments/transfers, report that result and coordinate residency with s3/s5 instead of claiming the kernel ratio. One k20 scalar vector is 32 MiB; an eight-vector/four-lane scratch budget is 1 GiB before all other allocations. These are capacity estimates, not measured peaks.

## E3 — Range-tag planner and the actual k19 feasibility gate

**OPEN QUESTION — Decision:** Is there a tag plan that reduces total work without pushing advice higher, and is a k19 design feasible only when coordinated with advice-row reductions?

**SOURCE — Targets:** `compute_optimal_limb_sizes`, `Pow2RangeChip::load_table`, and full circuit cost-model/floor-planner outputs. Captures: `raw/s4-lookups/midnight-limb-planner.md`, `midnight-pow2range.md`, `prior-loki06.md`. Stock synthesis already removes unused tags; the experiment must improve on that behavior.

**OPEN QUESTION — Exact steps:**

1. Re-run the supplied small DP model and retain its JSON. Capture, during **actual full synthesis**, counts of decomposition requests by bit bound, chosen limb vector, range tag, selector and caller category; distinguish foreign arithmetic, transcript, pairing and ordinary range checks. Record highest assigned row per advice/fixed/table column, every lookup argument, equality-enabled column, rotation, constraint degree, quotient extension and unusable-row count. Do not substitute advice extent for the number of active lookup queries.
2. Reproduce the unmodified full-circuit baseline separately for Sherlock P, Sherlock D, recl9 N and recl9 D. Confirm the recorded million-row table is actually the limiting region in those builds. Preserve all subgroup, exceptional-point and no-wrap constraints.
3. Keep four lanes and run caps **19,18,17,16,15**, first at the current k20. Recompute the same-tag row planner, not just `ceil(bits/cap)`. Next enumerate equal-row decompositions and choose tag sets with a global fixed-table objective: table cost is `sum(2^tag)` for distinct loaded tags, counted once, not once per query. Use the measured bound frequencies to price extra rows. Do not add lookup lanes as an unpriced shortcut.
4. For each candidate, synthesize the entire relevant circuit and downstream decider and record `(advice_extent, every_table_extent, unusable_rows, columns, lookups, degree)`. A candidate fits k19 only when **all** required extents including unusable rows are below the actual domain capacity. A small table with 958k advice rows remains k20. Reject any candidate spilling to k21; do not spend a full proof on a known capacity failure.
5. For candidates that stay k20 but reduce synthesis work, measure that work and complete proving time; do not attribute a 2× domain gain. For any candidate reaching k19, build new keys using the official SRS, require the stock VK reader and local ledger adapter to accept, and run positive/negative terminal fixtures. Record explicitly that it is a **new VK**, unsuitable for an unchanged deployed contract. No stagenet deployment or submission occurs in this research experiment.
6. Price extra advice columns/lookup arguments and recursive verification before claiming a win. Repeat warm total proof/chain timings only for candidates that clear the capacity and compatibility checks. Separate synthesis savings, domain savings and additional recursive costs.

**OPEN QUESTION — Proposed invocation after adding the synthesis driver; not an existing command:**

```bash
S4_LAB=/home/charl/code/Moriarty-perf/wiki-llm/recursion-perf/clusters/s4-lookups/wave-1/lookup-lab
/home/charl/code/Moriarty-perf/scripts/doctorwatson heavy --id S4-E3 --max-wall 1800 -- \
  "$S4_LAB" range-plans --circuits sherlock-p,sherlock-d,recl9-n,recl9-d \
  --caps 19,18,17,16,15 --lanes 4 --plans stock,global-tags --synthesize-only \
  --output /home/charl/code/Moriarty-perf/wiki-llm/recursion-perf/clusters/s4-lookups/wave-1/e3.jsonl
```

**ESTIMATE — Keep/stop measurement:** The recorded node and decider require **>45.28% and >37.71% advice reduction**, respectively, even before unusable rows, to fit k19. Unless full synthesis clears both barriers, mark the smaller-domain route unvalidated. Cap17 retains the original decomposition row count for 206 of 255 possible positive scalar bounds in the toy recurrence, versus 143 for cap15; those are **unweighted** counts, not expected circuit savings. If actual frequency-weighted results are unfavorable, stop the cap sweep and give the advice-row owners the measured budget rather than undertaking a protocol rewrite on this evidence.

**OPEN QUESTION — Deferred protocol experiments:** Only after E1 supplies actual query density and lane costs should LogUp/GKR, cq or Lasso receive a prototype budget. First derive their full commitment/transcript/SRS and in-circuit-verifier inventories, including public-input and accumulator binding. The source abstracts and published asymptotics do not provide an exact ledger-compatible integration recipe. See `raw/s4-lookups/{helper1-logup,logup-univariate,helper1-cq,lasso,midnight-kzg-params}.md` and the failed-PDF notes in FINDINGS.
