from pathlib import Path
from datetime import datetime, timezone
import hashlib,json,time
from urllib.parse import urlparse
from scrapling.fetchers import Fetcher
root=Path('raw/sources/language-design-2026-09-09/official');root.mkdir(parents=True,exist_ok=True)
urls=[('D01','elm-architecture','https://guide.elm-lang.org/architecture/'),('D02','elm-effects','https://guide.elm-lang.org/effects/'),('D03','elm-time','https://guide.elm-lang.org/effects/time.html'),('D04','elm-farewell-frp','https://elm-lang.org/news/farewell-to-frp'),('D05','elm-ports','https://guide.elm-lang.org/interop/ports.html'),('D06','unison-big-idea','https://www.unison-lang.org/docs/the-big-idea/'),('D07','unison-hashes','https://www.unison-lang.org/docs/language-reference/hashes/'),('D08','unison-abilities','https://www.unison-lang.org/docs/language-reference/abilities-and-ability-handlers/'),('D09','unison-tour','https://www.unison-lang.org/docs/tour/'),('D10','unison-types','https://www.unison-lang.org/docs/fundamentals/data-types/unique-and-structural-types/'),('D11','compact-reference','https://docs.midnight.network/compact/reference/compact-reference')]
# A bounded public reference capture. No credentials, browser, or challenge bypass.
for domain in sorted({urlparse(x[2]).netloc for x in urls}):
 p=root/('robots-'+domain+'.txt')
 if not p.exists():
  r=Fetcher.get('https://'+domain+'/robots.txt',timeout=30);p.write_bytes(r.body)
  (p.with_suffix('.json')).write_text(json.dumps({'url':'https://'+domain+'/robots.txt','status':r.status,'retrieved_at':datetime.now(timezone.utc).isoformat(),'sha256':hashlib.sha256(r.body).hexdigest()},indent=2))
for sid,slug,url in urls:
 rec=root/(sid+'-'+slug+'.json')
 if rec.exists():continue
 stamp=datetime.now(timezone.utc).isoformat();r=Fetcher.get(url,timeout=30)
 body=r.body;html=root/(sid+'-'+slug+'.html');
 if html.exists():
  html=root/(sid+'-'+slug+'-retry.html')
 html.write_bytes(body)
 scope=next((r.css(sel)[0] for sel in ('article','main','.page-inner','body') if r.css(sel)),None)
 txt=scope.get_all_text(separator='\n',strip=True) if scope is not None else ''
 extract=root/(sid+'-'+slug+'.txt');extract.write_text(txt)
 rec.write_text(json.dumps({'id':sid,'title':str(r.css('title::text').get() or slug),'publisher':urlparse(url).netloc,'source_kind':'official documentation','publication_date':'not established; dated retrieval snapshot','requested_url':url,'canonical_url':r.url,'status':r.status,'retrieved_at':stamp,'observation_window':'2026-09-09 UTC; rolling official pages, not a release guarantee','raw_path':str(html),'sha256':hashlib.sha256(body).hexdigest(),'text_path':str(extract),'text_sha256':hashlib.sha256(extract.read_bytes()).hexdigest(),'extraction':'Scrapling 0.4.15 article/main/page-inner/body visible text; raw HTML retained','access_limitations':'No execution or independent performance/usability replication; current docs may drift'},indent=2))
 print(sid,r.status,len(body),len(txt),flush=True);time.sleep(1)
