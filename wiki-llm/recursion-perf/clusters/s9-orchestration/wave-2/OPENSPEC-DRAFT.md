# OpenSpec draft: recursion-phase-profile

**ESTIMATE — Draft status:** This document proposes a change; no OpenSpec change directory, prover patch, factory predicate, receipt or deployment was created. The proposal/tasks/spec-delta sections below can become `openspec/changes/recursion-phase-profile/{proposal.md,tasks.md,specs/recursion-phase-profile/spec.md}` during implementation. The single capability delta is `recursion-phase-profile`. All SHALL language describes proposed behavior, not existing guarantees.

## Proposal

### Why

**SOURCE — Problem:** All nine wave-1 clusters support C0, but the available receipts expose command or proving totals rather than exclusive synthesis, commitment, lookup, quotient and opening costs (`raw/s9-orchestration/w2-consensus.md`, `recorded-runs.md`, `prior-astra3-review.md`). Published 0.8.3 interleaves synthesis, advice commitments and challenges; its `compute_trace` also performs lookup/permutation products, and `multi_open` contains two commitment MSMs (`w2-083-src-plonk-prover-rs.md`, `w2-083-src-poly-kzg-mod-rs.md`). Whole-method timers without ownership would double count the work used in accelerator estimates.

**ESTIMATE — Result:** An opt-in observation path will emit reproducible 0.8.3 phase profiles for A2's A102/A80, A3's parent, A4's terminal and one-batch B3, with wall/CPU/memory/data-movement semantics and existing verification outcomes. A reducer will reconcile phases with complete command receipts and calculate falsifiable C1–C10 bounds. This change claims no proving speedup and authorizes no cryptographic substitution.

### What changes

**ESTIMATE — Implementation scope:**

- Add feature-gated explicit timers to a pinned local patch of published midnight-proofs 0.8.3, and caller timers around stdlib 2.3.5 key load/setup/prove/verify. Preserve all cryptographic features and actual resolved dependency identities. The observation feature defaults off.
- Introduce a small bounded event collector, external RSS sampler, process CPU clocks, typed byte counters, and JSON sidecar conforming to [PROFILE.schema.json](PROFILE.schema.json). Schema structural checks are supplemented by interval/resource semantic validation.
- Add a factory research runner `scripts/qa/profile-recursion.py` exposing `prepare`, `calibrate`, `one`, `reduce` and `decide`, with the exact option contract in [EXPERIMENTS.md](EXPERIMENTS.md). All build/proof runs go through `doctorwatson heavy`; only factory writers append runs.jsonl.
- Join profiles to existing run receipts via a path/hash/schema line in the existing trace. Keep invocation IDs and pinned predecessor manifests; remove “newest directory” selection only from the profiling harness.
- Export C1–C10 gates from exclusive ownership, including null/unsupported outcomes for candidate geometry, GPU service, DRAM traffic and finality that were not measured.

**ESTIMATE — Exclusions:** This change does not add a GPU kernel, new proof system, new SRS, scheduler concurrency, key cache, proof market, ledger deployment or new circuit. It does not change existing queue/finality meanings or make profiling a public developer prerequisite. Existing wallets, private inputs, keys and contract state are preserved.

### Impact and acceptance

**ESTIMATE — Touched areas during implementation:** local patched crate and feature forwarding; private observation adapter at the existing proving callers; factory QA runner/reducer; source-controlled schema and tests for accounting, identity and failure semantics. `qa/holdout/` remains evaluator-owned. Factory receipt schema stays unchanged; a new typed field would require a separate contract amendment.

**ESTIMATE — Acceptance:** Every required proof and command scope reconciles within 5%; unexplained time also stays within 5%. Paired feature-on/off trials establish a one-sided 95% upper bound of ≤1% overhead for the claimed workload/cache stratum, or explicitly report inconclusive. Stock verifier and existing installed-key checks pass on unchanged protocol/keys/official SRS; local checks do not claim chain finality. Measurement failures do not alter proof bytes/semantics and do invalidate the profile.

**OPEN QUESTION — Hardware support:** Actual PMU availability, per-phase instantaneous RSS precision, CUDA counters for a later GPU backend and real multi-batch service demand must be discovered on the host. The schema supports explicit absence. No device-memory conclusion follows from host RSS.

## Tasks

**ESTIMATE — All tasks are unimplemented proposal items. Order is dependency order; reviews are inputs rather than blockers.**

- [ ] 1. Pin archive/member checksums from `raw/s9-orchestration/w2-083-manifest.md`; record resolved lockfile, versions/features/patches, compiler, release binary and patch hashes. Verify the observation path uses 0.8.3, not mutable main or recl9's 0.8.2.
- [ ] 2. Implement default-off `recursion-phase-profile` forwarding and explicit RAII timers. Feature-disabled builds eliminate the hooks and remain usable through the ordinary public API.
- [ ] 3. Implement every phase boundary in FINDINGS: SRS/VK/PK operations, setup_pk, key-generation fixed/permutation MSM batches, per-phase witness synthesis/resolve/blinding, all commitment owners, lookup/permutation products, trash, quotient internal/external NTTs, quotient commitments, evaluations, multiopen and transcript. Include returned-error paths.
- [ ] 4. Add the bounded integer event buffer, join-complete phase hierarchy, diagnostic child events and transcript reentrancy guard. Avoid per-cell instrumentation and preserve RNG/iteration/challenge behavior.
- [ ] 5. Add all-thread process CPU accounting for isolated proofs, external RSS boundary/sampling observations, explicit byte counters and unavailable-counter reasons. Account for sampler/flush costs; record observed versus instantaneous-unknown peaks separately.
- [ ] 6. Implement schema validation and semantic interval/resource validation: containment, child interval union, no overlapping accounting owners, exact RSS-delta formula, unexplained-time ceiling, no lost events, identity linkage and local-versus-finalized state rules.
- [ ] 7. Implement proposed runner commands and immutable pristine/off/on build manifests. Register a measured build budget and use existing proof budget keys per single invocation. Honor actual runner thread settings and capture BLST/Rayon limits.
- [ ] 8. Implement artifact-manifest inputs for exact A2→A3→A4 linkage; profile B3's actual 16-transaction command including wrappers, mock, proof stages and verify. Pass installed contract/VK readback identity through A4 without initiating deployment.
- [ ] 9. Emit the sidecar reference/hash into the factory trace. Test joins against receipts from heavy/check without allowing the child to append runs.jsonl. Keep failed, capped, interrupted and inconclusive measurements.
- [ ] 10. Validate timer/CPU/RSS/bytes behavior and overhead on small/A2 fixtures first. Test nested and parallel diagnostic spans, late short-lived RSS peaks, event overflow, timer failure, unsupported counters and non-monotonic readings. Compare deterministic intermediate operations and stock verification, with malformed proof/statement controls.
- [ ] 11. Collect A3, A4 and one-batch B3 profiles and paired controls under factory budgets. Include fresh-process and sequential resident-PK second-proof strata for A102, A80, P and D, retaining all priming work in composite receipts and honoring per-proof caps. Report both command and per-proof coverage. Qualify ≤1% overhead with the prespecified paired-log upper bound; do not infer it from timer microbenchmarks alone.
- [ ] 12. Implement C1–C10 decision output with source spans, denominators, formulas, overlap exclusions and OPEN QUESTION outcomes. In particular C8 is inapplicable to the already-fixed 0.8.3 dispatch; C1/C2 and C5 cannot be confirmed from baseline fractions alone.
- [ ] 13. Run relevant Rust formatting, clippy and workspace tests through the factory's heavy policy where appropriate; run normal A2/A3/A4/B3 checks with profiling off to preserve acceptance. No commit is part of this research delivery; implementation follows the then-current factory charter.

## Single spec delta: recursion-phase-profile

### ADDED Requirements

#### Requirement: Exact workload and implementation identity

The profiling harness SHALL target published midnight-proofs 0.8.3 first and SHALL record the release archive checksum, patch checksum, Cargo.lock checksum, resolved packages/features, compiler, binary checksum, public workload manifest and proof-specific VK/SRS identities. Each proof SHALL record its circuit shape, k, extended_k when applicable, transcript implementation and ordered children. Comparisons SHALL reject mismatched workload or implementation identities unless explicitly presented as a separate version or candidate comparison.

##### Scenario: Sherlock controls share one dependency closure

- **GIVEN** A102/A80, P and D fixtures built using pinned 0.8.3 and the official SRS
- **WHEN** the harness builds pristine, observation-off and observation-on variants
- **THEN** it SHALL preserve protocol-affecting features and dependencies, bind each binary to its manifest and identify the observation patch/feature as the controlled difference.

##### Scenario: Historical recl9 profile is offered as a control

- **GIVEN** a recl9 receipt resolving midnight-proofs 0.8.2
- **WHEN** it is grouped with 0.8.3 profiles
- **THEN** the reducer SHALL reject matched-control status and MAY retain it as a labeled historical reference; it SHALL NOT transfer its phase fractions or dispatch gain to 0.8.3.

#### Requirement: Complete phase ownership

The observer SHALL separately time key load, setup_pk, witness synthesis per advice phase, rational resolution/blinding, every advice/fixed commitment batch, lookup permutation and grand products, copy-permutation products, quotient evaluation with all extended-domain NTTs, multiopen and transcript work. It SHALL also account for instance preparation, trash, vanishing, evaluations, verification and command wrappers. Fixed-key commitment work SHALL be classified under VK construction. Every nested MSM/NTT SHALL have exactly one accounting owner.

##### Scenario: Advice challenges interleave with synthesis

- **GIVEN** a circuit with more than one advice phase
- **WHEN** `parse_advices` synthesizes, resolves and commits each phase before squeezing its challenge
- **THEN** the profile SHALL preserve that sequence and SHALL NOT classify all `compute_trace` time as synthesis or move commitments across challenges.

##### Scenario: Multiopen contains commitment MSMs

- **GIVEN** `KZGCommitmentScheme::multi_open` commits f and pi
- **WHEN** the reducer exports C3 and C9 fractions
- **THEN** it SHALL provide inclusive multiopen and exclusive non-MSM components and SHALL identify the two shared MSM intervals so a composed gain cannot credit them twice.

##### Scenario: Warm invocation does not build a VK

- **GIVEN** a proof using an already read/prepared VK
- **WHEN** no fixed-column MSM batch executes
- **THEN** the profile SHALL mark that batch not applicable and SHALL NOT invent a proving-time fixed-commitment cost.

#### Requirement: Reconciled wall and CPU accounting

The observer SHALL use a monotonic wall clock. Accounting SHALL subtract child interval unions, not sums of overlapping diagnostic children. The exclusive ownership partition plus explicit unexplained bucket SHALL sum to 100% of one named proof; inclusive diagnostics SHALL NOT enter that partition. Each proof and command SHALL satisfy absolute phase-sum discrepancy ≤5% of its measured total, unexplained time ≤5%, nonnegative durations and no overlapping accounting owners. Isolated-proof CPU measurements SHALL include all worker threads; overlapping jobs SHALL NOT each claim the same process CPU delta. Unknown CPU attribution SHALL invalidate resource qualification.

##### Scenario: Parallel NTT tasks join inside one phase

- **GIVEN** several worker NTT diagnostics overlap under a joined quotient phase
- **WHEN** the reducer sums wall time
- **THEN** it SHALL account for the joined wall interval once, retain worker diagnostics separately, and include process CPU consumed by all workers once.

##### Scenario: Wrapper work exceeds the proof scope

- **GIVEN** B3 spends time compiling, generating a fixture and running mock/verification commands
- **WHEN** whole-command reconciliation is checked
- **THEN** those intervals and launch/exit gaps SHALL appear explicitly; proof-only coverage SHALL NOT stand in for whole-command coverage.

#### Requirement: Honest memory and bytes semantics

Every phase record SHALL include wall seconds, CPU core-seconds or a reason for unavailable attribution, RSS entry/exit/sampled peak/peak delta, and typed bytes fields. RSS peak delta SHALL be `max(0,sampled_peak−entry)` and SHALL be labeled a sampled lower bound. Lifetime maxrss, cgroup charged memory and device memory SHALL remain distinct. Logical copy payloads and input/output footprints SHALL NOT be labeled actual DRAM traffic. Unsupported hardware counters SHALL be null with a reason, never fabricated zero. CPU-only backend CUDA traffic MAY be zero when identified as not used. GPU-capable records SHALL retain device interval clock domains, event correlation, enqueue/completion distinction, interval union and live/reserved memory; CPU-only records SHALL explicitly mark device intervals not applicable.

##### Scenario: A short phase lies between RSS samples

- **GIVEN** a phase shorter than the sampler gap
- **WHEN** only boundary RSS readings are available
- **THEN** the record SHALL identify boundary-only quality and SHALL NOT certify an instantaneous peak or memory-safe scheduler admission from it.

##### Scenario: Hardware memory-controller counters are unavailable

- **GIVEN** proc I/O and explicit copy counters are available but hardware DRAM counters are not
- **WHEN** a phase is serialized
- **THEN** logical/storage bytes SHALL retain their distinct semantics, DRAM bytes SHALL be null, and bandwidth-based conclusions SHALL remain unsupported or explicitly estimated.

#### Requirement: Bounded and qualified observation overhead

Profiling SHALL be behind a default-off compile-time feature. The enabled path SHALL use bounded event buffers, avoid per-cell events, time its flush, and record sampler costs and dropped events. Qualification SHALL require a one-sided 95% upper bound of ≤1% enabled-versus-disabled wall overhead for each claimed workload/cache stratum, with at least five initial paired runs and prespecified analysis. Inconclusive measurements SHALL remain inconclusive. Profile overflow or timer/sampler failure SHALL invalidate the profile without changing proof semantics.

##### Scenario: Paired-run noise exceeds the acceptance margin

- **GIVEN** a paired mean overhead below 1% but an upper confidence bound above 1%
- **WHEN** the report is produced
- **THEN** it SHALL report overhead qualification as inconclusive or failed, never passed solely on the mean or empty-timer estimate.

##### Scenario: Event buffer fills

- **GIVEN** a proof exceeds the event capacity
- **WHEN** the observer cannot retain a new event
- **THEN** proving SHALL retain its normal behavior, the dropped-event count SHALL increase and the profile SHALL be invalid for coverage/acceleration claims.

#### Requirement: Immutable proof behavior and public-path independence

Instrumentation SHALL preserve PLONK/KZG, BLS12-381, the official SRS, the exact existing transcript for every layer, terminal Blake2b, commitment/challenge ordering and fresh production randomness. It SHALL NOT log witness values, scalars, challenge contents, RNG seeds, wallet secrets or unredacted environments. Public compilation/proving/deployment SHALL work without the profiling feature or tools. Existing stock-verifier and installed-key checks SHALL still pass.

##### Scenario: Profile on and off prove the same public fixture

- **GIVEN** identical public inputs, keys and protocol settings
- **WHEN** off/on variants prove the fixture
- **THEN** independent stock verification SHALL accept both and the same malformed-proof/statement controls SHALL reject; deterministic intermediate arithmetic SHALL agree where comparability is defined. Production proof-byte identity SHALL NOT be required when legitimate randomness/order differences make it inappropriate.

#### Requirement: Factory receipts and failure preservation

Heavy builds and proving SHALL run through DoctorWatson preflight and measured caps. Only `doctorwatson heavy/check` SHALL append `qa/sherlock/runs.jsonl`. The child SHALL emit a profile artifact path/hash/schema into the run trace and preserve run/invocation linkage. Failures, interrupts, caps and incomplete profiles SHALL remain visible. A retry SHALL follow the factory hypothesis rule; repeated same-kind failures SHALL shrink the reproducer.

##### Scenario: A profile proof hits its resource cap

- **GIVEN** an A3 observation run exceeds its existing factory cap
- **WHEN** heavy terminates it
- **THEN** the failure receipt and partial trace SHALL remain, no successful profile SHALL be fabricated, and the next attempt SHALL carry a trace-grounded hypothesis.

#### Requirement: Required run matrix and decision output

Qualification SHALL include process-cold and resident-PK second-proof profiles for A102, A80, P and D with priming work retained in the outer receipts. It SHALL include distinct A102 k13 and A80 k12 A2 proof records, A3 P k20, A4 D k20 and the current B3 one-16-transaction-batch command. The reducer SHALL expose one named deciding metric for each C1–C10 with denominator and contributing spans. It SHALL distinguish arithmetic feasibility from measured implementation gain, reject overlapping gain credit, and leave absent candidate/service data unresolved.

##### Scenario: Baseline profile has no fused-terminal candidate or GPU run

- **GIVEN** only qualified CPU profiles of the existing recursion topology
- **WHEN** C1, C2 and C5 are evaluated
- **THEN** the reducer MAY report necessary bounds, SHALL leave candidate time or GPU service unknown, and SHALL NOT mark a 10× candidate or scheduler gain confirmed.

##### Scenario: C8 is evaluated on BLS12-381 0.8.3

- **GIVEN** the observed dispatcher uses the corrected BLST path
- **WHEN** a 0.8.2 backport gain is requested
- **THEN** the output SHALL mark it inapplicable to this version and credit no additional acceleration.

#### Requirement: Queued and Finalized semantics

The profile SHALL distinguish enqueue/admission, active service, proof-ready, local verification, installed-ledger verification, submission, inclusion and finality timestamps. Queued work SHALL bind its ordered dependencies and public manifest. Finalized SHALL require the existing chain finality and state-readback evidence; local verification alone SHALL NOT satisfy it.

##### Scenario: A4 verifies against an installed stagenet key locally

- **GIVEN** a terminal proof accepted by the installed-key verification path but no submitted transaction
- **WHEN** its sidecar is written
- **THEN** the completion state MAY be ledger_verified, submitted/included/finalized timestamps SHALL be null and no Finalized throughput SHALL be claimed.
