#!/usr/bin/env python3
"""Acquire a bounded primary-source comparison corpus through Scrapling."""

from __future__ import annotations

import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path

from scrapling.fetchers import Fetcher


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "raw/sources/comparative-primary-2026-09-02"
TARGETS = {
    "aiken-types.html": "https://aiken-lang.org/language-tour/custom-types",
    "aiken-modules.html": "https://aiken-lang.org/language-tour/modules",
    "daml-templates.html": "https://docs.daml.com/daml/reference/templates.html",
    "daml-choices.html": "https://docs.daml.com/daml/reference/choices.html",
    "actus-standard.html": "https://www.actusfrf.org/algorithmic-standard",
    "move-abilities.html": "https://move-book.com/reference/abilities/",
    "move-generics.html": "https://move-book.com/reference/generics.html",
    "scilla-in-depth.html": "https://scilla.readthedocs.io/en/latest/scilla-in-depth.html",
    "pact-reference.html": "https://pact-language.readthedocs.io/en/latest/pact-reference.html",
    "solidity.html": "https://docs.soliditylang.org/en/latest/",
    "vyper.html": "https://docs.vyperlang.org/en/stable/",
    "michelson-reference.html": "https://tezos.gitlab.io/michelson-reference/",
    "archetype.html": "https://docs.archetype-lang.org/",
    "smartpy.html": "https://smartpy.tezos.com/manual/introduction/overview.html",
    "scrypto.html": "https://docs.radixdlt.com/docs/scrypto",
    "reach.html": "https://docs.reach.sh/",
    "simplicity.pdf": "https://blockstream.com/simplicity.pdf",
    "bitml-paper.pdf": "https://arxiv.org/pdf/1711.03028",
}


def main() -> None:
    OUTPUT.mkdir(parents=True, exist_ok=True)
    manifest = {
        "retrieved_at_utc": datetime.now(timezone.utc).isoformat(),
        "fetcher": "Scrapling static",
        "sources": [],
    }
    for filename, url in TARGETS.items():
        try:
            page = Fetcher.get(url, timeout=60, retries=2, stealthy_headers=True)
            content = bytes(page.body)
            destination = OUTPUT / filename
            destination.write_bytes(content)
            row = {
                "requested_url": url,
                "final_url": str(page.url),
                "http_status": page.status,
                "content_type": page.headers.get("content-type"),
                "output": str(destination.relative_to(ROOT)),
                "bytes": len(content),
                "sha256": hashlib.sha256(content).hexdigest(),
            }
        except Exception as error:  # Preserve a failed acquisition as evidence.
            row = {"requested_url": url, "error": f"{type(error).__name__}: {error}"}
        manifest["sources"].append(row)
        print(json.dumps(row, sort_keys=True))
    (OUTPUT / "manifest.json").write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
