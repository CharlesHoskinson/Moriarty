from pathlib import Path
from scrapling.fetchers import Fetcher
import json,hashlib,datetime,time,sys
R=Path(__file__).parent
S=[('P01','cake','https://frontier.tech/the-cake-framework'),('P02','eip712','https://eips.ethereum.org/EIPS/eip-712'),('P03','erc1271','https://eips.ethereum.org/EIPS/eip-1271'),('P04','erc4337','https://eips.ethereum.org/EIPS/eip-4337'),('P05','eip7702','https://eips.ethereum.org/EIPS/eip-7702'),('P06','erc7715','https://eips.ethereum.org/EIPS/eip-7715'),('P07','rich-authorization','https://www.rfc-editor.org/rfc/rfc9396.html'),('P08','dpop','https://www.rfc-editor.org/rfc/rfc9449.html'),('P09','macaroons','https://research.google.com/pubs/archive/41892.pdf'),('P10','zcash','https://zips.z.cash/protocol/protocol.pdf'),('P11','chain-signatures','https://docs.near.org/chain-abstraction/chain-signatures'),('P12','frost','https://www.rfc-editor.org/rfc/rfc9591.html')]
start=int(sys.argv[1]);stop=int(sys.argv[2])
for sid,name,url in S[start:stop]:
 t=datetime.datetime.now(datetime.timezone.utc).isoformat()
 try:
  p=Fetcher.get(url,timeout=40);b=bytes(p.body);ext='.pdf' if b.startswith(b'%PDF-') else '.html';fn=name+ext;(R/fn).write_bytes(b)
  rec={'id':sid,'name':name,'requested_url':url,'final_url':p.url,'http_status':p.status,'retrieved_at_utc':t,'raw_file':fn,'raw_sha256':hashlib.sha256(b).hexdigest(),'raw_bytes':len(b),'scrapling_version':'0.4.15','authority':'primary','cookies_retained':False}
  if ext=='.html':
   node=p
   for selector in ['article','main','#content']:
    n=p.css(selector)
    if n:node=n[0];break
   tx=node.get_all_text(separator='\n',strip=True);(R/(name+'.txt')).write_text(tx);rec['extract_file']=name+'.txt';rec['extract_chars']=len(tx);rec['extract_sha256']=hashlib.sha256(tx.encode()).hexdigest()
  (R/(name+'.receipt.json')).write_text(json.dumps(rec,indent=2)+'\n');print(sid,name,p.status,len(b),flush=True)
 except Exception as e:
  (R/(name+'.error.json')).write_text(json.dumps({'id':sid,'url':url,'time':t,'error':str(e)},indent=2));print(sid,'ERROR',str(e),flush=True)
 time.sleep(.6)
