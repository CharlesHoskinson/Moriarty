import hashlib,json,time,urllib.robotparser
from datetime import datetime,timezone
from pathlib import Path
import scrapling
from scrapling.fetchers import Fetcher
out=Path(__file__).parent
paths=['robots.txt','docs/user_manual/','k-distribution/include/kframework/builtin/domains/','k-distribution/k-tutorial/1_basic/06_ints_and_bools/','k-distribution/k-tutorial/1_basic/07_side_conditions/','k-distribution/k-tutorial/1_basic/15_configurations/','k-distribution/k-tutorial/1_basic/13_rewrite_rules/','k-distribution/k-tutorial/1_basic/02_basics/','k-distribution/k-tutorial/1_basic/20_k_cells/']
rows=[]
robots=None
for i,path in enumerate(paths):
 url='https://kframework.org/'+path
 if robots is not None and not robots.can_fetch('MoriartyReferenceCapture',url):raise RuntimeError('robots denied '+url)
 r=Fetcher.get(url,timeout=30,follow_redirects=False,headers={'User-Agent':'MoriartyReferenceCapture/1.0'})
 raw=bytes(r.body);stem='robots' if i==0 else path.strip('/').split('/')[-1]
 (out/(stem+'.html')).write_bytes(raw)
 if i==0:
  robots=urllib.robotparser.RobotFileParser();robots.parse(raw.decode().splitlines())
  extracted=raw.decode()
 else:
  selected=r.css('main');assert r.status==200 and len(selected)==1,(url,r.status,len(selected))
  extracted=selected[0].get_all_text(separator='\n',strip=True)
 (out/(stem+'.txt')).write_text(extracted)
 rows.append({'requested_url':url,'canonical_url':str(r.url),'retrieved_at':datetime.now(timezone.utc).isoformat(),'http_status':r.status,'last_modified':r.headers.get('last-modified'),'source_class':'primary official reference documentation' if i else 'robots policy','acquisition_method':'scrapling.fetchers.Fetcher.get; Scrapling '+scrapling.__version__+'; Response.body bytes; redirects disabled','selector':'main' if i else None,'raw_file':stem+'.html','sha256':hashlib.sha256(raw).hexdigest(),'extracted_file':stem+'.txt','extracted_sha256':hashlib.sha256(extracted.encode()).hexdigest(),'raw_bytes':len(raw),'extracted_chars':len(extracted),'version_scope':'Live site on retrieval date; not pinned to local K 7.1.337','coverage_limitations':'Bounded page capture; selected sections inspected; no linked pages followed, runtime, proof or release equivalence established.'})
 (out/'manifest.json').write_text(json.dumps({'schema':'moriarty.official-k-docs-capture.v1','purpose':'bounded repayment K reference use; no training','robots_disposition':'Allow / for general agents; search=yes, ai-train=no, use=reference; no ai-input restriction stated; use bounded reference only','sources':rows},indent=2)+'\n')
 print(stem,r.status,len(raw),len(extracted),flush=True);time.sleep(1)
