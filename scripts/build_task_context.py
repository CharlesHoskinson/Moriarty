#!/usr/bin/env python3
"""Build bounded author context from explicit sources and a frozen advisory graph.

This command reads text and current Git metadata. It does not launch workers,
tests, reviewers or native work, and it never supplies admission or acceptance.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path, PurePosixPath


SCHEMA = "moriarty.parallel-language-workflow/1"
PACKET_SCHEMA = "moriarty.task-context/1"
MODELS = {"gpt-6.1-sol", "grok-4.7"}
DOC_OVERLAYS = {
    "AGENTS.md", "ROADMAP.md", "docs/FOOTGUNS.md", "docs/PARALLEL-DEVELOPMENT.md",
    "plugins/moriarty-dev/README.md", "plugins/moriarty-dev/skills/develop/SKILL.md",
    "plugins/moriarty-dev/skills/develop/references/execution-focus.md",
    "openspec/changes/consolidated-language-kernel/workflow.md",
}
ROOT_FILES = {"AGENTS.md", "README.md", "ROADMAP.md", "Cargo.toml", "Cargo.lock"}
ROOT_DIRS = {"crates", "docs", "openspec", "plugins", "scripts", "tests", "examples", "packages", "toolchains", "concepts"}
TEXT_SUFFIXES = {".md", ".rs", ".py", ".mori", ".qnt", ".json", ".toml", ".lock", ".ebnf", ".js", ".jsx", ".ts", ".tsx", ".mjs"}
PROTECTED = re.compile(r"actual.?pk|private.?pk|marker|private.?key|prover.?key|keyset|proving.?cache|credentials?|secrets?|password|cookies?|access.?token|wallet|srs|provider|\.env|\.ssh|\.aws", re.I)
PRIVATE_ROOTS = {"private", "credentials", "secrets", "keysets", "srs", "params", "prover-cache", "proving-cache", "provider-cache", "provider-state", ".codex", ".grok", ".claude", ".ssh", ".aws", ".git", "target", "node_modules", "cache", "caches", ".worktrees", ".moriarty-dev"}
FORMAL_ROOT = "experiments/moriarty-language/formal/quint/language/"
GRAPH_FILES = ("graph.json", "coverage.json", "inventory.json", "build-summary.json")
GRAPH_CAP = 128 * 1024 * 1024
SOURCE_CAP = 2 * 1024 * 1024
PACKET_CAP = 256 * 1024


class ContextError(ValueError):
    """A context input is invalid, excluded or stale."""


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def no_symlinks(path):
    for part in (path, *path.parents):
        if part.is_symlink():
            raise ContextError(f"symlink-excluded:{path.name}")


def safe_source(root, relative):
    if not isinstance(relative, str) or not relative or "\\" in relative:
        raise ContextError("invalid-source-path")
    path = PurePosixPath(relative)
    if path.is_absolute() or ".." in path.parts or "." in relative.split("/"):
        raise ContextError(f"outside-repo:{relative}")
    if path.as_posix() != relative.rstrip("/"):
        raise ContextError(f"noncanonical-path:{relative}")
    formal = relative.startswith(FORMAL_ROOT) and (relative.endswith("/") or path.suffix in {".qnt", ".mori", ".md"})
    if path.parts[0] not in ROOT_DIRS and relative not in ROOT_FILES and not formal:
        raise ContextError(f"excluded-scope:{relative}")
    # Public source/docs may discuss wallets, providers, markers or SRS.
    # Private storage roots and sensitive data filenames remain excluded.
    sensitive_data = path.suffix == ".json" and PROTECTED.search(path.name)
    private_material = re.search(r"actual.?pk|private.?pk|private.?key|credentials?|secrets?|password|cookies?|access.?token", path.name, re.I)
    if sensitive_data or private_material or any(p.casefold() in PRIVATE_ROOTS for p in path.parts):
        raise ContextError(f"protected-path:{relative}")
    if not relative.endswith("/") and path.suffix not in TEXT_SUFFIXES:
        raise ContextError(f"binary-or-unsupported-path:{relative}")
    absolute = root / relative
    no_symlinks(absolute)
    if not absolute.resolve().is_relative_to(root.resolve()):
        raise ContextError(f"outside-repo:{relative}")
    return absolute


def read_bytes(path, cap):
    no_symlinks(path)
    if not path.is_file():
        raise ContextError(f"missing-regular-file:{path.name}")
    with path.open("rb") as stream:
        raw = stream.read(cap + 1)
    if len(raw) > cap:
        raise ContextError(f"input-size:{path.name}")
    return raw


def metadata_path(path):
    """Reject provider/private roots before opening an explicit JSON sidecar."""
    if any(PROTECTED.search(part) or part in {".codex", ".grok", ".claude", ".git", "target", "node_modules", "cache", "caches"} for part in path.parts):
        raise ContextError("protected-metadata-path")
    no_symlinks(path)
    return path


def decode_json(raw):
    def pairs(items):
        result = {}
        for key, value in items:
            if key in result:
                raise ContextError(f"duplicate-json-key:{key}")
            result[key] = value
        return result
    try:
        return json.loads(raw.decode("utf-8"), object_pairs_hook=pairs)
    except (UnicodeError, json.JSONDecodeError) as exc:
        raise ContextError(f"invalid-json:{exc}") from exc


def load_graph_bundle(directory):
    metadata_path(directory)
    ready_raw = read_bytes(directory / "graph-ready.json", SOURCE_CAP)
    ready = decode_json(ready_raw)
    if ready.get("ready") is not True or not re.fullmatch(r"[0-9a-f]{40}", ready.get("commit", "")):
        raise ContextError("graph-not-frozen")
    bundle = {"commit": ready["commit"], "digests": {"graph-ready.json": digest(ready_raw)}}
    for name, key in zip(GRAPH_FILES, ("graph", "coverage", "inventory", "summary")):
        raw = read_bytes(directory / name, GRAPH_CAP)
        bound = ready.get("files", {}).get(name, {})
        if bound.get("sha256") != digest(raw) or bound.get("bytes") != len(raw):
            raise ContextError(f"stale-graph-digest:{name}")
        value = decode_json(raw)
        commit = value.get("built_at_commit") if key == "graph" else value.get("commit")
        if commit != ready["commit"]:
            raise ContextError(f"stale-graph-metadata:{name}")
        bundle[key] = value
        bundle["digests"][name] = digest(raw)
    if not bundle["coverage"].get("inventory_complete"):
        raise ContextError("graph-inventory-incomplete")
    graph, summary = bundle["graph"], bundle["summary"]
    if len(graph.get("nodes", [])) != summary.get("nodes") or len(graph.get("links", [])) != summary.get("edges"):
        raise ContextError("stale-graph-counts")
    if read_bytes(directory / "graph-ready.json", SOURCE_CAP) != ready_raw:
        raise ContextError("graph-changed-during-read")
    return bundle


def overlaps(left, right):
    a, b = left.rstrip("/"), right.rstrip("/")
    return a == b or a.startswith(b + "/") or b.startswith(a + "/")


def acyclic(graph):
    active, done = set(), set()
    def visit(node):
        if node in active:
            raise ContextError(f"dependency-cycle:{node}")
        if node in done:
            return
        if node not in graph:
            raise ContextError(f"unknown-dependency:{node}")
        active.add(node)
        for dependency in graph[node]:
            visit(dependency)
        active.remove(node)
        done.add(node)
    for node in graph:
        visit(node)


def validate_manifest(root, manifest):
    if manifest.get("schema") != SCHEMA or not re.fullmatch(r"[0-9a-f]{40}", manifest.get("candidate_base", "")):
        raise ContextError("invalid-manifest-schema-or-base")
    lanes = manifest.get("lanes", [])
    if {x["id"] for x in lanes} != {"S1", "S2", "S3", "G1", "G2", "G3"} or len(lanes) != 6:
        raise ContextError("six-unique-lanes-required")
    owners = []
    shared = manifest.get("shared_files", [])
    for path in shared:
        safe_source(root, path)
    for lane in lanes:
        expected = "gpt-6.1-sol" if lane["id"].startswith("S") else "grok-4.7"
        if lane["author"] != {"model": expected, "effort": "high"}:
            raise ContextError(f"invalid-author-routing:{lane['id']}")
        if not lane.get("write_set") or not lane.get("objective") or not lane.get("verification"):
            raise ContextError(f"incomplete-task:{lane['id']}")
        for path in lane["write_set"]:
            safe_source(root, path)
            if any(overlaps(path, other) for other in shared):
                raise ContextError(f"shared-owner-collision:{lane['id']}:{path}")
            for other_lane, other in owners:
                if overlaps(path, other):
                    raise ContextError(f"write-overlap:{lane['id']}:{other_lane}:{path}")
            owners.append((lane["id"], path))
        for entry in manifest.get("requirements", []) + lane.get("inputs", []):
            safe_source(root, entry["path"])
        for check in lane["verification"]:
            if not isinstance(check.get("argv"), list) or not all(isinstance(v, str) for v in check["argv"]):
                raise ContextError(f"invalid-verification:{lane['id']}")
            for path in check.get("requires_paths", []):
                safe_source(root, path)
    for path in manifest.get("closure_relevant_inputs", []):
        safe_source(root, path)
    acyclic({x["id"]: x.get("source_dependencies", []) for x in lanes})
    acyclic(manifest.get("acceptance_graph", {}))
    for lane in lanes:
        if any(x not in manifest["acceptance_graph"] for x in lane.get("acceptance_dependencies", [])):
            raise ContextError(f"unknown-acceptance-dependency:{lane['id']}")
    routing = manifest.get("review_round1", {})
    if set(routing) != {x["id"] for x in lanes}:
        raise ContextError("incomplete-review-routing")
    for lane in lanes:
        reviewers = routing[lane["id"]]
        if len(reviewers) != 2 or lane["id"] in reviewers or len(set(reviewers)) != 2:
            raise ContextError("author-self-audit-or-missing-reviewer")
        models = {next((x["author"]["model"] for x in lanes if x["id"] == r), None) for r in reviewers}
        if models != MODELS:
            raise ContextError("both-review-providers-required")
    integrated = manifest.get("integration_reviewers", [])
    if not isinstance(integrated, list) or len(integrated) != 2:
        raise ContextError("integration-review-pair-required")
    authors = {x["id"] for x in lanes} | {manifest["shared_owner"]}
    expected_roles = {"R-S": "gpt-6.1-sol", "R-G": "grok-4.7"}
    if {x.get("id") for x in integrated} != set(expected_roles):
        raise ContextError("integration-review-seats-must-be-outside-author-pool")
    for reviewer in integrated:
        if (reviewer["id"] in authors
                or reviewer.get("model") != expected_roles[reviewer["id"]]
                or reviewer.get("effort") != "high"
                or reviewer.get("role") != "integration-reviewer"
                or reviewer.get("read_only") is not True
                or reviewer.get("implementation_assignments") != []
                or reviewer.get("authored_current_composed_change") is not False):
            raise ContextError("integration-review-must-be-fresh-readonly-non-author")


def reviewers(manifest, lane_id, round_number):
    if round_number < 1:
        raise ContextError("invalid-review-round")
    result = []
    for initial in manifest["review_round1"][lane_id]:
        pool = [prefix + str(i) for i in range(1, 4) for prefix in [initial[0]]]
        if initial[0] == lane_id[0]:
            pool.remove(lane_id)
        result.append(pool[(pool.index(initial) + round_number - 1) % len(pool)])
    return result


def assemble(root, manifest, bundle, lane_id, current_commit, round_number=1):
    inventory = {x["path"]: x for x in bundle["inventory"]["tracked_files"]}
    # Refuse graph-protected inputs before path/stat/symlink checks or reads.
    declared = [x["path"] for x in manifest.get("requirements", [])]
    declared += manifest.get("shared_files", [])
    declared += manifest.get("closure_relevant_inputs", [])
    for task in manifest.get("lanes", []):
        declared += [x["path"] for x in task.get("inputs", [])]
        declared += task.get("write_set", [])
        declared += [p for check in task.get("verification", []) for p in check.get("requires_paths", [])]
    for path in declared:
        if inventory.get(path, {}).get("coverage_class") == "protected_excluded":
            raise ContextError(f"excluded-graph-coverage:{path}")
    validate_manifest(root, manifest)
    if current_commit != bundle["commit"] or manifest["candidate_base"] != bundle["commit"]:
        raise ContextError("stale-graph-commit:refresh-graph-and-candidate-base")
    lane = next((x for x in manifest["lanes"] if x["id"] == lane_id), None)
    if lane is None:
        raise ContextError(f"unknown-lane:{lane_id}")
    nodes = bundle["graph"]["nodes"]
    nodes_by_file = {}
    for node in nodes:
        nodes_by_file.setdefault(node.get("source_file"), []).append(node)
    sources, content = {}, {}

    def source(path, role, lines=None, inline=True, missing_ok=False):
        if path in sources:
            return sources[path]
        row = inventory.get(path)
        if row and row.get("coverage_class") == "protected_excluded":
            raise ContextError(f"excluded-graph-coverage:{path}")
        absolute = safe_source(root, path)
        if not absolute.exists():
            if not missing_ok:
                raise ContextError(f"missing-source:{path}")
            entry = {"path": path, "role": role, "sha256": None, "graph_layer": "not-created", "graph_freshness": "not-created", "content_status": "specified-deliverable-only"}
            sources[path] = entry
            return entry
        explicit_formal = path.startswith(FORMAL_ROOT) and Path(path).suffix in {".qnt", ".mori", ".md"}
        formal_archive = explicit_formal and row and row.get("coverage_class") == "archival_inventory_only"
        if row and row.get("coverage_class") != "candidate_content" and not formal_archive:
            raise ContextError(f"excluded-graph-coverage:{path}")
        raw = read_bytes(absolute, SOURCE_CAP)
        try:
            text = raw.decode("utf-8")
        except UnicodeError as exc:
            raise ContextError(f"binary-source:{path}") from exc
        if "\0" in text:
            raise ContextError(f"binary-source:{path}")
        oid = hashlib.sha1(b"blob " + str(len(raw)).encode() + b"\0" + raw).hexdigest()
        fresh = bool(row and oid == row.get("blob_oid"))
        if not fresh and path not in DOC_OVERLAYS:
            raise ContextError(f"stale-graph-source:{path}:refresh-graph")
        file_nodes = nodes_by_file.get(path, []) if fresh and row.get("coverage_class") == "candidate_content" else []
        origins = {n.get("_origin") for n in file_nodes}
        layer = "ast" if "ast" in origins else "prose-anchor" if any(n.get("node_type") in {"document_excerpt", "heading"} for n in file_nodes) else "inventory-only-direct-source"
        entry = {"path": path, "role": role, "sha256": digest(raw), "bytes": len(raw), "git_blob_oid": row.get("blob_oid") if row else None,
                 "coverage_class": row.get("coverage_class") if row else "explicit-new-workflow-document",
                 "graph_layer": layer, "graph_freshness": "matches-frozen-blob" if fresh else "current-doc-overlay-graph-hints-omitted", "writable": path in lane["write_set"]}
        if inline:
            start, end = lines or [1, 40]
            if not isinstance(start, int) or not isinstance(end, int) or start < 1 or end < start or end - start >= 160:
                raise ContextError(f"invalid-excerpt-range:{path}")
            split = text.splitlines()
            entry["excerpt"] = {"start_line": start, "end_line": min(end, len(split)), "text": "\n".join(split[start - 1:end]), "qualification": "partial-author-context-read-complete-source-before-review"}
        sources[path] = entry
        content[path] = raw
        return entry

    for entry in manifest.get("requirements", []) + lane.get("inputs", []):
        source(entry["path"], entry["role"], entry.get("lines"))
    for path in lane["write_set"]:
        if not path.endswith("/"):
            source(path, "owned-deliverable", missing_ok=True)
    closure = []
    for path in manifest.get("closure_relevant_inputs", []):
        entry = source(path, "closure-relevant-input", inline=False)
        closure.append({"path": path, "sha256": entry["sha256"], "graph_freshness": entry["graph_freshness"]})
    selected = {n["id"] for path, entry in sources.items() if entry["graph_freshness"] == "matches-frozen-blob" for n in nodes_by_file.get(path, [])}
    by_id = {n["id"]: n for n in nodes}
    hints, omissions = [], set()
    allowed_relations = {"calls", "imports_from", "imports", "re_exports", "references", "mentions_repository_path", "declares_reference"}
    for edge in bundle["graph"]["links"]:
        if edge.get("relation") not in allowed_relations or not ({edge.get("source"), edge.get("target")} & selected):
            continue
        neighbor = edge.get("target") if edge.get("source") in selected else edge.get("source")
        node = by_id.get(neighbor, {})
        path = node.get("source_file")
        row = inventory.get(path)
        if not path or not row or row.get("coverage_class") != "candidate_content":
            omissions.add("unresolved-or-excluded-inventory-neighbor")
            continue
        try:
            safe_source(root, path)
        except ContextError:
            omissions.add("protected-or-unsupported-neighbor")
            continue
        if path not in sources or sources[path]["graph_freshness"] != "matches-frozen-blob":
            omissions.add("outside-explicit-current-source-set")
            continue
        hints.append({"id": neighbor, "label": node.get("label"), "source_file": path, "source_location": node.get("source_location"),
                      "relation": edge.get("relation"), "edge_source_file": edge.get("source_file"), "edge_source_location": edge.get("source_location"), "confidence": node.get("confidence"), "qualification": "structural-pointer-verify-line-in-current-source"})
    hints = sorted({json.dumps(x, sort_keys=True): x for x in hints}.values(), key=lambda x: (x["source_file"], x["id"], x["relation"]))
    tool_sha256 = digest(read_bytes(Path(__file__), SOURCE_CAP))
    snapshot = {"manifest_sha256": digest(canonical(manifest)), "context_tool_sha256": tool_sha256, "current_commit": current_commit, "lane": lane_id, "review_round": round_number,
                "sources": {path: entry["sha256"] for path, entry in sources.items()}, "graph_digests": bundle["digests"]}
    verification = []
    for check in lane["verification"]:
        missing = [p for p in check.get("requires_paths", []) if not safe_source(root, p).is_file()]
        verification.append({**check, "current_applicability": "specified-only-missing-inputs" if missing else "inputs-present-command-unrun", "missing_inputs": missing})
    packet = {"schema": PACKET_SCHEMA, "lane": lane_id, "objective": lane["objective"], "milestones": lane["milestones"], "candidate_base": manifest["candidate_base"], "current_commit": current_commit, "context_tool_sha256": tool_sha256,
              "routing": {"author_requested": lane["author"], "author_returned": "unobserved", "review_round": round_number, "reviewers": reviewers(manifest, lane_id, round_number), "rule": "fresh-non-author-Sol-and-Grok-full-candidate-audits"},
              "integration_reviewers": manifest["integration_reviewers"],
              "integration_review_status": "planned-unobserved-fresh-sessions-outside-all-authors",
              "write_set": lane["write_set"], "shared_owner": manifest["shared_owner"], "source_set": list(sources.values()), "closure_relevant_inputs": closure,
              "closure_qualification": "selected-closure-relevant-inputs-not-complete-native-source-closure-certification",
              "source_dependencies": lane.get("source_dependencies", []), "acceptance_dependencies": lane.get("acceptance_dependencies", []), "dependency_blockers": lane.get("acceptance_blockers", []),
              "verification": verification, "native_gate": {"status": "not-admitted-by-context", "frozen_signed_call": "retain-existing-Opus5.5-and-Muse-audits"},
              "graph": {"qualification": "advisory-scoped-snapshot", "commit": bundle["commit"], "nodes": len(nodes), "links": len(bundle["graph"]["links"]), "digests": bundle["digests"], "neighbors": hints[:32], "neighbor_count_before_cap": len(hints), "coverage": bundle["coverage"], "omitted_categories": sorted(omissions)},
              "freshness": {"scope": "explicit-current-inputs-and-qualified-graph-hints-only", "whole_dirty_tree_certified": False, "snapshot_sha256": digest(canonical(snapshot))},
              "review_input": "This author context does not replace complete frozen source, applicable contracts, independent failures or retained actual-result bytes."}
    if len(canonical(packet)) > PACKET_CAP:
        raise ContextError("packet-size:reduce-explicit-excerpts")
    for path, raw in content.items():
        if read_bytes(safe_source(root, path), SOURCE_CAP) != raw:
            raise ContextError(f"source-changed-during-read:{path}")
    return packet


def check_context(previous, current):
    if previous.get("schema") != PACKET_SCHEMA or canonical(previous) != canonical(current):
        raise ContextError("stale-context:regenerate-before-use")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--manifest", type=Path)
    parser.add_argument("--graph-dir", type=Path, required=True)
    parser.add_argument("--lane", required=True)
    parser.add_argument("--review-round", type=int, default=1)
    parser.add_argument("--check-context", type=Path)
    args = parser.parse_args()
    try:
        no_symlinks(args.repo.absolute())
        root = args.repo.resolve()
        manifest_path = args.manifest or root / "openspec/parallel-language-workflow.json"
        if manifest_path.name != "parallel-language-workflow.json" or PROTECTED.search(manifest_path.name):
            raise ContextError("invalid-manifest-path")
        manifest = decode_json(read_bytes(metadata_path(manifest_path), SOURCE_CAP))
        result = subprocess.run(["git", "rev-parse", "HEAD"], cwd=root, check=True, capture_output=True, text=True, timeout=10)
        commit = result.stdout.strip()
        bundle = load_graph_bundle(args.graph_dir)
        packet = assemble(root, manifest, bundle, args.lane, commit, args.review_round)
        packet["manifest_path"] = str(manifest_path)
        packet["graph"]["path"] = str(args.graph_dir)
        if args.check_context:
            if args.check_context.suffix != ".json" or PROTECTED.search(args.check_context.name):
                raise ContextError("invalid-context-path")
            check_context(decode_json(read_bytes(metadata_path(args.check_context), PACKET_CAP)), packet)
        final_commit = subprocess.run(["git", "rev-parse", "HEAD"], cwd=root, check=True, capture_output=True, text=True, timeout=10).stdout.strip()
        if final_commit != commit:
            raise ContextError("commit-changed-during-read")
        print(json.dumps(packet, sort_keys=True, indent=2, ensure_ascii=False))
        return 0
    except (ValueError, OSError, KeyError, TypeError, AttributeError, StopIteration, subprocess.SubprocessError) as exc:
        print(json.dumps({"status": "context-unavailable", "error": str(exc), "acceptance": "not-evaluated"}), file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
