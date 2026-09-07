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
updated_at: 2026-09-07T18:01:49Z
---

# Research workflow

This vault lives at the Moriarty repository root in WSL. The existing `wiki/` notes, `raw/` captures, `evidence/` receipts and root roadmap remain their canonical copies. Open `/home/charl/Moriarty` as a folder in Obsidian. On Windows, the same directory is `\\wsl.localhost\Ubuntu-26.04\home\charl\Moriarty`.

## Read and query

Begin with [[wiki/index|the index]], [[wiki/hot|current context]] and the smallest relevant notes. Use the installed `wiki-query` skill for cited, read-only answers. Saving a useful answer is a separate `save` operation within the user's requested scope. Do not capture transcripts or mutate notes merely because a session ended.

## Ingest and maintain

1. Read [AGENTS.md](../AGENTS.md), [WIKI_SCHEMA.md](../WIKI_SCHEMA.md) and the relevant user assignment.
2. Stage new inputs in `inbox/`. Use `wiki-ingest` and its reviewed capture workflow to preserve new source bytes under `.raw/captured/`. Existing `raw/` captures remain immutable and are already inside this vault. Record each new SRC identifier in both the source inventory and the portable ledger mapping; update citing-page backlinks in the same intake operation.
3. Reuse topic pages, preserve source/claim IDs and distinguish source facts, experiments, inference and open questions. Keep conflicting findings visible in [[wiki/contradictions|contradictions]].
4. Draft one scoped transaction with expected file hashes, note changes, source/claim records and index/log updates. Inspect the exact changed paths before applying. Existing authorization covers routine work within its stated scope; elapsed time never supplies missing authority.
5. Apply through the pinned claude-obsidian core. On conflict, re-read and rebuild; on interruption, use transaction recovery. Run `wiki-lint` read-only afterward. Apply Humanizer to maintained prose while preserving citations, data and code.
6. Commit and publish separately when authorized. There are no automatic commits or pushes, and staged inbox files and runtime recovery data are ignored by Git.

The source and claim vocabulary is explained in [[wiki/meta/provenance|the provenance mapping]]. Existing S0-S7 lifecycle labels stay authoritative for legacy claims. A structurally valid ledger does not prove correctness or confer reviewer acceptance.

## Local commands

The [setup guide](../docs/OBSIDIAN.md) records the pinned skill and portable commands. From this repository:

```sh
python3 /home/charl/.local/share/claude-obsidian/scripts/claude-obsidian.py doctor
python3 /home/charl/.local/share/claude-obsidian/scripts/claude-obsidian.py lint --strict
```

No REST server, MCP transport or community plugin is required. Skills resolve the vault through `.claude-obsidian.json`. The installed product stays outside the vault.
