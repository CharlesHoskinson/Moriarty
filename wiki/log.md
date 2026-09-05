# Wiki log

## [2026-09-04] checkpoint | Complete S01 and begin S02 contract

Independent re-review approved the settlement correction with no remaining
material finding. Closed S01's seven bounded specification tasks and retained
the unmechanized theorem and signing-denied local experiment limits. Updated
the program register and index to the reviewed S02 contract plan. No model
comparison, architecture selection, or later release gate is claimed.

## [2026-09-04] correction | Separate settlement process from receipt

Corrected the S01 alias against XML v1.3: `settlement` names the process, while
`SettlementReceipt` is evidence about a step. Preserved the prior manifest
byte-for-byte and its SRC-0032 provenance; registered the revised manifest as
SRC-0034. The correction does not change Core or semantic scope. Independent
re-review remains required before closing S01. Also clarified that the user
directed Quint instead of TLC and the reviewed design selects Apalache.

## [2026-09-04] experiment | S01 evidence-only intent-safety freeze

Recorded the architecture-neutral S01 terminology, observation model, hard
predicates, preferences, assumptions, and candidate-unmechanized theorem. The
validator recomputed all ten local package predicates. The local swap baseline
was `VALID`, the extra-effect mutant was `UNAUTHORIZED_EXTRA_EFFECT`, and both
certificates denied signing. No production signing request was attempted. The
transition preserves semantic scope `0.0.0-e00.2` and its frozen digest.
Mechanization, authenticated effect completeness, runtime `SignBeforeResolve`
verification, proof and backend correspondence, ledger execution, ACTUS, human
pilots, the 24 prompt release gates, and Task 7 review remain open. S02 retains
the four-way architecture decision and uses Quint with Apalache, not TLA+/TLC.

## [2026-09-04] audit | Prompt v1.3 and initial S01 execution

Audited the focused XML assignment, approved S01 design, and implementation plans.
The [audit](../docs/superpowers/reviews/2026-09-04-moriarty-v1.3-execution-audit.md)
records repository observations, sample-code probes, and unresolved freeze obligations.
The S01 OpenSpec contract begins implementation. The theorem and aggregate
evidence gates remain open. The prompt, approved design, and Core snapshot retain their pins.

## [2026-09-03] research | ACTUS terminal completeness prompt

Captured the complete public ACTUS site and documentation sitemap with
Scrapling, pinned eleven official and three comparative repositories, and
inventoried 32 taxonomy rows, 18 executable contract types, and all 277 public
reference fixtures. Direct code inspection identified the public Haskell
implementation's four missing executable types, seven excluded fixtures,
analysis-date omission, three-field comparator, and binary32 payoff downcast.
The official Java core remained access-controlled and unused. Revised the
focused prompt to version 1.3 with typed ACTUS packages, two independent
semantics, shared Core and Compact compilation, full present-field comparison,
24 release gates, and compatibility-only language. Semantic scope
`0.0.0-e00.2` did not change.

## [2026-09-03] research | CAKE, NEAR Intents, and Ethereum intent lifecycle

Acquired the 14-link CAKE Working Group corpus and all 68 official NEAR Intents
documentation pages with Scrapling, including the full documentation aggregate
and two OpenAPI specifications. Reconstructed the deployed objective, quote,
authorization, Verifier, relay, bridge, fill, payout, cancellation, refund, and
finality boundaries. Revised the focused research prompt to version 1.1 with 33
intent data contracts, current ERC-7683 versus OIF version separation, resolver
snapshot obligations, delivery deduplication, asynchronous cancellation, and
cross-domain fulfillment tests. The semantic scope remains `0.0.0-e00.2`.

## [2026-09-03] design | Moriarty semantics and intent research prompt

Added a focused XML assignment for Moriarty semantics, intent correctness,
verified compilation, Compact and ZKIR realization, and the standard developer
interface. Acquired the CAKE framework and 28 official ERC and EIP pages with
Scrapling. Preserved exact status, source date, path, and SHA-256 evidence. The
assignment uses twelve evidence-gated sprints and makes no semantic change.

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

## [2026-09-03] research | Complete NEAR Intents documentation and prompt council

Acquired all 68 official NEAR Intents sitemap pages plus both published OpenAPI
documents with Scrapling. Separated internal-ledger execution, source fill,
claim, destination finality, and destination spendability. Grok, Sol, and
Fable reviewed one frozen prompt draft. The single correction pass produced
prompt version 1.2 with execution-state authorization, cryptographic lifecycle
bindings, typed verification certificates, and a separate confidential trust
profile. Semantic scope `0.0.0-e00.2` did not change.

## [2026-09-05] checkpoint | Record branch-only S02 progress and Council restart decision

Recorded the effect foundation candidate and corrected receipt commit
`1bd4bff04fc24855b1c45ff95b1eab137909434d`. Recorded the exact-parent
consumption foundation at `8b905114c1cec79faf555974c0267f183ca31399` and its
honest receipt whitespace correction at
`6a60a645c03acad83b7cbc6b85d43996cd40ca65`. All three commits remain only on
`s02-model-comparison`. They are not integrated or Council-approved.

Recorded the 2026-09-05 user decision to authorize the narrow Foreman runtime
binding fixes, GitHub publication and merge, the documented official model-route
limitations, and Council resumption. Runtime implementation remains pending.
No Council review was dispatched. Full S02 and S03 through S15 remain
incomplete. Historical log entries remain unchanged above this entry.

## [2026-09-05] assignment | Investigate Foreman PID-namespace degradation

Created the [Fable research assignment](../deliverables/foreman-grok-4-6-pidns-deep-research-prompt-2026-09-05.xml)
for the user's separate session. It includes the observed launcher warning,
source and binary context, protected live work, and the official 42-page Grok
4.6 card with its preserved PDF digest. The assignment requires read-only
diagnosis, complete-card coverage, and explicit scrutiny of cleanup claims.
The XML parser accepted the assignment. No root cause, remediation, Council
approval, or containment guarantee follows from creating this prompt.

## [2026-09-05] checkpoint | Resume the original v1.3 workstream

Recorded Foreman candidate `ac7c2deff6e39144c29836611ee85800ffed41c8`
and its incomplete status in research-journal claim CLM-0139. Task 1 needs
correction. Task 2 and all requested Council gates remain pending.
Recorded the scanner resource-limit refusal separately from a secret finding.
The unchanged scanner passed after the unused root dependency install left
the worktree. The install remains recoverable in the external run directory.
No security policy changed, no GitHub merge occurred, and no gate passed.
