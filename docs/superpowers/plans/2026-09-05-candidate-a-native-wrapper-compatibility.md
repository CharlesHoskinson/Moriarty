# Candidate A native wrapper compatibility addendum

This amendment follows actual installed Quint 0.32.0 diagnostics. It changes
wrapper packaging and the native ITF variable-order literal, not any authority,
Core, lifecycle, event, guard, state-field or case-inventory obligation.

## Observed incompatibilities

The original anonymous parameterized wrapper imports are not transitively
exported: the producer test reports QNT404 for imported init/state/action names.
A small named `DriverA4` instance followed by `export DriverA4.*` reports QNT500
for an uninitialized parameter in both test and native run. This spelling does
not preserve bound constants in the installed toolchain and is not adopted.
Neither diagnostic counts as either intended Candidate A behavioral RED.

A small literal-body wrapper passes the imported bound-ordinal32 test and native
one-step run. Its original raw ITF SHA256 is
`7b14b09c806313f38548a689f1c909033d6fbb7182350ef82c70c4932744aaf4`.
The actual native `vars` list is exactly:

```json
["authorityState","caseIndex","cursor","latestEvent"]
```

This is alphabetical order, not the declaration order assumed by the earlier
transport plan/checker. Both states retain ordinal32 and cursor0/1; the probe
does not execute Candidate A and is not largest-case feasibility evidence.
Original stages are under `.superpowers/sdd/a4-producer-receipts/`:
`task2-boundaries-red`, `task2-instance-probe-test`, `task2-instance-probe-run`,
`task2-literal-probe-test`, and `task2-literal-probe-run`.

## EARS and OpenSpec scenarios

### NW001 — Literal native driver binding

WHEN a native wrapper is generated, the renderer SHALL preserve the shared
driver body exactly except its module name, added case-table import and replacement
of the two constants with typed pure values for that wrapper's case and ordinal.
The complete shared template SHALL remain in the admitted source closure through
an explicit static-aggregate import. No authority/action body may be specialized.

Scenario: WHEN all 78 wrapper texts are checked THEN reversing only the declared
literal/header substitutions reproduces the exact shared template for each one.
The deliberate ordinal31 fault in wrapper032 remains only until its genuine
native initialization assertion fails; final regeneration restores ordinal32.

### NW002 — Exact native variable order

WHEN producer or checker reads native ITF metadata, it SHALL require exactly
`authorityState,caseIndex,cursor,latestEvent` in that order, with no missing or
additional variable. The state field set and meaning SHALL remain unchanged.

Scenario: WHEN the observed native-order fixture is checked against the old
checker THEN both small honest-shard tests fail at the exact raw-vars predicate.
After the sole checker tuple correction, all existing 79 tests SHALL pass.
This is a transport compatibility control, not a semantic mutant kill.

### NW003 — Complete renewed verification

After literal wrappers replace instances, the producer SHALL repeat static
aggregate size/closure preflight and native typecheck coverage for all 78 final
wrappers. The old one-driver parse measurement SHALL not describe the new bodies.

Scenario: WHEN the final driver test suite runs THEN both intended verification
and ordinal behavioral controls have original assertion failures, followed by
all 12 corrected Quint tests and exact generated-wrapper checks.

The largest-case native feasibility pilot and all final exports remain separately
required. No synthetic toy trace substitutes for either. Root controls source
edit windows and commits; no active producer command may observe checker edits.
