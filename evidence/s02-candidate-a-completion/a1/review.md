# A1 local plan review

Classification: repository observation and independent review disposition.
Scope: local planning only. Neither lifecycle implementation nor Council acceptance.

## Frozen inputs

- Installment plan: `docs/superpowers/plans/2026-09-05-moriarty-s02-candidate-a-authority-installment.md`; SHA256 `b0dfcb4d8c2522d160cd84eac265fb188a26f307c05bcb840754728590075b67`.
- Swap plan: `docs/superpowers/plans/2026-09-05-moriarty-s02-candidate-a-authority-swap.md`; SHA256 `a6160ad728502368f62dd0923196e1ba95e8c037211290dc836ca5889d85a5ab`.
- Frozen common/A source: `d14cfea98a1c9213e5ef5f12c1a088f4e966083d`.
- Pre-adoption branch HEAD: `d97d25bb59451473f14388a229eee696b8965a8a`.
- Dependency: local A0 evidence `0190cb97270e4775273cb3737dc649d874861e91`, canonical review index `95899b37fa03ccb61506e51c099dd6aea96bcc16`.

## Independence and findings

Planner: native agent `/root/a1_installment_plan`. Nonauthor source/semantic reviewer:
native agent `/root/a0_final_review`. Root independently read both complete plans
and owns temporary code assembly/typechecking and admission. The native reviewer
did not edit or run the planned implementations. This is not a blinded,
cross-provider Council, and it does not satisfy that later obligation.

Root corrected three draft syntax issues through the planner: a missing closing
parenthesis, typed action return annotations, and a local identifier that
shadowed Quint's built-in `enabled`. The temporary earlier installment draft
then typechecked with exit0. That result does not cover later assertion edits;
final exact-plan receipts are separate.

Material review corrections adopted before final review:

- Fresh cancellation uses FreshCancelAttempt and the original nonce0 signature;
  second fill reaches revision2. Recovery uses new nonce1 against cancelled facts.
- Rejection tests compare entire resulting states with exact original attempt,
  supplied evidence, current observed context, literal reason and boundary stage.
  Unrelated state must remain unchanged.
- Installment deadline refusals preserve complete nonce1 registration/signing
  and have no pending attempts. Recovery does not rewrite cancelled parent history.
- The no-cancellation negative tests the financial guard and recovery-preparation
  guard directly. Its unsigned composite rejection is not falsely attributed to
  only one condition.
- Both sampling harnesses retain a last-transition atomicity check. Commit
  comparisons use the unchanged common applyCommit result.
- Swap raw minimumTime0 remains unchanged; actual funding occurs at1 then2.
  Empty-effect timeout is a retained envelope rejection, not an authorized commit.
- Wrong actor means the actual Core actor or signed signer. PreparedAttempt.actor
  metadata is not authenticated by current common/A contracts; that limit is explicit.
- Pure swap stale routes belong to the lifecycle module, not the test module.
- Longest explicit routes are17 installment and19 swap transitions, with command
  bounds20 and22. Start100 samples; escalate only for a named missing witness.

## Final reviewer disposition

The reviewer approved installment hash b0dfcb4d after checking the full-state
assertion corrections. The final swap re-review approved hash a6160ad7 and
confirmed these exact classifiers: mutation/wrong signer UnauthorizedEffect;
wrong nonce EvidenceMissing; deadline CoreRejected(contract_closed); empty
timeout UnauthorizedEffect. No remaining semantic blocker was reported.

Proposed RED mutations were reviewed for detectability, not executed: altered
literal reduction count, unchanged commit update, and generic verification
admitting the before-resolution unused-successor mutant. Genuine compiling
behavioral RED and GREEN remain mandatory A2/A3 implementation work.

## Requirement mapping

| Requirement | Reviewed plan contract |
|---|---|
| A1-R01 | Both plans define four owned modules, complete typed helpers, literals, scenario tables, commands, witnesses and task reviews. |
| A1-R02 | Installment freshCancellationTest and FreshCancelAttempt retain the original parent policy. |
| A1-R03 | Installment twoFillsTest requires both used slots, paid10, allowance0 and revision2. |
| A1-R04 | recoveryMatrixTest and preparation/signing routes bind nonce1 to cancelled actual parent facts. |
| A1-R05 | No consumed-nonce0 re-signing is required; stale historical draft is retained in the original handoff archive. |

## Limits

Plan adoption does not execute these behavioral requirements. A2/A3 must obtain
exact-source compiling RED/GREEN, deterministic cases under both profiles,
nonzero action witnesses, full source review and terminal regression results.
Scheduled finite routes do not cover arbitrary interleavings. A4 correspondence,
A5 explicit bounded model checking, A6 full validator/Council and A7 integration
remain open. No common or A boundary semantics were changed for A1.
