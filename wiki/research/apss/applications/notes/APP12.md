---
id: apss.applications.app12
title: "Resolver interoperability is not settlement correctness"
status: draft
source_id: APP12
reviewed_at: 2026-09-19
type: reference
created: 2026-09-19
updated: 2026-09-19
tags: [moriarty, apss, research]
---

# Resolver interoperability is not settlement correctness

Source: [ERC-7683 Cross Chain Intents](https://eips.ethereum.org/EIPS/eip-7683). Publication/version: not determined. Retrieved 2026-09-19T16:51:55.471147+00:00.

SHA-256: `a4d5db4af9d56dceb95128dfd269240edbd40eb3f378b21ea28ce530910aa385`. Capture: `captures/APP12.html` (`.raw/captured/apss-2026-09-19/applications/captures/APP12.html`). Independence key: `erc7683`.

Evidence locator: Abstract; Motivation; Resolvers; Rationale; Security Considerations. PDF pages visually read: not applicable (HTML/Markdown source).

**Source claims.** The current ERC-7683 draft standardizes solver-facing resolution of protocol-specific orders, rather than a common escrow or settlement contract. Resolvers disclose assumptions. Its security section explicitly separates the interface from the security of the settlement protocol and considers the full capital/authorization exposure window.

**Moriarty inference.** Export useful order descriptions without making a single router mandatory. Resolver vetting is a solver's risk policy; it must not become a language-wide developer allowlist. Prove the actual accepted effects satisfy the signed program/intent regardless of a resolver's reputation.

**Limitation.** This current draft differs materially from older order-interface versions; historical captures must retain their version scope. Offchain `eth_call` resolution does not itself verify financial settlement. No EVM interoperability implementation is claimed for Moriarty.


Evidence archive: [captured source manifest and reading records](../evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.
