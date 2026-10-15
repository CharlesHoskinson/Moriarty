## Verdict: PASS

The top finding is that the V11 candidate is mechanically sound as a scoped, nonsecurity, exact-union graph integration, and its reports preserve the required attribution, chronology, evidence-class, and open-scope limits.

This PASS does not constitute source adoption, security, cryptographic, formal, financial, target-qualification, runtime, or product acceptance.

## Concrete findings

1. **Exact merge implementation — correct.**  
   [build_v11_integration.py](/home/charl/moriarty-eutxo-validation-2026-10-12/luna-graph-v11-integration-20261015/build_v11_integration.py) verifies that the Luna overlay’s `edges` and `links` are the identical collection and counts that collection only once. It then concatenates the frozen parent links with the 15 overlay edges, rather than deduplicating them. The resulting raw counts are 6,057 + 22 = 6,079 nodes, 16,790 + 15 = 16,805 links, and 1 + 0 = 1 hyperedge. It checks duplicate node IDs, duplicate explicit edge and hyperedge IDs, edge endpoints, hyperedge members, and final equality of raw links and hyperedges. The supplied root byte/union check reports exact preservation of all parent and new non-community payloads.

2. **Multiplicity and hyperedge preservation — correct, with analysis-only collapse disclosed.**  
   The raw JSON retains the reported 20 identical duplicate link records and five self-loops. The NetworkX directed analysis topology collapses 626 same-pair records, giving 16,179 analysis edges, but that collapse is not applied to the raw JSON. The sole hyperedge remains in the completed JSON. Community assignment is the only intended node-field overwrite, and the builder compares non-community payloads after stripping `community` and `community_name`.

3. **HTML limitation — correctly disclosed.**  
   The 6,079-node graph exceeds the 5,000-node HTML limit. The HTML is therefore community-aggregated into 165 community nodes, and intra-community hyperedge display may be omitted. [GRAPH_REPORT-v11.md](/home/charl/moriarty-eutxo-validation-2026-10-12/luna-graph-v11-integration-20261015/GRAPH_REPORT-v11.md) and [RUN.md](/home/charl/moriarty-eutxo-validation-2026-10-12/luna-graph-v11-integration-20261015/RUN.md) both state this limitation. Communities, cohesion values, and god-node rankings are presented as navigation only, not as semantic approval.

4. **Attribution and evidence class — correctly bounded at package level.**  
   All 22 Luna nodes and 15 edges carry a staged `source_file` and `source_sha256`, and the builder checks those references against the frozen input pins. The important statements remain source-attributed: the Opus ABI1 review and root adoption support only legacy local UNIT evidence; compiled v0 C0 and settle0 must never be funded; no ABI1 proof or C1 issue-acceptance path was observed; and SD-1 remains open. The P3 material remains uncompiled source with authored or predicted 106 Aiken and 18 Rust tests, nine first-vote mechanism texts, and no implemented authority or settlement acceptance. A non-blocking labeling caution is that some overlay records sourced from metadata-only PROCESS or manifest records carry the generic overlay field `semantic_review_performed: true`; the governing coverage receipt and Luna report nevertheless correctly classify those inputs as partial or metadata-only, and their node statements use only the corresponding metadata facts. They must not be upgraded to full semantic source review.

5. **Coverage separation — accurate.**  
   V11 covers 26 frozen records totalling 212,724 bytes: 10 full text reads through EOF, 10 full structural JSON reads, and six partial structural or metadata-only reads. The integrator byte-hashed the staged bodies and performed no fresh semantic extraction. Provider bodies and private content were not read, and the 25 ledger-packet blobs were not opened; only their manifest metadata is represented. Earlier V7, V8, V9, and V10 coverage remains separately embedded and is not relabelled as V11 coverage.

6. **Chronology and unfinished work — not overstated.**  
   All three relevant Opus jobs are represented as terminal provider-quota failures with no report and no verdict. Their P02/H45 work remains partial and non-final, the Groth16 source-fix DELTA remains unfinished and does not close R01/R04, and the catalogue manifest check remains partial rather than a complete audit. The “resets 12pm America/Denver” message is a historical provider message, not a current liveness claim. The Sherlock 35-clause registration question remains unresolved at the freeze, and no later Sol repair or guard result is imported. The V10 Muse PASS and published graph commit `e092` remain limited to nonsecurity navigation.

7. **Open scope, identity, and usage remain open or unknown.**  
   The reports retain as open all 20 pattern families, the general compiler, U/M/MC/SP/G, PCD, private and multiparent composition, physical settlement, and actual Preprod qualification. Preprod material remains unauthenticated provider observation. Luna was requested as `gpt-6-luna` at high effort, but its returned identity is unavailable. Integration was configured as `gpt-6.1-sol` at high effort. Token usage and monetary cost are null or unknown, not zero.

## Coverage of this audit

**Full inline coverage:** the changed V11 builder, RUN instructions, graph report, mechanical check, coverage receipt, manifest, Luna overlay containing all 22 nodes and 15 edges, Luna coverage records, freeze record, root byte/union check, and the 26 frozen cited sources supplied inline.

**Partial coverage:** the parent V10 full graph, the V11 output full graph, raw HTML content, and complete topology were excluded as large artifacts. For those, this audit relies on the supplied root byte/union check and makes no independent hash, execution, or runtime claim.

No private provider body was read, and no later live status, source acceptance, or target judgment was imported.