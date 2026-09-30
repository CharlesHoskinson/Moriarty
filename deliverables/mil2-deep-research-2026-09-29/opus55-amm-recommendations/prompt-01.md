# Independent Opus 5.5 AMM recommendation brief

You are one of five **sequential, independent** Claude Opus 5.5 agents. Review the current MIL/2 design against the AMM and exchange category. Do not read another agent's recommendations. Give recommendations for resolving the issues in the AMM category review; do not merely repeat its findings.

Repository root: `/home/charl/Moriarty/.worktrees/mil2-primary-research-20260929`. Read and apply `AGENTS.md`; load `plugins/moriarty-dev/skills/develop/SKILL.md` and run `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json` from the root before review. This is a read-only recommendation task; blocked campaign dispatch does not block it. Do not edit any file, run tests, prove, compile, or submit transactions. Do not use web search; the bounded primary-source corpus is already in the repo.

Read in full: `concepts/intent-language/DESIGN-MIL2.md`, `deliverables/mil2-deep-research-2026-09-29/category-review/01-amm-exchange.md`, `deliverables/mil2-deep-research-2026-09-29/MIL2-PROPOSED-SEMANTICS.tex`, and `ROADMAP.md`. Consult the prior `concepts/intent-language/CATEGORY-MAP.md` as superseded MIL/1 history, and `docs/MORIARTY-PRODUCT-CONTRACT.md` for binding product constraints. Use relevant primary-source captures under `deliverables/mil2-deep-research-2026-09-29/source-text/` only if needed. Cite repository file:line for decisive claims.

Preserve: bounded stages; checked finite-width arithmetic with exact role-specific rounding; mandatory native proof-carrying acceptance; no host-computed acceptance Boolean; debt separate from supply; no claim of global cross-domain rollback; the federated kernel optional; U0 Φ₀ / deferred Φ₁ and first-profile single-signer boundaries unless you explicitly recommend and justify a change.

Output Markdown only, at most 2,200 words. Include: (1) concrete verdict on AMM design fitness; (2) top five ranked changes as precise design edits, each with a small normative rule or transition sketch, counterexample addressed, and milestone/profile placement; (3) what can stay a library versus what must be core; (4) one smallest implementable AMM slice and a positive/hostile evidence pair; (5) disagreements with the category review or MIL/2, if any; (6) remaining risks and assumptions. Label every claim as source fact, design inference, or recommendation where ambiguity matters. Do not claim that a paper or graph edge proves Moriarty behavior.


# Your distinct focus

1. **Core semantics and typing:** exact pool state, `pre/post`, asset/share identity, transfer/effect/footprint derivation, linear resources, complete stage relation. Seek type errors and unsound rules.
