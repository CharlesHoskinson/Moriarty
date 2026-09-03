#!/usr/bin/env python3
"""Crawl the live Marlowe documentation sitemap through Scrapling.

The sitemap currently emits retired play.marlowe.iohk.io URLs.  The same paths
are requested from docs.marlowe-lang.org and both forms are recorded.
"""

from __future__ import annotations

import hashlib
import json
import time
from datetime import datetime, timezone
from pathlib import Path
from urllib.parse import urlsplit, urlunsplit
from xml.etree import ElementTree

from scrapling.fetchers import Fetcher


ROOT = Path(__file__).resolve().parents[1]
SITEMAP = ROOT / "raw" / "sources" / "docs-marlowe-lang-org-sitemap-2026-09-02.html"
RAW = ROOT / "raw" / "sources" / "marlowe-docs-live-2026-09-02"
TEXT = ROOT / "corpus" / "marlowe-docs-live-2026-09-02"
MANIFEST = ROOT / "evidence" / "marlowe-docs-live-acquisition-2026-09-02.json"


def rewritten_url(url: str) -> str:
    parts = urlsplit(url)
    return urlunsplit(("https", "docs.marlowe-lang.org", parts.path, parts.query, ""))


def filename(url: str, suffix: str) -> str:
    path = urlsplit(url).path.strip("/") or "home"
    return path.replace("/", "__") + suffix


def extract_text(page) -> str:
    nodes = page.css("main ::text") or page.css("body ::text")
    lines = []
    for node in nodes:
        line = " ".join(str(node).split())
        if line:
            lines.append(line)
    return "\n".join(lines) + "\n"


def main() -> None:
    sitemap = ElementTree.parse(SITEMAP).getroot()
    listed = [node.text for node in sitemap.iter() if node.tag.endswith("loc") and node.text]
    RAW.mkdir(parents=True, exist_ok=True)
    TEXT.mkdir(parents=True, exist_ok=True)
    captures = []
    for index, old_url in enumerate(listed, 1):
        url = rewritten_url(old_url)
        record = {"sitemap_url": old_url, "requested_url": url, "ordinal": index}
        try:
            page = Fetcher.get(url, timeout=45, retries=2, stealthy_headers=True)
            body = bytes(page.body)
            raw_path = RAW / filename(url, ".html")
            text_path = TEXT / filename(url, ".txt")
            raw_path.write_bytes(body)
            text_path.write_text(extract_text(page))
            record.update(
                {
                    "final_url": str(page.url),
                    "http_status": page.status,
                    "content_type": page.headers.get("content-type"),
                    "last_modified": page.headers.get("last-modified"),
                    "bytes": len(body),
                    "sha256": hashlib.sha256(body).hexdigest(),
                    "raw_path": str(raw_path.relative_to(ROOT)),
                    "text_path": str(text_path.relative_to(ROOT)),
                }
            )
        except Exception as error:  # capture failures without losing the run
            record.update({"http_status": None, "error": f"{type(error).__name__}: {error}"})
        captures.append(record)
        print(f"{index:03d}/{len(listed):03d} {record.get('http_status')} {url}", flush=True)
        time.sleep(0.15)

    result = {
        "retrieved_at_utc": datetime.now(timezone.utc).isoformat(),
        "acquisition_method": "Scrapling Fetcher; sequential; 150 ms inter-request delay",
        "sitemap": str(SITEMAP.relative_to(ROOT)),
        "sitemap_url_count": len(listed),
        "successful": sum(record.get("http_status") == 200 for record in captures),
        "failed": sum(record.get("http_status") != 200 for record in captures),
        "host_rewrite": "play.marlowe.iohk.io -> docs.marlowe-lang.org; path preserved",
        "captures": captures,
    }
    MANIFEST.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({key: result[key] for key in ("sitemap_url_count", "successful", "failed")}, indent=2))


if __name__ == "__main__":
    main()
