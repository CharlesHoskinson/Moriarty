"""Read-only, bounded Graphify pointers for the Cardano workflow selector.

The graph is navigation data. Neither freshness nor a graph label establishes a
proof result, ledger acceptance, resource authority, or a live workflow loop.
"""
from __future__ import annotations

import hashlib
import importlib.util
import json
import os
from pathlib import Path
from typing import Mapping


_CHECKOUT = Path(__file__).resolve().parents[3]
_NAV_SPEC = importlib.util.spec_from_file_location(
    "_cardano_graph_nav", _CHECKOUT / "scripts" / "graph-nav.py"
)
_NAV = importlib.util.module_from_spec(_NAV_SPEC)
_NAV_SPEC.loader.exec_module(_NAV)

# Exact existing source paths; no fuzzy label, presumed new node, or copied graph.
_SOURCES = (
    "docs/utxo/graphs/current-rust-groth16-first-proof-2026-10-16/README.md",
    "experiments/cardano-preprod-host/c2_issue_artifacts.py",
    "docs/utxo/doctorwatson/astra/cardano-design.md",
    "models/doctorwatson/doctorwatson.qnt",
)
_MAX_ISSUES = 8
_MAX_ISSUE_PATHS = 8
_QUESTION = (
    "Cardano Groth16 first Preview claim mint: locate existing public proof and "
    "verification-key records, compiled validator evidence, transaction-builder "
    "handoff and remaining ledger versus financial-settlement gaps"
)


def _sha(path: Path) -> str | None:
    if path.is_symlink() or not path.is_file():
        return None
    return hashlib.sha256(path.read_bytes()).hexdigest()


def _bounded_status(report: dict) -> dict:
    """Retain the original verdict; bound lists rather than hiding omissions."""
    result = dict(report)
    issues = report.get("issues", [])
    bounded = []
    for issue in issues[:_MAX_ISSUES]:
        item = dict(issue)
        if isinstance(item.get("paths"), list):
            item["pathCount"] = len(item["paths"])
            item["paths"] = item["paths"][:_MAX_ISSUE_PATHS]
            item["omittedPathCount"] = item["pathCount"] - len(item["paths"])
        bounded.append(item)
    result.update(issues=bounded, issueCount=len(issues),
                  omittedIssueCount=max(0, len(issues) - len(bounded)))
    return result


def _stale_summary(report: dict) -> str:
    parts = []
    for issue in report.get("issues", [])[:3]:
        detail = issue.get("path") or issue.get("recorded")
        if not detail and isinstance(issue.get("paths"), list):
            detail = f"{len(issue['paths'])} paths; first: " + ", ".join(issue["paths"][:2])
        parts.append(str(issue.get("kind", "unknown")) + (f" ({detail})" if detail else ""))
    return "; ".join(parts)


def _source_scope(out: Path, identity: dict, graph: dict) -> dict | None:
    """Validate an optional narrow source increment without blessing old rows."""
    expected = identity.get("cardano_first_proof_scope_sha256")
    if expected is None:
        return None
    scope_path = out / "cardano-first-proof-scope.json"
    if _sha(scope_path) != expected:
        return {"status": "STALE", "issues": ["source-scope-identity-mismatch"]}
    scope = json.loads(scope_path.read_text())
    if scope.get("format") != "cardano-first-proof-source-scope/1":
        return {"status": "STALE", "issues": ["source-scope-format-mismatch"]}
    issues = []
    sources = scope["sources"]
    if not isinstance(sources, list) or not sources:
        return {"status": "STALE", "issues": ["source-scope-empty"]}
    for row in sources:
        if _sha(Path(row["source_file"])) != row["sha256"]:
            issues.append({"kind": "source-scope-body-changed", "path": row["source_file"]})
    by_id = {node["id"]: node for node in graph["nodes"]}
    refs = scope["handoff_references"]
    if not isinstance(refs, list) or not refs or len(refs) > 4:
        issues.append({"kind": "source-scope-handoff-missing-or-oversize"})
        refs = []
    for ref in refs:
        node = by_id.get(ref["nodeId"], {})
        if (node.get("source_sha256") != ref["sourceSha256"]
                or node.get("source_file") != ref["sourceFile"]
                or node.get("source_location") != ref["sourceLocation"]
                or not any(row["source_file"] == ref["sourceFile"] and row["sha256"] == ref["sourceSha256"]
                           for row in sources)):
            issues.append({"kind": "source-scope-node-binding-mismatch", "nodeId": ref["nodeId"]})
    if scope.get("seed") not in {ref["nodeId"] for ref in refs}:
        issues.append({"kind": "source-scope-seed-mismatch"})
    return {"status": "FRESH_FOR_EXPLICIT_SOURCE_SCOPE" if not issues else "STALE",
            "namespace": scope.get("namespace"), "sourceCount": len(sources),
            "coverage": "deterministic source structure and named references only; partial semantics",
            "wholeGraphFresh": False, "scopeFile": str(scope_path),
            "issues": issues[:8], "issueCount": len(issues),
            "seed": scope.get("seed"), "handoffReferences": refs if not issues else []}


def _references(graph: dict, manifest: dict) -> list[dict]:
    refs = []
    rows = manifest.get("files", [])
    for rel in _SOURCES:
        candidates = [n for n in graph.get("nodes", [])
                      if n.get("source_file") == rel
                      and n.get("source_namespace") == "active_utxo_ba21"
                      and isinstance(n.get("id"), str)
                      and n.get("source_location")]
        # Prefer a source-located structural heading over an inferred concept.
        candidates.sort(key=lambda n: (n.get("_origin") != "structural", n["id"]))
        for node in candidates:
            matches = [r for r in rows if r.get("relative_path") == rel
                       and r.get("snapshot") == node.get("source_namespace")
                       and r.get("sha256") == node.get("source_sha256")]
            if len(matches) != 1:
                continue
            row = matches[0]
            path = Path(row["source_file"])
            actual = _sha(path)
            refs.append({
                "nodeId": node["id"], "sourceFile": str(path),
                "sourceLocation": node["source_location"],
                "sourceSha256": node["source_sha256"],
                "sourceSnapshot": node.get("source_snapshot"),
                "sourceStatus": node.get("source_status"),
                "currentSourceSha256": actual,
                "sourceCurrentness": "MATCH" if actual == row["sha256"] else "CHANGED_OR_MISSING",
                "readRequired": True,
            })
            break
    return refs


def load_graph_context(
    repo_root: Path | str,
    graphify_out: Path | str | None = None,
    *, environ: Mapping[str, str] | None = None,
) -> dict:
    """Resolve arg > GRAPHIFY_OUT > checkout-local graph; never auto-refresh.

    Freshness comes from the existing graph-nav implementation, including its
    complete pinned-source inventory check. The suggested native query is data,
    never executed here; on STALE it is explicitly dated discovery only.
    """
    root = Path(repo_root).resolve()
    env = os.environ if environ is None else environ
    selected = graphify_out if graphify_out is not None else env.get("GRAPHIFY_OUT")
    origin = "argument" if graphify_out is not None else "GRAPHIFY_OUT" if selected else "checkout-default"
    out = Path(selected).expanduser() if selected else root / "graphify-out"
    if not out.is_absolute():
        out = root / out
    out = out.resolve()
    report = _NAV.status(root, out)
    result = {
        "status": report["status"], "graphifyOut": str(out), "resolution": origin,
        "freshness": _bounded_status(report),
        "semanticCoverage": "partial; consult semantic-coverage.json and exact source bodies",
        "coverageFile": str(out / "semantic-coverage.json"),
        "identityFile": str(out / "graph-identity.json"),
        "sourceManifest": str(out / "source-manifest.json"),
        "sourceScope": None,
        "handoffReferences": [], "firstStageQuery": None,
        "navigationOnly": True, "ledgerAcceptance": False,
        "nextAction": "Inspect graph freshness issues and exact current source; do not rebuild automatically. " + _stale_summary(report),
    }
    try:
        identity = json.loads((out / "graph-identity.json").read_text())
        # A modified/unbound graph cannot supply even a suggested known-node query.
        if any(_sha(out / name) != identity[key] for name, key in (
            ("graph.json", "graph_sha256"),
            ("source-manifest.json", "source_manifest_sha256"),
            ("semantic-coverage.json", "semantic_coverage_sha256"),
        )):
            return result
        graph = json.loads((out / "graph.json").read_text())
        manifest = json.loads((out / "source-manifest.json").read_text())
        refs = _references(graph, manifest)
        scope = _source_scope(out, identity, graph)
        result["sourceScope"] = scope
        scope_fresh = scope is not None and scope["status"] == "FRESH_FOR_EXPLICIT_SOURCE_SCOPE"
        if scope_fresh:
            refs = scope["handoffReferences"]
        result["handoffReferences"] = refs
        result["identity"] = {"graphSha256": identity["graph_sha256"],
                              "nodes": len(graph["nodes"]),
                              "links": len(graph.get("links", graph.get("edges", []))),
                              "sourceRows": len(manifest["files"])}
        if refs:
            mode = ("current-explicit-source-scope-only" if scope_fresh else
                    "current-pinned-scope-navigation" if report["status"] == "FRESH_FOR_PINNED_SCOPE" else "dated-discovery-only")
            seed = scope["seed"] if scope_fresh else refs[0]["nodeId"]
            result["firstStageQuery"] = {
                "mode": mode, "seed": seed, "question": _QUESTION,
                "budget": 1500, "budgetIsHardCap": False,
                "argv": ["graphify", "query", _QUESTION, "--graph", str(out / "graph.json"),
                         "--seed", seed, "--budget", "1500"],
                "requiresExactSourceRead": True,
            }
            result["nextAction"] = (
                "Run the explicit seeded query as " + mode +
                "; read its exact sources and verify changed bodies before selecting work. " +
                ("Inherited graph remains STALE: " + _stale_summary(report) if report["status"] == "STALE" else "")
            )
    except (OSError, ValueError, TypeError, KeyError, AttributeError):
        # graph-nav's fail-closed verdict remains authoritative; no inferred IDs.
        result["contextLoadIssue"] = "graph-navigation-metadata-unavailable"
    return result
