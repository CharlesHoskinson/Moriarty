# Sherlock circuit optimizations

Status: DRAFT / PROPOSED; not adopted, implemented, validated or accepted. Public Sherlock reference commit: `35931362ca435b2ed10014ee052870716c8f10d3`.

## Why

Sherlock's return and error publication emitters contain an identical 132-line, 3,903-byte utility block: `crates/mori-backend/src/sherlock_machine/return_publication_step.rs:78–209` and `error_publication_step.rs:79–210`. Direct text readback gives SHA-256 `5acaa85d4ff99c201c3e908dcd2dcfc66403cabcc12c45a11d40bf98aad2126a`. This establishes duplication only. Ordered emitted-operation equivalence, net maintained-line reduction, circuit rows, k, CPU, memory and proof sizes remain unmeasured.

## Scope preserved

This additive package preserves the [original requirements](../../../sherlock-universal-interpreter-design/requirements.md), [completion refinements](../../../sherlock-completion-convergence/requirements.md), [recursive experiment](../../../sherlock-stagenet-recursive-rollup/requirements.md) and [product contract](../../../../../docs/MORIARTY-PRODUCT-CONTRACT.md). Existing designated-owner requirements are carried forward without new security analysis or policy. All scenarios are proposed acceptance contracts, not execution receipts.

## Requirement metadata retained

The traceability and status metadata for the two length-adjusted clauses is unchanged:

- SOPT-010: `Retains: UIREQ-023, UIREQ-024, UIREQ-025, SCON-001, SCON-004, SCON-022, SRPOC-002, SRPOC-005. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).`
- SOPT-012: `Retains: UIREQ-024, UIREQ-025, SCON-024, SRPOC-005, SRPOC-009. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).`

## What Changes

Propose source sharing first, then a typed layout/encoding seam with separate constraint, witness and export plans. Organize the existing 39 semantic events into eight maintenance families without reducing their distinct selectors. Specify a conditional same-cell constraint-sharing experiment and an optional final-parent/terminal fusion experiment, each judged against preserved baseline semantics and measured costs.

[Sixteen EARS clauses](requirements.md), [32 scenarios](specs/sherlock-circuit-optimizations/spec.md), [QA](qa.md), [traceability](traceability.json), [rule map](rule-families.json), [design](design.md) and [tasks](tasks.md) make these proposals reviewable. The actual recursive stagenet path continues; isolated source preparation may proceed before its baseline, but optimization integration follows the observed baseline. Integration means any change within the traced source/context identity membership of the native recursive child, parent or terminal path (SOPT-001/010); changes outside that membership are unaffected, and a maintenance retain disposition inside it remains isolated until the baseline is preserved.

## Impact

Potential future source consumers are the existing `mori-core` Sherlock representations/witness plans, `mori-backend` publication emitters and child/link adapters, and existing ledger transaction consumers. This change edits specifications only. It introduces no runtime, compiler replacement, foreign VM or orchestration framework.

UIREQ-001–035, SCON-001–025, SRPOC-001–009, required grouping/residualization, financial/communication obligations, existing designated-owner reviews, resource rules and all roadmap exits remain unchanged. These additive SOPT clauses are not a new completion denominator. The permissionless [product contract](../../../docs/MORIARTY-PRODUCT-CONTRACT.md) remains controlling. As of this dated draft revision on 2026-10-19, approval of the changed draft is not recorded; subsequent review and publication status is tracked separately. This package makes no security judgment and creates no new security policy, public permission, human approval requirement, resource cap or counter reset.
