---
id: moriarty.research.program
type: decision
title: Moriarty DeFi Kernel research program
status: active
updated_at: 2026-09-03T14:17:00Z
sources:
  - SRC-0016
  - SRC-0017
  - SRC-0018
  - SRC-0020
  - SRC-0021
  - SRC-0022
  - SRC-0023
  - SRC-0024
  - SRC-0025
  - SRC-0026
---

<!-- markdownlint-disable MD025 -->

# Moriarty DeFi Kernel research program

## Current decision

Run a sequence of evidence-gated feasibility sprints. Do not count calendar
time as progress. Treat F1-F6/P as a falsifiable human
taxonomy, not language syntax. Use M5 formal behaviors inside the specification
and assurance system. Generate reviewable Compact before ZKIR. Keep audited
Compact libraries as the default fallback if a separate language has no measured
assurance or review advantage.

**CLM-0115.** Three terminal round-1 provider results independently recommended
revision rather than production approval. They agreed on a finite Core, an
external taxonomy, M5 as the internal profile, generated Compact as the first
backend, explicit open-world assumptions, and a serious library-only fallback.
The blinded second round counted Grok and Sol; Fable 5.1 exhausted its bounded
budget and is recorded as an abstention. Source: SRC-0018 and its receipt;
created 2026-09-03; authority advisory design review; scope Moriarty feasibility
decision; evidence experiment observation; reproduction not applicable;
confidence medium; status S2.

The durable XML research prompt is
[`../deliverables/moriarty-defi-kernel-deep-research-prompt-2026-09-03.xml`](../deliverables/moriarty-defi-kernel-deep-research-prompt-2026-09-03.xml).
It defines 12 workstreams, 26 experiments, 22 deliverables, 18 release gates,
the Scrapling-only web policy, the wiki/graph loop, the 72-row coverage rule,
and the final ten-part decision contract. The active sprint contract is
[`../deliverables/moriarty-sprint-program-2026-09-03.md`](../deliverables/moriarty-sprint-program-2026-09-03.md).
The calendar-based plan is retained only as superseded history.

## Earliest stop test

Before enlarging Core, compile a Marlowe-shaped fragment with bounded accounts,
deposit, choice, enforceable timeout, and atomic two-token swap. Produce a
client-checkable artifact and disclosure manifest, inspect every loop,
collection, and witness callback, construct a translation-validation
certificate, and run 1,000 coverage-guided differential traces. An unbounded
generated collection or unconstrained witness callback forces library-only or
stop.

The next high-risk experiment composes binary conditional-token split/merge with
atomic exchange. It must exercise invalid attestation, replay, partial-payment
policy, timeout, refund, artifact identity, and disclosure behavior. Release
requires real proof generation and at least 10,000 materially distinct,
coverage-guided traces per canonical application.

## Graph

**CLM-0116.** The incremental Moriarty decision graph contains 46 nodes, 45
directed edges, three hyperedges, six source files, and zero missing endpoints.
It was built with Graphify API 0.9.53 from an `agy` semantic extraction. Inferred
and ambiguous edges remain hypotheses. Source: SRC-0018 plus
`evidence/moriarty-decision-graph-2026-09-03.json`; build date 2026-09-03;
authority derived experiment; scope research navigation; evidence experiment
observation; reproduced; confidence high for graph counts and low-to-medium for
inferred relationships; status S3.

The graph is at
[`../graphs/moriarty-decision-corpus/graphify-out/graph.json`](../graphs/moriarty-decision-corpus/graphify-out/graph.json).
Its shortest useful synthesis path joins the candidate taxonomy, M5 profile,
atomic-swap stop test, backend boundedness condition, and library-only fallback.
Graph centrality is not evidence authority.

## Focused semantics and intent program

**CLM-0120.** The CAKE framework separates a chain-abstraction product into
Application, Permission, Solver, and Settlement layers. It defines intent as an
expected output rather than one fixed transaction path. Moriarty will test this
as an architecture boundary, not as normative semantics. Source: SRC-0020,
section “Introducing the CAKE Framework”; published 2024-02-15; authority
primary descriptive research; scope comparative architecture; evidence source
fact; reproduction not applicable; confidence high; status S2.

**CLM-0121.** The official pages retrieved for ERC-7683 and ERC-7521 label both
standards Draft. The current ERC-7683 text centers on a resolver that translates
an opaque protocol payload into solver-facing instructions and assumptions. Its
security section does not guarantee settlement-protocol security. Source:
SRC-0021, records MIS-020 and MIS-024; created 2023-09-19 and 2024-04-11;
authority primary standards documents; scope comparative intent standards;
evidence source fact; reproduced acquisition; confidence high; status S2.

**CLM-0122.** EIP-712 is Final and defines typed structured-data signing with
domain separation. Its abstract and security section state that it does not add
replay protection. Moriarty must specify nonce, consumption, cancellation,
network, contract, version, sequence, and validity rules separately. Source:
SRC-0021, record MIS-006; created 2017-09-12; authority primary standards
document; scope signature-interface comparison; evidence source fact and design
inference; reproduced acquisition; confidence high; status S2.

**CLM-0123.** The complete official NEAR Intents documentation sitemap contains
68 pages. All 68 pages, the full documentation aggregate, and two published
OpenAPI documents were acquired with adjacent Scrapling receipts. The Verifier
provides an internal-ledger `token_diff` conservation rule, but external calls,
bridge withdrawals, payouts, and refunds have separate asynchronous and trust
assumptions. Source: SRC-0024; acquired 2026-09-03; authority primary
deployed-system documentation and schemas; scope comparative intent lifecycle;
evidence source fact and reproduced acquisition; confidence high for coverage
and medium for deployed correspondence; status S6.

**CLM-0124.** The standards do not define one interchangeable intent object.
Moriarty research must keep objective, quote, authorization, order payload,
resolution snapshot, solver plan, fill, fulfillment proof, claim, cancellation,
refund, and final settlement separate. Source: SRC-0023 through SRC-0025;
created 2026-09-03; authority evidence-backed architecture recommendation;
scope Moriarty intent calculus and SDK; evidence inference; confidence high;
status S2.

**CLM-0125.** Current ERC-7683 is a resolver interface and differs materially
from its prior order-and-fill draft and current OIF `StandardOrder` vocabulary.
Any Moriarty adapter must bind resolver code and upgrade state, resolution
block, payload, resolved plan, witnesses, queries, and assumptions. Source:
SRC-0025 and the primary standards in SRC-0021; observed 2026-09-03; authority
primary standards and repository comparison; scope Ethereum compatibility;
evidence contradiction and recommendation; confidence high; status S2.

The focused assignment is now version 1.1:
[`../deliverables/moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml`](../deliverables/moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml).
It defines the intent-refinement theorem, compiler and proof artifacts, CAKE and
NEAR/ERC/OIF review, 33 intent data contracts, a complete developer interface,
twelve workstreams, thirteen experiments, eighteen deliverables, twelve
sprints, and eighteen release gates. It does not change the active semantic
scope.

## Preserved dissent

The main unresolved classification dispute is whether F5, F6, and Prediction
deserve family status. The strongest rival uses four mechanisms—exchange,
credit, derivative, and consensus claim—then treats off-chain claim source,
mandate, and conditional tokens as facets or mechanisms. A separate proposal
adds F0 conditional transfer for escrow, vesting, payments, and streams. The
taxonomy workstream must compare these using the same corpus and real human
raters; Core must not wait for or encode the winning market vocabulary.
