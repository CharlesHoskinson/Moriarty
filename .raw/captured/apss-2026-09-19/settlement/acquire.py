import hashlib,json,datetime,time,subprocess
from pathlib import Path
from scrapling.fetchers import Fetcher
root=Path(__file__).parent; out=root/'sources';out.mkdir(exist_ok=True)
sources=[
('SET-01','cake','https://frontier.tech/the-cake-framework','html'),
('SET-02','atomic-swaps','https://arxiv.org/pdf/1801.09515','pdf'),
('SET-03','cross-chain-deals','https://www.vldb.org/pvldb/vol13/p100-herlihy.pdf','pdf'),
('SET-04','pcd','https://ic-people.epfl.ch/~achiesa/docs/CT10.pdf','pdf'),
('SET-05','nova','https://eprint.iacr.org/2021/370.pdf','pdf'),
('SET-06','ibc-packets','https://raw.githubusercontent.com/cosmos/ibc/6eb8792e987220d7afcc8c926426f3af5695cb7b/spec/core/ics-004-channel-and-packet-semantics/README.md','markdown'),
('SET-07','midnight-semantics','https://docs.midnight.network/concepts/how-midnight-works/semantics','html'),
('SET-08','compact-reference','https://docs.midnight.network/compact/reference/lang-ref','html'),
('SET-09','midnight-consensus','https://docs.midnight.network/concepts/network-architecture/consensus','html'),
('SET-10','flash-boys','https://arxiv.org/pdf/1904.05234','pdf'),
('SET-11','erc-7683','https://eips.ethereum.org/EIPS/eip-7683','html'),
('SET-12','kachina-docs','https://docs.midnight.network/concepts/kachina','html')]
receipts=[]
for sid,name,url,kind in sources:
 t=datetime.datetime.now(datetime.timezone.utc).isoformat()
 try:
  p=Fetcher.get(url,timeout=45);raw=bytes(p.body)
  suffix='pdf' if kind=='pdf' and raw.startswith(b'%PDF') else ('md' if kind=='markdown' else 'html')
  path=out/f'{sid}-{name}.{suffix}';path.write_bytes(raw)
  r={'id':sid,'name':name,'requested_url':url,'canonical_url':str(p.url),'status':p.status,'captured_at_utc':t,'artifact':str(path.relative_to(root)),'sha256':hashlib.sha256(raw).hexdigest(),'bytes':len(raw),'expected_kind':kind,'actual_kind':suffix,'acquisition':'Scrapling Fetcher 0.4.15; no retained credentials/cookies'}
  if suffix=='pdf':
   subprocess.run(['pdftotext','-layout',str(path),str(path.with_suffix('.txt'))],check=True)
  elif suffix=='html':
   # Extract visible article content, excluding raw script content from model input.
   nodes=p.css('main') or p.css('article') or p.css('body')
   extract='\n'.join(n.get_all_text(separator='\n',strip=True) for n in nodes)
   ep=path.with_suffix('.txt');ep.write_text(extract)
   links=[{'text':a.get_all_text(strip=True),'url':a.attrib.get('href')} for a in p.css('a[href]')]
   path.with_suffix('.links.json').write_text(json.dumps(links,indent=2))
  receipts.append(r); print(sid,p.status,suffix,len(raw),flush=True)
 except Exception as e:receipts.append({'id':sid,'requested_url':url,'captured_at_utc':t,'error':str(e)});print(sid,'ERROR',str(e),flush=True)
 (root/'manifest.json').write_text(json.dumps(receipts,indent=2)+'\n')
 time.sleep(.4)
