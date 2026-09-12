# Task2 repair notes 02

Requested model: Grok 4.6 high. Runtime identity: Grok 4.6 (same author session).
Worktree: `/home/charl/Moriarty/.worktrees/loan-origination-accrual`
Original receipts preserved: author-red-01.txt, author-checks-01.txt, grok-notes.md, legacy-type-probe-01.json.

R1 reproduced before the patch: `legacy-type-probe.py` baseline exit 0, candidate TS2322 (`AccrualEffect | OriginationEffect` not assignable to `never`).

## R1

Restored exact old `FundedExpressionPrepared` / `FundedExpressionRejected` / `FundedExpressionResult` (repayment `financialPost` and Transfer/Repayment `effects` only). Source `/5` and Core `/4` now return `LifecycleExpressionPrepared` / `LifecycleExpressionResult` with lifecycle state and Origination/Accrual effects. Shared envelope is an unexported generic. Old factory signatures are unchanged. The independent probe was not edited.

Regression: `src/successor/legacy-funded-result-consumer.ts` (typechecked public `/4` exhaustive switch) and `tests/legacy-funded-result-types.test.mjs` (tsc of the probe consumer must pass; the same Transfer/Repayment-only switch on source `/5` must fail TS2322).

## R2

Root README now has runnable `/5` check, format, simulate and `financial-lifecycle-demo` commands. Scope text states the current demo originates 100, accrues 10, repays 30, leaves outstanding 80, and that full settlement is Task 3. Package README gained the format command. `readme-probes.py` was not changed.

## Checks (author-checks-02.txt)

- `npm test`: 874 pass, 0 fail, exit 0
- `npm run typecheck`: exit 0
- `legacy-type-probe.py`: passed true (baseline and candidate exit 0)
- `kernel-probes.mjs`: 37 pass, 0 fail
- `source-probes.mjs`: 13 pass, 0 fail
- `readme-probes.py`: canonical grammar matched, 4 commands passed

No commit, push, K/Docker/Preview, contract edit, or plugin/user-file change. Not a final audit or overall-complete claim.
