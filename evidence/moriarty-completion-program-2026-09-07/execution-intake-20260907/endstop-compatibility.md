# Moriarty Endstop compatibility inspection

Verdict: no complete supported configuration was found in the inspected interfaces. The first actionful worker remains blocked by the required runtime predicates.

This is a bounded source inspection. It is not a program-plan review, MC01 result audit, runtime test, or approval gate.
No contract, ledger event, model call, runtime change, or Foreman repair was created. Only these requested reports were written.
The installed manifest digest supplied by the parent is `ba16689953d0ba08a857a3b1e492ea6a7731a677bde54805730124d1a0b3d4f9`. This inspection did not reverify that digest.

## Source observations

| Predicate | Finding | Exact source evidence |
|---|---|---|
| Immutable external contract | Existing V1 contract creation rejects a different digest for an existing contract. | `/home/charl/foreman/packages/orchestration/src/execution-ledger.ts:1594` |
| Custom cumulative resource fields | V1 allows only fixed action counters, `wallTimeMs`, and `noProductChangeMs`. The decoder rejects additional limit keys. | `/home/charl/foreman/packages/orchestration/src/execution-contract.ts:126`, `:155` |
| Cumulative subprocess time | V1 checks the current timestamp against `deadlineAt` when reserving an action. It does not measure accumulated subprocess duration. | `/home/charl/foreman/packages/orchestration/src/execution-terminal-policy.ts:247` |
| Historical debit | Initial counters are zero. The command union contains no debit-import operation. | `/home/charl/foreman/packages/orchestration/src/execution-terminal-policy.ts:69`, `:157`, `:170` |
| Generic protected package budgets | V1 dependencies require completed contracts. They are prerequisite checks, not parent budget reservations. | `/home/charl/foreman/packages/orchestration/src/execution-ledger.ts:1720` |
| Generic V2 family | V2 accepts only the release programs `v040` and `v050`. It requires a fixed family identity, limits, and exact predefined children. | `/home/charl/foreman/packages/policy/src/release-program.ts:1`, `/home/charl/foreman/packages/orchestration/src/execution-contract.ts:976`, `:1028`, `:1040` |
| Queue deadline enforcement | V1 reserves an action at admission, then calls queue submission. It does not install an Endstop deadline timer around execution. | `/home/charl/foreman/packages/orchestration/src/queue-cli.ts:358`, `:390` |
| Nested queue lifetime | Queue submission invokes `pueue add`, prints the task ID, and returns success. It does not wait for that task. | `/home/charl/foreman/packages/orchestration/src/queue-admission.ts:701`, `:720` |
| Existing process timeout | The launcher accepts a relative timeout and waits an additional grace interval before terminating the process tree. This timer does not read Endstop remaining time. | `/home/charl/foreman/packages/launcher/src/supervise.ts:260` |
| Evidence bytes and Preview gross debit | Neither field occurs in the closed V1 resource schema or execution state. Free-text acceptance hashes do not enforce these counters. | `/home/charl/foreman/packages/orchestration/src/execution-contract.ts:28`, `:126`, `/home/charl/foreman/packages/orchestration/src/execution-terminal-policy.ts:45` |

## Conservative V1 composition

Reducing a new deadline by known historical debits does not require fictional events. It can conservatively reduce future permission.
The supplied debit plus reserve is 2,623.047330504 seconds. The master remainder is 26,176.952669496 seconds, before subsequent chargeable work.
V1 requires whole-second UTC timestamps. Rounding a proposed interval down to 26,176 seconds would preserve that ceiling.
The timestamp requirement appears at `/home/charl/foreman/packages/orchestration/src/execution-contract.ts:253`. The exact deadline interval requirement appears at `:266`.

This arithmetic alone does not establish runtime enforcement. All actionful processes would require global serialization and termination before the reduced deadline.
Each command would also need admission against its full bound and remaining mandatory review reserves.
The charter requires those reservations at `/home/charl/Moriarty/openspec/MORIARTY-COMPLETION-PROGRAM.md:203`.

Nested root-bound and package-bound queue calls do not supply this supervision. The outer queued task finishes when the inner task is submitted.
The inner task can remain queued after the parent task exits. It can also execute after the parent's deadline.
A relative launcher timeout limits one process. It does not bind that process to both immutable contract deadlines or enforce aggregate resource counters.
Binding deadlines, waiting for queued descendants, reserving budgets, and reconciling outcomes would require additional harness logic unless another existing supported component is identified.
That component was not found in this bounded inspection. No harness development is authorized by this report.

V1 action counts can conservatively cap all admissions at 24. This also charges verification and audits against the worker allowance.
It cannot represent the required worker-only total while independently protecting all package allowances through the inspected nested composition.
Multiple independent V1 contracts can impose local action limits. They do not automatically enforce a shared master or byte/debit limits.

## Missing predicates and decision

Missing predicates are persistent cumulative subprocess accounting, historical debit representation, bound-before-launch reservation, and mandatory gate protection.
The inspected interfaces also lack a generic package family, combined worker accounting, retained-evidence byte limits, and Preview submission/gross-debit enforcement.
The authoritative requirements remain in `/home/charl/Moriarty/openspec/MORIARTY-COMPLETION-PROGRAM.md:178`, `:184`, `:188`, `:203`, and `:223`.

Smallest scoped decision: keep MC01 source-only architecture work moving and preserve the first-worker stop.
Before actionful dispatch, identify an already supported enforcing wrapper or explicitly authorize a bounded external enforcement change.
If the user instead changes the enforcement requirement, record that specific charter decision. Do not treat weaker V1 accounting as equivalent enforcement.
No new substantive review is requested by this inspection. Do not reopen the approved program plan or turn this blocker into a Foreman development sprint.
