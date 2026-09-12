# Multiple named actions implementation plan

> Execute the user-approved bounded capability through the existing workflow. Grok4.6 high implements; fresh GPT-6 Astra medium audits the complete final candidate. Use executing-plans for implementation.

**Goal:** Author multiple named actions in one source-defined financial agreement, statically check all of them, and explicitly select one for funded simulation.
**Architecture:** Add a separate /2 profile and wrapper API. Share declaration compilation with /1; derive an independent schema.args and checked Core for each action. Reuse the existing evaluator, funded preparation and repayment kernel.
**Tech stack:** Existing TypeScript, Node.js, node:test and CLI. No dependencies or infrastructure additions.

## Authority and scope

The user approved the proposed next step and explicitly said: Begin, arm a loop to complete it, same workflow.
The native goal is active. Routine implementation, repairs, review, integration and publication are authorized.
Baseline commit: ea40ab488d1d056c7192cc4dfd7ae250d44e3d49, source-defined repayment.
Worktree: /home/charl/Moriarty/.worktrees/multiple-named-actions.
Branch: feat/multiple-named-actions.
Evidence: /home/charl/Moriarty/deliverables/multiple-named-actions-2026-09-11.
No new financial operations, multiple actions executed in one invocation, source initialization, function calls between actions, K/native/proving/Preview work, plugin changes or unrelated roadmap work.
Existing live admission/accounting stops remain; local source development is authorized independently.
Copied AGENTS/README/ROADMAP/plugin/routing files are current guidance only. Do not edit or commit their pre-existing differences.

## Design decision

Root and independent Astra medium reviewed the current compiler seam and agree on a distinct `moriarty-financial-agreement-source/2` profile.
Extending /1 in place would change its existing single-action rejection and API contract. Separate source files per entry point would not deliver shared contract authoring.
Reuse one declaration compiler and protected binding definition. Do not copy the complete /1 compiler into a second implementation, strip source text, regenerate an artificial /1 program, add a second evaluator, or execute caller-supplied Core.
Keep original source, complete UTF-8 offsets and declaration order throughout compilation.

## API and artifact contract

New module `experiments/moriarty-language/src/successor/financial-agreement-source-v2.ts` exports `createFinancialAgreementSourceV2()`.
New frontend module `src/successor/financial-agreement-source-v2-frontend.ts` exports:
- `FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE = 'moriarty-financial-agreement-source/2'`
- `parseFinancialAgreementSourceV2(source)`
- `formatFinancialAgreementSourceV2(source)`
Parser/formatter additions use the existing frontend and format implementation, gated by the new profile.

The frozen factory has these public methods:
```ts
elaborate(source: string)
check(source: string)
evaluate(source: string, actionName: string, snapshotCanonicalJSON: string, repaymentStateJSON: string)
```
Elaborate returns either the existing SourceRejected shape or:
```ts
{
  judgmentResult: 'SourceElaborated', sourceProfile, contract, agreement,
  actions: [{ action, schema, core, staticWorkBound }]
}
```
Check returns either SourceRejected or:
```ts
{
  judgmentResult: 'SourceChecked', sourceProfile,
  actions: [{ action, staticWorkBound }]
}
```
Action arrays are in source declaration order. No aggregate staticWorkBound may masquerade as the selected runtime bound.
Each action schema shares the same declared units/assets/records/fields/operations and has only that action's parameters in args. Returned objects are owned inspectable artifacts; later evaluate compiles fresh and never consumes them.
Evaluate returns the unchanged FundedExpressionResult shape. Selection is mandatory even when /2 contains one action. No implicit first/only action fallback.

Compilation phases for /2:
1. Validate primitive bounded source, parse full original source.
2. Collect and validate shared declarations, names, types, cycles and protected bindings.
3. Build and validate every action schema, parameters and checked Core in declaration order with fresh lowering/local scope.
4. Validate selector and perform exact case-sensitive lookup using array.find/Map or own properties.
5. Parse owned repayment state, enforce work match, validate selected snapshots and execute existing funded preparation.

Static errors in any action reject before selector or runtime-input handling. Well-typed unselected runtime guards/ensures do not execute.
Malformed/missing/non-string/empty selector rejects with `SOURCE_ACTION_NAME`; a valid identifier absent from the action set rejects with `SOURCE_ACTION_UNKNOWN`. These are external selector errors and use a synthetic span, empty nodePath and workUsed0. Do not invent source token locations.
Selectors obey the existing ASCII identifier length64 rule. `constructor`/`toString` must not resolve by prototype inheritance, and invalid `__proto__` rejects. Avoid getters/coercion on non-string inputs.
Preserve /1 public signatures, result fields, diagnostic codes/spans and validation ordering, including exactly-one-action rejection. Existing expression/syntax profiles remain unchanged.

## Names, scopes and bounds

Retain separate unit/asset same-name allowance and all existing shared declaration collision rules.
Duplicate action names, or an action colliding with another top-level declaration, reject at the offending declaration.
Zero /2 actions rejects SOURCE_ACTION_COUNT. Reuse totalDeclarations256, source65536 bytes, token8192, AST8192, nesting64, statements256 per action, parameters256 and existing schema bounds. These compose; no unbounded action dictionary or relaxed bound.
Each action gets independent parameters and locals. Same names across actions, including different types, are allowed. References to another action's bindings reject.
Retain existing parameter exclusions against shared metadata and reserved terms; action entry labels are not callable values and do not introduce new expression name bindings.
Validate every action even if its arguments are absent from the selected snapshot. Runtime snapshots require exactly selected Args; never union all arguments or demand unselected values.
Preserve original source diagnostics for errors in later actions after comments/Unicode, including parameter names/types and shared schema names/operation cycles.

## Financial and work invariants

Require the same exact Transfer and Repay bindings during static checking. No weakened funding, unit, numeric range, identifier, allowance, residual-debt or protected operation checks.
Execute exactly one selected action, then one ordered kernel call. Preserve atomic combined ordinary/financial publication and no output leakage on any rejection.
Charge selected actual expression reductions E plus its kernel action count N only. Extra unselected actions and source ordering add no runtime debit. Compilation/checking is not runtime work.
Carry complete financialPost including unused balances/obligations, allowances remaining/spent, transfer/allocation tombstones, cumulative spent, remaining work and unchanged separate closure reserve.
Source ensures observes ordinary post only. Ordinary fields never substitute for the kernel projection.

## CLI contract

From repository root:
```sh
node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-agreement-source/2 SOURCE
node experiments/moriarty-language/src/cli.ts format --profile moriarty-financial-agreement-source/2 SOURCE
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-financial-agreement-source/2 --action NAME --snapshots SNAPSHOTS --repayment-state STATE SOURCE
```
The ordering above is the exact accepted positional form, consistent with existing CLI parsing.
/2 rejects --schema; check/format reject --action; simulate requires --action once. Existing profiles reject the new selection flag and retain existing forms/envelopes.
Successful /2 simulate emits SourceSimulated with sourceProfile, action, pre, financialPre, initialWork and result. Only /2 adds action. Semantic rejection: exit1, JSON stderr, empty stdout. Usage/I/O: existing exit2 contract. Keep bounded regular-file and UTF-8 reading.

## Demonstration fixture

Create `spec/successor/examples/multiple-action-payment.mori` with the existing shared Cash/TransferFields/RepayFields/due/paid declarations and two actions:
- `repay(nominal: Quantity<Units<Cash,1>,0>, transferId: Text, allocationId: Text)` checks nominal nonnegative, magnitude positive, updates paid and emits funded Transfer+Repay.
- `repay_installment(transferId: Text, allocationId: Text)` defines local nominal as `quantity<Units<Cash,1>,0>(20)` and performs the same bounded funded payment.
Use `nominal`, not reserved parameter name `amount`.
Create `multiple-action-payment.snapshots.json` with existing due100/paid0/Obs{}, workInitial100 and only repay's Args (nominal30,T1,Alloc1).
Reuse `expression-funded-payment.state.json` as initial repayment state. Document continuation with distinct T2/Alloc2, Args only for repay_installment, Pre=first.post and state=first.financialPost, workInitial=first.workRemaining.
Expected first principal/outstanding70, payer/lender70/30, allowance remaining/spent70/30. Second principal/outstanding50, payer/lender50/50, allowance50/50, both identifier pairs retained. due remains ordinary bookkeeping; financial projection supplies residual debt.
Determine actual E for each fixed body through existing reduction rules and compare the complete result to equivalent isolated /1 actions. Test fixture construction may isolate known action bodies; production code must not rewrite source.

## File ownership and implementation steps

Create the /2 API/frontend modules, `tests/financial-agreement-source-v2.test.mjs`, `tests/financial-agreement-cli-v2.test.mjs`, `spec/successor/financial-agreement-source-v2.md`, matching grammar and the example/snapshot fixture above.
Modify existing frontend.ts/format.ts/cli.ts and minimally refactor the /1 compiler into a shared internal module such as `financial-agreement-source-compiler.ts` if needed. Existing /1 wrapper adapts shared results without behavior changes.
Update language package README and relevant successor profile/CLI documentation with exact working-directory commands and bounded scope. Root README/ROADMAP are outside author write scope.

- [ ] Write focused failing API/CLI tests and save the actual red run before production changes.
- [ ] Refactor shared declaration compilation and preserve all existing /1 tests.
- [ ] Add /2 compilation of all actions with distinct Args/scopes and original spans.
- [ ] Add mandatory primitive selector and selected-only evaluation, then normal CLI consumer.
- [ ] Add fixture and commands proving repay30 followed by installment20 leaves50.
- [ ] Verify duplicate/zero/unknown actions, case sensitivity, hostile selector objects and prototype names.
- [ ] Verify unselected static error rejection, unselected runtime guard isolation, parameter/local reuse and cross-action leakage rejection.
- [ ] Verify exact selected Args, order-independent selected results/work, extra unselected action work invariance and bounds.
- [ ] Verify protected bindings, funding/guard/ensure/work rejection and full continuation projection.
- [ ] Verify /1 API signatures/envelopes/errors plus old CLI rejection/acceptance forms.
- [ ] Verify formatter idempotence, source span correctness and full-result equivalence.
- [ ] Run full package tests/typecheck and independent root CLI/API probes.
- [ ] Freeze complete candidate source+plan hashes; obtain fresh Astra medium full-path audit. Repair through the same Grok author and repeat exact-candidate review when bytes change.
- [ ] Commit only task changes, integrate against checked main baseline, publish stacked on feat/source-defined-repayment, and close the native goal only after acceptance is complete.

## Verification commands

```sh
node --test experiments/moriarty-language/tests/financial-agreement-source-v2.test.mjs experiments/moriarty-language/tests/financial-agreement-cli-v2.test.mjs
npm --prefix experiments/moriarty-language test
npm --prefix experiments/moriarty-language run typecheck
node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-agreement-source/2 experiments/moriarty-language/spec/successor/examples/multiple-action-payment.mori
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-financial-agreement-source/2 --action repay --snapshots experiments/moriarty-language/spec/successor/examples/multiple-action-payment.snapshots.json --repayment-state experiments/moriarty-language/spec/successor/examples/expression-funded-payment.state.json experiments/moriarty-language/spec/successor/examples/multiple-action-payment.mori
```

After two same-class failed corrections, reproduce the smallest actual defect and change the approach before another broad retry. Missing authors/auditors are not approval. Keep exact failed receipts. No new infrastructure or routine permission loop.
