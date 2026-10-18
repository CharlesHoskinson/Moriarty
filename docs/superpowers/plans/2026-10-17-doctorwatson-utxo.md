# `doctorwatson<UTXO>` implementation and audit plan

> **For agentic workers:** Use superpowers:subagent-driven-development. One integrator owns shared data and integration; authors own disjoint source files.

**Goal:** Build the local UTXO requirement reader, update retained EARS assessments, and give every remaining roadmap exit an evidence-bound disposition and next check.

**Architecture:** A small Python standard-library reader consumes a pinned requirement baseline and explicit public evidence records. A shell entrypoint runs it and ordinary regressions. It reports declarations and byte currentness; it cannot authenticate evidence or establish product semantics.

**Tech Stack:** Python 3 standard library, unittest, Bash, JSON, Markdown, existing Git worktree on `utxo`.

## Global constraints

- Display `doctorwatson<UTXO>`; machine slug `doctorwatson-utxo`.
- No Foreman, scheduler, hosted CI, provider polling, SDK, compiler, proof, wallet, node or ledger invocation from the checker.
- Native authority remains zero; the broader Preprod goal stays paused.
- Preserve original clauses, complete named scenarios, binding design references, histories and all twenty families.
- Keep legacy /5-/6, current selected Rust Core/1, staged Core/2, Preview, Preprod and Midnight scopes distinct.
- Fixed C2/S2′ evidence cannot close generated-Aiken/spending clauses EUC-012/013. New route clauses remain additive proposals until applicable owner review.
- GPT-6.1 Sol high authors ordinary source; fresh Gemini 4 Argon High reviews ordinary results. Security judgments retain their designated ownership.
- Stop factory expansion after this bootstrap. The resulting queue must point to actual artifact/claim/settlement consumers.

## Shared interface

Reader: `models/utxo/qa/check.py`. CLI: `--repo PATH`, `--baseline PATH`, `--evidence PATH`, `--json`, mutually exclusive `--next`, `--requirement ID_OR_KEY`, `--gate ID_OR_KEY`. Defaults are repository root, `docs/utxo/doctorwatson/EARS-BASELINE.json`, `openspec/changes/utxo-doctorwatson/current-evidence.json`.

Baseline has `groups[].{group,package?,requirements[]}` and `families[]`. A requirement's stable `key` is `source.path + '#' + heading`; preserve original nullable ID. Evidence format `doctorwatson-utxo.evidence/1`, rows keyed by the same string. Each row contains `key`, `id`, `group`, `source` (baseline path/hash), `heading`, `scenarioNames`, `applicability` (`status`, `profile`, `target`, `reason`), `implementation` (`status`,`detail`), `validation` (`status`,`evidenceRefs`), `acceptance` (`status`,`scope`,`reason`,`reviewRefs`), `scenarioRecords` (`name`,`status`,`expectation`,`evidenceRefs`), `sourcePins` (`path`,`sha256`), `dependsOn` (row keys), and `nextAction` (`consumer`,`expected`,`owner`,`blockedBy`,`readiness`). Statuses initially unassessed/unrun/open. Evidence refs are objects `{path,sha256}`; optional descriptive fields are inert. Family rows retain `id`, `name`, and independent `source`, `admission`, `compiled`, `native`, `private` status strings. Never infer one layer from another.

Reader compares exact baseline membership, source headings and ordered scenario names against actual Markdown and pinned whole-file hash; checks duplicates, missing rows, malformed fields, explicit public relative paths, source/evidence/review pins, declared dependencies and transitive staleness. No directory traversal or command execution. Report separates integrity, implementation, observations, declared acceptance and computed currentness. Valid all-open inventory exits 0; stale/missing/inconsistent evidence exits 1; malformed input exits 2. An open/unknown/stale gate exits 1. A closed declaration requires all scenarios observed, current source/dependencies, direct applicable profile/target, complete implementation, observed validation, nonempty actual evidence and independent review declarations. Matching files do not authenticate those declarations. Missing typed substantive contracts remain open; no automatic closure from tests.

## Task 1: Independent reporting expectations

**Owner:** Sol oracle/test author. **Files:** `models/utxo/qa/test_check.py` only.

- [ ] Write synthetic temporary-repository fixtures with independently authored normative Markdown and baseline/evidence declarations.
- [ ] Confirm absent entrypoint is an expected assertion failure; retain the initial RED observation.
- [ ] Cover valid all-open integrity versus failing selected closure, duplicate/missing IDs and scenario names at equal totals, edited normative source, stale dependency propagation, unknown/cyclic dependencies, malformed input, family omission/duplication, profile/target inheritance, missing review, exact gate selection, and inert command strings.
- [ ] Fixtures test the report only and never constitute product or ledger evidence.

## Task 2: Reader and entrypoint

**Owner:** Sol implementer. **Files:** `models/utxo/qa/check.py`, `scripts/doctorwatson-utxo.sh`, `models/utxo/qa/README.md` only.

- [ ] Wait for the oracle author's initial failing tests, inspect the failure, then implement the smallest reader.
- [ ] Make the default shell command run integrity followed by its focused regressions; selectors invoke only the reader. Keep Sherlock files unchanged.
- [ ] Run affected ordinary checks, fix behavior against the frozen expectations; report full command/exit and limitations.

## Task 3: Canonical assessment data

**Owner:** root integrator. **Files:** `openspec/changes/utxo-doctorwatson/`, shared `docs/utxo/doctorwatson/` metadata, AGENTS naming rule.

- [ ] Synthesize three Astra annexes, retaining their advisory limits.
- [ ] Generate the full 119/238, 11/16 and 18/38 initial assessment rows from original normative sources, not shortened coverage text. Keep the twenty independent family statuses.
- [ ] After the reader works, reconcile every row with source evidence, exact applicability, missing scenario evidence and next actual consumer. Preserve open acceptance where unestablished.

## Task 4: All-EARS and full-roadmap audit after build

**Owner:** disjoint source-audit workers; root integrates. **Files:** `docs/utxo/doctorwatson/audit/*.json`, `docs/utxo/doctorwatson/ROADMAP-AUDIT.md`; link existing `ROADMAP.md`, coverage and OpenSpec entrypoints.

- [ ] Reconcile every file/line in the retained 85-file/388-line baseline and every 160-heading crosswalk entry against current sources. Preserve historical/current deltas and addenda; do not claim these are atomic requirement counts.
- [ ] Audit all UTXO, first-contract and retained Midnight clauses/scenarios; distinguish source-inspected, partial, absent, unrun, stale, owner-unassessed and accepted.
- [ ] Map U0–U7, M1–M17, MC01–08, SP01–12 and G01–24 plus ACTUS, DeFi, formal methods, recursion, private handoff and all composition families. Name missing predicates, actual consumers, dependencies, owner and completion evidence.
- [ ] Keep uninspected or unresolved sources explicit; audit coverage is distinct from semantic acceptance.

## Task 5: Integration verification and fresh review

- [ ] Run local UTXO report/regressions, selectors and representative refusal gates. Capture actual results.
- [ ] Run the unchanged Sherlock ordinary check once for integration compatibility; do not invoke optional host/design/native lanes.
- [ ] Give fresh Gemini reviewer the exact full source, canonical data, applicable requirements, independent expectations and actual results. Resolve material findings and rerun only affected checks.
- [ ] Publish reviewed scoped changes to `utxo`; report any pending reviews honestly, without claiming product acceptance or rearming the paused prototype loop.
