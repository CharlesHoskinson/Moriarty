#!/home/charl/.local/share/uv/tools/graphifyy/bin/python
"""Rebuild the bounded role-10 graph with the pinned installed Graphify runtime."""
from __future__ import annotations
import hashlib
import importlib.metadata
import json
from pathlib import Path
import sys

EXPECTED_EXE = "/home/charl/.local/share/uv/tools/graphifyy/bin/python"
EXPECTED_VERSION = "0.9.91"
if sys.executable != EXPECTED_EXE:
    raise SystemExit(f"Refusing runtime {sys.executable!r}; expected {EXPECTED_EXE!r}")
version = importlib.metadata.version("graphifyy")
if version != EXPECTED_VERSION:
    raise SystemExit(f"Refusing Graphify {version}; expected {EXPECTED_VERSION}")

from graphify.build import build_from_json
from graphify.cluster import cluster, label_communities_by_hub
from graphify.export import to_html, to_json

ROOT = Path(__file__).resolve().parent
OUT = ROOT / "graphify-out"
SOURCE = ROOT / "MODEL-MAP.json"
OUT.mkdir(parents=True, exist_ok=True)
model = json.loads(SOURCE.read_text(encoding="utf-8"))
if len(model.get("nodes", [])) != 78 or len(model.get("edges", [])) != 74:
    raise SystemExit("Source map row counts changed; inspect before rendering")

def canonical_rows(rows: list[dict]) -> str:
    payload = json.dumps(rows, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return hashlib.sha256(payload).hexdigest()

G = build_from_json(model, directed=True, root=ROOT.parent.parent.parent)
if G.number_of_nodes() != 78 or G.number_of_edges() != 74 or not G.is_directed():
    raise SystemExit(f"Graphify built unexpected graph: {G.number_of_nodes()} nodes, {G.number_of_edges()} edges, directed={G.is_directed()}")
communities = cluster(G)
labels = label_communities_by_hub(G, communities)
graph_path = OUT / "graph.json"
to_json(G, communities, str(graph_path), force=True, community_labels=labels)
to_html(G, communities, str(OUT / "graph.html"), community_labels=labels)

raw = json.loads(graph_path.read_text(encoding="utf-8"))
links = raw.get("links", raw.get("edges", []))
if len(raw.get("nodes", [])) != 78 or len(links) != 74 or not raw.get("directed"):
    raise SystemExit("Exported Graphify graph failed strict count/direction check")
manifest = {
    "tool": "graphifyy",
    "version": version,
    "python": sys.executable,
    "modelMap": str(SOURCE.name),
    "sourceMapSha256": hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
    "nodeRowsSha256CanonicalOrder": canonical_rows(model["nodes"]),
    "edgeRowsSha256CanonicalOrder": canonical_rows(model["edges"]),
    "counts": {
        "sourceNodes": len(model["nodes"]),
        "sourceEdges": len(model["edges"]),
        "graphNodes": G.number_of_nodes(),
        "graphEdges": G.number_of_edges(),
        "clusters": len(communities),
        "weakComponents": __import__("networkx").number_weakly_connected_components(G),
        "proposalNodes": sum(1 for _, data in G.nodes(data=True) if data.get("type") == "proposal"),
    },
    "outputs": {
        "graphJsonSha256": hashlib.sha256(graph_path.read_bytes()).hexdigest(),
        "graphHtmlSha256": hashlib.sha256((OUT / "graph.html").read_bytes()).hexdigest(),
    },
    "scope": "Offline graph construction, deterministic clustering and rendering from the supplied bounded map; no semantic extraction or model/API calls.",
}
(OUT / "MANIFEST.json").write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print(json.dumps(manifest, indent=2, ensure_ascii=False))
