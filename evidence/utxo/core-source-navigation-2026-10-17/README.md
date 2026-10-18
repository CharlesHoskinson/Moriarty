# Moriarty source navigation

Two frozen maps make the compiler and draft image-bridge source easier to inspect:

- [Backend graph](backend/packet/graph.json): 68 nodes, 64 edges, 16 pinned sources. [Source map guide](backend/packet/README.md).
- [Caller graph](caller/packet/graph.json): 72 nodes, 78 edges. [Guide and repaired attributions](caller/packet/GRAPH_REPORT.md). [Luna attribution review](caller/LUNA-CALLER-V2-ATTRIBUTION.json).

Both maps received fresh nonauthor reviews through AGY with Gemini 4 Argon High requested. Each returned one complete review reporting that the source attributions were ready. The exported metadata reports the requested name but lacks typed assistant-model fields; exclusive actual model identity is not established. Complete reviews, frozen source bodies and [root qualifications](ROOT-QUALIFICATIONS.json) are included.

The caller map corrects module attribution, the proposed-profile method boundary, the public profile re-export chain and separate attribute/declaration citations. Test declarations and compatible types are labelled separately from runtime caller claims.

These maps describe their pinned source snapshots. They establish no executed compiler or bridge behavior, recursion proof, security approval or Cardano transaction. Runtime, security, operator and ledger readiness remain false. Native authority is zero.
