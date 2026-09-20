---
title: "Daml semantics: initial distinctions"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: explanation
tags: [moriarty, daml, research]
---

# Consent, authority and settlement

The [authorization tutorial](https://docs.canton.network/appdev/modules/m3-authorization) demonstrates why an IOU owner may need protection against unilateral issuer archival and why assigning a party a signed contractual position requires its authority. Its proposal and standing-role examples provide different ways to obtain that authority. This motivates [MPLR-019](../mplr/MPLR-019.md), not a blanket requirement to approve every positive asset receipt.

The same tutorial's nontransitive-authority example shows that authority available to an outer action is not automatically ambient authority throughout nested calls. Moriarty should investigate explicit authority scope and attenuation under [MPLR-008](../mplr/MPLR-008.md).

[Ledger API authorization](https://docs.canton.network/appdev/deep-dives/authorization) describes participant service credentials. Those credentials are a different layer from the Daml language's action authorization. They are not a template for Moriarty deployment permission.

The opening [ledger-model example](https://docs.canton.network/overview/reference/ledger-model-detailed) shows both combined accept-and-settle and separate accept then settle. That is a useful distinction for conditional workflows. The example does not prove arbitrary independent-chain atomicity or eventual delivery.

The [privacy overview](https://docs.canton.network/appdev/deep-dives/privacy-model) distinguishes visibility from control. Its broad statements about who learns what must be reconciled with detailed projections, disclosure, topology and metadata. Canton encrypted views are not automatically equivalent to Midnight zero-knowledge guarantees.

These are provisional source observations and research directions. No Daml programs were executed, and no Moriarty target proofs were discharged in this intake.


## Completed comparative study

The initial intake above is retained as research history. The current bounded workstream is complete; use the [synthesis](synthesis.md) and [verified scope](reference.md) for final findings and remaining implementation obligations.
