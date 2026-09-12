# Root checks during implementation

The independent CLI probe passed all 18 cases on the current working candidate,
including complete funded result equality, continuation, interest-first payment,
format idempotence, declaration rejection and no output leakage on failure.

The first root typecheck failed:
`src/successor/financial-agreement-source-v1.ts(262,9): TS18046: field is of type unknown`.
See `root-typecheck-01.txt`. Please resolve this before final verification.

The first root full suite also failed the new test
`transport bounds and old-profile source regressions stay unchanged` at
`tests/financial-agreement-source.test.mjs:442` (expected SourceChecked,
got undefined). All other tests passed in that run; see `root-tests-01.txt`.

These observations are during implementation; they do not freeze or approve the
candidate. The fresh Astra audit follows your terminal result and final checks.

## Reproduced declaration diagnostic gaps

Using the new example and createFinancialAgreementSourceV1().check:

- Change first parameter Quantity unit Cash to Missing: TYPE_NAME has synthetic
  span 0:0. Expected the parameter type's original source span.
- Change state paid type to ScaledAmount<Missing,2>: TYPE_NAME has synthetic span.
  Expected the declared type's original source span.
- Change allocationId parameter type to Record<Missing>: TYPE_NAME has synthetic
  span. Expected the parameter type's original source span.
- Change RepayFields nominalAmount type to UInt128: OPERATION_BINDING points to
  the entire unrelated TransferFields record. Expected the malformed Repay
  declaration/field (or its operation binding) in the original source.

The first three arise because parameter types and some indexed type forms go
straight to validateSchema without source-aware reference checking. Please cover
all supported declared type forms, including nested forms. Preserve the shared
binding definition; locate the offending operation/record without a second
divergent contract definition. Add focused regression cases.
