# astra-1 cross-critique

I read all six positions after publishing my independent position. This critique uses source base `8fd34ed794eda7186c32495f715609ec54a9a297`. Recommendations concern the consensus design, not permission to execute it.

## fable-1: VM architecture

**Agree.** Keep the existing machine, bounded representations, in-circuit dispatch, digest statements, IVC and final decider. Require a measured universal-leaf versus family decision. Bind the configuration and layout to the accepted relation. Retain explicit fragment premises and the original requirement IDs. [fable-1/position.md:K1-K8,I1,I4; crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:34-38]

**Disagree: the current statement needs an explicit repair.** K4 and I1 retain five semantic lanes, while the actual IVC consumer uses binding plus six lanes, including `steps`. The current C6 adapter emits six total lanes and the GP consumer requires seven. Freeze the producer/consumer schema before selecting S. Define whether height counts physical transitions or charged work. Free publication means these counts cannot be conflated. [crates/mori-backend/src/sherlock_c6_child.rs:817-834; crates/mori-recursion/src/ivc.rs:191-201; crates/mori-recursion/tests/sherlock_gp.rs:183-196; models/sherlock/k/main.k:73-107]

**Disagree: 24 random states and 40 mutation states are not an adequate coverage rule by themselves.** Smaller runs are useful, but replacing broad checks must preserve explicit coverage of every admitted rule, competing dispatch branch, priority boundary and invariant premise. Add stratified small fixtures and split heavy receipts. Keep stress sampling as supplemental coverage. The existing test is already sampled across alternate rules, not exhaustive over the state space. Its lack of a theorem does not make additional checks worthless. [fable-1/position.md:I2; crates/mori-backend/src/sherlock_machine/c6_dispatch_tests.rs:95-133]

**Disagree: S=8 is an experiment, not a qualified design fact.** The proposed hour estimate counts reduced node work without establishing the new leaf's cost or shape. Begin with S=1 and choose S after measuring the complete leaf/node/decider path. An S-step leaf also needs exact length, terminal padding and disabled-step rules. [fable-1/position.md:I1,M2; wiki-llm/recursion-perf/CONSENSUS.md; openspec/changes/recursion-phase-profile/design.md]

**Disagree: re-earning the entire A/B deployment chain should not block unrelated native implementation.** Keep dependency-specific gates. A legacy anchor qualification and a new C6 composition can proceed independently when their evidence stays separate. The B rollup uses a different workload and anchor. [fable-1/position.md:I8; qa/sherlock/predicates.yaml; docs/sherlock/design.md:277-284]

**Concede.** A single production witness-stream interface can reduce adapter drift. Keep independent semantic oracles and raw hostile-witness injection outside that production interface. Reusing the production encoder as the sole expected-output producer would hide shared defects. [crates/mori-backend/src/sherlock_c6_witness_stream.rs; models/sherlock/qa/test_paired_projection.py:95-102]

## fable-2: recursion and proving

**Agree.** Keep the native leaf/node/decider route. Add the GP workloads to C0 profiling. Pin configuration, key lineage and the recaptured call template. Keep the A102/A80 chain as regression evidence while qualifying real C6 leaves. [fable-2/position.md:K1-K7,I2-I8,C2; crates/mori-recursion/src/ivc.rs; crates/mori-recursion/src/run_decider.rs]

**Disagree: evidence is not a weakest-to-strongest ladder.** Resource measurements are orthogonal to semantic or cryptographic correctness. A network call does not prove a model theorem, and a native proof of a wrongly constrained relation does not repair that relation. Use an obligation matrix with independent evidence classes. [fable-2/position.md:Assurance strategy; openspec/changes/sherlock-universal-interpreter-design/security-requirements.md:Abstraction boundary and evidence classes]

**Disagree: public-lane mutation and flipped-proof tests are necessary but incomplete.** Mutating an existing proof's public inputs tests binding. Add fresh false-statement proof attempts through raw witnesses, coordinated mutations, invalid inner proofs with a valid outer form, and genesis-carried-accumulator tests. The current toy leaf intentionally leaves binding and step count unconstrained. No toy result closes C6 semantic assurance. [crates/mori-recursion/tests/sherlock_terminal.rs:310-330; crates/mori-recursion/tests/sherlock_ivc.rs:68-72; crates/mori-recursion/src/ivc.rs:308-368]

**Disagree: heights do not become bounded integers because the host uses `u64`.** State the in-circuit bounds or the exact lower-layer invariant establishing them. Add no-wrap and exact-length obligations. Keep current source observations as open proof obligations rather than calling them demonstrated exploits. [crates/mori-recursion/src/ivc.rs:308-368]

**Concede.** The first small complete C6 path is a better recursion acceptance target than a fixed-pair speedup. A measured S choice can provide useful later batching. Two concurrent heavy proofs would need an explicit bounded scheduler change, because the current factory contract uses one exclusive heavy lock. [fable-2/position.md:C2,I1; models/doctorwatson/factory/CONTRACT.md:heavy]

## fable-3: delivery workflow

**Agree.** Use the current factory commands. Distinguish mutating operations from read-only predicates. Add reverse requirement traceability, explicit target manifests and persistent public receipts. Require a production-compatible Preview rehearsal instead of transferring old stagenet evidence. [fable-3/position.md:I2-I8; models/doctorwatson/factory/CONTRACT.md; https://docs.midnight.network/relnotes/ledger-9]

**Disagree: resetting the stop clock is not an assurance milestone.** Re-running F4/F11 can supply current factory evidence, but a new factory pass does not resolve the old C6 timeout or missing call. Record the prior STOP, its trace and the changed engineering hypothesis. Do not present a reset timestamp as progress on recursion. `status._update` resets the counter on any newly passing predicate, while `stoprule.evaluate` only observes that counter and age. [models/doctorwatson/factory/status.py:77-112; models/doctorwatson/factory/stoprule.py:20-41]

**Disagree: keep offline T0 and live target checks separate.** A local manifest consistency check belongs in T0. Runtime/genesis/current-parameter observations belong in an explicit network qualification check. A transient RPC failure should not make unrelated local source work fail the pre-commit path. Record unreachable as infrastructure failure, with no compatibility credit. [fable-3/position.md:I8; qa/tiers/t0.sh; qa/tiers/t3.sh]

**Disagree: key compatibility must be checked rather than declared universally lost.** Regenerating the affected lineage under the chosen pins is reasonable. However, the released ledger still supports old proof versions, and the notes identify particular changed circuit classes. Do not assert that every old key is invalid merely because Mainnet upgraded. Distinguish old-artifact historical validity, fresh-recompile compatibility and current deployment qualification. [fable-3/position.md:Midnight ledger 9 impact; https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-9.1.0.0]

**Concede.** Preview can be this change's deployment exit, with Mainnet a named successor. The brief still requires a concrete Mainnet workflow or exact missing additions. Keep the T4/schema/runner work explicit if the workflow depends on it. A proposed command with network-specific scripts still hardcoded for stagenet is not yet runnable. [BRIEF.md:Phase 3; models/doctorwatson/factory/predicates.py; scripts/qa/check-a8.sh]

## astra-2: adversarial soundness

**Agree.** The six-versus-seven lane mismatch is source-confirmed. Explicit outcome/result binding, canonicality premises, integer bounds, authenticated genesis and final decider closure belong in the first real C6 qualification. Add coordinated mutations and reject evidence that fails before its claimed verifier stage. [astra-2/position.md; astra-2/security-clauses.md; independently verified sources listed above]

**Disagreement resolved.** My initial wording required a canonical post-state for every semantic step. The existing B contract permits unreachable internal rows under `StepInv`. I adopt invariant preservation internally and canonical decoding/projection at external boundaries. `assurance-proposed.md` has been corrected. [openspec/changes/sherlock-universal-interpreter-design/security-candidate-b.md:64-90]

**Qualification.** Require the evidence to distinguish a vulnerable isolated fragment from an accepted bad full run. A reachable-genesis and preservation argument may exclude some fragment witnesses. Conversely, a host constructor's refusal does not discharge the circuit argument. [astra-2/position.md:State and discharge the canonicality invariant; security-candidate-b.md:A4]

**Concede.** SND-001–007 provide the sharper security clauses. The chair can reuse those and retain ASR clauses only for evidence classification, oracle sensitivity and freshness. One requirement mapping is preferable to overlapping ID systems. [astra-2/security-clauses.md; astra-1/assurance-proposed.md]

## astra-3: independent critique

**Agree.** Treat A8's missing call, C6 dispatch's preprocessing timeout and full native qualification as separate problems. Keep the existing native architecture. Reuse current evidence only after checking relevant inputs and target identity. [astra-3/position.md; scripts/qa/check-a8.sh; crates/mori-backend/src/sherlock_machine/c6_dispatch_consumer.rs; models/doctorwatson/factory/status.py]

**Qualification.** Split dispatch work into small checks and retained coverage records, but do not let a reduced diagnostic fixture replace complete qualification coverage. An omitted semantic rule or invariant premise remains open even if every selected test passes. [astra-3/position.md:Treat C6 as a test-cost problem; UISEC-012]

**Concede.** I accept requalifying the existing recursive anchor alongside C6 completion. My initial milestone sequence was too serial if read as a prerequisite for that experiment. The exact candidate and scope must remain visible. [astra-3/position.md:Implementation plan; qa/sherlock/predicates.yaml:A2-A10,U023]

## Proposed resolution for the chair

1. Freeze one versioned leaf statement and an invariant/encoding contract before real C6 proof qualification.
2. Keep S=1 as the baseline. Choose batching, architecture and performance budgets from measured complete-path costs.
3. Use stratified rule/boundary coverage, typed failures and deliberately weakened controls. Preserve all untested obligations.
4. Bind every release result to its dependency closure and every target receipt to the exact candidate and chain instance.
5. Separate legacy endpoint recovery, GP qualification and future Mainnet promotion. Preserve broader financial obligations without putting them on the first pure-program experiment's critical path.

I have not cast a vote. I will review the chair's complete draft before voting.
