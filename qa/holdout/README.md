# Holdout scenarios (evaluator-owned)

Sealed scenarios for the DoctorWatson factory. The implementing agent sees only the exit code.
Entry point: `qa/holdout/run.sh <suite>` with suites `rejections-local`, `rejections-stagenet`,
`uireq-001` ... `uireq-035` and `list`.

Exit codes: 0 pass, 1 fail, 2 unimplemented (specified, but the system under test does not yet
expose what the case needs), 3 host unfit.

- `cases/rejections/*/case.json`: mutation, expected verdict (REJECT), target entrypoint, sources.
- `cases/uireq/NNN/case.json`: scenario text anchored to the openspec spec, expected outcome, executor.
- `config/`: harness configuration only (no expectations).
- `lib/`: runners (Python standard library; no new dependencies).

Expected outcomes come from the security and interpreter requirements, never from current
implementation behavior. Only the evaluator edits this tree (`DW_ROLE=evaluator`), then reseals
`MANIFEST.sha256` (`<sha256>  <path relative to qa/holdout>`, sorted, manifest excluded).
