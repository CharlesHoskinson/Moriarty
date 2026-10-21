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
# Newest first, restricted to the three explicitly published source scopes.
# The short question preserves the observed Graphify context-filter correction.
_CURRENT_SCOPE_QUESTION = "inspect protocol prepare plan evaluation remote request"
_SCOPES = (
    ("cardano-preview-cli-filebinding-scope.json", "cardano_preview_cli_filebinding_scope_sha256",
     "cardano-preview-cli-filebinding-source-scope/1", _CURRENT_SCOPE_QUESTION),
    ("cardano-protocol-planner-evaluation-scope.json", "cardano_protocol_planner_evaluation_scope_sha256",
     "cardano-protocol-planner-evaluation-source-scope/1", _CURRENT_SCOPE_QUESTION),
    ("cardano-first-proof-scope.json", "cardano_first_proof_scope_sha256",
     "cardano-first-proof-source-scope/1", _QUESTION),
)


def _sha(path: Path) -> str | None:
    if path.is_symlink() or not path.is_file():
        return None
    return hashlib.sha256(path.read_bytes()).hexdigest()


def _digest_matches(actual: str | None, expected: object) -> bool:
    """An unavailable digest never matches missing or null metadata."""
    return isinstance(actual, str) and isinstance(expected, str) and actual == expected


def _required_text(value: object) -> bool:
    """Named navigation pins require text with non-whitespace content."""
    return isinstance(value, str) and bool(value.strip())


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


def _scope_candidate(root: Path, out: Path, identity: dict, graph: dict,
                     name: str, key: str, expected_format: str, question: str) -> dict:
    """Check one known identity-bound scope against the requested checkout."""
    scope_path = out / name
    result = {"status": "STALE", "scopeFile": str(scope_path), "matchesCheckout": False,
              "issues": [], "issueCount": 0, "wholeGraphFresh": False,
              "handoffReferences": []}

    def reject(kind: str) -> dict:
        result.update(issues=[{"kind": kind}], issueCount=1)
        return result

    expected = identity[key]
    if not _digest_matches(_sha(scope_path), expected):
        return reject("source-scope-identity-mismatch")
    scope = json.loads(scope_path.read_text())
    if scope.get("format") != expected_format:
        return reject("source-scope-format-mismatch")
    issues = []
    sources = scope["sources"]
    if not isinstance(sources, list) or not sources:
        return reject("source-scope-empty")
    result.update(namespace=scope.get("namespace"), sourceCount=len(sources))
    # A valid scope from another worktree remains dated discovery, even when
    # every byte in that other worktree still matches its recorded hash.
    for row in sources:
        source = Path(row["source_file"])
        recorded_root = Path(row["source_root"])
        if (not recorded_root.is_absolute() or recorded_root.resolve() != root or not source.is_absolute()
                or source.resolve() != source or not source.is_relative_to(root)):
            return reject("source-scope-root-mismatch")
    result["matchesCheckout"] = True
    actual_digests = {}
    for row in sources:
        expected_digest = row.get("sha256")
        if not isinstance(expected_digest, str):
            return reject("source-scope-digest-invalid")
        actual = _sha(Path(row["source_file"]))
        actual_digests[row["source_file"]] = actual
        if not _digest_matches(actual, expected_digest):
            issues.append({"kind": "source-scope-body-changed", "path": row["source_file"]})
    by_id = {node["id"]: node for node in graph["nodes"] if _required_text(node.get("id"))}
    refs = scope["handoff_references"]
    if not isinstance(refs, list) or not refs or len(refs) > 4:
        issues.append({"kind": "source-scope-handoff-missing-or-oversize"})
        refs = []
    for ref in refs:
        if not _required_text(ref.get("nodeId")) or not _required_text(ref.get("sourceLocation")):
            return reject("source-scope-reference-pin-invalid")
        node = by_id.get(ref["nodeId"], {})
        if (node.get("source_sha256") != ref["sourceSha256"]
                or node.get("source_file") != ref["sourceFile"]
                or node.get("source_location") != ref["sourceLocation"]
                or not any(row["source_file"] == ref["sourceFile"] and row["sha256"] == ref["sourceSha256"]
                           for row in sources)):
            issues.append({"kind": "source-scope-node-binding-mismatch", "nodeId": ref["nodeId"]})
    if not _required_text(scope.get("seed")) or scope.get("seed") not in {ref["nodeId"] for ref in refs}:
        issues.append({"kind": "source-scope-seed-mismatch"})
    current_refs = [dict(ref, currentSourceSha256=actual_digests[ref["sourceFile"]],
                         sourceCurrentness="MATCH", readRequired=True)
                    for ref in refs] if not issues else []
    return {"status": "FRESH_FOR_EXPLICIT_SOURCE_SCOPE" if not issues else "STALE",
            "namespace": scope.get("namespace"), "sourceCount": len(sources),
            "coverage": "deterministic source structure and named references only; partial semantics",
            "wholeGraphFresh": False, "scopeFile": str(scope_path),
            "matchesCheckout": True, "queryQuestion": question,
            "issues": issues[:8], "issueCount": len(issues),
            "seed": scope.get("seed"), "handoffReferences": current_refs}


def _source_scope(root: Path, out: Path, identity: dict, graph: dict) -> dict | None:
    """Prefer the newest known scope whose checkout, source and node pins match."""
    candidates = []
    for name, key, expected_format, question in _SCOPES:
        if key not in identity:
            continue
        try:
            candidate = _scope_candidate(root, out, identity, graph, name, key, expected_format, question)
        except (OSError, ValueError, TypeError, KeyError, AttributeError):
            candidate = {"status": "STALE", "scopeFile": str(out / name),
                         "matchesCheckout": False, "wholeGraphFresh": False,
                         "issues": [{"kind": "source-scope-metadata-unavailable"}],
                         "issueCount": 1, "handoffReferences": []}
        candidates.append(candidate)
    if not candidates:
        return None
    matching = [c for c in candidates if c["matchesCheckout"]]
    selected = next((c for c in matching if c["status"] == "FRESH_FOR_EXPLICIT_SOURCE_SCOPE"),
                    matching[0] if matching else candidates[0])
    selected["candidateScopes"] = [
        {key: candidate[key] for key in ("scopeFile", "status", "matchesCheckout", "issueCount")}
        for candidate in candidates
    ]
    return selected


def _references(graph: dict, manifest: dict) -> list[dict]:
    refs = []
    rows = manifest.get("files", [])
    for rel in _SOURCES:
        candidates = [n for n in graph.get("nodes", [])
                      if n.get("source_file") == rel
                      and n.get("source_namespace") == "active_utxo_ba21"
                      and _required_text(n.get("id"))
                      and _required_text(n.get("source_location"))]
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
                "sourceCurrentness": "MATCH" if _digest_matches(actual, row["sha256"]) else "CHANGED_OR_MISSING",
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
        if any(not _digest_matches(_sha(out / name), identity[key]) for name, key in (
            ("graph.json", "graph_sha256"),
            ("source-manifest.json", "source_manifest_sha256"),
            ("semantic-coverage.json", "semantic_coverage_sha256"),
        )):
            return result
        graph = json.loads((out / "graph.json").read_text())
        manifest = json.loads((out / "source-manifest.json").read_text())
        refs = _references(graph, manifest)
        scope = _source_scope(root, out, identity, graph)
        result["sourceScope"] = scope
        scope_fresh = scope is not None and scope["status"] == "FRESH_FOR_EXPLICIT_SOURCE_SCOPE"
        scope_note = (" Explicit source scope is STALE: " + _stale_summary(scope)
                      if scope is not None and not scope_fresh else "")
        result["nextAction"] += scope_note
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
            question = scope["queryQuestion"] if scope_fresh else _QUESTION
            result["firstStageQuery"] = {
                "mode": mode, "seed": seed, "question": question,
                "budget": 1500, "budgetIsHardCap": False,
                "argv": ["graphify", "query", question, "--graph", str(out / "graph.json"),
                         "--seed", seed, "--budget", "1500"],
                "requiresExactSourceRead": True,
            }
            result["nextAction"] = (
                "Run the explicit seeded query as " + mode +
                "; read its exact sources and verify changed bodies before selecting work. " +
                ("Inherited graph remains STALE: " + _stale_summary(report) if report["status"] == "STALE" else "") + scope_note
            )
    except (OSError, ValueError, TypeError, KeyError, AttributeError):
        # graph-nav's fail-closed verdict remains authoritative; no inferred IDs.
        result["contextLoadIssue"] = "graph-navigation-metadata-unavailable"
    return result
