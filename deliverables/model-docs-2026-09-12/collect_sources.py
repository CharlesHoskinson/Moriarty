#!/usr/bin/env python3
"""Acquire a small, public, official model-documentation corpus with Scrapling.

The collector deliberately uses no authentication, cookies, browser profile, or
access-control bypass.  It saves raw public responses plus a selector-limited
plain-text extract for reading and graph construction.
"""
from __future__ import annotations

import hashlib
import json
import re
import subprocess
from datetime import datetime, timezone
from pathlib import Path
from urllib.parse import urljoin

from scrapling.fetchers import Fetcher


ROOT = Path(__file__).resolve().parent
SOURCES = ROOT / "sources"
RETRIEVED_AT = datetime.now(timezone.utc).replace(microsecond=0).isoformat()

# These are public, first-party documentation and product pages selected after
# checking each host's robots.txt. The named model pages carry the specifications;
# generic pages are retained only where they explicitly document that model's
# prompting, tools, reasoning, or CLI model selection.
TARGETS = [
    ("google-gemini-models", "Gemini 3.8 Flash model card and model ID", "https://ai.google.dev/gemini-api/docs/models"),
    ("google-gemini-38-model-card", "Gemini 3.8 Flash model-specific card and capability table", "https://ai.google.dev/gemini-api/docs/models/gemini-3.8-flash"),
    ("google-gemini-38-guide", "Gemini 3.8 Flash specifications, prompting, thinking, and tools", "https://ai.google.dev/gemini-api/docs/latest-model"),
    ("google-gemini-thinking", "Gemini thinking configuration reference", "https://ai.google.dev/gemini-api/docs/thinking"),
    ("google-gemini-function-calling", "Gemini function-calling tool documentation", "https://ai.google.dev/gemini-api/docs/function-calling"),
    ("agy-headless-model-selection", "Official AGY CLI model and effort selection", "https://www.agy.dev/docs/cli/headless/"),
    ("anthropic-opus-product", "Claude Opus 5 product specifications and API name", "https://www.anthropic.com/claude/opus"),
    ("anthropic-opus-announcement", "Claude Opus 5 announcement and availability", "https://www.anthropic.com/news/claude-opus-5"),
    ("anthropic-system-cards", "Anthropic system-card index used to discover the Opus 5 card", "https://www.anthropic.com/system-cards"),
    ("anthropic-prompting", "Claude Opus 5 prompting and reasoning guidance", "https://docs.anthropic.com/en/docs/build-with-claude/prompt-engineering/prompt-templates-and-variables"),
    ("anthropic-tool-use", "Claude tool-use documentation with Opus 5 examples", "https://docs.anthropic.com/en/docs/agents-and-tools/tool-use/overview"),
    ("anthropic-claude-code-cli", "Claude Code CLI model-selection reference", "https://docs.anthropic.com/en/docs/claude-code/cli-usage"),
    ("openai-gpt6-astra-model", "GPT-6 Astra model card; related but distinct from requested GPT 6", "https://developers.openai.com/api/docs/models/gpt-6-astra"),
    ("openai-gpt6-astra-guidance", "GPT-6 Astra prompting, reasoning, and tools guidance", "https://developers.openai.com/api/docs/guides/latest-model?model=gpt-6-astra"),
    ("openai-cli-model-retrieve", "OpenAI CLI model-identifier reference", "https://developers.openai.com/api/reference/cli/resources/models/methods/retrieve"),
]


def safe_name(value: str) -> str:
    return re.sub(r"[^a-z0-9-]+", "-", value.lower()).strip("-")


def save_one(slug: str, purpose: str, requested_url: str) -> dict:
    page = Fetcher.get(requested_url, impersonate="chrome", timeout=60)
    body = bytes(page.body)
    content_type = str(page.headers.get("content-type", ""))
    suffix = ".pdf" if "pdf" in content_type.lower() or body.startswith(b"%PDF-") else ".html"
    raw_path = SOURCES / f"{safe_name(slug)}{suffix}"
    raw_path.write_bytes(body)

    text_path = None
    if suffix == ".html":
        main = page.css("main").first or page.css("article").first or page.css("body").first
        extracted = main.get_all_text(separator="\n", strip=True) if main else ""
        text_path = SOURCES / f"{safe_name(slug)}.txt"
        text_path.write_text(extracted + "\n", encoding="utf-8")
    elif suffix == ".pdf":
        text_path = SOURCES / f"{safe_name(slug)}.txt"
        subprocess.run(["pdftotext", "-layout", str(raw_path), str(text_path)], check=True)

    record = {
        "slug": slug,
        "purpose": purpose,
        "requested_url": requested_url,
        "canonical_url": str(page.url),
        "retrieved_at_utc": RETRIEVED_AT,
        "http_status": page.status,
        "content_type": content_type,
        "raw_file": raw_path.name,
        "text_file": text_path.name if text_path else None,
        "bytes": len(body),
        "sha256": hashlib.sha256(body).hexdigest(),
        "text_sha256": hashlib.sha256(text_path.read_bytes()).hexdigest() if text_path else None,
        "access": "public, unauthenticated",
    }
    (SOURCES / f"{safe_name(slug)}.metadata.json").write_text(
        json.dumps(record, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    return record


def find_opus_system_card() -> tuple[str | None, str | None]:
    """Return the direct official Opus 5 card link from the saved index response."""
    page = Fetcher.get("https://www.anthropic.com/system-cards", impersonate="chrome", timeout=60)
    anchors = page.css("a")
    for anchor in anchors:
        label = anchor.get_all_text(separator=" ", strip=True).lower()
        href = anchor.attrib.get("href")
        if href and ("opus 5" in label or "opus-5" in href.lower()):
            return urljoin(str(page.url), href), label
    return None, None


def main() -> None:
    SOURCES.mkdir(parents=True, exist_ok=True)
    records = [save_one(*target) for target in TARGETS]
    card_url, card_label = find_opus_system_card()
    if card_url:
        records.append(save_one("anthropic-opus-5-system-card", "Claude Opus 5 system card", card_url))
    else:
        records.append({
            "slug": "anthropic-opus-5-system-card",
            "purpose": "Claude Opus 5 system card",
            "requested_url": None,
            "canonical_url": None,
            "retrieved_at_utc": RETRIEVED_AT,
            "http_status": None,
            "status": "missing-direct-link",
            "note": "The official system-card index was saved but no Opus 5 link was located by the public HTML selector.",
        })
    manifest = {
        "collection": "official model documentation corpus",
        "retrieved_at_utc": RETRIEVED_AT,
        "collector": "Scrapling Fetcher 0.4.15, public unauthenticated requests",
        "robots_checked": ["https://ai.google.dev/robots.txt", "https://docs.anthropic.com/robots.txt", "https://developers.openai.com/robots.txt"],
        "scope_note": "Requested labels are Gemini Flash 3.8, Claude Opus 5, and GPT 6. Official sources identify the first two as Gemini 3.8 Flash / gemini-3.8-flash and Claude Opus 5 / claude-opus-5. OpenAI sources identify GPT-6 Astra / gpt-6-astra; they do not establish a standalone exact model named GPT 6.",
        "records": records,
    }
    (ROOT / "MANIFEST.json").write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
