# Derivatives recommendations from five sequential Opus 5.5 reviews

**Date:** 2026-09-29. Five independent read-only sessions returned canonical `claude-opus-5-5`, all `completed`; combined reported cost USD 4.9795936. The [brief](BRIEF.md), prompts, raw receipts, logs and [five reviews](review-01.md) are retained. The rules below are proposals, not existing MIL/2 semantics or native evidence.

| Lens | Review | Main proposal |
| --- | --- | --- |
| Instrument and payoff | [1](review-01.md) | Nominal immutable terms, deterministic write-once fixing, total payoff and consumed exercise. |
| Margin and funding | [2](review-02.md) | Keep unrealized P&L out of balances; make funding a period-indexed obligation and partition collateral. |
| Liquidation and loss | [3](review-03.md) | Exhaustive non-erasing waterfall, bounded keeper rights, replay-safe seizes and explicit social-loss index. |
| Native feasibility | [4](review-04.md) | Dimensioned price arithmetic, total positive part and a bounded cash-settled option statement. |
| Adversarial plan | [5](review-05.md) | Reject wrong fixing, double exercise, sign errors, stale margin and erased shortfall. |

## Recommended first design cut

1. **Hash-bound instrument and position identity.** An instrument must bind underlying, strike, expiry, settlement domain, exercise style and payoff rule to an immutable nominal ID. Record each authorized side and claim so position conservation can be checked without treating a signed exposure as a spendable balance.
2. **Deterministic fixing and total payoff.** Use a single authenticated price round selected by a stated window/priority rule; make the fixing write-once. A typed positive-part and scaled-price operation must reject overflow or bad scale and name the rounding/remainder beneficiary. `fresh` by itself does not choose the correct fixing.
3. **One-shot exercise and durable settlement duty.** Exercise consumes a unique claim or marker; settlement creates a bounded payable liability and pays it with complete effects. If payment cannot complete in the same accepted stage, the liability persists. An option expiry cannot silently erase an unpaid duty.
4. **Later margin/funding profile.** Periodic funding is an indexed ratchet with no skipped or replayed period. Unrealized P&L is a valuation, not a balance. Margin rules need authenticated collateral reservations, initial and maintenance thresholds, a current mark policy, bounded liquidation and a shortfall waterfall. This is a separate profile from the first fully collateralized option.
5. **Native evidence.** The smallest proposed slice is one fully collateralized, cash-settled European call with a fixed observation policy, no variable funding, and one settlement. A valid witness and a wrong-round/double-exercise witness should have otherwise feasible envelopes. Exact instrument terms, fixing bytes, position/claim state, payoff, balance effects and tombstone must bind to the same native accepted stage.

## Unresolved design choices

| Choice | Positions from reviews | Next check |
| --- | --- | --- |
| Position representation | [1](review-01.md) favors a nominal instrument with exposure conservation; [5](review-05.md) proposes side-indexed unsigned legs to avoid sign ambiguity. | Specify a canonical mapping from long/short claims to `Position<I>` and prove a zero-sum law at issue and close. Do not let a negative exposure become a negative spendable balance. |
| Fixing selection | [1](review-01.md), [4](review-04.md) and [5](review-05.md) all require a write-once cell but differ in first-in-window detail. | State how the authenticated oracle round, stage time and finality interact, then give wrong-round and reorg controls. |
| First slice versus broader category | [1](review-01.md) and [4](review-04.md) prioritize a fully collateralized call; [2](review-02.md) and [3](review-03.md) expose the larger perps/margin/loss gap. | Label the call as a bounded derivative slice only. Do not promote its evidence to perps, portfolio margin or insolvency coverage. |
| Social loss | [3](review-03.md) proposes a lazy per-side loss index; the other reviews do not close a class-level waterfall. | Specify claimant completeness and priority before any ADL or socialized-loss claim. |

The [category coverage report](../category-review/04-derivatives.md) and [MIL/2 six freeze obligations](../../../concepts/intent-language/DESIGN-MIL2.md) remain open. No tests, proofs, compilation or Preview transactions were run in these reviews.
