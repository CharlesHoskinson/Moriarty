# Wiki log

## [2026-09-03] audit | Evidence and SDK gate hardening

Closed the independent review findings. The disclosure validator now handles
whitespace, excludes comments and strings, and rejects unnamed compound
expressions. Complete compiler-interface metadata is checked against the
Moriarty manifest. Clean reproduction and current-checkout evidence now have
separate identities. Sprint manifests bind package, scope, outputs, self-hash,
and package-specific gates. The SDK validator applies its schemas, requires
exact counts, closes component references, and recomputes its complete index.

## [2026-09-03] experiment | Clean Compact reproduction and complete SDK contracts

Reproduced E00 from a clean archive and fresh environment. Forty-three focused
tests passed, 1,000 traces had zero divergence, and all semantic, Compact,
compiler-manifest, ZKIR, and negative-control digests matched. Recorded the
Compact source-map path-sensitivity footgun. Added an independent Moriarty
visibility-manifest validator with missing and additional disclosure controls.
Specified 65 SDK components, 28 canonical data contracts, two JSON contract
schemas, and a 17-component minimum safety spine. Preserved the Grok, Sol, and
exact Fable 5.1 Council advisory and its requested changes.

## [2026-09-03] decision | Evidence-gated sprints and complete SDK scope

Superseded calendar-based progress with evidence-gated sprints. Added the
research journal and an iteration-scoped semantic ledger. Classified E00 as a
narrow Compact DSL feasibility result rather than general language evidence.
Expanded the SDK boundary to the complete authoring, compiler, analysis,
packaging, verification, wallet, chain, and operations development system.

## [2026-09-03] experiment | Moriarty Core atomic-swap stop test

Implemented the finite E00 Core subset and canonical two-token swap. Generated
fixed-state Compact, an artifact manifest, and an independent transition
machine. One thousand unique traces produced zero divergence and zero invariant
failures across 18 required deadline-boundary cells and both terminal-expiry
rejections. Follow-up semantic review added manifest-driven time guards and
timeout priority, canonical refund ordering for all accepted party names,
small-deadline coverage, and exact Compact integer validation. The first Compact
compile reproduced an undeclared-disclosure
footgun for the decision argument. The corrected source lists and applies the
public disclosure explicitly; the failing source and diagnostic digest are now
preserved. Compact emitted four ZKIR 3.0 circuits, and the pinned mock compiler
accepted all four. Constructor values, real keys, and proof generation remain
open.

## [2026-09-03] decision | DeFi Kernel prompt, council, and graph

Reproduced the 47/72 1-NN, 50/72 3-NN, Jaccard, and exact pair-rate results.
Ran tool-free round-1 proposals with Grok, exact Fable 5.1, and GPT-5.6 Sol,
then a blinded round 2; recorded Fable's budget-exhausted second round as an
abstention. Added the 12-workstream XML prompt, gated 90-day sprint, seven-family
Marlowe mapping, and the library-only stop path. Built a six-source Moriarty
decision graph with 46 nodes, 45 directed edges, and zero missing endpoints.

## [2026-09-03] ingest | Repository-verified taxonomy update

Preserved the decision-bearing claims from the user-supplied updated run as
SRC-0017. Reproduced the DeFiFormal pair, canonicalization, obligation, roster,
and v3 self-test counts at commit
`8ae0bbfaa3193078d1cabf6999db1382985b7f95`. Replaced M4+ as the top-level
decision with M2+M3 human-facing families and facets plus an M5 formal behavior
profile. Added a 72-row family/facet crosswalk while preserving the M4+ file as
historical input.

## 2026-09-02 — Moriarty decision packet

- Added the consolidated decision study and the standalone stakeholder
  pre-read.
- Added an experiment handoff for the compiled Compact/ZKIR escrow vertical
  slice.
- Kept Marlowe as the upstream and migration name; Moriarty is the new language
  and repository name.

## [2026-09-02] ingest | Marlowe organization and live documentation graph

Refreshed and pinned all 36 `marlowe-lang` repositories, acquired all 100 live
documentation sitemap paths with Scrapling, added the official online entry
points, and built the combined AST, semantic, documentation, and provenance
graph. The graph contains 8,864 nodes, 10,888 edges, and 120 hyperedges. Its
1,510 dangling endpoints and 265 undirected relation collapses are recorded as
health limitations rather than hidden.

## [2026-09-02] ingest | DeFi taxonomy report and local corpus reconciliation

Preserved and extracted the user-supplied 25-page taxonomy PDF. Reconciled its
public-access limitation against the authorized clean local DeFiFormal checkout
at commit `8ae0bbfaa3193078d1cabf6999db1382985b7f95`. Reproduced the 72-row corpus,
60 constructions, 1,259 obligations, 570 covered obligations, and 689 residue;
generated a one-to-one construction roster and draft M4+ crosswalk.

## [2026-09-02] design | Canonical Moriarty patterns for the 72-row roster

Assigned every DeFiFormal row to a bounded canonical application pattern and
added 13 surface-language strawmen. D01–D11 each receive one reference pattern;
D12 splits into an event-contingent market and a delegated-curator vault. The
sketches separate provable Core obligations from oracle, bridge, custody,
identity, solver, validator, and legal capabilities.

## [2026-09-02] ingest | Research assignment and LLM Wiki method

Initialized the research repository around the upstream Marlowe modernization
assignment, preserved the controlling prompt, installed the acquisition
environment, and ingested Karpathy's original LLM Wiki idea file as `SRC-0001`.

## [2026-09-02] decision | Rename to Moriarty

Renamed the repository and proposed language to Moriarty. Marlowe now refers
only to the upstream source language, implementation, and migration baseline.

## [2026-09-02] ingest | Midnight and active Compact repositories

Acquired the official Midnight documentation corpus with Scrapling, enumerated
and cloned all 74 public `midnightntwrk` repositories, followed the official
Compact relocation, and cloned all three public LFDT Minokawa repositories.

## [2026-09-02] experiment | Moriarty escrow to Compact and ZKIR 3

Built Compact compiler 0.34.100 from pinned source, compiled a finite Moriarty
escrow lowering with the ZKIR 3 backend, passed six acceptance tests and 44 ZKIR
library tests, and mock-compiled all three generated circuits. Full proof tests
remain dependent on external `MIDNIGHT_PP` parameter files.
