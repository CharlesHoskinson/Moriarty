from pathlib import Path
from scrapling.fetchers import Fetcher
import json,hashlib,datetime,urllib.request
r=Path(__file__).parent;(r/'docs').mkdir(exist_ok=True)
urls=[('proof-book','https://o1-labs.github.io/proof-systems/'),('zkprogram','https://o1-labs.github.io/o1js/api-reference/functions/ZkProgram/'),('dynamic-proof','https://o1-labs.github.io/o1js/api-reference/classes/DynamicProof/'),('serialization','https://o1-labs.github.io/o1js/advanced-concepts/serialization/'),('smart-contract','https://o1-labs.github.io/o1js/api-reference/classes/SmartContract/'),('pickles-audit-announcement','https://minaprotocol.com/blog/pickles-security-audit'),('pickles-audit','https://minaprotocol.com/wp-content/uploads/Least-Authority-Pickles-Final-Audit-Report.pdf')]
out=[]
for name,url in urls:
 try:
  response=Fetcher.get(url,timeout=40); body=response.body
  raw=r/'docs'/(name+('.pdf' if url.endswith('.pdf') else '.html'));raw.write_bytes(body)
  textpath=None
  if not url.endswith('.pdf'):
   textpath=r/'docs'/(name+'.txt');els=response.css('main,article');text='\n'.join(e.get_all_text(separator='\n',strip=True) for e in els) if els else response.get_all_text(separator='\n',strip=True);textpath.write_text(text)
  row={'url':url,'canonical_url':str(response.url),'status':response.status,'path':str(raw),'sha256':hashlib.sha256(body).hexdigest(),'bytes':len(body),'text':str(textpath) if textpath else None,'method':'Scrapling Fetcher','retrieved_at':datetime.datetime.now(datetime.timezone.utc).isoformat()}
 except Exception as e:row={'url':url,'error':str(e)}
 out.append(row);print(json.dumps(row),flush=True)
(r/'doc-receipts.json').write_text(json.dumps(out,indent=2))
