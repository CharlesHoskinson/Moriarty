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
updated_at: 2026-09-07T18:01:49Z
---

# Provenance mapping

The original [source inventory](../../evidence/source-inventory.csv) retains every `SRC-####` record and its original metadata. Existing `CLM-####` claims retain their wording, citations, confidence and S0-S7 lifecycle labels in the notes.

The portable [source ledger](ledgers/source-ledger.json) assigns the skill's content-derived `src-...` identity and stores a lossless `legacy_records` list for each original record. `legacy_source_ids` maps every original ID to its portable record. File hashes describe bytes observed during migration; `legacy_hash_matches_observed` distinguishes those hashes from the original inventory assertion. Missing, archived or compound locators are preserved as manual, unreviewed identities rather than invented file matches. Review state remains `unreviewed` until the portable provenance contract is assessed; this does not downgrade or replace the original scoped evidence.

The [legacy claim index](legacy-claim-index.json) maps exact observed claim identifiers to their note paths. It is a navigation index, not a semantic extraction or new proof. The portable [claim ledger](ledgers/claim-ledger.json) starts empty: migration does not automatically extract or accept claims from prose. Future scoped ingestion can add explicit support and contradiction records after reviewing the evidence. No automated claim acceptance occurred.

`created` on adopted notes records the earliest retained Git addition date. `updated` records the metadata migration date; the original `updated_at` remains available. Existing types, stable page IDs and lifecycle vocabularies are preserved.

The [migration receipt](../../evidence/obsidian-migration-2026-09-07/README.md) records the skill pin, transactions and checks. Return to [[wiki/workflow|the workflow]] or [[wiki/index|the research index]].
