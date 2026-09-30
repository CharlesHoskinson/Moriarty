# Stablecoin and synthetic recommendations from five sequential Opus 5.5 reviews

**Date:** 2026-09-29. Five independent read-only reviews returned canonical `claude-opus-5-5`, all with `completed` terminal status; combined CLI-reported cost USD 4.906285. [Prompts](BRIEF.md), raw receipts, logs and the [five reviews](review-01.md) remain separate. These are recommendations, not an adopted design or native evidence.

| Lens | Review | Principal recommendation |
| --- | --- | --- |
| Issuance and supply | [1](review-01.md) | Derive supply changes from typed mint/burn effects and bind a quantity-limited issue right. |
| Collateral and solvency | [2](review-02.md) | Authenticate aggregate ceilings and a typed collateral-value comparison; link debt to issuance. |
| Redemption and shutdown | [3](review-03.md) | Model redemption as request custody, queued duty and eventual burn; add one-way system modes. |
| Native path | [4](review-04.md) | Bind issuer, supply, debt, collateral and price cells to one accepted stage statement. |
| Adversarial plan | [5](review-05.md) | Require a backing co-effect for positive supply, per-line authority and hostile unbacked-mint controls. |

## Shared recommendations

1. **Make supply an effect, not a claimed field.** Specify typed `mint` and `burn` effects. Derive `Δsupply` from them and require a scoped, authenticated `issue` authority for each positive line. Keep burns from silently replenishing a mint quota unless the signed policy explicitly defines that replenishment. The balance conservation equation then checks actual effects against the derived supply change.
2. **Link issuance to an explicit backing claim.** A CDP mint must atomically create or increase a separate debtor obligation and collateral lock; reserve-backed or unbacked issuance must name its distinct backing mode. Debt remains separate from token supply. An authorized mint and local conservation alone do not establish solvency. Authenticate aggregate debt ceilings and reservations so concurrent draws cannot both pass against a stale total.
3. **Pin risk inputs and numeric roles.** Collateral valuation needs a typed price orientation, source identity, authenticated value, policy version, and conservative rounding. A narrow certified cross-asset comparison is a candidate before general Φ₁, but its width and native cost must be measured. A reserve attestation names a trust premise; it does not establish an off-chain bank balance by itself.
4. **Specify redemption and crisis states.** A redemption request must reserve custody or a claim, retain the duty while pending, and burn only at the defined settlement point. A system mode such as active, guarded or shutdown must gate every mint and redemption transition and change one way under an authorized rule. A shortfall needs a frozen, complete claimant/reserve snapshot and a pull-based pro-rata or priority rule; an escrow timeout alone cannot deliver that system-wide guarantee.
5. **Choose a smallest native slice.** One domain, one issuer, one collateral asset, one debt asset, a fixed ceiling and no imported reserve evidence can show a funded CDP draw and matching mint. A hostile positive-supply mutation with no matching authorized debt/lock must fail while all unrelated envelope fields remain valid. Redeem/burn, price stress and shutdown can follow as separate slices.

## Open choices and dissent

| Question | Alternative proposals | Required decision evidence |
| --- | --- | --- |
| Backing model | [1](review-01.md) allows an explicit unbacked bucket; [5](review-05.md) requires every positive supply delta to carry a declared backing co-effect. | Define issuer classes and exactly which signed policies may create unbacked supply. Keep the exception visible in the native statement and conservation report. |
| Solvency predicate | [2](review-02.md) and [4](review-04.md) propose a certified narrow priced comparison before general Φ₁; current MIL/2 defers variable products. | Bound widths and compare valid/adversarial native witnesses. Do not assert a category-level CDP without this or an explicitly narrower literal-price policy. |
| Rebase and shutdown | [3](review-03.md) proposes share-index accounting and a one-way mode; the other reviews prioritize issuance and debt first. | Treat rebasing and system-wide shutdown as separate profiles after core mint/burn and claimant completeness are specified. A finite stage cannot enumerate all holders by implication. |
| Circular backing | [2](review-02.md) proposes bounded backing edges and a cycle guard. | Decide whether cycle rejection is a core admission rule or a signed risk disclosure/policy. Acyclicity alone does not prove economic solvency. |

The [category coverage review](../category-review/03-stablecoins.md) and [MIL/2 design](../../../concepts/intent-language/DESIGN-MIL2.md) remain specified-only. U0 must settle effect/authority/state identities; U1/U2 need native evidence for a narrow mint; later profiles need redemption, impairment and supply-family conformance. None of the five reviews ran tests, proofs, compilation or Preview transactions.
