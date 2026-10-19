from pathlib import Path
import datetime,hashlib,json,os,signal,subprocess,time
D=Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
write=lambda n,o:(D/n).write_text(json.dumps(o,indent=2)+'\n')
p=json.loads((D/'PLAN.json').read_bytes());assert sha(__file__)==p['driverSha256']
source=json.loads(Path(p['sourcePins']).read_bytes());expected=[]
def verify():
 assert sha(p['binary'])==p['binarySha256']
 assert sha(Path(p['oracle'])/'FREEZE-MANIFEST.json')==p['oracleFreezeSha256']
 for x in source:
  f=Path(p['checkout'])/x['path'];assert f.stat().st_size==x['bytes'] and sha(f)==x['sha256'],x['path']
 for x in p['inputs']:
  f=Path(p['cwd'])/x['name'];assert f.stat().st_size==x['bytes'] and sha(f)==x['sha256']
 for x in json.loads((Path(p['oracle'])/'FREEZE-MANIFEST.json').read_bytes())['artifacts']:
  f=Path(p['oracle'])/x['path'];assert f.stat().st_size==x['bytes'] and sha(f)==x['sha256']
verify();assert not (D/'START.json').exists();write('START.json',{'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'planSha256':sha(D/'PLAN.json'),'attemptConsumed':True})
rows=[]
for spec in p['cases']:
 folder=D/spec['id'];folder.mkdir();started=time.monotonic();expired=False
 argv=['/usr/bin/timeout','--signal=TERM','--kill-after=2s','10s',p['binary'],'--color','never',*spec['args']]
 with (folder/'stdout.log').open('wb') as out,(folder/'stderr.log').open('wb') as err:
  c=subprocess.Popen(argv,cwd=p['cwd'],env=p['environment'],stdin=subprocess.DEVNULL,stdout=out,stderr=err,start_new_session=True)
  write(spec['id']+'/CHILD.json',{'pid':c.pid,'ownedSession':c.pid})
  try:c.wait(timeout=12)
  except subprocess.TimeoutExpired:
   expired=True
   try:os.killpg(c.pid,signal.SIGKILL)
   except ProcessLookupError:pass
   c.wait(timeout=2)
 census=subprocess.run(['/usr/bin/ps','--sid',str(c.pid),'-o','pid=,sid=,stat='],capture_output=True,text=True,timeout=1);assert census.returncode in (0,1)
 raw=(folder/'stdout.log').read_bytes();expect=json.loads((Path(p['oracle'])/spec['object']).read_bytes())
 parse_error=None
 try:actual=json.loads(raw);object_match=actual==expect
 except Exception as e:parse_error=type(e).__name__;actual=None;object_match=False
 wire_match=None if spec['wire'] is None else raw==(Path(p['oracle'])/spec['wire']).read_bytes()
 row={'id':spec['id'],'argv':argv,'exit':c.returncode,'wallSeconds':time.monotonic()-started,'parentDeadlineExceeded':expired,'ownedSessionRemaining':census.stdout.splitlines(),'parseError':parse_error,'completeObjectMatch':object_match,'predictedRawStdoutMatch':wire_match,'stderrBytes':(folder/'stderr.log').stat().st_size,'stdoutSha256':sha(folder/'stdout.log'),'stderrSha256':sha(folder/'stderr.log'),'actual':actual,'scope':'Conditional proposed D1/D2/D3 object comparison, not adopted source/runtime conformance or native acceptance'}
 write(spec['id']+'/RESULT.json',row);rows.append(row)
verify();result={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'binarySha256':p['binarySha256'],'sourcePinsBeforeAfter':len(source),'oracleFreezeSha256':p['oracleFreezeSha256'],'cases':rows,'allFourCompleteObjectsMatch':all(x['exit']==0 and x['completeObjectMatch'] and x['parseError'] is None and not x['ownedSessionRemaining'] for x in rows),'allPredictedRawBytesMatch':all(x['predictedRawStdoutMatch'] is not False for x in rows),'scope':'Complete conditional empty check/inspect/start/manifest observations only. Start/manifest wire predicted; check/inspect raw wire and nonquietmanifeststderr not independentlyspecified. Fullfinancial/compiler/native/parentexits unchanged.'}
write('RESULT.json',result);print(json.dumps(result,indent=2))
assert result['allFourCompleteObjectsMatch'] and result['allPredictedRawBytesMatch']
