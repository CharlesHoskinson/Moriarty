# DW2 streaming source review

**Verdict: APPROVE_SOURCE_WITH_SCOPE. Material corrections: none identified.** This approves the four exact afterimages for ordinary source integration review. It is not execution evidence, a security vote, native admission or full Sherlock acceptance. Root still selects any SDK-free operation and shell integration.

Candidate: `07-doctorwatson-local-qa`, based on W HEAD `628615a94d8a74837a1af510293927bb74d5c2b5`. I read README, the complete patch, both actual modified preimages and all four afterimages. I also inspected the existing batch producer, actual streaming extension, input builders, comparator adapter and shell consumer. No subject compiler/test/producer/comparison/provider/native operation ran. Only review outputs were written.

## Exact source and closure

The patch matches a fresh in-memory unified diff of the actual W preimages and four afterimages exactly, including file order. All 27 source-manifest rows matched actual baseline/candidate hashes and candidate byte lengths. This includes hashing oracle files without inspecting their expected values. Manifest SHA256: `46c1e48f81860752b6fe5190ac0d915cfa1a80d55a1957532d9f89a7dfcac587`; patch SHA256: `029dc01379c7174cf2eb4d16a93776006b1b600b24f985bcd79f3b2ea91e7d72`.

| Proposed path | Afterimage SHA256 |
|---|---|
| `models/sherlock/qa/run_host.py` | `15bfdea85de7abef1a1d0702fe8027b1a74730db1f139a48374a0c831a27808f` |
| `models/sherlock/qa/stream_observations.rs` | `724dd9b40b4f51ae70a1441a5beb693346fc795355a30cb6d6983012da70c921` |
| `models/sherlock/qa/test_run_host.py` | `91b04ead1fd5172d3be051ea59b144840b4a14cdca12d88df28ba6a836468ca2` |
| `models/sherlock/qa/test_run_stream.py` | `60715b35eb24c4d903d86e88a8bf17761cfa9b443caf7d99e298f8ac7f0773de` |

The eight producer members agree with the producer's direct module declarations plus nested `extra_oracle_programs.rs`. The nine test members add only `reference_stream_tests.rs`; no test-only expectation/transcription module enters the producer compile closure. The 25 attempt members separately include the unchanged comparator, literal corpus/transcriptions, toolchain/workspace metadata and shell source. Python wiring tests are not claimed as Rust compile members. Actual dep-info equality remains an execution-time predicate; source inspection does not establish that rustc accepts this closure.

## Real behavior and reachability

`stream_observations.rs:142` iterates the same eight fixed typed programs and fuel range `0..=completion_budget + 1` as the batch producer. The counts are 3+9+11+19+29+21+40+84 = **216 runs**. Stack limits and typed input builders are unchanged. Only the observation mechanism changes: repeated widths 1,2,4,8 call actual `Machine::visit_steps`, clone each borrowed complete callback `StepRecord`, and continue the same Machine until the streaming API reports Terminal. It does not call `run_steps`, use precomputed expected records, replace `step`, or synthesize missing records. Final snapshot and the existing terminal no-next probe remain.

The actual `reference_stream.rs:38` delegates each event to `Machine::step`, invokes the callback, drops its own record, and reports terminal/budget/observer stop under the existing priority. Candidate grouping changes no fuel or rule schedule. Complete retained serialization, including Program/entry, control, environments, top-first frames/depth, fuel/limits/outcome, rule and charge, is unchanged from the batch producer. The candidate openly retains all cloned records; it makes no memory-saving claim.

`run_host.py` adds a fixed stream Realization with its own receipt format, crate/output names, exact source/test closures and `--host-observations` comparator selection. `main`'s existing choices dispatch it into real `Attempt.run` compile/test/produce/compare steps. The source therefore reaches the actual API through a callable public wrapper command; it is not merely a disconnected helper. The diagnostic payload intentionally retains the existing host producer/schema identity, while the receipt's realization and artifact hash distinguish stream execution.

The eight declared test names match all eight existing `#[test]` functions in `reference_stream_tests.rs`; that file contains no ignore marker. Existing `libtest_results` receives this realization's names, so success requires those exact eight passing results and zero failed/ignored/measured/filtered tests. The existing suite, rather than newly invented assertions mirroring the producer, supplies budget/terminal/observer/resume checks. I inspected names/imports/selection, not independent expected literal values, and make no new claim that the assertions are semantically complete.

`scripts/doctorwatson.sh` still invokes the default host realization under `--host`; this candidate explicitly defers shell selection to Root. Approval here does not claim the shell now runs stream. The standalone `run_host.py --realization stream` route is source-wired. A future shell edit needs its own exact-source review.

## Preserved boundaries and remaining evidence

The complete existing wrapper execution body from `class Failure` onward is byte-identical after accounting for the CLI help-string change. Compiler selection/version checks, per-child 30-second timeout, failure ordering, source snapshots, dep-info checks, artifact binding, postflight and receipt/result publication are unchanged. Host/image realization declarations retain their prior closures/tests. No evaluator, source clock, production streaming implementation, guard, native admission, resource counter or authority file is changed by this four-file patch. These are source-diff facts, not a security judgment about the existing wrapper.

The author-reported Python wiring/refusal checks do not establish Rust behavior. This candidate is still **UNCOMPILED / UNEXECUTED** for the streaming Rust closure and full producer/comparison. Required observed evidence remains: exact compiler/source pins; eight- and nine-member emitted dep-info; eight exact nonignored Rust test passes; actual stream producer output; full eight-program/216-run diagnostic comparison; complete final receipt and postflight. Startup, hostile-input/host bounds, B schedules, K/Quint/Lean-Clean/circuit/native correspondence and all U/M/MC/SP/G exits remain unchanged.

## Combined-candidate integration boundary

After this original review, Root reported that the separate DW3 checker candidate introduces `source_reference_facts.py`, while this DW2 base declares only `host_observations.py` in CHECK_HELPERS. The original approval above remains tied to the original W checker and exact four afterimages. It does **not** approve a DW2+DW3 composition. That combined candidate needs the new helper in its declared/copied comparison closure, refreshed source identities, and a meaningful snapshot-membership regression; otherwise the combined checker cannot complete from the stream snapshot. This is a Root-reported integration correction pending my review of DW3/final additive bytes, not an independently verified finding against the original standalone candidate. Root has requested integration-02; a fresh scoped review of its final delta remains open.
