#!/usr/bin/env python3
"""Acquire the public Midnight documentation corpus through its Markdown routes.

The crawler is deliberately limited to docs.midnight.network.  It discovers pages
from the live llms.txt and sitemap.xml endpoints, checks every candidate against
robots.txt, writes one immutable response and receipt per URL, and resumes by
validating existing response hashes.  It does not use cookies, authentication,
browser challenges, or anti-bot bypasses.
"""

from __future__ import annotations

import argparse
import hashlib
import html
import json
import re
import threading
import time
import urllib.robotparser
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime, timezone
from pathlib import Path
from urllib.parse import unquote, urljoin, urlsplit, urlunsplit

from scrapling.fetchers import Fetcher


HOST = "docs.midnight.network"
BASE = f"https://{HOST}/"
DISCOVERY_URLS = (
    urljoin(BASE, "robots.txt"),
    urljoin(BASE, "sitemap.xml"),
    urljoin(BASE, "llms.txt"),
)
USER_AGENT = "MoriartyResearchDocsCrawler/1.0"
LINK_RE = re.compile(r"\[[^\]]*\]\(([^)\s]+)(?:\s+[^)]*)?\)")
LOC_RE = re.compile(r"<loc>\s*(.*?)\s*</loc>", re.I | re.S)


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def write_bytes_new(path: Path, data: bytes) -> None:
    """Create a raw artifact once; never replace an evidence byte."""
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("xb") as stream:
        stream.write(data)


def write_text_new(path: Path, data: str) -> None:
    """Create a raw text artifact once; never replace an evidence record."""
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("x", encoding="utf-8") as stream:
        stream.write(data)


def load_cached_response(response_path: Path, receipt_path: Path, requested_url: str) -> dict:
    """Validate and return a copy of an existing immutable response receipt."""
    if not response_path.is_file() or not receipt_path.is_file():
        raise RuntimeError(
            f"incomplete existing capture for {requested_url}; use a fresh dated raw directory"
        )
    body = response_path.read_bytes()
    receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
    if receipt.get("requested_url") != requested_url:
        raise RuntimeError(f"requested URL mismatch in immutable receipt {receipt_path}")
    if receipt.get("sha256") != sha256(body) or receipt.get("byte_count") != len(body):
        raise RuntimeError(
            f"corrupt existing response or receipt for {requested_url}; use a fresh dated raw directory"
        )
    result = dict(receipt)
    result["resume_status"] = "reused-verified"
    return result


def normalize(url: str) -> str | None:
    parts = urlsplit(urljoin(BASE, html.unescape(url.strip())))
    if parts.scheme != "https" or parts.hostname != HOST:
        return None
    path = re.sub(r"/{2,}", "/", unquote(parts.path)) or "/"
    return urlunsplit(("https", HOST, path, parts.query, ""))


def output_stem(url: str) -> str:
    parts = urlsplit(url)
    path = parts.path.strip("/") or "index"
    if path.endswith(".md"):
        path = path[:-3]
    safe = re.sub(r"[^A-Za-z0-9._/-]+", "_", path)
    if parts.query:
        safe += "__q_" + hashlib.sha256(parts.query.encode()).hexdigest()[:12]
    return safe


class RateGate:
    def __init__(self, interval: float) -> None:
        self.interval = interval
        self.next_at = 0.0
        self.lock = threading.Lock()

    def wait(self) -> None:
        with self.lock:
            now = time.monotonic()
            pause = max(0.0, self.next_at - now)
            self.next_at = max(now, self.next_at) + self.interval
        if pause:
            time.sleep(pause)


def fetch(url: str, gate: RateGate, retries: int = 2) -> dict:
    last_error = None
    for attempt in range(retries + 1):
        gate.wait()
        started = utc_now()
        try:
            page = Fetcher.get(
                url,
                impersonate="chrome",
                timeout=30,
                headers={"User-Agent": USER_AGENT, "Accept": "text/markdown,text/plain,text/html,application/xml;q=0.9,*/*;q=0.1"},
            )
            body = bytes(page.body)
            headers = {str(k).lower(): str(v) for k, v in dict(page.headers).items()}
            status = int(page.status)
            result = {
                "requested_url": url,
                "canonical_url": str(page.url),
                "retrieval_utc": started,
                "http_status": status,
                "content_type": headers.get("content-type"),
                "last_modified": headers.get("last-modified"),
                "etag": headers.get("etag"),
                "sha256": sha256(body),
                "byte_count": len(body),
                "body": body,
                "error": None,
                "attempts": attempt + 1,
            }
            if status not in (429, 500, 502, 503, 504) or attempt == retries:
                return result
        except Exception as exc:  # retain failures as evidence
            last_error = f"{type(exc).__name__}: {exc}"
            if attempt == retries:
                break
        time.sleep(2**attempt)
    return {
        "requested_url": url,
        "canonical_url": None,
        "retrieval_utc": utc_now(),
        "http_status": None,
        "content_type": None,
        "last_modified": None,
        "etag": None,
        "sha256": None,
        "byte_count": 0,
        "body": b"",
        "error": last_error or "request failed",
        "attempts": retries + 1,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--raw-dir", type=Path, required=True)
    parser.add_argument("--evidence-dir", type=Path, required=True)
    parser.add_argument("--workers", type=int, default=2)
    parser.add_argument("--delay", type=float, default=0.5, help="minimum seconds between all requests")
    parser.add_argument("--robots-fallback", type=Path)
    args = parser.parse_args()
    if args.workers < 1 or args.workers > 4 or args.delay < 0.25:
        parser.error("workers must be 1..4 and delay must be at least 0.25 seconds")

    raw_dir = args.raw_dir.resolve()
    evidence_dir = args.evidence_dir.resolve()
    cached_run = raw_dir.exists() and any(raw_dir.rglob("*"))
    responses_dir = raw_dir / "responses"
    markdown_dir = raw_dir / "markdown"
    text_dir = raw_dir / "text"
    receipt_dir = raw_dir / "receipts"
    for directory in (responses_dir, markdown_dir, text_dir, receipt_dir, evidence_dir):
        directory.mkdir(parents=True, exist_ok=True)

    gate = RateGate(args.delay)
    discovery = {}
    for url in DISCOVERY_URLS:
        name = urlsplit(url).path.rsplit("/", 1)[-1]
        path = responses_dir / name
        receipt_path = receipt_dir / f"{name}.json"
        if cached_run:
            result = load_cached_response(path, receipt_path, url)
        else:
            result = fetch(url, gate)
            body = result.pop("body")
            result["local_path"] = str(path.relative_to(raw_dir.parent.parent))
            result["source_class"] = "official documentation discovery endpoint"
            result["acquisition_method"] = "Scrapling 0.4.15 Fetcher"
            write_bytes_new(path, body)
            write_text_new(receipt_path, json.dumps(result, indent=2, sort_keys=True) + "\n")
        discovery[name] = result

    robots_text = ""
    robots_source = "live"
    robots_path = responses_dir / "robots.txt"
    if discovery["robots.txt"]["http_status"] == 200:
        robots_text = robots_path.read_text(errors="replace")
    elif args.robots_fallback and args.robots_fallback.is_file():
        robots_text = args.robots_fallback.read_text(errors="replace")
        robots_source = f"repository fallback: {args.robots_fallback.resolve()}"
    rp = urllib.robotparser.RobotFileParser()
    rp.set_url(DISCOVERY_URLS[0])
    rp.parse(robots_text.splitlines())

    candidates: set[str] = set()
    llms_path = responses_dir / "llms.txt"
    if discovery["llms.txt"]["http_status"] == 200:
        for link in LINK_RE.findall(llms_path.read_text(errors="replace")):
            normalized = normalize(link)
            if normalized:
                candidates.add(normalized)
    sitemap_path = responses_dir / "sitemap.xml"
    if discovery["sitemap.xml"]["http_status"] == 200:
        for link in LOC_RE.findall(sitemap_path.read_text(errors="replace")):
            normalized = normalize(link)
            if normalized:
                candidates.add(normalized)

    # Published .md routes are full documentation representations and avoid the
    # site's HTML security checkpoint. Convert sitemap HTML routes to that form.
    markdown_urls: set[str] = set()
    for url in candidates:
        parts = urlsplit(url)
        if parts.path.endswith(".md"):
            markdown_urls.add(url)
        elif not re.search(r"\.[A-Za-z0-9]{1,8}$", parts.path):
            path = parts.path.rstrip("/") or "/index"
            markdown_urls.add(urlunsplit((parts.scheme, parts.netloc, path + ".md", parts.query, "")))

    priority_terms = ("installation", "networks-and-environments", "node-endpoints", "acquire-tokens", "faucet", "dust")
    queue = sorted(markdown_urls, key=lambda u: (not any(t in u.lower() for t in priority_terms), u))
    allowed, excluded = [], []
    for url in queue:
        if robots_text and not rp.can_fetch(USER_AGENT, url):
            excluded.append({"url": url, "reason": "disallowed by robots policy"})
        else:
            allowed.append(url)

    receipts: list[dict] = []
    receipt_lock = threading.Lock()

    def acquire(url: str) -> dict:
        stem = output_stem(url)
        raw_path = markdown_dir / f"{stem}.md"
        text_path = text_dir / f"{stem}.txt"
        receipt_path = receipt_dir / f"{stem}.json"
        artifacts_exist = raw_path.exists() or text_path.exists() or receipt_path.exists()
        if cached_run or artifacts_exist:
            if not receipt_path.is_file():
                raise RuntimeError(
                    f"incomplete existing capture for {url}; use a fresh dated raw directory"
                )
            old = json.loads(receipt_path.read_text(encoding="utf-8"))
            if old.get("requested_url") != url:
                raise RuntimeError(f"requested URL mismatch in immutable receipt {receipt_path}")
            if old.get("http_status") != 200:
                if raw_path.exists() or text_path.exists():
                    raise RuntimeError(f"unexpected page body for failed immutable receipt {receipt_path}")
                reused = dict(old)
                reused["resume_status"] = "reused-recorded-failure"
                return reused
            if not raw_path.is_file() or not text_path.is_file():
                raise RuntimeError(
                    f"incomplete existing capture for {url}; use a fresh dated raw directory"
                )
            raw_body = raw_path.read_bytes()
            text_body = text_path.read_bytes()
            expected_text = raw_body.decode("utf-8", "replace").encode("utf-8")
            if old.get("sha256") != sha256(raw_body) or old.get("byte_count") != len(raw_body):
                raise RuntimeError(
                    f"corrupt existing Markdown or receipt for {url}; use a fresh dated raw directory"
                )
            if text_body != expected_text:
                raise RuntimeError(
                    f"corrupt existing text copy for {url}; use a fresh dated raw directory"
                )
            reused = dict(old)
            reused["resume_status"] = "reused-verified"
            reused["text_sha256"] = sha256(text_body)
            return reused
        result = fetch(url, gate)
        body = result.pop("body")
        if body:
            # Markdown is itself a complete textual representation. Preserve a
            # second .txt copy for tools that do not index Markdown.
            text_body = body.decode("utf-8", "replace").encode("utf-8")
            write_bytes_new(raw_path, body)
            write_bytes_new(text_path, text_body)
        result.update({
            "local_markdown_path": str(raw_path.relative_to(raw_dir.parent.parent)) if body else None,
            "local_text_path": str(text_path.relative_to(raw_dir.parent.parent)) if body else None,
            "source_class": "official published documentation",
            "acquisition_method": "Scrapling 0.4.15 Fetcher; public .md route; no cookies/authentication",
            "relevant_version": "current published documentation snapshot; page-specific versions remain in content",
            "coverage_limitations": "Published Markdown route; source HTML counterpart is separately blocked by a Vercel 429 security checkpoint.",
            "resume_status": "fetched",
            "text_sha256": sha256(text_body) if body else None,
        })
        write_text_new(receipt_path, json.dumps(result, indent=2, sort_keys=True) + "\n")
        return result

    with ThreadPoolExecutor(max_workers=args.workers) as pool:
        futures = {pool.submit(acquire, url): url for url in allowed}
        for index, future in enumerate(as_completed(futures), 1):
            receipt = future.result()
            with receipt_lock:
                receipts.append(receipt)
            if index % 100 == 0:
                print(f"completed {index}/{len(allowed)}", flush=True)

    receipts.sort(key=lambda item: item["requested_url"])
    status_counts: dict[str, int] = {}
    canonical_groups: dict[str, list[str]] = {}
    digest_groups: dict[str, list[str]] = {}
    for receipt in receipts:
        key = str(receipt.get("http_status") or "error")
        status_counts[key] = status_counts.get(key, 0) + 1
        canonical = receipt.get("canonical_url")
        if canonical:
            canonical_groups.setdefault(canonical, []).append(receipt["requested_url"])
        digest = receipt.get("sha256")
        if digest:
            digest_groups.setdefault(digest, []).append(receipt["requested_url"])

    manifest = {
        "schema_version": 1,
        "generated_utc": utc_now(),
        "scope": {"scheme": "https", "host": HOST, "representations": ["markdown", "text"]},
        "discovery": discovery,
        "robots_policy_source": robots_source,
        "robots_policy_sha256": sha256(robots_text.encode()) if robots_text else None,
        "counts": {
            "discovered_same_host": len(candidates),
            "discovered_markdown_routes": len(markdown_urls),
            "robots_allowed": len(allowed),
            "robots_excluded": len(excluded),
            "fetched_or_reused": len(receipts),
            "successful_200": sum(r.get("http_status") == 200 for r in receipts),
            "failed": sum(r.get("http_status") != 200 for r in receipts),
        },
        "status_counts": status_counts,
        "excluded": excluded,
        "failed": [r for r in receipts if r.get("http_status") != 200],
        "redirect_deduplication": {k: v for k, v in canonical_groups.items() if len(v) > 1},
        "content_deduplication": {k: v for k, v in digest_groups.items() if len(v) > 1},
        "receipts": receipts,
        "limitations": [
            "The live robots.txt and sitemap.xml endpoints may return a Vercel 429 security checkpoint; those responses are preserved.",
            "When live robots.txt is unavailable, the explicitly supplied pinned-repository robots.txt is used and identified as a fallback.",
            "The crawl is restricted to same-host URLs published by llms.txt or sitemap.xml; external assets and linked off-host sites are excluded.",
            "HTML document routes are not challenged or bypassed. Full official Markdown and byte-for-byte textual copies are preserved.",
        ],
    }
    (evidence_dir / "coverage-manifest.json").write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")
    print(json.dumps(manifest["counts"], sort_keys=True))
    return 0 if manifest["counts"]["failed"] == 0 else 2


if __name__ == "__main__":
    raise SystemExit(main())
