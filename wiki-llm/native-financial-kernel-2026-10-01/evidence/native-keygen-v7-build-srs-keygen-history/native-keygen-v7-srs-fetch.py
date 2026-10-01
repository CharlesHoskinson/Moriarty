"""One new full-body public request after consumed v6 transport attempt; no retry/resume."""
import hashlib,json,os,sys,time,urllib.request
from pathlib import Path
URL='https://srs.midnight.network/bls_midnight_2p17'
SIZE=25166212; SHA='4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74'
class NoRedirect(urllib.request.HTTPRedirectHandler):
 def redirect_request(self,*args,**kwargs):return None
root=Path(sys.argv[1]);assert len(sys.argv)==2 and not root.exists();root.mkdir()
started=time.monotonic();stage='before_request';written=0;last_progress=None
metadata=lambda:{'endpoint':URL,'stage':stage,'bytes_written':written,'elapsed_seconds':time.monotonic()-started,'last_progress_seconds':last_progress,'prior_consumed_transport_attempts':1,'attempts_this_allocation':1,'attempts':1,'headers_redacted':True,'range_resume_fallback':False}
try:
 opener=urllib.request.build_opener(NoRedirect)
 request=urllib.request.Request(URL,headers={'Accept-Encoding':'identity'})
 stage='request_open'
 with opener.open(request,timeout=60) as response:
  assert response.status==200 and response.geturl()==URL,'exact endpoint/status required'
  assert int(response.headers.get('Content-Length','-1'))==SIZE,'exact body length required'
  assert response.headers.get('Content-Encoding','identity')=='identity','encoded body refused'
  stage='response_headers_validated';digest=hashlib.sha256()
  with (root/'srs.part').open('xb') as out:
   stage='body_streaming'
   while True:
    block=response.read(min(65536,SIZE-written+1))
    if not block:break
    assert written+len(block)<=SIZE,'body ceiling exceeded'
    out.write(block);written+=len(block);digest.update(block);last_progress=time.monotonic()-started
   stage='body_complete';out.flush();os.fsync(out.fileno())
  assert written==SIZE and digest.hexdigest()==SHA,'body identity refused';stage='body_hash_verified'
 os.link(root/'srs.part',root/'bls_midnight_2p17');stage='final_name_published'
 with (root/'acquisition.json').open('x') as out:
  json.dump({**metadata(),'bytes':written,'sha256':SHA,'verified':True,'idle_socket_timeout_seconds':60,'supervisor_wall_limit_seconds':600,'ceremony_independently_audited':False},out,indent=2);out.flush();os.fsync(out.fileno())
 print('EXACT_PUBLIC_SRS_BODY_HASH_VERIFIED; previous v6 request remains consumed; not ceremony/keygen evidence')
except BaseException as error:
 with (root/'acquisition-failed.json').open('x') as out:
  json.dump({**metadata(),'failure_type':type(error).__name__,'verified':False,'idle_socket_timeout_seconds':60,'supervisor_wall_limit_seconds':600},out,indent=2);out.flush();os.fsync(out.fileno())
 print('SRS_ACQUISITION_REFUSED '+type(error).__name__);raise SystemExit(1)
