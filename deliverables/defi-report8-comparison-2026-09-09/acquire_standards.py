#!/usr/bin/env python3
"""Capture every explicitly named ERC/EIP in report 8; no recursive crawl."""
import concurrent.futures
import datetime
import hashlib
import json
import re
import time
from pathlib import Path
from urllib.parse import urlparse
from urllib.robotparser import RobotFileParser

from scrapling.fetchers import Fetcher

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
REPORT = ROOT / '.raw/captured/02e57f7b7616b730a7be59cb7cdc63e3753b007c2d0806393d63a35a9a6c1045.md'
OUT = HERE / 'sources/standards'


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def digest(data):
    return hashlib.sha256(data).hexdigest()


def main():
    ids = sorted(set(map(int, re.findall(r'\b(?:ERC|EIP)[-‑– ](\d{1,5})\b', REPORT.read_text()))))
    OUT.mkdir(parents=True, exist_ok=True)
    if (OUT / 'manifest.json').exists():
        raise SystemExit('Existing capture preserved; use a new dated output for refresh.')
    robots = Fetcher.get('https://eips.ethereum.org/robots.txt', timeout=25, follow_redirects=False)
    rule = RobotFileParser()
    if robots.status == 200:
        rule.parse(robots.body.decode('utf-8').splitlines())
    elif robots.status != 404:
        raise SystemExit(f'Robots availability unresolved: {robots.status}')
    (OUT / 'robots-response.txt').write_bytes(robots.body)

    def capture(number):
        url = f'https://eips.ethereum.org/EIPS/eip-{number}'
        row = {'number': number, 'requested_url': url, 'retrieved_at': now(),
               'status': 'unavailable', 'historical_cutoff_verified': False,
               'deployment_or_conformance_tested': False}
        if robots.status == 200 and not rule.can_fetch('*', url):
            return {**row, 'status': 'robots-disallowed'}
        try:
            time.sleep(0.25)
            page = Fetcher.get(url, timeout=30, follow_redirects=False)
            row.update(http_status=page.status, canonical_url=str(page.url))
            if page.status != 200 or urlparse(str(page.url)).hostname != 'eips.ethereum.org':
                return row
            main = page.css('main')
            if not main:
                return {**row, 'status': 'missing-main-selector'}
            text = main[0].get_all_text(separator='\n', strip=True)
            html_name, text_name = f'eip-{number}.html', f'eip-{number}.txt'
            (OUT / html_name).write_bytes(page.body)
            (OUT / text_name).write_text(text + '\n')
            title = page.css('h1')[0].get_all_text(strip=True)
            header = text.split('Table of Contents')[0]
            formal = re.search(r'\b(Final|Draft|Review|Last Call|Stagnant|Withdrawn|Living)\b', header)
            refs = []
            for tr in page.css('tr'):
                parts = tr.css('th,td')
                if len(parts) >= 2 and parts[0].get_all_text(strip=True) == 'Requires':
                    refs = sorted(set(map(int, re.findall(r'(?:EIP|ERC)-(\d+)', parts[1].get_all_text()))))
            row.update(status='captured', title=title,
                       formal_status=formal.group(1) if formal else None,
                       requires=refs, html=html_name, text=text_name,
                       html_sha256=digest(page.body), text_sha256=digest((text+'\n').encode()),
                       bytes=len(page.body))
            return row
        except Exception as exc:
            return {**row, 'error_type': type(exc).__name__}

    with concurrent.futures.ThreadPoolExecutor(max_workers=3) as pool:
        rows = list(pool.map(capture, ids))
    manifest = {'schema': 'moriarty.report8-standards-capture.v1', 'generated_at': now(),
                'report_sha256': digest(REPORT.read_bytes()),
                'scope': 'Every explicitly named ERC/EIP; dependency-only standards are graph stubs, not extra captures.',
                'robots_http_status': robots.status, 'records': rows}
    (OUT / 'manifest.json').write_text(json.dumps(manifest, indent=2)+'\n')
    print(json.dumps({'requested': len(rows), 'captured': sum(r['status']=='captured' for r in rows),
                      'unavailable': [r['number'] for r in rows if r['status']!='captured']}))


if __name__ == '__main__':
    main()
