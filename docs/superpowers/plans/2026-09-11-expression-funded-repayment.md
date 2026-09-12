# Expression-funded repayment implementation plan

> Execute one bounded product slice using Grok 4.6 high. A fresh GPT-6 Astra medium audits the complete candidate.

**Goal:** Run computed, guarded `.mori` repayments through the normal simulation CLI with atomic ordinary and financial results.

**Architecture:** Reuse the financial expression source evaluator and funded repayment kernel. A small adapter validates the binding, evaluates source, maps ordered descriptors, and prepares the financial transition. Publish combined output only after all steps succeed.

**Tech stack:** Existing TypeScript, Node.js, JSON, and node:test. No new dependency.

## Scope and decision

The user approved this slice and requested planning, Grok implementation, and Astra medium audit on September 11.
This task-specific review roster supersedes prior general model routing for this item.
This implements a bounded SP02/SP03 increment. Full source-defined schemas, multiple actions, K correspondence, authority authentication, proofs, and Preview remain open.
The existing campaign admission failure blocks dependent live dispatch only. Do not repair it for this local source task.

Alternatives: extending the old funded parser would duplicate expression semantics. Changing pure expression evaluation would silently change its contract.
Use an explicit funded adapter and CLI option with the existing financial source syntax instead.

## Interfaces

Create `experiments/moriarty-language/src/successor/funded-expression-source-v1.ts`.
Export `createFundedFinancialExpressionSourceV1(schemaCanonicalJSON: string)`.
Its `evaluate(source: string, snapshotCanonicalJSON: string, repaymentStateJSON: string)` consumes source text and owned parsed JSON only.
Reuse `createFinancialExpressionSourceV1` and `prepareRepayment`; do not implement a second financial transition engine.
Successful output is a distinct local prepared result containing ordinary post, full financial post, ordered actual effects, and remaining work.
Use `status: 'FundedExpressionPrepared'`, `post`, `financialPost`, `effects`, and `workRemaining` as its public fields.
A rejected result contains diagnostics only. It must not contain candidate post, financialPost, descriptors, or effects.

Modify `experiments/moriarty-language/src/cli.ts`.
Extend normal `simulate --profile moriarty-financial-expression-source/1 --schema SCHEMA --snapshots SNAPSHOTS --repayment-state STATE SOURCE`.
Keep the existing argument order, placing the optional repayment-state pair immediately before SOURCE.
The new option is legal only for this profile and simulate. Other profile and command combinations reject.
Preserve pure simulate, check, format, stdout/stderr conventions, file type checks, and byte limits.
Read STATE as bounded UTF-8 regular-file JSON; it is a bare RepaymentState object.
Success exits zero with initial ordinary state, initial financial state, initial work, and the combined result.
Rejection exits one with JSON on stderr and empty stdout. CLI misuse or I/O failure retains exit two.

## Binding and atomicity contract

Source inspection correction: `amount` is reserved metadata in the existing financial source grammar.
Use `transferAmount` in source records; retain kernel `amount`. No grammar change is needed.

Use the current trusted financial expression schema. Reject financial-class schema fields in this adapter.
Pre/post describe ordinary application fields only; financial state is the separate complete repayment projection.
Source ensures evaluates ordinary staged post before financial effects. It does not assert final financial postconditions.
Support only exact Transfer and Repay operation records. Reject missing, unknown, or malformed operation bindings.
Transfer fields: id/from/to/settlementAsset are Text; transferAmount is Amount<asset>.
Repay fields: allocationId/transferId/obligationId/payer are Text; nominalAmount is Quantity<Units<denomination,1>,0>.
Check the Transfer amount type asset against each emitted settlementAsset value.
Check the Repay quantity unit against the selected obligation denomination.
Require nonnegative nominal values. Quantity limits this adapter to signed-128 positive range; do not broaden the kernel or source type system.
Text identifiers remain subject to kernel identifier and relationship checks.
Map settlementAsset to kernel asset and transferAmount to kernel amount. Consume all descriptors in order through ONE kernel invocation.
Reject unsupported operations, empty batches, unfunded allocations, reused IDs, and mismatched parties/assets/units.
Do not publish tentative ordinary writes if descriptor binding or the kernel rejects.
Preserve unrelated balances, allowances, obligations, conversions, ordering, and tombstones.

## Work contract

Require snapshot workInitial == financial work.remaining. Do not reset carried work or allowance spent.
Expression evaluator uses this exact remaining value, within its existing 65536 ceiling.
The kernel reserve is already separate from remaining. Do not subtract or spend closureReserve.
For actual expression reductions E and N emitted kernel actions:
`remainingAfter = remainingBefore - E - N`
`spentAfter = spentBefore + E + N`
`closureReserveAfter = closureReserveBefore`
Pass a fresh state with expression work debited to one complete kernel call.
The kernel validates the full original-equivalent projection and final transition before publication.
Do not run the kernel with an empty batch for validation: its public contract rejects empty actions.
Do not charge the static expression bound or debit either stage twice.

## Task 1: Adapter and financial behavior

- [ ] Add `tests/funded-expression-source.test.mjs` under the language package.
- [ ] First run the focused tests before the module exists and retain the failing output.
- [ ] Implement the adapter and exact schema/descriptor boundary.
- [ ] Verify computed due100/pay30: payer70, lender30, principal70, allowance remaining70/spent30.
- [ ] Verify AccrualFirst P100/I10/pay7: P100/I3, outstanding103, cash debit7, principalDischarged0/accruedDischarged7.
- [ ] Verify arithmetic and a guard genuinely affect the value reaching Transfer and Repay.
- [ ] Verify failed guard/ensure after staged write or emission returns rejection only.
- [ ] Verify overflow, negative nominal, wrong units, unsupported operations, missing funding and repeated funding reject atomically.
- [ ] Verify exact work at E+N, one-unit short, nonzero previous spent, and unchanged reserve.
- [ ] Verify multiple allocations share one transfer's decreasing funding.
- [ ] Verify a second invocation preserves prior spent, IDs, unrelated state, and residual duties.

## Task 2: Normal CLI and runnable example

- [ ] Add `tests/funded-expression-cli.test.mjs` with actual process invocations.
- [ ] Implement the option and normal CLI consumer.
- [ ] Add `spec/successor/examples/expression-funded-payment.mori` plus schema, snapshots and state JSON files with matching stem.
- [ ] Include computed payment, guard and ordinary staged update in the example.
- [ ] Test success, semantic rejection with empty stdout, wrong profile, malformed and oversized inputs, and unchanged pure CLI behavior.
- [ ] Add `spec/successor/funded-expression-source.md` documenting exact schema, API, CLI, work, bounds and local-only scope.
- [ ] Add a short runnable entry to the language package README if present; otherwise link from the spec README.

## Task 3: Independent checks and fresh audit

Run from the worktree root:
```sh
node --test experiments/moriarty-language/tests/funded-expression-source.test.mjs experiments/moriarty-language/tests/funded-expression-cli.test.mjs
npm --prefix experiments/moriarty-language test
npm --prefix experiments/moriarty-language run typecheck
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-financial-expression-source/1 --schema experiments/moriarty-language/spec/successor/examples/expression-funded-payment.schema.json --snapshots experiments/moriarty-language/spec/successor/examples/expression-funded-payment.snapshots.json --repayment-state experiments/moriarty-language/spec/successor/examples/expression-funded-payment.state.json experiments/moriarty-language/spec/successor/examples/expression-funded-payment.mori
```
- [ ] Root checks the real CLI and at least one independent adverse case.
- [ ] Freeze hashes for the complete language source/spec/test candidate.
- [ ] Fresh Astra medium audits complete source, contract, tests and actual checks.
- [ ] Send findings to the same Grok author; changed bytes need a fresh candidate audit.
- [ ] Integrate only task changes after verifying main checkout baseline hashes.
- [ ] Report actual behavior, audit identity, check results and remaining scope.

## Constraints

Implement only under `experiments/moriarty-language/`. Keep author receipts in the task deliverable directory.
Do not change plugin, orchestration, native/Compact/K code, dependencies, wallets, services, wiki, or campaign records.
Preserve current user changes. Do not commit or publish from the author process.
No new public transactions or expensive proving runs are part of this task.
