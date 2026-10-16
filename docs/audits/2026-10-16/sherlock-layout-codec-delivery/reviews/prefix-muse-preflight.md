**Verdict: APPROVE_CODE_CONTRACT_PREFLIGHT_WITH_LIMITS**

No nonsecurity source, build-interface, or contract blocker was found by reading the complete supplied candidate. This approval does not authorize execution yet.

**Confirmed consumer behavior**

- The sole new leaf, [layout_observations.rs](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/layout-codec-01-preparation/prefix-01/candidate/models/sherlock/reference/layout_observations.rs), is registered as a `cfg(test)` child of private `layout_a`. It has no public input endpoint.
- It consumes the actual existing `Machine`, `normal_program`, `prepare_a`, `encode_observed_a`, `materialize_trace_a`, and `first_difference`. It does not supply expected states to execution or introduce another evaluator.
- Fixed inputs are only N01 and N02. Program capacities are `[64; 15]` with `data: 64`; state capacities are `64, 64, 32, 32, 16, 16, 32`, matching the existing state-encode fixture rather than creating defaults or admission limits.
- It generates three N01 fuel prefixes and nine N02 fuel prefixes, for the stated 12 runs. Internal materialization equality is checked separately from external corpus-projection equality.
- The runtime comparison in [compare_layout.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/layout-codec-01-preparation/prefix-01/candidate/compare_layout.py) first runs the existing `check.run`, validates the normal family, expands expectations through the existing `expected_observations`, filters only the already-expanded N01/N02 programs and runs, parses the observed document strictly, and uses the existing `compare_observations`/`first_difference`.
- `EXPECTED.json` is not a producer input. Its 2-program/12-run structure is consistent with the runtime comparator contract.
- [run_prefix.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/layout-codec-01-preparation/prefix-01/run_prefix.py) uses the inherited `Attempt` preflight, version, compile, produce, child, postflight, and finish behavior. It selects the fixed producer with `--cfg test`; no libtest suite or substitute test set is selected.
- The stated 27-input compile closure and 44-member snapshot are internally consistent with the supplied `CONFIG.json`. Actual rustc dep-info must still prove that closure during the future run.

**Binding conditions**

1. Do not execute until the pending baseline 35 tests have actually been observed and fresh affected-closure Opus and Muse preflights approve this exact candidate.
2. Use only the planned fresh `attempt-01` invocation after those approvals. Any compile-closure mismatch, preparation/state/materialization refusal, comparison mismatch, source change, or child failure is terminal.
3. Do not increase capacities, retry with different dimensions, substitute a libtest run, rewrite expected observations, or treat a wrapper-only result as success.
4. Opus retains all security and execution-boundary judgments, including the inherited host-runner controls and hostile-test scope. This Muse review makes no security approval.

**Limits**

No Rust, Python comparison, baseline test, or producer execution was performed or observed in this preflight. Compilation risks therefore remain unobserved, including exact `#[path]` module resolution, treatment of `cfg(test)` members under a non-libtest `--cfg test` producer build, dep-info formatting, and runtime encode/decode preservation.

A future pass would establish only complete N01/N02 corpus-projection equality and internal typed preservation for these 12 prefixes. It would not establish independent canonical physical-codec uniqueness, K execution, proof/native/ledger authority, general codec completeness, or any Candidate A/F3/F5 or roadmap gate.
