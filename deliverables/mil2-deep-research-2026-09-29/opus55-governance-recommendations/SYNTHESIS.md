# Governance recommendations from five sequential Opus 5.5 reviews

**Date:** 2026-09-29. Five independent read-only sessions returned canonical `claude-opus-5-5`, all `completed`; combined reported cost USD 4.3332922. [Brief](BRIEF.md), prompts, raw receipts, logs and [five reviews](review-01.md) are retained. The recommendations below do not amend MIL/2 or establish native enforcement.

| Lens | Review | Main proposal |
| --- | --- | --- |
| Authority/policy state | [1](review-01.md) | Ledger grant state, attenuating delegation, policy-version pinning and non-stranding revocation. |
| Voting/threshold | [2](review-02.md) | Distinct-key authority source, snapshot-bound voting, one-vote receipts and separate delegation kinds. |
| Timelock/pause/migration | [3](review-03.md) | Pending queue/execute/cancel relation, protected exits and consent-aware migration. |
| Native amendment | [4](review-04.md) | One authenticated `PolicyHead`, head-fresh authority and exact amend effect. |
| Adversarial plan | [5](review-05.md) | Monotone grant epoch, consumed approval set, rechecked queue and beneficiary protection. |

## Shared recommendations

1. **Make authority a current ledger state, not a stale signature or Boolean.** Grant, revoke and delegate need typed transitions, a monotone epoch, scope, expiry, remaining budget and an authenticated head. Delegation can narrow scope or budget, never copy spending power. Threshold authorization counts distinct eligible keys and consumes a digest-bound approval set if the action is one-shot.
2. **Pin the policy governing existing duties.** Each persistent obligation, position or escrow should carry a policy version/commitment. An amendment applies to future actions unless a preservation relation or affected-party consent allows migration. Revocation can stop future authority without erasing an already owed duty or the owner's exit/recovery rights.
3. **Specify the administrative transition.** `amend` must name the exact policy cell, old/new version, authorized effect, beneficiary protection and current-head check. A queued timelock action is pending state with execute/cancel outcomes; execution rechecks current authority and policy conditions rather than trusting an old approval.
4. **Keep emergency remedies usable.** A pause rule must state exactly which actions stop and which repayment, withdrawal, challenge or recovery paths remain possible. An emergency actor cannot silently gain beneficiary-changing or supply-changing authority through a generic pause flag.
5. **Separate process from effect.** Voting, proposal and snapshot logic can be a versioned governance library, but the core must carry the authenticated outcome and restrict the financial effect it authorizes. One-signer U0 stages can exercise pre-existing approvals only if the native statement verifies the consumed outcome; general multi-signer stage admission remains a later profile.

## Open design choices

| Choice | Reviewer positions | Required resolution |
| --- | --- | --- |
| Pause authority form | [3](review-03.md) proposes a ninth `suspend` right; [4](review-04.md) models pause as a policy parameter with unpausable remedies. | Decide whether an explicit right kind is necessary for nondelegable emergency scope. In either form, prove that existing recovery and debt duties survive. |
| Revocation and queued action precedence | [1](review-01.md), [3](review-03.md) and [5](review-05.md) all require current-head checks, with different queue/cancel details. | Specify an exact state machine and same-head ordering: queued approval, grant revocation, execution and cancellation cannot all win. |
| Voting versus authority | [2](review-02.md) reserves a core hook for snapshot outcomes; [4](review-04.md) starts with a single already-authorized amendment. | First prove one authenticated policy amendment. Add snapshot/vote semantics as a separate library and native statement profile; do not infer it from `k_of_n`. |

The first evidence slice is a single policy amendment that leaves a previously signed obligation under its prior policy. Hostile cases revoke the grant before execution, replay a threshold approval, change the beneficiary, or pause a repayment path while keeping the envelope otherwise valid. The [category review](../category-review/06-governance.md) and [MIL/2 obligations](../../../concepts/intent-language/DESIGN-MIL2.md) remain open. No tests, proofs, compilation or Preview transactions were run in these reviews.
