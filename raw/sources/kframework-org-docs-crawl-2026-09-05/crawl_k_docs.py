"""Full crawl of kframework.org with Scrapling: every HTML page reachable from
the root (tutorial, user manual, cheat sheet, tools, builtins, pyk docs,
PL tutorial), saving the main text of each page as Markdown-ish plain text.
Output: pages/<slug>.txt, index.json (url, title, bytes, outgoing links),
best_practices.json (pages and paragraphs mentioning practice/pitfall/idiom).
"""
import json, re, time, hashlib
from collections import deque
from urllib.parse import urljoin, urlparse, urldefrag
from pathlib import Path
from scrapling.fetchers import Fetcher

ROOT = 'https://kframework.org/'
MAX_PAGES = 600
OUT = Path(__file__).resolve().parent
seen, queue, index = set(), deque([ROOT]), {}
KEY = re.compile(r'best practice|pitfall|idiom|recommend|should (not|never|always)|avoid|caveat|gotcha|performance|efficien|deprecated', re.I)
practices = []

def slug(url):
    p = urlparse(url).path.strip('/') or 'index'
    s = re.sub(r'[^A-Za-z0-9._-]+', '_', p)[:150]
    return s + '_' + hashlib.sha1(url.encode()).hexdigest()[:6]

while queue and len(index) < MAX_PAGES:
    url = queue.popleft()
    if url in seen: continue
    seen.add(url)
    try:
        page = Fetcher.get(url, impersonate='chrome', timeout=40)
    except Exception as e:
        print('ERR', url, e); continue
    if page.status != 200:
        print('STATUS', page.status, url); continue
    ctype = str(page.headers.get('content-type', '')) if hasattr(page, 'headers') else ''
    if 'html' not in ctype.lower() and not url.endswith(('/', '.html')):
        continue
    title = (page.css('title::text').get() or '').strip()
    main = page.css('main') or page.css('article') or page.css('div.md-content') or page.css('body')
    node = main[0] if main else page
    text = node.get_all_text(separator='\n', strip=True) if hasattr(node, 'get_all_text') else ''
    links = [urldefrag(urljoin(url, h)).url for h in page.css('a::attr(href)').getall()]
    internal = sorted({l for l in links if urlparse(l).netloc == 'kframework.org' and not re.search(r'\.(pdf|png|jpg|jpeg|svg|zip|tar|gz|epub|mobi|ico|css|js)$', l)})
    name = slug(url)
    (OUT / 'pages' / f'{name}.txt').write_text(f'# {title}\n# {url}\n\n{text}\n', encoding='utf-8')
    index[url] = {'title': title, 'file': f'pages/{name}.txt', 'bytes': len(text.encode()), 'links': len(links)}
    for para in re.split(r'\n{2,}', text):
        if KEY.search(para) and 40 < len(para) < 1500:
            practices.append({'url': url, 'title': title, 'paragraph': para.strip()})
    for l in internal:
        if l not in seen: queue.append(l)
    time.sleep(0.25)

(OUT / 'index.json').write_text(json.dumps({'root': ROOT, 'pages': index, 'crawled_at': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())}, indent=1), encoding='utf-8')
(OUT / 'best_practices.json').write_text(json.dumps(practices, indent=1, ensure_ascii=False), encoding='utf-8')
print('PAGES', len(index), 'PRACTICE PARAGRAPHS', len(practices))
