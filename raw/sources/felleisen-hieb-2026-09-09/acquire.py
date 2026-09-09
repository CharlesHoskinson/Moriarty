from pathlib import Path
from scrapling.fetchers import Fetcher
from urllib.parse import urlparse
from urllib.robotparser import RobotFileParser
import hashlib,json,datetime,subprocess
r=Path('/home/charl/Moriarty/.worktrees/felleisen-hieb-readme/raw/sources/felleisen-hieb-2026-09-09');r.mkdir(parents=True,exist_ok=True)
urls=['https://plv.mpi-sws.org/plerg/papers/felleisen-hieb-92-2up.pdf']
for i,url in enumerate(urls):
 origin='https://'+urlparse(url).netloc; rob=Fetcher.get(origin+'/robots.txt');(r/f'robots-{i}.txt').write_bytes(rob.body)
 if rob.status==200:
  parser=RobotFileParser();parser.parse(rob.body.decode('utf-8','replace').splitlines());assert parser.can_fetch('*',url),'robots disallows source'
 res=Fetcher.get(url);data=res.body
 receipt={'requested_url':url,'canonical_url':str(res.url),'retrieved_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'status':res.status,'sha256':hashlib.sha256(data).hexdigest(),'bytes':len(data),'method':'Scrapling 0.4.15 Fetcher.get; public request, no authentication or bypass','robots_status':rob.status,'robots_file':f'robots-{i}.txt'}
 (r/f'capture-{i}.json').write_text(json.dumps(receipt,indent=2)+'\n')
 if res.status==200 and data.startswith(b'%PDF'):
  (r/'felleisen-hieb-1992.pdf').write_bytes(data);subprocess.run(['pdftotext','-layout',str(r/'felleisen-hieb-1992.pdf'),str(r/'felleisen-hieb-1992.txt')],check=True);print(json.dumps(receipt));break
else:raise SystemExit('No full primary PDF acquired')
