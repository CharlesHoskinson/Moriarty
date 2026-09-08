# FOREMAN_REPORT

This pass repairs three evidence-identity defects. It does not accept Milestone 1.

Parent candidate `509c619ceb55a031674ca40bc7c0490f2e57f8986def35580ada32f884b54f39` is unchanged. This pass owns only `records.py`, `test_records.py`, and these two reports.

## Initial red

Independent GPT-6 review of candidate 04 was BLOCKED (M1-C4-E1, M1-C4-E2, M1-C4-E3).

Reproduced before edits against current source:

- `loan-as-atomic-prerequisites` and `loan-as-finalized-prerequisites`: `entryEligible=true` after overlay of loan binding/review/acceptance onto atomic-prepare and atomic-accept.
- `publication-added-approval-fields`, `publication-existing-scope-escalation`, `publication-closure-escalation`: `candidateCurrent=true` after nested acceptance flags, full-native/Preview claims, or blanket closure rewrite with an updated digest.
- `acceptance-unknown-complete-status` and `acceptance-REVOKED-status`: `candidateCurrent=true` for `complete-NOT_REVIEWED` and `complete-but-REVOKED`.

Copied-budget admission is not live resource authority. Resource defects stay open.

## Changed behavior

Stage identity: `binding.stage`, campaign `stage`, and required `stage.id` must match for the direct action and each prerequisite. Accepted profile is the `semanticProfile` from a hash-verified `moriarty-bounds/1` input. Loan-design evidence cannot satisfy atomic-prepare, atomic-accept, or finalized-financial-settlement.

The valid prerequisite control is the genuine `rp01-mc02` family. Atomic overlay is a negative case. Cycle detection remains. Independent F0 blockage still does not block SP05 when the gate uses that genuine stage.

Acceptance status is `complete-design-subset-only`. Campaign complete status is `complete`. Review pass is `APPROVED` with scope `RP01-MC02 specified design consistency only`. Unknown `complete-*` strings stay unresolved.

Publication: freeze original bytes still authenticate the bridge. Nested RP01-MC02 acceptance is closed. Scope, networkMilestones, staticVerification, candidateManifest, and review must match the demonstrated design-only transform. Closure text may replace `This subset remains pending-review.` only. Extra approval flags, unbound candidateHashStatus, unsupported output statuses, and obligation-erasure claims reject.

## Exact checks

Positive: genuine `sp01-loan-swap-grok-01` keeps `candidateCurrent` and `entryEligible` true. Requiring `rp01-mc02` as a prerequisite keeps both true. Blocked F0 still leaves an SP05 gate that requires only `rp01-mc02` eligible.

Failures keep the flags distinct. Loan-as-atomic overlay sets `entryEligible` false and leaves `candidateCurrent` true. Nested publication and closed-status defects set `candidateCurrent` false and leave `entryEligible` true. Genuine atomic-accept remains `required-stage-profile-unresolved:atomic-accept`.

## Remaining open

Resource defects stay open. Unknown amendment fields, omitted charges, and unrelated pass scope still admit under the copied-budget fixture.

Milestone 2 store and CLI, Milestone 3 packaging, and Milestone 4 ledger demonstration are not in this pass. Genuine atomic-prepare and atomic-accept records still lack the supported freeze, binding stage, bounds profile, and acceptance shape.

This pass does not accept the plugin, any sprint, or product behavior.
