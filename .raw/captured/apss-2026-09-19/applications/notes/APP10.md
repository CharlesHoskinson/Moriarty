---
id: apss.applications.app10
title: "AMM validity versus order-generation advice"
status: draft
source_id: APP10
reviewed_at: 2026-09-19
---

# AMM validity versus order-generation advice

Source: [CoW AMM technical specification](https://raw.githubusercontent.com/cowprotocol/cow-amm/bb4b1ee43eeffd463bb8a981e343844a5a515e2a/docs/amm.md). Publication/version: 2024-05-14T08:27:31Z. Retrieved 2026-09-19T16:51:34.855067+00:00.

SHA-256: `f13252e5578bdf62d72398f8443fde2e7515b3b42390c01186540acb56d649b7`. Capture: [`captures/APP10.md`](../captures/APP10.md). Independence key: `cowprotocol`.

Evidence locator: Overview; Limitations; Settling a custom order; Risk profile. PDF pages visually read: not applicable (HTML/Markdown source).

**Source claims.** CoW AMM permits custom orders respecting a nondecreasing reserve-product invariant, restricts each AMM to one order per batch, and requires a pre-interaction commitment. Its oracle assists order generation but does not determine order validity. The document states that this implementation does not pool liquidity across users.

**Moriarty inference.** Keep optimization/advice outside mandatory validity predicates. A solver can propose another valid trade without inheriting the oracle's authority. Composition constraints such as one operation per instance per batch belong in the formal effect relation, not only the UI.

**Limitation.** Reserve-product preservation alone does not prove best execution or the user's minimum net receipt. The documentation's economic-performance assertions are not reproduced here. Both protocol-specific assumptions and complete user intent remain necessary; do not adopt this AMM as Moriarty's sole swap primitive.
