from pathlib import Path
import urllib.request,urllib.error,json,hashlib,datetime,sys
D=Path(__file__).parent
url,name=sys.argv[1:]
assert url.startswith('https://api.github.com/repos/midnightntwrk/')
assert len(list(D.glob('*.receipt.json')))<64
assert sum(p.stat().st_size for p in D.iterdir() if p.is_file())<32*1024*1024
assert not (D/(name+'.receipt.json')).exists()
ts=datetime.datetime.now(datetime.timezone.utc).isoformat()
try:
 with urllib.request.urlopen(urllib.request.Request(url,headers={'Accept':'application/vnd.github+json','User-Agent':'Moriarty-read-only-source-inspection'}),timeout=20) as r:
  status=r.status;body=r.read(2*1024*1024+1);assert len(body)<=2*1024*1024
except urllib.error.HTTPError as e:status=e.code;body=e.read(65536)
except Exception as e:status=None;body=json.dumps({'error_type':type(e).__name__}).encode()
(D/(name+'.json')).write_bytes(body)
(D/(name+'.receipt.json')).write_text(json.dumps({'timestamp_utc':ts,'requested_url':url,'status':status,'bytes':len(body),'sha256':hashlib.sha256(body).hexdigest(),'method':'GitHub structured public REST GET; no auth;20second timeout;2MiB response bound','coverage':'single request; no retry; no binary artifacts'},indent=2)+'\n')
print(name,status,len(body))
