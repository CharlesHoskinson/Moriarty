from pathlib import Path
import datetime,hashlib,json,os,selectors,signal,subprocess,time
D=Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
write=lambda n,o:(D/n).write_text(json.dumps(o,indent=2)+'\n')
p=json.loads((D/'PLAN.json').read_text())
assert sha(__file__)==p['driverSha256']
assert not (D/'START.json').exists()
def verify():
 for row in json.loads(Path(p['sourcePins']).read_text()):
  f=Path(p['checkout'])/row['path']
  assert f.stat().st_size==row['bytes'] and sha(f)==row['sha256'],str(f)
 for k in ['binary','input','expected']:
  assert sha(p[k])==p[k+'Sha256'],k
verify()
write('START.json',{'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'planSha256':sha(D/'PLAN.json')})
started=time.monotonic(); raw=b''; error=None
with (D/'STDERR.log').open('wb') as err:
 c=subprocess.Popen(['/usr/bin/timeout','--signal=TERM','--kill-after=2s','10s',p['binary'],'mcp'],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=err,cwd=p['cwd'],env=p['environment'],start_new_session=True,bufsize=0)
 write('CHILD.json',{'pid':c.pid,'ownedSession':c.pid})
 sel=selectors.DefaultSelector();sel.register(c.stdout,selectors.EVENT_READ)
 buffer=b''
 try:
  for line in Path(p['input']).read_bytes().splitlines():
   msg=json.loads(line);c.stdin.write(line+b'\n');c.stdin.flush()
   if 'id' not in msg:continue
   while b'\n' not in buffer:
    remaining=12-(time.monotonic()-started)
    if remaining<=0 or not sel.select(remaining):raise TimeoutError('response wait expired')
    chunk=os.read(c.stdout.fileno(),65536)
    if not chunk:raise EOFError('stdout closed before response')
    raw+=chunk;buffer+=chunk
   _,buffer=buffer.split(b'\n',1)
  c.stdin.close();c.wait(timeout=max(.01,12-(time.monotonic()-started)))
  raw+=c.stdout.read()
 except Exception as e:
  error=type(e).__name__+': '+str(e)
  try:os.killpg(c.pid,signal.SIGKILL)
  except ProcessLookupError:pass
  c.wait(timeout=2)
  raw+=c.stdout.read()
 finally:sel.close()
(D/'STDOUT.log').write_bytes(raw)
a=[json.loads(x) for x in raw.splitlines()]
e=[json.loads(x) for x in Path(p['expected']).read_bytes().splitlines()]
census=subprocess.run(['/usr/bin/ps','--sid',str(c.pid),'-o','pid=,sid=,stat='],capture_output=True,text=True,timeout=1)
assert census.returncode in (0,1)
verify()
result={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit':c.returncode,'wallSeconds':time.monotonic()-started,'error':error,'actual':a,'expectedResponses':len(e),'matched':[i<len(a) and a[i]==x for i,x in enumerate(e)],'allMatch':a==e,'stderrBytes':(D/'STDERR.log').stat().st_size,'ownedSessionRemaining':census.stdout.splitlines(),'sourcePinsBeforeAfter':239,'binarySha256':sha(p['binary']),'stdoutSha256':sha(D/'STDOUT.log'),'stderrSha256':sha(D/'STDERR.log'),'scope':'One ordinary positive metadata lifecycle; no source evaluation, SDK or ledger operation'}
write('RESULT.json',result)
print(json.dumps(result,indent=2));print(raw.decode());print((D/'STDERR.log').read_text())
assert c.returncode==0 and error is None and not result['ownedSessionRemaining'] and result['stderrBytes']==0
