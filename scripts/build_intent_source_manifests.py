#!/usr/bin/env python3
"""Build and verify deterministic CAKE and NEAR Intents acquisition manifests."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
from xml.etree import ElementTree


ROOT = Path(__file__).resolve().parents[1]
RAW = ROOT / "raw" / "sources"
EVIDENCE = ROOT / "evidence"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_receipts(directory: Path) -> list[dict[str, object]]:
    records: list[dict[str, object]] = []
    for receipt_path in sorted(directory.rglob("*.receipt.json")):
        receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
        output = ROOT / str(receipt["output"])
        if not output.is_file():
            raise SystemExit(f"missing receipt output: {output}")
        actual = sha256(output)
        if actual != receipt["sha256"]:
            raise SystemExit(f"receipt digest mismatch: {output}")
        if output.stat().st_size != receipt["bytes"]:
            raise SystemExit(f"receipt byte count mismatch: {output}")
        records.append(
            {
                "requested_url": receipt["requested_url"],
                "final_url": receipt["final_url"],
                "retrieved_at_utc": receipt["retrieved_at_utc"],
                "http_status": receipt["http_status"],
                "content_type": receipt["content_type"],
                "bytes": receipt["bytes"],
                "sha256": receipt["sha256"],
                "local_path": output.relative_to(ROOT).as_posix(),
                "receipt_path": receipt_path.relative_to(ROOT).as_posix(),
            }
        )
    return records


def write_json(path: Path, value: object) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def build_near() -> None:
    directory = RAW / "near-intents-docs-2026-09-03"
    receipts = load_receipts(directory)
    sitemap = ElementTree.fromstring((directory / "sitemap.txt").read_text(encoding="utf-8"))
    namespace = {"s": "http://www.sitemaps.org/schemas/sitemap/0.9"}
    pages = []
    for entry in sitemap.findall("s:url", namespace):
        url = entry.findtext("s:loc", namespaces=namespace)
        modified = entry.findtext("s:lastmod", namespaces=namespace)
        if not url:
            raise SystemExit("sitemap entry has no URL")
        relative = url.removeprefix("https://docs.near-intents.org/") + ".md"
        local = directory / relative
        if not local.is_file():
            raise SystemExit(f"sitemap page not acquired: {url}")
        pages.append({"url": url, "last_modified": modified, "local_path": local.relative_to(ROOT).as_posix()})
    if len(pages) != 68:
        raise SystemExit(f"expected 68 NEAR documentation pages, found {len(pages)}")
    result = {
        "schema_version": "1.0.0",
        "corpus": "NEAR Intents official documentation",
        "retrieval_date": "2026-09-03",
        "acquisition_method": "Scrapling 0.4.15 static and dynamic fetchers",
        "coverage": {
            "sitemap_pages_expected": 68,
            "sitemap_pages_acquired": len(pages),
            "successful_receipts": len(receipts),
            "openapi_documents": 2,
        },
        "authority_notes": [
            "docs.near-intents.org is linked by the official docs.near.org overview.",
            "near-intents.org displays a migration notice; it is not the documentation corpus.",
            "Narrative pages, OpenAPI documents, source repositories, and deployed contracts are separate evidence streams.",
        ],
        "limitations": [
            "This manifest proves acquisition and byte identity, not correspondence with deployed services.",
            "Linked audit reports and every linked source repository require separate acquisition and version pins.",
        ],
        "sitemap_pages": pages,
        "receipts": receipts,
    }
    write_json(EVIDENCE / "near-intents-docs-acquisition-2026-09-03.json", result)


def build_cake() -> None:
    directory = RAW / "cake-working-group-2026-09-03"
    receipts = load_receipts(directory)
    dispositions = [
        ("https://frontier.tech/", "navigation", "fetched"),
        ("https://frontier.tech/cake-working-group", "first-party CAKE", "fetched"),
        ("https://frontier.tech/the-cake-framework", "first-party CAKE", "fetched"),
        ("https://www.erc4337.io/", "primary standard documentation", "fetched with redirect"),
        ("https://erc7579.com/", "primary standard documentation", "fetched"),
        ("https://ethereum-magicians.org/t/erc-7555-single-sign-on-for-account-discovery/16536", "primary-standard discussion", "fetched"),
        ("https://eips.ethereum.org/EIPS/eip-6900", "primary standard", "fetched"),
        ("https://eips.ethereum.org/EIPS/eip-7521", "primary standard", "fetched"),
        ("https://www.xerc20.com/", "primary standard site", "TLS handshake failed; no content acquired"),
        ("https://0yqvvfq7bc1.typeform.com/to/MkMGs2wX", "membership form", "fetched; excluded from semantic evidence"),
        ("https://lu.ma/ChADay", "event", "fetched; excluded from semantic evidence"),
        ("https://lu.ma/CA-standards-workshop?utm_source=working-group-post", "event", "fetched; excluded from semantic evidence"),
        ("https://lu.ma/CA-standards-workshop-london?utm_source=working-group-post", "event", "fetched; excluded from semantic evidence"),
        ("https://hackmd.io/DWu3qD-PQCOOn5QaEkuT5w", "first-party working notes", "fetched through public Markdown endpoint"),
    ]
    result = {
        "schema_version": "1.0.0",
        "corpus": "CAKE Working Group public one-hop link corpus",
        "retrieval_date": "2026-09-03",
        "acquisition_method": "Scrapling 0.4.15 static fetcher",
        "coverage": {
            "landing_page_unique_links": len(dispositions),
            "successful_receipts": len(receipts),
            "failed_links": 1,
        },
        "link_dispositions": [
            {"url": url, "classification": classification, "disposition": disposition}
            for url, classification, disposition in dispositions
        ],
        "limitations": [
            "xerc20.com rejected the available TLS handshake before an HTTP response; no bypass was attempted.",
            "Workshop notes are incomplete historical discussion and are not a ratified protocol specification.",
        ],
        "receipts": receipts,
    }
    write_json(EVIDENCE / "cake-working-group-acquisition-2026-09-03.json", result)


if __name__ == "__main__":
    build_near()
    build_cake()
    print("verified 68 NEAR pages and 14 CAKE link dispositions")
