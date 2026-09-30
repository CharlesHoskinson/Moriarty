# W-D2H repair-04 disposition

**Decision, 2026-09-30:** retain the purpose-1 Source definition, purpose-3 raw Core package and purpose-4 linked policy images as the bounded specified-only W-D2H candidate. This does not adopt image producers or authenticated B05–B07 semantics.

The exact [packet](w-d2h-grok-repair-04-candidate-packet.md) has SHA256 `1e47bfc15567b12630da95814807caeffe739776a42cb9041fcfebf1b5fe1a3b`. GPT-6.1 Sol high [recomputed its 14 manifest hashes](w-d2h-grok-repair-04-gpt-6.1-sol-high-audit.md) and reported no high/medium defect. Grok was requested as 4.7 xhigh; its [raw result](w-d2h-grok-repair-04-grok-4.7-xhigh-raw.json) reports `grok-4.7-build` and no high/medium contradiction. Grok read embedded bytes only and did not independently recompute hashes.

The two construction positives, one presentation invariant and 28 hostile/invariance cases are specified predicates, not executions. The repaired tag-8 constructor and tag-16 diagnostic order agree within this packet. No encoder, selected-artifact provider, loaded-code correspondence, signature, B16 history transport or B17 ledger result is established by the design packet. W-D2, W-D3, B01–B17 and Sprint 1 stay open.

Low static-review residue for later implementation tests: make package unpaired-kind failure explicit at tag 11; pin purpose-4 actionId/kind admission separately from signed tag-35 equality; retain the keyRef image-domain step in any summary of tag 12. These do not change a numbered oracle in the frozen packet.
