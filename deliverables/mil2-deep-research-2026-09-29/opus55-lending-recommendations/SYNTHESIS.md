# Lending recommendations from five sequential Opus 5.5 reviews

**Date:** 2026-09-29. Five independent, read-only sessions returned canonical model `claude-opus-5-5` and terminal status `completed`; recorded combined cost USD 4.6590254. Their [brief](BRIEF.md), separate [reviews](review-01.md), prompts, raw JSON and logs are retained. This is a recommendation synthesis, not an adopted design or product evidence.

| Lens | Review | Principal edit |
| --- | --- | --- |
| Debt and payments | [1](review-01.md) | Obligation state machine; derive outstanding; bind discharge and origination to funded transfers and one-shot consent. |
| Collateral and price | [2](review-02.md) | Authenticated aggregate lock total, current price/head binding, typed fixed-width health comparison. |
| Liquidation and loss | [3](review-03.md) | Price-banded partial seizure, close factor, explicit impairment distinct from forgiveness, bounded waterfall. |
| Native path | [4](review-04.md) | Role-directed numeric operations, authenticated obligation/lock cells and a smallest no-oracle loan transition. |
| Adversarial plan | [5](review-05.md) | Every debit checks aggregate locked value; current oracle round, funded discharge and ledger-version concurrency. |

## Shared recommendations

1. **Freeze a real obligation transition.** `Obligation` fields alone do not define debt. State typed origination, accrual, partial payment, discharge, impairment and authorized forgiveness as distinct transitions. Derive `outstanding` from principal and accrued state; a payment must include a matching funded effect in the same accepted phase. A debt-reducing host flag or an unbound creditor write is insufficient.
2. **Make lock safety local to every debit.** The stated `Σ active locks ≤ balance` cannot be checked from a footprint that reads only one encumbrance. Add authenticated aggregate lock state or an equivalent completeness proof, update it atomically with encumbrance and balance state, and include it in every owner debit. Define reservation, commit, seize, release and cancellation; an encumbrance backing several obligations cannot be released after only one discharges.
3. **Bind risk decisions to current evidence.** `fresh(price)` alone can select a favorable old-but-fresh observation. The health rule needs a typed asset/price orientation, authenticated current oracle round or a signed selection policy, explicit stage-time semantics and checked arithmetic. A variable collateral amount times a variable price is outside the current generic Φ₀ grammar; a narrowly certified comparison is a candidate change, not an existing MIL/2 rule.
4. **Specify liquidation economics.** A keeper's signed `enforce` right must pair funded repayment, bounded collateral seizure, a close factor, bonus or price band, residual debt/lock and an impaired state when collateral is insufficient. Claim-class rank is not itself a waterfall. Separate forgiveness from recognition of a loss and name who funds any keeper reward or backstop.
5. **Prove the narrow path before market features.** A fixed-rate, one-debtor, one-creditor, single-domain loan with origination, partial repayment and full discharge can exercise the source/Core/native debt chain. Add collateral and price-conditioned liquidation in separate slices after the lock and oracle interfaces are demonstrated. Positive controls must be financially feasible; hostile controls should mutate one bound fact while keeping the envelope valid.

## Decisions and dissent

| Question | Reviewer positions | Reviewable resolution |
| --- | --- | --- |
| First native slice | [4](review-04.md) starts with a no-oracle loan lifecycle; [2](review-02.md), [3](review-03.md) and [5](review-05.md) emphasize collateral and liquidation as the category discriminator. | Use the no-oracle stage to establish funded L1, consent and native binding, then a separate collateralized and price-conditioned stage before claiming DeFi lending coverage. |
| Aggregate lock representation | [2](review-02.md) and [5](review-05.md) propose a `lockTotal` cell; [4](review-04.md) distinguishes custody from lien. | Specify both the asset custody cell and an authenticated aggregate lien/reservation commitment with an inductive update rule. Compare native cost and prove that no debit path bypasses it. |
| Liquidator ordering | MIL/2 asks for a signed total order of competing seizes. [5](review-05.md) recommends ledger head/version serialization for permissionless keepers. | State whether ordering is an owner-signed policy (priority) or a ledger concurrency rule. For first-come execution, bind a current encumbrance version to each proof and reject stale proofs. |
| Numeric profile | [1](review-01.md) keeps bounded accrual in literal-rate arithmetic; [2](review-02.md) and [5](review-05.md) propose one certified product comparison for health. | Keep general variable rate/index models deferred, but measure a single fixed-width health predicate under U1 and make its role-directed rounding explicit. |

The minimum acceptance sequence is: a funded and consent-bound loan transition; authenticated locks on all debit paths; current price binding; bounded liquidation with residual debt; then native proof and ledger readback with double-pledge, stale-price, forgiven-debt and concurrent-keeper hostile cases. The [MIL/2 freeze obligations](../../../concepts/intent-language/DESIGN-MIL2.md) and U0–U6 roadmap gates remain open. No tests, proofs or Preview transactions were run in these reviews.
