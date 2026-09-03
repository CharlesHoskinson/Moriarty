#!/usr/bin/env python3
"""Acquire the public ACTUS documentation corpus and record repository pins."""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
from collections import Counter
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timezone
from pathlib import Path
from urllib.parse import unquote, urlsplit, urlunsplit
from xml.etree import ElementTree

from scrapling.core.shell import Convertor
from scrapling.fetchers import Fetcher


ROOT = Path(__file__).resolve().parents[1]
RAW = ROOT / "raw" / "sources" / "actus-public-2026-09-03"
MANIFEST = ROOT / "evidence" / "actus-public-source-acquisition-2026-09-03.json"
MAIN_ORIGIN = "https://www.actusfrf.org"
DOCS_ORIGIN = "https://documentation.actusfrf.org"
PLACEHOLDER_ORIGIN = "https://your-docusaurus-site.example.com"
EXPECTED_ORG_REPOSITORIES = (
    "actus-core-license",
    "actus-dictionary",
    "actus-distributions",
    "actus-docker-networks",
    "actus-resources",
    "actus-riskservice",
    "actus-service",
    "actus-techspecs",
    "actus-tests",
    "actus-userguides",
    "actus-webapp",
)
EXECUTABLE_CONTRACT_TYPES = (
    "PAM",
    "LAM",
    "LAX",
    "NAM",
    "ANN",
    "CLM",
    "UMP",
    "CSH",
    "STK",
    "COM",
    "FXOUT",
    "SWPPV",
    "SWAPS",
    "CAPFL",
    "OPTNS",
    "FUTUR",
    "CEG",
    "CEC",
)


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat()


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def write_json(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def sitemap_urls(body: bytes) -> list[str]:
    root = ElementTree.fromstring(body)
    primary = [
        *root.findall("./{*}sitemap/{*}loc"),
        *root.findall("./{*}url/{*}loc"),
    ]
    return [element.text.strip() for element in primary if element.text]


def sitemap_auxiliary_urls(body: bytes) -> list[str]:
    root = ElementTree.fromstring(body)
    primary = set(sitemap_urls(body))
    return [
        element.text.strip()
        for element in root.findall(".//{*}loc")
        if element.text and element.text.strip() not in primary
    ]


def effective_docs_url(source_url: str) -> str:
    parts = urlsplit(source_url)
    placeholder = urlsplit(PLACEHOLDER_ORIGIN)
    if (parts.scheme, parts.netloc) != (placeholder.scheme, placeholder.netloc):
        return source_url
    target = urlsplit(DOCS_ORIGIN)
    return urlunsplit((target.scheme, target.netloc, parts.path, parts.query, parts.fragment))


def output_stem(index: int, source_url: str) -> str:
    parts = urlsplit(source_url)
    label = unquote(parts.path).strip("/") or "root"
    label = re.sub(r"[^A-Za-z0-9._-]+", "-", label).strip("-").lower()
    label = label[-100:] or "root"
    short_hash = hashlib.sha256(source_url.encode("utf-8")).hexdigest()[:12]
    return f"{index:04d}-{label}-{short_hash}"


def response_headers(page: object) -> dict[str, str | None]:
    headers = page.headers
    return {
        "content_type": headers.get("content-type"),
        "etag": headers.get("etag"),
        "last_modified": headers.get("last-modified"),
    }


def acquire(
    source_url: str,
    effective_url: str,
    directory: Path,
    index: int,
    source_class: str,
    extract_markdown: bool,
) -> dict[str, object]:
    directory.mkdir(parents=True, exist_ok=True)
    stem = output_stem(index, source_url)
    receipt_path = directory / f"{stem}.receipt.json"
    record: dict[str, object] = {
        "source_url": source_url,
        "effective_url": effective_url,
        "source_class": source_class,
        "retrieved_at_utc": utc_now(),
        "acquisition_method": "Scrapling 0.4.15 static fetcher",
        "host_substitution": source_url != effective_url,
    }
    try:
        page = Fetcher.get(effective_url, timeout=60, block_ads=True)
        body = bytes(page.body)
        content_type = page.headers.get("content-type", "")
        extension = ".xml" if "xml" in content_type else ".txt" if "text/plain" in content_type else ".html"
        raw_path = directory / f"{stem}{extension}"
        raw_path.write_bytes(body)
        record.update(
            {
                "final_url": str(page.url),
                "http_status": page.status,
                "raw_path": raw_path.relative_to(ROOT).as_posix(),
                "raw_bytes": len(body),
                "raw_sha256": digest(body),
                **response_headers(page),
            }
        )
        if extract_markdown and "html" in content_type:
            markdown_path = directory / f"{stem}.md"
            Convertor.write_content_to_file(page, str(markdown_path), main_content_only=True)
            markdown = markdown_path.read_bytes()
            record.update(
                {
                    "markdown_path": markdown_path.relative_to(ROOT).as_posix(),
                    "markdown_bytes": len(markdown),
                    "markdown_sha256": digest(markdown),
                }
            )
        record["disposition"] = "fetched" if page.status < 400 else "http-error"
    except Exception as error:  # Preserve a specific failure instead of losing the URL.
        record.update(
            {
                "disposition": "fetch-error",
                "error_type": type(error).__name__,
                "error": str(error),
            }
        )
    record["receipt_path"] = receipt_path.relative_to(ROOT).as_posix()
    write_json(receipt_path, record)
    return record


def acquire_many(
    entries: list[tuple[str, str]], directory: Path, source_class: str
) -> list[dict[str, object]]:
    def task(item: tuple[int, tuple[str, str]]) -> dict[str, object]:
        index, (source_url, effective_url) = item
        return acquire(source_url, effective_url, directory, index, source_class, True)

    with ThreadPoolExecutor(max_workers=4) as executor:
        return list(executor.map(task, enumerate(entries, start=1)))


def git(repository: Path, *args: str) -> str:
    return subprocess.check_output(
        ["git", "-C", str(repository), *args], text=True, stderr=subprocess.DEVNULL
    ).strip()


def repository_record(name: str, path: Path, scope_path: str = ".") -> dict[str, object]:
    tracked = git(path, "ls-files").splitlines()
    source_suffixes = {".hs", ".java", ".js", ".jsx", ".mjs", ".purs", ".py", ".ts", ".tsx"}
    documentation_suffixes = {".md", ".mdx", ".adoc", ".tex"}
    return {
        "name": name,
        "local_path": path.relative_to(ROOT).as_posix(),
        "scope_path": scope_path,
        "remote": git(path, "remote", "get-url", "origin"),
        "branch": git(path, "branch", "--show-current"),
        "full_commit": git(path, "rev-parse", "HEAD"),
        "dirty": bool(git(path, "status", "--porcelain")),
        "submodules": git(path, "submodule", "status", "--recursive"),
        "tracked_files": len(tracked),
        "source_files": sum(Path(item).suffix.lower() in source_suffixes for item in tracked),
        "documentation_files": sum(Path(item).suffix.lower() in documentation_suffixes for item in tracked),
    }


def repository_inventory() -> dict[str, object]:
    official_root = ROOT / "repos" / "actusfrf"
    official = [repository_record(name, official_root / name) for name in EXPECTED_ORG_REPOSITORIES]
    comparative = [
        repository_record("marlowe-lang/actus-core", ROOT / "repos" / "actus-core"),
        repository_record("marlowe-lang/marlowe-actus-labs", ROOT / "repos" / "marlowe-actus-labs"),
        repository_record(
            "marlowe-lang/marlowe-cardano",
            ROOT / "repos" / "marlowe-cardano",
            "marlowe-actus",
        ),
    ]
    return {"official_public": official, "comparative_public": comparative}


def reference_fixture_inventory() -> dict[str, object]:
    tests_dir = ROOT / "repos" / "actusfrf" / "actus-tests" / "tests"
    counts: dict[str, int] = {}
    contract_counts: Counter[str] = Counter()
    result_fields: set[str] = set()
    event_types: set[str] = set()
    for path in sorted(tests_dir.glob("actus-tests-*.json")):
        cases = json.loads(path.read_text(encoding="utf-8"))
        counts[path.name] = len(cases)
        for case in cases.values():
            contract_counts[case["terms"]["contractType"]] += 1
            for event in case["results"]:
                result_fields.update(event)
                event_types.add(event["eventType"])
    return {
        "files": counts,
        "total_fixtures": sum(counts.values()),
        "contract_fixtures": sum(count for name, count in counts.items() if name != "actus-tests-ad0.json"),
        "analysis_date_fixtures": counts.get("actus-tests-ad0.json", 0),
        "contract_type_counts": dict(sorted(contract_counts.items())),
        "executable_contract_types": list(EXECUTABLE_CONTRACT_TYPES),
        "result_fields": sorted(result_fields),
        "event_types": sorted(event_types),
    }


def dictionary_inventory() -> dict[str, object]:
    path = ROOT / "repos" / "actusfrf" / "actus-dictionary" / "actus-dictionary.json"
    dictionary = json.loads(path.read_text(encoding="utf-8"))
    taxonomy = dictionary["taxonomy"]
    return {
        "version": dictionary["version"],
        "taxonomy_rows": len(taxonomy),
        "terms": len(dictionary["terms"]),
        "states": len(dictionary["states"]),
        "event_record_fields": len(dictionary["event"]),
        "applicability_records": len(dictionary["applicability"]),
        "contract_reference_fields": len(dictionary["contractReference"]),
    }


def main() -> None:
    if RAW.exists() or MANIFEST.exists():
        raise SystemExit("ACTUS acquisition output already exists; preserve it as immutable evidence")

    metadata = RAW / "metadata"
    main_robots = acquire(
        f"{MAIN_ORIGIN}/robots.txt",
        f"{MAIN_ORIGIN}/robots.txt",
        metadata,
        1,
        "robots",
        False,
    )
    main_sitemap = acquire(
        f"{MAIN_ORIGIN}/sitemap.xml",
        f"{MAIN_ORIGIN}/sitemap.xml",
        metadata,
        2,
        "sitemap-index",
        False,
    )
    main_sitemap_body = (ROOT / str(main_sitemap["raw_path"])).read_bytes()
    child_sitemap_urls = sitemap_urls(main_sitemap_body)
    child_sitemaps = [
        acquire(url, url, metadata / "main-sitemaps", index, "sitemap", False)
        for index, url in enumerate(child_sitemap_urls, start=1)
    ]
    main_page_urls: list[str] = []
    main_auxiliary_urls: list[str] = []
    for record in child_sitemaps:
        if record["disposition"] != "fetched":
            continue
        sitemap_body = (ROOT / str(record["raw_path"])).read_bytes()
        main_page_urls.extend(sitemap_urls(sitemap_body))
        main_auxiliary_urls.extend(sitemap_auxiliary_urls(sitemap_body))
    main_page_urls = list(dict.fromkeys(main_page_urls))
    main_auxiliary_urls = list(dict.fromkeys(main_auxiliary_urls))
    main_urls = list(dict.fromkeys([*main_page_urls, *main_auxiliary_urls]))
    main_pages = acquire_many([(url, url) for url in main_urls], RAW / "main-site", "main-site-page")

    docs_robots = acquire(
        f"{DOCS_ORIGIN}/robots.txt",
        f"{DOCS_ORIGIN}/robots.txt",
        metadata,
        3,
        "robots",
        False,
    )
    docs_sitemap = acquire(
        f"{DOCS_ORIGIN}/sitemap.xml",
        f"{DOCS_ORIGIN}/sitemap.xml",
        metadata,
        4,
        "sitemap",
        False,
    )
    docs_urls = sitemap_urls((ROOT / str(docs_sitemap["raw_path"])).read_bytes())
    docs_entries = [(url, effective_docs_url(url)) for url in docs_urls]
    docs_pages = acquire_many(docs_entries, RAW / "documentation", "documentation-page")

    records = [
        main_robots,
        main_sitemap,
        *child_sitemaps,
        *main_pages,
        docs_robots,
        docs_sitemap,
        *docs_pages,
    ]
    dispositions = Counter(str(record["disposition"]) for record in records)
    http_statuses = Counter(str(record.get("http_status", "none")) for record in records)
    manifest = {
        "schema_version": "1.0.0",
        "corpus": "ACTUS public website, documentation, reference fixtures, and repository source",
        "retrieval_date": "2026-09-03",
        "generated_at_utc": utc_now(),
        "acquisition_method": "Scrapling 0.4.15 static fetcher with AI-targeted Markdown extraction",
        "coverage": {
            "main_child_sitemaps": len(child_sitemaps),
            "main_sitemap_pages": len(main_page_urls),
            "main_sitemap_auxiliary_unique_urls": len(main_auxiliary_urls),
            "main_sitemap_total_unique_urls": len(main_urls),
            "documentation_sitemap_entries": len(docs_urls),
            "documentation_placeholder_rewrites": sum(source != effective for source, effective in docs_entries),
            "total_http_dispositions": len(records),
            "dispositions": dict(sorted(dispositions.items())),
            "http_statuses": dict(sorted(http_statuses.items())),
            "entries_without_disposition": sum("disposition" not in record for record in records),
        },
        "robots": {
            "main_site": main_robots,
            "documentation_site": docs_robots,
            "documentation_note": "The documentation origin returned HTTP 404 for robots.txt.",
        },
        "documentation_host_rule": {
            "sitemap_origin": PLACEHOLDER_ORIGIN,
            "retrieval_origin": DOCS_ORIGIN,
            "transformation": "Replace scheme and authority only; preserve path, query, and fragment.",
        },
        "reference_fixture_inventory": reference_fixture_inventory(),
        "dictionary_inventory": dictionary_inventory(),
        "repository_inventory": repository_inventory(),
        "records": records,
        "limitations": [
            "Acquisition proves URL disposition and byte identity, not semantic correctness.",
            "The documentation sitemap publishes a placeholder origin; retrieval used the recorded deterministic origin substitution.",
            "HTTP error pages remain explicit dispositions and are not counted as acquired documentation content.",
            "The official Java actus-core source requires registration and was not acquired or used.",
            "Repository observations apply only to the recorded commits.",
        ],
    }
    write_json(MANIFEST, manifest)
    print(json.dumps(manifest["coverage"], indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
