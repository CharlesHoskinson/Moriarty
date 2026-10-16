# Moriarty wiki-llm


Current security scope is maintained separately in [the Opus security currentness guide](SECURITY-CURRENTNESS-2026-10-14.md).
This directory preserves architecture research, design alternatives, dated delivery
narratives and their evidence. It does not replace the adopted language, the
canonical `wiki/` provenance ledger or the [delivery roadmap](../ROADMAP.md).

## Current reading — October 14, 2026

The reconciliation baseline is merged [PR50](https://github.com/CharlesHoskinson/Moriarty/pull/50),
commit `fc6353bf19b87a76410319c378efe529d52d23d9`. This is a source/documentation
observation; it records no new runtime, native or target result.

- [Current alignment and retained October 1 review](ALIGNMENT-REVIEW-2026-10-01.md)
  separates delivered child capabilities from open U0–U7/M1–M17 exits.
- [Research-to-current-source reconciliation](RECONCILIATION-2026-10-14.md)
  maps the research, profiles, tooling and remaining work to concrete source.
- [Mid-roadmap review](../docs/audits/2026-10-14/MID-ROADMAP-REVIEW.md) and
  [audit plan](../docs/audits/2026-10-14/AUDIT-PLAN.md) supply current evidence
  classes, next demonstrations and the unchanged native execution boundary.
- [Rust native Core guide](../docs/NATIVE-CORE.md) describes `moriarty/1` /
  `moriarty-core/1` and its compile, reference and static-call consumers.
- [Node authoring beta](beta-language-2026-09-30/README.md) describes the
  separate beta → experimental TypeScript Source/6 → Core/5 preparation path.
  Its editor/MCP delivery does not implement Rust's `mori lsp` / `mori mcp` stubs.

The roadmap's latest M1–M17 rows are direct user requirements. Preview receipts
remain scoped to Preview; Fenrir network/Anchor, committee/registry identities
and a qualified target tuple remain unestablished. Neither target substitutes
for the other. Full native, recursion/private history, library conformance and
release acceptance remain open under their retained exits. No completion
percentage follows from this directory.

## Research and delivery navigation

| Record | Read it for | Currentness and evidence class |
| --- | --- | --- |
| [Sherlock implementation journal](sherlock-implementation-journal/README.md) | Findings, implementation choices, evidence and open questions recorded after each iteration | Dated working notes; source, local results and acceptance remain separate |
| [Circuit atlas concept log](sherlock-circuit-atlas/README.md) | Opus-authored visual notes for Moriarty circuit and PCB scenes, updated with each frozen circuit iteration | Planning concepts grounded in source pins; no website implementation or circuit validation |
| [Beta research and delivery](beta-language-2026-09-30/README.md) | Ten domain memos, design convergence, authoring package and six developer trials | September 30 research and captured local results; use the package/source for current behavior |
| [Typed full-language horizon](beta-language-2026-09-30/FULL-LANGUAGE-HORIZON.md) | Proposed ADTs, arithmetic, libraries and eight-family lifecycle interfaces | Hash-bound specified-only design; no implementation follows from checked coverage boxes |
| [Website work](website-2026-10-01/PLAN.md) | Tutorial implementation and recorded source/result publication review | Dated local captures; current site source is [`site/`](../site/); no fresh live deployment observation here |
| [Native financial history](native-financial-kernel-2026-10-01/README.md) | Exact October 1 build/preparation/failure and scoped later result records | Historical public narrative; original artifacts and native authority remain separately scoped |
| [Signed-intent history](signed-intent-2026-10-01/README.md) and [primitives](midnight-crypto-2026-09-30/README.md) | Original protocol/design/delivery scope | Dated evidence; current security judgments belong to [Opus](../docs/audits/2026-10-14/SECURITY-ROUTING.md) |

## September 29–30 research archive

- [MIL/4 S0 executable boundary and next decision](mil4-s0-executable-boundary-2026-09-30.md): exact local S1B and effect-commitment review scope, surviving wire/semantic gaps, and a concrete consumer recommendation.
- [MIL/4 K semantics surge recommendation](mil4-k-semantics-surge-2026-09-29.md): compiled specified-only K, nine-seat audit, explicit family admission boundary and recommended M4-C1–C5 order.
- [Kernel API recommendation](kernel-api-recommendation-2026-09-29.md): concrete architecture choices, first adapter, open-source stack, implementation order and acceptance criterion.
- [Kernel API and interface design candidate](kernel-api-consensus-2026-09-29.md): nine substantive architect memos, model receipts, preserved disagreements, a bounded interface probe, and an independent audit. Raw proposals and prompts are retained under [`deliverables/kernel-api-architects-2026-09-29/`](../deliverables/kernel-api-architects-2026-09-29/).
- [Independent audit](kernel-api-audit-2026-09-29.md): initial requested changes, revisions, and approval limited to a research candidate.
- [Aeon kernel experiment evidence](aeon-kernel-experiment-evidence-2026-09-29.md): authority, bridge, and signing probes with explicit trust limits.
- Prior [NEAR kernel scope proposal](../deliverables/near-kernel-scope-2026-09-29/PROPOSAL.md), [Hyperliquid comparison](../deliverables/hyperliquid-kernel-comparison-2026-09-29/COMPARISON.md), and [Aeon experiments](../deliverables/aeon-kernel-experiments-2026-09-29/) are inputs. Their evidence and limits remain separate.
