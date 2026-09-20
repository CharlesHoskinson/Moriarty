---
title: Solver literature and enforcement reference
diataxis: reference
status: research-draft
---
# Solver literature and enforcement reference

Retrieved 2026-09-19. Reading is scoped below. Primary source means the source defines or studies its own mechanism, not that its claims were independently reproduced.

## SOL-01: CAKE framework

- Source: [CAKE framework](https://frontier.tech/the-cake-framework).
- Author/date: Frontier Research, 2024-02-15.
- Contribution: Applications and three infrastructure roles; solver markets vary in participation.
- Limit: Historical taxonomy; not a theorem or deployed conformance standard.

## SOL-02: Orderflow Auction Design Space

- Source: [Orderflow Auction Design Space](https://frontier.tech/the-orderflow-auction-design-space).
- Author/date: Gosselin and Chiplunkar, 2023-03-02.
- Contribution: Separates order type, disclosed information, bids and inclusion.
- Limit: Market statistics and product classifications are historical.

## SOL-03: Anoma Intent Machine

- Source: [Anoma Intent Machine](https://specs.anoma.net/main/system_architecture/state/intent_machine/index.html).
- Author/date: Anoma specification, retrieved 2026-09-19.
- Contribution: Solver interface accepts unbalanced transactions and returns balanced candidates.
- Limit: Abstract/versioned specification; not a Midnight implementation.

## SOL-04: Intents from the resource model perspective

- Source: [Intents from the resource model perspective](https://anoma.net/blog/intents-rm).
- Author/date: Yulia Khalniyazova, 2024-08-21.
- Contribution: Hard resource predicates and preferences are distinct; intent is not one transaction encoding.
- Limit: Explanatory design post; no global optimum or liveness proof.

## SOL-05: CoW mathematical solving problem

- Source: [CoW mathematical solving problem](https://docs.cow.fi/cow-protocol/reference/core/auctions/the-problem).
- Author/date: CoW documentation, retrieved 2026-09-19.
- Contribution: Acceptance sets, surplus, routing and bidding have separate roles.
- Limit: Restricted pairwise order model; continuous math requires execution semantics.

## SOL-06: CoW solver competition rules

- Source: [CoW solver competition rules](https://docs.cow.fi/cow-protocol/reference/core/auctions/competition-rules).
- Author/date: CoW documentation, retrieved 2026-09-19.
- Contribution: On-chain, off-chain and governance rules differ; solver whitelist required.
- Limit: Mutable service/protocol policy; social rules are not cryptographic enforcement.

## SOL-07: CoW bonding pools

- Source: [CoW bonding pools](https://docs.cow.fi/cow-protocol/reference/core/auctions/bonding-pools).
- Author/date: CoW documentation, retrieved 2026-09-19.
- Contribution: Solver participation uses vouching and team/governance processes.
- Limit: Do not import this service admission into Moriarty deployment.

## SOL-08: CoW fair combinatorial auction

- Source: [CoW fair combinatorial auction](https://docs.cow.fi/cow-protocol/concepts/introduction/fair-combinatorial-auction).
- Author/date: CoW documentation, retrieved 2026-09-19.
- Contribution: Batch and individual bids compete subject to a fairness filter.
- Limit: Consult detailed directed-pair rules; concept wording is simplified.

## SOL-09: UniswapX

- Source: [UniswapX](https://app.uniswap.org/whitepaper-uniswapx.pdf).
- Author/date: Adams et al., July 2023.
- Contribution: Signed Dutch orders, fillers, reactors and optional exclusivity; separate cross-chain designs.
- Limit: PDF 6 pages; visual pages 1–4 only. Historical design, not current deployment attestation.

## SOL-10: Optimal Routing for Constant Function Market Makers

- Source: [Optimal Routing for Constant Function Market Makers](https://arxiv.org/abs/2204.05238).
- Author/date: Angeris, Chitra, Evans, Boyd; title December 2021; arXiv v1 2022-04-11.
- Contribution: Convex routing under assumptions; fixed costs add mixed-integer decisions.
- Limit: PDF 16 pages; visual pages 1,5,6,7,12 only. Execution sequencing explicitly left to chain.

## SOL-11: CoW overview

- Source: [CoW overview](https://docs.cow.fi/).
- Author/date: CoW documentation, retrieved 2026-09-19.
- Contribution: Describes protocol as permissionless at overview level.
- Limit: Qualify by role using SOL-06/07; not independent corroboration of CoW claims.

## SOL-12: NEAR Intents market makers

- Source: [NEAR Intents market makers](https://docs.near-intents.org/integration/market-makers/introduction).
- Author/date: NEAR Intents documentation, refreshed 2026-09-19.
- Contribution: Message Bus is optional for protocol participation.
- Limit: Transport accessibility does not imply proof of general program correctness.

## Terms

| Term | Meaning here |
|---|---|
| Hard constraint | Predicate every accepted execution must satisfy |
| Preference | Objective used to rank already acceptable candidates |
| Economic solver | Candidate execution searcher |
| SMT solver | Formula checker within a theory and encoding |
| Synthesizer | Program-term searcher |
| Reactor/verifier | Mechanism checking the bound execution predicate |
| Settlement | Ledger-recognized effects under explicit finality assumptions |

## Evidence coverage

Web extracts and binary hashes are in round receipt files. PDF visuals are in `pixelrag/`; exact read pages and image hashes are in `visual-coverage.json`. PDF text extraction was used to navigate to fixed-cost analysis, not as a substitute for PixelRAG visual reading. No embedding retrieval index was built. CAKE/OFA and NEAR sources reuse this study's fresh captures; they are not counted again as independent sources. Diátaxis captures are methodology sources, not solver literature.

## Proposed Moriarty interface fields

`programHash`, `semanticsVersion`, `intentHash`, `predecessor`, `observationBindings`, `candidateEffects`, `feeVector`, `expiry`, `proofRequirements`, `evidence`, `searchOutcome`. These are design requirements, not a shipped API. Every field needs canonical encoding before implementation.
