#!/home/charl/.local/share/uv/tools/graphifyy/bin/python
"""Rebuild and verify this bounded 8174 source-navigation graph offline."""
from __future__ import annotations
import hashlib, importlib.metadata, json, sys
from pathlib import Path

EXPECTED_PYTHON = "/home/charl/.local/share/uv/tools/graphifyy/bin/python"
EXPECTED_VERSION = "0.9.91"
if sys.executable != EXPECTED_PYTHON:
    raise SystemExit(f"Refusing Python {sys.executable}; expected {EXPECTED_PYTHON}")
version = importlib.metadata.version("graphifyy")
if version != EXPECTED_VERSION:
    raise SystemExit(f"Refusing Graphify {version}; expected {EXPECTED_VERSION}")

from graphify.build import build_from_json
from graphify.cluster import cluster, label_communities_by_hub
from graphify.export import to_html, to_json
import networkx as nx

ROOT = Path(__file__).resolve().parent
BASE = ROOT.parent
OUT = ROOT / "graphify-out"
MAP_PATH = ROOT / "MODEL-MAP.json"
EXPECTED = {
    "current-rust-upstream-8174-source-reconciliation-sol-20261017/RECONCILIATION.md": "e06cdf7d4163301bc7ff8019382fec5c3c5b913a6e71731c7e3fdec089c5ead1",
    "current-rust-upstream-8174-source-reconciliation-sol-20261017/MACHINE-DELTA.json": "765af1f7ad8d91c5cec24dcc1dd81a662c11e990ebd1e9ffea10ea11a84506bc",
    "current-rust-upstream-8174-source-reconciliation-sol-20261017/SOURCE-MANIFEST.json": "5068187a41a0c1687254d83198f74169a05e562bb3a47db28b4239b2ff73cdbf",
    "current-rust-upstream-8174-source-reconciliation-sol-20261017/REPORT.json": "19b807fa4282ea2e6d15ee19118781fc07687c1fd70169e1d6048ce08435e8d1",
    "current-rust-upstream-8174-root-check-20261017/RESULT.json": "e5848e94f6623a06f6675fa78dde1c9315fc5dbe52fd29b4420a11e3736c7024",
    "current-rust-upstream-8174-fresh-muse-source-review-20261017/MODEL-REVIEW.json": "026e670ec6dc7fa2491fa5720efb7521c4fa1a06f5b0ab221fbb601e25cd2b72",
    "current-rust-upstream-8174-fresh-muse-source-review-20261017/PROCESS.json": "76b3cdbff11470c578c01a7ae9d81f69f07768c015e0a9f01c512609befbd7e4",
}

def sha_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()
def canonical_rows(rows: list[dict]) -> str:
    return sha_bytes(json.dumps(rows, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode())
def resolve_pointer(obj, pointer: str):
    if pointer == "": return obj
    if not pointer.startswith("/"): raise ValueError(f"invalid JSON Pointer: {pointer}")
    cur = obj
    for raw in pointer[1:].split("/"):
        token = raw.replace("~1", "/").replace("~0", "~")
        cur = cur[int(token)] if isinstance(cur, list) else cur[token]
    return cur

source_hashes = {}
loaded = {}
for rel, expected in EXPECTED.items():
    path = BASE / rel
    raw = path.read_bytes()
    actual = sha_bytes(raw)
    if actual != expected: raise SystemExit(f"source hash mismatch: {rel}: {actual}")
    source_hashes[rel] = actual
    if path.suffix == ".json": loaded[rel] = json.loads(raw)
model = json.loads(MAP_PATH.read_text(encoding="utf-8"))
if len(model.get("nodes", [])) > 40 or len(model.get("edges", [])) > 60:
    raise SystemExit("Map exceeds the assigned 40-node/60-edge bound")
node_ids = {n["id"] for n in model["nodes"]}
for kind in ("nodes", "edges"):
    for i, row in enumerate(model[kind]):
        if row.get("confidence") not in {"EXTRACTED", "INFERRED"}:
            raise SystemExit(f"{kind}[{i}] missing EXTRACTED/INFERRED confidence")
        refs = row.get("citations") or []
        if not refs: raise SystemExit(f"{kind}[{i}] has no source citation")
        for ref in refs:
            rel = ref["inputFile"]
            if rel not in EXPECTED: raise SystemExit(f"citation points outside named input set: {rel}")
            if "pointer" in ref: resolve_pointer(loaded[rel], ref["pointer"])
            if "lines" in ref:
                rawlines = (BASE / rel).read_text(encoding="utf-8").splitlines()
                start, end = ref["lines"]
                if start < 1 or end < start or end > len(rawlines):
                    raise SystemExit(f"invalid line citation {rel}:{start}-{end}")
        if kind == "edges" and (row.get("source") not in node_ids or row.get("target") not in node_ids):
            raise SystemExit(f"dangling edge in {kind}[{i}]")

OUT.mkdir(parents=True, exist_ok=True)
G = build_from_json(model, directed=True, root=BASE)
expected_nodes, expected_edges = len(model["nodes"]), len(model["edges"])
if G.number_of_nodes() != expected_nodes or G.number_of_edges() != expected_edges or not G.is_directed():
    raise SystemExit(f"unexpected Graphify graph: {G.number_of_nodes()} nodes, {G.number_of_edges()} edges, directed={G.is_directed()}")
communities = cluster(G)
labels = label_communities_by_hub(G, communities)
graph_path, html_path = OUT / "graph.json", OUT / "graph.html"
to_json(G, communities, str(graph_path), force=True, community_labels=labels)
to_html(G, communities, str(html_path), community_labels=labels)
exported = json.loads(graph_path.read_text(encoding="utf-8"))
export_edges = exported.get("links", exported.get("edges", []))
if len(exported.get("nodes", [])) != expected_nodes or len(export_edges) != expected_edges:
    raise SystemExit("Graphify export row counts do not match the cited source map")
manifest = {
    "format": "moriarty-source-navigation-graphify-manifest/1",
    "tool": "graphifyy", "version": version, "python": sys.executable,
    "sourceRepository": "https://github.com/CharlesHoskinson/Moriarty",
    "sourceHead": model["sourceHead"],
    "scope": "Offline build/cluster/JSON export/HTML render of supplied cited map only; no semantic extraction or model/API calls.",
    "sourceFiles": source_hashes,
    "modelMapSha256": sha_bytes(MAP_PATH.read_bytes()),
    "canonicalNodeRowsSha256": canonical_rows(model["nodes"]),
    "canonicalEdgeRowsSha256": canonical_rows(model["edges"]),
    "counts": {"nodes": G.number_of_nodes(), "edges": G.number_of_edges(), "clusters": len(communities), "weakComponents": nx.number_weakly_connected_components(G), "openNodes": sum(n.get("status") == "OPEN" for n in model["nodes"])},
    "outputs": {"graphJsonSha256": sha_bytes(graph_path.read_bytes()), "graphHtmlSha256": sha_bytes(html_path.read_bytes())},
}
(OUT / "MANIFEST.json").write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print(json.dumps(manifest, indent=2, ensure_ascii=False))
