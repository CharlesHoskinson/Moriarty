#!/usr/bin/env python3
"""Build a reproducible inventory from Scrapling API snapshots and local clones.

This script intentionally distinguishes repository metadata from source-tree
evidence. GitHub's ``archived`` flag is not treated as authoritative when the
repository README explicitly says that development moved elsewhere.
"""

from __future__ import annotations

import argparse
import csv
import json
import subprocess
from pathlib import Path


AUTHORITATIVE = {
    "midnight-node": "node and consensus implementation",
    "midnight-ledger": "ledger implementation and specification",
    "midnight-indexer": "chain indexer",
    "midnight-js": "application SDK",
    "midnight-sdk": "application SDK workspace",
    "midnight-zk": "zero-knowledge implementation",
    "midnight-zkir": "ZKIR implementation",
    "midnight-wallet": "wallet core",
    "midnight-dapp-connector-api": "wallet connector API",
    "midnight-docs": "developer documentation",
    "midnight-improvement-proposals": "governance and protocol proposals",
}

EXPERIMENTAL = {
    "k-rust": "formal-semantics tooling",
    "k-framework-ts": "formal-semantics tooling",
    "compact-playground": "Compact service experiment",
    "learn-compact": "learning material under development",
    "midnight-expert": "AI tooling experiment",
    "passport": "application/platform under development",
}

INFRASTRUCTURE_WORDS = (
    "docker",
    "action",
    "faucet",
    "local-dev",
    "releases",
    "renovate",
    "servicedesk",
    "telemetry",
    "template",
    "workflows",
)


def git(repo: Path, *args: str) -> str:
    process = subprocess.run(
        ["git", "-C", str(repo), *args], capture_output=True, text=True
    )
    return process.stdout.strip() if process.returncode == 0 else ""


def classify(item: dict[str, object], readme: str) -> tuple[str, str, str]:
    name = str(item["name"])
    description = str(item.get("description") or "")
    lower = f"{name} {description} {readme}".lower()
    replacement = ""

    if name == "compact":
        replacement = "LFDT-Minokawa/compact"
        return "superseded", "S5 release-artifact mirror", replacement
    if bool(item.get("archived")) or "repository is archived" in lower:
        return "archived", "S0", replacement
    if bool(item.get("fork")):
        return "infrastructure or dependency fork", "S3/S6 unclear", replacement
    if name in AUTHORITATIVE:
        return "active and authoritative", "S4-S6; deployment not inferred", replacement
    if name in EXPERIMENTAL:
        return "active but experimental", "S3", replacement
    if name.startswith("example-") or any(
        word in name for word in ("demo", "tip-jar", "leaderboard", "idea-board")
    ):
        return "demonstration/example material", "S3", replacement
    if name in {"compact-js", "platform-js"} and "todo - new repo owner" in lower:
        return "unclear", "S1 placeholder", replacement
    if any(word in name for word in INFRASTRUCTURE_WORDS):
        return "infrastructure or deployment support", "S3/S6", replacement
    if name in {"midnight-architecture", "midnight-engineering"}:
        return "active and authoritative", "S2 documentation", replacement
    if any(word in name for word in ("did", "credential", "passport", "trust-registry")):
        return "active but experimental", "S3/S5", replacement
    return "unclear", "unverified", replacement


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("snapshot", type=Path)
    parser.add_argument("clone_root", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()

    snapshot = json.loads(args.snapshot.read_text())
    fields = [
        "repository",
        "category",
        "status",
        "description",
        "default_branch",
        "local_head",
        "head_commit_date",
        "github_pushed_at",
        "language",
        "archived_api",
        "fork",
        "open_issues_and_prs_api",
        "local_tag_count",
        "security_policy",
        "codeowners",
        "replacement",
        "html_url",
    ]
    rows: list[dict[str, object]] = []
    for item in snapshot:
        repo = args.clone_root / str(item["name"])
        readme_path = next(
            (path for path in (repo / "README.md", repo / "README") if path.exists()),
            None,
        )
        readme = readme_path.read_text(errors="replace") if readme_path else ""
        category, status, replacement = classify(item, readme)
        security = any(
            path.exists()
            for path in (repo / "SECURITY.md", repo / ".github" / "SECURITY.md")
        )
        codeowners = any(
            path.exists()
            for path in (
                repo / "CODEOWNERS",
                repo / ".github" / "CODEOWNERS",
                repo / "docs" / "CODEOWNERS",
            )
        )
        rows.append(
            {
                "repository": item["full_name"],
                "category": category,
                "status": status,
                "description": item.get("description") or "",
                "default_branch": item["default_branch"],
                "local_head": git(repo, "rev-parse", "HEAD"),
                "head_commit_date": git(repo, "log", "-1", "--format=%cI"),
                "github_pushed_at": item.get("pushed_at") or "",
                "language": item.get("language") or "",
                "archived_api": item.get("archived", False),
                "fork": item.get("fork", False),
                "open_issues_and_prs_api": item.get("open_issues_count", ""),
                "local_tag_count": len(git(repo, "tag", "--list").splitlines()),
                "security_policy": security,
                "codeowners": codeowners,
                "replacement": replacement,
                "html_url": item["html_url"],
            }
        )

    args.output.parent.mkdir(parents=True, exist_ok=True)
    with args.output.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, dialect="excel-tab")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {len(rows)} rows to {args.output}")


if __name__ == "__main__":
    main()
