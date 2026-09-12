# Task 1 author repair 01

Observed before author terminal; recheck against terminal candidate.

## R1: exported prefix leaks a callable staging continuation

`src/successor/financial-expression-v1.ts` exports `evaluateFinancialExpressionV3Prefix` and `V3PrefixReady`. Any caller can import the function, receive ordinary post/descriptors and `continueSuffix`, and supply an arbitrary `financialPost` and kernel action count. The frozen OpenSpec requires private staging and no public post-state/callback/resumable token API.

Reproduction: run `node /home/charl/Moriarty/deliverables/language-to-ledger-2026-09-12/postconditions/continuation-probe.mjs /home/charl/Moriarty/.worktrees/financial-postconditions`. Real repayment rejects an ensure that debt remains 100 (`ENSURES_FAILED`, workUsed54); the exported prefix accepts original financial PRE as alleged POST and returns `SuffixReady`. This is a public staging-contract violation; the standard source/Core factory itself correctly rejects, and this is not evidence of ledger publication.

Keep the same logical machine and one kernel call, but contain the staging function/type/closure inside the module or integrated evaluator. Export only a complete primitive-string integrated evaluation boundary for new execution. Do not merely rename the exported helper, label it internal, hide it in a different importable module or add a caller-supplied token. Preserve original profile behavior, exact work, error order and no callbacks. Add a meaningful regression for module export/continuation isolation and rerun all checks.

Root independent in-progress results: 17 financial, 25 Core, 261 historical comparisons passed; four actual README commands and canonical grammar passed. Preserve those semantics while repairing R1. No current final audit has been issued.

## R1 update and R2: rejected outputs share mutable nodePath

Author independently moved prefix helper private during its live turn; continuation-probe now passes. Preserve the earlier observed failure but do not repeat a solved repair.

New reproduction: rejection-ownership-probe.mjs against the checkout. Obtain normal FINANCIAL_CONTEXT_REQUIRED from a typed valid action, push `999` onto its returned nodePath, then evaluate again or create an independent factory. Both new results inherit ["999"] instead of required []. `V3_ENVELOPE` is a module-level shallow spread with one mutable array. Return owned diagnostic arrays each time (or immutable outputs without shared mutable state). Add regression through valid action/missing context and separate factories. No old profile output contract changes are required.

## R3: profile reserved-name documentation

New spec/successor/financial-agreement-source-v4.md Declarations says both the six PRE and six POST names are reserved only in /4 and remain legal identifiers in /1–/3. PRE names were already reserved in /3. Correct to: six PRE names stay reserved in /3 and /4; six new POST names reserved only in /4 and remain legal in /1–/3. No source behavior change. This is an observed documentation defect within Task1 scope.
