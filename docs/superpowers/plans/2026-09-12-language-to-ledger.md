# Language-to-ledger Implementation Plan

> **For agentic workers:** Use executing-plans for the assigned task only. Grok4.6 high implements; fresh Astra medium audits. Root integrates.

**Goal:** Deliver financial postconditions, originated/accruing loans, a complete lifecycle, scoped K agreement, authenticated Docker/Preview execution and accurate roadmap status.

**Architecture:** Preserve frozen profiles through explicit new versions. Reuse one staged expression machine and protected financial kernel. Carry its reviewed meaning into K and the actual Midnight caller.

**Tech Stack:** Existing Node24 TypeScript, OpenSpec, pinned K, Compact, Midnight SDK and Docker. No new scheduler or service framework.

## Global constraints

- Normative scope: openspec/changes/language-to-ledger-lifecycle/ and current user instruction.
- Baseline main229545fada12c151ed5693f4e6cebdab956bd222 includes the four merged language PRs.
- Existing dirty roadmap, plugin, OpenSpec and vault work remains preserved.
- Native goal owns continuation. Each task has actual checks, fresh exact-candidate audit, bounded repairs and scoped publication.
- One implementation writer owns shared language files at a time. Independent expectations and reviews may run concurrently.
- Primitive input ownership, closed schemas, exact units, bounded arithmetic and work remain required.
- Closure reserve is separate from spendable work. No operation resets spent counters, liabilities or success identities.
- Local projection validation is not signature, time or ledger authentication.
- No public action runs without current plugin admission, bounded resources and positive funding/attempt evidence.
- After two same-class failures, reproduce the defect and change approach before another broad correction.
- Required K, Docker and Preview failures remain incomplete; passing source tests never replace them.

## Delivery order and acceptance records

| Order | Task | Observable completion | Evidence directory |
| --- | --- | --- | --- |
| 0 | Reconcile earlier local scope | Four merged capabilities documented; wider gates remain open | reconciliation/ |
| 1 | Financial postconditions | Actual resulting debt/balances/allowances checked, atomic failure | postconditions/ |
| 2 | Origination and accrual | Source creates funded debt and accrues exactly once per eligible period | origination-accrual/ |
| 3 | Complete lifecycle | Source-driven debt0→100→110→80→0, complete conserved state | lifecycle/ |
| 4 | K agreement | Fresh complete positive/negative observations match evaluator and oracle | k-agreement/ |
| 5 | Ledger integration | Authenticated Docker lifecycle, then admitted finalized Preview with raw exit | ledger-integration/ |

All directories are under deliverables/language-to-ledger-2026-09-12/. Stage0 documentation can proceed while task1 runs. Task5 source exploration can proceed without public admission; its dispatch cannot.

## Task 0: Reconcile merged local capabilities

Files: ROADMAP.md, new openspec/changes/language-to-ledger-lifecycle/, this plan, reconciliation/receipt.json.

- [ ] Add scoped entries for PR1 e6dc9f68, PR2 ea40ab48, PR3 96860344 and PR4 98f6d59f.
- [ ] Link original RESULT and final audit receipts. Record merge dc9d516e.
- [ ] Replace the stale statement that source-defined schemas and multiple actions remain absent.
- [ ] Preserve all unrelated user roadmap amendments and full MC/SP acceptance checkboxes.
- [ ] Run openspec validate language-to-ledger-lifecycle --strict --no-interactive.
- [ ] Review the complete new spec, plan and isolated roadmap delta before publication.

Expected: strict OpenSpec validation succeeds. New behavior remains S2; merged local behavior is S4. No deployment or proof is implied.

## Task 1: Financial postconditions

Files to create:
- experiments/moriarty-language/src/successor/financial-agreement-source-v4.ts
- experiments/moriarty-language/src/successor/financial-agreement-source-v4-frontend.ts
- experiments/moriarty-language/src/successor/financial-expression-v3.ts
- experiments/moriarty-language/spec/successor/financial-agreement-source-v4.md
- experiments/moriarty-language/spec/successor/financial-agreement-source-v4-grammar.ebnf
- experiments/moriarty-language/spec/successor/examples/financial-postconditions-payment.mori
- experiments/moriarty-language/examples/financial-postconditions-payment.mjs
- experiments/moriarty-language/tests/financial-agreement-source-v4.test.mjs
- experiments/moriarty-language/tests/financial-agreement-cli-v4.test.mjs
- experiments/moriarty-language/tests/financial-expression-contract-v3.test.mjs

Modify shared frontend.ts, format.ts, financial-agreement-source-compiler.ts, financial-expression-source-{types,v1,lower}.ts, financial-expression-v1.ts, funded-expression-source-v1.ts, cli.ts, package.json and relevant READMEs. A private machine module may be extracted if needed; preserve every old API result. The repayment kernel changes only for shared admission or internal reuse; add no operation here.

Public interfaces and evaluation order are fixed by design.md. Source/4 uses the existing four primitive evaluate arguments. Core/3 factory binds primitive schema and optional state text, with contextless check and action-only integrated funded evaluate. No public continuation or supplied financialPost is allowed.

- [ ] Read design-review/postconditions-expectations.md and the complete existing machine/adapter path.
- [ ] Add failing source/Core/CLI tests before implementation. Retain the red output.
- [ ] Test six post_* types, original PRE meaning, source spans and static rejection outside ensures.
- [ ] Add action-body/private-kernel/suffix execution using the same logical machine and carried locals.
- [ ] Preserve full admission, exact selector, protected descriptor mapping and EMPTY_BATCH before suffix.
- [ ] Test exact first/last false-condition rollback and kernel-before-ensure priority in /4.
- [ ] Implement real CLI check/format/simulate and an example with financial assertions.
- [ ] Update current grammar and README semantics with tested commands.
- [ ] Run package tests, typecheck, independent API/Core/CLI/legacy probes and README commands.
- [ ] Freeze full language+README candidate hashes. Obtain a fresh full Astra medium audit.
- [ ] Repair with the same author, then revalidate and obtain current approval after changed bytes.
- [ ] Publish and integrate only scoped approved files; preserve main preimages and unrelated hashes.

Independent fixture: use the current financial-state-payment repay_remaining body at debt100 with adequate balances/allowance. Its prefix costs41 and ordinary ensures6. Add three Quantity post comparisons, each cost5, and three Amount comparisons, each cost6. Values: post principal0/accrued0/outstanding0; payer balance0; allowance remaining0/spent100. Total work is41+2+6+33=82. Spendable82 succeeds at remaining0, reserve16 unchanged. Spendable81 rejects WORK_EXHAUSTED/workUsed81 with no output;43 fails first ensure;42 fails kernel INSUFFICIENT_WORK. False first financial ensure reports54; false last reports82. Retain prior spent17 in a separate control.

Additional required cases: unselected ill-typed action; extra request fields/callbacks; malformed unrelated state; missing created/missing post rows; wrong denomination; signed128 quantity limit/fullU128 Amount; identity newline rejection; old names as legal old-profile identifiers; dead-branch post scope; runtime short-circuit cost; failed transfer/repay atomicity; deterministic unchanged-input retry.

Commands:
```sh
npm --prefix experiments/moriarty-language test
npm --prefix experiments/moriarty-language run typecheck
node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-agreement-source/4 experiments/moriarty-language/spec/successor/examples/financial-postconditions-payment.mori
npm --prefix experiments/moriarty-language run financial-postconditions-demo
```

Expected: current tests plus new tests pass, exact documented financial assertions hold, negative commands exit1 with empty stdout.

## Task 2: Source origination and bounded accrual

Files: create src/successor/financial-lifecycle.ts, financial-agreement-source-v5.ts and corresponding profile grammar/spec/tests under experiments/moriarty-language/. Extend shared compiler, typed descriptors and staged evaluation only through explicit version gates. Keep repayment/0 and source/1–/4 closed contracts unchanged.

Interface: createFinancialAgreementSourceV5() retains elaborate/check/four-string evaluate. The lifecycle kernel exports prepareFinancialLifecycle(inputJSON) and primitive admission. Input binds a versioned state and protected Transfer, Repay, Originate or Accrue actions. The state retains balances, allowances, obligations, used transfer/allocation IDs, new origination/accrual IDs, terms/period cursor and work. No automatic migration may invent missing terms or historical authority.

Originate binds obligationId, transferId, originationId, debtor, creditor, nominalAmount, denomination, settlementAsset, conversion, allocationRule and accrualTerms. Accrual terms bind nonnegative numerator, positive denominator, floor/ceil rounding, positive periodSeconds and firstPeriodStart. Accrue binds accrualId, obligationId, periodIndex and observedTime. Ledger context must authenticate observedTime before network acceptance. Cursor0 precedes first period; only next index and its elapsed boundary are admissible. Debt creation and accrued values must fit the supported signed Quantity domain. Source terms are immutable after origination in this bounded profile.

- [ ] Freeze the exact closed source record/state schemas and static units in a task-specific review before authoring.
- [ ] Add independent failing funded-origination, rounding and duplicate-period tests.
- [ ] Implement disbursement funding consumption and unique debt creation.
- [ ] Implement bounded simple interest on current principal with checked multiplication before division.
- [ ] Reject invalid periods, settled debt accrual, duplicate IDs, unsupported terms and unsigned liability at authenticated boundaries.
- [ ] Reuse postcondition staging so new obligations and accrued debt are visible only as tentative financial POST.
- [ ] Run full legacy differential checks and actual API/CLI examples.
- [ ] Obtain fresh exact-candidate audit, repair and integrate the scoped result.

Numeric tests: principal101 at1/10 yields accrued10 floor or11 ceil; principal100 accrues10. Zero rate creates no debt delta but still records an accepted unique period and work. Repeat/reverse/skip period rejects. Partial payment cannot reset its cursor. Overflowing principal*numerator rejects even if division would fit.

Commands: package test/typecheck plus node --test experiments/moriarty-language/tests/financial-lifecycle.test.mjs and financial-agreement-source-v5.test.mjs. Exact source check/format/simulate fixtures accompany the new profile.

## Task 3: Complete source lifecycle

Files: spec/successor/examples/loan-lifecycle.{mori,state.json,snapshots.json}, examples/loan-lifecycle.mjs, tests/loan-lifecycle.test.mjs, package.json and README under experiments/moriarty-language/.

Interface: npm run loan-lifecycle-demo loads one source and passes actual previous output to the next public evaluation. It adjusts action Args and copies previous ordinary/financial output; it never edits financial values/history/work.

- [ ] Write independent complete expected states for originate100, accrue10, pay30 and settle80.
- [ ] Start lender100/borrower10; allowances lender100/borrower110; carry unrelated rows unchanged.
- [ ] Implement source actions and the public consumer, preserving all returned IDs and period metadata.
- [ ] Verify debt0→100→110→80→0 and final lender110/borrower0.
- [ ] Add duplicate accrual, late false ensure, failed funding and settled-payment controls.
- [ ] Derive exact work from final constructors; compare spent/remaining/reserve at every step.
- [ ] Run documented commands and audit complete outputs before integration.

Expected: no manual state repair, no liability loss and deterministic retry from failed input. Source assertions check financial results themselves.

## Task 4: Scoped K agreement

Files: existing formal/k/{codec.py,run.py,moriarty.k}; reviewed expression work from .worktrees/sp03-expression-k; new lifecycle fixtures/observations under formal/k/. Reconcile unmerged source before adopting it. Do not overwrite or delete its failure history.

- [ ] Verify exact retained trace106 input, parser, compiled interpreter/definition and toolchain hashes.
- [ ] Record a bounded amendment for one known backend-failure reproduction under existing controls.
- [ ] Run only that case first, preserving actual exit/diagnostics and changed-hypothesis disposition.
- [ ] Fix the reproduced cause without narrowing accepted metadata or replacing evaluation with host results.
- [ ] Implement supported financial PRE/POST, Originate, Accrue, identities, periods and staged rollback in K.
- [ ] Extend actual Core loader and complete output serializer; reject unsupported constructs.
- [ ] Run fresh complete applicable K/source/evaluator comparisons after changes.
- [ ] Prove the comparator detects a deliberately incorrect debt split, allowance or work value.
- [ ] Obtain independent semantic/result audit. Leave undischargeable metatheorems explicitly open.

Existing main command: python3 experiments/moriarty-language/formal/k/run.py --help. Existing expression worktree command: python3 run.py --suite expression-v1 evaluate INPUT. Exact invocation uses the inspected pinned artifact and admitted controls, not an invented runner. Lifecycle fixture commands are added to that existing runner with bounded counts. A unavailable runtime keeps this stage incomplete.

## Task 5: Authenticated ledger integration

Files: experiments/moriarty-midnight-financial/custody/{generate.mjs,loan.compact,bindings.json}; ledger/{run-local,integrate-local,launch-local,providers,receipt,financial-comparison,finalized-financial-state,contract-balances}.mjs; successor lowering in experiments/moriarty-language/; required existing executor contract in openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md.

Interfaces: source-bound program/profile and complete lifecycle state feed the constrained Compact transition. The actual SDK driver carries authority and expected revision, submits real operations, and reads native financial results. A host JSON post-state is never an authenticated transition witness by itself.

- [ ] Freeze the supported source-to-Compact mapping and exact authenticated predecessor/state/authority/time fields.
- [ ] Obtain task-specific design review of those constraints and the network failure/fee oracle before production edits.
- [ ] Add failing substitution, stale-revision, unauthorized debt, replay and fee-inclusive result cases.
- [ ] Implement the actual lowerer/custody/driver/comparator path together.
- [ ] Implement the already specified exit-retaining executor and static prover lifetime boundary where missing.
- [ ] Run offline ledger, compiled, finalized-state and Preview adapter tests in their actual declared scope.
- [ ] Obtain fresh complete source audit before any service or public invocation.
- [ ] Reconcile current source binding, actual historical accounting and bounded resource/attempt authority once.
- [ ] Obtain required substantive votes for any bounded amendment without resetting prior charges.
- [ ] Start only admitted preserved-identity Docker services under the verified lifetime boundary.
- [ ] Execute the complete authenticated lifecycle, failed-transaction readback and raw exit/cleanup checks.
- [ ] Audit actual Docker result before preparing Preview.
- [ ] Dispatch Preview only through python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . run --action ACTION.
- [ ] Report each public transaction ID/status, retain finality/fees/native state and actual exit zero, then audit results.
- [ ] Reconcile task status and preserve unresolved MC04/MC05 and full SP05 gates.

Offline commands from experiments/moriarty-midnight-financial:
```sh
npm run test:ledger
npm run test:compiled
npm run test:finalized-state
npm run test:preview
```

ACTION is obtained from current registered admission, never invented. Missing grant, funding, attempt capacity, executor or authentication keeps dependent dispatch unavailable. No synthetic result can satisfy Docker or Preview acceptance.

## Native loop and completion

The active native goal names all six outcomes. Root checks actual state, selects one eligible task, supplies independent expectations, dispatches Grok, verifies results, obtains fresh Astra audit, repairs and safely publishes. Current source tasks use isolated worktrees. Existing plugin run handles registered campaign actions. Do not create a Foreman scheduler, new daemon or process-only approval project.

The first author task is financial postconditions. Next tasks start only after their semantic dependency is reviewed; independent status reconciliation continues when eligible. Every completed scope retains full candidate and result receipts. Overall completion requires all six outcomes; an external gate stays explicit rather than being silently waived.
