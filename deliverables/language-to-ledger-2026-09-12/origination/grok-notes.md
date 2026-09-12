# Task2 author notes — loan origination and interest accrual

Requested model: Grok 4.6 high. Runtime identity: Grok 4.6 (this session).
Worktree: `/home/charl/Moriarty/.worktrees/loan-origination-accrual`
Branch: `feat/loan-origination-accrual`
Baseline: Task1 `53f9b78981031f8c12759b422416eef78adc51fd`
Planning: `a7beda220ea727dcd8b296f8fd72c48460e1500d`
Contract SHA-256: `74299d10cca2874828c4c847f6817677b033090bb85d38bb898dd462fee98383`

No commit, push, PR, subagent, external model, package install, K, Docker, Preview, service, or wallet work was performed. Plugin status showed dependent loan-swap dispatch blocked; this native source work used the existing worktree.

## Red

`author-red-01.txt` records the first focused run against missing modules. Package tests were 835 pass / 4 fail. The four new files failed at load (`ERR_MODULE_NOT_FOUND`) for:

- `src/successor/financial-lifecycle.ts`
- `src/successor/financial-agreement-source-v5.ts`
- `src/successor/financial-expression-v4.ts`

That is the intended red: the APIs did not exist.

## Implementation

New kernel `prepareFinancialLifecycle` / `admitFinancialLifecycleStateJSON` in `financial-lifecycle.ts`. Versions: `moriarty-financial-lifecycle/1` and `moriarty-financial-lifecycle-state/1`. Source `/5` and Core `/4` reuse the Task1 private prefix/kernel/suffix path. Old repayment `/0`, source `/1`–`/4` and Core `/1`–`/3` stay on their previous factories.

## Repairs after green started

1. Transfer amount 101 against cash 100 failed `INSUFFICIENT_BALANCE` before amount mismatch. Test seed now funds 200 so `TRANSFER_AMOUNT_MISMATCH` is reachable.
2. Duplicate origination after a spent lender balance failed Transfer at index 0. Test restores lender cash/allowance before the second disbursement.
3. Product-overflow fixture put `usedAccrualIds` on an obligation record (unknown field, admission `actionIndex: null`). Fixture now mutates only obligation arithmetic fields.
4. Example/CLI snapshots had a trailing newline; `parseCanonical` requires exact compact JSON. Newline stripped.
5. Core `/4` test imported `FINANCIAL_EXPRESSION_CONTRACT_V1` from the `/4` entry. Import restored to the `/1` module.
6. Typecheck: Originate mapping now asserts through `unknown` onto `LifecycleAction`.

## Checks

- `npm test`: 871 pass, 0 fail, exit 0 (`author-checks-01.txt`)
- `npm run typecheck`: exit 0
- `npm run financial-lifecycle-demo`: originate 100, accrue 10, repay 30 with incurred 110 retained
- CLI check/format/simulate for profile `/5` match the API

Independent kernel oracle main trace: remaining/spent 254/19, 253/20, 251/22, 249/24, reserve 16. Source work adds prefix/suffix E (example originate spent 76 = 17 prior + 57 expression + 2 kernel). Result-bound fixtures: compact input 65516 rejects `RESULT_BOUND`; 65508 prepares post 65536 and re-admits.

This is Task2 implementation evidence only. It is not a final audit, ledger acceptance, K correspondence, or overall-complete claim.
