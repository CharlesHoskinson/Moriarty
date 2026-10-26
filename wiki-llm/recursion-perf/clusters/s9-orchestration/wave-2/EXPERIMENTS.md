# First experiments: recursion-phase-profile

**ESTIMATE — Status:** This is an implementation/run recipe, not evidence of executed proving. `scripts/qa/profile-recursion.py`, the Cargo feature and the validator below are **proposed interfaces to implement in OPENSPEC-DRAFT.md**; they do not yet exist in this research folder. No proof, GPU benchmark, wallet operation or ledger submission was run for this wave. SOURCE paths are relative to the research root.

**SOURCE — Existing interfaces:** `raw/s9-orchestration/w2-factory-contract.md` defines `scripts/doctorwatson heavy --id ID --budget KEY -- CMD...` and `check ID --hypothesis TEXT`. Existing checks are `scripts/qa/check-a2.sh`, `check-a3.sh`, `check-a4.sh`, `check-b3.sh`, captured in `w2-a2-runner.md`, `raw/s3-commit-msm/factory-a3.md`, `factory-a4.md`, and `raw/s9-orchestration/b3-command.md`. Budget keys are `a2-linked-children`, `a3-parent-k20`, `a4-terminal-d`, `b3-full-batch-local` in the current predicate definitions.

**ESTIMATE — Common setup:** All future shell snippets run from the Moriarty repository root, not this research folder. Preserve official SRS/VK identities, wallets and contract state. Use the pinned Rust 1.99 toolchain and release builds. Record Cargo.lock, full resolved package/features/patch list, source/archive/patch/binary hashes, input manifest, CPU affinity, Rayon/BLST pool sizes, actual available RAM, driver/runtime and host/device caps. Start with existing runner thread settings (A3/A4/B3 use Rayon 4); cap all CPU pools within available cores. Merely setting Rayon does not establish BLST's pool size. GPU is idle in the initial CPU profile, and GPU fields say so. Keep the 0.8.2 Fenrir workload outside this matrix.

**ESTIMATE — Proposed CLI contract:** `prepare` creates immutable pristine/off/on artifacts and a manifest, with identical locked dependencies and workload features except the instrumentation feature. `one --id ... --variant pristine|off|on --cache process-cold|prepared|resident-pk --manifest ... --out ...` runs one exact workload check, writes a phase sidecar, and prints its path/hash to the factory trace. `prepared` builds/loads the key before the timed proof scope but **keeps preparation in the outer command total**; it does not pretend this is an already persistent service. The runner does not choose the newest A2/A3 directory: `--children-manifest` and `--parent-manifest` bind the exact completed predecessor artifacts. `resident-pk --warmup-proofs 1` runs a priming proof, retains the exact prepared key/SRS in memory and then runs a fresh measured proof; both proofs remain in the outer receipt. `reduce` reads receipts/sidecars without proving. `decide` calculates C1–C10 gates and emits null for unavailable candidate/service data. Driver scripts must not write runs.jsonl themselves.

## E1 — qualify the observer and A2 before paying for k20

**ESTIMATE — Question:** Can coarse timers produce trustworthy coverage/resource accounting without changing proof semantics or exceeding 1% overhead?

1. **ESTIMATE — Implement only observation:** Patch the published 0.8.3 source using the FINDINGS map. Add feature forwarding at the caller. Implement bounded buffers, nested interval reducer, process CPU boundaries, external 50-ms RSS sampling, explicit copy/footprint counters, source identity and transcript reentrancy protection. Feature off must remove the hooks. Never log scalar/challenge contents. Add semantic checks beyond JSON Schema: time containment, union subtraction, nonnegative CPU, RSS delta formula, missing-phase classification and finality invariants.
2. **ESTIMATE — Build/prepare through heavy, then run calibration.** Proposed commands:

   ```sh
   scripts/doctorwatson heavy --id C0-BUILD --budget c0-profile-build -- \
     python3 -B scripts/qa/profile-recursion.py prepare \
       --proofs-version 0.8.3 --variants pristine,off,on \
       --out target/recursion-phase-profile/build

   python3 -B scripts/qa/profile-recursion.py calibrate \
     --manifest target/recursion-phase-profile/build/manifest.json \
     --out target/recursion-phase-profile/calibration.json
   ```

   **ESTIMATE — Build budget:** Register `c0-profile-build` as a measured build budget during implementation; existing proof budgets below stay in force. Calibration is a lightweight timer/sampler-only exercise, not a synthetic MSM/NTT benchmark. It reports N, paired-timer cost, sampler cost and flush cost. If this exercise becomes heavy, route it through a measured heavy budget too.
3. **ESTIMATE — Run A2 separately for each variant:** one invocation per heavy cap; do not wrap an unbounded repetition loop in a single proof budget. Example ON command (repeat with `--variant pristine` and `off` in distinct output paths):

   ```sh
   scripts/doctorwatson heavy --id C0-A2-ON --budget a2-linked-children -- \
     python3 -B scripts/qa/profile-recursion.py one --id A2 --variant on \
       --cache process-cold \
       --manifest target/recursion-phase-profile/build/manifest.json \
       --out target/recursion-phase-profile/a2-on-01
   ```

   **ESTIMATE — Shape obligation:** Output separate records for A102 k13 and A80 k12 and the A2 command root. Preserve linked-child ordering and independent verifier reconstruction. Retain the exact public children manifest for A3. Count synthesis phases, advice commitment batches, lookup/permutation products, quotient transforms, multiopen and transcript. Explicitly classify absent fixed-key MSMs as not applicable when the command reads a VK.
4. **ESTIMATE — Qualification:** First use small public fixtures to compare intermediate commitments/assignments under controlled in-memory test randomness and verify both outputs with the stock verifier. Exact proof-byte equality is checked only when all source iteration order and randomness are deterministic; if stock HashMap iteration causes differences, diagnose it and compare verifier acceptance, public statements and deterministic intermediate arithmetic, without changing production iteration order. Include malformed proof/public-statement negatives. No deterministic production seed enters a receipt.
5. **ESTIMATE — Accounting decision:** Fail C0 qualification if either child or the command has >5% phase-sum discrepancy, >5% unexplained time, any dropped/unclosed event, misattributed CPU, unsupported non-null traffic counter, or changed proof semantics. JSON structural validation alone is insufficient. Use sampled RSS as a lower bound, and ensure the largest child's lifetime max is not mislabeled as a process-tree simultaneous peak.
6. **ESTIMATE — Overhead decision:** Begin five paired ON/OFF invocations per cache stratum, in balanced AB/BA order. Pristine-versus-OFF is a separate equivalence/observer check. The runner's reducer computes `d_i=ln(T_on_i/T_off_i)`; report `exp(mean(d)+t_(.95,n−1)*sd(d)/sqrt(n))−1`, the prespecified one-sided 95% upper overhead bound under the paired-log model. For n=5, use t=2.1318. Report raw pairs, variance and model caveat; do not select the fastest run. Pass only if the bound is ≤.01 for both complete command and each proof scope; otherwise fail or remain inconclusive. More samples require budget headroom and a stated hypothesis. The independent cost budget is `N*c/T+r*q+F/T≤.008`; it is not a substitute for paired proof measurements.

**OPEN QUESTION — Deciding measurements:** A2 coverage error, `overhead_upper95`, CPU attribution validity and verifier outcome are not yet observed. A fast empty-timer microbenchmark cannot establish the prover's overhead.

## E2 — collect A3, A4 and one 16-tx B3 profile with matched controls

**ESTIMATE — Question:** Which phases, key operations and wrapper costs occupy the real 0.8.3 path? Start only after E1's instrumentation works; do not start a GPU adapter or scheduler first.

1. **ESTIMATE — Pin predecessor artifacts:** Run the next checks against E1's exact verified A2 manifest, then A3's exact verified parent manifest. The runner must override/replace the legacy “latest directory” discovery for this experiment, verify hashes before use, and fail on a mismatch. A4 records the installed contract/VK readback identity; no deployment or chain transaction is requested by these local commands.
2. **ESTIMATE — Initial ON runs:**

   ```sh
   scripts/doctorwatson heavy --id C0-A3-ON --budget a3-parent-k20 -- \
     python3 -B scripts/qa/profile-recursion.py one --id A3 --variant on \
       --cache process-cold \
       --manifest target/recursion-phase-profile/build/manifest.json \
       --children-manifest target/recursion-phase-profile/a2-on-01/children-manifest.json \
       --out target/recursion-phase-profile/a3-on-01

   scripts/doctorwatson heavy --id C0-A4-ON --budget a4-terminal-d -- \
     python3 -B scripts/qa/profile-recursion.py one --id A4 --variant on \
       --cache process-cold \
       --manifest target/recursion-phase-profile/build/manifest.json \
       --parent-manifest target/recursion-phase-profile/a3-on-01/parent-manifest.json \
       --out target/recursion-phase-profile/a4-on-01

   scripts/doctorwatson heavy --id C0-B3-ON --budget b3-full-batch-local -- \
     python3 -B scripts/qa/profile-recursion.py one --id B3 --variant on \
       --cache process-cold --batches 1 \
       --manifest target/recursion-phase-profile/build/manifest.json \
       --out target/recursion-phase-profile/b3-on-01
   ```

3. **ESTIMATE — B3 denominator:** One batch means 16 transactions through the **current B3 runner's actual shapes**, not a substituted four-k17-leaf Fenrir chain. Instrument preparation/tests/build and fixture work even when cached. A prebaked binary-only measurement is an additional prove-service scope and cannot replace the B3 whole-command denominator. B3 mock/leaf/parent/decider/verify subprocesses each receive an invocation ID linked to one command manifest. The observer measures launch/gap/exit intervals so internal proof sums can reconcile to the factory wall receipt.
4. **ESTIMATE — Repeat controls:** Repeat each command with OFF and five paired ON/OFF repetitions as in E1, reusing identical public input/manifests. Each B3 repetition is **one batch**, not B5 or a throughput window. Report process-cold (OS cache state observed, never called disk-cold without evidence), prepared-key proof scope and whole-command strata separately. If budgets cannot support enough repetitions, publish the complete profile with `overhead.qualification=inconclusive`; do not assert C0 is fully qualified. **Also run resident-PK controls for A2, A3 and A4:** the first proof primes the same process, the second uses the retained key with fresh randomness. Register measured composite budgets `c0-a2-resident-pair`, `c0-a3-resident-pair`, `c0-a4-resident-pair` during implementation; preserve the original per-proof time and peak-memory limits and apply the tighter host cap throughout. An insufficient cap is a failed/incomplete experiment, not permission to bypass it. Example A3 invocation:

   ```sh
   scripts/doctorwatson heavy --id C0-A3-RESIDENT-ON --budget c0-a3-resident-pair -- \
     python3 -B scripts/qa/profile-recursion.py one --id A3 --variant on \
       --cache resident-pk --warmup-proofs 1 \
       --manifest target/recursion-phase-profile/build/manifest.json \
       --children-manifest target/recursion-phase-profile/a2-on-01/children-manifest.json \
       --out target/recursion-phase-profile/a3-resident-on-01
   ```

   **ESTIMATE — Other resident commands:** Use the same options for `--id A2` with budget `c0-a2-resident-pair` and no child/parent manifest; use `--id A4` with budget `c0-a4-resident-pair` and the exact `--parent-manifest` from step 2. Give every output/run a unique ID. Repeat each resident variant pair five times initially, applying the same confidence rule separately to second-proof and complete pair durations. This is a measurement harness for sequential key retention, not a concurrent production prover service. Keep B3's initial whole-command scope one batch; no hidden extra batch is introduced there.
5. **ESTIMATE — Reduce and validate:**

   ```sh
   python3 -B scripts/qa/profile-recursion.py reduce \
     --receipts qa/sherlock/runs.jsonl \
     --profiles target/recursion-phase-profile \
     --schema wiki-llm/recursion-perf/clusters/s9-orchestration/wave-2/PROFILE.schema.json \
     --require A2,A3,A4,B3 --max-sum-error .05 --max-overhead .01 \
     --out target/recursion-phase-profile/summary.json
   ```

   **ESTIMATE — Reducer result:** Export per-shape seconds/core-seconds, percentage of the named denominator, sampled RSS entry/exit/peak/delta, logical-copy and input/output bytes, nullable DRAM/device traffic, transform dimensions, commitment batch counts, setup mode, stock-verifier outcomes and run IDs. Use the sidecar hash line in the factory trace to join receipts. Preserve every failed, capped and interrupted run; never append invented phase data to runs.jsonl. Explicitly reject mixing version 0.8.2 or different statements/keys into a matched group.

**OPEN QUESTION — Deciding measurements:** Synthesis versus MSM versus quotient share, key reconstruction cost, unresolved wrapper time, CPU core demand, memory lifetimes and actual extended_k are the needed outputs. Acceptance is coverage ≤5%, overhead upper bound ≤1%, complete required counters with explicit limitations, and existing stock/installed-key verification. These local checks do not set Finalized.

## E3 — issue the C1–C10 decision sheet, then choose one next intervention

**ESTIMATE — Question:** Which claimed bounds survive measured fractions and resource demands? This reducer experiment runs no additional proofs, makes no GPU launch and sends no ledger transaction.

```sh
python3 -B scripts/qa/profile-recursion.py decide \
  --summary target/recursion-phase-profile/summary.json \
  --consensus wiki-llm/recursion-perf/CONSENSUS.md \
  --host-cores 20 --latency-target-ratio .1 \
  --out target/recursion-phase-profile/decisions.json
```

**ESTIMATE — Exact decisions:** Calculate the single gate for each C1–C10 from FINDINGS; retain its denominator, formula and contributing span IDs. C1/C2 return OPEN QUESTION without actual replacement circuit measurements; C5 returns OPEN QUESTION without residual GPU service, legal W, memory-safe admission and any applicable finality/rate data. C8 returns inapplicable/1.00× for the observed 0.8.3 dispatch. Report C3/C9 overlap explicitly and recompute residual fractions before composing. C10 wallet sync is not applicable to a run in which no sync occurred. A baseline fraction may falsify a claimed ceiling; it never certifies an unimplemented accelerator.

**ESTIMATE — First follow-up selected by data:** If MSM dominates, benchmark a same-SRS GPU MSM service including transfers; if quotient dominates, benchmark the measured extended-domain shape; if witness dominates, measure work/span before adding workers. If C5 is still plausible, first prove the memory reservation/safe completion schedule with measured retained/scratch sizes, then test bounded W on the same host. These are subsequent implementation tasks, not extra experiments performed by this research pass.

**ESTIMATE — Failure discipline:** Read each failed heavy trace, record a one-line hypothesis, retry with `--hypothesis`, and shrink the same-kind reproducer after two failures. Run the factory stop rule during implementation. Never raise a cap merely to hide a failed 20-core/90-GB/32-GB fit. Factory T3 review/finality rules still apply to a later promotion; no review or permission wait is imposed by this research draft.
