# PROPOSED ONLY. Never import or execute for checks; use AST extraction.
import pathlib,json,hashlib,os,subprocess,time,shlex,shutil,socket,stat
R=pathlib.Path('/home/charl/Moriarty/.worktrees/sp05-preview-owner');D=R/'deliverables/sp05-financial-integration-2026-09-09';O=D/'preview-loan-01'
if not __debug__:raise SystemExit('ASSERTIONS_REQUIRED')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
A=json.loads((O/'execution-admission.json').read_text());assert A['status']=='ADMITTED'
files={f['path']:f['sha256'] for f in A['files']};assert len(files)==len(A['files'])
def bound(ref):
 rel=pathlib.Path(ref['path']);assert not rel.is_absolute() and '..' not in rel.parts
 p=R/rel
 for q in [p,*p.parents]:assert not q.is_symlink()
 assert p.is_file() and files.get(ref['path'])==ref['sha256'] and sha(p)==ref['sha256'];return p
for f in A['files']:bound(f)
P=json.loads(bound({'path':str((O/'resource-proposal-draft.json').relative_to(R)),'sha256':A['proposalSha256']}).read_text())
assert P['sourceCandidate'] and A['sourceCandidate']==P['sourceCandidate']
source=json.loads(bound(A['sourceCandidate']).read_text());assert isinstance(source['files'],dict) and source['files']
for p,digest in source['files'].items():bound({'path':p,'sha256':digest})
for name in ['execute-once.py','COMMANDS-DRAFT.md','public-plan-draft.json','resource-proposal-draft.json','metadata-observation.json','check-static.py']:assert str((O/name).relative_to(R)) in files
for name in ['preview-bootstrap.mjs','preview-launch.mjs','integrate-preview.mjs','launch-runtime-pins.json','providers.mjs','receipt.mjs','financial-rpc.mjs','indexed-owner.mjs']:
 assert 'experiments/moriarty-midnight-financial/ledger/'+name in source['files']
pins=json.loads((R/'experiments/moriarty-midnight-financial/ledger/launch-runtime-pins.json').read_text())
sdkRoot=pathlib.Path('/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules')
for package,pin in pins.items():
 for relative,digest in pin['files'].items():
  p=sdkRoot/package/relative;assert '..' not in p.parts and p.is_relative_to(sdkRoot)
  for q in [p,*p.parents]:assert not q.is_symlink()
  assert p.is_file() and sha(p)==digest,'SDK_SOURCE_DRIFT'
for group,field,digest,verdicts in [('sourceReviews','candidateSha256',A['sourceCandidate']['sha256'],['PASS_SCOPED','APPROVED']),('resourceVotes','proposalSha256',A['proposalSha256'],['APPROVE','APPROVED','APPROVE_BOUNDED_RUN'])]:
 refs=A[group];assert len(refs)==2 and {r['reviewer'] for r in refs}=={'gpt-6-astra','grok-4.6'}
 for ref in refs:
  receipt=json.loads(bound(ref).read_text());assert receipt['reviewer']==ref['reviewer'] and receipt['verdict'] in verdicts and receipt[field]==digest;bound(receipt['actualReview'])
for relative,digest in [('full-build-01/gpt6-build-result-review.json','f8baed4d44ac753ae856fabf7fae98b059a28203707437d34e1f6f96de6601dc'),('grok-build-review-02/review.json','0109d4b2f5995393ec63d0ece04bb0fefa60417011bd3d74c19a6b64ea4f3a69')]:
 rel=str((D/relative).relative_to(R));assert json.loads(bound({'path':rel,'sha256':digest}).read_text())['verdict']=='APPROVED'
plan=json.loads(bound(P['publicPlanDraft']).read_text())
assert plan['schema']=='moriarty.preview-financial-launch/1' and plan['kind']=='loan' and plan['limits']['deadlineMs']==0
assert P['attempts']==1 and P['submissions']==plan['limits']['submissions']==4
assert P['dustFeeSpeck']==plan['limits']['dustFee']=='2000000000000000' and P['grossByLogicalAsset']==plan['limits']['grossByLogicalAsset']=={'USD_TEST_ASSET':'20000000000'}
assert P['proofAccounting']['expectedLogicalFinancialStages']==4 and P['proofAccounting']['applicationRetries']==0
assert P['proofAccounting']['hardProofCallbackCountEnforced'] is False
assert P['priorCharges']['priorLocalReservedSubmissions']==11 and P['priorCharges']['priorLocalReservedDustSpeck']=='3300000000000011' and P['priorCharges']['priorLocalAdmittedDustCeilingSpeck']=='4000000000000010'
for k,v in {'activationAndSetupMax':120,'operationDeadlineAfterArming':2490,'bootstrapHardExitAfterOperationDeadline':6,'independentShutdownAfterArming':2500,'cleanupMax':31,'latestPlannedContainmentAfterArming':2531,'envelopeMax':2550}.items():assert P['timeSeconds'][k]==v
unit=P['launcherService']['unit'];timer=P['launcherService']['stopUnit'];base=pathlib.Path(P['privatePreparation']['base'])
assert unit=='moriarty-sp05-preview-loan-01.service' and timer=='moriarty-sp05-preview-loan-stop-01'
assert str(base)=='/home/charl/.local/state/moriarty/sp05-preview-loan-20260910-01'
assert A['allocationId']==plan['limits']['allocationId']=='sp05-preview-loan-20260910-01'
assert plan['outputDirectory']==str(base/'run') and plan['privateState']['directory']==str(base/'contract-state')
names=['moriarty-midnight-proof-server'];stopped=['moriarty-midnight-node','moriarty-midnight-indexer'];timerLaunchAttempted=False;timerActiveObserved=False
os.umask(0o077)
def run(argv,timeout=10):return subprocess.run(argv,capture_output=True,text=True,timeout=timeout,check=True).stdout
def syncdir(path):
 fd=os.open(path,os.O_RDONLY|os.O_DIRECTORY)
 try:os.fsync(fd)
 finally:os.close(fd)
def save(path,value):
 raw=(json.dumps(value,indent=2)+'\n').encode();assert len(raw)<=65536
 fd=os.open(path,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600)
 try:
  remaining=memoryview(raw)
  while remaining:
   n=os.write(fd,remaining);assert n>0;remaining=remaining[n:]
  os.fsync(fd)
 finally:os.close(fd)
 syncdir(path.parent)
def privateMetadata(path,directory=False):
 p=pathlib.Path(path)
 for q in [p,*p.parents]:assert not q.is_symlink()
 s=p.lstat();assert s.st_uid==os.getuid() and stat.S_IMODE(s.st_mode)==(0o700 if directory else 0o600)
 assert stat.S_ISDIR(s.st_mode) if directory else stat.S_ISREG(s.st_mode)
def setupcheck():assert time.monotonic()<setupDeadline,'SETUP_DEADLINE'
def activate(argv,record):
 setupcheck();activationAllowance=int(setupDeadline-time.monotonic())-1;assert activationAllowance>=1
 argv=list(argv);argv.insert(4,'--property=TimeoutStartSec='+str(activationAllowance)+'s')
 save(O/record,{'argv':argv,'activationDeadlineMonotonicSeconds':setupDeadline,'activationAllowanceSeconds':activationAllowance})
 setupcheck();remaining=setupDeadline-time.monotonic();assert remaining>=activationAllowance,'ACTIVATION_RESERVE_CONSUMED'
 result=run(argv,timeout=remaining);setupcheck();return result
for name in [unit,timer+'.timer',timer+'.service']:assert run(['systemctl','--user','show',name,'--property=LoadState','--value']).strip()=='not-found'
for path in [base,*base.parents]:assert not path.is_symlink()
assert not base.exists() and not (O/'attempt.json').exists();privateMetadata(base.parent,True)
for path in [plan['wallet']['seedFile'],plan['roles']['secretsFile'],plan['privateState']['passwordFile']]:privateMetadata(path)
privateMetadata(plan['wallet']['stateDirectory'],True)
snapshot=pathlib.Path(plan['wallet']['stateDirectory'])/'.midnight-wallet-state/preview';privateMetadata(snapshot,True)
for kind in ['shielded','unshielded','dust']:privateMetadata(snapshot/(kind+'.json'))
for name in ['.moriarty-persistence-pending.json','.moriarty-backup-'+plan['limits']['allocationId']]:assert not (snapshot/name).exists() and not (snapshot/name).is_symlink()
mem=dict(x.split(':',1) for x in pathlib.Path('/proc/meminfo').read_text().splitlines());assert int(mem['MemAvailable'].split()[0])*1024>=P['storage']['minimumHostAvailableMemoryBytes'];assert shutil.disk_usage(base.parent).free>=P['storage']['minimumFreeBytes']
assert len(P['containersAllowedToStart'])==1 and {x['name'] for x in P['containersMustRemainStopped']}==set(stopped)
for c in json.loads(run(['docker','inspect',*names,*stopped])):
 pin=next(p for p in P['containersAllowedToStart']+P['containersMustRemainStopped'] if '/'+p['name']==c['Name']);assert c['State']['Status']=='exited' and c['Image']==pin['image']
 if c['Name']=='/'+names[0]:
  h=c['HostConfig'];assert h['Memory']==5368709120 and h['MemorySwap']==10737418240 and h['NanoCpus']==4000000000 and h['RestartPolicy']['Name']=='no'
  assert h['PortBindings']=={'6300/tcp':[{'HostIp':'127.0.0.1','HostPort':'16300'}]}
timerArgv=['/usr/bin/systemd-run','--user','--unit='+timer,'--on-active=2500s','--timer-property=AccuracySec=1s','--property=Type=oneshot','--property=TimeoutStartSec=25s','--property=TimeoutStopSec=5s','--property=ExecStopPost=/usr/bin/timeout --signal=KILL 5s /usr/bin/docker kill '+names[0],'/usr/bin/bash','-c','/usr/bin/timeout --signal=KILL 5s /usr/bin/systemctl --user kill --signal=KILL --kill-whom=all '+unit+'; /usr/bin/timeout --signal=KILL 20s /usr/bin/docker stop -t 5 '+names[0]]
save(O/'attempt.json',{'schema':'moriarty.preview-loan-attempt/1','status':'CONSUMED','allocationId':A['allocationId'],'admissionSha256':sha(O/'execution-admission.json'),'timerArgv':timerArgv,'retryAllowed':False,'priorCharges':P['priorCharges']})
try:
 timerLaunchAttempted=True;run(timerArgv)
 obj=shlex.split(run(['busctl','--user','call','org.freedesktop.systemd1','/org/freedesktop/systemd1','org.freedesktop.systemd1.Manager','GetUnit','s',timer+'.timer']))[1]
 def prop(key):return int(shlex.split(run(['busctl','--user','get-property','org.freedesktop.systemd1',obj,'org.freedesktop.systemd1.Timer',key]))[1])
 nextUS=prop('NextElapseUSecMonotonic');accuracy=prop('AccuracyUSec');assert 0<accuracy<=1000000
 arm=nextUS/1e6-2500;setupDeadline=arm+120;operationDeadline=arm+2490
 assert run(['systemctl','--user','is-active',timer+'.timer']).strip()=='active';timerActiveObserved=True;setupcheck()
 # Sample monotonic first so sampling delay shortens, never extends, the wall projection.
 mono=time.monotonic();wall=time.time();deadlineMs=int((wall+operationDeadline-mono)*1000)
 # Subtract elapsed sampling time to retain a conservative projection.
 deadlineMs-=int((time.monotonic()-mono)*1000)+1
 save(O/'timer-armed.json',{'argv':timerArgv,'armingMonotonicSeconds':arm,'nextElapseMonotonicMicroseconds':nextUS,'accuracyMicroseconds':accuracy,'setupDeadlineMonotonicSeconds':setupDeadline,'operationDeadlineMonotonicSeconds':operationDeadline,'operationDeadlineMs':deadlineMs})
 setupcheck();run(['docker','start',*names],timeout=setupDeadline-time.monotonic());setupcheck()
 ready=False
 for sample in range(30):
  setupcheck()
  try:
   with socket.create_connection(('127.0.0.1',16300),timeout=min(2,setupDeadline-time.monotonic())):pass
   ready=True;break
  except OSError:
   setupcheck()
   if sample<29:time.sleep(min(1,max(0,setupDeadline-time.monotonic())))
 setupcheck();assert ready,'PROOF_LISTENER_UNAVAILABLE'
 base.mkdir(mode=0o700);syncdir(base.parent);syncdir(base);setupcheck()
 plan['limits']['deadlineMs']=deadlineMs;save(base/'plan.json',plan);planhash=sha(base/'plan.json');setupcheck()
 # Raw SDK diagnostics stay private; pre-create both exclusively before manager activation.
 for logname in ['sdk.stdout','sdk.stderr']:
  fd=os.open(base/logname,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600)
  try:os.fsync(fd)
  finally:os.close(fd)
 syncdir(base);setupcheck()
 runtimeSeconds=int(operationDeadline-time.monotonic())+6;assert 1<=runtimeSeconds<=2496
 argv=['/usr/bin/systemd-run','--user','--unit='+unit.removesuffix('.service'),'--property=Type=exec','--property=MemoryMax=4294967296','--property=MemorySwapMax=0','--property=CPUQuota=200%','--property=KillMode=control-group','--property=FinalKillSignal=SIGKILL','--property=SendSIGKILL=yes','--property=UMask=0077','--property=RuntimeMaxSec='+str(runtimeSeconds),'--property=TimeoutStopSec=1','--property=StandardOutput=append:'+str(base/'sdk.stdout'),'--property=StandardError=append:'+str(base/'sdk.stderr'),'--property=ExecStopPost=/usr/bin/timeout --signal=KILL 20s /usr/bin/docker stop -t 5 '+names[0],'--working-directory='+str(R),'/usr/local/bin/node',str(R/'experiments/moriarty-midnight-financial/ledger/preview-bootstrap.mjs'),'--run','--plan',str(base/'plan.json'),'--sha256',planhash]
 assert run(['systemctl','--user','is-active',timer+'.timer'],timeout=setupDeadline-time.monotonic()).strip()=='active';activate(argv,'resolved-launch.json')
 fields=dict(x.split('=',1) for x in run(['systemctl','--user','show',unit,'--property=ActiveState,SubState,MainPID,ControlGroup,ActiveEnterTimestampMonotonic'],timeout=setupDeadline-time.monotonic()).splitlines());setupcheck();assert fields['ActiveState']=='active' and int(fields['MainPID'])>0 and arm<=int(fields['ActiveEnterTimestampMonotonic'])/1e6<=setupDeadline
 save(O/'launcher-active.json',{'observed':fields,'planSha256':planhash,'deadlineMs':deadlineMs,'elapsedSinceArmingSeconds':time.monotonic()-arm});setupcheck()
 print('Preview loan launcher active. Retain public IDs and stage results; stop on UNKNOWN. Verify actual containment before timer cancellation.')
except BaseException as error:
 if timerLaunchAttempted:
  for cmd in [['/usr/bin/timeout','--signal=KILL','5s','systemctl','--user','kill','--signal=KILL','--kill-whom=all',unit],['/usr/bin/timeout','--signal=KILL','20s','docker','stop','-t','5',*names],['/usr/bin/timeout','--signal=KILL','5s','docker','kill',*names]]:
   try:subprocess.run(cmd,capture_output=True,timeout=22)
   except Exception:pass
 save(O/'setup-failure.json',{'status':'FAILED','errorClass':type(error).__name__,'timerLaunchAttempted':timerLaunchAttempted,'timerActiveObserved':timerActiveObserved,'retryAllowed':False,'scope':'Verify launcher cgroup absent, proof server exited and local node/indexer still stopped. No retry or refund.'})
 raise SystemExit('Preview setup failed; preserve evidence and verify containment. No retry.')
