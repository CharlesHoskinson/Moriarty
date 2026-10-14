# Requirements, wiki and plugin reconciliation — 2026-10-14

This dossier records the requested currentness audit of Moriarty OpenSpec/EARS,
`wiki-llm` and the development plugin at PR50 baseline
`fc6353bf19b87a76410319c378efe529d52d23d9`.
It updates language documentation and maintainer guidance. No product, SDK,
formal-model, native or ledger execution occurred, and no roadmap exit closes.

## Current entrypoints

- [OpenSpec reconciliation](../../../../openspec/changes/consolidated-language-kernel/reconciliation-2026-10-14.md)
  preserves the existing sequence and maps delivered child capabilities to remaining work.
- [Current EARS index](../../../../openspec/current-requirement-crosswalk.json)
  retains 160 canonical requirement headings and separately inventories the 15 Opus-authored OSR headings.
  Neither index is an atomic acceptance denominator.
- [Wiki reconciliation](../../../../wiki-llm/RECONCILIATION-2026-10-14.md)
  connects the dated research, Rust Core, Node/TypeScript beta, tools and website source.
- [Plugin guidance](../../../../plugins/moriarty-dev/skills/develop/SKILL.md), version 0.1.1,
  follows the current request and GPT-6.1 Sol high / Muse Wonder high nonsecurity routing.
  All security work belongs to Opus 5.5.

## Coverage and preservation

Every one of 128 baseline OpenSpec files, 1,871 wiki-llm paths and 28 plugin files
has an explicit inventory/disposition. Methods are recorded per file in the three
`*-audit.json` manifests. These inventories are not claims of full semantic review
of every artifact. The OpenSpec lane reviewed 70 nonsecurity documents, four
unchanged templates and the two retention-validator sources; structural/historical
checks and security handoffs are separate. The wiki lane reviewed 44 nonsecurity
documents/prose records and handed 41 authored security/mixed records to Opus;
three frozen publication reports and raw/provider/native/private artifacts remain
inventory-only. Approximately 87 KB of proposed horizon contract fences were not
semantically revalidated. Plugin runtime files were inventoried/parsed and compared,
not security audited by GPT.

Original normative clauses and task text remain. The mechanical baseline inventory
contains 388 original normative lines across 85 documents, all still present.
The OpenSpec check additionally retains all 153 original canonical clauses, 659
checkbox lines, 239 original mapped tasks and 24 release gates. The three existing
machine registers remain byte-identical to baseline; current guidance lives in
maintained documentation. Original rosters, contracts, resource limits and terminal
records remain unchanged. ACTUS/DeFi membership and
full U/M/MC/SP/G obligations remain in force.

Frozen results, audits, receipts, mockups and design captures remain scoped to their
original trees. Maintained entrypoints explain superseding results and current
profile/target distinctions. Historical source-hash reconciliation uses an exact
Git-recovered snapshot rather than changing the original report digest.

## Verification and independent review

[root-verification.json](root-verification.json) records actual commands and results:
14 strict OpenSpec packages, 11 retention tests and nine plugin guidance/package
tests pass. The isolated package test reproduced 13 broken links before repair;
checkout references now resolve from the active checkout rather than the cache.
The actual plugin status path also reproduced a strict-loader rejection of an
added guidance field. The machine registers were restored instead of changing
runtime admission. Historical status blockers remain; they were not waived.
A root insertion produced one trailing empty line; its failed receipt and repair
are retained. Product/runtime tests and native operations were not run.

[Opus source authoring](OPUS-SECURITY-REQUIREMENTS.md) supplies the exact security
addendum and three maintained-family paragraphs. Its first-pass coverage limits
are explicit. A separate Opus coverage continuation addresses the public handoffs;
final independent reviews are retained alongside their frozen candidate identities.
This dossier does not claim a full end-to-end language or security audit.
The prior [runtime audit plan](../AUDIT-PLAN.md) remains owed.

The install receipt is retained separately after the reviewed plugin is published
and refreshed through the standard host CLI. Updating guidance does not establish
host hook interception or target qualification.
