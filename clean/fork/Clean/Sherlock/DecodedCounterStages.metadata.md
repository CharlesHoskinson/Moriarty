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

No custom axioms or `sorry` declarations are introduced. The four theorem
dependency sets are queried in `DecodedCounterStagesAxioms.lean`; they have not
yet been observed. The import closure includes the unchanged adapter and
`NullaryCounterHeadroom` and ultimately `Init`.

Kernel status: **unverified: corrected source not built**.

The coordinator's first `memcap 48G lake build` log reports a rewrite failure
in `p4_helper_preserves_accounting`: rewriting by a projected-record equality
did not match the target's unfolded finite values. The one-line hypothesis is
that transporting the equality with `congrArg` avoids that rewrite-pattern
mismatch. The corrected source uses that direct equality argument; it has not
yet been built. No theorem dependency observations are recorded for the changed
module. Direct capped execution from the worker sandbox is unavailable because
`memcap 48G` cannot connect to the systemd user bus. No worker bypass was attempted.
The existing U021/U026 report checks passed but do not verify these proofs.
