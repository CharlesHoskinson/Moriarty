# Deliver the bounded loan language through ledger execution

## Why

Merged PRs 1–4 establish computed funded repayments, source-defined schemas, multiple named actions and validated financial reads. The terminal decision remains open: source ensures cannot inspect financial results, obligations cannot originate or accrue through this adapter, and this successor flow has no complete K or authenticated ledger execution evidence.

The user approved six concrete follow-ups on September 12, 2026. This package supplies their requirements and ordered completion checks. It does not replace MC01–MC08 or relax their acceptance gates.

## What Changes

- Add financial postconditions with tentative kernel execution and atomic publication.
- Add explicit obligation origination and bounded interest accrual through .mori actions.
- Execute one complete loan lifecycle with conserved liabilities, identities and cumulative work.
- Compare its complete observations across source, evaluator and K.
- Connect the supported language path to authenticated state, then test Docker and admitted Preview execution.
- Reconcile the four merged capabilities at their implemented local scope.

## Capabilities

### New Capabilities
- `financial-postconditions`: typed financial POST reads in ensures, versioned evaluation and rollback.
- `loan-origination-accrual`: protected creation, explicit terms and replay-resistant bounded accrual.
- `bounded-loan-lifecycle`: executable source, continuation and complete independent observations.
- `lifecycle-k-agreement`: runnable supported Core semantics and full differential cases.
- `lifecycle-ledger-integration`: authenticated provenance, real caller, Docker and Preview results.
- `language-delivery-reconciliation`: scoped roadmap and evidence status.

### Modified Capabilities
None. Existing MC and sprint acceptance remains normative. New implementation extends explicit profiles instead of changing frozen wire contracts.

## Impact

Primary ownership: `experiments/moriarty-language/**`, its source/CLI/examples/K rules, root README and ROADMAP. Ledger implementation extends the existing Midnight adapter and reviewed caller identified in design.md. Runtime accounting changes are limited to concrete missing prerequisites of an admitted execution. No scheduler, plugin replacement, broad graph rebuild, general recursion campaign, wallet replacement or unrelated financial library is included.

Baseline: main `229545fada12c151ed5693f4e6cebdab956bd222`, including merged language tree `dc9d516eabb5757591e6ef73a7544639cbe55134`. Existing uncommitted user amendments remain preserved.

New behavior is S2, specified-only until executed and independently reviewed. Four earlier capabilities are S4 local implementation, not network acceptance. Evidence lives in `deliverables/language-to-ledger-2026-09-12/`.
