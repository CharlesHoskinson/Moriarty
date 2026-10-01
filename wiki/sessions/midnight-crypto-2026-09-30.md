---
title: Midnight Rust cryptography experiments
type: session
status: research-draft
created: 2026-09-30
updated: 2026-09-30
tags: [moriarty, midnight, cryptography]
sources: [SRC-0114]
---

# Midnight Rust cryptography experiments

User-requested follow-up to the published authoring beta. [Working dossier](../../wiki-llm/midnight-crypto-2026-09-30/README.md) and [measured results](../../wiki-llm/midnight-crypto-2026-09-30/RESULT.md) preserve source pins, separate dependency families, original failures and test predicates.

Experiment observations: official Rust BIP340/ECDSA verification on five actual local financial fixtures, 2,506 changed-leaf signature refusals, 25 independently observed Core rejections with freshly valid signatures, local Merkle state, a smallest real PLONK/KZG proof and privileged in-memory ledger failure tests. Twelve Rust integration tests pass. Source collection SRC-0114 contains 16 full selected files, manifests and licenses; relevant API sections were inspected, not the full repositories.

These tests do not implement the production intent codec, authenticated key/head bindings, complete native financial proofs, atomic Moriarty financial ledger settlement or Preview finality. The beta remains PreparedUnqualified. [Delivery and audit receipts](../../wiki-llm/midnight-crypto-2026-09-30/DELIVERY.md) report completion separately; this note creates no accepted financial/proof claim.
