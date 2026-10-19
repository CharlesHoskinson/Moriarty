# Small Graphify queries for the current Sherlock priority

Start with one known node and one relation. When the existing Graphify MCP service is available, use `get_node({"node_id":"sherlock::stagenet_recursive_rollup_priority_20261019"})`, then `get_neighbors({"node_id":"sherlock::stagenet_recursive_rollup_priority_20261019","relation_filter":"priority_document","token_budget":500})`. `relation_filter` matches the relation label as a case-insensitive substring. The proposed priority ID becomes usable only after Root installs the final additive proposal.

For real code callers/callees, the installed CLI supports:

```sh
/home/charl/.local/share/uv/tools/graphifyy/bin/python -m graphify query "Sherlock proof call path" --seed sherlock::crates_mori_backend_src_sherlock_machine_literal_bridge_verify_literal_transition --context call --budget 700 --graph /home/charl/Moriarty/graphify-out/graph.json
```

`--context call` filters edge **context** values; it is not a free-form relation-name filter. It can omit the handwritten priority/document edges because they are semantic indexing relations rather than call edges. For those use `get_neighbors` with `relation_filter`, or query the specific document/priority seed without `--context`. CLI traversal depth is fixed at2; MCP query_graph exposes a depth argument. CLI token budget is soft and may return a complete answer above budget.

`get_node` returns source_file/source_location and a compact node description. Resolve new Sherlock source rows through `graphify-out/SHERLOCK-SOURCE-RESOLUTION.json` and verify the exact file hash in `SHERLOCK-SOURCE-MANIFEST.json` before reading a broad closure. The historical main repository root remains unchanged. Do not treat get_node output as a complete dump of every provenance attribute.

The controlling goal is Sherlock deployment on Midnight stagenet with a validated recursive proof, followed by a synthetic Fenrir-derived rollup experiment measuring degree k and performance. A saved literal k18 proof bundle is scoped preceding evidence. Recursive relation acceptance, independent verification, ledger acceptance and stagenet settlement need their own observations. This note is usage/intent, not a result or liveness claim.
