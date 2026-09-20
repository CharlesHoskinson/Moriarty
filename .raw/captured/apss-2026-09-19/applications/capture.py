from pathlib import Path
from scrapling.fetchers import Fetcher
from datetime import datetime,timezone
import json,hashlib,time,sys
ROOT=Path(__file__).parent
sources=[
('APP01','CAKE framework','https://frontier.tech/the-cake-framework','html','2024-02-15','frontier-cake'),
('APP02','Composing contracts: an adventure in financial engineering','https://www.microsoft.com/en-us/research/wp-content/uploads/2016/07/contracts-icfp.pdf','pdf','2000-09','peyton-jones-eber-seward'),
('APP03','Marlowe: Implementing and Analysing Financial Contracts on Blockchain','https://link.springer.com/content/pdf/10.1007/978-3-030-54455-3_35.pdf','pdf','2020-08-07','marlowe'),
('APP04','Certifying Findel Derivatives for Blockchain','https://arxiv.org/pdf/2005.13602','pdf','2020','findel-certification'),
('APP05','Rich Specifications for Ethereum Smart Contract Verification','https://arxiv.org/pdf/2104.10274','pdf','2021','2vyper'),
('APP06','Zexe: Enabling Decentralized Private Computation','https://www.cs.umd.edu/~imiers/pdf/zexe.pdf','pdf','2020','zexe'),
('APP07','Anoma resource logic specification','https://specs.anoma.net/main/arch/system/state/resource_machine/data_structures/proof/logic.html','html','2024-12-05 (page timestamp)','anoma'),
('APP08','Intents from the resource model perspective','https://anoma.net/blog/intents-rm','html','2024-08-21','anoma'),
('APP09','ComposableCoW README','https://raw.githubusercontent.com/cowprotocol/composable-cow/main/README.md','md',None,'cowprotocol'),
('APP10','CoW AMM technical specification','https://raw.githubusercontent.com/cowprotocol/cow-amm/main/docs/amm.md','md',None,'cowprotocol'),
('APP11','ERC-7540 Asynchronous ERC-4626 Tokenized Vaults','https://eips.ethereum.org/EIPS/eip-7540','html','2023-10-18 (created)','erc7540'),
('APP12','ERC-7683 Cross Chain Intents','https://eips.ethereum.org/EIPS/eip-7683','html',None,'erc7683'),
]
receipts=[]
if (ROOT/'manifest.json').exists(): receipts=json.loads((ROOT/'manifest.json').read_text())
lo=int(sys.argv[1]); hi=int(sys.argv[2])
for sid,title,url,typ,date,key in sources[lo:hi]:
 try:
  r=Fetcher.get(url,timeout=40)
  raw=bytes(r.body); fn=ROOT/'captures'/f'{sid}.{typ}'; fn.write_bytes(raw)
  ispdf=raw.startswith(b'%PDF-')
  if typ=='html':
   nodes=r.css('main, article, .c-article-body'); text=(nodes[0] if nodes else r).get_all_text(separator='\n',strip=True)
   (ROOT/'captures'/f'{sid}.txt').write_text(text)
  receipt=dict(id=sid,title=title,requested_url=url,final_url=str(r.url),http_status=r.status,retrieved_at=datetime.now(timezone.utc).isoformat(),publication_date=date,sha256=hashlib.sha256(raw).hexdigest(),bytes=len(raw),path=str(fn.relative_to(ROOT)),authority='primary' if typ=='pdf' else 'official',independence_key=key,pdf_valid=ispdf if typ=='pdf' else None,review_state='unreviewed')
  receipts=[x for x in receipts if x['id']!=sid]+[receipt]
  print(sid,r.status,len(raw),'PDF',ispdf,flush=True)
 except Exception as e: print(sid,type(e).__name__,str(e)[:100],flush=True)
 (ROOT/'manifest.json').write_text(json.dumps(receipts,indent=2)+'\n')
 time.sleep(1)
