# Staged decoded-counter accounting

`DecodedCounterStages.lean` extends the original adapter in a separate module,
preserving all historically recorded adapter source hashes. Its implementation
reference is `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs`:
the SI03/SI04 equations at lines 4532–4551, P4/P6 at lines 3794–3850 and
P7 reference handling at lines 3479–3486 of commit `0699194f9`.

The definitions and conditional theorems cover only the five finite counter
lanes already projected by `DecodedCounterAdapter`. P3 leaves counters unchanged.
P4 consumes one work unit; helper-eligible P4 additionally increments helper
work. Non-helper P4 preserves the fuel sum but breaks the reference-balance
equation until successful LegacyComplete P7. A non-helper charged P6 refusal
therefore cannot be described as preserving this adapter's `Accounting`.
Helper P4 increments consumed and helper together, preserving both equations,
including on the counter slice retained by a helper P6 refusal.

This is a mathematical consequence of the stated updates, not a report that a
particular Rust dispatch is reachable or that every owner event has this shape.
Arithmetic-success bounds and positive dispatched work remain explicit premises.
No whole-state rollback, helper-visit accounting, selected-rule validation,
full SI03/StepInv, checked Rust/Fin correspondence, field constraints, financial
correctness or native proof result follows.

No custom axioms or `sorry` declarations are introduced. In the coordinator's
recorded 2026-10-26 Lean 4.33.1 build, `helper_next_bounded` and
`p4_nonhelper_not_accounting` have empty axiom sets;
`p4_nonhelper_preserves_fuel` and `p4_helper_preserves_accounting` depend only
on `propext`. The import closure includes the unchanged adapter,
`NullaryCounterHeadroom` and ultimately `Init`.

Kernel status: **LOCKED_VERIFIED (observed coordinator run)**.

The coordinator's README-conformant
`scripts/doctorwatson heavy --id sherlock-lean-build -- memcap 48G lake -d clean/fork build`
completed all 21 package jobs with exit 0 and normal preflight in run
`20261026T035951Z-sherlock-lean-build`. The ledger is committed in `8853c8f8c`;
the retained output is
`qa/sherlock/traces/20261026T035951Z-sherlock-lean-build.log`.

The coordinator reports an outer flock holding the real shared `heavy.lock`;
the runner's recorded lock is a separate `DW_LOCK` proxy. The worker read the
trace and ledger and checked the current source/query bytes against the run's
Git head. It did not execute the build or independently monitor the outer lock.
Source, log, resource, timing and exact dependency records are in the JSON metadata.
The package build was incremental after build 18. Its timings describe
compilation, not clean-build or runtime performance.

Earlier builds 1–18 are capped, unlocked historical observations. The old
17-job result and its 511 ms / 242 ms module/query measurements remain in
`historicalBuildEvidence`; they do not establish lock-compliant verification.
Direct capped execution from the worker sandbox remains unavailable because
`memcap 48G` cannot connect to its systemd user bus. No worker bypass or delegated
denied operation was requested. U021/U026 report checks passed separately;
they do not verify these proofs. The original adapter and its historical metadata
remain unchanged.
