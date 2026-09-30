# MIL/4 working-draft review

**Date:** 2026-09-29. **Scope:** coherence of the working successor draft. These reviews do not decide M4-C1–C5, W-D0–W-D6, admit a DeFi profile, close U0, or accept an implementation or Midnight result.

| Candidate SHA-256 | Grok 4.6 high | GPT-6 Astra | Disposition |
| --- | --- | --- | --- |
| `94c89e3dc1f74a2bf1495d4d6225b66186c74f02d7c316a4c51755d415ce8134` | WARN; [raw response](grok-working-raw.json) names ten scope, trace and wording defects. | WARN; overbroad S0 W-D4 gate. | Revised the candidate. Earlier votes do not carry to new bytes. |
| `702503f47e5d261f218e5ce5b79e959dcab26399e76c3c5150f5edceb16824e0` | APPROVE, working-draft coherence; [raw response](grok-recheck-raw.json) finds no material remaining issue. | APPROVE, working-draft coherence; no material remaining issue. | Two independent working-draft reviews agree on the exact current candidate. No design choice is disposed. |

The Grok CLI was requested with `--model grok-4.6 --reasoning-effort high`. Both substantive JSON responses report `stopReason: end_turn` and `modelUsage` key `grok-4.6-build`; the CLI's self-description says Grok 4.6. The Astra agents were requested through host spawn as `gpt-6-astra` at medium effort. Their child contexts did not expose a separate returned backend identity or effort, so only the request metadata is recorded. Both agents reported completed terminal status and read the full candidate; the recheck independently confirmed the current hash before and after review.

Eight category-specific MIL/4 review assignments informed the candidate. AMM, stablecoin, derivative, oracle and governance assignments ran sequentially in one reusable agent thread; lending, bridge and staking used separate agents. Their findings are summarized in the eight rows of the [working draft](../DESIGN-MIL4-WORKING.md#3-eight-defi-profile-comparisons). The prior MIL/2 category reports and five-review Opus recommendation sets remain linked there. Current-turn category and preparation responses have no independently hashed on-disk raw receipts, so they cannot serve as required design-freeze votes or as native evidence. A design-freeze packet must retain their exact receipts or re-acquire reviewed evidence.

The formal preparation review found missing six-judgment instantiations, an ambiguous AMM fee discriminator, an underdetermined option trace and a bridge partial-receipt gap. The financial review checked the arithmetic and required complete accounting for AMM fees, CDP backing, option reserve, bridge remainder and vault rounding. The working draft now states these as live issues or bounded examples. It has not selected the policies or implemented their rules.

The next substantive step is the inherited S0 decision contract: W-D0–W-D3 and the S0 portion of W-D4. MIL/4 family design choices follow their dependency order. SP01.6 still occupies the blocked delivery slot. No test, proof, compiler, campaign dispatch or Preview transaction was run for this research and review pass.
