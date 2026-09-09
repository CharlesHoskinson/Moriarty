# Reactive semantics, effects and artifact identity

Six complete primary PDFs support the comparisons below. These are historical research sources and S2 design recommendations. No source result establishes Moriarty acceptance, performance, native proofs, or source/Core/K/ledger correspondence.

## Comparative atlas

| Source | Useful distinction | Mandatory limit |
|---|---|---|
| [S01](https://people.seas.harvard.edu/~chong/pubs/pldi13-elm.pdf) | Use immutable state transitions and explicitly identified inputs for tooling. Keep async results as proposals until authority and ledger checks accept them. | Neither first-stage normalization nor responsiveness establishes finite lifetime execution, finite queues, observation truth, quantitative conservation, or ledger acceptance. |
| [S03](https://arxiv.org/pdf/1903.05879) | Treat temporal availability as an independent typing concern. Define a bounded observation record with identity, effective time, expiry and authority before considering reactive syntax. | Productivity describes potentially infinite execution. Garbage collection does not establish a uniform byte/time bound; explicit state can grow. Fairness extensions are future work. These results say nothing about financial liveness or mandatory proofs. |
| [S04](https://eelcovisser.org/publications/2004/DolstraJV04.pdf) | Use separate source, Core, build and output identities. Include semantic profile, dependency closure and toolchain in artifact manifests. Authorize mutable financial migration separately from changing a code name. | An input-derived identifier assumes adequately captured inputs and appropriate determinism. It is not an independently checked output digest, semantic-equivalence proof, authorization, or migration theorem. The paper’s old hash examples are historical, not a recommended modern construction. |
| [S05](https://arxiv.org/pdf/1406.2061) | Give Moriarty primitive effects explicit typing and operational rules; close admitted effect sets before execution. Keep effect summaries separate from authority quantities and ordered ledger effects. | Termination alone does not give a registered work bound. Effect rows are not payment counts, rights, or affine continuation rules. This 2014 source does not establish the behavior of current Koka or arbitrary user-defined handlers. |
| [S06](https://www.microsoft.com/en-us/research/wp-content/uploads/2018/03/build-systems.pdf) | Cache each proof/compile stage by its actual immediate inputs and output identity; record semantic/compiler changes as dependencies. Specify an invalidation test for changed profile, kernel, observation policy and verifier lineage. | These are simplified executable models, not a certification of contemporary build products. Correct build outputs need not implement correct financial semantics. Shallow storage and nondeterminism change the correctness predicate. |
| [S07](https://elm-lang.org/assets/papers/concurrent-frp.pdf) | Prefer an approachable functional surface over a small explicit Core, with stable diagnostic/source mappings. Keep preview recomputation separate from accepted state and residual obligations. | Historical syntax and runtime claims do not specify current Elm. The thesis and PLDI paper belong to the same research line, so they are not independent replications. Embedding/dynamic-switching discussion does not authorize dynamic or unbounded financial execution. |

## S01: Asynchronous Functional Reactive Programming for GUIs

Evan Czaplicki and Stephen Chong. 2013. [Asynchronous Functional Reactive Programming for GUIs](https://people.seas.harvard.edu/~chong/pubs/pldi13-elm.pdf). Locators: §§3.1–3.3, pp.3–7; §4.3 and §5, pp.9–10.

**Source fact.** FElm separates normalization of functional expressions from ongoing signal execution. Its stratified types exclude signals of signals. Theorem 1 establishes type soundness and normalization of the first stage. The pipelined signal semantics uses unbounded FIFO queues; async preserves order inside a subgraph while relaxing global order between subgraphs. The implementation section explicitly omits direct performance evaluations.

**Moriarty recommendation.** Use immutable state transitions and explicitly identified inputs for tooling. Keep async results as proposals until authority and ledger checks accept them.

**Limits and inference.** Neither first-stage normalization nor responsiveness establishes finite lifetime execution, finite queues, observation truth, quantitative conservation, or ledger acceptance.

## S03: Simply RaTT

Patrick Bahr, Christian Uldal Graulund and Rasmus Ejlers Møgelberg. arXiv:1903.05879v2, 2019-06-11. [Simply RaTT](https://arxiv.org/pdf/1903.05879). Locators: §§2.1–2.3, pp.6–9; §§3.2–3.4, pp.12–15; Theorems 3.1/3.2 and 6.3, Proposition 6.4, pp.13–14,23–25; §8 p.26.

**Source fact.** Simply RaTT uses later and stable modalities with context restrictions, guarded recursion and a two-heap machine. For closed well-typed streams over value types built from unit, Nat, sums and products, Theorem 3.1 gives arbitrarily long finite productive prefixes. Theorem 3.2 gives typed stream-transducer progress; its proof establishes causality. The machine can discard the previous heap. The type system rejects a particular fixed-point-under-delay time leak.

**Moriarty recommendation.** Treat temporal availability as an independent typing concern. Define a bounded observation record with identity, effective time, expiry and authority before considering reactive syntax.

**Limits and inference.** Productivity describes potentially infinite execution. Garbage collection does not establish a uniform byte/time bound; explicit state can grow. Fairness extensions are future work. These results say nothing about financial liveness or mandatory proofs.

## S04: Nix: A Safe and Policy-Free System for Software Deployment

Eelco Dolstra, Merijn de Jonge and Eelco Visser. LISA XVIII, 2004, pp.79–92. [Nix: A Safe and Policy-Free System for Software Deployment](https://eelcovisser.org/publications/2004/DolstraJV04.pdf). Locators: Overview/Nix Store pp.81–83; Implementation/The Store and Building Components pp.84–85; User Environment Policies pp.87–88; Conclusion p.91 (PDF pages subtract 78).

**Source fact.** Nix separates user-facing names from immutable installation objects, identifies variants through hashes of build inputs, lowers high-level expressions to simpler store expressions, and deploys dependency closures. Build inputs include scripts, platform and environment bindings. The paper describes atomic environment changes through POSIX rename and retains old generations. Mutable service state is explicitly outside Nix control.

**Moriarty recommendation.** Use separate source, Core, build and output identities. Include semantic profile, dependency closure and toolchain in artifact manifests. Authorize mutable financial migration separately from changing a code name.

**Limits and inference.** An input-derived identifier assumes adequately captured inputs and appropriate determinism. It is not an independently checked output digest, semantic-equivalence proof, authorization, or migration theorem. The paper’s old hash examples are historical, not a recommended modern construction.

## S05: Koka: Programming with Row Polymorphic Effect Types

Daan Leijen. EPTCS 153, 2014, pp.100–126. [Koka: Programming with Row Polymorphic Effect Types](https://arxiv.org/pdf/1406.2061). Locators: §§2.1–2.7 pp.102–107; §5.2 p.117; §6 Theorems 2–4 pp.118–119 (PDF pages subtract 99).

**Source fact.** Koka uses strict evaluation and inferred row-polymorphic effects with duplicate labels. In its formal calculus, absence of exn excludes an unhandled exception result, not internally caught exceptions; absence of div entails termination. Heap encapsulation prevents local references escaping under its typing side conditions. The formal read primitive conservatively includes divergence; §2.7 distinguishes a finer implementation analysis and an incompletely implemented constraint case.

**Moriarty recommendation.** Give Moriarty primitive effects explicit typing and operational rules; close admitted effect sets before execution. Keep effect summaries separate from authority quantities and ordered ledger effects.

**Limits and inference.** Termination alone does not give a registered work bound. Effect rows are not payment counts, rights, or affine continuation rules. This 2014 source does not establish the behavior of current Koka or arbitrary user-defined handlers.

## S06: Build Systems a la Carte

Andrey Mokhov, Neil Mitchell and Simon Peyton Jones. PACMPL 2(ICFP), Article 79, 2018. [Build Systems a la Carte](https://www.microsoft.com/en-us/research/wp-content/uploads/2018/03/build-systems.pdf). Locators: Definition 2.1 p.3; Definition 3.1 p.11; §§4.2–4.4 pp.14–16; §§6.3–6.6 pp.23–26.

**Source fact.** The framework separates scheduling from rebuilding. Correctness preserves reachable inputs and requires every reachable computed key to agree with recomputation against the final store, assuming acyclic tasks. Verifying traces store hashes; constructive traces also store results. Deep traces omit intermediate dependencies and require determinism. The §6.4 Frankenbuild example combines nondeterministic intermediates with deep caching to yield inconsistent artifacts.

**Moriarty recommendation.** Cache each proof/compile stage by its actual immediate inputs and output identity; record semantic/compiler changes as dependencies. Specify an invalidation test for changed profile, kernel, observation policy and verifier lineage.

**Limits and inference.** These are simplified executable models, not a certification of contemporary build products. Correct build outputs need not implement correct financial semantics. Shallow storage and nondeterminism change the correctness predicate.

## S07: Elm: Concurrent FRP for Functional GUIs

Evan Czaplicki. Senior thesis, Harvard, 2012-03-30. [Elm: Concurrent FRP for Functional GUIs](https://elm-lang.org/assets/papers/concurrent-frp.pdf). Locators: Ch.3 pp.13–21; Ch.4 pp.22–31; Ch.6 pp.39–40; Ch.7 p.41 (PDF pages add 3).

**Source fact.** The thesis explains discrete signals, lift and foldp, a two-tier intermediate representation, and explicit async subgraphs. Let-bound signal representations prevent duplicate runtime nodes. Its examples separate presentation from reactive processing. Chapter 6 records concrete JavaScript backend limitations; Chapter 7 presents filtering and further reactive machinery as future work.

**Moriarty recommendation.** Prefer an approachable functional surface over a small explicit Core, with stable diagnostic/source mappings. Keep preview recomputation separate from accepted state and residual obligations.

**Limits and inference.** Historical syntax and runtime claims do not specify current Elm. The thesis and PLDI paper belong to the same research line, so they are not independent replications. Embedding/dynamic-switching discussion does not authorize dynamic or unbounded financial execution.

## Proposed semantic boundary

A reactive environment may supply a candidate observation or requested action. A bounded, deterministic Core transition prepares state, effects, remaining duties, residual authority and remaining work. A separate acceptance relation checks authenticated authority, admissible observations, predecessor consumption, all four mandatory claims, native verification and exact ledger effects. Reactive arrival and render completion are not acceptance events. This is a proposal constrained by the existing [language design](../defi-language-design-2026-09-07/LANGUAGE-DESIGN.md).

For a partial loan payment, a preview can update immediately when a quote arrives. Accepted progress must still allocate payment according to the agreement, retain unpaid principal and fees, reduce remaining authority/work, and persist predecessor consumption. Neither a pure update function nor a causal stream makes a quoted price true or pays the creditor.

A future handler design must address invocation multiplicity explicitly. A handler that can resume a continuation repeatedly must not thereby duplicate a transfer, owned resource, nonce or residual work. This is a required design question, not a result established by the 2014 Koka paper. Keep the finance-bearing operation set closed and first-order unless a separately reviewed resource argument admits more.

## Identity is a family of commitments

| Identity | Proposed committed content | Does not establish |
|---|---|---|
| Source identity | Exact UTF-8 bytes | Meaning, authorization or correct elaboration |
| Core identity | Canonical typed Core, semantic profile and encoding version | Source/Core correspondence |
| Build identity | Input closure, compiler/kernel/toolchain, flags, target and semantic dependencies | That returned output was honestly built |
| Artifact identity | Exact artifact bytes, distinct from build recipe | Correctness of those bytes |
| Execution statement | Program/profile, signed authority digest, predecessor, observations, complete effects, duties, successors and remaining work | Acceptance before native and ledger checks |

These are proposed domains, not a finalized encoding. A human name can map to a content identifier for authoring convenience, but a signed financial request must bind the permitted identities or an explicit upgrade policy. A code change cannot silently reinterpret a previously authorized obligation. Financial state migration needs its own conservation, authority and residual-duty rules.

## Specified-only validation cases

1. Supply the same bounded observation sequence in different delivery schedules. Compare accepted order, full effects and residuals under the explicit ordering policy; UI update timing may differ.
2. Present two asynchronous results for the same consumed predecessor. Exactly one may accept if the agreement permits only one consumption.
3. Generate inputs faster than computation completes. Check the defined queue/admission bound and failure result; do not infer a bound from a fixed graph.
4. Keep a stream productive while its explicit accumulator grows. The financial profile must reject growth beyond its registered size/work bound.
5. Change a compiler, canonicalization rule, semantic profile or immediate intermediate artifact. A cache entry with the old commitment must not establish the new result.
6. Preserve source bytes while changing only a human alias; preserve meaning under formatting while changing source bytes. Verify that each identity domain behaves according to its own contract.
7. Partially pay a loan, then expire or cancel a continuation. Residual duties must remain represented or be resolved by separately authorized rules.
8. Attempt multiple resumptions of a finance-bearing continuation. Require explicit rejection or separately proved resource-safe behavior; effects cannot replenish spent allowance.

All eight cases are proposals. No execution results are claimed.

## Coverage and provenance

The immutable source directory is [raw semantics](../../raw/sources/language-design-2026-09-09/semantics/). Each PDF has a capture receipt with requested/canonical URL, UTC retrieval time, HTTP status and SHA-256. [Reading manifest](../../raw/sources/language-design-2026-09-09/semantics/reading-manifest.json) adds page count, version, reviewed locators and extraction limits. [Typed claims](semantics-claims.json) distinguish source facts, recommendations and limiting inferences. The six PDFs contain 157 pages in total.

Coverage is substantive full-text source review at the named locators, rather than abstract-only evidence; it is not an independent check of every proof/appendix. PDF extraction loses some modal glyphs and diagram structure. In Simply RaTT, use the arXiv v2 date on the first page, not the manuscript’s stale template footer. Build Systems uses the 2018 paper; its later journal expansion is not included. The Elm thesis and paper are related versions and count as two documents, not two independent confirmations.

Elliott’s push-pull author host failed DNS resolution during the robots request. The [failure receipt](../../raw/sources/language-design-2026-09-09/semantics/S02-access-failure.json) is retained; that work is not counted as read. The 2012 Elm thesis supplies the sixth accessible primary source. Current Elm and Unison behavior requires the separately captured official documentation; historical FRP claims are not transferred to current Elm.
