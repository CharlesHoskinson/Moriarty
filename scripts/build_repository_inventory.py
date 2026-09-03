#!/usr/bin/env python3
"""Build a reproducible Marlowe organization inventory from captured API JSON.

This script performs no network access.  It reads the immutable Scrapling
captures in raw/sources/github-api and writes a dated evidence table.
"""

from __future__ import annotations

import csv
import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
API = ROOT / "raw" / "sources" / "github-api"
OUT = ROOT / "evidence" / "repository-classification-2026-09-02.tsv"


CLASSIFICATION = {
    ".github": ("infrastructure or deployment support", "Organization profile, transition, and shared governance files."),
    "actus-core": ("active and authoritative", "Current Haskell implementation of the ACTUS taxonomy; a library, not Marlowe semantics."),
    "admin": ("maintenance-only", "Community administration, meeting, and planning material."),
    "awesome-marlowe": ("demonstration/example material", "Curated links and copied examples; not normative and not a current-service registry."),
    "marlowe": ("formal specification", "Isabelle V1 semantics, generated Haskell, analysis, and specification tests; authority must be scoped per artifact."),
    "marlowe-actus-labs": ("demonstration/example material", "Browser laboratory for ACTUS-derived Marlowe contracts."),
    "marlowe-agda": ("formal specification", "Experimental Agda semantics and proof work; no released/deployed authority found."),
    "marlowe-cardano": ("maintenance-only", "Deployed-era V1 implementation and full Runtime lineage; active modernization moved to marlowe-plutus branches."),
    "marlowe-cardano-minimal": ("archived", "Archived minimal-validator experiment."),
    "marlowe-deploy": ("infrastructure or deployment support", "Nix deployment definitions and network configuration."),
    "marlowe-doc": ("maintenance-only", "Documentation source; live pages and artifacts show drift and require version reconciliation."),
    "marlowe-hydra-poc": ("active but experimental", "Hydra proof of concept; not production-backed portability evidence."),
    "marlowe-lambda": ("superseded", "Archived and explicitly replaced by the Runtime web server app."),
    "marlowe-oracle-protocol": ("active but experimental", "Actively developed oracle protocol/CIP work; extension-layer proposal, not deployed core semantics."),
    "marlowe-order-book-swap": ("demonstration/example material", "Prototype order-book swap application."),
    "marlowe-payouts": ("demonstration/example material", "Generic payout withdrawal prototype."),
    "marlowe-playground": ("maintenance-only", "Legacy visual/text authoring application; dependency activity is not feature evolution."),
    "marlowe-plutus": ("active but experimental", "Default validator repository plus unreleased 2026 migration/Runtime work on feature branches."),
    "marlowe-runner": ("maintenance-only", "Runtime-dependent browser application; recent work is largely dependency/release maintenance."),
    "marlowe-runtime-ng": ("unclear", "One-commit placeholder; implementation work is instead visible on marlowe-plutus feature branches."),
    "marlowe-scan": ("maintenance-only", "Runtime-dependent explorer supporting Runtime 0.0.4 through 1.0.0."),
    "marlowe-starter-kit": ("demonstration/example material", "Tutorial notebooks for REST and CLI workflows."),
    "marlowe-token-plans": ("active but experimental", "Token-plan/vesting application prototype."),
    "marlowe-ts-dapp-swap": ("archived", "Archived TypeScript swap experiment."),
    "marlowe-ts-sdk": ("client library", "V1 TypeScript language/Runtime SDK; newer work exists on beta branches but no V2 core release found."),
    "marlowe-website": ("active and authoritative", "Current project website and V2 workshop report; descriptive/proposal authority only."),
    "MIPs": ("maintenance-only", "Improvement-proposal process with old default branch and several unmerged proposal branches."),
    "purescript-bridge-json-helpers": ("superseded", "Legacy PureScript bridge helper used by the prior web stack."),
    "purescript-cardano-multiplatform-lib": ("superseded", "Legacy PureScript Cardano binding layer."),
    "purescript-cardano-wallet-client": ("superseded", "Legacy PureScript wallet client."),
    "purescript-datetime-iso": ("superseded", "Small legacy PureScript serialization utility."),
    "purescript-markdown": ("superseded", "Small legacy PureScript Markdown library."),
    "purescript-marlowe": ("client library", "PureScript V1 language implementation used by the legacy application stack."),
    "purescript-marlowe-runtime-client": ("client library", "PureScript Runtime client used by the legacy application stack."),
    "purescript-web-common": ("superseded", "Shared legacy PureScript web utilities."),
    "real-world-marlowe": ("archived", "Archived gallery and application examples; adoption claims need independent chain evidence."),
}


MAINTENANCE = re.compile(
    r"^(merge |bump |update (flake|nix|depend|workflow|actions)|dependabot|release |"
    r"prepare release|chore(?:\(|:)|ci(?:\(|:)|format(?:\(|:)|fix typo|readme)",
    re.IGNORECASE,
)


def data(repo: str, kind: str):
    path = API / repo / f"{kind}-2026-09-02.json"
    return json.loads(path.read_text())["data"]


def one_line(value: str | None) -> str:
    lines = (value or "").splitlines()
    return lines[0].strip() if lines else ""


def latest_meaningful(commits: list[dict]) -> tuple[str, str, str]:
    for commit in commits:
        message = one_line(commit["commit"]["message"])
        if not MAINTENANCE.search(message):
            return commit["sha"], commit["commit"]["author"]["date"], message
    if commits:
        commit = commits[0]
        return commit["sha"], commit["commit"]["author"]["date"], one_line(commit["commit"]["message"])
    return "", "", ""


def main() -> None:
    rows = []
    for repo_dir in sorted(path for path in API.iterdir() if path.is_dir()):
        repo = repo_dir.name
        metadata = data(repo, "metadata")
        commits = data(repo, "commits")
        issues = data(repo, "issues-open")
        pulls = data(repo, "pulls-open")
        releases = data(repo, "releases")
        tags = data(repo, "tags")
        branches = data(repo, "branches")
        contributors = data(repo, "contributors")
        sha, date, message = latest_meaningful(commits)
        category, note = CLASSIFICATION[repo]
        rows.append(
            {
                "repository": repo,
                "classification": category,
                "archived": str(metadata["archived"]).lower(),
                "language": metadata.get("language") or "",
                "default_branch": metadata["default_branch"],
                "default_head": commits[0]["sha"] if commits else "",
                "default_head_date": commits[0]["commit"]["author"]["date"] if commits else "",
                "latest_meaningful_sha_heuristic": sha,
                "latest_meaningful_date_heuristic": date,
                "latest_meaningful_message_heuristic": message,
                "pushed_at_any_branch": metadata["pushed_at"],
                "releases": len(releases),
                "latest_release": releases[0].get("tag_name", "") if releases else "",
                "tags_captured": len(tags),
                "branches_captured": len(branches),
                "open_issues_excluding_prs": sum("pull_request" not in issue for issue in issues),
                "open_pulls": len(pulls),
                "contributors_captured": len(contributors),
                "size_kib": metadata["size"],
                "description": one_line(metadata.get("description")),
                "classification_basis": note,
            }
        )

    OUT.parent.mkdir(parents=True, exist_ok=True)
    with OUT.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=rows[0].keys(), delimiter="\t")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {len(rows)} rows to {OUT.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
