# Bridge recommendations from five sequential Opus 5.5 reviews

**Date:** 2026-09-29. Five independent read-only sessions returned canonical `claude-opus-5-5`, all `completed`; combined reported cost USD 4.4135542. The [brief](BRIEF.md), prompts, raw receipts, logs and [five reviews](review-01.md) remain separate. These are proposals for a specified-only design, not a working cross-domain bridge.

| Lens | Review | Main proposal |
| --- | --- | --- |
| Paired claims | [1](review-01.md) | Unique transfer identity, transfer-form type and one-shot destination receipt with local backing counters. |
| Foreign evidence | [2](review-02.md) | Digest-bound verifier policy, replay by foreign message ID, destination-clock nonreceipt and typed finality. |
| Async lifecycle | [3](review-03.md) | Destination receipt decides receive/timeout race; partial fills and fast-fill claims persist. |
| Native path | [4](review-04.md) | Local claim/backing equation, enumerated verifier mode, policy-independent nullifier and exact decimal conversion. |
| Adversarial plan | [5](review-05.md) | Paired one-shot claim, domain-pair binding, false-timeout control and loss-as-debt response. |

## Shared recommendations

1. **Add a unique paired transfer claim.** A bridge needs an immutable transfer ID, source and destination domains, canonical and represented assets, transfer form (lock-mint, burn-mint or release), amount conversion, owner/recipient, policy and nonce. Each local stage updates a claim state; a destination issue must consume exactly one verified entitlement. Per-domain conservation remains necessary but does not prove foreign backing.
2. **Type and enforce the foreign verifier policy.** `imported(Policy)` should bind a digest, mode, source domain/chain epoch, finality class, verifier keys or light-client relation, accepted message shape and challenge/revocation rule. The verifier assigns the evidence class after checking bytes; a host Boolean or an issuer label is not proof. Replay prevention keys the foreign message/claim identity, not the specific proof serialization.
3. **Make receive versus timeout a destination decision.** The destination receipt is one-shot and decides whether a later source refund may be authorized. A local deadline is insufficient. Refund needs authenticated destination nonreceipt under a policy that also prevents later receipt for that transfer. Unknown or partitioned status stays pending, with conditional liveness assumptions and retained duties.
4. **Keep partial delivery and fast fill as claims, not supply shortcuts.** A partial destination delivery reduces the remaining entitlement; cumulative amounts and fees roll forward. A fast filler advances its own capital and receives a bounded reimbursement claim and priority; it cannot create an unbacked mint. Bond, challenge and loss allocation are separate policy obligations.
5. **Start with a domain-local native discriminator.** One source lock or burn and one later destination claim receipt, under one explicit verifier mode, must bind exact intent, transfer ID, domain pair, imported message bytes, nullifier, supply/balance effects, predecessor and tombstone. Hostile controls should keep the envelope valid while mutating proof domain, message ID, amount, replay state, nonreceipt status or late receipt.

## Limits and decisions

| Question | Positions in reviews | Required resolution |
| --- | --- | --- |
| Milestone | [3](review-03.md) and [5](review-05.md) describe a U3 async slice; [4](review-04.md) stresses that usable imported verification is deferred to U4 in MIL/2. | U3 may specify and execute local pending/recovery rules under a named premise. Do not claim a proven foreign bridge until the selected verifier mode is available and native-bound. |
| Backing invariant | [1](review-01.md) proposes local backing counters; [4](review-04.md) a one-sided backing equation; all reject a fictional global ledger equation. | Define the paired claim invariant under the chosen trust model and prove local updates plus authenticated foreign evidence imply the claimed bound. State what fails under verifier compromise. |
| Reorg response | [2](review-02.md) and [5](review-05.md) distinguish finality from outcome and propose a disputed state. | Define whether credit can be withdrawn, frozen or compensated after a reorg; name loss bearer and maximum exposure. Do not call any local action global rollback. |
| Numeric conversion | [1](review-01.md) and [4](review-04.md) require exact decimal scaling and a dust claimant. | Specify the representation ratio, directed rounding and supply cap before minting across different decimals. |

The [category review](../category-review/07-bridges.md) and [MIL/2 obligations](../../../concepts/intent-language/DESIGN-MIL2.md) remain open. The [IBC packet specification](https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md) is a comparative primary source for timeout/nonreceipt discipline; it is not a Midnight adapter. No tests, proofs, compilation or Preview transactions were run in these reviews.
