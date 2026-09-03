#!/usr/bin/env python3
"""Capture one public URL with Scrapling and emit a machine-readable receipt."""

from __future__ import annotations

import argparse
import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path

from scrapling.fetchers import DynamicFetcher, Fetcher, StealthyFetcher


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("url")
    parser.add_argument("output", type=Path)
    parser.add_argument("--fetcher", choices=("static", "dynamic", "stealth"), default="static")
    parser.add_argument("--timeout", type=int, default=60)
    parser.add_argument("--wait", type=int, default=0, help="Browser wait after load, in milliseconds")
    parser.add_argument("--accept", default="*/*")
    parser.add_argument("--method", choices=("GET", "POST"), default="GET")
    parser.add_argument(
        "--json-body",
        help="JSON object or array for a static POST request; dynamic fetchers support GET only",
    )
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    headers = {"Accept": args.accept}
    if args.fetcher == "static":
        if args.method == "POST":
            if args.json_body is None:
                raise SystemExit("--json-body is required with --method POST")
            page = Fetcher.post(
                args.url,
                headers=headers,
                timeout=args.timeout,
                json=json.loads(args.json_body),
            )
        else:
            page = Fetcher.get(args.url, headers=headers, timeout=args.timeout)
    elif args.fetcher == "dynamic":
        if args.method != "GET":
            raise SystemExit("dynamic fetcher supports GET only")
        page = DynamicFetcher.fetch(
            args.url,
            headless=True,
            timeout=args.timeout * 1000,
            network_idle=True,
            wait=args.wait,
        )
    else:
        if args.method != "GET":
            raise SystemExit("stealth fetcher supports GET only")
        page = StealthyFetcher.fetch(
            args.url,
            headless=True,
            timeout=args.timeout * 1000,
            network_idle=True,
            wait=args.wait,
        )

    body = bytes(page.body)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_bytes(body)
    receipt = {
        "requested_url": args.url,
        "final_url": str(page.url),
        "retrieved_at_utc": datetime.now(timezone.utc).isoformat(),
        "http_status": page.status,
        "content_type": page.headers.get("content-type"),
        "etag": page.headers.get("etag"),
        "last_modified": page.headers.get("last-modified"),
        "fetcher": f"Scrapling {args.fetcher}",
        "method": args.method,
        "output": str(args.output),
        "bytes": len(body),
        "sha256": hashlib.sha256(body).hexdigest(),
    }
    receipt_path = args.output.with_suffix(args.output.suffix + ".receipt.json")
    receipt_path.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
    print(json.dumps(receipt, indent=2, sort_keys=True))
    if page.status >= 400:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
