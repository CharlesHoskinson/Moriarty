---
id: apss.applications.app09
title: "Programmable orders, optional watchers and funding hazards"
status: draft
source_id: APP09
reviewed_at: 2026-09-19
type: reference
created: 2026-09-19
updated: 2026-09-19
tags: [moriarty, apss, research]
---

# Programmable orders, optional watchers and funding hazards

Source: [ComposableCoW README](https://raw.githubusercontent.com/cowprotocol/composable-cow/bc4d5f66fe044fdd8fbc78c9cf6ae99158f06a4c/README.md). Publication/version: 2026-09-12T15:07:54Z. Retrieved 2026-09-19T16:51:33.650142+00:00.

SHA-256: `5915b88256e19b3a0b3071b2f5591e37b6e8eed3e3104f8c4b3e725587ff8966`. Capture: `captures/APP09.md` (`.raw/captured/apss-2026-09-19/applications/captures/APP09.md`). Independence key: `cowprotocol`.

Evidence locator: README Methodology; TWAP; Just-in-time funding; Handler compatibility. PDF pages visually read: not applicable (HTML/Markdown source).

**Source claims.** ComposableCoW separates conditional-order parameters, handlers, generated discrete orders and cancellation. Event dispatch to a watchtower is optional. Its funding-poller documentation allows independently built handlers without a handler allowlist but warns that handler outputs control token/amount and that new digests can trigger repeated funding.

**Moriarty inference.** A programmable order service is an application built from language capabilities. Permissionless extension must be paired with signed cumulative debit and lifecycle constraints, not trust that a handler's new digest represents a genuinely new economic authorization.

**Limitation.** This is captured repository documentation, not a source-code audit or deployment test. No claim is made that all CoW solver participation is permissionless. Optional watchtower disclosure does not prove end-to-end privacy. Handler authority and token approvals must be analyzed separately from the ability to publish handlers.


Evidence archive: [captured source manifest and reading records](../evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.
