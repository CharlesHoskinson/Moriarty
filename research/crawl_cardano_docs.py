#!/usr/bin/env python3
"""Scrapling crawl of official current Cardano documentation; creates receipts."""
import concurrent.futures, datetime, hashlib, json, logging, pathlib, re, threading, time, sys
from collections import deque
from urllib.parse import urljoin, urlsplit, urlunsplit
from urllib.robotparser import RobotFileParser
from lxml import etree
from scrapling.fetchers import Fetcher

API_MODE='--api' in sys.argv
METADATA=pathlib.Path(__file__).resolve().parent/'cardano-developer-docs'
ROOT=pathlib.Path(__file__).resolve().parent/('cardano-plutus-api' if API_MODE else 'cardano-developer-docs')
ROOT.mkdir(exist_ok=True);(ROOT/'pages').mkdir(exist_ok=True)
logging.getLogger('scrapling').setLevel(logging.ERROR)
HOSTS={'plutus.cardano.intersectmbo.org':'/haddock/latest'} if API_MODE else {'developers.cardano.org':'/docs/','docs.cardano.org':'/','plutus.cardano.intersectmbo.org':'/docs/'}
robots={}; rules={}; locks={h:threading.Lock() for h in HOSTS}; last={h:0.0 for h in HOSTS}
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def canonical(url):
    s=urlsplit(url)
    if s.scheme!='https' or s.netloc not in HOSTS or s.query:return None
    path=s.path or '/'
    if not path.startswith(HOSTS[s.netloc]):return None
    if re.search(r'\.(pdf|png|jpe?g|gif|svg|zip|css|js|map|woff2?|ttf|mp4|ico|xml|json|txt|md)$',path,re.I):return None
    if '/assets/' in path or '/search' in path or '/src/' in path:return None
    # Docusaurus trailing slash conventions differ; deduplicate equivalent routes.
    if path!='/' and not path.endswith('.html'):path=path.rstrip('/')
    return urlunsplit(('https',s.netloc,path,'',''))
def request(url):
    host=urlsplit(url).netloc
    with locks[host]:
        delay=max(1,rules.get(host,1)); wait=delay-(time.monotonic()-last[host])
        if wait>0:time.sleep(wait)
        last[host]=time.monotonic()
    return Fetcher.get(url,timeout=25,retries=1,headers={'User-Agent':'MoriartyResearch/1.0 (public documentation archive)'})
for host in HOSTS:
    slug={'developers.cardano.org':'portal','docs.cardano.org':'cardano','plutus.cardano.intersectmbo.org':'plutus'}[host]
    p=METADATA/(slug+'-robots.txt'); rp=RobotFileParser()
    receipt=METADATA/(slug+'-robots.receipt.json')
    status=json.loads(receipt.read_text()).get('http_status') if receipt.exists() else None
    if status==200:
        rp.parse(p.read_text().splitlines());robots[host]=rp
        rules[host]=rp.crawl_delay('MoriartyResearch') or rp.crawl_delay('*') or 1
    else:robots[host]=None;rules[host]=1

seeds=set()
for slug in ([] if API_MODE else ['portal','cardano']):
    tree=etree.fromstring((ROOT/(slug+'-sitemap.txt')).read_bytes())
    for url in tree.xpath('//*[local-name()="loc"]/text()'):
        c=canonical(url)
        if c:seeds.add(c)
seeds.add('https://plutus.cardano.intersectmbo.org/haddock/latest' if API_MODE else 'https://plutus.cardano.intersectmbo.org/docs')
queue=deque(sorted(seeds));seen=set(seeds);records=[];started=now();safety_limit=5000
def fetch(url):
    slug=hashlib.sha256(url.encode()).hexdigest()[:20]
    receipt=ROOT/'pages'/(slug+'.receipt.json')
    if receipt.exists():
        row=json.loads(receipt.read_text());return row,row.get('discovered_links',[])
    host=urlsplit(url).netloc
    row={'requested_url':url,'retrieved_at':now(),'method':'scrapling.Fetcher','authority':'official','review_state':'captured-unreviewed','independence_key':host,'robots_status':'allowed' if robots[host] else 'unavailable-404','limitations':[]}
    links=[]
    try:
        if robots[host] and not robots[host].can_fetch('MoriartyResearch',url):row['status']='robots-disallowed'
        else:
            page=request(url);data=bytes(page.body);ct=str(page.headers.get('content-type',''))
            row.update(http_status=page.status,canonical_url=str(page.url),content_type=ct,bytes=len(data),sha256=hashlib.sha256(data).hexdigest())
            raw=ROOT/'pages'/(slug+'.html');raw.write_bytes(data);row['payload_path']=str(raw)
            row['status']='downloaded' if page.status==200 and 'html' in ct else 'http-error'
            title=page.css('title::text').get();row['title']=title
            elements=page.css('article') or page.css('main') or page.css('#content')
            text='\n\n'.join(e.get_all_text(separator='\n',strip=True) for e in elements)
            out=ROOT/'pages'/(slug+'.extract.txt');out.write_text(text);row.update(extract_path=str(out),extract_characters=len(text))
            if page.status==200:
                for href in page.css('a::attr(href)').getall():
                    target=canonical(urljoin(str(page.url),href))
                    if target:links.append(target)
                links=sorted(set(links))
            if len(text)<200:row['limitations'].append('Short article extraction; retained HTML requires inspection.')
            if not robots[host]:row['limitations'].append('Site robots endpoint returned404; bounded public documentation traversal with1second minimum request spacing.')
    except Exception as exc:row.update(status='failed',error=str(exc)[:500])
    row.update(finished_at=now(),receipt_path=str(receipt),discovered_links=links)
    receipt.write_text(json.dumps(row,indent=2)+'\n');return row,links

with concurrent.futures.ThreadPoolExecutor(max_workers=3) as pool:
    pending={}
    while queue or pending:
        while queue and len(pending)<3 and len(records)+len(pending)<safety_limit:
            url=queue.popleft();pending[pool.submit(fetch,url)]=url
        if not pending:break
        done,_=concurrent.futures.wait(pending,return_when=concurrent.futures.FIRST_COMPLETED)
        for f in done:
            pending.pop(f);row,links=f.result();records.append(row)
            for link in links:
                if link not in seen:seen.add(link);queue.append(link)
        if len(records)%25==0:
            print(json.dumps({'completed':len(records),'discovered':len(seen),'queued':len(queue),'at':now()}),flush=True)
            (ROOT/'PROGRESS.json').write_text(json.dumps({'completed':len(records),'discovered':len(seen),'queued':len(queue),'updated_at':now()},indent=2)+'\n')

from collections import Counter
summary={'started_at':started,'finished_at':now(),'initial_seed_count':len(seeds),'discovered_in_scope':len(seen),'attempted':len(records),'remaining_queue':len(queue),'statuses':dict(Counter(r.get('status') for r in records)),'hosts':{host:dict(Counter(r.get('status') for r in records if urlsplit(r['requested_url']).netloc==host)) for host in HOSTS},'link_graph_exhausted':not queue,'safety_limit':safety_limit,'scope':HOSTS,'excluded':['Blog/community/showcase pages outside portal /docs/','Translations and version archives; API mode takes latest only','External ecosystem sites; generated API retained in separate API crawl','Source-code HTML views (source repositories retained separately)','Binary assets and query variants'],'limitations':['Sitemap plus in-scope links enumerated; unlinked pages cannot be guaranteed.','Downloads are a corpus, not all pages substantively reviewed.','No access-control bypass or browser/login workflows.']}
(ROOT/'MANIFEST.json').write_text(json.dumps({'summary':summary,'pages':sorted(records,key=lambda x:x['requested_url'])},indent=2)+'\n')
lines=['# Official Cardano developer documentation archive','',f"{summary['attempted']} pages attempted; statuses: {summary['statuses']}. In-scope link graph exhausted: {summary['link_graph_exhausted']}.",'','| Title | Original URL | Status | Local text |','|---|---|---|---|']
for r in sorted(records,key=lambda x:x['requested_url']):
    extract=pathlib.Path(r['extract_path']).relative_to(ROOT) if r.get('extract_path') else None
    lines.append(f"| {str(r.get('title') or 'Untitled').replace('|','/')} | [{r['requested_url']}]({r['requested_url']}) | {r.get('status')} | {'[text]('+str(extract)+')' if extract else '—'} |")
(ROOT/'INDEX.md').write_text('\n'.join(lines)+'\n');print(json.dumps(summary,indent=2),flush=True)
