# Task3 author notes — complete source lifecycle

Requested model: Grok 4.6 high. Runtime identity: Grok 4.6 (this session).
Worktree: `/home/charl/Moriarty/.worktrees/loan-lifecycle`
Reviewed merged baseline: `a3ada9d9f4c7b6bf41b5503a2eb1d308ba55cbdd` (PR6)
Planning head at launch: `143ba30696fa4d26209ffd80ef4b0d0497bbdf7e`
Task2 source commit named in the launch: `dee7b4768b16138172840d5da83946687d875e1e`
Task2 audit: `origination/audit-origination-01.json` APPROVED, candidate `2a3a69e6170e000f3b9c85991868bcfc1ed080266f96434569163ca8ddf47a61`

No commit, push, PR, subagent, K, Docker, Preview, service, or wallet work was performed. Plugin `status --json` showed dependent loan-swap dispatch blocked by binding/accounting/resource gaps. This local source task used the supplied worktree. Latest task routing was applied; stale Opus queues were not followed.

## Red

`author-red-01.txt` records `npm --prefix experiments/moriarty-language test` against the new tests with no implementation files. Package tests were 874 pass / 1 fail. The new file failed at load (`ERR_MODULE_NOT_FOUND`) for `examples/loan-lifecycle.mjs`. Focused `node --test tests/loan-lifecycle.test.mjs` reproduced that missing-module failure. That is the intended red.

## Implementation

One source/5 agreement `spec/successor/examples/loan-lifecycle.mori` with actions `originate`, `accrue`, `repay`, `settle`. Ordinary state is `phase` and `paid`. The public consumer `examples/loan-lifecycle.mjs` loads that source once and calls `createFinancialAgreementSourceV5().evaluate`. After a successful result, the next call uses that result's ordinary `post`, `financialPost`, and `work.remaining`. Only the action name and Args change. Obs is empty. Settlement Args are IDs only; the source reads `outstanding<Cash>("Loan1")`, requires magnitude > 0, and derives Amount from that magnitude.

Seed work is remaining 512 / spent 17 / reserve 16. Kernel/language semantics and frozen profiles were not edited. Task2 `financial-lifecycle-demo` remains a partial originate/accrue/repay consumer.

## Constructor counts and source digest

Source SHA-256 (UTF-8 file bytes): `a0efd5166fdf27106110c69548d3b3065070e8d759d231296c454d19757bbaaa`

Counts are from the elaborated Core constructors of the frozen body, split at the first Ensure. N is kernel operations 2/1/2/2. StaticWorkBound equals Eprefix+Esuffix and excludes N. These are not evaluator remaining values.

| Action | statements (prefix then suffix) | Eprefix | N | Esuffix | W | remaining | spent |
| --- | --- | --- | --- | --- | --- | --- | --- |
| originate | Let2, Next4, Emit8, Emit22; Ensure 6,4,5,5,5,6,6,6,6,6,6 | 36 | 2 | 61 | 99 | 413 | 116 |
| accrue | Next4, Emit6; Ensure 6,4,5,5,5,5,6,6,6,6 | 10 | 1 | 54 | 65 | 348 | 181 |
| repay | Require5, Let3, Require4, Next4, Next4, Emit8, Emit7; Ensure 6,6,5,5,5,6,6,6,6 | 35 | 2 | 51 | 88 | 260 | 269 |
| settle | Let3, Let3, Require4, Next4, Next4, Emit8, Emit7; Ensure 6,6,5,5,5,6,6,6,6 | 33 | 2 | 51 | 86 | 174 | 355 |

Cumulative Ci = 99, 164, 252, 338. Remaining+spent stays 529; with reserve 16 the total is 545. Exact W succeeds with remaining 0. W-1 is WORK_EXHAUSTED. Prefix+N-1 is INSUFFICIENT_WORK. Prefix+N is WORK_EXHAUSTED. Late false Lender111 ensure uses diagnostic workUsed 86 and publishes no candidate.

## Checks

Recorded in `author-checks-01.txt`. All listed commands exited 0:

- `npm --prefix experiments/moriarty-language test` — 883 pass, 0 fail
- `npm --prefix experiments/moriarty-language run typecheck`
- Root README `/5` check/format/simulate for Task2 `financial-lifecycle-payment` and Task3 `loan-lifecycle`
- `npm --prefix experiments/moriarty-language run financial-lifecycle-demo`
- `npm --prefix experiments/moriarty-language run loan-lifecycle-demo`
- Task2 `kernel-probes.mjs`, `source-probes.mjs`, `readme-probes.py`
- Task1 `postconditions` independent/core/continuation/rejection-ownership/legacy probes
- Task2 `legacy-type-probe.py`, `legacy-types-all-probe.py`, `audit-origination-01-probes.mjs`

Root independent exact-work verification and a fresh full candidate audit are not claimed here. This is Task3 author evidence only. It is not a source audit, product acceptance, K correspondence, Docker, Preview, or ledger settlement result.
