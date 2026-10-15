# Requirements, wiki and plugin reconciliation — 2026-10-14

This dossier records the requested currentness audit of Moriarty OpenSpec/EARS,
`wiki-llm` and the development plugin at PR50 baseline
`fc6353bf19b87a76410319c378efe529d52d23d9`.
It updates language documentation and maintainer guidance. No product, SDK,
formal-model, native or ledger execution occurred, and no roadmap exit closes.

## Delivered status

Reviewed source: `9bd7ccc1b5522b22cf633b46cdad080d24448f39`, projection
`15a69be761f4d97312725a1a683df7e8541ddc367acbc9740d097f0340e1a762`.
Fresh [Muse composition review](MUSE-APPROVED-CANDIDATE-REVIEW.md) and
[Opus applicable security review](OPUS-APPROVED-CANDIDATE-REVIEW.md) approve
their stated scopes. Earlier full reviews and corrections remain below.

Plugin **0.1.1 is installed and enabled**. The actual
[upgrade receipt](plugin-installation.json) records installer exit 0,
`restorationComplete: true`, `installedVerified: true`, exact 37-file
source/installed matches and preservation of the original 35-file cache.
The [post-install receipt](plugin-post-install-verification.json) records
25 compatibility tests passing with actual exit 0, matching hashes before/after,
no bytecode, and no HOME or CODEX_HOME override. These actual results received
fresh [Muse](MUSE-INSTALL-RESULT-REVIEW.md) and
[Opus](OPUS-INSTALL-RESULT-REVIEW.md) approvals within their separate scopes.

The earlier 211-test run and its 20 exclusions retain their original scope.
The later installed suite supplies separate evidence for five previously excluded
InstallationCompatibility cases. No full 231-test-suite pass is claimed.
The cache's filesystem mode observation is recorded separately; no operating-system
read-only guarantee or host hook interception is inferred. All native, target,
product and roadmap acceptance limits remain unchanged.

Final publication adds only these receipts and review records to the frozen
source. Source-phase statements below that say installation/review is pending
are retained history, superseded by this delivered status. The first final Opus
provider error produced no verdict; the readiness canary and subsequent fresh
reviews are recorded separately. No credential-status output is published.

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
are retained. Language-product tests and native operations were not run. Opus separately records the plugin runtime verification below.

[Opus source authoring](OPUS-SECURITY-REQUIREMENTS.md) supplies the exact security
addendum and three maintained-family paragraphs. Its first-pass coverage limits
are explicit. The [Opus coverage continuation](OPUS-SECURITY-COVERAGE.md) inspected all
64 handed-off OpenSpec entries and the wiki handoff
(34 authored paths plus eight exact-claim locations). It does not approve its own
requirements. Its per-file limits and corrections supersede the first pass
where explicitly stated; the first public output remains unchanged.
Independent review of candidate `f14df735` produced
[Muse nonsecurity approval](MUSE-NONSECURITY-REVIEW-R2.md) and an
[Opus security request for changes](OPUS-INDEPENDENT-REVIEW.md).
The latter found an installed/repository plugin-runtime divergence. Opus owns
the source reconciliation and all security corrections before fresh reviews.
This dossier does not claim a full end-to-end language or security audit.
The prior [runtime audit plan](../AUDIT-PLAN.md) remains owed.

Installation remains pending independent review of the reconciled package. The
existing source installer must preserve active cached runtimes; bare replacement
from the older repository runtime is superseded by the Opus review. The
[public plugin inventory](plugin-pre-reconciliation-inventory.json) records that
the current mutable source and installed source files match. The original
`plugin-audit.*` files describe the earlier guidance-only candidate; they do not
approve the later runtime reconciliation. Installation evidence will record its
actual reviewed source and result. No hook interception or target qualification
is inferred from package refresh.

> **Plugin runtime scope (corrected 2026-10-14).** The plugin change is no longer guidance-only. Opus reconciled the hardened runtime already deployed from the personal source as `0.1.0+codex.20260912173932` into the canonical `plugins/moriarty-dev/`. This adds the pinned `python3 -I` hook bootstrap (runtime digest `0b40d8c3…`), the `bootstrap`, `hook_protocol`, `compatibility` and `deployment` modules, the hardened `hook`, `cli` and `store` modules, `COMPATIBILITY.md`, and the compatibility and upgrade tests. Runtime Python files and `hooks/hooks.json` are byte-identical to the deployed source. Manifest 0.1.1 and the current guidance are kept, and a new read-only test requires registrations to match the generated pin. Opus's receipts, exclusions and finding dispositions are in [OPUS-PLUGIN-RECONCILIATION.md](OPUS-PLUGIN-RECONCILIATION.md). Installation is pending an independent review of this candidate and must use the source `deployment.py install` path, not a bare `codex plugin add`. Host hook interception remains unverified, native authority remains zero, and no product or milestone acceptance follows.

## Integrated source identities

For the integrated candidate, use [integrated-source-manifest.json](integrated-source-manifest.json),
[final-verification.json](final-verification.json) and
[final-navigation.json](final-navigation.json). The source projection excludes
only append-only reconciliation receipts to avoid self-reference; it does not
establish semantic or security acceptance. All 686 checked maintained navigation
targets resolve. The original `source-navigation.json` remains an earlier
`b585967f` receipt. Earlier author manifests, including the pre-integration runtime
copy of `wiki-audit.json`, retain their original review scopes; the integrated
manifest records current source bytes. The plugin source manifest is the exact
37-file Opus manifest, verified separately. No frozen provider result is relabelled
as a review of later bytes.

## M5 correction and review supersession

The [fresh Opus review of f2923e22](OPUS-SECURITY-FINAL-REVIEW-R2.md)
found no blocking runtime/upgrade defect but requested one blocking requirements
correction. Its exact OSR-1014-11 text is applied: 64 KB per transaction, fraud
k ≤ 19 and the per-contract replay budget remain mandatory. Measurement cannot
relax them. Existing JSON/envelope/source limits do not implement the transaction
requirement. The source author's earlier statement that the transaction ceiling
was wrong is superseded by this correction; its original report is preserved.
The plugin's exact 37 source files remain unchanged. Fresh review of this narrow
correction is recorded separately from the f2923e22 reviews.

The [Muse review of f2923e22](MUSE-NONSECURITY-FINAL-REVIEW.md) approved its
nonsecurity scope. Its introductory sentence calling the security coverage
continuation pending is stale: that continuation had completed and is retained
above. The outstanding item at that time was the independent Opus verdict.
The receipt is preserved verbatim; this status correction does not supply a
security judgment.
