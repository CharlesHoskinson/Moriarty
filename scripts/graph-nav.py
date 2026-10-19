#!/usr/bin/env python3
"""Thin current-worktree freshness check and native Graphify query delegation."""
from pathlib import Path
import argparse, hashlib, json, os, subprocess, sys

def git(root, *args):
    return subprocess.check_output(["git", "-C", str(root), *args])

def primary(rel, policy):
    p = Path(rel)
    if set(p.parts) & set(policy["excluded_components"]): return False
    if any(rel.startswith(x) for x in policy["excluded_prefixes"]): return False
    if any(x.startswith(".") for x in p.parts[:-1]): return False
    if p.suffix.lower() in policy["private_binary_extensions"]: return False
    if p.name.startswith(".env") or p.name in ["provider.private.jsonl", "stderr.private"] or "private-witness" in p.name or p.name.endswith(".witness.json"): return False
    return p.suffix.lower() in policy["supported_primary_text_extensions"] or p.name in ["Dockerfile", "Makefile", "justfile", "LICENSE", "AGENTS.md", "README.md"]

def _status(root, out):
    manifest = json.loads((out / "source-manifest.json").read_text())
    issues = []
    identity = json.loads((out / "graph-identity.json").read_text())
    for name, expected in [("graph.json", identity["graph_sha256"]), ("source-manifest.json", identity["source_manifest_sha256"]), ("semantic-coverage.json", identity["semantic_coverage_sha256"])]:
        path = out / name
        if not path.is_file() or path.is_symlink() or hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            issues.append({"kind": "graph-artifact-changed-or-missing", "path": str(path)})
    if Path(manifest["root"]).resolve() != root: issues.append({"kind": "wrong-root", "recorded": manifest["root"]})
    head = git(root, "rev-parse", "HEAD").decode().strip()
    active = [x for x in manifest["snapshots"] if Path(x["source_root"]).resolve() == root]
    if len(active) != 1:
        issues.append({"kind": "snapshot-cardinality", "expected": 1, "actual": len(active)})
    elif active[0]["git_commit"] != head:
        issues.append({"kind": "HEAD-changed", "actual": head, "recorded": active[0]["git_commit"]})
    rows = manifest["files"]
    for row in rows:
        p = Path(row["source_file"])
        if not p.is_file() or p.is_symlink(): issues.append({"kind": "missing-or-link", "path": str(p)}); continue
        if hashlib.sha256(p.read_bytes()).hexdigest() != row["sha256"]: issues.append({"kind": "body-changed", "path": str(p)})
    names = git(root, "ls-files", "--cached", "--others", "--exclude-standard", "-z").decode().split("\0")
    current = set()
    for n in names:
        p = root / n
        if not n or not primary(n, manifest["policy"]) or not p.is_file() or p.is_symlink(): continue
        data = p.read_bytes()
        try: data.decode("utf8")
        except UnicodeDecodeError: continue
        if b"\0" not in data: current.add(n)
    recorded = {r["relative_path"] for r in rows if Path(r["source_root"]).resolve() == root and primary(r["relative_path"], manifest["policy"])}
    if current - recorded: issues.append({"kind": "new-primary-files", "paths": sorted(current-recorded)})
    if recorded - current: issues.append({"kind": "deleted-primary-files", "paths": sorted(recorded-current)})
    return {"root": str(root), "graph": str(out/"graph.json"), "HEAD": head, "status": "FRESH_FOR_PINNED_SCOPE" if not issues else "STALE", "issues": issues, "semanticCoverage": "partial; see semantic-coverage.json"}

def status(root, out):
    try:
        return _status(root, out)
    except (OSError, ValueError, TypeError, KeyError, subprocess.CalledProcessError) as error:
        return {"root": str(root), "graph": str(out/"graph.json"), "status": "STALE", "issues": [{"kind": "metadata-or-source-read-error", "errorType": type(error).__name__}], "semanticCoverage": "unverified; metadata could not be checked"}

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["status", "query"])
    parser.add_argument("question", nargs="?")
    parser.add_argument("--seed")
    parser.add_argument("--budget", type=int, default=1500, help="native approximate retrieval target, not a hard cap")
    args = parser.parse_args()
    try:
        root = Path(git(Path(__file__).resolve().parent, "rev-parse", "--show-toplevel").decode().strip()).resolve()
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        print(json.dumps({"status": "STALE", "issues": [{"kind": "worktree-root-unavailable", "errorType": type(error).__name__}]}))
        return 1
    out = Path(os.environ.get("GRAPHIFY_OUT", str(root/"graphify-out"))).resolve()
    report = status(root, out)
    if args.command == "status": print(json.dumps(report, indent=2)); return int(bool(report["issues"]))
    if report["issues"]: print(json.dumps(report, indent=2), file=sys.stderr); return 1
    if not args.question: parser.error("query requires a question")
    command = ["graphify", "query", args.question, "--graph", str(out/"graph.json"), "--budget", str(args.budget)]
    if args.seed: command += ["--seed", args.seed]
    return subprocess.call(command, env={**os.environ, "GRAPHIFY_OUT": str(out)})

if __name__ == "__main__":
    raise SystemExit(main())
