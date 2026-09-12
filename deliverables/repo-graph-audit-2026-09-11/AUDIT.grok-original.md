# Moriarty repository graph and evidence audit

Date: 2026-09-11. Auditor: Grok 4.6 (requested high effort; this session). Gemini was not used. This is a read-only graph and evidence audit. It does not resume any AFK queue, campaign, or Preview execution.

Checkout: `/home/charl/Moriarty`. HEAD `89b0c7b9809b501c091223c78f6dd2df8581936c` on `main` (`89b0c7b Fix Moriarty plugin history parsing and focus execution on acceptance`). Uncommitted work was left in place. Product code, hooks, canonical wiki, and raw receipts were not edited by this auditor. Graph outputs were written under `deliverables/repo-graph-audit-2026-09-11/`. The previous `graphify-out/` tree was copied to `deliverables/repo-graph-audit-2026-09-11/graphify-out-prior/` before this build; the live `graphify-out/graph.json` timestamp remains 2026-09-10T22:23.

## Plugin status (guarded CLI)

Command: `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json`

| Field | Value |
| --- | --- |
| capability | SP01.6 loan-swap-subset |
| lastResult | Stages complete: atomic-prepare, atomic-accept, rp01-mc02. Remaining include rp01-mc03, rp01-full, f0, f0a, f1-fixtures, f1, i2, f2, f3, mandatory, composition, finance, release, successor-frontend, successor-semantics, actus-semantics, defi-semantics, native-path-freeze |
| blockedAction | implementation/repair of loan-swap-subset |
| nextAction | sp01-loan-report |
| pendingTransactions | none |
| missingEvidence | binding-input-stale:`openspec/sprints/sp01-financial-contract-and-execution-admission.md`; candidate-input-stale: same; current-accounting-missing:`.moriarty-dev/runtime/current-accounting.json`; resource-live-state-unavailable:`sp01-loan-swap-grok-01`; operational-history |

`doctor`: host coverage **unverified**. `next --json`: `sp01-loan-report` is ALLOWED as a report action; the missing accounting/history still blocks dependent campaign dispatch. `.moriarty-dev/runtime/current-accounting.json` is absent on disk.

`report --json`: all twelve sprints are `status: open`, `demonstrated: false`.

This audit did not call `run`, notify, or submit any transaction.

## Large corpus warning (skill override)

Graphify detect on the entire checkout, with `.gitignore` disabled and extra excludes for secrets/caches/`.git`/this audit’s backup:

- **42,275 supported files**, **~46,277,977 words**
- Detector warning retained in `CORPUS_WARNING.md` and `GRAPH_REPORT.md`
- User overrode the skill’s “narrow to a subdirectory” prompt. Entire-repository scope was kept for inventory and detect.

Top detected first-level directories: `repos/` 24,964; `raw/` 9,954; `deliverables/` 4,309; `evidence/` 2,060; `experiments/` 287.

Detected categories: code 21,161; document 19,431; paper 129; image 1,554; video 0.

Skipped sensitive: 231 (includes real keys under `repos/marlowe-cardano/.../*.key` and false positives such as Midnight docs named `tokens.md`). Ignored extra-excludes: 85 (`.codex/`, `.foreman/*.db*`, `.vault-meta/`, this audit’s `graphify-out-prior/`, `.env` files). Walk errors: 0.

## Whole-repo inventory (not the same as extraction)

Filesystem walk of `/home/charl/Moriarty` (no symlink follow; prune `.git`, `node_modules`, `__pycache__`, venvs, `graphify-out`): **874,073 files / 25.35 GiB**.

| Top-level | Files | Notes |
| --- | ---: | --- |
| `.worktrees/` | 800,027 | Linked checkouts. Graphify `_SKIP_DIRS` prunes these. Not extracted. |
| `repos/` | 55,210 | Pinned third-party clones. Gitignored. Detected as supported files; **not AST-parsed this run** after full-tree AST hung on 45 MB JSON dumps. History navigation is via inventory + prior `graphify-out-prior`. |
| `raw/` | 10,085 | Immutable receipts. Indexed; subset semantically extracted. |
| `deliverables/` | 5,133 | Dated packets. |
| `evidence/` | 2,329 | Retained results. |
| `experiments/` | 329 | Product source. AST + some docs. |
| remaining product trees | hundreds | `openspec/` 83, `wiki/` 44, `docs/` 40, `plugins/` 28, `site/` 51, `tests/` 2. |

Without `.worktrees/`, the live checkout is about **74,046 files**. Graphify classified **16,469** of the detected-adjacent files as **unclassified** (inventory only). Important unsupported extensions:

- **860 `.compact`** files (Compact kernels). Product-owned examples include `experiments/moriarty-midnight-financial/custody/loan.compact`, `swap.compact`, `experiments/moriarty-language/compact/*.compact`. Graphify AST does not parse Compact. 25 product `.compact`/`.k`/`.zkir` files were added as inventory-only file nodes.
- **1,669 `.k`** files (K framework). Product `experiments/moriarty-language/formal/k/moriarty.k` is unclassified by the detector.
- Also unclassified: `.hs`, `.purs`, `.zkir`, `.kore`, `.tap`, many extensionless files.

**Do not read “42,275 detected” as “42,275 semantically understood.”** Layers below.

## Extraction layers (what was actually read)

| Layer | What it is | Count | Model |
| --- | --- | ---: | --- |
| Filesystem inventory | path + size; no content | 874,073 (incl. worktrees) | none |
| Graphify detect | supported-type index + word counts | 42,275 | none |
| Deterministic AST | tree-sitter extract of **product-owned** source ≤1 MB | 749 files → 10,250 nodes, 30,611 edges | none (`graphify.extract`) |
| Grok 4.6 semantic | 32 general-purpose subagents, 22 files/chunk, DEEP_MODE=false | 703 priority docs → 3,136 new semantic nodes | **grok-4.6** |
| Semantic cache replay | prior graphify cache (mixed historical backends, including a 2026-09-09 Gemini run recorded in the old `cost.json`) | 346 files → 3,548 cached nodes | cached; not this model |
| Indexed only | detected docs/papers/images not in the 703 + 346 | ~20,065 | **not semantically read** |
| Oversize/generated code skipped | chain-spec JSON, compiled.json, 45 MB graph dumps, lace_wallet bundles | 112+ files; plus 20,412 detected code files outside product AST scope | not AST-parsed this run |

Semantic extraction: ~703 files → 32 agents. Host did not return per-agent `usage`, so this run’s `input_tokens`/`output_tokens` are **0** in `graphify-out/cost.json`. That is missing measurement, not a free extraction. Gemini was not called.

Raw extract JSON (pre-build): 15,106 nodes, 40,120 edges, 202 hyperedges. Confidence on raw edges: **EXTRACTED 38,822 · INFERRED 1,274 · AMBIGUOUS 24**. Built undirected graph: **15,085 nodes · 29,827 edges · 1,205 communities**.

Health check (must be visible): **3,378 dangling-endpoint edges**; 6,163 directed collapsed; 6,302 undirected collapsed. Typical cause: semantic node IDs that do not match AST IDs, plus cached edges from older graphs. The navigable `graph.json` drops missing endpoints. Self-loops: 0. HTML is **aggregated** (1,205 community nodes) because 15,085 > 5,000.

## Implementation versus roadmap / OpenSpec

Normative sources: `ROADMAP.md`, `openspec/MORIARTY-COMPLETION-PROGRAM.md`, `openspec/moriarty-completion-program.json`, `openspec/PCD-ROADMAP-2026-09-11.md`.

Program JSON `status`: `planning-reviewed`. Packages:

| ID | JSON status | Historical progress field | Gate |
| --- | --- | --- | --- |
| MC01 | pending-audit | integrated-experimental-awaiting-result-review | Atomic subset accepted (`atomicAcceptance.status=complete`, candidate `2f1fb864…`). Full MC01 open. |
| MC02 | specified-only | ledger-interface-source-preparation-complete | Preview loan/swap evidence exists; package acceptance open. Stage `i2` is `pending-review`. |
| MC03 | specified-only | legacy-source-reviewed-no-current-execution-admission | R3 exhausted k17. No current recursive proof. |
| MC04 | interface-blocked | specified-only | Native-to-Preview verifier open. |
| MC05–MC08 | specified-only | specified-only | Mandatory PCD, composition, full conformance, release. |

Stage admission (`openspec/moriarty-completion-program.json` → `reportReconciliation.stageAdmission`): complete = `atomic-prepare`, `atomic-accept`, `rp01-mc02`. `i2` pending-review. All later stages specified-only.

`openspec/PCD-ROADMAP-2026-09-11.md`: **specified-only; not adopted**. It does not change MC/SP gates. Wiki index updated 2026-09-11 records the PCD decision pages; that is research filing, not acceptance.

CLI report: **no sprint is demonstrated**.

## Real demonstrated financial execution versus mandatory PCD / full conformance

### What is demonstrated (scoped, retained)

Hello-world Preview settlement: `evidence/midnight-preview-2026-09-07/README.md`. Call `f79580fe0075dc26ae3f97f10557706f340cdf7a3b3118cd65a72b4fd870110a` at block 755889. This is public deploy/call/readback, not a financial product.

Reviewed Preview loan (`deliverables/sp05-financial-integration-2026-09-09/preview-loan-01/RESULT.md`, 2026-09-10): four stages at blocks 807289, 807293, 807297, 807301. Lender received 533,972,602 test-asset units; residual notional 4,500,000,000. Independent GPT-6 Astra and Grok 4.6 audits. Raw integration `FAILED` / driver `INCOMPLETE` / launcher exit 1 remain.

Reviewed Preview swap (`…/preview-swap-01/RESULT.md`): blocks 807510, 807515, 807526, 807530. 10,000 A for 19,743 B; close withdraws reserves to zero. Same auditor pair. Same raw FAILED/INCOMPLETE/exit 1.

Corrected Preview loan (`…/preview-loan-exit-01/RESULT.md`): blocks 808053–808067, contract `ffedd46ff0fd451e6a93eddbd241afbad1ca292ecada37bd0646992fefa3c30d`. Both auditors **PASS_SCOPED** for the financial slice. **Raw process exit unavailable**; strict exit-zero **UNMET**. Both public loan attempts consumed.

Corrected Preview swap (`…/preview-swap-exit-01/RESULT.md`): blocks 808320–808341, contract `a1bc37f889ca6a2cb17f6425a43e236a46446f8bdfd5b0dae9f5f73734682c96`. Both auditors PASS_SCOPED including command exit zero and separate outer containment. Full SP05 and mandatory PCD remain open. Both public swap attempts consumed.

Local loan/swap traces and rejection readback exist under the same deliverable tree with REVIEWED-RESULT.md files. Local success does not close Preview MC02.

### What is not demonstrated

- Mandatory proof-carrying acceptance / SP09 / MC05.
- Verification-enabled Preview acceptance.
- Real recursive history (MC03/SP06); native retry still blocked by k17 and missing F0.
- Full ACTUS 277 fixtures / 18 types (SP07).
- Full DeFi 72-row + intent libraries (SP08).
- Private handoff and five composition operators (SP10).
- Full financial/formal conformance (SP11) and developer release (SP12).
- Current accounting file and live resource state for the loan-swap campaign.
- I2/MC02 complete-effect certification: `i2` is pending-review, not complete.

Trusted node/indexer observations are not authenticated state proofs.

## Evidence freshness

| Item | Freshness | Locator |
| --- | --- | --- |
| Plugin status / report | this session | CLI JSON above |
| Preview loan/swap RESULT.md | 2026-09-10 | `deliverables/sp05-financial-integration-2026-09-09/preview-*-01/RESULT.md` |
| Preview loan/swap exit RESULT.md | 2026-09-10 | `preview-loan-exit-01`, `preview-swap-exit-01` |
| Hello-world Preview | 2026-09-07 | `evidence/midnight-preview-2026-09-07/README.md` |
| PCD architecture + roadmap | 2026-09-11 specified-only | `openspec/PCD-ROADMAP-2026-09-11.md`, `deliverables/pcd-midnight-native-2026-09-11/REPORT.md` |
| Wiki index | updated 2026-09-11T17:24:22Z | `wiki/index.md` (not edited by this auditor) |
| Prior graph | 2026-09-10, 103,755 nodes | `graphify-out-prior/` and live `graphify-out/` (preserved) |
| Completion program JSON | planning-reviewed; auditModels still list `claude-fable-5-1` and `gpt-6-astra` | `openspec/moriarty-completion-program.json` |
| Current accounting | **missing** | `.moriarty-dev/runtime/current-accounting.json` |
| SP01 sprint markdown | CLI flags **stale** as binding/candidate input | `openspec/sprints/sp01-financial-contract-and-execution-admission.md` |

Historical receipts describe their recorded trees. They are not current acceptance of successor language or mandatory PCD.

## Test and build observations (local, no network)

- `python3 -m pytest plugins/moriarty-dev/tests -q --tb=no`: **174 passed, 64 subtests passed in 12.30s**.
- `experiments/moriarty-language`: 26 test files listed. `node_modules` absent. `npm test` was **not** run (install would mutate the tree). ROADMAP claims a retained 441-test language regression and 682 tests for the simulation CLI; those claims were not re-executed here.
- `experiments/moriarty-midnight-financial`: `node_modules` absent. Preview/local financial tests not run. No new campaign, no Docker, no Preview RPC.
- Compact/K sources are present but not typechecked in this session.

## Historical result scope (do not recycle as current acceptance)

Superseded A4/A5, Candidate A, S01/S02, and old K loops remain archive (`docs/ARCHIVE.md`). R3 native rows exhausted at k17. MockProver / host flags do not establish PCD. The 2026-09-10 graph (103k nodes) imported third-party `repos/` snapshots and is a different artifact from this 15k-node product-focused graph.

## Limitations (honest)

1. Entire-repo **inventory and detect**, not entire-repo **semantic read**.
2. Product AST only (749 files). `repos/` and giant JSON/JS dumps are inventory.
3. ~20,065 documents/images/papers indexed only.
4. `.compact` / `.k` / `.zkir` have no AST parser in graphify.
5. 3,378 dangling semantic endpoints; graph.json is the post-build subset.
6. Token usage for the 32 Grok subagents was not returned by the host (recorded as 0).
7. Cached semantic nodes may include older Gemini-era entries from `graphify-out-prior/cost.json`; new extraction is grok-4.6 only.
8. HTML is community-aggregated (15,085 > 5,000).
9. Plugin host hooks remain unverified; root owns hook repair independently.
10. This audit is not dispatch authority and not AFK resumption.

## Artifact paths

| Artifact | Path |
| --- | --- |
| Graph JSON | `deliverables/repo-graph-audit-2026-09-11/graph.json` (hardlinked with `graphify-out/graph.json`) |
| Graph HTML | `deliverables/repo-graph-audit-2026-09-11/graph.html` |
| Graph report | `deliverables/repo-graph-audit-2026-09-11/GRAPH_REPORT.md` |
| This audit | `deliverables/repo-graph-audit-2026-09-11/AUDIT.md` |
| Corpus warning | `deliverables/repo-graph-audit-2026-09-11/CORPUS_WARNING.md` |
| Inventory | `deliverables/repo-graph-audit-2026-09-11/inventory.json` |
| Prior graph backup | `deliverables/repo-graph-audit-2026-09-11/graphify-out-prior/` |
| Scripts | `deliverables/repo-graph-audit-2026-09-11/scripts/` |
| Cost | `deliverables/repo-graph-audit-2026-09-11/graphify-out/cost.json` |

## Counts

| Metric | Value |
| --- | ---: |
| Inventory files (with worktrees) | 874,073 |
| Detected supported files | 42,275 |
| Words (detect) | 46,277,977 |
| AST product files / nodes / edges | 749 / 10,250 / 30,611 |
| Grok semantic files / new nodes | 703 / 3,136 |
| Cache semantic files / nodes | 346 / 3,548 |
| Built graph nodes / edges / communities | 15,085 / 29,827 / 1,205 |
| Raw EXTRACTED / INFERRED / AMBIGUOUS edges | 38,822 / 1,274 / 24 |
| Semantic chunks | 32/32 valid |
| Plugin tests this session | 174 passed |
| Pending Preview txs | 0 |
