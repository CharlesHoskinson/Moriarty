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
3. Capture new source bytes into `.raw/captured/` through the portable core and
   record provenance and SHA-256. Existing `raw/` receipts remain immutable.
   Add the same SRC identifier to the legacy inventory and portable source mapping.
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
