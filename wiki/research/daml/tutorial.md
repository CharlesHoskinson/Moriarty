---
title: "Escrow comparative thought experiment"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: tutorial
tags: [moriarty, daml, research]
---

# Walk a proposed settlement

This is a research exercise, not executable Moriarty or a deployed Daml application.

1. Alice proposes exchanging an asset with Bob. Record terms and the required evidence, but do not call the proposal funded.
2. Bob accepts the exact position or uses a previously authorized policy. Test a modified proposal that would give Bob an unexpected liability; it must not borrow consent from the original proposal.
3. Reserve assets using a declared escrow policy. Record who may release, cancel or recover each reservation.
4. Accumulate signatures, document attestations and proofs. Proving an attestation signature does not prove the external fact it describes.
5. If only one leg can settle, preserve the committed prefix and all residual obligations. A coordinated atomic ledger commit and a multi-chain staged workflow require different evidence.
6. Race cancellation with a late successful external transfer. Classify each observed outcome; do not infer nonexecution from timeout alone.

Compare Daml's [proposal and role patterns](https://docs.canton.network/appdev/modules/m3-authorization) with NEAR's staged receipts and the existing [MPLRs](../mplr/index.md). The research direction defines the further cases to test.
