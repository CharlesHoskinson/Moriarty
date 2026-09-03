#!/usr/bin/env python3
"""Clone every repository in a Scrapling-acquired GitHub organization snapshot."""

from __future__ import annotations

import argparse
import json
import subprocess
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime, timezone
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("snapshot", type=Path)
    parser.add_argument("destination", type=Path)
    parser.add_argument("--workers", type=int, default=4)
    return parser.parse_args()


def clone_one(repo: dict[str, object], destination: Path) -> dict[str, object]:
    name = str(repo["name"])
    target = destination / name
    if target.exists():
        command = ["git", "-C", str(target), "fetch", "--depth=1", "origin", str(repo["default_branch"])]
    else:
        command = [
            "git",
            "clone",
            "--quiet",
            "--depth=1",
            "--filter=blob:none",
            "--branch",
            str(repo["default_branch"]),
            str(repo["clone_url"]),
            str(target),
        ]
    process = subprocess.run(command, capture_output=True, text=True, timeout=900)
    row: dict[str, object] = {
        "name": name,
        "clone_url": repo["clone_url"],
        "default_branch": repo["default_branch"],
        "archived": repo["archived"],
        "fork": repo["fork"],
        "exit_code": process.returncode,
        "stderr": process.stderr.strip(),
    }
    if process.returncode == 0:
        head = subprocess.run(
            ["git", "-C", str(target), "rev-parse", "HEAD"],
            capture_output=True,
            text=True,
            check=True,
        )
        row["head"] = head.stdout.strip()
    return row


def main() -> None:
    args = parse_args()
    repos = json.loads(args.snapshot.read_text())
    args.destination.mkdir(parents=True, exist_ok=True)
    results = []
    with ThreadPoolExecutor(max_workers=args.workers) as pool:
        futures = [pool.submit(clone_one, repo, args.destination) for repo in repos]
        for future in as_completed(futures):
            row = future.result()
            results.append(row)
            print(json.dumps(row, sort_keys=True), flush=True)
    results.sort(key=lambda row: str(row["name"]).lower())
    manifest = {
        "generated_at_utc": datetime.now(timezone.utc).isoformat(),
        "snapshot": str(args.snapshot),
        "destination": str(args.destination),
        "scope": "Shallow, blob-filtered clone of every default branch in the supplied public API snapshot.",
        "repositories": results,
    }
    manifest_path = args.destination / "clone-manifest.json"
    manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")
    failures = [row for row in results if row["exit_code"] != 0]
    print(f"cloned={len(results) - len(failures)} failed={len(failures)} total={len(results)}", flush=True)
    if failures:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
