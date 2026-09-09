from scrapling.fetchers import Fetcher
from pathlib import Path
import json,hashlib,datetime,subprocess
BASE=Path(__file__).parent
sources=[('U03b','https://www.dcc.uchile.cl/~rrobbes/p/ICPC2012-maintainability.pdf'),('U08b','https://www.dcc.uchile.cl/~rrobbes/p/ICSE2014-docstypes.pdf'),('U09','https://ppig.org/files/2000-PPIG-12th-blackwell.pdf'),('U10','https://arxiv.org/pdf/2011.07565'),('U08','https://www.inf.unibz.it/~rrobbes/p/ICSE2014-docstypes.pdf'),('U01','https://www.vidarholen.net/~vidar/An_Empirical_Investigation_into_Programming_Language_Syntax.pdf'),('U02','https://www.cs.cmu.edu/~jssunshi/assets/pdf/coblenz2021PLIERS.pdf'),('U03','https://www.inf.unibz.it/~rrobbes/p/ICPC2012-maintainability.pdf'),('U04','https://www.dcc.uchile.cl/TR/2012/TR_DCC-20120418-005.pdf'),('U05','https://digitalcommons.acu.edu/cgi/viewcontent.cgi?article=1003&context=info_tech_computing'),('U06','https://www.comsis.org/pdf.php?id=0702'),('U07','https://citeseerx.ist.psu.edu/document?doi=571c0a4a7d870ce5315366464b79a7f445203d13&repid=rep1&type=pdf')]
for sid,url in sources:
 if (BASE/f'{sid}.receipt.json').exists():continue
 stamp=datetime.datetime.now(datetime.timezone.utc).isoformat()
 try:
  r=Fetcher.get(url,timeout=45);b=r.body;pdf=b.startswith(b'%PDF');p=BASE/f'{sid}.{ "pdf" if pdf else "response"}';p.write_bytes(b)
  rec=dict(id=sid,requested_url=url,canonical_url=str(r.url),retrieved_at=stamp,status=r.status,sha256=hashlib.sha256(b).hexdigest(),bytes=len(b),raw_file=str(p),full_pdf=pdf,method='Scrapling 0.4.15 Fetcher.get; no bypass or authentication')
  if pdf:
   subprocess.run(['pdftotext','-layout',str(p),str(BASE/f'{sid}.txt')],check=True)
   rec['pages']= (BASE/f'{sid}.txt').read_text().count('\f')
  (BASE/f'{sid}.receipt.json').write_text(json.dumps(rec,indent=2)+'\n');print(sid,r.status,len(b),pdf,flush=True)
 except Exception as e: print(sid,type(e).__name__,str(e),flush=True)
