# L18 public check profile contract

Source-only patch against main74a909d69639df8823a527a4d608a7516ccd69a2.
CHANGE.patch adds one ordinary integration target and six normal fixture/report
pairs under crates/. BEFORE.json records each new path absent at the exact base;
AFTER/ contains every full candidate file. SOURCE-BEFORE/ and SOURCE-PINS.json
retain exact referenced source bytes and hashes. AFTER.json pins candidate files
and the patch. HANDOFF.json records the unrun command and limitations.

The test invokes the real public binary for check --json. Two contrasting
financial source forms per beta/Core1/Core2 fix complete literal report objects,
exit0 and empty stderr. ORACLES.md explains independent expectation derivation.
No parser/evaluator/report output generated the expectations.

No test, compiler, application runtime, SDK, provider or network operation was
run. No Git or shared source was modified. This is an authored review candidate;
normal fixture validity and implementation agreement await authorized execution.
