# astra-1 independent position

Lens: assurance strategy. Source base: `8fd34ed794eda7186c32495f715609ec54a9a297`. Written before reading peer positions. Recommendations below are proposals. Observations cite their sources.

## Keep

1. Keep the explicit machine, admitted `ProgramImage`, complete state snapshots, charged steps and terminal publication. Preserve the separation between technical chunks, language suspension and financial commitment. The implemented pipeline already uses these distinctions. [docs/sherlock/design.md:240-288]
2. Keep the native child → recursive parent → terminal decider route. The existing acceptance chain distinguishes cryptographic verification, local ledger validation and target readback. Reuse that separation. [qa/sherlock/predicates.yaml:12-97]
3. Keep K, Quint, the reference interpreter, bounded circuit models and independent literal oracles as different assurance tools. Existing U021 tests explicitly report `refinement='none'` and `execution='not-run'` when comparing retained F03 artifacts. That honesty is useful. [models/sherlock/qa/test_model_package.py:24-42]
4. Keep the current DoctorWatson loop, resource caps, hypothesis-based retries and evaluator-owned holdouts. Use executable predicates to close requirements. Do not restore retired adoption gates. [AGENTS.md; models/doctorwatson/factory/CONTRACT.md]
5. Keep the security requirements for code identity, complete effects, exact arithmetic, authenticated genesis and final discharge of deferred obligations. Map their IDs into the new change rather than losing them during consolidation. [openspec/changes/sherlock-universal-interpreter-design/security-requirements.md:UISEC-001–012]

## Improve

### State exactly what each result establishes

Introduce a typed assurance record per requirement and milestone. Record claim, evidence class, source closure, profile, numeric bounds, command, tool versions, result, artifact hashes and remaining assumptions. A source review, bounded model run, witness check, cryptographic proof and chain receipt establish different facts. Existing UISEC-012 already requires this separation. [openspec/changes/sherlock-universal-interpreter-design/security-requirements.md:UISEC-012]

Do not use a cached green status as release evidence. `status.view` counts the retained `passes` value. `_update` copies older entries while writing a new top-level HEAD. It does not check each entry's source closure. Add dependency fingerprints and explicit historical/current/stale states, or require a release manifest that independently validates every needed result. Reuse unchanged results only after their declared inputs match. [models/doctorwatson/factory/status.py:34-47,77-112]

### Make negative evidence classify the actual rejection

`qa_mutation::run_once` maps synthesis errors to `Rejected`. `run_suite` also increments `rejected` for panics. `assert_sound` checks nonempty sampling and no accepted mutations, but does not require zero panics. Therefore that assertion alone cannot establish constraint rejection. [crates/mori-backend/src/qa_mutation.rs:458-486,640-650; crates/mori-backend/tests/soundness_mutation.rs:227-245]

Require these distinct outcomes: accepted, constraint-rejected, input-rejected, synthesis-failed, panicked, timed-out and not-run. Only a reached constraint check can supply constraint-rejection evidence. A baseline failure invalidates the mutation campaign. Record mutation coverage by obligation and field, including inactive lanes, padding, selectors, indices, range checks and all public statement components. Supply deliberately weakened circuit controls to establish test sensitivity. This changes assurance accounting, not the mathematical status of any existing circuit. [proposal grounded in the harness above and UISEC-002,005,009]

### Join the independent representations

Specify a relation between source semantics, admitted image, host state, canonical encoding, circuit state and recursive statement. Define error priority, free publication, fuel charges and hidden helper steps in that relation. Matching only terminal values misses intermediate-state defects. An existing U021 test already checks an intermediate environment mutation despite equal final results. [models/sherlock/qa/test_model_package.py:44 onward]

For each supported rule, require both directions:

- Completeness: every valid, bounded semantic step has an admissible witness for its declared relation.
- Soundness: every satisfying witness decodes to a valid semantic step with the same public commitments.

These are proof obligations, not consequences of a finite test suite. State the domain and assumptions wherever a theorem or test claims coverage. The retained model package reports bounded Clean encoding, bounded step circuit and native integration as unimplemented tasks. A successful task-report command does not discharge those tasks. [models/sherlock/qa/model_package.py:229-277]

### Strengthen target receipt identity

A8 accepts the first `ContractCall` found at a pinned address. It does not compare that call with the candidate proof, statement or intended state change. A10 adds readback, but selects its statement by newest file modification time. The chain should carry one explicit artifact manifest from A2 through A10. [scripts/qa/check-a8.sh:7-42; scripts/qa/check-a10.sh:7-24]

Bind that manifest to the source closure, relation/profile, verifier key, public inputs, network genesis or reset identity, runtime version, contract address, transaction and finalized block. Check predecessor state and successor effects independently. Replayed old receipts and receipts from a reset chain must fail the current qualification. This is a proposed replacement for implicit artifact selection. [proposal grounded in A8/A10 and UISEC-009,011]

## Cut

- Cut aggregate pass counts as a completion argument. Retain them as navigation. `sherlock-next` reads status rather than rerunning predicates. [scripts/sherlock-next; models/doctorwatson/factory/CONTRACT.md]
- Cut any promotion from model agreement to circuit soundness, or from `ProofPreimage::check` to native proof verification. The dispatch test explicitly calls the witness checker. [crates/mori-backend/src/sherlock_machine/c6_dispatch_tests.rs:1-8,85-87]
- Cut performance choices based on estimated multiplicative speedups. The recursion consensus says its candidate estimates overlap and only phase measurement is agreed. [wiki-llm/recursion-perf/CONSENSUS.md]
- Cut giant dispatch runs as the first diagnostic. Preserve broad coverage, but first isolate one rule and the smallest state that distinguishes it. The factorial test visits a sequence of states and checks several alternate rules per state. Its last recorded run reached the wall cap without a receipt. [crates/mori-backend/src/sherlock_machine/c6_dispatch_tests.rs:89-135; docs/sherlock/HANDOFF-2026-10-26.md:19-20]
- Cut duplicate governance machinery and unrestricted new language scope from this recursion change. Retain existing product obligations as separately named work. [AGENTS.md; docs/MORIARTY-PRODUCT-CONTRACT.md]

## Implementation plan, with ordered milestones

### M0: pin the contract and evidence

Publish a source/profile/network compatibility manifest and the crosswalk to UIREQ/UISEC/ZR/MNR obligations. Add typed negative outcomes and source-aware evidence validation to the factory. Reconcile release pins against the actual ledger-9 target before new heavy proofs. Acceptance: a stale result, wrong proof version, wrong verifier key and synthetic receipt each fail their relevant check. [proposed milestone; baseline interfaces in models/doctorwatson/factory/CONTRACT.md]

### M1: establish a small complete path

Use the existing literal program and the smallest recursive example to connect source, complete semantic trace, C6 encoding, leaf proof, recursive parent, decider and independent local ledger validation. Include zero-fuel, exhaustion, rejection and terminal publication cases at their correct layer. Do not claim general interpreter qualification. Acceptance: a manifest names every traversed component and every untested obligation. [proposed milestone; existing sequence in docs/sherlock/HANDOFF-2026-10-26.md:30-40]

### M2: close the rule and representation matrix

Expand to each supported dispatch/helper rule and boundary condition. Build small fixtures for competing rule selection before factorial/tree stress runs. Compare complete pre/post state and all preserved fields. Prove or explicitly track the semantic-step and encoding obligations. Reject unsupported profiles before proving. Acceptance: every admitted native rule has a positive case, a distinguishing negative case and an evidence-class label. [proposed milestone; dispatch consumer in crates/mori-backend/src/sherlock_machine/c6_dispatch_tests.rs]

### M3: qualify recursive composition

Use linked children, the existing k20 parent and terminal decider as the baseline. Test authenticated genesis, predecessor continuity, correct key membership, step order, bounded counters and final accumulator discharge. Exercise multiple depths and a changed program under the same authorized VM relation. A wrong relation, skipped predecessor or skipped final obligation must fail. Optimize only after measurement, followed by equivalent-output and rejection checks on the changed implementation. [proposed milestone; qa/sherlock/predicates.yaml:A2-A7; UISEC-009,010; openspec/changes/recursion-phase-profile/design.md]

### M4: qualify the exact target

Run the pinned local ledger rehearsal, then stagenet and ledger-9 Preview checks with separate evidence records. Prove, submit and independently read the same manifest-selected result. Include competing successors, replay and failed-call state preservation. Obtain target runtime information directly rather than inferring it from an endpoint name. Acceptance: finality, unique consumption and effects belong to the same statement and chain instance. [proposed milestone; UISEC-011; scripts/qa/check-a8.sh; scripts/qa/check-a10.sh]

### M5: promote a named release profile

Publish coverage and open obligations for the precise supported profile. Re-run affected evidence after compiler, circuit, prover, key or ledger changes. Mainnet deployment requires its own qualified adapter and rollback procedure. Keep experimental demonstrations distinct from complete Sherlock acceptance. [proposed milestone; docs/sherlock/design.md:307-311; qa/tiers/t3.sh]

Run implementation through `scripts/sherlock-next`, `scripts/doctorwatson check <ID>`, `scripts/doctorwatson heavy` and `scripts/doctorwatson stop-rule`. Add predicates and evidence validation to these commands rather than inventing a second engine. Heavy failures retain traces and hypotheses. Two failures of one kind trigger a smaller reproducer. This council itself performs no implementation, deployment or heavy build. [AGENTS.md; BRIEF.md]

## Assurance strategy

### Evidence classes

| Class | Required content | Limit |
| --- | --- | --- |
| Source inspection | Exact path/hash, relevant definitions and assumptions | Establishes authored behavior, not execution |
| Model execution | Model/tool version, bounds, seed or exhaustive state-space claim, reached actions and trace | Covers that model instance only |
| Differential execution | Independent oracle provenance, complete ordered observations, projection and first divergence | Finite correspondence, unless a theorem proves more |
| Constraint check | Valid baseline, mutation location, reached verifier stage, concrete failure | No cryptographic or chain acceptance claim |
| Native cryptography | Exact proof/VK/SRS/statement/backend identities, independent verification, invalid controls | No current ledger entitlement |
| Target observation | Chain instance/runtime, manifest, call, finalized block, pre/post state and effects | Applies only to that target and operation |
| Formal theorem | Statement, premises, supported domain and independently checkable proof artifact | Applies only to the encoded definitions and assumptions |

This proposed record format implements the existing evidence separation. No lower class substitutes for another. A hash binds bytes but does not prove semantic equivalence or independent authorship. [UISEC-012; models/sherlock/qa/model_package.py:152-159,264-277]

### Oracles and properties

Keep frozen literal expectations and maintain a model evaluator independent of production transition helpers. Use generated programs and traces for coverage, then shrink divergences into small regression fixtures. Record generator version, seed and bounds. A model-derived trace is not an independent oracle for the same model. [proposal grounded in models/sherlock/qa/test_model_package.py and openspec/changes/sherlock-universal-interpreter-design/model-coverage.md]

Require properties for exact integer arithmetic, immutable allocated data, well-bracketed frames, deterministic rule choice, work monotonicity, canonical encoding and preservation of untouched fields. Compare chunked versus uninterrupted execution at every boundary. Compare optimized versus baseline traces through the declared step adapter. Keep financial effects, pending duties and at-most-once settlement in their own stateful cases. [proposal grounded in UISEC-002–009 and docs/sherlock/design.md:31-45]

Retain independent cryptographic verification, while stating shared-library dependence. A second process using the same verifier implementation checks artifact handling independently, but is not a second cryptographic implementation. Native verification and local ledger validation must both precede target qualification. [qa/sherlock/predicates.yaml:A5-A8]

### Suggested stable requirements

- **ASR-001, ubiquitous:** The assurance reporter SHALL label every result with evidence class, source identity, semantic profile and tested bounds.
- **ASR-002, event-driven:** When a dependency fingerprint changes, the qualification checker SHALL mark dependent evidence stale until revalidated.
- **ASR-003, unwanted behavior:** If mutation execution panics, times out or fails synthesis, the checker SHALL refuse constraint-rejection credit.
- **ASR-004, state-driven:** While a recursive result has undischarged obligations, the verifier SHALL refuse final-valid classification.
- **ASR-005, event-driven:** When a target call qualifies a candidate, the checker SHALL bind its receipt and complete effects to that candidate's manifest.
- **ASR-006, optional:** Where an optimization is enabled, the checker SHALL compare its declared semantic observations with the baseline on the qualified domain.

Acceptance checks are respectively schema/label tests, changed-input tests, injected harness failures, skipped-decider tests, wrong-manifest receipts and differential traces. These are proposed additions, not claims of implementation. [proposal grounded in the gaps cited above]

## Midnight ledger 9 impact

The current official upgrade page places Mainnet and Preview on ledger 9.1.0.1 and leaves Preprod on ledger 8.1.3. Preview reset while retaining its network ID and endpoints. Therefore chain identity must include more than the string `preview`. Ledger-8 transaction envelopes no longer decode on Mainnet. Contract-to-contract calls initially cover unshielded data. Existing Sherlock assumptions need a new compatibility run. [https://docs.midnight.network/relnotes/ledger-9]

The released ledger-9 proving stack uses zk-stdlib v2 and circuits v7, carries new proofs as `ProofVersioned::V3`, and adds V4 contract-operation verifier keys. Older proof support remains version-dispatched. ZKIR v3 is supported through the separate ZKIR repository. Some hash-to-curve and Jubjub-scalar encoding circuits require regenerated keys. Preserve explicit proof, IR and key versions in the manifest rather than treating the number 9 as a sufficient pin. [https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-9.1.0.0]

Contract-call transcripts now reject noncanonical fields and unnormalized `noop 0`. Fee parameters separate validation, guaranteed execution and fallible execution, with an explicit price floor. Test canonical transcript serialization and remeasure real fees for the chosen operation. Old fee observations cannot qualify the new target. [https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-9.1.0.0]

The documented Mainnet/Preview toolchain uses Compact 0.36.0, runtime 0.21.0 and proof server 9.0.0. Preprod uses different pins. Recompilation changes the deployed key relationship, so preserve old artifacts and qualify replacement keys explicitly. Our stagenet must advertise and demonstrate its own runtime compatibility. Its hostname alone establishes no equivalence with Mainnet. [https://docs.midnight.network/getting-started/installation; https://docs.midnight.network/relnotes/ledger-9]

Official pages disagree on whether the Preview reset occurred November 6 or 7. Do not resolve that conflict by assumption. Read the live genesis/reset identity and let the scout reconcile the records. Node release notes also describe a `VerifierKeyRemove` client issue. Rollback must be rehearsed for the chosen release and operation rather than assuming key removal works. [https://docs.midnight.network/relnotes/node; https://docs.midnight.network/relnotes/ledger-9]

## Open questions

1. Which profile and capacities define the first supported recursive release, and which required semantics remain experimental?
2. Which semantic-step and encoding theorems are required for that release, and where do their checkable proof artifacts live?
3. Does the final C6 constraint path reject hostile raw witnesses after host validation is bypassed?
4. What exact ledger/prover/key combination does the current stagenet run, and which released components reproduce it locally?
5. What finality and reset identity can the selected RPC/indexer expose for receipt qualification?
6. Which operations remain possible for rollback under the deployed maintenance authority and current key-update behavior?

These questions are proposed closure conditions. Their implementation evidence is not supplied by this position.

## Confidence, with reasons

High confidence in the evidence-accounting and harness findings: they follow directly from inspected code. High confidence that retained finite comparisons must remain distinct from formal correspondence: the current package says so explicitly. [models/doctorwatson/factory/status.py; crates/mori-backend/src/qa_mutation.rs; models/sherlock/qa/test_model_package.py]

Moderate confidence in the milestone sequence: it reuses the implemented factory and recursive chain, but the complete C6 path still has a recorded dispatch resource failure. Low confidence in current performance or full ledger-9 qualification: this review ran no proofs, heavy builds or target transactions. The only executed project status check was `scripts/sherlock-next --json`, which reported A8 first, with A 7/10, B 4/9, C 5/6, U 34/35 and F 11/13. Those are retained status counts. [docs/sherlock/HANDOFF-2026-10-26.md; qa/sherlock/status.json; scripts/sherlock-next]
