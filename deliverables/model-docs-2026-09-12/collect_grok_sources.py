#!/usr/bin/env python3
"""Add a bounded, public xAI Grok 4.6 corpus using Scrapling Fetcher."""
from __future__ import annotations

import hashlib
import json
import subprocess
from datetime import datetime, timezone
from pathlib import Path

from scrapling.fetchers import Fetcher


ROOT = Path(__file__).resolve().parent
SOURCES = ROOT / "sources"
CORPUS = ROOT / "corpus"
MANIFEST = ROOT / "MANIFEST.json"
RETRIEVED_AT = datetime.now(timezone.utc).replace(microsecond=0).isoformat()
TARGETS = [
    (
        "xai-grok-46-api-guide",
        "Grok 4.6 API guide, runtime locations, and cache guidance",
        "https://docs.x.ai/developers/grok-4-6",
    ),
    (
        "xai-grok-46-model-card",
        "Grok 4.6 model card and rate-limit detail",
        "https://docs.x.ai/developers/models/grok-4.6",
    ),
    (
        "xai-grok-reasoning",
        "Grok 4.6 reasoning-effort documentation",
        "https://docs.x.ai/developers/model-capabilities/text/reasoning",
    ),
    (
        "xai-grok-46-safety-model-card",
        "Grok 4.6 safety model card linked from xAI Safety",
        "https://media.x.ai/v1/website/card-4p6-4cd2dc57.pdf",
    ),
]


def write_wrapper(text_path: Path, record: dict) -> None:
    wrapper = (
        "---\n"
        f'source_url: "{record["canonical_url"]}"\n'
        f'captured_at: "{record["retrieved_at_utc"]}"\n'
        f'source_text_sha256: "{record["text_sha256"]}"\n'
        "---\n\n"
        + text_path.read_text(encoding="utf-8")
    )
    (CORPUS / f'{record["slug"]}.md').write_text(wrapper, encoding="utf-8")


def capture(slug: str, purpose: str, requested_url: str) -> dict:
    page = Fetcher.get(requested_url, impersonate="chrome", timeout=60)
    body = bytes(page.body)
    content_type = str(page.headers.get("content-type", ""))
    is_pdf = "pdf" in content_type.lower() or body.startswith(b"%PDF-")
    raw_path = SOURCES / f'{slug}{".pdf" if is_pdf else ".html"}'
    raw_path.write_bytes(body)
    text_path = SOURCES / f"{slug}.txt"
    if is_pdf:
        subprocess.run(["pdftotext", "-layout", str(raw_path), str(text_path)], check=True)
    else:
        main = page.css("main").first or page.css("article").first or page.css("body").first
        text_path.write_text((main.get_all_text(separator="\n", strip=True) if main else "") + "\n", encoding="utf-8")
    record = {
        "slug": slug,
        "purpose": purpose,
        "requested_url": requested_url,
        "canonical_url": str(page.url),
        "retrieved_at_utc": RETRIEVED_AT,
        "http_status": page.status,
        "content_type": content_type,
        "raw_file": raw_path.name,
        "text_file": text_path.name,
        "bytes": len(body),
        "sha256": hashlib.sha256(body).hexdigest(),
        "text_sha256": hashlib.sha256(text_path.read_bytes()).hexdigest(),
        "access": "public, unauthenticated",
    }
    (SOURCES / f"{slug}.metadata.json").write_text(json.dumps(record, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    write_wrapper(text_path, record)
    return record


def main() -> None:
    SOURCES.mkdir(exist_ok=True)
    CORPUS.mkdir(exist_ok=True)
    records = [capture(*target) for target in TARGETS]
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    new_slugs = {record["slug"] for record in records}
    manifest["records"] = [record for record in manifest["records"] if record.get("slug") not in new_slugs] + records
    manifest["updated_at_utc"] = RETRIEVED_AT
    robots = manifest.setdefault("robots_checked", [])
    for url in ("https://x.ai/robots.txt", "https://docs.x.ai/robots.txt"):
        if url not in robots:
            robots.append(url)
    manifest["scope_note"] = (
        manifest.get("scope_note", "")
        + " xAI Grok 4.6 sources added 2026-09-12: the official API guide, model card, reasoning guide, and safety model card."
    )
    MANIFEST.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
