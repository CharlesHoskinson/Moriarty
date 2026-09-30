# Independent Opus 5.5 recommendation brief — Staking, restaking and yield

You are one of five **sequential, independent** Claude Opus 5.5 agents. Do not read another agent's recommendation or synthesis. Review the MIL/2 specified-only design and recommend how to resolve the issues in the Staking, restaking and yield category. Your own assigned lens appears below.

Repository root: `/home/charl/Moriarty/.worktrees/mil2-primary-research-20260929`. Read/apply `AGENTS.md`, load `plugins/moriarty-dev/skills/develop/SKILL.md`, and run `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json` before review. This is read-only design work; do not edit files, run tests, proofs or compilation, or send transactions. A blocked campaign dispatch does not block this review.

Read in full: `concepts/intent-language/DESIGN-MIL2.md`, `deliverables/mil2-deep-research-2026-09-29/category-review/08-staking-yield.md`, `deliverables/mil2-deep-research-2026-09-29/MIL2-PROPOSED-SEMANTICS.tex`, `ROADMAP.md`, and `docs/MORIARTY-PRODUCT-CONTRACT.md`. The old `concepts/intent-language/CATEGORY-MAP.md` is superseded MIL/1 history. Use the applicable earlier R2-R8 category review via the links in the new category report as requirement context. You may search public web for *official primary* protocol standards/specifications as comparative best practice; cite the direct URL and distinguish external practice from a Moriarty rule. Relevant captured primary sources already exist under `deliverables/mil2-deep-research-2026-09-29/source-text/`.

Preserve bounded stages, checked finite-width arithmetic with explicit role-directed rounding, mandatory native proof-carrying acceptance, no host-computed acceptance Boolean, debt separate from supply, no global cross-domain rollback, and an optional federated kernel. If changing a U0 or deferred Φ₁ boundary, state exactly why.

Output Markdown only, maximum 1,800 words. Include: (1) verdict on category design fitness; (2) five ranked precise design edits, each with a rule/transition sketch, counterexample, and milestone/profile placement; (3) core versus library boundary; (4) smallest implementable slice and a valid positive/hostile evidence pair; (5) explicit disagreements; (6) residual assumptions. Cite file:line for decisive repository claims. Label recommendations as such. Do not claim implementation, proof or ledger acceptance.

# Your distinct focus

Compiler/native feasibility: narrow share-vault operation with certified arithmetic, authenticated totals and complete effects.
