# Task2 root observations before author terminal

Recheck these observations against the terminal candidate; this is not a final audit.

## R1: legacy TypeScript result contract broadened

The live change widens existing exported `FundedExpressionPrepared.financialPost` and `effects` to lifecycle unions. Every old source/Core factory still returns the shared `FundedExpressionResult`, so a valid source4 TypeScript consumer with exhaustive Transfer/Repayment handling no longer compiles: TS2322 sees new Origination/Accrual effect kinds even though old profile4 cannot produce them. This violates the old APIs' frozen result contract.

Reproduce with `python3 /home/charl/Moriarty/deliverables/language-to-ledger-2026-09-12/origination/legacy-type-probe.py /home/charl/Moriarty/.worktrees/loan-origination-accrual`. The immutable postconditions baseline compiles; the live candidate reports the exhaustive-client error. Initial receipt: legacy-type-probe-01.json. A separate in-progress compiler cast error also appeared; normal author typecheck should handle it.

Preserve old public FundedExpressionPrepared/Result shapes and old source1–4/Core1–3 return types. Give lifecycle execution its own precise result type, using internal generic/union factoring or overloads where needed without widening old factories. Source5/Core4 may return the new lifecycle effect set. Do not fix the probe with a cast, weaken the consumer, or modify old source profile output behavior.

Root independent kernel checks passed37 while the candidate was live. New source fixtures/probes in this directory pin independently counted origin55/accrual21 work and source final-debit RESULT_BOUND; these have not yet run against the new source5 entry. Preserve all original receipts and run current probes after implementation is complete.
