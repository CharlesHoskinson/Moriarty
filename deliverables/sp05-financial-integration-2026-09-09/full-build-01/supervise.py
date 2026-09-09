"""One-shot root supervision of the reviewed fixed launcher; no retries."""
import hashlib,json,os,pathlib,subprocess,time,shutil
R=pathlib.Path(__file__).resolve().parents[3]
D=R/'deliverables/sp05-financial-integration-2026-09-09'; E=pathlib.Path(__file__).resolve().parent
S=pathlib.Path('/home/charl/.local/state/moriarty/sp05-full-build-20260909-01')
UNIT='moriarty-sp05-financial-build.service'; CAM='sp05-financial-key-build-01'
SOURCE='a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'
def h(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def read(p):return json.loads(p.read_text())
def write(p,d):
 with p.open('x') as f:json.dump(d,f,indent=2);f.write('\n');f.flush();os.fsync(f.fileno())
 fd=os.open(p.parent,os.O_DIRECTORY);os.fsync(fd);os.close(fd)
def pin(p):return {'path':str(p),'sha256':h(p)}
def verify():
 for f in ['build-review-04/execution-manifest.json','build-review-02/toolchain-files.json']:
  for e in read(D/f)['files']:
   p=R/e['path'];assert h(p)==e['sha256'],str(p)
   if 'resolvedPath' in e:assert str(p.resolve())==e['resolvedPath']
 assert hashlib.sha256(json.dumps(read(D/'build-review-02/source-manifest.json'),separators=(',',':')).encode()).hexdigest()==SOURCE
 for e in AUTH:assert h(pathlib.Path(e['path']))==e['sha256']
def unit():
 p=subprocess.run(['systemctl','--user','show',UNIT,'--property=LoadState,ActiveState,SubState,ControlGroup,RuntimeMaxUSec,TimeoutStopUSec,SendSIGKILL,FinalKillSignal,KillMode,MemoryMax,MemorySwapMax,Result,ExecMainStatus'],capture_output=True,text=True,timeout=4)
 return dict(l.split('=',1) for l in p.stdout.splitlines() if '=' in l)
def empty(u):
 cg=u.get('ControlGroup');p=pathlib.Path('/sys/fs/cgroup')/cg.lstrip('/') if cg else None
 return u.get('ActiveState') in ('inactive','failed') and (p is None or not p.exists() or all(not x.read_text().strip() for x in p.rglob('cgroup.procs')))
AUTH=[pin(D/x) for x in ['build-review-02/gpt6-builder-review.json','build-review-02/opus-builder-result.json','build-review-04/gpt6-launcher-review.json','build-review-04/opus-launcher-result.json','build-review-02/resource-clarification.json','build-review-04/execution-manifest.json','build-review-02/toolchain-files.json','build-review-03/stop-timeout-zero-probe.json','build-review-04/probe-scope-disposition.json']]
verify();u=unit();assert u.get('LoadState')=='not-found' and empty(u),u
assert not subprocess.run(['ps','-C','compact,compactc,compactc.bin,zkir,zkir-v3,kompile,krun','-o','pid='],capture_output=True,text=True).stdout.strip()
assert shutil.disk_usage(R).free>2*1073741824+16777216
S.mkdir(mode=0o700)
for kind in ['loan','swap']:(S/(kind+'-output')).mkdir(mode=0o700)
reserve=E/'diagnostic-headroom.reserve'
with reserve.open('xb') as f:os.posix_fallocate(f.fileno(),0,16777216);os.fsync(f.fileno())
program=R/'openspec/moriarty-completion-program.json';reg=R/'evidence/moriarty-completion-program-2026-09-07/report-reconciliation/campaign-admission.json'
p=read(program);st=p['reportReconciliation']['stageAdmission']['stages'];by={s['id']:s for s in st}
assert all(by[s]['status']=='complete' for s in by['i2']['requires'])
assert by['i2']['status']=='specified-only'
ad={'schema':'moriarty.sp05-root-build-admission/1','approved':True,'createdAtMs':int(time.time()*1000),'scope':'I2 existing atomic-profile compiler key artifacts only; no transaction proofs, wallet, Docker or Preview execution; no I2 acceptance','authority':'Existing user delegated bounded successor authority; two independent agreeing resource votes retained','sourceCandidateSha':SOURCE,'executionManifestSha256':h(D/'build-review-04/execution-manifest.json'),'reviewsAndInputs':AUTH,'supervisor':pin(pathlib.Path(__file__).resolve()),'outputBase':str(S),'cases':['loan','swap'],'maxAttemptsPerCase':1,'maxCompileMsPerCase':300000,'maxServiceMsPerCase':330000,'maxTotalWallMs':720000,'resourceProposal':pin(D/'build-review-02/resource-clarification.json'),'prerequisites':[by[s] for s in by['i2']['requires']],'priorCharges':'All previous campaigns, failures, stops, compiler and proof charges preserved without reset. This allocation starts with zero attempts.','preflight':{'sourcePins':17,'toolchainPins':7,'heavyProcesses':0,'buildUnit':u,'freeBytes':shutil.disk_usage(R).free,'diagnosticHeadroomReservedBytes':16777216},'conditionsDisposition':'Current launcher reviews resolve builder C1-C5 with serial330s/aggregate720s, separately pinned17-file execution manifest, reviewed quota/retention, raw cgroup probe, and reserved outside headroom. C6 enforced here; result acceptance still requires independent terminal/artifact review.'}
write(E/'root-admission.json',ad)
registry=read(reg);assert CAM not in registry['campaigns'];registry['campaigns'][CAM]={'stage':'i2','owner':'MC02','status':'admitted-compiler-artifacts-only','binding':str((E/'root-admission.json').relative_to(R)),'bindingSha256':h(E/'root-admission.json'),'candidateManifest':str((D/'build-review-04/execution-manifest.json').relative_to(R)),'candidateSha256':ad['executionManifestSha256'],'resourceAmendment':str((D/'build-review-02/resource-clarification.json').relative_to(R)),'reviewRequirement':'Independent GPT6 and Opus source/resource approvals bound in root admission; actual result review required before acceptance','executor':'Root supervised reviewed fixed launcher; not an old registered campaign dispatch','scope':ad['scope'],'recordedAt':ad['createdAtMs'],'dispatchCondition':'One loan then one swap; stop first failed control; preserve consumed attempt files and all historical stops'}
for path,data in [(reg,registry),(program,p)]:
 if path==program:by['i2'].update(status='pending-review',acceptedProfile='moriarty-bounded-atomic/1',candidateHash=ad['executionManifestSha256'],campaignRecordId=CAM)
 write(E/(path.name+'.before.json'),read(path));temp=path.with_suffix(path.suffix+'.sp05-tmp');write(temp,data);os.replace(temp,path)
refs=[pin(D/'build-review-02/gpt6-builder-review.json'),pin(E/'opus-builder-actual-review.json')]
started=time.monotonic();results=[]
try:
 for kind in ['loan','swap']:
  verify();assert empty(unit());assert 720-(time.monotonic()-started)>330
  output=S/(kind+'-output')/'build';attempt=E/(kind+'-attempt.json');resource=E/(kind+'-resource.json');request=E/(kind+'-request.json');result=E/(kind+'-outer-result.json')
  rid=CAM+'-'+kind;deadline=int(time.time()*1000)+299900
  alloc={'id':rid,'sourceCandidateSha':SOURCE,'case':kind,'outputDir':str(output),'maxCompileMs':300000,'deadlineMs':deadline,'maxAttempts':1,'attemptFile':str(attempt)};write(resource,alloc)
  req={'case':kind,'outputDir':str(output),'sourceManifest':read(D/'build-review-02/source-manifest.json'),'admission':{'id':rid,'resourceId':rid,'candidateHash':SOURCE,'case':kind,'outputDir':str(output),'allowFullBuild':True,'maxCompileMs':300000,'deadlineMs':deadline,'maxAttempts':1,'resourceRecord':pin(resource),'reviews':refs},'resourceCounters':{'resourceId':rid,'attempts':0,'maxAttempts':1}}
  write(request,req);write(E/(kind+'-dispatch-binding.json'),{'request':pin(request),'admission':pin(E/'root-admission.json'),'elapsedMs':int((time.monotonic()-started)*1000)})
  cmd=['python3',str(R/'experiments/moriarty-midnight-financial/ledger/run-build-bounded.py'),'--request',str(request),'--request-sha256',h(request),'--result',str(result)]
  out=E/(kind+'-stdout.txt');err=E/(kind+'-stderr.txt');begin=time.monotonic();samples=[];fault=None
  print('START',kind,'request',h(request),flush=True)
  with out.open('xb') as so,err.open('xb') as se:
   child=subprocess.Popen(cmd,cwd=R,stdout=so,stderr=se)
   try:
    while child.poll() is None:
     now=time.monotonic();u=unit()
     if u.get('ActiveState')=='active' and u.get('ControlGroup'):
      group=pathlib.Path('/sys/fs/cgroup')/u['ControlGroup'].lstrip('/')
      if group.exists():
       raw={k:(group/k).read_text().strip() for k in ['memory.max','memory.swap.max'] if (group/k).exists()}
       sample={'elapsedMs':int((now-begin)*1000),'unit':u,'raw':raw}
       if len(samples)<3:samples.append(sample)
       for k,v in {'RuntimeMaxUSec':'5min 30s','TimeoutStopUSec':'0','SendSIGKILL':'yes','FinalKillSignal':'9','KillMode':'control-group','MemoryMax':'4294967296','MemorySwapMax':'0'}.items():assert u.get(k)==v,(k,u)
       assert raw=={'memory.max':'4294967296','memory.swap.max':'0'},raw
     logs=sum(x.stat().st_size for x in E.iterdir() if x.is_file() and x!=reserve)
     assert logs<16777216,'outside diagnostic ceiling exceeded'
     assert now-started<720 and now-begin<350,'external elapsed ceiling'
     time.sleep(.2)
   except BaseException as exc:
    fault=repr(exc);subprocess.run(['systemctl','--user','kill','--kill-whom=all','--signal=KILL',UNIT],capture_output=True,timeout=5)
    try:child.wait(timeout=8)
    except subprocess.TimeoutExpired:child.kill();child.wait(timeout=3)
  terminal=unit();terminal_empty=empty(terminal)
  if not terminal_empty:
   subprocess.run(['systemctl','--user','kill','--kill-whom=all','--signal=KILL',UNIT],capture_output=True,timeout=5);terminal=unit();terminal_empty=empty(terminal)
  receipt=output/'build-receipt.json';assets=[]
  rec=read(receipt) if receipt.exists() else None
  if rec:
   for f in rec.get('artifacts',[]):
    assets.append(f)
  observed={'case':kind,'exitCode':child.returncode,'fault':fault,'wallMs':int((time.monotonic()-begin)*1000),'attemptCharged':attempt.exists(),'unitSamples':samples,'terminal':terminal,'terminalCgroupEmpty':terminal_empty,'receipt':pin(receipt) if receipt.exists() else None,'receiptStatus':rec.get('status') if rec else None,'artifactCount':len(assets),'accepted':False,'transactionProofs':0,'submissions':0}
  write(E/(kind+'-observed.json'),observed);results.append(observed);print('FINISH',kind,json.dumps(observed),flush=True)
  assert child.returncode==0 and fault is None and terminal_empty and samples and rec and rec['status']=='built','stop: required build/control failed'
  verify()
finally:
 write(E/'run-summary.json',{'elapsedMs':int((time.monotonic()-started)*1000),'cases':results,'accepted':False,'priorChargesPreserved':True,'scope':'compiler key artifacts only','transactionProofs':0,'walletOperations':0,'submissions':0})
 reserve.unlink()
