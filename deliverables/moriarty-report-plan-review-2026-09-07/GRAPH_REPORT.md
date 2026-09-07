# Combined report graph

The directed graph contains 512 nodes and 1114 edges, preserving 1116 source statements. Nine report hyperedges remain in the lossless extraction. Two parallel edge statements are preserved in each rendered edge's `statements` list rather than silently dropped.

## Coverage and limits

All three reports were read in full: intents 2,517 lines, PCD 1,151 lines, DeFi 1,225 lines. Their reviews contain section-by-section coverage. The corpus is 28,342 words and fits within a large context window; the graph is used for persistent relationships and traceability, not because full-text reading is impossible. It is a semantic map, not a lossless representation of every sentence. Original report bytes remain available.

The graph includes all reported concepts extracted by the three reviewers and selected current-code/evidence links, not a full repository AST graph. Report claims, current observations and planning recommendations carry separate evidence-kind metadata. Opaque source citations and missing attachments remain unverified.

## Central concepts

Graphify calls high-connectivity concepts "god nodes". Degree and centrality measure graph structure, not truth or priority.
- {"id": "raw_reports_unified_2026_09_07_defi_rebuilding_the_defi_kernel_report", "label": "Rebuilding the DeFi Kernel report", "degree": 175}
- {"id": "raw_reports_unified_2026_09_07_intents_report", "label": "Designing an Intents-First Language for Composable DeFi", "degree": 140}
- {"id": "raw_reports_unified_2026_09_07_pcd_report", "label": "Proof-Carrying Transactions report", "degree": 106}
- {"id": "raw_reports_unified_2026_09_07_intents_authority", "label": "Bounded authority", "degree": 19}
- {"id": "plan_mc01", "label": "MC01 bounded language", "degree": 16}
- {"id": "raw_reports_unified_2026_09_07_intents_intent", "label": "Signed intent", "degree": 15}
- {"id": "raw_reports_unified_2026_09_07_intents_conformance", "label": "Required semantic and mutation conformance tests", "degree": 15}
- {"id": "plan_mc03", "label": "MC03 native recursive proof", "degree": 15}

## Connections worth following

- Liability authority links the intents report's signed permission to create debt to the DeFi report's authority-safety requirement. Current transfer debit budgets do not close this gap.
- PCD predecessor/policy compatibility links to assume-guarantee composition. Verifying two unrelated proofs is insufficient to compose financial obligations.
- Model fidelity links both financial reports to compiler/ledger correspondence. A certificate over an inaccurate model does not certify the deployed behavior.

## Suggested questions

- Which artifacts must a private successor receive to extend Midnight history without inheriting predecessor secrets?
- How do signed nominal-debt limits survive refinance, partial settlement and split/join?
- Which financial behaviors force a new bounded source/IR profile, and which fit a verified library?

## Community cohesion

Scores below are raw structural density values. A two-node community can score 1.0; that is not independent corroboration.

| Community | Nodes | Cohesion |
|---|---:|---:|
| DeFi sources and architecture | 119 | 0.01680672268907563 |
| Moriarty execution plan | 48 | 0.12056737588652482 |
| Authority and intent composition | 37 | 0.0960960960960961 |
| Proof systems and freshness | 33 | 0.10606060606060606 |
| Canonical intent and signing | 28 | 0.1746031746031746 |
| Capabilities and recurring payments | 16 | 0.18333333333333332 |
| Bounded plans and claims | 16 | 0.175 |
| Financial refinement and assumptions | 16 | 0.15 |
| Signed effects and execution | 15 | 0.20952380952380953 |
| Financial challenge products | 13 | 0.15384615384615385 |
| Mandatory claim envelopes | 12 | 0.2878787878787879 |
| Ledger currentness and invariants | 12 | 0.25757575757575757 |
| Economic function taxonomy | 11 | 0.18181818181818182 |
| Private branching PCD | 11 | 0.34545454545454546 |
| Certificate judgments | 10 | 0.2 |
| Typed semantic effects | 10 | 0.2 |
| Assurance and trust boundaries | 10 | 0.26666666666666666 |
| Temporal obligation workflows | 8 | 0.39285714285714285 |
| Proof cost and acceleration | 8 | 0.32142857142857145 |
| Environment and message composition | 7 | 0.2857142857142857 |
| Financial refinement and assumptions | 7 | 0.42857142857142855 |
| Authority and intent composition | 7 | 0.2857142857142857 |
| Assurance and trust boundaries | 7 | 0.42857142857142855 |
| Semantic kernel layers | 5 | 0.4 |
| Authority and intent composition | 5 | 0.4 |
| Source fidelity validation | 4 | 0.5 |
| Liability preservation | 4 | 0.5 |
| Exact financial arithmetic | 3 | 0.6666666666666666 |
| Report migration proposals | 3 | 0.6666666666666666 |
| Authority safety | 2 | 1.0 |
| Attestation truth boundary | 2 | 1.0 |
| Shared-state settlement | 2 | 1.0 |
| Structural composition | 2 | 1.0 |
| Conditional economic guarantees | 2 | 1.0 |
| Corpus identity normalization | 2 | 1.0 |
| Observational equivalence | 2 | 1.0 |
| Evidence maturity | 2 | 1.0 |
| Finite domain disposition | 2 | 1.0 |
| Taxonomy annotation | 2 | 1.0 |
| State footprint isolation | 2 | 1.0 |
| Kernel complexity | 2 | 1.0 |
| Interface accounting | 2 | 1.0 |
| Moriarty execution plan | 1 | 1.0 |

## Reproduction and usage

Rebuild with the installed Graphify Python environment:

```sh
/home/charl/.local/share/uv/tools/graphifyy/bin/python3 deliverables/moriarty-report-plan-review-2026-09-07/build-graph.py
```

The build consumes retained extraction JSON and the current package register; it does not rerun semantic agents or validate outside claims. Source snapshots and extraction fragments are retained separately. Host-agent token usage and dollar cost are unavailable. Zero token fields in extraction fragments are schema placeholders, not measured zero usage. [The query-size benchmark](benchmark.json) is an estimate of retrieved graph text, not actual token savings, answer quality or extraction cost.
