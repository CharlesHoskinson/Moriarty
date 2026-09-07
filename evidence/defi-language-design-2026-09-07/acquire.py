from pathlib import Path
from scrapling.fetchers import Fetcher
import json,hashlib,datetime,time,sys
root=Path(__file__).resolve().parents[2] / 'inbox/defi-language-design-2026-09-07'; root.mkdir(parents=True,exist_ok=True)
urls=json.loads(Path(sys.argv[1]).read_text())
for item in urls:
 key=item['id']; dest=root/(key+'.receipt.json')
 if dest.exists(): continue
 at=datetime.datetime.now(datetime.timezone.utc).isoformat()
 try:
  p=Fetcher.get(item['url'],timeout=30,retries=0)
  body=p.body; body=body.encode() if isinstance(body,str) else bytes(body)
  ispdf=body.startswith(b'%PDF-'); ext='.pdf' if ispdf else '.html'
  (root/(key+ext)).write_bytes(body)
  txt='' if ispdf else (p.css('main, article')[0] if p.css('main, article') else p).get_all_text(separator='\n',strip=True)
  if txt: (root/(key+'.txt')).write_text(txt)
  rec={**item,'canonical_url':str(p.url),'retrieved_at':at,'http_status':p.status,'method':'Scrapling Fetcher 0.4.15; static HTTP; no authenticated session','sha256':hashlib.sha256(body).hexdigest(),'bytes':len(body),'payload':str((root/(key+ext)).relative_to(root.parent.parent)),'extraction':'full PDF pending pdftotext' if ispdf else 'main/article text or visible DOM text','coverage':'access only; individual cited sections must be inspected; full papers retained locally, not republished'}
 except Exception as e: rec={**item,'retrieved_at':at,'error':type(e).__name__+': '+str(e)[:250]}
 dest.write_text(json.dumps(rec,indent=2)+'\n'); print(json.dumps({k:rec.get(k) for k in ('id','http_status','bytes','error')}),flush=True); time.sleep(1)
