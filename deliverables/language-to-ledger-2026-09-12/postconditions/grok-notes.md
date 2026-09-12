# Task 1 author notes

Requested model: grok-4.6 high. Terminal: Node v24.18.1, Linux 6.18.33.2-microsoft-standard-WSL2 x86_64, branch feat/financial-postconditions, HEAD 02b21594 (uncommitted language+README work). Not a final audit. Task 2 was not started.

## Initial red

`author-red-01.txt`: three new test files failed `ERR_MODULE_NOT_FOUND` for missing `financial-agreement-source-v4.ts` and `financial-expression-v3.ts`.

## Implementation

Source `/4` and Core `/3` with six Ensure-only `ReadPost*` constructors. All ensures run after one private kernel preparation. Unprefixed reads keep PRE meaning. Public APIs stay primitive-string. Core `/3` evaluate is funded and action-only.

## Fixes kept

1. Tests that put `ensures` before `next` hit parser `STATEMENT_AFTER_ENSURES`; those cases were rewritten as suffix ensures.
2. Core `/3` factory key order is `evaluate` then `check`, matching `/2`.
3. `prepareStagedKernel` rejects empty descriptors as `EMPTY_BATCH` before operation binding.
4. Trailing-newline identities: Core `/3` uses an explicit full-string check; Core `/2` already rejects them under its existing non-multiline `$` regex. The `/2` contract was not changed.
5. Root R1: `evaluateFinancialExpressionV3Prefix` was a public continuation. Staging is now private inside `createFinancialExpressionContractV3`. Continuation probe returns `{"pass":true,"publicPrefixExport":false}`.

## Checks after R1

- `npm --prefix experiments/moriarty-language test`: 834 pass, 0 fail
- `npm --prefix experiments/moriarty-language run typecheck`: pass
- README `check` / `format` / `simulate --action repay` / `financial-postconditions-demo`: success; remaining work 82, exact 82 remaining 0 reserve 16
- False first financial ensure CLI: exit 1, empty stdout, `ENSURES_FAILED` workUsed 54

Existing `/1`–`/3` source and `/1`–`/2` Core tests remained in the 834. Stopped before Task 2.
