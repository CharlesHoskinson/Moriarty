Independently review this Obsidian migration, scoped to preservation, useful developer workflow and truthful validation. User explicitly requested: "After we clean up the repo, migrate the current wiki-llm workflow to an obsidian vault in WSL associated with the repo using this skill AgriciDaniel/claude-obsidian". GitHub cleanup is complete, main published as 8df38d25f6863e90bb7e16b691b578231c62458d with recovery tag; only main remains remotely. This migration is the next separate commit. Installed skill commit ad67087cad22ad84cc3288f915588ae42c0c2b44 matches fetched upstream and all 15 Codex links pass readiness. Use only the supplied packet; no tools or filesystem inspection are needed. Return JSON verdict with blockingFindings, nonblockingFindings, acceptedScope and limits.

Design: adopt repo root /home/charl/Moriarty as WSL vault so existing wiki/raw/evidence links keep one canonical copy. Product checkout remains outside vault. Original source bytes, page IDs/types/status, claim identifiers, confidence and roadmap stay preserved. Portable source identities use the tool's stable hash IDs and preserve every CSV row verbatim as legacy_records plus legacy_source_ids mapping. All portable source records remain unreviewed/unknown until assessed under the new contract; original scoped classifications stay in legacy metadata. The claim ledger remains empty; an exact-occurrence legacy claim index gives navigation without automatic claim extraction or acceptance. No network research, proof generation, financial settlement or semantic compiler change occurred. Explicit Git publication is authorized. No automatic hooks, cloud sync or REST server installed.

Applied operations: core adopt (11 missing foundation files); migration settings (.gitignore, .obsidian/app.json); generic wiki operation (existing note metadata + overview, hot context, workflow, provenance/source ledger and legacy claim index with index/log updates); Canvas (10 file nodes and 10 labeled edges plus optional catalog); scoped workflow polish. Every operation was inspected with expected hashes, applied using its exact approval hash, and has changed-path/hash receipts. No legacy note or raw source was deleted. All 33 legacy notes retain claim IDs. Lint no longer has broken heading links or missing metadata. ONE known naming advisory remains because the optional Canvas catalog is wiki/canvases/index.md and the research index is wiki/index.md. All links are qualified; ambiguous_targets=0. Upstream lint --strict exits nonzero for this advisory; docs state it openly and no finding is suppressed. The core supports create/replace, not removal of canonical notes; the optional catalog is retained at the documented path. Assess this as a named navigation convention, not an invented zero-findings claim.

Verification: deterministic source comparison confirms 78 original CSV rows retained exactly across 76 portable records, all 620 observed CLM identifiers retained, no original raw file changed, roadmap byte-unchanged, source-ledger schema/hash validation passes, Canvas IDs/targets/edges valid. Core doctor ready. Humanizer applied to maintained prose. Original historical review packets/receipts remain unchanged. End-user opens the repo with Obsidian's "Open folder as vault"; desktop interaction itself was not automated or visually verified.

### docs/OBSIDIAN.md
# Moriarty Obsidian vault

Open `/home/charl/Moriarty` with **Open folder as vault** in Obsidian. Start at
[the vault overview](../wiki/overview.md), [research index](../wiki/index.md) or
[language map](../wiki/canvases/moriarty.canvas). The repository itself is the
vault, so Git and Obsidian edit the same notes and citations.

The Windows path is `\\wsl.localhost\Ubuntu-26.04\home\charl\Moriarty`.
Agent writes run in WSL. Linux Obsidian is installed on this machine; using the
Windows editor does not move the vault out of WSL.

## Workflow and tool

[Research workflow](../wiki/workflow.md) describes ingest, query, save and lint.
[WIKI_SCHEMA.md](../WIKI_SCHEMA.md) retains Moriarty's evidence contract.
[Provenance mapping](../wiki/meta/provenance.md) explains how existing source and
claim identifiers coexist with the portable ledgers.

The requested [claude-obsidian project](https://github.com/AgriciDaniel/claude-obsidian)
is pinned at `ad67087cad22ad84cc3288f915588ae42c0c2b44`. Its installed location is
`/home/charl/.local/share/claude-obsidian`, separate from the vault. Existing Codex
skills link to that checkout; no second installation is needed. The
[tool receipt](../raw/sources/claude-obsidian-2026-09-07/receipt.json) records the pin
and captured instructions. The [upstream WSL guide](https://github.com/AgriciDaniel/claude-obsidian/blob/ad67087cad22ad84cc3288f915588ae42c0c2b44/docs/windows-wsl.md)
explains its filesystem requirements.

From the repository:

```sh
python3 /home/charl/.local/share/claude-obsidian/scripts/claude-obsidian.py doctor
python3 /home/charl/.local/share/claude-obsidian/scripts/claude-obsidian.py lint
```

Use the installed `wiki`, `wiki-ingest`, `wiki-query`, `save`, `wiki-lint` and
`canvas` skills in Codex. Claude Code can load the same local product with
`claude --plugin-dir /home/charl/.local/share/claude-obsidian` from this repository.
The workspace config resolves the vault relative to its own location, so a
fresh clone can also act as a vault after installing the pinned tool.

For another WSL machine, clone the tool outside Moriarty and install its portable
Codex links:

```sh
git clone https://github.com/AgriciDaniel/claude-obsidian.git ../claude-obsidian
git -C ../claude-obsidian checkout ad67087cad22ad84cc3288f915588ae42c0c2b44
bash ../claude-obsidian/bin/setup-multi-agent.sh --host codex
bash ../claude-obsidian/bin/setup-multi-agent.sh --host codex --apply
```

Inspect the first command's installation preview before applying. Substitute
that checkout's absolute path in the core commands above. Do not use the tool
checkout as a vault or overwrite an existing unrelated skill installation.

## Source control

The notes, shared vault settings, Canvas and evidence are tracked. Personal
workspace state, local transports, staged inbox inputs and transaction recovery
journals are ignored. Review source classification before publishing a new
capture. Commits and pushes are explicit; no automatic sync hooks are installed.

The graph starts with `path:wiki`. Source captures, repository clones, generated
graphs and local runtime directories are excluded from routine Obsidian search.
They remain on disk and retain their source locators. Historical execution work
is accessible through [the recovery archive](ARCHIVE.md).

The upstream linter reports a duplicate basename for `wiki/index.md` and the
optional `wiki/canvases/index.md` catalog. Links use full paths and resolve
without ambiguity. `lint --strict` treats that naming advisory as a failure;
[the validation receipt](../evidence/obsidian-migration-2026-09-07/verification.json)
records the disposition. No lint findings are suppressed.


### wiki/workflow.md
---
id: moriarty.vault.workflow
title: Research workflow
type: overview
status: active
created: 2026-09-07
updated: 2026-09-07
tags:
  - moriarty
  - research
sources:
  - SRC-0077
  - SRC-0078
---

# Research workflow

This vault lives at the Moriarty repository root in WSL. The existing `wiki/` notes, `raw/` captures, `evidence/` receipts and root roadmap remain their canonical copies. Open `/home/charl/Moriarty` as a folder in Obsidian. On Windows, the same directory is `\\wsl.localhost\Ubuntu-26.04\home\charl\Moriarty`.

## Read and query

Begin with [[wiki/index|the index]], [[wiki/hot|current context]] and the smallest relevant notes. Use the installed `wiki-query` skill for cited, read-only answers. Saving a useful answer is a separate `save` operation within the user's requested scope. Do not capture transcripts or mutate notes merely because a session ended.

## Ingest and maintain

1. Read [AGENTS.md](../AGENTS.md), [WIKI_SCHEMA.md](../WIKI_SCHEMA.md) and the relevant user assignment.
2. Stage new inputs in `inbox/`. Use `wiki-ingest` and its reviewed capture workflow to preserve new source bytes under `.raw/captured/`. Existing `raw/` captures remain immutable and are already inside this vault.
3. Reuse topic pages, preserve source/claim IDs and distinguish source facts, experiments, inference and open questions. Keep conflicting findings visible in [[wiki/contradictions|contradictions]].
4. Draft one scoped transaction with expected file hashes, note changes, source/claim records and index/log updates. Inspect the exact changed paths before applying. Existing authorization covers routine work within its stated scope; elapsed time never supplies missing authority.
5. Apply through the pinned claude-obsidian core. On conflict, re-read and rebuild; on interruption, use transaction recovery. Run `wiki-lint` read-only afterward. Apply Humanizer to maintained prose while preserving citations, data and code.
6. Commit and publish separately when authorized. There are no automatic commits or pushes, and staged inbox files and runtime recovery data are ignored by Git.

The source and claim vocabulary is explained in [[wiki/meta/provenance|the provenance mapping]]. Existing S0-S7 lifecycle labels stay authoritative for legacy claims. A structurally valid ledger does not prove correctness or confer reviewer acceptance.

## Local commands

The [setup guide](../docs/OBSIDIAN.md) records the pinned skill and portable commands. From this repository:

```sh
python3 /home/charl/.local/share/claude-obsidian/scripts/claude-obsidian.py doctor
python3 /home/charl/.local/share/claude-obsidian/scripts/claude-obsidian.py lint
```

No REST server, MCP transport or community plugin is required. Skills resolve the vault through `.claude-obsidian.json`. The installed product stays outside the vault.


### wiki/meta/provenance.md
---
id: moriarty.vault.provenance
title: Provenance mapping
type: overview
status: active
created: 2026-09-07
updated: 2026-09-07
tags:
  - moriarty
  - research
sources:
  - SRC-0077
  - SRC-0078
---

# Provenance mapping

The original [source inventory](../../evidence/source-inventory.csv) retains every `SRC-####` record and its original metadata. Existing `CLM-####` claims retain their wording, citations, confidence and S0-S7 lifecycle labels in the notes.

The portable [source ledger](ledgers/source-ledger.json) assigns the skill's content-derived `src-...` identity and stores a lossless `legacy_records` list for each original record. `legacy_source_ids` maps every original ID to its portable record. File hashes describe bytes observed during migration; `legacy_hash_matches_observed` distinguishes those hashes from the original inventory assertion. Missing, archived or compound locators are preserved as manual, unreviewed identities rather than invented file matches. Review state remains `unreviewed` until the portable provenance contract is assessed; this does not downgrade or replace the original scoped evidence.

The [legacy claim index](legacy-claim-index.json) maps exact observed claim identifiers to their note paths. It is a navigation index, not a semantic extraction or new proof. The portable [claim ledger](ledgers/claim-ledger.json) starts empty: migration does not automatically extract or accept claims from prose. Future scoped ingestion can add explicit support and contradiction records after reviewing the evidence. No automated claim acceptance occurred.

`created` on adopted notes records the earliest retained Git addition date. `updated` records the metadata migration date; the original `updated_at` remains available. Existing types, stable page IDs and lifecycle vocabularies are preserved.

The [migration receipt](../../evidence/obsidian-migration-2026-09-07/README.md) records the skill pin, transactions and checks. Return to [[wiki/workflow|the workflow]] or [[wiki/index|the research index]].


### WIKI_SCHEMA.md
# Moriarty wiki operating schema

## Purpose and layers

This repository implements Andrej Karpathy's LLM Wiki pattern for Moriarty's
Midnight financial-language research and design.

- The repository root is an Obsidian vault selected by `.claude-obsidian.json`.
  `wiki/workflow.md` describes the portable transaction workflow.
- `inbox/` stages new inputs; `.raw/captured/` holds new immutable captures made
  by the portable core. Existing `raw/` captures keep their canonical paths.
- `raw/` is immutable source material, prompt inputs, receipts, and dated
  research notes. Corrections supersede; they do not overwrite.
- `wiki/` is maintained synthesis. Update an existing concept page instead of
  creating a near-duplicate.
- `WIKI_SCHEMA.md` and `AGENTS.md` define the operating contract.
- `evidence/` contains inventories, repository locks, deployment records, and
  experiment manifests.
- `experiments/` contains executable or fully specified benchmarks and tests.
- `deliverables/` contains requested outputs and reports. The controlling XML
  v1.3 requires 22 deliverables, D01–D22; the earlier 18-output report does not
  satisfy the additional ACTUS obligations by its presence.

## Operations

### Ingest

1. Read `wiki/index.md` and search the wiki for the source's concepts.
2. Add a stable source identifier (`SRC-####`) to the inventory.
3. Acquire the source into `raw/` and record provenance and SHA-256.
4. Classify authority and lifecycle scope before extracting claims.
5. Update the smallest applicable existing wiki pages.
6. Link claims to their source receipt and, where relevant, pinned code.
7. Record disagreements in `wiki/contradictions.md`.
8. Update `wiki/index.md` and append an entry to `wiki/log.md`.

### Query

Read `wiki/index.md`, then the smallest relevant linked pages and receipts.
Return cited synthesis without mutation. Save reusable knowledge only through a
separately scoped Save transaction, with the required index and log updates.

### Lint

Check required metadata, unique page IDs, broken relative links, source IDs,
orphan pages, duplicate concepts, stale moving facts, missing commit hashes,
unresolved contradictions, unsupported status levels, and mismatches between
the index and on-disk pages.

## Page metadata

Every maintained page includes the existing domain metadata plus Obsidian
properties. For adopted pages, `created` records the earliest retained Git
addition date; `updated` records a content or metadata change. Preserve the
original `updated_at` timestamp as a separate historical field. Example:

```yaml
---
id: stable.dotted.identifier
type: overview|source|component|semantics|formal|runtime|language|security|benchmark|comparison|adoption|governance|decision|question|contradiction
title: Plain title
status: active|blocked|superseded|resolved
updated_at: YYYY-MM-DDTHH:MM:SSZ
created: YYYY-MM-DD
updated: YYYY-MM-DD
tags:
  - moriarty
sources:
  - SRC-####
---
```

## Claim record

Every material claim must make the following recoverable from its paragraph,
table row, or adjacent claim block:

- stable claim ID (`CLM-####`);
- exact source ID and locator;
- publication, release, deployment, or commit date;
- repository, tag, branch, and full commit hash where relevant;
- authority: normative, descriptive, historical, experimental, or promotional;
- scope: V1, deployed validator, current service, unreleased implementation,
  V2.0 proposal, V2.1 proposal, or speculative future;
- evidence kind: source fact, repository observation, experiment observation,
  inference, recommendation, contradiction, or open question;
- reproduction: reproduced, partially reproduced, not reproduced, or not
  applicable;
- confidence: high, medium, low, or unknown;
- lifecycle status S0 through S7.

Use these lifecycle meanings exactly:

- `S0`: historical or superseded.
- `S1`: idea or discussion only.
- `S2`: written proposal or design.
- `S3`: prototype or experimental branch.
- `S4`: implemented and tested but unreleased.
- `S5`: released but not deployed or broadly supported.
- `S6`: deployed and supported.
- `S7`: deployed, independently audited, and operationally evidenced.

Status is scoped to the claim, not assigned wholesale to a repository.

## Source receipt fields

Each acquired web source requires requested URL, canonical URL, retrieval UTC,
HTTP status, publication or last-modified date if known, source class, local
path, SHA-256, acquisition method, relevant version, and coverage limitations.

Repository receipts additionally require remote, default branch, checked-out
branch or tag, full commit, submodule state, dirty state, and acquisition UTC.

## Contradictions and moving facts

Do not select the newest-looking source silently. Record each conflicting claim,
the scopes that may explain the conflict, reproduction evidence, and the
provisional disposition. Dynamic facts such as protocol parameters, deployment
status, open issue counts, or branch heads must carry an `observed_at` time and
must not be copied into multiple canonical locations.

## Formal and empirical language

State guarantees in theorem-like form with assumptions. Keep semantic
non-locking distinct from validator acceptance, ledger-resource feasibility,
transaction-construction feasibility, continuation availability, participant
cooperation and keys, wallet/Runtime correctness, and future protocol support.

Use `specified-only` for an experiment design that has not run. Use `reproduced`
only when commands, inputs, environment, raw outputs, and acceptance predicates
are preserved.

## Portable provenance

The source inventory keeps `SRC-####` identities. The portable source ledger
keeps a lossless legacy-ID mapping and separately observed content hashes.
The legacy claim index is navigation only; no claim is accepted by migration.
See [the mapping](wiki/meta/provenance.md) before adding or reviewing portable
ledger entries. Retain the original lifecycle and confidence vocabulary.


### CLAUDE.md
# Moriarty

Read [AGENTS.md](AGENTS.md), [WIKI_SCHEMA.md](WIKI_SCHEMA.md) and
[wiki/workflow.md](wiki/workflow.md). This repository is the Obsidian vault;
`.claude-obsidian.json` selects its root. The claude-obsidian product checkout
is external and must never become the vault.

Keep queries read-only. Apply canonical wiki writes through inspected portable
transactions. Preserve source bytes, claim identifiers and evidence scope.
Follow the current Midnight roadmap and exact independent reviewer rules.


### .claude-obsidian.json
{
  "legacy_raw": ".raw",
  "role": "vault",
  "schema": "claude-obsidian.workspace.v1",
  "source_inbox": "inbox",
  "vault": "."
}


### .obsidian/app.json
{
  "userIgnoreFilters": [
    ".raw/",
    ".vault-meta/",
    "raw/",
    "repos/",
    "graphs/",
    "graphify-out/",
    ".worktrees/",
    ".venv/",
    "node_modules/"
  ],
  "newLinkFormat": "absolute",
  "useMarkdownLinks": false,
  "alwaysUpdateLinks": true
}


### .obsidian/graph.json
{
  "search": "path:wiki",
  "showAttachments": false,
  "showArrow": true,
  "showOrphans": false
}


### .gitignore
# Python-generated files
__pycache__/
*.py[oc]
build/
dist/
wheels/
*.egg-info

# Virtual environments
.venv
.worktrees/

# Runtime checkpoint database files are local. Commit the typed NDJSON export.
.foreman/*.db
.foreman/*.db-*

# Acquired repository checkouts and generated analysis products
repos/
graphs/
graphify-out/
/_apalache-out/

# Acquisition and browser caches
.scrapling/
downloads/

# Local credentials or authenticated session material
*.local.md
*.cookies.json

# Obsidian personal state, local transport and transaction recovery
.vault-meta/
.mcp.json
.obsidian/workspace*.json
.obsidian/cache/
.obsidian/plugins/
.trash/
.DS_Store
Thumbs.db
inbox/*
!inbox/.gitkeep


### evidence/obsidian-migration-2026-09-07/verification.json
{
  "observed_at": "2026-09-07T17:54:24.268109+00:00",
  "baseline_commit": "8df38d25f6863e90bb7e16b691b578231c62458d",
  "legacy_notes_preserved": 33,
  "source_inventory_rows": 78,
  "portable_source_records": 76,
  "legacy_source_rows_preserved_exactly": true,
  "claim_identifiers_lost": [],
  "original_raw_files_modified": [],
  "roadmap_unchanged": true,
  "canvas_file_targets_and_edges_valid": true,
  "lint_category_counts": {
    "ambiguous_targets": 0,
    "configuration_errors": 0,
    "dead_links": 0,
    "duplicate_basenames": 1,
    "empty_sections": 0,
    "missing_frontmatter": 0,
    "orphans": 0,
    "provenance_errors": 0,
    "read_errors": 0,
    "stale_index_entries": 0
  },
  "lint_disposition": "The optional Canvas catalog uses the documented index.md name, also used by the root wiki index. All links are qualified; ambiguous_targets is zero. This is the sole naming advisory, not a navigation/provenance failure. Strict upstream lint exits nonzero for it.",
  "legacy_note_sha256": {
    "wiki/benchmarks.md": "76368bab63f0a71e1d283f8efd2aeaa1362da918effbc9963cfcb827919a357b",
    "wiki/contradictions.md": "d16fa4dd391f1f621188f56973ebc35e587d5518255a7281250d02232b5e1153",
    "wiki/decision.md": "1d3fee1f75f220620fe771a7204f49b8af6b77e3a0b578700935df4f2b409e69",
    "wiki/defiformal-taxonomy.md": "885119687b2753810e09e6a8de371b4e24d0ac24fe495c64c01f8b90bf1e0d08",
    "wiki/formal-assurance.md": "1e8b739038b0876bcf15125d9050fd55bf0b3dd57f4c6040f5e6e035e4bf339e",
    "wiki/index.md": "05abada312115d0aac398e65b690a5e7e748c903d4020b0d2b6a0bd52ada48e6",
    "wiki/k-framework/k-backends-and-tools.md": "d97ac8e7238a01408356c73cd6ce271eca9bc6d0790cc0959f4928f5891a427c",
    "wiki/k-framework/k-best-practices.md": "0b73fd4dc0c23869721988789439290aecb03eda8fffe9474d5cb8cd2b6e21b7",
    "wiki/k-framework/k-builtins.md": "b3bd0f89281cbff5afd92cc7f362c8eea1ca0fa23668dc2e3f250a1b4bd96a33",
    "wiki/k-framework/k-documentation-graph.md": "cafeaf954ab35cf8247a02daf65d9002f83830fad42c7d1c78203c4f57d08664",
    "wiki/k-framework/k-framework-overview.md": "e73b1cadcb66945f9dd5d9742444d873672d3cec1343c2cb8487c59a32472933",
    "wiki/k-framework/k-tutorial-basic.md": "9537e1a47d744dda597bb24d6b6bf38a859033864911f75e74339beb9cb59360",
    "wiki/k-framework/k-tutorial-intermediate.md": "e92c473b0cd38f7801d60b99cc670a020a294072840f45d505f87ddafe3344d6",
    "wiki/k-framework/k-user-manual.md": "b2beb534bbb4f83c00359ec690b72f3da1f01272ea21cf67ea32d4b97a63f79f",
    "wiki/log.md": "21597119519b4fc262aa4400c75260e9048b4d1ff4fb37132f8c8be11a64da78",
    "wiki/marlowe-baseline.md": "e818303111b8a9f1c5edafb6a913f7e715d8cfb1042dd1304ab74941f5b22aae",
    "wiki/marlowe-repository-graph.md": "3a41e9797fb0b6757a0f0fc244d8fddad957cad6286e1c801185bd83db43973b",
    "wiki/midnight-repositories.md": "69febb59a745aaed93801709ccdbc43b8e42af8d6226c13261cf3449c77f213f",
    "wiki/moriarty-architecture.md": "a7e87aa3fdcfb5bf2e0773e2d5a82ae4bcaafffeaa96a56f7a7254a07f9f2c14",
    "wiki/open-questions.md": "009768c12ec70cff44c0de8910514505952daad826a9d29ebc72389e1bde7731",
    "wiki/research-journal.md": "22322967d65959da6425125512626c9118f7a6eac271662b469560ef521fad53",
    "wiki/research-program.md": "8817f5875a355f7b67b926bb60026ccee9ad5f23708e6b9afb1edb051fc9e2a1",
    "wiki/security.md": "b5a1a73c1b9dc7f456d97c0c0b58b29f121b31131e8ea94281da5832622eddf7",
    "wiki/sources/llm-wiki-pattern.md": "3a531b7280f59facc959313e151f5db8c3fdabf658df05541f8729b17e34dcf8",
    "wiki/zkir-k-semantics-plan.md": "e7a8452002d276067ab48619099214ffae52d3041158dde773b088dff246d927",
    "wiki/zkir/compact-to-zkir-pipeline.md": "0257de08c5ea51efb72e750d18e5ed95736ef9339c5426019ea3145f0a2c5597",
    "wiki/zkir/midnight-k-tooling.md": "622a393059d068cc4d91f407ec49be97eb413ce768e208ecec8b46b29c461354",
    "wiki/zkir/zkir-formal-spec-agda.md": "2793a8727eba976953cad516b832fd6bb6048af9522548778df22ff5a30ca414",
    "wiki/zkir/zkir-instruction-set.md": "d04af68d75617781ace85b76c344a7da638c56db771ba6313f1c52dd33dd2575",
    "wiki/zkir/zkir-k-definition.md": "0fc592a736257fd3e6c9291b5eedd2924b999eacf96403a4790722d1c0ce2fee",
    "wiki/zkir/zkir-type-system.md": "fe3ec613d085b57d23612e727426bb00418b96bc99505d8107b4157db848cb07",
    "wiki/zkir/zkir-v3-divergence-review.md": "0703e5136cf38a3114031ba71d80629fbd52f46930887aa5649faaad743bcdef",
    "wiki/zkir/zkir-vm-semantics.md": "cbf5ef0dc98358b06fdfd03a10bf33367645fcca7f94087e8f7f4b2bb3277c7b"
  }
}


### evidence/obsidian-migration-2026-09-07/doctor.json
{
  "checks": {
    "meta_writable": true,
    "mutation_lock_held": false,
    "obsidian_config": true,
    "raw": true,
    "vault_exists": true,
    "wiki": true
  },
  "legacy_layout": false,
  "ok": true,
  "schema": "claude-obsidian.doctor.v1",
  "selection_source": "workspace-config",
  "vault_root": "/home/charl/Moriarty",
  "version": "2.1.1"
}


### evidence/obsidian-migration-2026-09-07/README.md
# Obsidian migration evidence

The Moriarty repository in WSL is the Obsidian vault. The existing wiki, raw source captures, evidence and roadmap keep their paths. The user requested AgriciDaniel/claude-obsidian after GitHub cleanup; SRC-0077 records that instruction and SRC-0078 pins the tool instructions.

The installed portable core and all 15 Codex skill links were already present at the pinned commit. The migration uses the core's inspected transactions for adoption, note metadata and navigation, portable provenance records, settings and Canvas. Exact changed paths and file hashes appear in the transaction receipts. Personal Obsidian state, inbox inputs and transaction recovery data are ignored by Git.

The legacy source inventory and all observed claim identifiers remain available. Portable source records preserve each complete legacy row and separately record observed file hashes. Legacy claims are indexed by location, not automatically extracted or accepted into the new claim ledger. Original confidence and S0-S7 scope stay in the notes.

The initial deterministic lint found four incompatible heading links and missing metadata on 33 notes. The migration repairs navigation and supplies Obsidian metadata. Doctor resolves the workspace and reports ready. Lint has no broken or ambiguous links, missing metadata, orphan pages or provenance errors. Its sole finding is the shared `index.md` basename for the wiki and optional Canvas catalog; qualified links resolve, and strict lint reports a nonzero exit for that advisory. The verification receipt checks exact preservation of all 78 legacy source rows, all observed claim identifiers, raw source bytes and the roadmap, plus Canvas targets and edges. Humanizer was applied to maintained prose. This operation does not establish a native recursive proof, ledger correspondence or any MC01-MC08 acceptance.


### wiki/canvases/moriarty.canvas
{
  "nodes": [
    {
      "id": "6254bb2ff3d51a62",
      "type": "file",
      "file": "docs/research/2026-09-06-actus-defi-design-study.md",
      "x": 0,
      "y": 0,
      "width": 420,
      "height": 280
    },
    {
      "id": "da29b74ea4d9f0b8",
      "type": "file",
      "file": "docs/research/2026-09-06-intents-report-integration.md",
      "x": 480,
      "y": 0,
      "width": 420,
      "height": 280
    },
    {
      "id": "fab79171a6f0d147",
      "type": "file",
      "file": "docs/research/2026-09-06-pcd-report-integration.md",
      "x": 960,
      "y": 0,
      "width": 420,
      "height": 280
    },
    {
      "id": "b7e02df1ef53b458",
      "type": "file",
      "file": "wiki/moriarty-architecture.md",
      "x": 480,
      "y": 360,
      "width": 420,
      "height": 280
    },
    {
      "id": "0c1d179944505f7a",
      "type": "file",
      "file": "experiments/moriarty-language/README.md",
      "x": 0,
      "y": 720,
      "width": 420,
      "height": 280
    },
    {
      "id": "8fa08baaa0a80d6c",
      "type": "file",
      "file": "experiments/moriarty-language/compact/MAPPING.md",
      "x": 480,
      "y": 720,
      "width": 420,
      "height": 280
    },
    {
      "id": "e8cddb5a8a6ee608",
      "type": "file",
      "file": "evidence/moriarty-native-ivc-r3-2026-09-07/README.md",
      "x": 960,
      "y": 720,
      "width": 420,
      "height": 280
    },
    {
      "id": "e15c728a6f71b436",
      "type": "file",
      "file": "evidence/midnight-preview-2026-09-07/README.md",
      "x": 0,
      "y": 1080,
      "width": 420,
      "height": 280
    },
    {
      "id": "be5b8b169f2ece02",
      "type": "file",
      "file": "ROADMAP.md",
      "x": 480,
      "y": 1080,
      "width": 420,
      "height": 280
    },
    {
      "id": "e41f2ce318e50582",
      "type": "file",
      "file": "deliverables/moriarty-report-plan-review-2026-09-07/README.md",
      "x": 960,
      "y": 1080,
      "width": 420,
      "height": 280
    }
  ],
  "edges": [
    {
      "id": "adfa01a70198ae5b",
      "fromNode": "6254bb2ff3d51a62",
      "toNode": "b7e02df1ef53b458",
      "fromSide": "bottom",
      "toSide": "top",
      "label": "financial behavior"
    },
    {
      "id": "b4baca894ba024f0",
      "fromNode": "da29b74ea4d9f0b8",
      "toNode": "b7e02df1ef53b458",
      "fromSide": "bottom",
      "toSide": "top",
      "label": "authority constraints"
    },
    {
      "id": "bee95ea87c7ca866",
      "fromNode": "fab79171a6f0d147",
      "toNode": "b7e02df1ef53b458",
      "fromSide": "bottom",
      "toSide": "top",
      "label": "mandatory claims"
    },
    {
      "id": "05aed700ad29d503",
      "fromNode": "b7e02df1ef53b458",
      "toNode": "0c1d179944505f7a",
      "fromSide": "bottom",
      "toSide": "top",
      "label": "evaluation"
    },
    {
      "id": "651bbd6dbf7b1548",
      "fromNode": "0c1d179944505f7a",
      "toNode": "8fa08baaa0a80d6c",
      "fromSide": "bottom",
      "toSide": "top",
      "label": "restricted compilation"
    },
    {
      "id": "ec1f4a6c39e3d95e",
      "fromNode": "fab79171a6f0d147",
      "toNode": "e8cddb5a8a6ee608",
      "fromSide": "bottom",
      "toSide": "top",
      "label": "recursive mechanism under study"
    },
    {
      "id": "8f0534ff6cb5beec",
      "fromNode": "8fa08baaa0a80d6c",
      "toNode": "be5b8b169f2ece02",
      "fromSide": "bottom",
      "toSide": "top",
      "label": "ledger correspondence remains open"
    },
    {
      "id": "fe3979fe2d4367a6",
      "fromNode": "e8cddb5a8a6ee608",
      "toNode": "be5b8b169f2ece02",
      "fromSide": "bottom",
      "toSide": "top",
      "label": "native proof remains open"
    },
    {
      "id": "baefcef8b1e7e311",
      "fromNode": "e15c728a6f71b436",
      "toNode": "be5b8b169f2ece02",
      "fromSide": "bottom",
      "toSide": "top",
      "label": "hello-world only"
    },
    {
      "id": "df0eddc029cd1204",
      "fromNode": "e41f2ce318e50582",
      "toNode": "be5b8b169f2ece02",
      "fromSide": "bottom",
      "toSide": "top",
      "label": "report requirements"
    }
  ]
}


### Representative source records
{
  "src-60a17363aa2e38790ce2": {
    "origin": {
      "kind": "file",
      "locator": "raw/assignments/moriarty-obsidian-vault-2026-09-07.md"
    },
    "title": "Obsidian vault migration instruction",
    "content_kind": "document",
    "authority": "unknown",
    "review_status": "unreviewed",
    "content_sha256": "2218438869412a29899b92bf554741d14186ae14077f0a726231830e6e684a1e",
    "ingested_at": null,
    "retrieved_at": "2026-09-07",
    "refresh_due": null,
    "pages": [],
    "independence_key": null,
    "legacy_records": [
      {
        "source_id": "SRC-0077",
        "title": "Obsidian vault migration instruction",
        "requested_url": "",
        "canonical_url": "",
        "published_or_commit_date": "2026-09-07",
        "retrieved_at_utc": "2026-09-07T17:51:26Z",
        "http_status": "",
        "authority": "user instruction",
        "scope": "research workflow maintenance",
        "status": "S2",
        "reproduction": "repository inspected",
        "confidence": "high",
        "local_path": "raw/assignments/moriarty-obsidian-vault-2026-09-07.md",
        "sha256": "2218438869412a29899b92bf554741d14186ae14077f0a726231830e6e684a1e",
        "notes": "Migration of workflow; no product or proof acceptance."
      }
    ],
    "legacy_hash_matches_observed": {
      "SRC-0077": true
    }
  },
  "src-36fed3b744e863656e8c": {
    "origin": {
      "kind": "file",
      "locator": "raw/sources/claude-obsidian-2026-09-07/receipt.json"
    },
    "title": "claude-obsidian portable workflow and WSL instructions",
    "content_kind": "document",
    "authority": "unknown",
    "review_status": "unreviewed",
    "content_sha256": "8830418e92fb445f85e815956d720c9ca901289a411fcbcc39710e41df33807f",
    "ingested_at": null,
    "retrieved_at": "2026-09-07",
    "refresh_due": null,
    "pages": [],
    "independence_key": null,
    "legacy_records": [
      {
        "source_id": "SRC-0078",
        "title": "claude-obsidian portable workflow and WSL instructions",
        "requested_url": "",
        "canonical_url": "",
        "published_or_commit_date": "2026-09-07",
        "retrieved_at_utc": "2026-09-07T17:51:26Z",
        "http_status": "",
        "authority": "primary repository documentation",
        "scope": "research workflow maintenance",
        "status": "S2",
        "reproduction": "repository inspected",
        "confidence": "high",
        "local_path": "raw/sources/claude-obsidian-2026-09-07/receipt.json",
        "sha256": "8830418e92fb445f85e815956d720c9ca901289a411fcbcc39710e41df33807f",
        "notes": "Migration of workflow; no product or proof acceptance."
      }
    ],
    "legacy_hash_matches_observed": {
      "SRC-0078": true
    }
  }
}
### Claim ledger
{
  "claims": {},
  "generated_at": "2026-09-07T17:45:46Z",
  "schema": "claude-obsidian.claim-ledger.v1"
}
