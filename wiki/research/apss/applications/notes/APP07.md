---
id: apss.applications.app07
title: "Three levels of resource logic"
status: draft
source_id: APP07
reviewed_at: 2026-09-19
type: reference
created: 2026-09-19
updated: 2026-09-19
tags: [moriarty, apss, research]
---

# Three levels of resource logic

Source: [Anoma resource logic specification](https://specs.anoma.net/main/arch/system/state/resource_machine/data_structures/proof/logic.html). Publication/version: 2024-12-05 (page timestamp). Retrieved 2026-09-19T16:51:30.974339+00:00.

SHA-256: `c310a90a06b116b1f7b8d06673825df4e7838e2e7408cd46dab146bd6046fa28`. Capture: `captures/APP07.html` (`.raw/captured/apss-2026-09-19/applications/captures/APP07.html`). Independence key: `anoma`.

Evidence locator: Resource Logic; Proving; Instance, Witness and Constraints sections. PDF pages visually read: not applicable (HTML/Markdown source).

**Source claims.** Anoma resource logic constrains resource creation/consumption; the corresponding proof is required for action validity. The specification distinguishes architecture-level, instantiation-level and application-level inputs/constraints, with commitment and nullifier integrity checks.

**Moriarty inference.** Separate universal proof/ledger invariants, Midnight-specific encodings, and developer-defined predicates. This offers a useful design vocabulary for permissionless applications: shared validity does not imply a centrally curated catalog of application meanings.

**Limitation.** This is a particular Anoma specification snapshot, not a proof that Moriarty implements its machine. Its generic claim that a predicate is computable cannot replace Moriarty's documented finite fragment and cost bounds. Resource logic proofs do not by themselves establish observed external facts or solve transaction ordering/conflicts.


Evidence archive: [captured source manifest and reading records](../evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.
