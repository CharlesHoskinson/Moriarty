# Tasks

Tasks 1–3 below reflect the merged PR5–PR7 local implementations and their [independent audits](../../../deliverables/language-to-ledger-2026-09-12/). This reconciliation does not close K, ledger or proof requirements.

## 0. Scope reconciliation and plan
- [ ] 0.1 Preserve current main and unrelated preimages; record merged PR1–PR4 receipts.
- [ ] 0.2 Update ROADMAP.md with implemented local scope and retained wider gates.
- [ ] 0.3 Validate this package with `openspec validate language-to-ledger-lifecycle --strict --no-interactive`.
- [ ] 0.4 Review the complete design and plan at docs/superpowers/plans/2026-09-12-language-to-ledger.md.

## 1. Financial postconditions
- [x] 1.1 Add failing source/Core/CLI tests for the six post reads and atomic failure.
- [x] 1.2 Implement source/4, Core/3 and private body/kernel/ensures staging.
- [x] 1.3 Add runnable example and update README syntax, semantics and commands.
- [x] 1.4 Run package test/typecheck and independent exact-work/rollback/legacy probes.
- [x] 1.5 Obtain fresh exact-candidate Astra audit; repair, publish and integrate scoped files.

## 2. Origination and accrual
- [x] 2.1 Freeze versioned closed schemas, terms, authority boundary and independent numeric expectations.
- [x] 2.2 Add failing funded origination, floor/ceil, duplicate-period and overflow tests.
- [x] 2.3 Implement protected operations, complete effects, source/CLI consumer and cursor retention.
- [x] 2.4 Run actual API/CLI and legacy checks; obtain fresh audit, repair and integrate.

## 3. Complete lifecycle
- [x] 3.1 Add source and independent complete states for debt0→100→110→80→0.
- [x] 3.2 Implement examples/loan-lifecycle.mjs and npm loan-lifecycle-demo.
- [x] 3.3 Check conservation, cumulative work, IDs, failed continuation and documented commands.
- [x] 3.4 Audit complete outputs and publish the scoped result.

## 4. K agreement

The [September 17 native K results](../../../deliverables/k-lifecycle-execution-2026-09-17/RESULT.md) supersede the earlier unavailable-runtime statement: a pinned rebuild ran the finite expression/lifecycle corpus and direct probes. Historical binary recovery was not achieved. The checklist below remains open pending exact item-to-evidence reconciliation; finite K agreement does not discharge general theorems or ledger correspondence. No item is newly marked complete by this audit.
- [ ] 4.1 Pin retained failing case and compiled artifact; admit one bounded reproduction.
- [ ] 4.2 Resolve the reproduced backend defect without narrowing the accepted language.
- [ ] 4.3 Implement supported lifecycle rules, loader and complete observations in formal/k/.
- [ ] 4.4 Run fresh positive/negative differential corpus and a comparator mutation control.
- [ ] 4.5 Audit actual K results and retain unproved metatheorem/correspondence gates.

## 5. Authenticated Docker and Preview
- [ ] 5.1 Freeze and independently review source/profile/head, authenticated state/authority/time and failure/fee constraints before implementation.
- [ ] 5.1a Bind those reviewed constraints in the actual lowerer/custody/SDK caller.
- [ ] 5.2 Implement the already specified executor/lifetime boundary and offline fault matrix.
- [ ] 5.3 Run test:ledger, test:compiled, test:finalized-state and test:preview; audit exact source.
- [ ] 5.4 Reconcile real accounting, binding, funding and bounded reviewed admission evidence.
- [ ] 5.5 Run complete Docker lifecycle with native readback, rejection control, raw exit and containment.
- [ ] 5.6 Obtain independent Docker result audit and current Preview admission.
- [ ] 5.7 Run the admitted Preview action through the plugin; report IDs/status and verify complete financial effects.
- [ ] 5.8 Audit actual Preview result and update scoped roadmap evidence without closing unrelated proof gates.

## 6. Final reconciliation
- [ ] 6.1 Verify every requirement has a current result or a precise open blocker.
- [ ] 6.2 Publish reviewed changes and preserve unrelated user work and historical failures.
- [ ] 6.3 Close the native goal only when all six requested acceptance outcomes pass.

Exact files, interfaces, test inputs and commands are in the implementation plan. Evidence root: deliverables/language-to-ledger-2026-09-12/.
