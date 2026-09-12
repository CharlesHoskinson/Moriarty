# Bounded language-to-ledger design

## Inputs and dependencies

Baseline is main229545f, including PR1–PR4 at merge dc9d516. Current language APIs and complete audit receipts are normative for compatibility. Current user scope and Grok4.6 high/Astra medium routing supersede recovered model queues. SP02/SP03 own source and semantics; MC04/MC05 own authenticated correspondence and acceptance. SP07 owns later scheduled financial coverage. This package closes only its named bounded predicates.

The September11 SDK design remains specified-only. Preserve its program/profile/head identity, gross debit, net credit, fee and liability distinctions. No multichain implementation is added.

## Alternatives and selected approach

1. **Selected: versioned staged evaluation.** Extend the existing machine with a private prefix/kernel/suffix boundary. Add explicit financial POST intrinsics to a new profile. This preserves old meanings, executes each statement once and allows atomic financial assertions.
2. Re-run the action with synthesized post snapshots. Rejected: duplicated work/effects, forged observation risk and inconsistent locals.
3. Expose host callbacks or caller-provided post state. Rejected: the claimed result would not be bound to kernel execution.

Independent Astra expected-results review agrees with the selected approach. Preserve its receipt in deliverables/language-to-ledger-2026-09-12/design-review/. A fresh full candidate audit remains required after implementation.

## Financial postconditions: public interface

New source profile: `moriarty-financial-agreement-source/4`.
New Core contract: `moriarty-financial-expression-contract/3`.

`createFinancialAgreementSourceV4()` exposes frozen elaborate(source), check(source), evaluate(source, actionName, snapshotCanonicalJSON, financialPreStateJSON). All arguments are primitive strings. Outputs retain the existing source artifact and funded-result conventions, with explicit new profile IDs.

`createFinancialExpressionContractV3(schemaCanonicalJSON, financialPreStateJSON?)` exposes frozen check(requestCanonicalJSON) and evaluate(requestCanonicalJSON). The request retains the existing closed contract/source/core/Pre/Args/Obs/workInitial envelope. check validates without financial context. evaluate requires an action and owned validated financial PRE; it returns the funded result, never a bare expression preparation. Static errors precede missing context. A well-typed action without state rejects FINANCIAL_CONTEXT_REQUIRED. Standalone expression execution rejects TYPE_ACTION_REQUIRED after static admission. Both use `{status:"Rejected",code,span:{kind:"synthetic",start:"0",end:"0"},nodePath:[],workUsed:"0"}`. A post read outside an Ensure rejects TYPE_POST_SCOPE with its original source/Core location, including dead branches. Source and Core /3 state-admission errors return the existing admitRepaymentStateJSON result unchanged: primitive transport errors keep bare INPUT_SCHEMA/INPUT_BOUND; malformed kernel state retains its code and actionIndex:null. This differs explicitly from the old Core /2 normalized expression envelope, which remains unchanged. Neither API accepts a post-state, continuation, executable callback or object input.

Post intrinsics are post_outstanding<U>(id), post_principal<U>(id), post_accrued<U>(id), post_balance<A>(party), post_allowance_remaining<A>(party), post_allowance_spent<A>(party). The first three return Quantity<Units<U,1>,0>; the other three return Amount<A>. They occur only in existing ensures expressions, including nested/short-circuited expressions. Static placement still checks dead branches. Core constructors use distinct ReadPost* names with the same closed unit-or-asset and identity operands as corresponding Read* nodes. Unprefixed reads always use financial PRE.

## Evaluation and publication

1. Parse and type-check all actions, then select the exact action.
2. Admit full financial state before runtime snapshots or reduction.
3. Check snapshot workInitial against admitted financial work first; mismatch returns bare Rejected/WORK_MISMATCH. Then validate complete canonical snapshots.
4. Execute the prefix once, retaining typed locals, ordinary writes and descriptors.
5. Bound ordinary post and descriptors, map protected operations, and reject EMPTY_BATCH if no descriptors exist.
6. Debit prefix work into a private owned financial state and execute the existing repayment kernel once.
7. Preserve completeFundedPreparation nominalUnitsMatch against prepared.post obligations selected by the mapped Repay actions. Reject mismatch before any suffix or POST publication. Then continue the same logical machine through all ensures, retaining original statement paths and source spans.
8. Read financial PRE from the original state and financial POST from the kernel result.
9. Debit suffix work, validate final aggregate results and publish all state/effects together.

All ensures run after the kernel in /4, including ordinary-only ensures. Earlier profiles retain their original order. Ordinary post means staged ordinary writes. Internal continuation is private and cannot be replayed by consumers. Factoring the machine is allowed; a second interpreter or public resumable token is not.

Work is E_prefix+N_kernel+E_suffix. Runtime short-circuiting preserves exact dynamic reductions. Closure reserve is a separate retained field, not subtracted twice from spendable work. Failed suffix reductions report total attempted work including the successful tentative kernel. Kernel failures retain their defined code/actionIndex envelope; source/Core admission failures retain their documented envelope. No failed preparation publishes tentative state or implies actual on-ledger fee refunds.

## Origination and accrual boundary

The next explicit source/kernel-state version extends protected operations with Originate and Accrue. Do not add fields to old closed projection schemas. Reuse Transfer funding identity and the existing allocation/conversion rules.

The first policy is one nonnegative simple-interest rational rate per exact period. Terms contain numerator, denominator, rounding, period duration and an initial period boundary. Each obligation owns its accepted period cursor. The exact versioned field names and protected record schema are frozen in the step-specific plan before production edits. This is a deliberate stage contract boundary: no generic scheduling, compounding or ACTUS convention is implied.

Originate consumes one lender-to-debtor disbursement with matching amount, asset and conversion; it creates unique principal and retained terms/cursor. Accrue adds floor/ceil(principal*numerator/denominator) once for the next eligible period, with bounded intermediate arithmetic. It moves no cash. The period cursor remains after repayment and settlement. Settled debt cannot accrue. Local terms are input, not proof of debtor authority; ledger acceptance binds signed consent and time later.

The complete example begins lender100 and borrower reserve10. Originate100, accrue10, repay30 interest-first, then computed settle80. Expected debt sequence is 0→100→110→80→0. Final balances are lender110/borrower0. Allowances, identities and work carry across all four actions. Failed actions return no tentative output.

## K implementation and known defect

Main K is a bounded repayment projection, not the complete expression machine. Reusable expression work is in .worktrees/sp03-expression-k. Retained backend failures are in .worktrees/sp03-expression-k-parse04 and -macro05, formal/k/.build-expression-v1/trace-106.*. Both had105 matching observations before one large-metadata failure. Do not replay the passing prefix or reduce supported metadata limits to hide the fault.

Before expanding K, reproduce the selected single retained input against its exact existing compiled artifact under a bounded reviewed amendment. Preserve actual exit, stderr and pins. Then implement admitted financial state and staged prefix/kernel/suffix behavior through existing codecs/rules. Final acceptance requires a fresh complete relevant differential corpus after changes. Historical results do not substitute. Metatheorem and correspondence proofs remain separate gates.

## Ledger implementation and trust boundaries

Reuse `experiments/moriarty-midnight-financial/custody/{generate.mjs,loan.compact,bindings.json}` and `ledger/{run-local,providers,receipt,financial-comparison,finalized-financial-state,contract-balances}.mjs`. The current loan initializes hardcoded older state. Replace that semantic seam with source-bound lifecycle state and constrained transitions. Do not pass arbitrary JSON as authenticated ledger state.

Bind source/Core/profile digest, deployment/network identity, expected head/revision, debtor/lender authority, balances, allowances, outstanding principal/interest and period/replay history. Derive complete state from actual accepted ledger reads and verify transitions through compiled constraints. Fees remain distinct from loan settlement amounts and must enter fee-inclusive net goals.

The real exit-retaining executor is missing. Reuse `openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md`, its reviewed collector candidate and existing launch/receipt modules. Implement the caller and specified static prover-lifetime helper; do not invent another launcher framework. Source tests alone cannot establish process containment or financial success.

Docker services are currently stopped. First complete source/driver changes and reviews. Then prepare preserved-identity services under the admitted lifetime contract, execute the exact lifecycle, retain raw exit before cleanup and independently decode complete results. Preview follows only after successful Docker evidence and current plugin admission.

Current plugin gaps are stale binding/candidate, missing current-accounting.json and unavailable resource state. Reconcile actual historical charges and immutable records. Any new grant or attempt amendment needs bounded limits and required substantive votes. Never reset counters or create accounting from zero. Missing evidence keeps the dependent public action blocked while independent work continues.

## Failure handling and completion

Each capability has its own reviewed candidate and actual-result evidence. Keep one implementation writer at a time. Return failures to the same author for bounded repair. After two same-class failures, reproduce and change approach. Preserve originals and fresh audit identities. Publish scoped reviewed changes without staging unrelated user work. Reconcile local implemented scope separately from K, Docker, Preview and proofs.

The native goal is the loop. A planning record is not process liveness. Query actual author/audit/runtime state before reporting activity. No new scheduler is required. All six acceptance outcomes are required for overall completion.
