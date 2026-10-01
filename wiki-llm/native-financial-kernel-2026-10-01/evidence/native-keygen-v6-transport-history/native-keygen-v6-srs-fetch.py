"""One exact public SRS acquisition; execution requires separate supervisor approval."""
import hashlib,json,os,sys,urllib.request
from pathlib import Path
URL='https://srs.midnight.network/bls_midnight_2p17'
SIZE=25166212; SHA='4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74'
class NoRedirect(urllib.request.HTTPRedirectHandler):
 def redirect_request(self,*args,**kwargs):return None
root=Path(sys.argv[1]);assert len(sys.argv)==2 and not root.exists();root.mkdir()
try:
 opener=urllib.request.build_opener(NoRedirect)
 request=urllib.request.Request(URL,headers={'Accept-Encoding':'identity'})
 with opener.open(request,timeout=5) as response:
  assert response.status==200 and response.geturl()==URL,'exact endpoint/status required'
  assert int(response.headers.get('Content-Length','-1'))==SIZE,'exact body length required'
  assert response.headers.get('Content-Encoding','identity')=='identity','encoded body refused'
  digest=hashlib.sha256();count=0
  with (root/'srs.part').open('xb') as out:
   while True:
    block=response.read(min(65536,SIZE-count+1))
    if not block:break
    count+=len(block);assert count<=SIZE,'body ceiling exceeded'
    digest.update(block);out.write(block)
   out.flush();os.fsync(out.fileno())
  assert count==SIZE and digest.hexdigest()==SHA,'body identity refused'
 os.link(root/'srs.part',root/'bls_midnight_2p17') # exclusive final name; preserve partial/history
 with (root/'acquisition.json').open('x') as out:
  json.dump({'endpoint':URL,'bytes':count,'sha256':SHA,'verified':True,'attempts':1,'headers_redacted':True,'ceremony_independently_audited':False},out,indent=2);out.flush();os.fsync(out.fileno())
 print('EXACT_PUBLIC_SRS_BODY_HASH_VERIFIED; not ceremony or keygen evidence')
except BaseException as error:
 # No server headers/tokens/error URLs are printed. Retain partial body and failure type only.
 with (root/'acquisition-failed.json').open('x') as out:json.dump({'failure_type':type(error).__name__,'verified':False,'attempts':1,'headers_redacted':True},out)
 print('SRS_ACQUISITION_REFUSED '+type(error).__name__);raise SystemExit(1)
