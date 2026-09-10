from scrapling.fetchers import Fetcher
from pathlib import Path
from urllib.parse import urlsplit
from urllib.robotparser import RobotFileParser
import json,hashlib,datetime,subprocess
root=Path(__file__).parent
sources=[("S07","Elm: Concurrent FRP for Functional GUIs",2012,"https://elm-lang.org/assets/papers/concurrent-frp.pdf"),("S01","Asynchronous Functional Reactive Programming for GUIs",2013,"https://people.seas.harvard.edu/~chong/pubs/pldi13-elm.pdf"),("S02","Push-pull Functional Reactive Programming",2009,"https://conal.net/papers/push-pull-frp/push-pull-frp.pdf"),("S03","Simply RaTT",2019,"https://arxiv.org/pdf/1903.05879"),("S04","Nix: A Safe and Policy-Free System for Software Deployment",2004,"https://eelcovisser.org/publications/2004/DolstraJV04.pdf"),("S05","Koka: Programming with Row Polymorphic Effect Types",2014,"https://arxiv.org/pdf/1406.2061"),("S06","Build Systems a la Carte",2018,"https://www.microsoft.com/en-us/research/wp-content/uploads/2018/03/build-systems.pdf")]
for sid,title,year,url in sources:
 if sid=="S02" or (root/(sid+".json")).exists(): continue
 now=datetime.datetime.now(datetime.timezone.utc).isoformat(); host=urlsplit(url); robots=host.scheme+"://"+host.netloc+"/robots.txt"
 r=Fetcher.get(robots,timeout=30); (root/(sid+"-robots.txt")).write_bytes(r.body)
 allowed=True
 if r.status==200:
  rp=RobotFileParser(); rp.parse(r.body.decode("utf-8",errors="replace").splitlines()); allowed=rp.can_fetch("*",url)
 if not allowed:
  print(sid,"robots denied"); continue
 r=Fetcher.get(url,timeout=60); body=r.body; pdf=body.startswith(b"%PDF-"); path=root/(sid+(".pdf" if pdf else ".response")); path.write_bytes(body)
 receipt=dict(id=sid,title=title,publication_year=year,requested_url=url,canonical_url=r.url,retrieved_at=now,http_status=r.status,sha256=hashlib.sha256(body).hexdigest(),local_path=str(path),acquisition_method="Scrapling 0.4.15 Fetcher.get; public no bypass",source_class="primary academic author/institution/preprint",robots_status=int(Fetcher.get(robots,timeout=30).status),coverage="full PDF captured; reading tracked separately" if pdf else "not PDF; failure retained")
 if pdf:
  txt=root/(sid+".txt"); subprocess.run(["pdftotext","-layout",str(path),str(txt)],check=True); receipt["text_sha256"]=hashlib.sha256(txt.read_bytes()).hexdigest(); receipt["pdf_pages"]=len(txt.read_text().split("\f"))-1
 (root/(sid+".json")).write_text(json.dumps(receipt,indent=2)+"\n"); print(sid,r.status,len(body),receipt.get("pdf_pages"))
