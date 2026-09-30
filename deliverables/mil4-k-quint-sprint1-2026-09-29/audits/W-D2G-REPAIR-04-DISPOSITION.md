# W-D2G repair-04 disposition

**Decision, 2026-09-30:** retain mapping A as the bounded specified-only identity candidate. It uses distinct agreement instance, stage, episode, action and Core-program identities, the current Source/6 builtin selectors, an explicit stage/episode wrapper, and B11 as sole current-head authority. This is not adopted authentication or W-D2 closure.

The exact [packet](w-d2g-grok-repair-04-candidate-packet.md) has SHA256 `acf54f334d395390c4293efcabde84ce7880672b54a5b643c1821db21253d7ed`. GPT-6.1 Sol high [recomputed its 11 manifest hashes](w-d2g-grok-repair-04-gpt-6.1-sol-high-audit.md) and found no high/medium defect. Grok was requested as 4.7 xhigh; its [raw result](w-d2g-grok-repair-04-grok-4.7-xhigh-raw.json) reports `grok-4.7-build` and no high/medium contradiction. Grok read embedded bytes only and did not independently recompute hashes. The G29 tag-6/tag-10 contradiction is resolved.

The 53 numbered cases are specified expectations: two positive fragment/membership controls and 51 rejection/control cases. No registry proof, identity verifier or consumer executed. B05–B17, W-D2 and Sprint 1 remain open. Historical W-D2E `actionId=Action` fixtures do not become mapping-A positives.

Low review residue for executable tests: tag-10 step 5 is unreachable as the first failure after prior tag-6/tag-10 comparisons; add an isolating row for genuine current-action fact mismatch, and pin binding/fact/source path detail for abbreviated Core-program rows. These do not change the frozen numbered outcomes.
