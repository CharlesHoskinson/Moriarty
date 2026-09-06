# Graph Report - intents-2026-09-06  (2026-09-06)

## Corpus Check
- Corpus is ~10,494 words - fits in a single context window. You may not need a graph.

## Summary
- 78 nodes · 134 edges · 7 communities
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 1 edges (avg confidence: 0.75)
- Token cost: **unmeasured**. Host-agent usage is unavailable; extraction-schema zeros are compatibility placeholders, not zero usage or billing.

## Community Hubs (Navigation)
- Checked Adapters and Acceptance
- Intent Goals and Authority
- Canonical Plans and Settlement
- Persistent Authority and Composition
- Bounded Language and Libraries
- Evidence and Financial Types
- Asynchronous Claims and Recovery

## God Nodes (most connected - your core abstractions)
1. `Bounded authority` - 11 edges
2. `Canonical outcome Intent IR` - 11 edges
3. `Plan acceptance judgment` - 10 edges
4. `Typed settlement adapter` - 9 edges
5. `Solver Plan IR` - 9 edges
6. `Asynchronous lifecycle state machine` - 9 edges
7. `Intent` - 8 edges
8. `Residual capability budget` - 8 edges
9. `Execution or lifecycle receipt` - 7 edges
10. `Compiler and adapter trace refinement` - 7 edges

## Surprising Connections (you probably didn't know these)
- `Residual capability budget` --semantically_similar_to--> `Persistent financial obligation`  [INFERRED] [semantically similar]
  intents.md → intents.md  _Bridges community 6 → community 3_
- `Intent` --references--> `Declared environmental assumption`  [EXTRACTED]
  intents.md → intents.md  _Bridges community 1 → community 3_
- `Intent` --references--> `Asynchronous lifecycle state machine`  [EXTRACTED]
  intents.md → intents.md  _Bridges community 1 → community 6_
- `Intent` --conceptually_related_to--> `Permission`  [EXTRACTED]
  intents.md → intents.md  _Bridges community 1 → community 4_
- `Concrete solver plan` --references--> `Plan acceptance judgment`  [EXTRACTED]
  intents.md → intents.md  _Bridges community 4 → community 0_

## Hyperedges (group relationships)
- **Joint plan acceptance conditions** — raw_reports_intents_2026_09_06_intents_authority, raw_reports_intents_2026_09_06_intents_safety, raw_reports_intents_2026_09_06_intents_goal, raw_reports_intents_2026_09_06_intents_assumption, raw_reports_intents_2026_09_06_intents_replay, raw_reports_intents_2026_09_06_intents_verification [EXTRACTED 1.00]
- **Persistent authority and claims across asynchronous progress** — raw_reports_intents_2026_09_06_intents_commitment, raw_reports_intents_2026_09_06_intents_residual, raw_reports_intents_2026_09_06_intents_obligation, raw_reports_intents_2026_09_06_intents_async, raw_reports_intents_2026_09_06_intents_goal_progress [EXTRACTED 1.00]
- **Distinct artifacts across authorization and settlement** — raw_reports_intents_2026_09_06_intents_intent, raw_reports_intents_2026_09_06_intents_permission, raw_reports_intents_2026_09_06_intents_plan, raw_reports_intents_2026_09_06_intents_execution, raw_reports_intents_2026_09_06_intents_receipt [EXTRACTED 1.00]

## Communities (7 total, 0 thin omitted)

### Community 0 - "Checked Adapters and Acceptance"
Cohesion: 0.19
Nodes (14): Typed settlement adapter, Adversarial conformance suite, EVM account and execution adapters, Semantic extension governance, Feasibility and anti-vacuity, Partial-fill lifecycle, Reported IKL checker prototype, Compiler and adapter trace refinement (+6 more)

### Community 1 - "Intent Goals and Authority"
Cohesion: 0.18
Nodes (14): Anoma resource semantics, Bounded authority, Hard outcome goal, Progress versus terminal fulfillment, Gross debit and prefix exposure, Intent, Explicit liabilities and claims, Solver preference (+6 more)

### Community 2 - "Canonical Plans and Settlement"
Cohesion: 0.22
Nodes (14): Canonical semantic encoding, Source-to-backend compiler pipeline, Asset conservation, Complete authority-consuming effects, ERC-7683 solver interface, Quote-first exact-plan profile, Canonical outcome Intent IR, Time-indexed liquidity (+6 more)

### Community 3 - "Persistent Authority and Composition"
Cohesion: 0.22
Nodes (11): Declared environmental assumption, Linearized authority home, Alternative branch authority, Resource commitment or lock, Intent composition operators, Delegated authority non-amplification, Extended UTxO portability, Conditional settlement liveness (+3 more)

### Community 4 - "Bounded Language and Libraries"
Cohesion: 0.20
Nodes (11): Withdrawn four-primitive basis, Bounded deterministic verification, CAKE reference architecture, DeFiFormal coverage corpus, Backend execution, Held-out financial expressivity test, Versioned financial libraries, Permission (+3 more)

### Community 5 - "Evidence and Financial Types"
Cohesion: 0.29
Nodes (7): Domain and issuer-qualified asset, External economic truth boundary, Provenanced observation, Solver optimality boundary, Distinct evidence classes, Explicit rounded arithmetic, Financial units and quantities

### Community 6 - "Asynchronous Claims and Recovery"
Cohesion: 0.38
Nodes (7): Asynchronous lifecycle state machine, Compensation versus rollback, ERC-7540 asynchronous vault, Finality and recovery race policy, Marlowe temporal financial semantics, Persistent financial obligation, Execution or lifecycle receipt

## Knowledge Gaps
- **19 isolated node(s):** `Intents-first language report`, `Feasibility and anti-vacuity`, `Time and validity domain`, `Resource separation and noninterference`, `Alternative branch authority` (+14 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 19 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Bounded authority` connect `Intent Goals and Authority` to `Checked Adapters and Acceptance`, `Canonical Plans and Settlement`, `Persistent Authority and Composition`?**
  _High betweenness centrality (0.190) - this node is a cross-community bridge._
- **Why does `Typed settlement adapter` connect `Checked Adapters and Acceptance` to `Canonical Plans and Settlement`, `Bounded Language and Libraries`?**
  _High betweenness centrality (0.165) - this node is a cross-community bridge._
- **Why does `Plan acceptance judgment` connect `Checked Adapters and Acceptance` to `Intent Goals and Authority`, `Canonical Plans and Settlement`, `Persistent Authority and Composition`, `Bounded Language and Libraries`, `Asynchronous Claims and Recovery`?**
  _High betweenness centrality (0.147) - this node is a cross-community bridge._
- **What connects `Intents-first language report`, `Feasibility and anti-vacuity`, `Time and validity domain` to the rest of the system?**
  _19 weakly-connected nodes found - possible documentation gaps or missing edges._
## Evidence boundary

EXTRACTED means explicit in the supplied report, not independently verified.
The report uses opaque citations. Its IKL name, prototype results, backend priorities
and selective-proof recommendations do not become Moriarty requirements.
The graph does not establish recursively compliant PCD histories or implementation correctness.

The canonical extraction preserves 3 hyperedges; the interactive view uses the
78 nodes and 134 undirected pairwise edges. Health checks found no missing or
dangling endpoints, self-loops or collapsed edges. Graphify diagnostic fields
called `unverified_node_count` concern extraction structure, not scientific verification.

## Reproduction and limitations

Graphify 0.9.53 was used with the installed 0.9.48 skill after inspecting current APIs.
No tool upgrade occurred. The preserved extraction, detector output, manifest,
semantic cache and provenance receipt permit review. The HTML uses the pinned
remote vis-network script from unpkg with subresource integrity; it is not fully offline.
Token-reduction benchmark results are heuristic context-size estimates, not
measured billing savings, correctness or answer quality.

## Query checks and benchmark

The saved query reaches gross debit, bounded authority, proof-carrying plans and
exact-plan signing with source locators. Graphify returned 54 nodes/102 edges and
estimated 5,729 tokens despite a requested 2,500-token budget; that CLI budget
was not a strict cap. `query-paths.json` preserves narrower concept paths.

The built-in benchmark used the detected 10,494-word corpus and estimated
13,992 naive tokens versus 2,729 query tokens (5.1x). Its single generated
question was “what is the main entry point”, which is poorly matched to this
research report. These figures are heuristic sizes, not measured billing
savings or retrieval-quality evidence. See `benchmark.txt` and its receipt.

Chromium rendered 78 nodes and 134 edges at 1440×1000. Searching “gross debit”
returned the expected concept, with zero page errors or failed requests.
The screenshot and browser request log are preserved. Semantic cache roundtrip
recovered all 78 nodes, 134 edges and 3 hyperedges with no uncached sources.
