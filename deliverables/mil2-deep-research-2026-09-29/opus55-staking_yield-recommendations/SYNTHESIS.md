# Staking, restaking and yield recommendations from five sequential Opus 5.5 reviews

**Date:** 2026-09-29. **Baseline:** Moriarty `983a4bb49e3ccae399ee2514da2f6fa3f03593fd`. Five fresh read-only Claude Code sessions ran sequentially with requested and returned canonical model `claude-opus-5-5`. Each ended normally; aggregate CLI-reported cost is USD 4.349892. Prompts, raw responses, logs and extracted reviews are retained here. These are design recommendations, not accepted semantics or implementation evidence.

| Lens | Review | Main recommendation |
| --- | --- | --- |
| Vault accounting | [1](review-01.md) | Fix rounding by operation, authenticate pre-state pool totals, conserve shares and sign slippage bounds. |
| Validator and withdrawals | [2](review-02.md) | Add a request/queue/finalize lifecycle; keep unbonding stake slashable and settle rewards through checkpoints. |
| Restaking and slashing | [3](review-03.md) | Model per-service allocation, correlated loss budgets, replay-consumed verdicts and aggregate pool losses. |
| Compiler and native feasibility | [4](review-04.md) | Use certified quotient/remainder conversion, complete effects and authenticated managed totals. |
| Adversarial acceptance | [5](review-05.md) | Close donation/inflation, duplicate-reward, duplicate-slash, stale-queue and strategy-drift paths. |

## Shared recommendations

1. Make `deposit`, `mint`, `withdraw` and `redeem` distinct operations. Their rounding direction follows the operation, not an author-supplied `Rounding` parameter. A checked integer quotient/remainder witness can serve as a narrow transition primitive before general Φ₁ predicate authoring, subject to native cost and field-width evidence. Keep signed minimum or maximum outcomes in Φ₀.
2. Authenticate `managed` pool assets and total share supply in the pre-state. Treat unsolicited custody transfers as surplus until a declared transition accounts for them. Specify one immutable bootstrap policy with virtual offset or dead shares, and conserve account shares against total shares.
3. Add explicit reward index and per-holder checkpoints. Settle accrued rewards before any share write in the same stage, initialize new checkpoints at the current index, and fund rewards through issuance authority or a reserve transfer.
4. Represent withdrawal request, unbonding, finalization and claim as separate persistent transitions. A pending withdrawal remains locked and slashable. Finalization reserves liquidity and sets a declared rate policy; timeout alone cannot erase a slash liability.
5. Allocate stake to services under an aggregate exposure budget. Slash verdicts need typed evidence, replay keys and a loss rule that writes down managed assets once. A strategy mandate should bind venue caps, fees, loss and amendment policy to the signed intent.

## Choices that need explicit resolution

| Choice | Open alternatives or risk |
| --- | --- |
| Conversion arithmetic placement | Reviews [4](review-04.md) and [5](review-05.md) put a certified divmod transition before general Φ₁. This changes the MIL/2 U0/U1 boundary and requires measured native bounds. |
| Bootstrap defense | A virtual offset and dead-share burn are alternative policies. The numerical offset in [5](review-05.md) is a candidate, not a universal safe constant. |
| Withdrawal rate freeze | [2](review-02.md) and [5](review-05.md) recommend the lesser of request-time and finalization-time rates, with slashing retained through finalization. That rule needs a precise reserve and fairness analysis. |
| Restaking priority | [3](review-03.md) recommends origination priority plus seizure reservations instead of the current signed total order. It must be checked against the intended consent and loss allocation model. |
| Core versus library | Accounted totals, conversion checks, replay and complete footprints are core candidates. Queue parameters, reward curves, validator operations and strategy templates can be profile rules; their required cell tags still need reservation before U0 freeze. |

## First slice and acceptance evidence

A one-domain, one-asset, one-class vault with fixed bootstrap policy, deposit and redeem is the smallest common candidate. It must bind pre-state totals, funding and share effects; reject a forged quotient, unauthorized rounding direction, stale total, zero-output deposit and incomplete footprint with well-formed envelopes. Reward, queue, restaking and strategy profiles follow as separate slices. No test, proof, compilation or Preview transaction was run by these reviewers. The [MIL/2 pre-freeze obligations](../../../concepts/intent-language/DESIGN-MIL2.md) remain open.
