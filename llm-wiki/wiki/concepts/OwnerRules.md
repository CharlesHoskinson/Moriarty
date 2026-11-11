---
title: "Owner rules"
type: concept
tags: [owner-rule]
sources: [agents-charter, general-purpose-definition, runlength-consensus-brief, k21-recalibration-brief, sherlock-terminology]
last_updated: 2026-11-11
---

# Owner rules

Standing rules from the factory charter and the owner's GP definition.

1. **GP-1 to GP-4 are never narrowed without the owner.** General purpose means all of GP-1 to GP-4: "Every requirement and task in this change is to be checked against GP-1 to GP-4. A narrowing needs an explicit owner decision." ([[general-purpose-definition]]; restated in [[runlength-consensus-brief]] as "Never narrow them without the owner.") See [[GeneralPurposeRecursion]].
2. **Commits** go to branch `main`, with no AI `Co-authored-by` trailers and no Claude attribution in commits or PRs ([[agents-charter]]).
3. **Reviews are inputs.** Nothing waits on a reviewer. The one review point is the T3 stagenet promotion ([[agents-charter]]).
4. **Heavy work** runs through `scripts/doctorwatson heavy` ([[agents-charter]]).
5. **Secrets:** never commit credentials, wallet seeds, private witnesses or unredacted environments; preserve existing wallets, keys and contract state ([[agents-charter]]).
6. **Product boundary:** maintainer tooling never becomes a prerequisite for public developers to compile, prove or deploy ([[agents-charter]]).
7. **Model CLIs:** subscription logins only; never set `OPENAI_API_KEY`, `CODEX_API_KEY` or `XAI_API_KEY` ([[agents-charter]]).
8. **Zero knowledge end to end.** Owner, 2026-11-11: "we need to make this zero knowledge or else its useless" ([[k21-recalibration-brief]]). A hard requirement at the level of GP-1 to GP-4. See [[ZeroKnowledge]].
9. **Terminology.** Sherlock is a universal, elastic proving co-processor for Midnight; owner: "don't lose the terminology" ([[sherlock-terminology]]).
