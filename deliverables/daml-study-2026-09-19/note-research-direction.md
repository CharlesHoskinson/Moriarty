---
title: "Daml research direction"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: explanation
tags: [moriarty, daml, research]
---

# Daml research direction for Moriarty

Status: initial research design, 2026-09-19. This is an evidence agenda, not an adopted compiler design or completed full Daml study.

## Purpose and constraints

Moriarty is an open programming language for all Midnight DeFi developers. Its output must execute on Midnight ZKIRv3 and carry evidence for authenticated formal intention. Conditional settlement with programmable escrow is a major abstraction pillar. No project permission to deploy programs belongs in language validity. Party consent and application-defined authorization remain essential.

The Daml study should answer what a language must express, what its semantics can guarantee, what requires a runtime or external assumption, and what can survive lowering to Midnight. It must not conflate Daml, Canton synchronization, participant Ledger API authorization, Splice, token standards, or hosted services.

## Approaches considered

1. Workflow-first, with semantics and adversarial traces (recommended). Start with a multiparty escrow and systematically vary consent, evidence, liquidity, timing and settlement domains. It exposes required behavior and gives language comparisons a common test. Risk: missing unrelated useful language features; mitigate with the full documentation inventory and a feature-to-workflow crosswalk.
2. Language-first feature catalog. Templates, choices, interfaces, packages and data types are easy to compare against Moriarty/Aeon. Risk: copying syntax or implementation mechanisms without knowing which user need they satisfy. Use as a secondary index.
3. Canton protocol-first teardown. Useful for privacy and synchronized commits. Risk: attributing network guarantees to the language or importing participant onboarding into public program validity. Use as a separate assumption and runtime-boundary track.

## Eight research tracks

| Track | Questions | Evidence and decisive test | Moriarty destination |
|---|---|---|---|
| Obligation formation | Can a party receive a liability without consenting? Does receiving value differ from accepting a debt or duty? | IOU/propose-accept examples; reject unsolicited negative positions; allow policy-authorized positive transfers | New MPLR-019 candidate, MPLR-008/016/017 |
| Scoped authority | Which authority is available at each nested action? Can a helper accidentally inherit a caller's unrelated rights? How do standing roles expire or get revoked? | Formal authorization judgments; nontransitive-authority negative example; stale delegation trace | Refine MPLR-008; research capability and authorization logics |
| Conditional settlement | How are propose/accept/reject/cancel, allocations, escrow and DvP represented? Which party can cancel, and when? | Current token interfaces v1/v2, application examples, blocked or partially funded settlement | MPLR-003/004/005/010/018 |
| Composition and atomicity | Is composition a single committed transaction or a persistent multi-transaction business workflow? What is supported across synchronizers versus independent chains? | Transaction-tree semantics, conflict/abort traces, explicit boundary matrix | MPLR-001/002/006/007/011/014 |
| Evidence and privacy | Who sees contract data, nested actions, fetched evidence and metadata? Does disclosure confer authority? | Ledger-model rules and protocol papers; adversarial observer/controller and shared-validator cases | MPLR-013/018; proof-relevant evidence and information-flow research |
| Evolution and identity | Which package/interface upgrades preserve outstanding consent? Can a condition or asset identifier silently change meaning? | Version-pinned upgrade rules, package hashes, keys and interface semantics; stale signed-intent witness | Refine existing requirements or propose distinct MPLR after evidence |
| Financial semantics | Are holdings assets, issuer liabilities, entitlements or reservations? How are rounding, settlement fees, deadlines and residuals defined? | Daml Finance and current token standard, quantity conservation and liability conservation separately | MPLR-004/009/017 |
| Proof and trusted basis | What is proven about Daml semantics, and what remains runtime validation? What can inspire certified Moriarty primitives? | Daml paper, formal ledger work and implementation correspondence; relate to Simplicity jets and ZKIRv3 obligations | MPLR-012/014 plus certified-basis agenda |

## Common case study and variants

Start with a buyer, seller, issuer/custodian, optional document attestor, solver and destination owner. The request fixes the intended assets, quantities, conditions, settlement destination, fees, evidence policy and deadlines. Distinguish request recording, asset reservation, evidence accumulation, readiness, delivery and discharge of residual duties.

Vary one dimension at a time: an unfunded proposal; staged funding; third-party acceptance; all-of and threshold evidence; revoked or stale documents; privacy-preserving proof of a document predicate; partial fills; expiry while a remote leg is unresolved; duplicate/reordered results; late successful delivery; cancellation racing with acceptance; asset issuer default; application-selected dispute resolution; upgrades after an intent is signed.

A condition proof cannot by itself establish an external document's truth or another chain's finality. Model who attests, what proposition is verified, freshness, identity binding, and all trust/liveness premises. Do not assume a timeout proves nonexecution. Do not impose discretionary dispute actors on every program.

## Evidence method

Capture current documentation with Scrapling after robots inspection; retain URL, status, exact bytes and hashes. Keep archived versioned docs separate from current docs. Record retrieval versus actual reading separately. Use PixelRAG rendering for PDFs and record inspected pages. Cite primary sources per claim. Mark source-based observations provisional until semantics and examples support them; high-level documentation can overstate privacy and atomicity.

The initial batch is 190 pages selected from a 1,427-URL current sitemap, plus discovery captures and one 14-page Daml language paper. The remainder of the current inventory, archive inventories, detailed compiler/runtime correspondence, Daml Finance and relevant additional papers remain open. This is not an all-version archive claim.

## Deliverables and exit criteria

- Diataxis notes: explanation of semantics; reference with source/version/coverage; how-to for reproducing claim checks; tutorial that walks the escrow trace without pretending it is implemented Moriarty.
- A Daml/NEAR/Moriarty boundary matrix and counterexamples, not merely feature recommendations.
- MPLR additions only for distinct required behavior; refinements preserve existing IDs. Each has a positive witness, negative witness, source locator, open PL hypotheses and Midnight obligations.
- Proposed OpenSpec/EARS changes only after source-supported behavior is separated from possible mechanisms. Pel remains a repository implementation workflow, never a public deployment condition.
- Separate labels for collected, read, source-checked, executed and formally proved. No example executions or theorem completion are implied by document ingestion.


## Completed comparative study

The initial intake above is retained as research history. The current bounded workstream is complete; use the [synthesis](synthesis.md) and [verified scope](reference.md) for final findings and remaining implementation obligations.
