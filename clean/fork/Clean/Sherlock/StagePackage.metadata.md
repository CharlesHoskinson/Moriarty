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

No custom axioms or `sorry` declarations are introduced. Kernel dependency sets
must be read from the five `#print axioms` queries after a successful build.
They have not yet been observed for these sources.

Kernel status: **unverified: build not run**.

Verification so far: `scripts/doctorwatson check U021 U026` passed both checks
on 2026-10-25. Those checks validate the existing reporting entrypoint and do
not compile these Lean modules. Lean execution is blocked because `memcap 48G`
cannot connect to the user systemd bus; no uncapped Lean build was attempted.
The shared Lake package pins Lean 4.33.1, installed successfully with elan. This does not supersede
the adapter's historical 4.33.1 observations.

Open obligations remain the current stage report's recursive generic semantics,
helper-metered fuel schedule, admitted bounded encoding, actual step constraints,
soundness/completeness, exporter capabilities and stock ledger-9 integration.
The private implementation stays private, and these maintainer modules add no
public developer build prerequisite.
