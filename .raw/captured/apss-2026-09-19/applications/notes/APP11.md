---
id: apss.applications.app11
title: "Asynchronous vault applications need explicit claim states"
status: draft
source_id: APP11
reviewed_at: 2026-09-19
---

# Asynchronous vault applications need explicit claim states

Source: [ERC-7540 Asynchronous ERC-4626 Tokenized Vaults](https://eips.ethereum.org/EIPS/eip-7540). Publication/version: 2023-10-18 (created). Retrieved 2026-09-19T16:51:54.365510+00:00.

SHA-256: `1191348dda76df6e603350e0b6979c4d09d4a0abab4450cc6a1957f724dc631b`. Capture: [`captures/APP11.html`](../captures/APP11.html). Independence key: `erc7540`.

Evidence locator: Specification: Definitions, Request Flows, Request Lifecycle, Request Ids; cancellation rationale. PDF pages visually read: not applicable (HTML/Markdown source).

**Source claims.** ERC-7540 separates Pending, Claimable and Claimed requests, with controller/operator roles. Request and claim are distinct calls; supported asynchronous preview functions revert. Pending amounts need not retain yield or a fixed asset/share exchange rate. The standard does not prescribe a general cancellation flow.

**Moriarty inference.** Model a pending entitlement as an outstanding obligation, not a completed payment. Bind request identity/controller, consumed input, claimed amount and residual amount across transitions. Application-specific recovery and cancellation require explicit semantics instead of assumed ERC behavior.

**Limitation.** This is an EVM interface standard, not an implementation proof or a Midnight adapter. A interface-compatible contract can still contain bugs. No fixed redemption price or eventual availability follows from the existence of a claim. The current captured page identifies the standard as Final; its bytes and retrieval date are retained.
