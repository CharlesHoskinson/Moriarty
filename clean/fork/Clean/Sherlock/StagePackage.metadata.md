# U021/U026 stage contract

`StagePackage.lean` follows `models/sherlock/qa/model_package.py` at commit
`0699194f95524033699288678d8405df078c4f40`. It names the six required tasks,
their current statuses and dependency lists. `StagePackageAxioms.lean` emits
those definitions as JSON for comparison with the live `stage` command.
The comparison model preserves the implemented distinction between a ready
package, an omitted comparison, a mismatch, and acceptance (`none`).

`CodecObligations` states partial encoding, admission and complete-image relation
premises; `admitted_round_trip` derives a round trip from those supplied obligations.
`StepSoundness` is an explicit parameter to `trace_soundness`, which
lifts same-clock single steps to traces with the same ordered work labels.
Zero-work publication remains a labeled step. This generic theorem does not
construct an actual codec, semantic interpreter, circuit or proof of their
correspondence. Relating B helper steps to A logical steps needs a separate
erasure relation. Counter-only projection cannot instantiate the complete-image
codec obligation.

No custom axioms or `sorry` declarations are introduced.

Kernel status: **LOCKED_VERIFIED (observed coordinator run)**.

On 2026-10-26 the coordinator ran the README command through
`scripts/doctorwatson heavy --id sherlock-lean-build -- memcap 48G lake -d clean/fork build`.
Run `20261026T035951Z-sherlock-lean-build` completed all 21 package jobs with
exit 0 and normal preflight under Lean 4.33.1. Its ledger record is committed
in `8853c8f8c`; its retained output is
`qa/sherlock/traces/20261026T035951Z-sherlock-lean-build.log`.

The coordinator reports an outer flock holding the real shared `heavy.lock`;
the heavy runner records a separate `DW_LOCK` proxy. The worker inspected the
trace and ledger and checked the current source/query bytes against the run's
Git head. It did not execute the build or independently monitor the outer lock.
All five theorem queries report empty axiom sets. The six emitted task rows
match the own-checkout stage entrypoint with and without `--retained-comparison`;
the stage function's byte hash also matches the observed implementation commits.
Exact source, log, timing, resource and per-theorem records are in the JSON metadata.
The package build was incremental after build 18; its timings are compilation
observations, not a clean-build or runtime benchmark.

Earlier builds 1–18 remain capped, unlocked historical observations. In particular,
the second build's 17-job success is retained as history rather than evidence of
a lock-compliant run. `scripts/doctorwatson check U021 U026` separately passed
the report checks; those checks do not kernel-check this module. Direct capped
Lean execution from the worker sandbox remains unavailable through its systemd
user scope. No worker bypass or delegated denied operation was requested. The
original adapter's source and historical metadata remain unchanged.

A digest binding alone cannot supply `StepSoundness`. Batched semantic steps
also need an explicit count, cursor delta and schedule relation; these are open
implementation obligations, not additional axioms.

Open obligations remain the current stage report's recursive generic semantics,
helper-metered fuel schedule, admitted bounded encoding, actual step constraints,
soundness/completeness, exporter capabilities and stock ledger-9 integration.
The private implementation stays private, and these maintainer modules add no
public developer build prerequisite.
