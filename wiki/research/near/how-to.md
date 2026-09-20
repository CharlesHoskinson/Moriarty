---
title: "Inspect the NEAR corpus"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: how-to
tags: [moriarty, near-teardown, research]
---

# Trace a NEAR behavior into an MPLR

1. Start with the [catalog graph](../../attachments/near-teardown/graph.html), then identify the pinned repository in `clone-results.json`.
2. Use the appropriate `graphify-out/ast/OWNER/REPO/graph.json` to find symbols. Check its health/extraction log for omissions.
3. Inspect the exact source range and corresponding captured documentation; distinguish source configuration from deployed behavior.
4. Trace dispatch, local effects, asynchronous results, recovery and all remaining duties. Use a negative example that can falsify the proposed guarantee.
5. Link the observation to an MPLR behavioral requirement; record theory alternatives, missing evidence and the corresponding EARS scenario.

The reproducible acquisition, graph and PDF scripts are in `/home/charl/research/near-teardown-2026-09-19`. They fetch public sources and write local research artifacts. No study submitted financial transactions. The abstraction study includes an independently reproduced signing-vector check; the runtime and kernel studies do not claim fresh full builds or live-network tests.

A source hash proves which bytes were studied. It does not prove a semantic judgment. A model endorsement does not replace the actual target proof or a negative-control result.
