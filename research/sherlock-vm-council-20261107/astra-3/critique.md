# astra-3 cross-critique

All six independent positions were read. Source base: `8fd34ed794eda7186c32495f715609ec54a9a297`. This critique retains the distinction between observed behavior, source findings and proposed changes.

## fable-1: VM architecture

**Agreement.** Keep the explicit machine, bounded images, private witnesses, native linear recursion and terminal discharge. A production leaf must constrain dispatch and effects together. Use one candidate implementation and a measured fallback instead of building several competing backends. Sources: `fable-1/position.md`, `crates/mori-recursion/src/ivc.rs`, `crates/mori-recursion/src/run_decider.rs`.

**Disagreement 1: an S=8 leaf is an experiment, not the selected performance result.** The cited node timings come from historical small/stand-in runs. Multiplying 95 seconds by 37 nodes omits the unmeasured S-step leaf cost, preparation, memory and changed key geometry. Start with a correctly composed S=1 leaf, then benchmark candidate S values on the complete path. The k≤17 decision may be a useful admission bound, but k alone does not select the lowest latency or memory. Sources: `docs/sherlock/stagenet-gp-recursion.md:184-201`, `wiki-llm/recursion-perf/CONSENSUS.md:19`.

The S-step contract also needs an exact active length and a final partial chunk. For a 293-transition trace with capacity eight, the last chunk has five active transitions. Define inactive suffixes, early termination, exhaustion, no-wrap height and semantic-work accounting. Do not count padding as execution or confuse free publication with zero physical transitions. The current IVC expects a seventh `steps` lane, missing from the current C6 digest producer. Sources: `crates/mori-recursion/src/ivc.rs:191-201,305-320`, `crates/mori-backend/src/sherlock_c6_child.rs:817-834`, `openspec/changes/sherlock-universal-interpreter-design/design.md:25-35`.

**Disagreement 2: replace expensive test organization before deleting coverage.** “Exhaustion buys nothing” is too strong. Finite tests are not soundness proofs, but more distinguishing cases can detect defects. A sample of 24 states and 40 mutations needs a coverage argument. Require directed cases for every rule, priority boundary and relevant malformed shape, then a reproducible sample and stress tier. Preserve the old matrix where feasible through rule-major batches with bounded circuit retention. No reduced run may acquire the complete receipt's meaning. Sources: `crates/mori-backend/src/sherlock_machine/c6_dispatch_tests.rs:65-137`, `qa/holdout/lib/uireq.py:1195-1237`.

**Disagreement 3: avoid unrelated deployment gates on native work.** Re-earning A/B deployments before accepting any new native work serializes independent tasks. The old literal chain is useful migration/regression evidence. The Fenrir B workload is separate from closing the C6 leaf. Require each local result's exact dependencies, then target qualification for that relation. Sources: `qa/sherlock/predicates.yaml`, `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:805-813`, `fable-2/position.md` C2/C5.

**Disagreement 4: one production witness stream must not become one oracle.** Keep the shared producer for application code. Preserve independent expected traces and raw hostile-witness entry points for assurance. Otherwise a shared producer error can make all three consumers agree. Source: `openspec/changes/sherlock-universal-interpreter-design/design.md:17-23` expressly separates metadata reuse from independent expected behavior.

**Concession.** My position left leaf shape selection too implicit. The linear node requires a fixed authorized leaf relation or a properly authenticated finite family. A measured U/F decision belongs in the plan. I also accept excluding `/2` native constraints from a clearly named `/1` milestone, provided the financial and suspension requirements retain tasks and acceptance exits.

## fable-2: recursion and proving

**Agreement.** Keep the digest leaf, linear node and run decider. Extend the existing profiler to those workloads. Keep the fixed literal parent and Fenrir batch as regression/parallel workloads. They must not masquerade as a general interpreter or delay unrelated C6 closure. Sources: `fable-2/position.md`, `crates/mori-recursion/src/ivc.rs:1-21`, `docs/sherlock/stagenet-gp-recursion.md:184-201`.

**Disagreement 1: do not postpone the useful profile until after expensive end-to-end runs.** M5 runs C0 after G4, although M3 already chooses S and earlier work requires a leaf-shape decision. Qualify instrumentation with the smallest real leaf/node/decider, then use it during shape selection. Preserve the later full-program profile as a release measurement. Source: `openspec/changes/recursion-phase-profile/tasks.md:44-62` records that real on/off proof and overhead qualification remain open.

**Disagreement 2: supersede implementation routes without deleting financial obligations.** MC03's old wrapper and process can retire. Its original financial episode, retained-artifact verification and malformed-context obligations need explicit successors in traceability. A pure `/1` result does not close them. Sources: `openspec/changes/mc03-native-recursive-proof/tasks.md:20-40`, `docs/MORIARTY-PRODUCT-CONTRACT.md:35-49`.

**Disagreement 3: performance limits and concurrency need exact scope.** A proposed two-hour/48-GB target can be a release objective. It is not an observed capability. A shortfall must leave that performance requirement open. Two proofs require measured aggregate process memory and an explicit scheduling design under the existing exclusive heavy lock. Adding independent peak estimates is not enough. Sources: `models/doctorwatson/factory/CONTRACT.md`, `docs/sherlock/stagenet-gp-recursion.md:190-201`, `wiki-llm/recursion-perf/CONSENSUS.md:19`.

**Precision.** The nonzero-low-byte result is a fixture/readback convention. Do not turn it into a general UInt128 language restriction. Qualify zero and trailing-zero encodings before claiming arbitrary results. Source: `tools/sherlock-sdk/sherlock-run.compact:19-24`.

**Concession.** The plan should explicitly remove the old A/B demonstrators from the general-purpose critical path. My initial milestone list made rollup requalification sound more serial than necessary. The exact production-capable recursion relation, rather than every historical workload, should determine promotion dependencies.

## fable-3: delivery workflow

**Agreement.** Use the current factory exclusively. Name deployment/call operations separately from read-only predicates. Add executable traceability and preserve public receipts outside ephemeral target directories. Preview is the production-compatible public rehearsal target. Sources: `fable-3/position.md`, `models/doctorwatson/factory/CONTRACT.md`, `scripts/qa/check-a8.sh`, `wiki/midnight-ledger-9-hardfork.md:60-76`.

**Disagreement 1: no blanket claim that all old keys are invalid.** Cargo already uses ledger-v9. The release preserves older proof/key verification and identifies particular changed circuit classes. Qualify the exact candidate and regenerate the affected dependency closure. The release does not establish that every current handwritten key must fail. Sources: `Cargo.toml:27-42`, [ledger 9.1.0.0 release](https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-9.1.0.0).

**Disagreement 2: T0 should remain local.** I8 proposes a live RPC check in T0. Split it into static manifest/lock validation in T0 and live target freshness in target preflight/T3. Remote outages must not block unrelated local commits. Sources for the current separation: `qa/tiers/t0.sh`, `qa/tiers/t3.sh`.

**Disagreement 3: preserve the Mainnet path and existing obligations.** The brief requires a proposed Mainnet workflow. Describe exact additions and future authorization without implying this council can submit transactions. A new T4 tier is optional implementation structure, not a network requirement. Likewise, an unrelated F4/F11 transition can restart the factory clock but cannot close the A8/U023 escalation. Sources: `BRIEF.md`, `AGENTS.md:13-25`, `models/doctorwatson/factory/predicates.py:12`.

**Disagreement 4: recovery requires more than re-pinning.** Git reversal does not reverse chain state or discharge liabilities. Key insertion does not revoke an old vulnerable key. Specify which maintenance operations actually work for the chosen client/node release, and how an abandoned deployment's state and duties remain accounted for. Sources: `docs/MORIARTY-PRODUCT-CONTRACT.md:41-47`, [Compact 0.36 release](https://docs.midnight.network/relnotes/compact/toolchain-0.36.0).

**Concession.** Named operation/precondition/postcondition records improve my original prose plan. I also accept dropping any mandatory additional stagenet submission. Keep stagenet as an optional diagnostic target with its own evidence class. More detail: [workflow critique notes](workflow-critique-notes.md).

## astra-1: assurance

**Agreement.** Negative tests need typed outcomes and a valid baseline. Candidate selection must be explicit, and evidence freshness must follow dependencies. A constraint check, native proof and chain receipt establish different facts. I verified the panic/synthesis accounting and modification-time selection directly. Sources: `crates/mori-backend/src/qa_mutation.rs:458-486,640-650`, `crates/mori-backend/tests/soundness_mutation.rs:227-245`, `scripts/qa/check-a10.sh:7-13`.

**Disagreement/implementation preference.** Add the proposed fields to existing records and candidate manifests. Avoid a parallel assurance database or an additional workflow engine. There is no substantive disagreement with the evidence requirements. The current factory already has result, source and trace records to extend. Source: `models/doctorwatson/factory/CONTRACT.md`.

**Concession.** Evidence classification belongs before using the negative campaigns as qualification. My original strategy named classes but did not identify the harness defect as an early implementation task. Also retain the peer's refinement that internal microsteps preserve `StepInv`, with canonical projection at declared boundaries rather than every microstep. Source: `openspec/changes/sherlock-universal-interpreter-design/security-candidate-b.md`, A4.

## astra-2: security

**Agreement.** The six-versus-seven lane mismatch is a direct integration blocker. Exact segment length, bounded counters, authorized keys, initial invariants and final accumulator discharge must be constraints or explicit proved lower-layer obligations. Raw hostile witnesses are necessary to test the host/circuit boundary. Sources: `crates/mori-backend/src/sherlock_c6_child.rs:817-834`, `crates/mori-recursion/src/ivc.rs:191-201,305-320`, `crates/mori-recursion/src/run_decider.rs:208-276`.

**Disagreement/precision.** There is no substantive disagreement with the security requirements. The canonicality alternative should name the existing `StepInv` induction contract as the default. Full canonical projection on every leaf is a more expensive alternative whose cost must be measured. Retain the publication mutation as a proposed challenge until a raw witness run reaches the relevant validator. Source: `security-candidate-b.md`, A4, under the universal-interpreter change.

**Concession.** My first plan under-specified the new leaf interface. I now require a versioned producer/consumer contract before C6 recursion qualification. Its `steps` field must distinguish physical transitions from charged work, including free publication and partial chunks. A7/A8 regression recovery can proceed independently and cannot qualify this new C6 relation.

## Shared amendments needed in the draft

1. Name `/1` qualification scope and the retained downstream financial obligations precisely.
2. Make the leaf schema, bounded counters and canonicality induction explicit before shape optimization.
3. Keep dispatch diagnostics bounded while preserving directed semantic and adversarial coverage.
4. Use candidate-bound, source-aware evidence and truthful rejection classes.
5. Use Preview for production compatibility, and treat the scout's deployed-source mismatch as an unresolved reproduction limit.
6. Make the complete proposed Mainnet path runnable after the named additions and future operation authorization.

The deployed-source mismatch is the scout's finding, not an independent reproduction by this lane. Its exact source/RPC references are in `wiki/midnight-ledger-9-hardfork.md:166`. Local release tests and direct Preview observations must remain separately named while that gap exists.
