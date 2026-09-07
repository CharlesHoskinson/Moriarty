---
id: source.llm-wiki-pattern
type: source
title: LLM Wiki research workflow
status: active
updated_at: 2026-09-02T17:18:27Z
sources:
  - SRC-0001
created: 2026-09-02
updated: 2026-09-07
tags:
  - moriarty
  - research
---

# LLM Wiki research workflow

**CLM-0001 — Source fact.** Karpathy's April 4, 2026 idea file defines three
layers: immutable raw sources, an LLM-maintained directory of interlinked
Markdown synthesis, and a schema/instruction document that governs maintenance.
It names ingest, query, and lint as the core operations and recommends a
content-oriented index plus an append-only chronological log. Authority:
descriptive design; scope: research method; reproduction: reproduced by local
acquisition; confidence: high; lifecycle: S2. [Receipt](../../raw/receipts/SRC-0001.md)

**Decision.** This repository adopts that core structure and adds fields needed
by the Marlowe assignment: source authority, version scope, S0–S7 lifecycle,
reproduction state, confidence, contradiction records, repository locks, and
experiment evidence. These additions instantiate the domain-specific schema
that the original idea intentionally leaves open.

The wiki is a synthesis layer, not a substitute for source inspection. Exact
details, code, numerical limits, and theorem statements must be verified against
the preserved primary artifact.
