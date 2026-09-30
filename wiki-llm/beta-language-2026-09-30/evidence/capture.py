"""Bounded public-document capture with Scrapling; never stores headers/cookies."""
import argparse, hashlib, json, pathlib, datetime
from scrapling.fetchers import Fetcher

p = argparse.ArgumentParser()
p.add_argument('url')
p.add_argument('output', help='create-only capture directory')
p.add_argument('--selector', default='main, article, [role="main"], .content')
a = p.parse_args()
out = pathlib.Path(a.output)
out.mkdir(parents=True, exist_ok=False)
now = datetime.datetime.now(datetime.timezone.utc).isoformat()
page = Fetcher.get(a.url, impersonate='chrome', timeout=30)
raw = bytes(page.body)
nodes = page.css(a.selector)
content = '\n\n'.join(n.get_all_text(separator='\n', strip=True) for n in nodes) if nodes else page.get_all_text(separator='\n', strip=True)
title = page.css('title::text').get() or a.url
(out / 'content.txt').write_text(content, encoding='utf-8')
(out / 'response.html').write_bytes(raw)
receipt = {'requested_url': a.url, 'canonical_url': str(page.url), 'title': title, 'publisher': pathlib.PurePosixPath(a.url.split('//',1)[1]).parts[0], 'retrieved_at': now, 'status': page.status, 'payload_sha256': hashlib.sha256(raw).hexdigest(), 'content_sha256': hashlib.sha256(content.encode()).hexdigest(), 'method': 'Scrapling Fetcher 0.4.15', 'selector': a.selector, 'authority': 'official', 'independence_key': a.url.split('//',1)[1].split('/')[0], 'publication_date': None, 'limits': 'HTML/text inspection only; no runtime experiment; no cookies or response headers retained'}
(out / 'receipt.json').write_text(json.dumps(receipt, indent=2)+'\n')
print(json.dumps({'status': page.status, 'characters': len(content), 'receipt': str(out / 'receipt.json')}))
