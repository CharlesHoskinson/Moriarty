---
title: Moriarty beta language research and delivery loop
status: completed-authoring-beta
created: 2026-09-30
updated: 2026-10-14
---

# Moriarty beta language

## Current source relationship — October 14, 2026

This directory records the completed September 30 authoring-beta scope and its
research. The baseline for this reconciliation is merged PR50,
`fc6353bf19b87a76410319c378efe529d52d23d9`. Current product acceptance belongs
to the [roadmap](../../ROADMAP.md), not the historical checklist or design votes.

The Node [bridge](../../packages/moriarty-beta/src/bridge.ts) still imports the
experimental TypeScript Source/6 parser/preparer and returns local preparation.
Rust's [profile constants](../../crates/mori-core/src/ir.rs) instead declare
`moriarty/1` and `moriarty-core/1`; the [Rust compile command](../../crates/mori/src/commands/compile.rs)
is implemented. Node's [LSP/MCP services](../../packages/moriarty-beta/src/servers.ts)
are delivered authoring tools, while Rust's [LSP](../../crates/mori/src/commands/lsp.rs)
and [MCP](../../crates/mori/src/commands/mcp.rs) commands remain stubs.

The initial candidate, convergence, mockup, horizon, RESULT and developer-trial
records keep their original exact-byte review scopes. Their present-tense
observations refer to their recorded trees. [The current crosswalk](../RECONCILIATION-2026-10-14.md)
distinguishes selected proposals, delivered local tools and still-open language
work, including the later tutorial implementation in [`site/`](../../site/).

## Historical user contract

The user requests deep research on widely used language syntax and design,
domain decomposition and design choices, and developer experience from syntax
highlighting to IDE and AI coding plugins. Use Scrapling and research agents.
Record findings and design rationale here. After research, five independent
programming-language reviewers using GPT-6.1 Sol at medium effort challenge the
decisions. Converge on a beta language design, then implement it autonomously
while the user is AFK. An additional parallel thread researches meta tools and
frameworks for language development. This authorizes the research, design,
implementation, repair and verification loop without intermediate questions.

These notes contain inspectable findings, alternatives and decisions, not
private deliberation. They are research/delivery records under `wiki-llm` and
do not automatically update the canonical `wiki` claim ledger.

## Evidence and limits

- Baseline: `870f998b36ecda04622fa4274132e74902942d0b`.
- Branch: `feat/moriarty-beta-20260930`, isolated linked worktree.
- Nine domain researchers, at most three concurrent workers alongside the lead.
- Main research: three acquisition rounds, at most five fresh captures each.
- Meta-tool thread: one additional round, at most five fresh captures.
- Ecosystem thread added by the user: one additional round, at most five captures.
- Existing repository source captures may be reused with their original dates.
- Public network consent follows the explicit Scrapling/deep-research request.
- Official language manuals, protocol specifications and vendor documentation
  are the approved source classes; popularity uses explicitly named surveys or
  platform measurements. No private source content is sent to external services.
- Scrapling 0.4.15 is installed and is the latest version observed on PyPI.
- Raw public-document captures and immutable retrieval receipts reside under
  `/home/charl/research/moriarty-beta-2026-09-30/`; the reproducible capture helper
  is `capture.py`. Repository notes cite URLs, sections, hashes and receipt paths.
- Stop acquisition at budget exhaustion or when evidence repeats; record gaps.
- No syntax, toolkit popularity or vendor AI quality claim follows from taste.

## Domains

1. [Surface syntax and sugar](domains/01-surface.md).
2. [Types and financial resources](domains/02-types.md).
3. [Agreements, effects and authority](domains/03-agreements.md).
4. [Stages, concurrency and asynchronous workflows](domains/04-workflows.md).
5. [Modules, packages, testing and documentation](domains/05-toolchain.md).
6. [Syntax highlighting, language servers and IDEs](domains/06-editors.md).
7. [AI coding integrations](domains/07-ai.md).
8. [Language-development meta tools](domains/08-meta-tools.md).
9. [Blockchain-native notation and financial primitives](domains/09-ecosystems.md).

[Java/C#/C++ supplement](domains/10-popular-syntax-addendum.md) closes the named
feature-documentation gap with a bounded additional three-target primary-source
round. It adds no new popularity ranking or beta grammar. Its inventory remains
separate, preserving the original acquisition record.

## Delivery sequence

1. Acquire evidence and finish the nine domain memos.
2. Compare alternatives and freeze a cited candidate beta design.
3. Run the five requested independent medium-effort PL reviews on that candidate.
4. Reconcile every substantive challenge; preserve dissent and scoped verdicts.
5. Record the converged beta grammar, desugaring boundaries and implementation plan.
6. Implement the parser, authoring checks, executable supported slice and tools.
7. Verify real command paths and adversarial cases; repair failures and review changes.
8. Record exact release status, remaining formal/native obligations and evidence.

## Acceptance boundary

The [programmer-facing mockup requirements](../../docs/language/PROGRAMMER-FACING-MOCKUP-REQUIREMENTS-2026-09-30.md)
control the broad eight-family examples and visible local/specified/open labels.
For this Node beta, experimental TypeScript Source/6 and Core/5 S0 transfer and
funded AccrualFirst repayment are the local preparation baseline. A beta authoring frontend may improve syntax and
tooling but must preserve complete signed limits and ordered effects. It must
reject unsupported execution requests explicitly. The product remains the
permissionless Midnight language with an optional kernel; proof, signature,
authenticated state and ledger commitment gates retain their exact requirements.

## Loop state

Research and five PL reviews are complete. [Synthesis](SYNTHESIS.md),
[convergence amendments](CONVERGENCE.md) and [implementation plan](IMPLEMENTATION-PLAN.md)
select the beta. The original [candidate](BETA-DESIGN.md) and all reviews remain
unchanged evidence of their recorded scopes. [Programmer mockup](PROGRAMMER-MOCKUP.md)
and [typed full-language horizon](FULL-LANGUAGE-HORIZON.md) preserve all eight
families and the explicit local/specified/open split.

Implementation and six developer trials are complete. [Final result](RESULT.md)
records the captured September 30 measurements, exact review scope and publication state.
[DevEx results](DEVEX-RESULT.md) and the [tutorial site plan](TUTORIAL-SITE-PLAN.md)
preserve the feedback and onboarding decisions. Historical baseline counts do
not establish beta acceptance.

The authorized authoring-beta delivery loop is complete: [PR11](https://github.com/CharlesHoskinson/Moriarty/pull/11) is merged and the [prerelease](https://github.com/CharlesHoskinson/Moriarty/releases/tag/moriarty-authoring-beta-0.1.0-beta.1) is published with verified downloads. Financial/native product obligations remain open.

[Complete task checklist](CHECKLIST.md) separates delivered beta work from open product obligations and unperformed optional validations.
