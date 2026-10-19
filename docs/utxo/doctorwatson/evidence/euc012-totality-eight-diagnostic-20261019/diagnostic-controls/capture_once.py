"""ONE root-owned ordinary capture. Subject executes ONLY with --execute-once.
No baseline script exists: previous root capture was inline. Whole new source supplied.
"""
from pathlib import Path
import argparse,datetime,hashlib,json,os,signal,subprocess,time
D=Path(__file__).resolve().parent
PLAN_SHA256='480c54823d064d70a7e87dd6670766b732381f49f38f44891bd6f0a949172c72'
def sha(raw):return hashlib.sha256(raw).hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(path,value):
 data=(json.dumps(value,indent=2)+'\n').encode();temp=path.with_name(path.name+'.tmp')
 with temp.open('xb') as f:f.write(data)
 os.replace(temp,path)
def pins_status(plan):
 rows=[]
 for x in plan['pins']:
  try:
   p=Path(x['path']);b=p.read_bytes();actual=sha(b);match=len(b)==x['bytes'] and actual==x['sha256'];rows.append({'path':str(p),'expectedSha256':x['sha256'],'actualSha256':actual,'bytes':len(b),'match':match})
  except OSError as e:rows.append({'path':x['path'],'match':False,'exceptionClass':type(e).__name__})
 return rows

def preflight():
 raw=(D/'PLAN.json').read_bytes();assert sha(raw)==PLAN_SHA256;plan=json.loads(raw);cmd=plan['command']
 assert plan['maxAttempts']==cmd['maxAttempts']==1 and len(plan['pins'])==22 and plan['sourceFiles']==20 and plan['tools']==2
 assert plan['innerChildBoundSeconds']==95 and plan['parentCommunicateCaptureTimeoutSeconds']==100
 assert cmd['argv']==['/usr/bin/timeout','--signal=TERM','--kill-after=5s','90s','/home/charl/.foreman/tools/fnm/node-versions/v24.21.0/installation/bin/node','--experimental-vm-modules','--test','tests/custody-failure-isolation.test.mjs']
 assert cmd['env']=={'PATH':'/usr/bin:/bin','LANG':'C.UTF-8'} and cmd['inheritEnvironment'] is False
 assert cmd['wallSeconds']==90 and cmd['terminationGraceSeconds']==5 and cmd['parentBoundSeconds']==95
 assert sha(Path(plan['commandSource']).read_bytes())==plan['commandSourceSha256'] and sha(Path(plan['assemblySource']).read_bytes())==plan['assemblySha256']
 source=json.loads(Path(plan['commandSource']).read_bytes());assert cmd==source['commands'][0] and plan['cwd']==source['cwd']
 rows=pins_status(plan);assert all(x['match'] for x in rows)
 assert Path(plan['cwd']).is_dir() and not Path(plan['outputDirectory']).exists()
 return plan,rows

def census(pgid,sid):
 # Public process identity fields only; no args, environment, config or private capture.
 raw=subprocess.check_output(['/usr/bin/ps','-eo','pid=,pgid=,sid='],timeout=2,text=True)
 return [{'pid':int(p),'pgid':int(g),'sid':int(s)} for p,g,s in (line.split() for line in raw.splitlines()) if int(g)==pgid or int(s)==sid]
def kill_group(pgid):
 try:os.killpg(pgid,signal.SIGKILL)
 except ProcessLookupError:pass

def run(plan,prepins):
 out=Path(plan['outputDirectory']);out.mkdir(mode=0o700,exist_ok=False)
 write(out/'START.json',{'startedAt':now(),'planSha256':PLAN_SHA256,'argv':plan['command']['argv'],'cwd':plan['cwd'],'env':plan['command']['env'],'inheritEnvironment':False,'pins':prepins,'attemptsConsumed':1,'attemptsRemaining':0,'scope':plan['scope']})
 p=None;start=time.monotonic();stdout=b'';stderr=b'';eof=False;captureTimedOut=False;captureError=None;exitcode=None
 try:
  p=subprocess.Popen(plan['command']['argv'],cwd=plan['cwd'],env=plan['command']['env'],stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
  write(out/'CHILD.json',{'pid':p.pid,'ownedPGID':p.pid,'ownedSID':p.pid})
  try:
   stdout,stderr=p.communicate(timeout=100);eof=True
  except subprocess.TimeoutExpired:
   captureTimedOut=True;kill_group(p.pid)
   try:stdout,stderr=p.communicate(timeout=2);eof=True
   except subprocess.TimeoutExpired as e:
    stdout=e.output or b'';stderr=e.stderr or b'';captureError='CAPTURE_EOF_NOT_REACHED_AFTER_OWNED_KILL'
    p.stdout.close();p.stderr.close()
   if p.poll() is None:p.wait(timeout=1)
  exitcode=p.returncode
 except Exception as e:
  captureError=type(e).__name__
  if p is not None:
   kill_group(p.pid)
   try:
    stdout,stderr=p.communicate(timeout=2);eof=True
   except subprocess.TimeoutExpired as late:
    stdout=late.output or b'';stderr=late.stderr or b'';p.stdout.close();p.stderr.close()
   except Exception:pass
   if p.poll() is None:
    try:p.wait(timeout=1)
    except subprocess.TimeoutExpired:pass
   exitcode=p.returncode
 elapsed=time.monotonic()-start
 # Preserve known terminal facts FIRST; postcheck errors must never erase actual exit/streams.
 with(out/'stdout.txt').open('xb') as f:f.write(stdout)
 with(out/'stderr.txt').open('xb') as f:f.write(stderr)
 result={'recordedAt':now(),'exitCode':exitcode,'elapsedSeconds':elapsed,'captureReachedEOF':eof,'stdoutReachedEOF':eof,'stderrReachedEOF':eof,'captureTimedOut':captureTimedOut,'captureError':captureError,'stdoutBytes':len(stdout),'stdoutSha256':sha(stdout),'stderrBytes':len(stderr),'stderrSha256':sha(stderr),'attemptsConsumed':1,'attemptsRemaining':0,'validationComplete':False,'sourceApprovalOrAll13Closure':False}
 write(out/'RESULT.json',result)
 validationErrors=[];remaining=None
 if p is not None:
  try:
   remaining=census(p.pid,p.pid)
   if remaining:
    kill_group(p.pid);time.sleep(.05);remaining=census(p.pid,p.pid)
  except Exception as e:validationErrors.append('ownedCensus:'+type(e).__name__)
 else:remaining=[]
 postpins=pins_status(plan);postok=all(x['match'] for x in postpins)
 write(out/'POST-PINS.json',{'all22PinsUnchanged':postok,'pins':postpins})
 write(out/'CLEANUP.json',{'recordedAt':now(),'ownedPGID':p.pid if p else None,'ownedSID':p.pid if p else None,'ownedGroupAndSIDRemaining':remaining,'ownedCleanupComplete':remaining==[],'captureReachedEOF':eof,'errors':validationErrors})
 result.update(validationComplete=True,postPinsUnchanged=postok,ownedGroupAndSIDRemaining=remaining,ownedCleanupComplete=remaining==[],validationErrors=validationErrors)
 write(out/'RESULT.json',result)
 print(json.dumps(result,indent=2))
 return 0 if exitcode==0 and eof and remaining==[] and postok and not validationErrors and captureError is None else 1

def main():
 a=argparse.ArgumentParser();a.add_argument('--preflight',action='store_true');a.add_argument('--execute-once',action='store_true');args=a.parse_args();assert args.preflight!=args.execute_once
 plan,rows=preflight()
 if args.preflight:print(json.dumps({'status':'PREFLIGHT_ONLY','pins':len(rows),'outputAbsent':True,'subjectExecuted':False,'commandIndex':1,'maxAttempts':1,'planSha256':PLAN_SHA256},indent=2));return 0
 return run(plan,rows)
if __name__=='__main__':raise SystemExit(main())
