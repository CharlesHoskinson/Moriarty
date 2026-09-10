from pathlib import Path
from datetime import datetime,timezone
import json,hashlib
from scrapling.fetchers import Fetcher,DynamicFetcher
root=Path('raw/sources/language-design-2026-09-09/official')
for sid,slug,url in [('D12','unison-updates','https://www.unison-lang.org/docs/usage-topics/workflow-how-tos/update-code/'),('D13','unison-metadata','https://www.unison-lang.org/docs/tooling/author-license/'),('D04R','elm-farewell-rendered','https://elm-lang.org/news/farewell-to-frp')]:
 if list(root.glob(sid+'-*.json')):continue
 stamp=datetime.now(timezone.utc).isoformat()
 r=DynamicFetcher.fetch(url,headless=True,executable_path='/home/charl/.cache/ms-playwright/chromium-1234/chrome-linux64/chrome',network_idle=True,timeout=30000) if sid=='D04R' else Fetcher.get(url,timeout=30)
 p=root/(sid+'-'+slug+'.html');p.write_bytes(r.body)
 nodes=r.css('article');node=nodes[-1] if nodes else r.css('body')[0]
 t=p.with_suffix('.txt');t.write_text(node.get_all_text(separator='\n',strip=True))
 p.with_suffix('.json').write_text(json.dumps({'id':sid,'requested_url':url,'canonical_url':r.url,'status':r.status,'retrieved_at':stamp,'raw_path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'text_path':str(t),'text_sha256':hashlib.sha256(t.read_bytes()).hexdigest(),'extraction':'Scrapling 0.4.15 last article or rendered body; no retained cookies','title':str(r.css('title::text').get()),'source_kind':'official documentation','publication_date':'page dated where available; see text','limitations':'Documentation observation, no behavior reproduction'},indent=2))
 print(sid,r.status,len(t.read_text()),flush=True)
