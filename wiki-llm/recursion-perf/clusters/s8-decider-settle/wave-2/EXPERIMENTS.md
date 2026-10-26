# s8 wave 2 — first three experiments

**SOURCE — Scope:** this file specifies future work; no experiment below was executed in wave 2. Historical observations and source inspection are in `FINDINGS.md`; source captures are under `raw/s8-decider-settle/`. **ESTIMATE — Run E1 and E3 before investing in E2.** E1 measures the benefit C10 can actually deliver; E3 checks whether that benefit survives operational failure. E2 is the separate changed-VK research branch. No step authorizes stagenet contact.

**ESTIMATE — Harness interface to implement, not an existing command:** the commands below use a proposed `s8-service-lab` binary with `residency`, `latency`, `rows`, `prove-fixtures`, and `wallet-replay` subcommands. Its flag contract is specified here so future runs are reproducible rather than relying on handwritten shell timing. Paths identify future fixture/manifests and output under this cluster. Implementation and fixture creation are prerequisite tasks, not completed artifacts. The harness must be offline by default, reject remote node/indexer/SRS URLs, and refuse any request whose explicit VK pin differs from its manifest.

**SOURCE — The factory's heavy-work interface is `scripts/doctorwatson heavy --id ID [--budget KEY] [--hypothesis TEXT] -- CMD…`** (`models/doctorwatson/factory/CONTRACT.md`, inspected locally). **ESTIMATE — At implementation time register these experimental IDs/budgets, run from the Moriarty repository root, and execute the commands through that wrapper.** This current research task permits writes only in the research directories; do not run the wrapper during this wave because it writes factory traces elsewhere. Future execution requires the implementation task's broader scope. Do not run the existing A4 shell wrapper as an “offline benchmark”: it fetches contract state.

## E1 — resident PK correctness, capacity and warm benefit

**OPEN QUESTION — Does C10 retain the prepared objects, fit the host budget, and improve a matched request timeline?** Historical recl9 D setup/prove are 46.9/586.4 s; these predict only up to 1.080× invocation benefit from removing setup. Historical A4 is not a prove-only baseline. Sources: `raw/s8-decider-settle/w2-private-evidence-summary.md`, `raw/s8-decider-settle/recorded-runs.md`.

**ESTIMATE — Exact procedure:**

1. Freeze a manifest for **one** family, selecting the exact parent and D VKs, checked relation serialization, full public vector/template, official k20 SRS hash, binary/Cargo.lock/features and a known honest parent fixture. Use synthetic/nonsecret witnesses for reproducibility. Capture the already-approved deployment snapshot locally; do not fetch stagenet. Reject missing pins instead of generating substitute artifacts. Save this as `clusters/s8-decider-settle/wave-2/lab/manifest.json` during implementation.
2. Add phase markers and allocator counters at: empty worker, SRS decoded, parent PK prepared, D PK prepared, warm idle, proof start/end, independent stock verify end, idle after fifth proof. Report each PK's fixed/permutation counts, polynomial lengths/capacities, `cs.degree()`, extended domain, and allocation payload. Read `/proc/<pid>/smaps_rollup`, cgroup current/peak and process high-water RSS; do not use `size_of::<MidnightPK>()` as heap size. Reset process for each cold replicate and distinguish OS page-cache warmness; do not drop host-wide caches.
3. Run five fresh-process cold repetitions and five sequential requests inside one prepared worker with distinct idempotency IDs and fresh proof randomness. Hold the exact circuit/VK/public statement constant. Time SRS decode, PK load/preparation, witness, proof, independent verify and total request, keeping startup/prewarm outside the warm-request timer but reporting it. Compare medians and all individual samples. Never report a five-sample p95 as a reliable production tail bound.
4. Start at six threads for recl9; preserve four threads if reproducing the A4 harness and label that separately. After baseline correctness/capacity passes, sweep 10,16,20 threads serially; no simultaneous workloads. Do not change thread count and backend in the same comparison. Record CPU time and memory pressure alongside wall time. Check two-key retained idle after each proof for allocator growth.
5. Optional later GPU repeat only after its backend is independently qualified: one context, one large job, ≤24 GiB measured device allocations and ≤1 GiB pinned host buffers. Measure SRS upload, other transfers, kernel durations and total; no proof-time inference from transfer bandwidth alone. Ensure GPU absence still permits CPU proving.

**ESTIMATE — Proposed exact invocations after prerequisites:**

```bash
scripts/doctorwatson heavy --id S8E1-residency -- \
  s8-service-lab residency --offline \
  --manifest wiki-llm/recursion-perf/clusters/s8-decider-settle/wave-2/lab/manifest.json \
  --keys parent,decider --warm-sequentially --threads 6 \
  --output wiki-llm/recursion-perf/clusters/s8-decider-settle/wave-2/lab/e1-residency.json

scripts/doctorwatson heavy --id S8E1-latency -- \
  s8-service-lab latency --offline \
  --manifest wiki-llm/recursion-perf/clusters/s8-decider-settle/wave-2/lab/manifest.json \
  --modes cold-process,warm-process --repetitions 5 --threads 6 \
  --independent-ledger-verification --fresh-randomness \
  --output wiki-llm/recursion-perf/clusters/s8-decider-settle/wave-2/lab/e1-latency.json
```

**ESTIMATE — Decision measurements:** prepared P and D must each fit the 16 GiB reservation, normal worker peak stay ≤56 GiB with no pressure-induced latency cliff, and the configured hard 60 GiB limit must remain an emergency guard rather than the operating point. Warm-path PK-preparation and SRS-decode counters must be zero, all returned proofs must verify under the same full VK identity, and retained allocation must plateau over requests. Gain is `median cold total / median warm total`, reported separately from prove-only. If readiness cannot fit, choose one resident family/key at a time or revise the measured budget; do not add workers or unbounded swap. If prove-only is unchanged, that supports the model, not a failed optimization.

**ESTIMATE — Failure/retry discipline:** read the failing phase trace and write one line such as “Hypothesis: retained allocator arenas, not live PK arrays, exceed the 56 GiB budget.” Retry through `--hypothesis` with the smallest phase that reproduces it; after two same-kind failures shrink to one key or a smaller synthetic relation. Keep keys and witness values out of the trace.

## E2 — settle k19 with a complete row and acceptance census

**OPEN QUESTION — Can a changed but complete relation fit k19 and improve real proof time?** The unchanged recl9 needs ≥317,401 fewer advice rows assuming seven unusable rows. The historical pairing-only count is not a current D census. Sources: `raw/s8-decider-settle/w2-private-evidence-summary.md`, `raw/s8-decider-settle/stdlib-2.3.5-source.md`.

**ESTIMATE — Exact procedure:**

1. Freeze the same family/parent fixture as E1. Instrument row offsets/regions without changing constraints at: parent proof parsing and algebraic verification; in-circuit Poseidon challenges; fold; fixed-base resolve/collapse; canonical/subgroup assignments; pairing residue; statement/public-input and communication binding. Report high-water row indices and overlapping/shared regions honestly; do not add regional deltas when the layout overlaps. Keep a whole-circuit total as the authoritative figure.
2. Run explicit full synthesis/cost census at k20 then k19 with four range columns. Record advice/table maxima, used lookup tags, range width, unusable rows, public count, fixed/advice/permutation columns, degree and extended domain. A k19 capacity failure is a result; it does not authorize silently selecting k20. The baseline should fail k19, but collect the actual trace rather than treating the prior split as new measurement.
3. Only after that baseline, test one specialization at a time: fixed VK-base MSM windowing / resolve-collapse simplification, or one sound stock-gate foreign-field operation. Preserve all scalars, challenge widths/order, subgroup and identity checks, communication opening and final pairing. Do not substitute an unchecked digest or assume “smaller transcript” means dropping verifier equations. Track whether additional columns/degree cancel the smaller domain.
4. A candidate progresses only if `max(advice_rows,table_rows) ≤ 2^19−unusable_rows`, with all declared architecture/shape constraints checked. Then create **new local** candidate keys and run honest and negative fixtures under MockProver and real proof verification. Cover invalid carried accumulator, noncanonical encoding, non-unit side scalar, wrong parent/VK, flipped proof byte, swapped children, wrong state/height/order, each public-input slot mutation, and wrong communication opening. Reject malformed relation serialization and trailing bytes too.
5. Read the candidate VK through stock ledger-9 and verify with its real proof-verifying feature enabled. Compare full proof generation against the k20 baseline with identical host/thread policy, five runs after prewarm. Keep candidate artifacts separate: proving with the old installed VK must not be claimed or attempted. No deployment, contract modification or stagenet submission in this experiment.

**ESTIMATE — Proposed invocations:**

```bash
scripts/doctorwatson heavy --id S8E2-rows -- \
  s8-service-lab rows --offline \
  --manifest wiki-llm/recursion-perf/clusters/s8-decider-settle/wave-2/lab/manifest.json \
  --k 20,19 --range-columns 4 --component-census --fail-on-auto-resize \
  --output wiki-llm/recursion-perf/clusters/s8-decider-settle/wave-2/lab/e2-rows.json

scripts/doctorwatson heavy --id S8E2-candidate -- \
  s8-service-lab prove-fixtures --offline \
  --manifest wiki-llm/recursion-perf/clusters/s8-decider-settle/wave-2/lab/candidate-k19.json \
  --fixture-suite complete-decider --independent-ledger-verification \
  --threads 6 --repetitions 5 --fresh-randomness \
  --output wiki-llm/recursion-perf/clusters/s8-decider-settle/wave-2/lab/e2-candidate.json
```

**ESTIMATE — Decision:** retain k20 unless a complete candidate fits both row limits, rejects every negative fixture, is stock-readable/verifiable and improves measured warm proof latency without exceeding host budgets. The `19/40` N log N ratio is only a planning comparison; measure all other work and any change in width/degree. A fitting gadget alone is not a fitting D. If no candidate clears the row gate, stop before costly real proving.

## E3 — offline wallet/service fault and identity qualification

**OPEN QUESTION — Does the service preserve exact key identity and one-spend semantics through crashes, stale state and expiry?** Current docs' synchronization and reservation rules motivate these cases, but cannot establish rc.8 compatibility. Sources: `raw/s8-decider-settle/w2-manage-dust.md`, `raw/s8-decider-settle/w2-node-wallet.md`, `raw/s8-decider-settle/local-terminal-test.md`.

**ESTIMATE — Exact procedure:**

1. Pin the deployed wallet/proof-server/ledger package versions. Implement local fake indexer/node/proof-server endpoints with deterministic event replay and a virtual clock; no upstream forwarding. Use synthetic wallet keys/UTXOs and synthetic receipts, clearly labeled as fixtures. If an official compatible local-network harness is available, run the same cases against it after fake-endpoint tests. None of these timings is a stagenet measurement.
2. Start one wallet owner and restore each valid sub-wallet snapshot; intentionally lag the DUST stream and then the indexer head. Assert admission remains closed even if another stream is synced. Apply the missing events, advance head freshness, and observe readiness. Supply snapshots for another network/address, a partial write and an unsupported SDK schema; all must refuse readiness without overwriting the recoverable original.
3. Submit two authenticated synthetic jobs competing for the same fee input. Assert at most one reservation/spend and durable idempotent handling. Repeat with disjoint fee coins but the same contract predecessor: contract ordering still prevents invalid parallel settlement. Reuse an idempotency key with changed binding/VK/public data: reject as a conflict.
4. Inject crashes after reservation, after recipe construction, after proof, immediately before send, and after send but before acknowledgment. On restart, assert reconciliation precedes new spending. Simulate delayed success then rejection, expiry and indexer disconnect; do not release an ambiguous transaction's inputs solely because a lease/timer elapsed. An acknowledged submission must not enter Finalized before matching finality and effects evidence.
5. Advance virtual time so remaining TTL is below queue+proof+wallet+submission allowance; reject/rebuild before proof admission. Change transaction binding during rebuild: old D proof/result must be unusable. Simulate clock ahead of chain and DUST validity-window failure. Record actual parameters from the pinned fixture rather than using documentation's mainnet example.
6. Exercise caller/VK matrix: unauthorized UID; approved UID/wrong contract; missing expected VK; wrong expected VK; cached PK from another parent; altered SRS/architecture/template; corrupt result envelope; valid proof for different public vector; generation change while queued or proving. Assert refusal before proof dispatch when possible, otherwise stale-result quarantine before signing. Correct jobs still need independent stock verification; mock transport success does not waive it.

**ESTIMATE — Proposed invocation:**

```bash
scripts/doctorwatson heavy --id S8E3-replay -- \
  s8-service-lab wallet-replay --offline --virtual-clock \
  --fixtures wiki-llm/recursion-perf/clusters/s8-decider-settle/wave-2/lab/wallet-fixtures \
  --cases all --external-network deny --crash-at-every-durable-boundary \
  --output wiki-llm/recursion-perf/clusters/s8-decider-settle/wave-2/lab/e3-replay.json
```

**ESTIMATE — Decision:** zero wrong-VK dispatches, zero unverified Decided events, zero double-owned fee inputs, zero premature Finalized events, and deterministic recovery of each journaled attempt. Record dispatch count, proof-server request count, readiness delay, reserved inputs, deadline slack, declared/required/consumed fee fields and readback evidence. Current fee quantities and true finality latency remain OPEN QUESTION until separately authorized network measurements; this experiment validates design behavior only.
