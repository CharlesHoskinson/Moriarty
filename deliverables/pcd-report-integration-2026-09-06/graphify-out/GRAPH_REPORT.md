# Source and scope notice

This graph extracts the supplied secondary report, not verified facts. EXTRACTED means explicit in the report. Backend corrections and adopted decisions are in ../README.md and the linked reconciliation. Extraction token usage is **unmeasured**; any zero token fields below are schema placeholders, not measured cost.

# Graph Report - pcd-2026-09-06  (2026-09-06)

## Corpus Check
- Corpus is ~10,174 words - fits in a single context window. You may not need a graph.

## Summary
- 73 nodes · 110 edges · 8 communities
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 1 edges (avg confidence: 0.75)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Intent and Statement Binding
- History and Witness Handoff
- Enforcement and Proof Economics
- Freshness and Certificate Reuse
- Scope of Proof Claims
- Mandatory Claim Policy
- Midnight and Bounded Computation
- Accumulation and Folding Research

## God Nodes (most connected - your core abstractions)
1. `Proof-Carrying Transaction Claim Envelope` - 13 edges
2. `Proof-carrying data` - 10 edges
3. `Statement binding` - 10 edges
4. `Mandatory safety claim` - 7 edges
5. `Current-state applicability` - 7 edges
6. `Signed intent` - 6 edges
7. `Execution correctness` - 6 edges
8. `Semantic acceptance theorem` - 6 edges
9. `Semantic property composition` - 5 edges
10. `Verification cost budget` - 5 edges

## Surprising Connections (you probably didn't know these)
- `Proof-Carrying Transaction Claim Envelope` --references--> `Canonical encoding and domain separation`  [EXTRACTED]
  PCD.md → PCD.md  _Bridges community 0 → community 5_
- `Proof-Carrying Transaction Claim Envelope` --references--> `Authenticated dependency footprint`  [EXTRACTED]
  PCD.md → PCD.md  _Bridges community 0 → community 3_
- `Proof-Carrying Transaction Claim Envelope` --references--> `Execution correctness`  [EXTRACTED]
  PCD.md → PCD.md  _Bridges community 0 → community 4_
- `Proof-Carrying Transaction Claim Envelope` --references--> `Application invariant claim`  [EXTRACTED]
  PCD.md → PCD.md  _Bridges community 0 → community 2_
- `Proof-Carrying Transaction Claim Envelope` --references--> `Private authorization and eligibility`  [EXTRACTED]
  PCD.md → PCD.md  _Bridges community 0 → community 6_

## Hyperedges (group relationships)
- **Joint semantic acceptance premises** — raw_reports_pcd_2026_09_06_pcd_proof_soundness, raw_reports_pcd_2026_09_06_pcd_statement_binding, raw_reports_pcd_2026_09_06_pcd_specification, raw_reports_pcd_2026_09_06_pcd_freshness, raw_reports_pcd_2026_09_06_pcd_external_truth [EXTRACTED 1.00]
- **Private-witness branch and join experiment** — raw_reports_pcd_2026_09_06_pcd_dag_prototype, raw_reports_pcd_2026_09_06_pcd_witness_handoff, raw_reports_pcd_2026_09_06_pcd_property_composition, raw_reports_pcd_2026_09_06_pcd_compliance_predicate [EXTRACTED 1.00]
- **Two-stage authorization and evidence binding** — raw_reports_pcd_2026_09_06_pcd_transaction_core, raw_reports_pcd_2026_09_06_pcd_signed_intent, raw_reports_pcd_2026_09_06_pcd_signed_claim_root, raw_reports_pcd_2026_09_06_pcd_evidence_sidecar [EXTRACTED 1.00]

## Communities (8 total, 0 thin omitted)

### Community 0 - "Intent and Statement Binding"
Cohesion: 0.18
Nodes (18): Committed program identity, Effect constraints, Authenticated evidence sidecar, Adversarial binding mutation suite, Proof-carrying transaction, Proof-Carrying Transaction Claim Envelope, Proof-system soundness, Proof-Carrying Transactions report (+10 more)

### Community 1 - "History and Witness Handoff"
Cohesion: 0.19
Nodes (15): Cardano extended UTxO, History compliance predicate, Multi-party provenance prototype, Consensus finality, Global uniqueness and ordering, Halo2 proof technology, Holography accumulation, Multi-folding PCD (+7 more)

### Community 2 - "Enforcement and Proof Economics"
Cohesion: 0.18
Nodes (11): Execution accelerator prototype, Data and witness availability, End-to-end proof economics, EIP-8025 optional execution proofs, Independent verifier implementations, Application invariant claim, Ordinary ledger acceptance, Network-wide inductive invariant (+3 more)

### Community 3 - "Freshness and Certificate Reuse"
Cohesion: 0.38
Nodes (7): Authenticated-fragment proof, Conditional transition proof, Authenticated dependency footprint, Reusable formal certificate, Current-state applicability, Guard and path conditions, Theorem-Carrying Transactions

### Community 4 - "Scope of Proof Claims"
Cohesion: 0.29
Nodes (7): Compiler and circuit correspondence, Conditional receipt assumptions, Execution correctness, Execution-specific property, Mina proof-authorized updates and chain recursion, Program-wide property, RISC Zero receipts

### Community 5 - "Mandatory Claim Policy"
Cohesion: 0.47
Nodes (6): Canonical encoding and domain separation, Claim dependency graph, Mandatory safety claim, Validity interval, Verification cost budget, Verifier authority and version

### Community 6 - "Midnight and Bounded Computation"
Cohesion: 0.40
Nodes (5): Compact bounded source language, External-world truth assumptions, Midnight proof-bearing transactions, Private authorization and eligibility, Abstract resource bounds

### Community 7 - "Accumulation and Folding Research"
Cohesion: 0.50
Nodes (4): PCD from accumulation, Incrementally verifiable computation, Neo and SuperNeo, Nova folding

## Knowledge Gaps
- **14 isolated node(s):** `Proof-Carrying Transactions report`, `Program-wide property`, `Simulation-extractable PCD`, `Neo and SuperNeo`, `SP1 proof stack` (+9 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 14 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Proof-Carrying Transaction Claim Envelope` connect `Intent and Statement Binding` to `Enforcement and Proof Economics`, `Freshness and Certificate Reuse`, `Scope of Proof Claims`, `Mandatory Claim Policy`, `Midnight and Bounded Computation`?**
  _High betweenness centrality (0.330) - this node is a cross-community bridge._
- **Why does `Statement binding` connect `Intent and Statement Binding` to `History and Witness Handoff`, `Mandatory Claim Policy`?**
  _High betweenness centrality (0.321) - this node is a cross-community bridge._
- **Why does `Semantic property composition` connect `History and Witness Handoff` to `Scope of Proof Claims`?**
  _High betweenness centrality (0.249) - this node is a cross-community bridge._
- **What connects `Proof-Carrying Transactions report`, `Program-wide property`, `Simulation-extractable PCD` to the rest of the system?**
  _14 weakly-connected nodes found - possible documentation gaps or missing edges._