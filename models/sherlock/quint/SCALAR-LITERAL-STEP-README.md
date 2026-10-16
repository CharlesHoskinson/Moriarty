# Scalar literal transition checks

The two tests in `scalarLiteralStepTest.qnt` exercise the unchanged generic
`sherlock-k-a/1` model on UInt 7 and Bool false. Both selected tests and the
typecheck completed with exit 0 in the captured October 16, 2026 execution.
The source and model hashes at execution remain unchanged. The source comment
“Tests UNRUN” records its pre-execution freeze; the captured results below
provide the later observation. The original guide is preserved byte for byte.

Each test compares every Snapshot field before execution, after charged literal
rule 102 and after free publication rule 80. Startup rule 74 is explicit. The
UInt case publishes at zero remaining fuel; the Bool case retains two units.
Depth derives from the empty frame list and consumed work from initial minus
remaining. No Candidate B allocation counter is supplied.

`scalarLiteralStep.expected.json` is the hand-derived fixture frozen before the
emitter implementation. Its specified-only status describes that original
freeze. The Quint process did not consume this JSON file; expectations appear
as explicit normal states in the tests. No interpreter generated expectations.

The [evidence packet](scalar-literal-step-evidence-r1/README.md) preserves the
exact commands, limits, source/tool pins, independent source audit, execution
classification, separate Root admissions, raw output and observed results.
These are two finite normal model observations. No K/Clean/circuit
correspondence, theorem, constraint check, native proof or target result follows.
Earlier model cohorts were not rerun. All parent exits and native authority zero
remain unchanged. Current source/result/publication reviews are pending.
