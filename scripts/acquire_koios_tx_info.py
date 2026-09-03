#!/usr/bin/env python3
"""Acquire Koios transaction details in bounded Scrapling POST requests."""

from __future__ import annotations

import argparse
import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path

from scrapling.fetchers import Fetcher


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("transactions", type=Path, help="Koios credential_txs JSON")
    parser.add_argument("output_dir", type=Path)
    parser.add_argument("--chunk-size", type=int, default=20)
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    rows = json.loads(args.transactions.read_text())
    tx_hashes = [row["tx_hash"] for row in rows]
    args.output_dir.mkdir(parents=True, exist_ok=True)
    manifest = {
        "source": str(args.transactions),
        "endpoint": "https://api.koios.rest/api/v1/tx_info",
        "retrieved_at_utc": datetime.now(timezone.utc).isoformat(),
        "fetcher": "Scrapling static",
        "chunk_size": args.chunk_size,
        "transactions_requested": len(tx_hashes),
        "chunks": [],
    }
    for index, start in enumerate(range(0, len(tx_hashes), args.chunk_size), 1):
        requested = tx_hashes[start : start + args.chunk_size]
        body = {
            "_tx_hashes": requested,
            "_inputs": True,
            "_metadata": False,
            "_assets": True,
            "_withdrawals": False,
            "_certs": False,
            "_scripts": True,
            "_bytecode": False,
        }
        page = Fetcher.post(
            manifest["endpoint"],
            headers={"Accept": "application/json"},
            timeout=60,
            json=body,
        )
        content = bytes(page.body)
        destination = args.output_dir / f"chunk-{index:03d}.json"
        destination.write_bytes(content)
        chunk = {
            "chunk": index,
            "output": str(destination),
            "requested": len(requested),
            "returned": len(json.loads(content)) if page.status == 200 else None,
            "http_status": page.status,
            "bytes": len(content),
            "sha256": hashlib.sha256(content).hexdigest(),
        }
        manifest["chunks"].append(chunk)
        print(json.dumps(chunk, sort_keys=True))
        if page.status >= 400:
            raise SystemExit(f"Koios returned HTTP {page.status} for chunk {index}")
    manifest_path = args.output_dir / "manifest.json"
    manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
