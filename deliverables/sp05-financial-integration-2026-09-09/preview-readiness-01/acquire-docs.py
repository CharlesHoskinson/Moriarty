from scrapling.fetchers import Fetcher
from pathlib import Path
import hashlib,json,datetime
O=Path('/tmp/moriarty-preview-readiness-20260910');receipts=[]
for name,url in [('robots','https://docs.midnight.network/robots.txt'),('node-endpoints','https://docs.midnight.network/nodes/node-endpoints.md'),('networks-and-environments','https://docs.midnight.network/guides/networks-and-environments.md')]:
 started=datetime.datetime.now(datetime.timezone.utc).isoformat();page=Fetcher.get(url,timeout=15,retries=1,follow_redirects=False);raw=bytes(page.body);assert len(raw)<=1048576
 (O/(name+'.txt')).write_bytes(raw);receipt={'requestedUrl':url,'canonicalUrl':str(page.url),'status':page.status,'retrievedAt':started,'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest(),'path':name+'.txt','method':'Scrapling0.4.15 Fetcher.get, one attempt,15s; no credentials/cookie persistence'};receipts.append(receipt)
 print(json.dumps(receipt))
(O/'documentation-receipts.json').write_text(json.dumps(receipts,indent=2)+'\n')
