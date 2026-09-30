# Oracle recommendations from five sequential Opus 5.5 reviews

**Date:** 2026-09-29. Five independent read-only sessions returned canonical `claude-opus-5-5`, all `completed`; combined reported cost USD 4.6287876. [Brief](BRIEF.md), prompts, raw receipts, logs and [five separate reviews](review-01.md) are retained. All rules below are recommendations for a specified-only design.

| Lens | Review | Main proposal |
| --- | --- | --- |
| Identity and provenance | [1](review-01.md) | Identity-keyed source sets; separate data origin, attestor and authenticated read locus. |
| Aggregation and TWAP | [2](review-02.md) | Bounded distinct sample collections, counted median and authenticated accumulator/window semantics. |
| Imported/attested policy | [3](review-03.md) | Digest-bound evidence policy, issuer epochs/revocation and typed finality outcome. |
| Native binding | [4](review-04.md) | Exact observation tuple and signed comparison clause in the public/transcript relation. |
| Adversarial plan | [5](review-05.md) | Verifier-assigned labels, branch-independent admission and replay/status controls. |

## Shared recommendations

1. **Give every observation a canonical identity.** The record must bind feed/subject, typed value and unit, origin domain, source/attestor, policy digest, observation and inclusion times, round/sequence, finality/status and content commitment. Provenance should track observation IDs, not just broad classes such as `anchored@d`.
2. **Assign trust labels at a verification locus.** A witness may supply bytes, but cannot choose `anchored`, `imported` or `attested`. The verifier or authenticated ledger read produces the classification after checking the policy. Imported evidence remains a named, deferred premise until the foreign verifier and finality rules are specified.
3. **Validate all named observations at admission.** A Boolean short-circuit or unused branch must not let a malformed signed observation bypass freshness, issuer, status or domain checks. Separate mandatory admission checks from expression evaluation, then prove source-set propagation through every operator.
4. **Bind time and status to the accepted head.** Freshness requires an authenticated stage time and a stated tolerance for clock domains; a merely asserted `observedAt` is inadequate. Revocation/key epochs and replay state must be checked against the same acceptance head. Threshold evidence must count distinct authorized issuers when the policy claims issuer diversity.
5. **Keep aggregation bounded and typed.** Median/TWAP need a finite distinct sample set or an authenticated accumulator with explicit update and time-weighting rules. A host-computed average is an observation under its own trust policy, not a proved aggregation of anchored samples.

## Choices and limits

| Choice | Reviewer positions | Resolution work |
| --- | --- | --- |
| Finality representation | [3](review-03.md) wants finality as a typed verification outcome rather than a free Φ guard; [1](review-01.md) and [4](review-04.md) emphasize its binding in the observation tuple. | Define which component establishes finality per domain and what a later reorg or revocation can invalidate. Make `final(obs)` a projection of checked status if retained. |
| Reuse versus consumption | [3](review-03.md) proposes linear/affine reuse by evidence kind; [5](review-05.md) requires replay/status consumption for one-shot claims. | Distinguish reusable price reads from consumable delivery, reserve or entitlement receipts. Do not globally consume every observation. |
| Φ₀ arithmetic | [4](review-04.md) proposes a narrow scaled-price comparison atom, changing the existing deferred boundary. | Specify widths, units, rounding and native constraints; admit only after a positive and hostile witness under U1/U2. General nonlinear Φ remains deferred. |
| First aggregation | [2](review-02.md) gives both finite-sample median and accumulator TWAP routes. | Start with one authenticated scalar price-conditioned stage; treat median/TWAP as separate profiles after observation identity is proved. |

The first acceptance slice should use one anchored typed price and one signed threshold, with current authenticated value/time/policy and exact stage effects. Hostile witnesses should alter the value, unit, source class, round, status or claimed timestamp one at a time while leaving the financial envelope valid. The [category review](../category-review/05-oracles.md), [MIL/2 obligations](../../../concepts/intent-language/DESIGN-MIL2.md) and native source-to-ledger correspondence remain open. No tests, proofs, compilation or Preview transactions were run in these reviews.
