# Moriarty intent prompt council advisory

Date: 2026-09-03

Status: advisory review completed; one correction pass applied

Semantic scope: unchanged at `0.0.0-e00.2`

## Outcome

Grok, Sol, and Fable all completed the same blind review contract and requested changes. The correction pass produced prompt version 1.2. The council did not approve an implementation or a semantic-scope change; it strengthened the research contract that must precede either decision.

| Reviewer | Requested identity | Observed identity | Verdict | Severity |
|---|---|---|---|---|
| Grok | `grok-4.6` | `grok-4.6-build` | changes requested | medium |
| Sol | `gpt-5.6-sol` | `gpt-5.6-sol` | changes requested | high |
| Fable | `claude-fable-5-1` | `claude-fable-5-1` | changes requested | medium |

The Fable call also reported a `claude-haiku-4-5` helper. The canonical substantive model remained `claude-fable-5-1`, and its readiness canary returned `FOREMAN_FABLE_5_1_READY_V1`. Grok completed one substantive `grok-4.6-build` call and is counted as a full council member.

The frozen blind brief is [moriarty-intent-prompt-blind-review-2026-09-03.md](../raw/council/moriarty-intent-prompt-blind-review-2026-09-03.md), SHA-256 `f260cf0f933dfed78407552566877a5ddd4d58c1fdd0fd5ffd7541571fc75b6a`. Reviewers had no web or tool access. The normalized evidence record is [moriarty-intent-prompt-council-2026-09-03.json](../evidence/moriarty-intent-prompt-council-2026-09-03.json). It is not a raw transcript or an independent audit.

## Corrections applied

The council required seven common improvements:

1. Bind each lifecycle artifact to its predecessor digest, execution state, domain, code identity, assumptions, and version.
2. Evaluate authorization at execution state and distinguish `SignAfterResolve` from `SignBeforeResolve`.
3. Index evidence from internal-ledger transition through destination spendability. Do not infer final settlement from a fill.
4. Specify cancel-or-fill races, residual authorization, signed refund destinations, delivery deduplication, typed reversals, and compensation.
5. Require typed verification certificates or structured rejections before signing, submission, claim, or refund. A bare Boolean is insufficient.
6. Separate confidential execution into its own trusted-computing-base, leakage, and theorem profile. An operator attestation is not a proof.
7. Keep CAKE, deployed NEAR behavior, current ERC-7683, prior ERC-7683, and OIF in five distinct status and evidence rows.

Prompt version 1.2 incorporates these changes. It contains 12 workstreams, 13 experiments, 18 deliverables, 12 evidence-gated sprints, 18 release gates, and 38 proposed intent data contracts. The new SDK objects and operations remain `specified-only` until an accepted OpenSpec change modifies the frozen 65-component and 28-data-contract inventories.

## Preserved dissent and limit

Sol assigned high severity because the draft did not yet require a cryptographic refinement chain and typed verifier certificates. Grok and Fable assigned medium severity but independently found the same settlement, authorization, cancellation, and confidentiality boundaries. The council reviewed a research prompt, not an implementation. Its verdict does not establish that Moriarty is sound, usable, compilable to Compact, or correct on Midnight.
