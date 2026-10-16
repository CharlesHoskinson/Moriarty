# Moriarty DoctorWatson — Fresh Nonsecurity Final Candidate and Actual-Result Audit

## Verdict: **APPROVE_WITH_SCOPE**

This approval covers the exact frozen candidate identified below, for the bounded, local, stdlib-only `--host` delivery described in this report. It is a nonsecurity candidate/result approval only. It does not substitute for the SECURITY reviewer's (Opus) judgments, and it makes no claim beyond the scoped host result.

**Top finding:** No nonsecurity merge blockers were found in the exact candidate or its actual results. The new `Machine::run_steps` behavior, its real producer consumer, the complete normal-prefix comparison, the workflow model results, and the claims/limits documentation are mutually consistent within the approved scope.

## Candidate identity audited

- Base commit: `4172d561128194e4fd4a8d80ec1d9b62c4283b99`
- Staged tree: `f91045548545f10cad5959b6df981681c6b223b4`
- Staged manifest: 184 files. The compressed snapshots and log history under `docs/audits/2026-10-16/doctorwatson-delivery/` are immutable evidence data, not executable changes.
- Changed active behavior is centred on:
  - `crates/mori-core/src/sherlock/reference.rs` — new `Machine::run_steps`
  - `crates/mori-core/src/sherlock/reference_bounded_tests.rs` — 7 bounded normal tests
  - `crates/mori-core/src/sherlock/normal_oracle_inputs.rs` — mechanically extracted shared typed inputs
  - `crates/mori-core/src/sherlock/mod.rs`, `crates/mori-core/src/sherlock/reference_oracles_tests.rs` — test-module wiring for the extraction
  - `models/sherlock/reference/normal_observations.rs` — fixed-input producer calling `run_steps`
  - `models/sherlock/qa/run_host.py`, `models/sherlock/qa/check.py`, `models/sherlock/qa/host_observations.py` — wrapper and diagnostic comparison
  - `scripts/doctorwatson.sh` — explicit `--host` mode on the already-approved stdlib-only command
  - `models/doctorwatson/doctorwatson.qnt`, `models/doctorwatson/doctorwatson_test.qnt` — workflow model and scenarios

## Audit method and its limits

I read the supplied full active source, contextual compile/read inputs, receipts, logs, and the original Muse and Opus preflight reports as untrusted data. Per instructions I ran no tools, commands, or network access, and I did not independently decompress or recompute the ~8 MB producer trace. The integrator independently verified the decompressed SHA-256 of `host-FINAL/host-observations.json.gz` and every consumed current/snapshot hash; hash- and trace-level conclusions below rest on those supplied receipts plus that integrator verification, not on my own reproduction. Compressed artifacts are excluded from the prompt by the source manifests that bind them.

## Merge blockers

**None (nonsecurity).**

For completeness, the two preflight merge-blocking findings were re-examined against the actual final candidate and results:

- **M1 (test-closure extraction) — resolved by inspection, not umbrella compilation.** `crates/mori-core/src/sherlock/reference_oracles_tests.rs` imports only `expr`, `no_functions`, `normal_program`, `uint` from the extracted `normal_oracle_inputs`. Both actual child modules were supplied in full: `helper_call_adapter_tests.rs` and `checkpoint_normal_tests.rs` use only names available through that import set and the parent module's existing scope (the remaining helpers resolve inside `normal_oracle_inputs` and its `extra_oracle_programs` child). No removed name (`boolean`, `var`, `add`, `call`) is referenced through `super::`. Recorded in `docs/audits/2026-10-16/doctorwatson-delivery/source-compatibility.json`. The `mod.rs` umbrella itself was not compiled in this lane; see the limitations section.
- **M2 (no wrapper / `--host` wiring tests) — resolved by actual tests.** The final 79-test Python runs include 10 real entrypoint tests (`models/sherlock/qa/test_doctorwatson_entrypoint.py`, covering `--host` ordering after ordinary QA, exact caller arguments, wrapper-failure propagation, missing configuration reaching wrapper preflight, and `--host`/`--design` mutual exclusion before the checker) and 22 Opus-authored wrapper guard tests (`models/sherlock/qa/test_run_host.py`, covering libtest membership/summary refusal, dep-info closure exactness and outside-input refusal, toolchain-channel pinning, rustup-proxy/compiler-shape refusal, symlinked/nonregular source refusal, source-membership invariants, and CLI argument refusals). Opus authored the wrapper and these guard tests; author identities are retained under `docs/audits/2026-10-16/doctorwatson-delivery/reviews/`. This fresh context has no authoring history.

## Correctness of the new behavior

`Machine::run_steps` in `crates/mori-core/src/sherlock/reference.rs`:

- Delegates each event to the existing `step`, so charged rules, free terminal publication, rule line numbers, and charges are the machine's own, not a reimplementation.
- Counts free terminal publication as one event and stops immediately after the first non-`Pending` outcome.
- A zero budget returns no records and leaves the machine byte-identical; later calls resume the same machine, including a pending publication; an already-published machine returns no further events. These are directly tested (zero-event, partial/resume, free-publication-at-zero-fuel, terminal idempotence).
- Widths 1/2/4/8 retain every independent N01/N02 literal prefix with full `StepRecord` equality (both complete snapshots, exact rule line, raw charge). The one mixed `step`/`run_steps` test is labelled in-source as internal agreement, not an independent oracle — that labelling is honest.
- Terminal publication precedence in `step` (return/error publication before fuel exhaustion, including at zero fuel) matches the frozen K rules 80/82/84 semantics on the post-constructor domain.
- The budget is documented as a host observation grouping limit, not a new fuel/resource policy; the H1 unmetered-host warnings on `step`/`run` are retained and not weakened.

Producer and comparison:

- `models/sherlock/reference/normal_observations.rs` imports only typed input builders, calls the actual `Machine::run_steps` with `fuel + 1` events (sufficient for all charged steps plus one free publication on every finite prefix), emits diagnostic K strings via the existing `TraceView`/`KDisplay` projection, and checks `terminal_has_next` by an actual further `step()` call.
- `models/sherlock/qa/host_observations.py` reconstructs expectations solely from the frozen corpus prefix-overlay contract; `check.py --host-observations` compares all 8 program/entry identities before the 216 runs, every record's rule/charge/complete before-and-after state, terminal state, and absence of a next step, reporting the first difference with raw records/states. A supplied file alone is explicitly not treated as execution evidence (`producer_authenticity: not-established`, `execution_receipt: not-supplied` in the receipt).
- `models/sherlock/qa/run_host.py` fails closed in the ways the receipts demonstrate: fresh exclusive attempt directory outside the checkout, read-once regular non-symlink sources snapshotted read-only, real toolchain binary (rustup proxy refused) with release equal to the pinned channel, rustc dep-info required to equal the declared producer (7-file) and test (8-file) closures exactly, exactly the 7 named libtest results with 0 filtered, artifact hashes rechecked before execution, comparison bound to the produced artifact's SHA-256 and to snapshot hashes, postflight rehash of checkout/snapshot/interpreter/compiler, first failure marking all dependent steps `not-run`, 30-second per-child wall clock with process-group kill on timeout, no retry, no shell.

## Actual results accepted

| Evidence | Result in supplied receipts |
|---|---|
| Ordinary Python QA (`host-FINAL-outer/stderr`, `design-FINAL/stderr`) | 79 tests passed, exit propagated correctly |
| Predecessor RED (`host-RED/receipt.json`, `host-RED/logs/compile-producer.stderr`) | Ordinary QA passed (53 predecessor tests); `compile-producer` failed with exactly E0599 `run_steps` not found; all dependent stages `not-run`; postflight ok; terminal `failed:compile-producer` |
| Compiled Rust bounded tests (`host-FINAL/logs/run-tests.stdout`) | 7/7 passed, 0 filtered |
| Producer vs independent corpus (`host-FINAL/receipt.json`, `host-FINAL/comparison.json`, `RESULT.json`) | 8 programs, 216 fuel-prefix runs, 5,355 complete records matched; `pass`/`match`; decompressed output SHA-256 `cfdc36e0…da3` (integrator-verified) |
| Complete `--host` entrypoint (`host-FINAL-outer/result.json`) | Exit 0, 3.611 s, consumed sources unchanged before/after |
| Workflow model (`design-FINAL/stdout`, `RESULT.json`) | 34 main scenarios + 1 directed-completion test passed; 10,000 samples, 60 steps, seed 1, no invariant violation in 21.383 s; whole command exit 0 in 31.733 s |

The counts are internally consistent: completion fuels 1+7+9+17+27+19+38+82 = 200 charged edges; Σ(fuel+2) = 216 runs; the record total 5,355 matches the per-case prefix arithmetic. The workflow model (34+1 deterministic, sampled separately) is design evidence about the delivery workflow only and is not presented as Sherlock interpreter evidence. The eight zero random witnesses (`testPassed`, `reviewed`, `rebased`, `integrated`, `dependenciesChanged`, `layerRecorded`, `completed`, `sourceOnly`) remain reported as zero; directed reachability is reported separately and does not overwrite them. No formal proof is claimed.

## Claims, OpenSpec/EARS, and consumer usefulness

- The real consumer exists and was exercised: `./scripts/doctorwatson.sh --host` runs ordinary QA, snapshots sources, compiles the actual production Sherlock source and the new `run_steps` tests with the pinned installed rustc, executes the producer, and compares the complete normal observations. The default lane remains Python-only; there is no automatic hosted CI (the site workflow stays manual `workflow_dispatch` only), no Foreman, and no new orchestration infrastructure.
- Documentation (`README.md`, `ROADMAP.md`, `docs/DOCTORWATSON.md`, `docs/DOCTORWATSON-CI.md`, delivery `README.md`) scopes the result accurately: post-constructor (`Machine::new`) domain only, K string output is a diagnostic projection rather than a canonical codec or typed-equality proof, and no parent exit is closed — native authority 0, C4, frozen counters/terminal attempts, and all U0–U7/M1–M17/MC/SP/G exits unchanged.
- Not executed, and correctly reported as not-run: K itself, the Sherlock interpreter Quint projection, Lean, Clean, handwritten circuits, native/SDK/prove/deploy/keys, and Midnight stagenet. SF-01–15 coverage claims in `openspec/changes/doctorwatson/requirements.md` match the concrete Python/shell/Rust evidence supplied; the Quint workflow model is not used to claim interpreter or shell fidelity.

## Nonblocking limitations (accepted; do not block merge)

- **N1:** On a compare failure the raw comparison JSON and its digest are retained on disk, but the wrapper receipt does not parse `first_difference` into the receipt.
- **N2:** Producer `INPUTS` budgets derive from frozen corpus `exact_completion_fuel` metadata for input selection only; no expected traces or outputs are linked into the producer. Wording in the contract/README reflects this; a wrong budget would fail the run/coverage counts.
- **N3:** Snapshot files are `0444` but directories remain writable; the wrapper's own hash is taken after load; normally-exiting children can leave unswept descendants (only timeout kills the process group); no memory or disk caps; the rustc sysroot, system linker/libc, Python stdlib, and kernel are trusted and unhashed; a nonexistent rustc records as `failed:preflight`. Local hashes authenticate nothing in a hostile checkout.
- The `mod.rs` test umbrella (including the pre-existing oracle/image/helper test closure) was not compiled in this lane; M1 rests on the recorded source inspection above.
- A single cosmetic extra final blank line in the mechanically extracted `crates/mori-core/src/sherlock/normal_oracle_inputs.rs` is preserved to match the execution snapshot; `git diff --check` reports only that. It does not alter execution.
- Constructor/K startup (`uiRun`, rule 74) is outside the observation domain by design and remains an explicit gap, as do all cross-layer correspondence obligations.

## Concrete concerns flagged for the SECURITY reviewer (Opus)

These are handed over, not adjudicated here: (1) the H1 unmetered host evaluator (no host time/heap/native-stack bounds, full-snapshot cloning, recursive `Drop`/`Clone`/`PartialEq` and K rendering over `Arc<Expr>`/`Arc<Tree>`) is reachable in this lane only with fixed trusted ASTs and literal budgets, but the public `Machine` API itself remains unsafe for untrusted input; (2) `check.py --repo` executes the pinned serializer's exact hashed bytes while its imports, the interpreter, and cross-file snapshot consistency remain unpinned (TOCTOU disclosed in-source); (3) the N3 wrapper items above (post-load wrapper hashing, writable snapshot directories, unswept normal-exit descendants, unhashed sysroot/linker/stdlib). The wrapper and guard tests were Opus-authored, so the final security merge judgment should be made with that authorship in view.

## Scope of this approval

Merging this exact staged tree is approved from the nonsecurity side for the bounded local DoctorWatson `--host` delivery only, consistent with the user's authorization to merge after applicable review. Any change to a consumed input, the integration base, or the candidate tree invalidates this approval for the affected closure and requires refreshed checks and review (SF-09/SF-11). Nothing here closes, or should be read as closing, any Sherlock cross-layer, circuit, native, stagenet, or roadmap parent exit.
