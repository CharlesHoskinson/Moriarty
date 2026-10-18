# Core/2 reserve → Quint projection: unapplied source packet

Source basis is exactly `74a909d69639df8823a527a4d608a7516ccd69a2`, read with `git show` because the shared checkout HEAD had advanced. All writes for this lane are inside this new packet directory. No repository source, Git index/commit, provider, target, secret or external cache was deliberately modified. No Quint/TLC/SDK/compiler/test process was executed. The root integrator owns applicability, integration, independent review and actual-check admission.

The predicate is one existing ordinary reserve trajectory through the actual Core/2 library API and the existing finite lifecycle model, comparing every field that those model record types currently expose against hand-derived literals. This is neither a full cross-layer refinement claim nor native/circuit/public-CLI correspondence. The existing model files/actions/types and prior oracle remain unchanged.

`PROPOSED.patch` has four repository paths:

- `crates/mori-core/tests/continuations.rs`: checks the already-frozen expected continuation site and the advanced segment's frame_post/continuation boundary coherence.
- `crates/mori-core/tests/reserve_quint_projection.rs`: new ordinary integration test and fixed-context extractor. It calls stage::load/start/resume only; prints a JSON semantic projection after complete expected equality.
- `crates/mori-core/tests/fixtures/continuations/reserve-quint-projection.json`: new manually derived complete initial/prefix and five-observation expectations; no existing expected bytes replaced.
- `models/core-stage-correspondence/reserve_shared_projection_test.qnt`: new deterministic test importing the unchanged existing model; complete initial/prefix FinancialFrame equality and complete Observation equality at five cuts. No new model action, state field, event domain or invariant.

`candidate/` retains complete proposed file bodies. `basis/` retains all 36 complete source files, including full model prefix/lifecycle/test, adapter/explanation, the actual Rust comparison consumer/types/evaluator/admission/plan/cost/decoder and fixed reserve source/scenario/event inputs. `BASIS.json` pins their exact bytes. The basis includes the full preexisting oracle for comparison-gap diagnosis, not as an input to the new test. This is a scoped source basis rather than a standalone compilable repository or a complete transitive dependency closure.

`independent-reserve-trace.json` is the literal new expectation freeze; `EXPECTED-FREEZE.json` pins it and discloses that the author read the old oracle. `DERIVATION.md` gives the independent hand arithmetic, source ordinals, ordered effects and work counts; no evaluator or model calculation generated the values. `FIELD-MAPPING.json` names every current record field plus what is absent, fixed by the adapter, or untested. Existing historical model captures remain historical and were not rerun.

## Justified comparison changes

The current `compare_segment` checks semantics, pre/post snapshots, aggregate effects and each emitted step's index/value/snapshot/effects. It does not compare action, full pending frames or typed statement trees/ExprIds. Its `waiting` caller compares attempted/committed work, captures and residuals but omits the existing expected site. These are source facts, not observed test failures.

The proposed helper delta adds a direct independent expected-site comparison, then ties advanced frame_post.site/captures and execution.post to the already-checked continuation. It leaves the frozen ABI-style padded `pending_frame.slots` alone: those slots are not the logical capture list (site1 has extra2/held42 with a third ABI padding zero). The new reserve extractor checks exact source action/semantics, typed capture names and entry/exit frame coherence for its selected trace. Typed statements and expression IDs stay explicitly outside the current Quint representation; they are not invented from the old prose `source_statement` strings. No production evaluator/model fix is asserted or applied.

The new adapter projects every field of FinancialFrame (11), PendingState (8), Invocation (8) and Observation (3). It preserves complete account/state map key sets, explicit duty presence, ordered capture values/effects/source ordinals, committed work, invocation flags and attempted totals. Static asset/party/duty/capture identities are checked before projection, rather than silently dropping additional assets/duties. All numeric projection is restricted to the disclosed normal 0..142 domain. A completed c is an abstract post-state and never a fabricated continuation. The prefix is separate from lifecycle initialization's NotInvoked record.

## Expected observations and limits

One deterministic trajectory uses approval41 (false), approval43 (true), settlementfalse, settlementtrue. Attempted totals are11,15,30,32,42; committed totals11,11,26,26,36. Final reserve/target/treasury/observer balances are56/42/2/42, state basis78/allocated42/marker77, duty present5, idle site0 and captures[]. Gross debit44 = transfer42 + fee2; opening5 + accrued42 − discharged42 = residual5. See the derivation for all intermediate complete states and ordered effects. False observations preserve c exactly but still charge attempts.

A future selected Rust pass must compare the full projected JSON, expose the final projection line and terminate with no pending continuation. A future selected Quint pass must execute exactly the new one-case full-record test. Direct exact comparisons should fail for an incorrect shared field; no expected-value regeneration, field omission or refusal substitution should mask a mismatch. A missing import, compile error or source mismatch is an unapplied-source defect to repair, not a runtime pass. No such outcome has been observed here.

Even dual passes would show one finite trajectory against a shared independent oracle. This packet does not include a runtime Quint trace exporter or claim direct comparison of two independently captured execution streams. The JSON tags are a proposed adapter representation; Quint typed assertions do not attest that the CLI's actual trace serialization has those bytes. General model/source correspondence, full variants (including quote51 BusinessRejected and present-zero/absent transitions), numeric boundaries, recovery, K, Lean/Clean, circuit/native and target qualification remain separate.

## Proposed ordinary selectors for root admission

These are **unrun proposals**, to be bound to the integrated exact tree, actual local tool identities and existing resource admission. No provider or executable discovery was performed by this lane.

```sh
cargo test --frozen -j 1 -p mori-core --no-default-features --test reserve_quint_projection -- --exact reserve_existing_trace_matches_complete_shared_quint_projection --test-threads=1 --nocapture
cargo test --frozen -j 1 -p mori-core --no-default-features --test continuations -- --test-threads=1
quint typecheck models/core-stage-correspondence/reserve_shared_projection_test.qnt
quint test models/core-stage-correspondence/reserve_shared_projection_test.qnt --main=reserve_shared_projection_test --backend=typescript --match='^reserveSharedProjectionTraceTest$' --max-samples=1 --seed=20261018
```

At this **current** basis, `crates/mori-core/Cargo.toml` makes midnight-base-crypto/midnight-serialize optional under `midnight-compat-tests`. The proposed no-default-features Core integration selections do not select those SDK edges. Do not generalize the older PR74/75 report's dev-dependency graph to this later manifest, enable compatibility/disjoint/all-features, or substitute a CLI/workspace package test. The ordinary dependency build graph still has scripts/macros/cache/toolchain effects requiring the integrator's normal binding; this source lane did not inspect external dependency/config/cache contents. Runtime calls are local Core operations, ordinary JSON projection and stdout; no subprocess, network, SDK, VM, prover or target API is added by the test body.

Use the existing admitted local Quint/Node installation and TypeScript engine, not an install/fallback or TLC/verify/native backend. The new test has four explicit resume transitions. Tool/model/test execution remains UNRUN, including formatter/clippy, compiler, patch application and actual import resolution. The applicable Quint skills were consulted; their runtime review steps were intentionally not invoked because this assignment prohibits execution. Graph generation/provider use was not attempted under this lane's write/provider restrictions.

No source-only packet, historical model result or eventual one-trace observation closes U0–U7, M1–M17, MC/SP/G, security or product acceptance. This lane supplies no security judgment or authority change.
