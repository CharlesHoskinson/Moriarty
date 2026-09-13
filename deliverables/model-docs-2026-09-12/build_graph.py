#!/usr/bin/env python3
"""Build the frozen documentation graph from four retained semantic fragments.

Run with the interpreter recorded in graphify-out/.graphify_python.
--prepare validates inputs and writes communities for human label review.
The normal run requires matching curated community-labels.json, then exports.
No network requests, model calls, corpus edits, or canonical vault writes occur.
"""
from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.metadata
import io
import json
import os
import re
import subprocess
import sys
from collections import Counter
from contextlib import redirect_stdout
from datetime import datetime, timezone
from pathlib import Path

BASE = Path(__file__).resolve().parent
ROOT = BASE / "corpus"
OUT = BASE / "graphify-out"
SPEC = Path("/home/charl/.agents/skills/graphify/references/extraction-spec.md")
USAGE_NOTE = (
    "Extraction token usage and monetary cost are unavailable: host agent usage "
    "was not exposed. Zero fields in retained upstream chunks are placeholders, "
    "not measured usage; merged/accounting outputs use null."
)
BENCH_NOTE = (
    "This is a token-footprint estimate, not a retrieval-quality evaluation. "
    "Graphify estimates corpus tokens as words*100/75 and retrieved graph tokens "
    "as characters/4. Its depth-3 label-seeded traversal includes node labels, "
    "source locations and edges, but omits rationale text and source passages. "
    "Unmatched queries are omitted by the library; no answers, recall, factual "
    "accuracy or latency were measured."
)


def dump(path: Path, value: object) -> None:
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n")


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def fingerprint(members: list[str]) -> str:
    return hashlib.sha256("\n".join(sorted(members)).encode()).hexdigest()


def frontmatter(path: Path) -> tuple[dict, bytes]:
    m = re.fullmatch(rb"---\n(.*?)\n---\n\n(.*)", path.read_bytes(), re.S)
    if not m:
        raise ValueError(f"Malformed corpus frontmatter: {path}")
    fields = {}
    for line in m[1].decode().splitlines():
        key, value = line.split(":", 1)
        fields[key] = json.loads(value.strip())
    return fields, m[2]


def validate_sources() -> tuple[list[dict], dict[str, dict]]:
    manifest = json.loads((BASE / "MANIFEST.json").read_text())
    records = {r["slug"]: r for r in manifest["records"]}
    assert len(records) == len(manifest["records"]), "Duplicate source records"
    inventory, metadata = [], {}
    for path in sorted(ROOT.glob("*.md")):
        record = records[path.stem]
        raw = BASE / "sources" / record["raw_file"]
        text = BASE / "sources" / record["text_file"]
        header, body = frontmatter(path)
        raw_hash, text_hash = sha(raw), sha(text)
        assert raw_hash == record["sha256"], f"Raw source hash mismatch: {raw}"
        assert text_hash == record["text_sha256"], f"Text source hash mismatch: {text}"
        assert hashlib.sha256(body).hexdigest() == text_hash, f"Wrapper differs: {path}"
        assert header["source_text_sha256"] == text_hash
        assert header["source_url"] == record["canonical_url"]
        assert header["captured_at"] == record["retrieved_at_utc"]
        metadata[str(path)] = header
        inventory.append({
            "source_file": str(path), "source_url": header["source_url"],
            "captured_at": header["captured_at"], "wrapper_sha256": sha(path),
            "raw_sha256": raw_hash, "text_sha256": text_hash,
            "all_hashes_match": True,
        })
    assert set(records) == {Path(r["source_file"]).stem for r in inventory}
    return inventory, metadata


def validate_location(item: dict) -> None:
    loc = item.get("source_location")
    if loc is None:
        return
    numbers = list(map(int, re.findall(r"\d+", str(loc))))
    count = len(Path(item["source_file"]).read_text().split("\n"))
    assert numbers and all(1 <= n <= count for n in numbers), (item, count)


def merge_chunks(metadata: dict[str, dict]) -> tuple[dict, list[dict]]:
    result = {"nodes": [], "edges": [], "hyperedges": [],
              "input_tokens": None, "output_tokens": None, "usage_note": USAGE_NOTE}
    receipts = []
    for number in range(1, 5):
        path = OUT / f".graphify_chunk_{number:02d}.json"
        chunk = json.loads(path.read_text())
        receipts.append({"file": path.name, "sha256": sha(path),
                         "nodes": len(chunk["nodes"]), "edges": len(chunk["edges"]),
                         "input_tokens": None, "output_tokens": None})
        for collection in ("nodes", "edges", "hyperedges"):
            result[collection].extend(chunk.get(collection, []))
    ids = {n["id"] for n in result["nodes"]}
    assert len(ids) == len(result["nodes"]), "Duplicate node IDs require explicit review"
    for node in result["nodes"]:
        assert re.fullmatch(r"[a-z0-9_]+", node["id"])
        assert node["file_type"] in {"code", "document", "paper", "image", "rationale", "concept"}
        sf = node["source_file"]
        assert sf in metadata, sf
        stem = re.sub(r"[^a-z0-9]+", "_", Path(sf).stem.lower())
        assert node["id"].startswith(stem + "_"), node["id"]
        for field in ("source_url", "captured_at", "author", "contributor"):
            assert node.get(field) == metadata[sf].get(field), (node["id"], field)
        validate_location(node)
    for edge in result["edges"]:
        assert edge["source"] in ids and edge["target"] in ids, edge
        assert edge["source_file"] in metadata
        confidence, score = edge["confidence"], edge["confidence_score"]
        assert confidence in {"EXTRACTED", "INFERRED", "AMBIGUOUS"}
        assert (confidence != "EXTRACTED" or score == 1.0)
        assert (confidence != "INFERRED" or score in {.95, .85, .75, .65, .55})
        assert (confidence != "AMBIGUOUS" or .1 <= score <= .3)
        validate_location(edge)
    for hyper in result["hyperedges"]:
        assert len(hyper["nodes"]) >= 3 and set(hyper["nodes"]) <= ids
        assert hyper["source_file"] in metadata
    covered = {n["source_file"] for n in result["nodes"]}
    assert covered == set(metadata), f"Uncovered corpus files: {set(metadata) - covered}"
    return result, receipts


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--prepare", action="store_true")
    args = parser.parse_args()
    os.chdir(BASE)
    from graphify.analyze import god_nodes, surprising_connections, suggest_questions
    from graphify.benchmark import run_benchmark, print_benchmark
    from graphify.build import build_from_json
    from graphify.cache import save_semantic_cache, check_semantic_cache
    from graphify.cli import _stamped_manifest_files
    from graphify.cluster import cluster, score_all
    from graphify.detect import detect, save_manifest
    from graphify.diagnostics import diagnose_extraction, format_diagnostic_report
    from graphify.export import to_json, to_html
    from graphify.report import generate

    inventory, metadata = validate_sources()
    extraction, chunks = merge_chunks(metadata)
    detection = detect(ROOT, cache_root=BASE)
    detected = {f for bucket in detection["files"].values() for f in bucket}
    assert detected == set(metadata), "Detection and explicit corpus differ"
    diagnostics = diagnose_extraction(copy.deepcopy(extraction), directed=False, root=ROOT)
    health_text = format_diagnostic_report(diagnostics)
    graph = build_from_json(copy.deepcopy(extraction), root=ROOT, directed=False)
    assert graph.number_of_nodes() > 0
    assert set(graph.nodes) == {n["id"] for n in extraction["nodes"]}
    import networkx as nx
    isolated = [{"id": n, "label": graph.nodes[n]["label"]} for n in nx.isolates(graph)]
    components = nx.number_connected_components(graph)
    communities = cluster(graph)
    cohesion = score_all(graph, communities)
    review = {}
    for cid, members in communities.items():
        ordered = sorted(members, key=lambda n: (-graph.degree(n), graph.nodes[n]["label"]))
        review[str(cid)] = {"membership_sha256": fingerprint(members), "size": len(members),
                            "labels": [graph.nodes[n]["label"] for n in ordered],
                            "source_files": sorted({graph.nodes[n]["source_file"] for n in members})}
    for name, data in [(".graphify_detect.json", detection),
                       (".graphify_extract.json", extraction),
                       (".graphify_semantic.json", extraction),
                       ("source-validation.json", inventory),
                       ("graph-diagnostics.json", diagnostics),
                       ("community-review.json", review)]:
        dump(OUT / name, data)
    (OUT / "graph-diagnostics.txt").write_text(health_text + "\n")
    if args.prepare:
        print(json.dumps({"prepared": True, "sources": len(inventory),
                          "communities": len(communities), "review": str(OUT / "community-review.json")}))
        return
    curated = json.loads((OUT / "community-labels.json").read_text())
    assert set(curated) == set(review), "Community labels need review"
    for cid in curated:
        assert curated[cid]["membership_sha256"] == review[cid]["membership_sha256"], "Stale labels"
    labels = {int(cid): record["label"] for cid, record in curated.items()}
    gods = god_nodes(graph)
    surprises = surprising_connections(graph, communities)
    questions = suggest_questions(graph, communities, labels)
    graph.graph.update(extraction_usage={"input_tokens": None, "output_tokens": None,
                                         "monetary_cost": None, "note": USAGE_NOTE},
                       source_inventory_sha256=sha(OUT / "source-validation.json"),
                       source_root="../corpus", evidence_scope="Frozen official documentation only")
    assert to_json(graph, communities, str(OUT / "graph.json"), community_labels=labels, built_at_commit="")
    # report.generate requires numbers for formatting; replace before writing any report.
    report = generate(graph, communities, cohesion, labels, gods, surprises, detection,
                      {"input": 0, "output": 0}, str(ROOT), suggested_questions=questions,
                      min_community_size=1)
    report = re.sub(r"^- Token cost:.*$", "- Token cost: unavailable (input, output and money unmeasured).", report, flags=re.M)
    report = re.sub(r"^- Verdict:.*$", f"- Coverage: all {len(inventory)} captured documents represented; navigation quality is not yet evaluated.", report, flags=re.M)
    report += "\n## Evidence and build limitations\n\n" + USAGE_NOTE + "\n\n"
    report += (
        "This navigation graph is undirected; original relation direction remains in the retained extraction fragments. "
        "it is not a cross-provider equivalence or ranking model. No cross-provider edges were added "
        "during merge. Semantic extraction is selective even where sources were read in full. "
        "Document hubs can dominate connectivity, and thin communities are included with raw cohesion. "
        "EXTRACTED means explicit in the captured source, not independently reproduced or guaranteed true. "
        "PDF raster figures were not comprehensively inspected. Source conflicts remain labeled.\n\n"
        "The original chunks, merged extraction, source hash receipt and diagnostics are retained. "
        "The source hash receipt, rather than the surrounding repository commit, pins these documents.\n\n"
        + BENCH_NOTE + "\n\n## Graph integrity diagnostics\n\n```text\n" + health_text + "\n```\n"
    )
    report += f"\nThe graph has {components} connected components and {len(isolated)} isolated node(s): " + ", ".join(n["label"] for n in isolated) + ". No linking evidence was invented to connect them.\n"
    assert "Token cost: 0" not in report
    (OUT / "GRAPH_REPORT.md").write_text(report)
    dump(OUT / ".graphify_labels.json", {str(k): v for k, v in labels.items()})
    dump(OUT / ".graphify_analysis.json", {"communities": communities, "cohesion": cohesion,
                                          "gods": gods, "surprises": surprises, "questions": questions})
    assert to_html(graph, communities, str(OUT / "graph.html"), community_labels=labels)
    # Explicit cache_root keeps derived caches outside the frozen corpus.
    saved = save_semantic_cache(extraction["nodes"], extraction["edges"], extraction["hyperedges"],
                                root=ROOT, cache_root=BASE, allowed_source_files=detected, prompt_file=SPEC)
    cn, ce, ch, misses = check_semantic_cache(sorted(detected), root=ROOT, cache_root=BASE, prompt_file=SPEC)
    assert not misses and {n["id"] for n in cn} == set(graph.nodes), "Cache replay incomplete"
    canonical = lambda records: Counter(json.dumps(r, sort_keys=True) for r in records)
    assert canonical(ce) == canonical(extraction["edges"]), "Cache edge replay differs"
    assert canonical(ch) == canonical(extraction["hyperedges"]), "Cache hyperedge replay differs"
    (OUT / ".graphify_uncached.txt").write_text("")
    stamped = _stamped_manifest_files(detection["files"], extraction, ROOT)
    stamped_set = {f for bucket in stamped.values() for f in bucket}
    save_manifest(stamped, str(OUT / "manifest.json"), kind="semantic", root=ROOT,
                  scan_corpus=detected, clear_semantic=detected - stamped_set or None)
    (OUT / ".graphify_root").write_text(str(ROOT) + "\n")
    (OUT / ".graphify_python").write_text(sys.executable + "\n")
    now = datetime.now(timezone.utc).isoformat()
    cost_path = OUT / "cost.json"
    cost = json.loads(cost_path.read_text()) if cost_path.exists() else {"runs": []}
    extraction_hash = sha(OUT / ".graphify_extract.json")
    if not any(r.get("extraction_sha256") == extraction_hash for r in cost["runs"]):
        cost["runs"].append({"date": now, "input_tokens": None, "output_tokens": None,
                             "monetary_cost": None, "files": len(inventory),
                             "extraction_sha256": extraction_hash, "usage_note": USAGE_NOTE})
    cost.update(total_input_tokens=None, total_output_tokens=None, total_monetary_cost=None, usage_note=USAGE_NOTE)
    dump(cost_path, cost)
    # Native CLI expects .graphify_detect.json in cwd; running from OUT supplies the measured word count.
    native = subprocess.run([sys.executable, "-m", "graphify", "benchmark", str(OUT / "graph.json")],
                            cwd=OUT, text=True, capture_output=True)
    (OUT / "benchmark-native.txt").write_text(native.stdout + native.stderr + "\n" + BENCH_NOTE + "\n")
    assert native.returncode == 0, native.stderr
    print(native.stdout, end="")
    domain_questions = [
        "Claude Opus 5 verification scope effort",
        "Gemini 3.8 Flash thinking tool execution",
        "GPT-6 Astra reasoning effort Responses API",
        "Grok 4.6 reasoning effort tools",
        "CLI model selection permissions schema",
        "Prompt injection safeguards benchmark limitations",
    ]
    benchmark = run_benchmark(str(OUT / "graph.json"), corpus_words=detection["total_words"], questions=domain_questions)
    assert "error" not in benchmark
    matched = {p["question"] for p in benchmark["per_question"]}
    benchmark.update(method_limitations=BENCH_NOTE, requested_questions=domain_questions,
                     unmatched_questions=[q for q in domain_questions if q not in matched],
                     answer_quality_evaluated=False)
    dump(OUT / "benchmark.json", benchmark)
    stream = io.StringIO()
    with redirect_stdout(stream):
        print_benchmark(benchmark)
    (OUT / "benchmark.txt").write_text(stream.getvalue() + BENCH_NOTE + "\n")
    print(stream.getvalue(), end="")
    final = json.loads((OUT / "graph.json").read_text())
    final_ids = {n["id"] for n in final["nodes"]}
    assert all(e["source"] in final_ids and e["target"] in final_ids for e in final["links"])
    assert "<html" in (OUT / "graph.html").read_text().lower()
    assert all(n.get("community_name") for n in final["nodes"])
    after, _ = validate_sources()
    assert after == inventory, "Sources changed during build"
    summary = {"built_at": now, "graphify_version": importlib.metadata.version("graphifyy"),
               "directed": False, "sources": len(inventory), "corpus_words": detection["total_words"],
               "nodes": graph.number_of_nodes(), "raw_edges": len(extraction["edges"]),
               "graph_edges": graph.number_of_edges(), "hyperedges": len(extraction["hyperedges"]),
               "communities": len(communities), "connected_components": components,
               "isolated_nodes": isolated,
               "raw_confidence_counts": dict(Counter(e["confidence"] for e in extraction["edges"])),
               "source_hash_validation": "passed", "endpoint_validation": "passed",
               "cache_files_saved": saved, "cache_files_replayed": len(detected),
               "manifest_files_stamped": len(stamped_set), "chunks": chunks,
               "semantic_prompt_file": str(SPEC), "semantic_prompt_sha256": sha(SPEC),
               "usage_note": USAGE_NOTE, "input_tokens": None, "output_tokens": None,
               "monetary_cost": None, "benchmark_note": BENCH_NOTE,
               "graph_sha256": sha(OUT / "graph.json"), "html_sha256": sha(OUT / "graph.html")}
    dump(OUT / "build-receipt.json", summary)
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
