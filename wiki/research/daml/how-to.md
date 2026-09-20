---
title: "Evaluate a Daml workflow for Moriarty"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: how-to
tags: [moriarty, daml, research]
---

# Trace a claim into a language requirement

1. Identify whether the source describes Daml semantics, Canton protocol, API access, a token interface or a hosted service. Record version and source hash.
2. Write the intended economic outcome and each party's explicit consent. Separate evidence predicates from the truth or availability of external evidence.
3. Enumerate create, exercise, fetch, consume, reserve and transfer transitions. Identify atomic boundaries and persistent obligations between commits.
4. Construct one successful trace and adversarial traces: missing consent, stale delegation, double consumption, late success, cancellation race and partial delivery.
5. Record who observes each transition and which external assumptions are necessary.
6. Map the result to an existing MPLR before allocating a new ID. A new mechanism alone is not a new requirement.
7. State the Midnight ZKIRv3 correspondence obligation. Keep source reading, execution tests and proofs separate.

The first batch's manifest and reproduction scripts are under `/home/charl/research/daml-teardown-2026-09-19`. [Coverage](reference.md) records which pages were actually read.
