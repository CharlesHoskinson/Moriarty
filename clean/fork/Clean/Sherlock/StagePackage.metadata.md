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

No custom axioms or `sorry` declarations are introduced. A coordinator-produced
`memcap 48G lake build` log on 2026-10-25 records successful builds of this module
and its query module under Lean 4.33.1. All five theorem queries report empty axiom
sets. The emitted task JSON was compared with the live U026 stage entrypoint.
The first package build failed in three other modules; after their repairs,
the coordinator's second capped build succeeded with 17 jobs and exit 0.
Log digests and the exact per-theorem sets are recorded in the JSON metadata.

`scripts/doctorwatson check U021 U026` separately passed both report checks on
2026-10-25. Direct capped Lean execution from the worker sandbox remains blocked
by its unavailable systemd user scope. The module result above was read from
the coordinator's independently produced build log; no worker build bypass was
attempted. The shared Lake pin matches the adapter's historical 4.33.1 toolchain,
whose historical records remain unchanged.

Open obligations remain the current stage report's recursive generic semantics,
helper-metered fuel schedule, admitted bounded encoding, actual step constraints,
soundness/completeness, exporter capabilities and stock ledger-9 integration.
The private implementation stays private, and these maintainer modules add no
public developer build prerequisite.
