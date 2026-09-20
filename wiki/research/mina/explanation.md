---
title: "What Mina adds to native Midnight recursion"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: explanation
tags: [moriarty, consolidation, recursion]
---

# What Mina adds to native Midnight recursion

Source observations: Pickles partitions verification between step/wrap and deferred/terminal checks. Its native verifier also checks accumulators. Kimchi binds verifier-index and recursive challenge context into transcripts. o1js separates proof parsing from verification, VK integrity from authorization, auxiliary data from proof statements, and proved computation from current-state settlement.

Recommendations: add MNR01–08 to the existing ZR contract for transcript order, deferred obligations/base masks, heterogeneous profiles, verified-value bindings, real proof modes, parameter/cache provenance, batch soundness and exact component assurance. Full details and source ranges are retained in the [study](../../../deliverables/mina-recursion-study-2026-09-19/RESULT.md), [semantics review](../../../deliverables/mina-recursion-study-2026-09-19/recursion-semantics.md) and [developer review](../../../deliverables/mina-recursion-study-2026-09-19/developer-integration.md).

The acquired Mina proof-systems gitlink matches the acquired proof-systems pin, but other independently acquired dependency HEADs differ. No integrated build is established. Mina uses distinct proof artifacts and cryptographic assumptions; its design does not prove Midnight correctness. Historical audit findings remain tied to 2023 scope and do not certify current HEADs.

[Interactive graph](../../../deliverables/mina-recursion-study-2026-09-19/graphify-out/graph.html) covers 400 selected files, with extracted/inferred relationships distinguished. It is navigation, not a proof graph. [Backend requirements](../../../docs/MORIARTY-BACKEND-REQUIREMENTS.md) · [Index](index.md).

