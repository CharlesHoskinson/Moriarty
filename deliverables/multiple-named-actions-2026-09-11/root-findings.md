# Root findings during implementation

## V1 validation order changed

Reproduced in root-legacy-order-reproduction.txt against the reviewed baseline:

1. Load the existing source-defined-payment.mori (/1).
2. Replace second: Quantity<Units<Cash,1>,0> with first: Quantity<Units<Cash,1>,0>.
3. Remove operation Repay: RepayFields;.
4. Call createFinancialAgreementSourceV1().check(source).

Reviewed baseline returns SOURCE_PARAMETER_NAMES at the duplicate parameter.
Current shared compiler returns OPERATION_BINDING at the agreement instead.

The old /1 flow validated parameters before protected operation bindings.
The new /1 branch calls requireProtectedBinding before compileAction, and
compileAction contains parameter validation. Preserve the /1 order explicitly:
normalize -> parameter validation -> binding validation -> lowering/checking.
The /2 shared-binding-before-action ordering is intentional and can stay.
Add compound-invalid-input regressions to preserve error codes and spans.

Root owns these findings/probes; leave their original evidence intact. This is
an implementation observation, not a completed final audit.

## Published snapshot fixture is not canonical

The actual multiple-action-payment.snapshots.json ends with LF (byte10).
Reading it unchanged into the /2 API produces INPUT_SCHEMA; trimming only the
trailing whitespace produces the expected funded payment. Preserve strict
canonical JSON validation and correct the fixture bytes. This affects the
documented CLI command too. See root-fixture-reproduction.txt and root-api-01.txt.

The first root full test run also has failures; see root-tests-01.txt. These are
during implementation, so please finish correcting them before final output.
The independent root CLI scenarios pass23 cases and typecheck passes. That does
not waive the /1 differential failures or the actual fixture rejection.

## Updated observation after author focused repairs

root-api-02.txt now passes against the actual fixture bytes; the newline issue
is resolved. root-v1-differential-02.txt still has18 failures of48 comparisons
against the reviewed /1 implementation. These are the parameter-before-binding
ordering regression above, across check/elaborate/evaluate. That is the current
unresolved root finding. Run independent-v1-differential.mjs unchanged to verify
the repair; keep all prior failed receipts.
