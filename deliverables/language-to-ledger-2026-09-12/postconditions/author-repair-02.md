# Task 1 author repair 02

Requested model: grok-4.6 high. R1 remains closed. This repair is R2 only.

## Defect

`rejection-ownership-probe.mjs` obtained `FINANCIAL_CONTEXT_REQUIRED` from a typed action with no financial context, pushed `"999"` onto `nodePath`, then evaluated again and through a second factory. Both later results showed `nodePath: ["999"]` instead of `[]`.

Root cause: `v3Envelope` spread module-level `V3_ENVELOPE`, whose `nodePath: []` was one shared mutable array. Envelope shape was already `{status, code, span, nodePath, workUsed}`. Old `/1`–`/2` factories already allocated `nodePath: []` per rejection.

## Change

`v3Envelope` now returns a new `nodePath: []` on every call. Specified envelope fields and old-profile constructors are unchanged.

Regression: `missing-context rejections own nodePath across calls and separate factories` in `tests/financial-expression-contract-v3.test.mjs`.

## Checks

Package tests 835/835. Typecheck pass. Continuation probe pass. Rejection-ownership probe pass. Independent 17, Core 25, legacy 261, README 4 commands plus canonical `/4` EBNF. No commit, no Task 2, no final audit.
