#!/usr/bin/env python3
"""Render knowledge-focus.html from knowledge-focus.json without Graphify.build."""
from pathlib import Path
import collections
import json
import networkx as nx
from graphify.exporters.html import to_html

out = Path(__file__).parent / "graphify-out"
x = json.loads((out / "knowledge-focus.json").read_text())
G = nx.DiGraph()
communities = collections.defaultdict(list)
for n in x["nodes"]:
    d = dict(n)
    i = d.pop("id")
    G.add_node(i, **d)
    communities[d.get("community", 6164)].append(i)
for e in x.get("links") or x.get("edges") or []:
    d = dict(e)
    a = d.pop("source")
    b = d.pop("target")
    G.add_edge(a, b, **d)
to_html(G, dict(communities), str(out / "knowledge-focus.html"), node_limit=7000)
print("Focused experiment view", G.number_of_nodes(), "nodes", G.number_of_edges(), "edges")
