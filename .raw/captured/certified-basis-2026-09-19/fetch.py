import json,hashlib,time,sys
from pathlib import Path
from datetime import datetime,timezone
from urllib.parse import urlsplit
from urllib.robotparser import RobotFileParser
from scrapling.fetchers import Fetcher
R=Path(__file__).parent
urls=json.loads(Path(sys.argv[1]).read_text()); rec=[]; robots={}
for name,url in urls:
 domain=urlsplit(url).netloc
 if domain not in robots:
  rp=Fetcher.get('https://'+domain+'/robots.txt',timeout=30); (R/(domain+'-robots.txt')).write_bytes(bytes(rp.body)); parser=RobotFileParser();parser.parse(bytes(rp.body).decode(errors='replace').splitlines() if rp.status==200 else []);robots[domain]=parser
 if not robots[domain].can_fetch('*',url):
  rec.append({'id':name,'url':url,'status':'robots-disallowed'});continue
 try:
  p=Fetcher.get(url,timeout=40);b=bytes(p.body); pdf=b.startswith(b'%PDF-');ct=p.headers.get('content-type','');suffix='.pdf' if pdf else '.html' if 'html' in ct else '.txt';f=R/(name+suffix);f.write_bytes(b)
  txt=''
  if not pdf:
   if 'html' in ct:
    nodes=p.css('article') or p.css('main') or p.css('body');txt=nodes[0].get_all_text(separator='\n',strip=True) if nodes else ''
   else:txt=b.decode(errors='replace')
   (R/(name+'.extract.txt')).write_text(txt)
  item={'id':name,'url':url,'final_url':str(p.url),'status':p.status,'retrieved_at':datetime.now(timezone.utc).isoformat(),'raw_file':f.name,'sha256':hashlib.sha256(b).hexdigest(),'bytes':len(b),'extract_chars':len(txt),'fetcher':'Scrapling 0.4.15','pdf':pdf}
  rec.append(item);print(name,p.status,len(b),len(txt),flush=True)
 except Exception as e:rec.append({'id':name,'url':url,'error':str(e)})
 time.sleep(1)
(R/(Path(sys.argv[1]).stem+'-receipts.json')).write_text(json.dumps(rec,indent=2))
