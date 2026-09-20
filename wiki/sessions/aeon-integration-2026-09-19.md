---
id: moriarty.session.aeon-integration-2026-09-19
title: Aeon integration research and PL review
type: session
status: reviewed
created: 2026-09-19
updated: 2026-09-19
tags: [moriarty, aeon, language-design]
---

# Aeon integration research and PL review

This note preserves provisional session conclusions and open decisions. It is not accepted language policy.

The user requested an Aeon study, six PL expert reviewers (three Claude Fable 5.1 medium and three GPT-6 Astra medium), roadmap cleanup, and aligned OpenSpec, EARS and Foreman Pel planning. The user selected claude-obsidian for intermediate research notes.

Initial evidence and limitations are in [the research report](../../deliverables/aeon-study-2026-09-19/RESEARCH.md). Aeon is pinned at `ef66bd95e6b7d2d5309453ee63640bc7fa1d988f`. Moriarty is pinned at `1896217a28553e0254b0ba319422ac4c0b39ac0d`.

Provisional direction: use refinement-guided proposals, explicit trust assumptions, and counterexamples in an off-chain authoring workflow. Preserve exact financial types, bounded execution and the independent ledger/proof gates. Six-member review has not yet reached a decision.

Open questions: what should be implemented first, which guarantees can be checked statically, how should funding capabilities be represented, and how should one active delivery sequence retain all old acceptance obligations?

[[wiki/index|Research index]] · [[wiki/moriarty-architecture|Existing architecture]] · [[wiki/open-questions|Open questions]]

## User corrections and reviewed result

The user clarified permissionless deployment for all Midnight DeFi developers, formal provable intention, mandatory ZKIRv3 execution, Simplicity-style certified jets and APSS literature research using Scrapling, PixelRAG and Diátaxis. Earlier v1/v2 recommendations preserving all old gates are superseded.

All six requested experts endorse revision4 at SHA256 `5bbf8787a6a14e2342c7d04033acf20963c7350db29777615bc1d1ee84b2dc26`. [Consensus](../../deliverables/aeon-study-2026-09-19/review/FINAL-CONSENSUS.md). The panel required explicit compiler trust boundaries, adversarial witness conditions, phase-aware authority/fees and complete accounting. This is advisory design consensus, not proof or deployment evidence.

[APSS literature bank](../research/apss/index.md), [product contract](../../docs/MORIARTY-PRODUCT-CONTRACT.md), [roadmap](../../ROADMAP.md), [requirements](../../openspec/changes/permissionless-provable-intention/requirements.md), [Aeon graph](../../deliverables/aeon-study-2026-09-19/graph/graph.html). The original local architecture branch and jet studies remain preserved. Compilation/proof enhancements are specified for implementation; this session's corrections do not claim those features implemented.

## Completed research and revised roadmap

The [final delivery result](../../deliverables/aeon-study-2026-09-19/RESULT.md) supersedes the initial proposals above. Six experts explicitly endorsed the same revision. The [current roadmap](../../ROADMAP.md), [new OpenSpec/EARS](../../openspec/changes/permissionless-provable-intention/proposal.md) and [Pel implementation plan](../../openspec/changes/permissionless-provable-intention/implementation-plan.md) implement the documentation direction. Product proof/compiler work remains specified, not completed.
