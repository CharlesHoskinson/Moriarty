from pathlib import Path
from datetime import datetime,timezone
from urllib.parse import urlsplit
from urllib.robotparser import RobotFileParser
from scrapling.fetchers import Fetcher
import hashlib,json
root=Path('/home/charl/Moriarty/.worktrees/developer-mock/raw/pcd-supplement-2026-09-06');root.mkdir(exist_ok=True)
rows=[]
for name,url in [('holography','https://eprint.iacr.org/2026/538'),('tct','https://arxiv.org/abs/2408.06478v2')]:
 roboturl='https://'+urlsplit(url).netloc+'/robots.txt'
 robot=Fetcher.get(roboturl,timeout=25)
 (root/(name+'-robots.txt')).write_bytes(robot.body)
 p=RobotFileParser();p.parse(robot.body.decode(errors='replace').splitlines())
 allowed=robot.status==200 and p.can_fetch('*',url)
 row={'requested_url':url,'retrieved_at_utc':datetime.now(timezone.utc).isoformat(),'robots_url':roboturl,'robots_status':robot.status,'robots_allowed':allowed,'acquisition_method':'Scrapling Fetcher; public unauthenticated GET','source_class':'primary author abstract and metadata','coverage_limitations':'Abstract and metadata only; no proof, implementation or benchmark reproduced. No cookies saved.'}
 if allowed:
  res=Fetcher.get(url,timeout=25)
  path=root/(name+'.html');path.write_bytes(res.body)
  text=res.get_all_text(separator='\n',strip=True)
  (root/(name+'.txt')).write_text(text)
  row.update(canonical_url=str(res.url),http_status=res.status,local_path=str(path.relative_to(root.parent.parent)),sha256=hashlib.sha256(res.body).hexdigest(),published_or_last_modified='2026-06-08 revision' if name=='holography' else '2025-08-06 v2')
 else: row.update(http_status=None,coverage_limitations='Scrapling acquisition not attempted: robots status/policy disallowed. Primary abstract inspected with web tool; no local page receipt.')
 rows.append(row)
(root/'receipt.json').write_text(json.dumps(rows,indent=2))
print(json.dumps(rows,indent=2))
