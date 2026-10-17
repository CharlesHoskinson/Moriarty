# DoctorWatson compiler evidence records

The existing local checker now emits an optional seven-field companion for a
published compiler capture: predicate, recorded source inventory, execution
profile, consumer convention/pinned client, bounds, target and evidence class.
It distinguishes historical compile-only output from reported host observation.
The compiler artifact/1 and default comparison semantics are preserved.

Select it with the existing checker:

```sh
python3 -I -B models/sherlock/qa/check.py --repo . \
  --compile-evidence-record docs/results/sherlock-source-compile-20261017 \
  --compile-evidence-case quote
```

Source inventories and library bindings remain recorded historical identities.
The client call route is not checked; consumer identity is a supported-family
convention and observation mode is inferred from the artifact. No compiler,
interpreter, model, SDK, proof or ledger is invoked. Source/input program digests
and implementation inventories remain distinct.

The [original RED](actual-red-01/RESULT.json) reproduced two absent-API errors.
A fresh ordinary source review found that client byte membership does not prove
a call route. Root froze complete revised expectations before the qualification
edit; the [two-case GREEN](actual-green-02/RESULT.json) then passed. Fixtures are
synthetic reporting graphs and are not interpreter execution evidence.

The first integrated local CI [failed two of254 tests](actual-default-ci-01/RESULT.json):
the optional sibling helper was absent from the existing declared snapshot file
set. The correction puts the actual reporting function inside the already
snapshotted checker and deletes the extra module. Detector, guard and declared
helper set remain unchanged. The [four affected checks](actual-targeted-03/RESULT.json)
passed, followed by [all254 local tests](actual-default-ci-03/RESULT.json) in the
corrected tree. The earlier failed source and results remain here.

The final function also [read both archived real captures](actual-historical-capture-report-03/RESULT.json);
both remain compile-only, with113 recorded source members each. Their output
bytes match the earlier separated implementation. The in-process read has its
actual result, time, checker pin and output digests; it has no separate command
receipt and does not establish invocation authenticity.

The [fresh feature/result review](../sherlock-compiler-evidence-record-20261017/review/r122/REPORT.md)
approves the source and result binding. [Root adopts that increment](ROOT-FEATURE-ADOPTION.json).
An additional [Root-observed optional CLI invocation](actual-optional-cli-03/RESULT.json)
passed after the review, without changing source; its actual result is separate
from the raw opinion's earlier source-only CLI coverage. Original UIREQ025 remains partial
and open: this increment alone does not test its historical-model-only scenario
or turn a standalone unbound artifact into a complete evidence record. Existing
eight original host closures remain separate. No interpreter transition changed;
K, Quint, Lean/Clean, circuits and native/target acceptance retain their own scope.
