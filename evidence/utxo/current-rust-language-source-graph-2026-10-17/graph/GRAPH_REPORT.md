# Core language source graph v2: complete node attribution

This v2 candidate adds exact parent-table anchors to the 37 `PROFILE_LABEL` nodes and 30 `CITED_SOURCE_IDENTITY` nodes that lacked per-node anchors in the prior graph. The stop record `moriarty-c2-opus55-restored-20261017/CORE-LANGUAGE-GRAPH-MECHANICAL-ANCHOR-STOP.json` identified those 67 nodes. The earlier report/check statement that every node assertion had an anchor was mechanically incorrect; this candidate corrects that metadata defect and preserves the prior semantic assertions and labels.

## Inputs and verification

- Frozen base: `core-language-source-graph/graph.json`, SHA-256 `a5c57c89eb544e9033890674847da8bcd99dce1d653ccb6221bd0e7cff8262d2`.
- Parent table: `ADMISSION-SOURCE-TABLE.json`, SHA-256 `23a186d5f142239eab03fe6239ef188799f3fda539b2c356803a57cd5e3ed706`.
- Acceptance and inventory input hashes were also checked: `033bce67c7c4b3fc92394374f754b6c881cb55884553eddbf92c38bc0e8334c8` and `8fde592bfc941b1743da9427a88223a1689df2fa3bb33772adfddfbbfcd23997`.
- Missing-node fixes: 37 profile labels each point to the first matching literal `/claims/i/profile` field; 30 cited-source identity nodes each point to the first matching `/claims/i/citations/j` object whose exact `inputFile` and cited-source SHA-256 literals match that node. The new node-level `source_sha256` is the parent table hash; the cited body digest remains in `source_identity_sha256_literal`. No cited body path was followed.
- Graphify `0.9.92` at `/home/charl/.local/share/uv/tools/graphifyy/bin/python` ran `build_from_json(..., directed=True)` with default `dedup=True`, then `to_json(..., original_links=...)` and `to_html`. All 289 node IDs, 481 directed edge identities, existing node/edge fields, and new anchors were retained. Every node and edge anchor matched its exact parent document, JSON pointer, physical line range, hash and excerpt across the three pinned inputs.

## Preserved scope

The graph still represents exactly 61 claim objects, 100 claim citations, 20 `UNLOCATED_IN_THIS_PACKET` families and 20 family citations. No driver34, lexer53 or Sherlock26 notes were added. This is an attribution metadata correction only; it makes no semantic, runtime, security, proof, financial, UTXO or ledger approval. Native authority remains 0 and the full goal remains open. V2 is unreviewed and unpublished; a fresh nonauthor graph-attribution review is required.
