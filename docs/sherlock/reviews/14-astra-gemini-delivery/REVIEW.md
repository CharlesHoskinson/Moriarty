# Astra delivery reconciliation of Gemini 09 and 10

Verdict: approve Candidate 02's convergence direction for ordinary implementation after the targeted amendments below and Root's final design/EARS/OpenSpec reconciliation. This is a planning verdict, not implementation, security, formal, native or release acceptance. Root may then arm the newly requested implementation loop. The old cleared goal is not resumed by this review.

I fully read the requested materials and fresh receipts. Candidate 02 SHA-256: `3f689d5fbe9503fb997c8f305331c58801e9db15e618080f2d52cc90068ad7bb`. Gemini receipts report `gemini-4-argon-high` success without backend attestation. I passively verified source/artifact hashes; no product/provider/model/native execution, W/Git edits or peer Astra opinions. W and R denote the supplied public worktree and review runtime.

## Mandatory amendments before final design/spec closure

1. **Replace stale execution gaps with scoped observations.** Seven reviewed ordinary QA files are integrated. The recorded default command passed 241 Python tests. S1 actually compiled the stream producer and tests under Rust 1.99, passed all eight selected Rust tests, ran the producer and independently matched eight programs/216 complete-prefix runs; all 26 snapshot pins remained unchanged. S3 recorded eight unique paths/nine input reads/293,065 bytes. These counts describe different sets: producer compiler inputs eight, test compiler inputs nine, snapshot files 26, explicit checker reads eight/nine. The retaining producer and per-step snapshot cloning establish no memory/allocation improvement. The outer receipt supplies execution; checker `Rust-host: not-run` remains scoped to supplied-data comparison. [S1 receipt][s1], [comparison][s1compare], [ordinary result][ordinary].

2. **S2 is now an actual scoped result, not a proposed execution.** The first SDK-free host compilation succeeded; its following lexical dep-info comparison failed on a `../../../../` spelling and the original failed result remains. The continuation normalized paths as the existing checker does, confirmed the exact ten host members, reused that compiled host artifact, compiled the client, ran one capture, extracted the two payloads, and passed both independent 333-member comparisons. It records 117 unchanged pins. I rehashed all 23 bound continuation artifacts. Context2 remains UNBOUND_NOT_ZERO; checked335, general interpreter, native/target and independent result-review approval do not follow. Preserve both attempts and their scopes. [Initial result][s2initial], [continuation][s2], [named comparison][s2named], [ordered comparison][s2ordered].

3. **Correct the N07/N08 premise.** Gemini 09:17,42–47 and Candidate 02:17 repeat an earlier omission. The published Optimized147 capture passed all 147 selected tests, including all six N03–N08 exhaustive prefix/cut suites. Its 22,040 schedules are source-derived, not measured runtime counters. The earlier 145-test result and exact-two timeout remain historical facts. My earlier AD3 paragraph also needs this later-evidence qualification. Do not restore or rerun already passed N07/N08 merely to satisfy the stale recommendation. This capture does not close current Core165, full-Core/Cargo, B N01/full96 or independent whole-B complete-state qualification. [Optimized147 README:3–11][optimized], [complete harness][optimizedlog], [implementation review:37][implementation].

4. **Keep residualization and witness requirements precise without adopting foreign mechanisms as gates.** Candidate 02:67 already retains B-T07–09/UIREQ-031–035 in v1. Adopt finite immutable specialization keys and complete state preservation on fallback. Reject Gemini's dynamic `(pc, mode, helperPhase)` key as written: the existing interface keeps runtime control, helper cursor, fuel, frames and outcomes dynamic. `StepEventC` is not a complete state image. Generic fallback at the residual-size bound does not exhaust the application, and it preserves every pending/free publication and charged boundary. Do not turn finite comparison into proof of all39/B or promise dispatch elimination. [Interfaces:279–290][interfaces], [C2:81,95–99][c2profiles].

   The required uncached witness DAG needs explicit dependencies, lifetime units and boundary live-outs under the existing owner-approved storage/mutability contract. Foreign allocation rules, extra widths and identical physical digests are not new requirements. Compare complete decoded meaning; hashes bind artifacts. Security implications remain with their owner. [Gemini 09:21–40][g09], [C2:99][c2profiles].

5. **Make implementation and documentation follow the existing consumer path.** Keep Candidate 02:83–91's extension of the existing `mori compile` path, freeze its profile/action/artifact contract in the existing OpenSpec, and route actual source through the parser/Core adapter and admitted program to a real consumer. Use two independently expected contrasting positive programs; unsupported features must remain explicit. Generic caller work subject to H1 awaits its existing designated-owner disposition; unrelated source/oracle preparation continues. Native controls and C4 are dependencies of affected work, not a stop on every ordinary module. Keep the entire current v1 and U0–U7 release scope; a literal demonstration is an incremental result. [C2:83–91][c2consumer], [ownership/dependencies][ownership].

## Which Gemini proposals to adopt

| Gemini proposal | Decision |
|---|---|
| 09.1 witness lifetimes | Accept required DAG/live-outs. Defer optional reuse; retain owner-controlled mutability. No imported allocation rules or prover-memory claim. |
| 09.2 residual fallback | Accept immutable keys and full dynamic-state fallback. Reject dynamic keys, event-as-state and guaranteed performance/proof claims. |
| 09.3 zero-clone/digest | Defer optimization until measured. Digest remains auxiliary to full records/first divergence. A216 and B96 differ. |
| 09.4 resources/tests | Extend existing accounting with independent complete/prefix/overflow relations under UIREQ-013/025. UIREQ-034 is ADT matching. Reject stale N07/N08 reopening. |
| 10.1 closure/diagnostics | Closure fixed and actually exercised. New rustc JSON and advisory8/9 enforcement are optional, not guarded-wrapper prerequisites. |
| 10.2 S2/codec | S2 actual333 is observed. Add a small production codec consumer with independent initial/step1 expectations; the proposed new flag/runner does not exist. |
| 10.3 review tools | Optional exact-byte/span/hash diagnostics. Fuzzy matching, token occurrence and packet reconstruction prove neither APIs nor reading. No universal review gate. |
| 10.4 completeness | Add independent required-member checks in existing R0/UIREQ-017/025. Preserve measured zero/unmeasured distinction. No universal ten-stage framework. |

DECISIONS.json and DESIGN-ADDITIONS.md provide detailed decisions and paste-ready text, grounded in current consumers and the backlog. [Resource aggregation][resource], [runner][runner], [backlog][backlog], [Gemini 10][g10].

## Small final DoctorWatson improvement and execution readiness

The concrete remaining factory correction is the three existing documentation afterimages in R/15-doctorwatson-final: `docs/DOCTORWATSON.md`, `docs/DOCTORWATSON-CI.md`, and `models/sherlock/qa/README.md`. They describe the real stream command, measured selected results, source-read membership and retained-sink limit. Recommend Root review/integration; it is still a candidate. DECISIONS.json binds its exact patch. Its S1/S3 scope correctly makes no S2 claim. No executable fix or unchanged product rerun is needed for these docs. [Handoff][docsfix].

The critical ordinary path is final design/EARS/OpenSpec plus bounded Quint reconciliation, then Root's new implementation loop: production codec/complete-observation slice, existing public compile consumer under applicable H1, and independently expected rule/helper/ADT/frame/arithmetic/financial slices. Required K/Quint/Lean/Clean correspondence advances per slice. Native/target acceptance retains separate prerequisites. Existing owners work on ready disjoint files; I owns shared schema, exports, CLI wiring and integration. FACTORY-READINESS.json supplies task inputs, write sets, consumer, QA, documentation and dependencies without a new scheduler.

Measure exact-candidate build/run/compare times, compared members/prefixes, reference work and instrumented storage where available. CPU/RAM/disk headroom and review throughput are unknown; there is no justified worker ceiling, deadline or percentage. Root chooses physical concurrency from resources and ready disjoint ownership.

Bounded Quint must exercise state evolution, initial/adjacent/final continuity, zero budget, observer stop/resume, publication/terminal priority, exhaustion with pending frames, canonicalization success/refusal, residual fallback and carried committed/tentative financial fixture fields across technical cuts. Bind finite profile/domain/bounds/seeds and nonvacuous reachability. This cannot establish full39/B, arbitrary recursion/finance, K/Lean/Clean correspondence, memory bounds, native proof or target settlement. Full-v1 release evidence remains required. Native authority 0, C4 OPEN, R2 UNSATISFIED and 60460 HOLD stay unchanged; no Amendment10 retry/reroute or substitute security judgment.

[c2profiles]: /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/DESIGN-CANDIDATE-02.md:81
[c2consumer]: /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/DESIGN-CANDIDATE-02.md:83
[g09]: /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/gemini-research/09-optimization/REPORT.md:21
[g10]: /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/gemini-research/10-delivery-factory/REPORT.md:17
[interfaces]: /home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/openspec/changes/sherlock-universal-interpreter-design/interfaces.md:279
[ownership]: /home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/docs/SHERLOCK-IMPLEMENTATION-PLAN.md:39
[resource]: /home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/crates/mori-core/src/sherlock/helper_machine/resource_event_accounting.rs:1
[runner]: /home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/models/sherlock/qa/run_host.py:325
[implementation]: /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/01-implementation/REPORT.md:37
[backlog]: /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/06-doctorwatson-backlog/REPORT.md:43
[optimized]: /home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/docs/results/sherlock-normal-optimized147-20261017/README.md:3
[optimizedlog]: /home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/docs/results/sherlock-normal-optimized147-20261017/test-harness.stdout
[ordinary]: /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/checks/integration-ordinary-01/RESULT.json
[s1]: /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/checks/stream-actual-01/receipt.json
[s1compare]: /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/checks/stream-actual-01/comparison.json
[s2initial]: /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/checks/composed-actual-01/RESULT.json
[s2]: /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/checks/composed-actual-01/continuation-01/RESULT.json
[s2named]: /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/checks/composed-actual-01/continuation-01/compare-named.stdout
[s2ordered]: /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/checks/composed-actual-01/continuation-01/compare-ordered.stdout
[docsfix]: /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/15-doctorwatson-final/HANDOFF.json
