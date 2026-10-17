# R49 complete B pairwise source handoff

Implemented a concrete supplied-trace comparison leaf, 9 positive test functions, and public usage documentation. No W/Git/shared/sourceguard changes were made. Root alone registers the new module after fresh review.

## Exact afterimages

All paths below are relative to `49-b-complete-pairwise-source/candidate/`:

- `crates/mori-core/src/sherlock/helper_machine/complete_comparison.rs` — SHA256 `762da43c80b60622bb24a5ef26746b8e23eb06b5fd37175bcd2f952817560d61`, 3151 bytes.
- `crates/mori-core/tests/sherlock_complete_comparison_positive.rs` — SHA256 `0a4c4c93bdee63e3dd1e9f834c18d872e1c102e1fc210f2ee8bee06e404cceeb`, 19598 bytes.
- `docs/sherlock/b-complete-comparison.md` — SHA256 `ec1f478149e9289082ddf62229d330ecf912e1b8747644dfd8f41d3590ae0c7f`, 1697 bytes.

`CANDIDATE-FREEZE-02.json` is controlling. `candidate.patch` contains only these three new files. First freeze remains intact. Revision02 adds exact independent R52 position fixtures; implementation/docs remain unchanged.

## Behavior and real consumer

`first_difference(expected_initial, expected_records, observed_initial, observed_records)` borrows actual existing `HostSnapshot`/`CompleteStepRecord` types. It returns a borrowed `TraceDifference` at initial, ascending common record index (before/event/after), or length after the common prefix. Empty records are present zero events. Existing exact typed Eq retains all admission/program/layout/image fields, raw references, enum/Option tags, vector order and complete StepEventC. It allocates nothing, serializes nothing, invokes no owner, and does no A/B clock conversion.

Callers holding the existing complete observer's initial snapshot/owned records can invoke this leaf directly. Root's sole required shared edit is `pub mod complete_comparison;` in helper_machine/mod.rs after review. No mod.rs afterimage is provided. Continuity (`first_discontinuity`), returned-state comparison and transition validity remain separate. The helper compares supplied data even when traces are noncontiguous; it is not an authorization, provenance or terminality judgment.

## Test source and independent positions

Tests copy the exact existing R46 UInt7/fuel64/stack8 public admission fixture, then capture once per test and clone owned values. SegBase remains private and has no invented constructor. Public event literals are comparison specimens, not claimed actual owner events. There is no owner step/run or record_step call in the new tests.

Test source precedes implementation source. Cases include equality, both strict-prefix directions and zero events, priority within/between records, original borrowed-reference identity, 25 snapshot-group contrasts, and one contrast for each of the 28 public top-level StepEventC fields. All 16 independently frozen R52 positions have exact additional source checks. R52 was read after first source freeze; its table is unchanged. See `TEST-PLAN.md` and `R52-RECONCILIATION.md`.

## Observed checks

Only ordinary read/hash/Python source-structure utilities ran. Startup guarded status exited 0 and retained its historical blocked loan record. Source inventory checks exited 0: 28 declared StepEventC field names exactly match 28 separate mutation labels; no implementation owner call; zero test step/run/record_step sites; one fixture admit/admit_run/capture call site each; all E01–E16 labels present; all frozen W inputs unchanged. `SOURCE-CHECK-01.json` records these limited facts. Final hash/patch checks are recorded separately.

Rust compilation, borrow/type/derive checks, test compilation and all test bodies are **UNRUN**. Fresh designated-owner classification is required for the new admission/capture test closure; R44/R46 dispositions do not transfer. No Cargo, rustc, build script, proc macro, SDK/native/model/provider/ledger/private operation or broad CI was invoked by this author.

## Prospective Root checks and limits

`PROSPECTIVE-COMMANDS.json` supplies one bounded compile-only metadata argv and side-effect inventory. It includes the exact candidate leaf over real types from completed R46 `attempt-01/libmori_core.rlib`, SHA256 `a805777a7d7c407d6251dcb5cc83da7057931dc22bce9e9ceffe407ece796831`; the earlier preparatory PLAN output path is not used. `prospective/metadata_probe.rs` has no main, test or owner call. This command is unrun and Root-owned.

The old production rlib does not export the new module. Test compilation requires Root registration and a reviewed new production artifact; the plan does not pretend old exports exist. Root owns exact test invocation after applicable classification and fresh non-author source review.

These are representative typed tests, not exhaustive snapshot/variant execution coverage. All captured SegBase values are the existing implied zero; a differing SegBase cannot be constructed through this fixed public genesis fixture. The comparator nevertheless uses complete existing Eq. No native/full-interpreter/formal/financial acceptance is claimed; SCON-007 remains broader. `READSET.json` and `SOURCE-CLOSURE.md` contain source pins, actual composition and exclusions. R20 expectations/outputs were not consumed.
