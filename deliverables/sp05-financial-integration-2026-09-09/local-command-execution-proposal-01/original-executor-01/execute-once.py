# PROPOSED ONLY. Never import/execute for checks: use AST extraction.
import pathlib,json,hashlib,os,subprocess,time,shlex,shutil,urllib.request,socket,datetime,sys,stat
R=pathlib.Path('/home/charl/Moriarty/.worktrees/sp05-local-completion');D=R/'deliverables/sp05-financial-integration-2026-09-09';PACKET=D/'local-command-execution-proposal-01'
if sys.argv[1:] not in [['--case','loan'],['--case','swap']]:raise SystemExit('EXACT_LOCAL_CASE_REQUIRED')
kind=sys.argv[2];O=PACKET/kind;admission=PACKET/('execution-admission-'+kind+'.json')
if not __debug__:raise SystemExit('ASSERTIONS_REQUIRED')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
A=json.loads(admission.read_text());assert A['status']=='ADMITTED'
files={f['path']:f['sha256'] for f in A['files']};assert len(files)==len(A['files'])
def bound(ref):
 rel=pathlib.Path(ref['path']);assert not rel.is_absolute() and '..' not in rel.parts
 p=R/rel
 for q in [p,*p.parents]:assert not q.is_symlink()
 assert p.is_file() and files.get(ref['path'])==ref['sha256'] and sha(p)==ref['sha256'];return p
for f in A['files']:bound(f)
P=json.loads(bound({'path':str((PACKET/'resource-proposal-02.json').relative_to(R)),'sha256':A['proposalSha256']}).read_text())
C=next(c for c in P['cases'] if c['kind']==kind)
assert A['case']==kind and A['allocationId']=='sp05-local-'+kind+'-completion-20260910-01'
for path,digest in P['packetFiles'].items():bound({'path':path,'sha256':digest})
assert P['sourceCandidate']['path']!='UNBOUND' and P['sourceCandidate']['sha256']!='UNBOUND'
assert A['sourceCandidate']==P['sourceCandidate'];candidate=bound(A['sourceCandidate'])
source=json.loads(candidate.read_text());assert isinstance(source['files'],dict) and source['files']
for p,digest in source['files'].items():bound({'path':p,'sha256':digest})
sdkRoot=pathlib.Path('/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules')
pinPath='experiments/moriarty-midnight-financial/ledger/launch-runtime-pins.json'
pins=json.loads(bound({'path':pinPath,'sha256':source['files'][pinPath]}).read_text())
for package,pin in pins.items():
 for relative,digest in pin['files'].items():
  sdkFile=sdkRoot/package/relative;assert sdkFile.is_relative_to(sdkRoot) and '..' not in sdkFile.parts
  for q in [sdkFile,*sdkFile.parents]:assert not q.is_symlink()
  assert sdkFile.is_file() and sha(sdkFile)==digest,'SDK_SOURCE_DRIFT'
for path,digest in P['runtimeClosure'].items():bound({'path':path,'sha256':digest})
accepted=json.loads(bound(P['acceptedCompletionApproval']).read_text());assert accepted['candidateSha256']==P['acceptedCompletionSource']['sha256'] and accepted['verdict']=='PASS_SCOPED_SOURCE_ONLY'
bound(P['acceptedCompletionSource'])
for name,digest in accepted['reviews'].items():bound({'path':str((bound(P['acceptedCompletionApproval']).parent/name).relative_to(R)),'sha256':digest})
for group,field,digest,verdicts in [('sourceReviews','candidateSha256',A['sourceCandidate']['sha256'],['PASS_SCOPED','APPROVED']),('resourceVotes','proposalSha256',A['proposalSha256'],['APPROVE','APPROVED','APPROVE_BOUNDED_RUN'])]:
 refs=A[group];assert len(refs)==2 and {r['reviewer'] for r in refs}=={'gpt-6-astra','grok-4.6'}
 for ref in refs:
  receipt=json.loads(bound(ref).read_text());assert receipt['reviewer']==ref['reviewer'] and receipt['verdict'] in verdicts and receipt[field]==digest;bound(receipt['actualReview'])
for relative,digest in [('full-build-01/gpt6-build-result-review.json','f8baed4d44ac753ae856fabf7fae98b059a28203707437d34e1f6f96de6601dc'),('grok-build-review-02/review.json','0109d4b2f5995393ec63d0ece04bb0fefa60417011bd3d74c19a6b64ea4f3a69')]:
 rel=str((D/relative).relative_to(R));assert json.loads(bound({'path':rel,'sha256':digest}).read_text())['verdict']=='APPROVED'
probe=D/'local-execution-04/committee-probe.mjs';assert files.get(str(probe.relative_to(R)))=='1baf836d99dab191a8af61b91ee54c43f1c6037ae812a28e2e051338adca5c29'
assert C['attempts']==1 and C['submissionMax']==4 and C['dustFeeSpeck']=='2000000000000000'
assert C['grossByLogicalAsset']==({'USD_TEST_ASSET':'20000000000'} if kind=='loan' else {'ASSET_A':'100000','ASSET_B':'0'})
assert P['priorCharges']['priorCombinedReservations']==27 and P['priorCharges']['priorCombinedReservedSpeck']=='8100000000000027'
unit=C['launcherUnit'];diagnostic='moriarty-sp05-local-'+kind+'-completion-preflight-01.service';timer=C['stopUnit'];base=pathlib.Path(C['base'])
assert unit=='moriarty-sp05-local-'+kind+'-completion-01.service' and timer=='moriarty-sp05-local-'+kind+'-completion-stop-01'
assert str(base)=='/home/charl/.local/state/moriarty/sp05-local-'+kind+'-completion-20260910-01'
plan=json.loads(bound(C['publicPlan']).read_text());assert plan['kind']==kind and plan['limits']['deadlineMs']==0 and plan['limits']['allocationId']==A['allocationId']
assert plan['limits']['submissions']==4 and plan['limits']['dustFee']==C['dustFeeSpeck'] and plan['limits']['grossByLogicalAsset']==C['grossByLogicalAsset']
assert plan['outputDirectory']==str(base/'run') and plan['privateState']['directory']==str(base/'contract-state')
network=plan['networkConfig'];assert network['networkId']=='undeployed'
names=['moriarty-midnight-proof-server','moriarty-midnight-indexer','moriarty-midnight-node'];timerLaunchAttempted=False;timerActiveObserved=False
os.umask(0o077)
def run(argv,timeout=10,input=None):return subprocess.run(argv,capture_output=True,text=True,timeout=timeout,check=True,input=input).stdout
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
def setupcheck():assert time.monotonic()<setupDeadline,'SETUP_DEADLINE'
def activate(argv,record,input=None,isDiagnostic=False):
 # The exact dispatched argv is durable before requesting manager activation.
 setupcheck();activationAllowance=int(setupDeadline-time.monotonic())-1;assert activationAllowance>=1
 argv=list(argv);argv.insert(4,'--property=TimeoutStartSec='+str(activationAllowance)+'s')
 if isDiagnostic:argv.insert(5,'--property=RuntimeMaxSec='+str(activationAllowance)+'s')
 save(O/record,{'argv':argv,'activationDeadlineMonotonicSeconds':setupDeadline,'activationAllowanceSeconds':activationAllowance})
 setupcheck();remaining=setupDeadline-time.monotonic();assert remaining>=activationAllowance,'ACTIVATION_RESERVE_CONSUMED'
 result=run(argv,timeout=remaining,input=input);setupcheck();return result
def jsonpost(url,payload):
 setupcheck();req=urllib.request.Request(url,data=json.dumps(payload).encode(),headers={'Content-Type':'application/json'})
 with urllib.request.urlopen(req,timeout=min(5,setupDeadline-time.monotonic())) as response:
  raw=response.read(65537);assert len(raw)<=65536;result=json.loads(raw)
 setupcheck();return result
other='swap' if kind=='loan' else 'loan'
otherUnits=['moriarty-sp05-local-'+other+'-completion-01.service','moriarty-sp05-local-'+other+'-completion-preflight-01.service','moriarty-sp05-local-'+other+'-completion-stop-01.timer','moriarty-sp05-local-'+other+'-completion-stop-01.service']
for name in [unit,diagnostic,timer+'.timer',timer+'.service',*otherUnits]:assert run(['systemctl','--user','show',name,'--property=LoadState','--value']).strip()=='not-found'
for path in [base,*base.parents]:assert not path.is_symlink()
assert not base.exists() and not O.exists()
assert P['timeSecondsPerCase']['setupAndActivationMax']==120 and P['timeSecondsPerCase']['walletOperationMax']==1200 and P['timeSecondsPerCase']['independentStopAfterArming']==1300 and P['timeSecondsPerCase']['cleanupMax']==31 and P['timeSecondsPerCase']['envelopeMax']==1350
for path,isDirectory in [(base.parent,True),(pathlib.Path(plan['wallet']['stateDirectory']),True),*[(pathlib.Path(plan[k][f]),False) for k,f in [('wallet','seedFile'),('roles','secretsFile'),('privateState','passwordFile')]],*[(pathlib.Path(plan['wallet']['stateDirectory'])/'.midnight-wallet-state/undeployed'/(child+'.json'),False) for child in ['shielded','unshielded','dust']]]:
 for q in [path,*path.parents]:assert not q.is_symlink()
 st=path.lstat();assert st.st_uid==os.getuid() and stat.S_IMODE(st.st_mode)==(0o700 if isDirectory else 0o600) and (stat.S_ISDIR(st.st_mode) if isDirectory else stat.S_ISREG(st.st_mode))
mem=dict(x.split(':',1) for x in pathlib.Path('/proc/meminfo').read_text().splitlines());assert int(mem['MemAvailable'].split()[0])*1024>=P['memoryAvailableMinimumBytes'];assert shutil.disk_usage(base.parent).free>=P['freeDiskMinimumBytes']
for c in json.loads(run(['docker','inspect',*names])):
 pin=next(p for p in P['containers'] if '/'+p['name']==c['Name']);h=c['HostConfig'];assert c['State']['Status']=='exited' and c['Image']=='sha256:'+pin['imageSha256'];assert h['Memory']==pin['memory'] and h['MemorySwap']==pin['memorySwapTotal'] and h['NanoCpus']==pin['nanoCpus'] and h['RestartPolicy']['Name']=='no'
timerArgv=['/usr/bin/systemd-run','--user','--unit='+timer,'--on-active=1300s','--timer-property=AccuracySec=1s','--property=Type=oneshot','--property=TimeoutStartSec=25s','--property=TimeoutStopSec=5s','--property=ExecStopPost=/usr/bin/timeout --signal=KILL 5s /usr/bin/docker kill '+' '.join(names),'/usr/bin/bash','-c','/usr/bin/timeout --signal=KILL 5s /usr/bin/systemctl --user kill --signal=KILL --kill-whom=all '+unit+' '+diagnostic+'; /usr/bin/timeout --signal=KILL 20s /usr/bin/docker stop -t 5 '+' '.join(names)]
O.mkdir(mode=0o700);syncdir(PACKET)
save(O/'attempt.json',{'schema':'moriarty.local-command-attempt/1','status':'CONSUMED','allocationId':A['allocationId'],'admissionSha256':shaadmission,'timerArgv':timerArgv,'retryAllowed':False,'priorCharges':P['priorCharges'],'otherCaseAttemptSha256':sha(PACKET/other/'attempt.json') if (PACKET/other/'attempt.json').is_file() else None})
try:
 timerLaunchAttempted=True;run(timerArgv)
 obj=shlex.split(run(['busctl','--user','call','org.freedesktop.systemd1','/org/freedesktop/systemd1','org.freedesktop.systemd1.Manager','GetUnit','s',timer+'.timer']))[1]
 def prop(key):return int(shlex.split(run(['busctl','--user','get-property','org.freedesktop.systemd1',obj,'org.freedesktop.systemd1.Timer',key]))[1])
 nextUS=prop('NextElapseUSecMonotonic');accuracy=prop('AccuracyUSec');assert 0<accuracy<=1000000
 arm=nextUS/1e6-1300;setupDeadline=arm+120
 mono=time.monotonic();wall=time.time();shutdownWallMs=int((wall+nextUS/1e6-mono)*1000)-int((time.monotonic()-mono)*1000)-1
 assert run(['systemctl','--user','is-active',timer+'.timer']).strip()=='active';timerActiveObserved=True;setupcheck()
 save(O/'timer-armed.json',{'argv':timerArgv,'armingMonotonicSeconds':arm,'nextElapseMonotonicMicroseconds':nextUS,'accuracyMicroseconds':accuracy,'setupDeadlineMonotonicSeconds':setupDeadline,'shutdownWallMsProjection':shutdownWallMs})
 setupcheck();run(['docker','start','moriarty-midnight-node'],timeout=setupDeadline-time.monotonic());setupcheck()
 while True:
  setupcheck()
  try:
   genesis=jsonpost(network['node'],{'jsonrpc':'2.0','id':1,'method':'chain_getBlockHash','params':[0]})['result'];break
  except Exception:time.sleep(min(1,max(0,setupDeadline-time.monotonic())))
 assert genesis=='0xe72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846'
 setupcheck();run(['docker','start','moriarty-midnight-indexer','moriarty-midnight-proof-server'],timeout=setupDeadline-time.monotonic());setupcheck()
 while True:
  setupcheck()
  try:
   indexed=jsonpost(network['indexer'],{'query':'{ block(offset: null) { height hash timestamp } }'});assert indexed.get('data',{}).get('block')
   with socket.create_connection(('127.0.0.1',16300),timeout=min(5,setupDeadline-time.monotonic())):pass
   break
  except Exception:time.sleep(min(1,max(0,setupDeadline-time.monotonic())))
 setupcheck();deadlineMs=min(int(time.time()*1000)+1200000,shutdownWallMs-5000)
 plan['limits']['deadlineMs']=deadlineMs
 common=['--property=MemoryMax=4294967296','--property=MemorySwapMax=0','--property=CPUQuota=200%','--property=KillMode=control-group','--property=FinalKillSignal=SIGKILL','--property=SendSIGKILL=yes','--property=UMask=0077']
 diagnosticArgv=['/usr/bin/systemd-run','--user','--unit='+diagnostic.removesuffix('.service'),'--property=Type=exec','--quiet','--pipe','--wait','--property=TimeoutStopSec=1',*common,'--working-directory='+str(R),'/usr/local/bin/node',str(PACKET/'public-preflight.mjs')]
 setupcheck();input=json.dumps({'plan':plan,'setupDeadlineMs':int((time.time()+setupDeadline-time.monotonic())*1000)})
 try:output=activate(diagnosticArgv,'resolved-preflight.json',input=input,isDiagnostic=True)
 except subprocess.CalledProcessError as error:output=error.stdout or ''
 assert len(output.encode())<=65536;lines=output.splitlines()
 try:public=json.loads(lines[-1])
 except Exception:public={'status':'PUBLIC_PREFLIGHT_FAILED','code':'PUBLIC_OUTPUT_INVALID'}
 save(O/'public-preflight-result.json',public)
 assert public['status']=='FRESH_LOCAL_PUBLIC_VERIFIED' and public['kind']==kind and public['receiptSha256']==plan['build']['receiptSha256'] and public['networkTag']==plan['networkTag']
 setupcheck();base.mkdir(mode=0o700);syncdir(base.parent);syncdir(base)
 privateState=base/'contract-state';privateState.mkdir(mode=0o700);syncdir(privateState);syncdir(base)
 assert not any(privateState.iterdir()) and not (base/'run').exists()
 for name in ['sdk.stdout','sdk.stderr']:
  fd=os.open(base/name,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600)
  try:os.fsync(fd)
  finally:os.close(fd)
 syncdir(base)
 save(base/'plan.json',plan);planhash=sha(base/'plan.json')
 argv=['/usr/bin/systemd-run','--user','--unit='+unit.removesuffix('.service'),'--property=Type=exec','--property=RemainAfterExit=yes',*common,'--property=RuntimeMaxSec=1206','--property=TimeoutStopSec=5','--property=StandardOutput=append:'+str(base/'sdk.stdout'),'--property=StandardError=append:'+str(base/'sdk.stderr'),'--property=ExecStopPost=/usr/bin/timeout --signal=KILL 20s /usr/bin/docker stop -t 5 '+' '.join(names),'--working-directory='+str(R/'experiments/moriarty-midnight-financial'),'/usr/local/bin/node','ledger/launch-local.mjs','--run','--plan',str(base/'plan.json'),'--sha256',planhash]
 setupcheck();assert run(['systemctl','--user','is-active',timer+'.timer'],timeout=setupDeadline-time.monotonic()).strip()=='active'
 activate(argv,'resolved-launch.json')
 fields=dict(x.split('=',1) for x in run(['systemctl','--user','show',unit,'--property=LoadState,ActiveState,SubState,MainPID,ControlGroup,ActiveEnterTimestampMonotonic,InvocationID'],timeout=setupDeadline-time.monotonic()).splitlines());setupcheck();assert fields['ActiveState']=='active' and int(fields['MainPID'])>0 and arm<=int(fields['ActiveEnterTimestampMonotonic'])/1e6<=setupDeadline
 assert fields['LoadState']=='loaded' and len(fields['InvocationID'])==32 and all(c in '0123456789abcdef' for c in fields['InvocationID']) and int(fields['InvocationID'],16)>0
 save(O/'launcher-active.json',{'observed':fields,'planSha256':planhash,'deadlineMs':deadlineMs,'elapsedSinceArmingSeconds':time.monotonic()-arm})
 print('Fresh local '+kind+' launcher active. Retain loaded terminal exit before explicit stop; independent containment before timer cancellation.')
except BaseException as error:
 cleanup=[]
 if timerLaunchAttempted:
  for cmd in [['/usr/bin/timeout','--signal=KILL','5s','systemctl','--user','kill','--signal=KILL','--kill-whom=all',unit,diagnostic],['/usr/bin/timeout','--signal=KILL','20s','docker','stop','-t','5',*names],['/usr/bin/timeout','--signal=KILL','5s','docker','kill',*names]]:
   try:
    result=subprocess.run(cmd,capture_output=True,timeout=22);cleanup.append({'argv':cmd,'returnCode':result.returncode})
   except Exception as stopError:cleanup.append({'argv':cmd,'errorClass':type(stopError).__name__})
 save(O/'setup-failure.json',{'status':'FAILED','cleanupActions':cleanup,'errorClass':type(error).__name__,'timerLaunchAttempted':timerLaunchAttempted,'timerActiveObserved':timerActiveObserved,'retryAllowed':False,'scope':'Verify both cgroups and all three containers actually stopped; no retry or refund.'})
 raise SystemExit('Local completion setup failed; retain evidence and inspect containment. No retry.')
