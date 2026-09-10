# Draft: one local recovery attempt, executable only with exact reviewed admission.
# Never import this operational script for checks. Parse its AST instead.
import pathlib,json,hashlib,subprocess,time,datetime,os,secrets,urllib.request,shlex
R=pathlib.Path('/home/charl/Moriarty/.worktrees/sp05-deadline-review');E=pathlib.Path('/home/charl/Moriarty/.worktrees/sp05-financial-integration');D=R/'deliverables/sp05-financial-integration-2026-09-09';O=D/'local-recovery-03'
sha=lambda b:hashlib.sha256(b).hexdigest()
if not __debug__:
 raise SystemExit('ASSERTIONS_REQUIRED: optimized Python is forbidden')
A=json.loads((O/'execution-admission.json').read_text());assert A['status']=='ADMITTED'
def public_path(relative):
 p=R/relative;assert not pathlib.Path(relative).is_absolute() and '..' not in pathlib.Path(relative).parts
 for q in [p,*p.parents]:assert not q.is_symlink()
 assert p.is_file();return p
def bound(ref):
 p=public_path(ref['path']);assert sha(p.read_bytes())==ref['sha256'];return p
files={f['path']:f['sha256'] for f in A['files']};assert len(files)==len(A['files'])
for f in A['files']:bound(f)
def admitted(ref):
 assert files.get(ref['path'])==ref['sha256'];return bound(ref)
proposal=admitted({'path':A['proposalPath'],'sha256':A['proposalSha256']});assert proposal.parent==O
P=json.loads(proposal.read_text());candidate=admitted(A['sourceCandidate'])
assert (proposal.parent/P['sourceCandidate']['path']).resolve()==candidate and P['sourceCandidate']['sha256']==A['sourceCandidate']['sha256']
assert candidate==D/'recovery-head-sampling-01/source-candidate-05.json'
source=json.loads(candidate.read_text())
assert isinstance(source['files'],dict) and source['files']
for path,digest in source['files'].items():bound({'path':path,'sha256':digest})
for group,field,digest,verdicts in [('sourceReviews','candidateSha256',A['sourceCandidate']['sha256'],('PASS_SCOPED','APPROVED')),('resourceVotes','proposalSha256',A['proposalSha256'],('APPROVE','APPROVED','APPROVE_BOUNDED_RUN'))]:
 refs=A[group];assert len(refs)==2 and {x['reviewer'] for x in refs}=={'gpt-6-astra','grok-4.6'}
 for ref in refs:
  receipt=json.loads(admitted(ref).read_text())
  assert receipt['reviewer']==ref['reviewer'] and receipt['verdict'] in verdicts and receipt[field]==digest
  admitted(receipt['actualReview']) # The normalized receipt must retain the real reviewed response.
for ref in [{'path':str((D/'local-execution-04/attempt-result.json').relative_to(R)),'sha256':'bf1d456714ea134ad7e320849b3841088b2a2a352435d75ced65854c48031f54'},{'path':str((D/'local-execution-04/run-public/public-transactions/0af6f3ed8960b1a7d1d5e8c204d9e133255e784dd2c5fad1f160c5212d02bd3c.bin').relative_to(R)),'sha256':'0af6f3ed8960b1a7d1d5e8c204d9e133255e784dd2c5fad1f160c5212d02bd3c'}]:admitted(ref)
for path in ['execute-once.py','commands-01.md']:
 rel=str((O/path).relative_to(R));assert rel in files
probe=D/'local-execution-04/committee-probe.mjs'
assert files.get(str(probe.relative_to(R)))=='1baf836d99dab191a8af61b91ee54c43f1c6037ae812a28e2e051338adca5c29'
for relative,digest in [('full-build-01/gpt6-build-result-review.json','f8baed4d44ac753ae856fabf7fae98b059a28203707437d34e1f6f96de6601dc'),('grok-build-review-02/review.json','0109d4b2f5995393ec63d0ece04bb0fefa60417011bd3d74c19a6b64ea4f3a69')]:
 receipt=json.loads(admitted({'path':str((D/relative).relative_to(R)),'sha256':digest}).read_text());assert receipt['verdict']=='APPROVED'
assert P['attempts']==1 and P['submissions']==3 and P['dustFeeSpeck']=='2000000000000000' and P['grossByLogicalAsset']=={'USD_TEST_ASSET':'20000000000'}
assert P['timeSeconds']=={'serviceReadinessMax':120,'walletOperationMax':1200,'cleanupReserved':30,'totalMax':1350,'independentShutdownStart':1300}
assert P['privatePreparation']['base']=='/home/charl/.local/state/moriarty/sp05-local-loan-recovery-20260910-03'
unit=P['nodeService']['unit'];timer=P['outerShutdown']['unit'];names=['moriarty-midnight-proof-server','moriarty-midnight-indexer','moriarty-midnight-node'];base=pathlib.Path(P['privatePreparation']['base']);armed=False;activated=False
assert unit=='moriarty-sp05-local-loan-recovery-03.service' and timer=='moriarty-sp05-local-recovery-stop-03'
os.umask(0o077)
def run(argv,timeout=10,input=None):return subprocess.run(argv,input=input,capture_output=True,text=True,timeout=timeout,check=True).stdout

def syncdir(p):
 fd=os.open(p,os.O_RDONLY)
 try:os.fsync(fd)
 finally:os.close(fd)
def save(p,v):
 raw=(json.dumps(v,indent=2)+'\n').encode() if not isinstance(v,bytes) else v
 fd=os.open(p,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600)
 try:
  remaining=memoryview(raw)
  while remaining:
   count=os.write(fd,remaining);assert count>0;remaining=remaining[count:]
  os.fsync(fd)
 finally:os.close(fd)
 syncdir(p.parent)
def jsonpost(url,payload,timeout=5):
 timeout=min(timeout,setupDeadline-time.monotonic());assert timeout>0
 req=urllib.request.Request(url,data=json.dumps(payload).encode(),headers={'Content-Type':'application/json'})
 with urllib.request.urlopen(req,timeout=timeout) as response:
  b=response.read(65537);assert len(b)<=65536;return json.loads(b)
def rpc(method,params=[]):
 v=jsonpost(P['network']['node'],{'jsonrpc':'2.0','id':1,'method':method,'params':params});assert 'error' not in v;return v['result']
def setupcheck():assert time.monotonic()<setupDeadline,'SETUP_DEADLINE'
for name in [unit,timer+'.timer',timer+'.service']:assert run(['systemctl','--user','show',name,'--property=LoadState','--value']).strip()=='not-found'
assert not base.exists() and not (O/'attempt.json').exists()
mem=dict(x.split(':',1) for x in pathlib.Path('/proc/meminfo').read_text().splitlines());assert int(mem['MemAvailable'].split()[0])*1024>=20*1024**3
import shutil
assert shutil.disk_usage(base.parent).free>=P['storage']['minimumFreeBytes']
containers=json.loads(run(['docker','inspect',*names]))
for x in containers:
 pin=next(c for c in P['containers'] if '/'+c['name']==x['Name']);h=x['HostConfig'];assert x['State']['Status']=='exited' and x['Image']=='sha256:'+pin['imageSha256'];assert h['Memory']==pin['memory'] and h['MemorySwap']==pin['memorySwapTotal'] and h['NanoCpus']==pin['nanoCpus'] and h['RestartPolicy']['Name']=='no'
old=D/'local-execution-02'
# Exact inspected predecessor argv, including whole launcher cgroup termination.
originalTimer=json.loads(admitted({'path':str((old/'timer-armed.json').relative_to(R)),'sha256':'557ae48a8e76cb794082a0edc9070a7977918f1083b2ed62cccc7070138ccccf'}).read_text())['argv']
timerArgv=[s.replace('moriarty-sp05-local-stop-01',timer).replace('moriarty-sp05-local-loan-01.service',unit) for s in originalTimer]
assert timerArgv[2]=='--unit='+timer and timerArgv[3]=='--on-active=1300s'
assert timerArgv[-1]=='/usr/bin/timeout --signal=KILL 5s /usr/bin/systemctl --user kill --signal=KILL --kill-whom=all '+unit+'; /usr/bin/timeout --signal=KILL 20s /usr/bin/docker stop -t 5 '+' '.join(names)
# A stopped/failed service gate consumes this attempt. Original journals stay untouched.
save(O/'attempt.json',{'schema':'moriarty.local-loan-recovery-attempt/1','allocationId':'sp05-local-loan-recovery-03','createdAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),'admissionSha256':sha((O/'execution-admission.json').read_bytes()),'timerArgv':timerArgv,'retryAllowed':False,'priorAttempt':'../local-recovery-02/attempt-result.json','earlierRecovery':'../local-recovery-01/attempt-result.json','originalDeploymentAttempt':'../local-execution-04/attempt-result.json','consumedReadOnlyDiagnostic':'../local-recovery-diagnostic-01/attempt.json','priorReservedSubmissions':3,'priorReservedDustSpeck':'900000000000003','otherHistoricalCharges':'recovery01, recovery02, diagnostic01 and all other service/review/compiler charges retained separately, not zeroed'})
try:
 armed=True;run(timerArgv)
 obj=shlex.split(run(['busctl','--user','call','org.freedesktop.systemd1','/org/freedesktop/systemd1','org.freedesktop.systemd1.Manager','GetUnit','s',timer+'.timer']))[1]
 def prop(key):return int(shlex.split(run(['busctl','--user','get-property','org.freedesktop.systemd1',obj,'org.freedesktop.systemd1.Timer',key]))[1])
 nextUS=prop('NextElapseUSecMonotonic');accuracy=prop('AccuracyUSec');assert 0<accuracy<=1000000
 arm=nextUS/1e6-1300;setupDeadline=arm+120;shutdownWallMs=int((time.time()+nextUS/1e6-time.monotonic())*1000)
 assert run(['systemctl','--user','is-active',timer+'.timer']).strip()=='active';setupcheck()
 save(O/'timer-armed.json',{'armingMonotonicSeconds':arm,'nextElapseMonotonicMicroseconds':nextUS,'accuracyMicroseconds':accuracy,'shutdownWallMsProjection':shutdownWallMs,'setupDeadlineMonotonicSeconds':setupDeadline,'argv':timerArgv})
 setupcheck();run(['docker','start','moriarty-midnight-node'],timeout=max(0.001,setupDeadline-time.monotonic()))
 while True:
  setupcheck()
  try:genesis=rpc('chain_getBlockHash',[0]);break
  except Exception:time.sleep(min(1,max(0,setupDeadline-time.monotonic())))
 assert genesis=='0xe72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846'
 setupcheck();run(['docker','start','moriarty-midnight-indexer','moriarty-midnight-proof-server'],timeout=max(0.001,setupDeadline-time.monotonic()));setupcheck()
 while True:
  setupcheck()
  try:
   data=jsonpost(P['network']['indexer'],{'query':'{ block(offset: null) { height hash timestamp } }'});assert data.get('data',{}).get('block');break
  except Exception:time.sleep(min(1,max(0,setupDeadline-time.monotonic())))
 import socket
 while True:
  setupcheck()
  try:
   with socket.create_connection(('127.0.0.1',16300),timeout=min(5,max(.001,setupDeadline-time.monotonic()))):pass
   break
  except OSError:time.sleep(min(1,max(0,setupDeadline-time.monotonic())))
 samples=[]
 while True:
  setupcheck()
  p=subprocess.run(['/usr/local/bin/node',str(probe),P['network']['node'],P['network']['indexer']],capture_output=True,text=True,timeout=min(16,setupDeadline-time.monotonic()))
  try:sample=json.loads(p.stdout.strip().splitlines()[-1])
  except Exception:sample={'status':'NOT_READY','error':'PROBE_OUTPUT_INVALID'}
  samples.append(sample);save(O/('committee-sample-'+str(len(samples)).zfill(3)+'.json'),sample)
  if p.returncode==0 and sample.get('status')=='READY' and sample['indexer']['hash']==sample['hash'][2:] and sample['indexer']['height']==sample['height']:break
  if time.monotonic()<setupDeadline:time.sleep(min(1,setupDeadline-time.monotonic()))
 save(O/'committee-readiness.json',{'samples':samples,'ready':sample,'elapsedSinceArmingSeconds':time.monotonic()-arm})
 # Construct the complete plan in memory. No original private plan/roles/seed is read.
 setupcheck();deadlineMs=min(int(time.time()*1000)+1200000,shutdownWallMs)
 transactionHash='0af6f3ed8960b1a7d1d5e8c204d9e133255e784dd2c5fad1f160c5212d02bd3c'
 retained=D/'local-execution-04';privateState=base/'contract-state'
 plan={'schema':'moriarty.local-financial-launch/1','kind':'loan',
  'build':{'receiptPath':'/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/loan-output/build/build-receipt.json','receiptSha256':'51ee2d4d60216464a9ace67966ba0ab253699844187aeb5652b46dc6e9ca5bf7','sourceManifestHash':'a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'},
  'networkConfig':{k:P['network'][k] for k in ['networkId','node','indexer','indexerWS','proofServer']},
  'wallet':{'seedFile':P['privatePreparation']['seedFile'],'stateDirectory':P['privatePreparation']['walletStateDirectory'],'expectedAddress':P['roles']['expectedAddress']},
  'roles':{'firstAddress':P['roles']['firstAddress'],'secondAddress':P['roles']['secondAddress'],'secretsFile':P['privatePreparation']['originalRolesFile']},
  'privateState':{'directory':str(privateState),'passwordFile':str(base/'password')},'networkTag':genesis[2:],'expectedProtocolVersion':1000000,
  'limits':{'allocationId':'sp05-local-loan-recovery-03','deadlineMs':deadlineMs,'submissions':3,'dustFee':P['dustFeeSpeck'],'grossByLogicalAsset':P['grossByLogicalAsset']},'outputDirectory':str(base/'run'),
  'existingDeployment':{'schema':'moriarty.existing-local-loan/1','transactionFile':str(retained/'run-public/public-transactions'/(transactionHash+'.bin')),'transactionHash':transactionHash,
   'identifiers':['00b6140ece5793d57c801512e8e7c9e2ec2687e19b1c48a1f56167f2cd1dfc9e66','00959c51e7d62ee9160bf1396ce0ab52f26757a7c5adec669cb083d5a8787d1de9'],
   'txId':'00959c51e7d62ee9160bf1396ce0ab52f26757a7c5adec669cb083d5a8787d1de9','contractAddress':'ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713',
   'buildReceiptSha256':'51ee2d4d60216464a9ace67966ba0ab253699844187aeb5652b46dc6e9ca5bf7','networkTag':genesis[2:],'expectedProtocolVersion':1000000,
   'sourceAllocationId':'sp05-local-loan-03','sourceResultFile':str(retained/'attempt-result.json'),'sourceResultSha256':'bf1d456714ea134ad7e320849b3841088b2a2a352435d75ced65854c48031f54',
   'sourcePrivateStateDirectory':P['privatePreparation']['originalStore'],'inspectionDirectory':str(base/'failed-store-inspection'),'destinationDirectory':str(privateState)}}
 # JSON is inserted as a JSON string literal into stdin code, never shell text.
 publicProjection="const closedObservation=x=>{if(!x||Object.getPrototypeOf(x)!==Object.prototype)return;const ds=Object.getOwnPropertyDescriptors(x);if(Reflect.ownKeys(ds).some(k=>typeof k!=='string'||!Object.hasOwn(ds[k],'value')))return;const keys=Object.keys(ds).sort().join(',');const base='elapsedMs,reason,samples',full='elapsedMs,fromHash,fromHeight,phase,reason,samples,toHash,toHeight';if(keys!==base&&keys!==full)return;const safe=n=>Number.isSafeInteger(n)&&n>=0;if(!safe(x.samples)||x.samples>6||!safe(x.elapsedMs)||!['deadline','sample-limit'].includes(x.reason))return;const out={samples:x.samples,elapsedMs:x.elapsedMs,reason:x.reason};if(keys===full){if(!['before-state','after-state','final-node'].includes(x.phase)||typeof x.fromHash!=='string'||typeof x.toHash!=='string'||!/^0x[a-f0-9]{64}$/.test(x.fromHash)||!/^0x[a-f0-9]{64}$/.test(x.toHash)||!safe(x.fromHeight)||!safe(x.toHeight))return;Object.assign(out,{phase:x.phase,fromHash:x.fromHash,toHash:x.toHash,fromHeight:x.fromHeight,toHeight:x.toHeight});}return out;};"
 publicJs=publicProjection+"const clip=x=>String(x).replace(/[^\\x20-\\x7e]/g,' ').slice(0,2048);try{const {validateLocalLaunchPlan}=await import("+json.dumps((R/'experiments/moriarty-midnight-financial/ledger/launch-local.mjs').as_uri())+");const {preflightLocalRecovery}=await import("+json.dumps((R/'experiments/moriarty-midnight-financial/ledger/integrate-local.mjs').as_uri())+");const p=validateLocalLaunchPlan(JSON.parse("+json.dumps(json.dumps(plan))+"));p.limits.deadlineMs=Math.min(p.limits.deadlineMs,Date.now()+"+str(max(1,int((setupDeadline-time.monotonic())*1000)))+");const r=await preflightLocalRecovery(p);if(r.snapshotSamples!==undefined&&(!Number.isSafeInteger(r.snapshotSamples)||r.snapshotSamples<1||r.snapshotSamples>6))throw Error('RECOVERY_PUBLIC_SAMPLE_COUNT');console.log(JSON.stringify({status:r.status,transactionHash:r.binding.transactionHash,contractAddress:r.binding.contractAddress,txId:r.binding.txId,finalizedHash:r.stateBlock.hash,finalizedHeight:r.stateBlock.height,...(r.snapshotSamples===undefined?{}:{snapshotSamples:r.snapshotSamples})}));process.exit(0);}catch(e){const observation=e?.message==='RECOVERY_SNAPSHOT_UNSTABLE'?closedObservation(e.recoveryObservation):undefined;console.log(JSON.stringify({status:'PUBLIC_PREFLIGHT_FAILED',errorClass:clip(e?.name??'Error').slice(0,80),localCode:typeof e?.message==='string'&&/^[A-Z][A-Z0-9_]{0,127}$/.test(e.message)?e.message:'PUBLIC_ERROR_MESSAGE',publicMessage:clip(e?.message??e),...(observation===undefined?{}:{recoveryObservation:observation})}));process.exit(1);}"
 setupcheck()
 try:
  child=subprocess.run(['/usr/local/bin/node','--input-type=module'],input=publicJs,capture_output=True,text=True,timeout=setupDeadline-time.monotonic())
  lines=child.stdout.splitlines()
  if len(child.stdout.encode())>65536 or not lines or len(lines[-1].encode())>8192:
   publicResult={'status':'PUBLIC_PREFLIGHT_FAILED','errorClass':'PublicOutputError','localCode':'PUBLIC_OUTPUT_BOUND_OR_EMPTY','publicMessage':'Public preflight did not retain a bounded result.'}
  else:
   try:publicResult=json.loads(lines[-1])
   except Exception:publicResult={'status':'PUBLIC_PREFLIGHT_FAILED','errorClass':'PublicOutputError','localCode':'PUBLIC_OUTPUT_INVALID_JSON','publicMessage':'Public preflight result could not be decoded.'}
 except subprocess.TimeoutExpired:
  publicResult={'status':'PUBLIC_PREFLIGHT_FAILED','errorClass':'TimeoutExpired','localCode':'PUBLIC_PREFLIGHT_SETUP_DEADLINE','publicMessage':'Public preflight exceeded the remaining setup/activation allowance.'};child=None
 save(O/'public-recovery-preflight.json',publicResult)
 assert child is not None and child.returncode==0 and publicResult['status']=='PUBLIC_STATE_VERIFIED' and publicResult['transactionHash']==transactionHash and publicResult['contractAddress']==plan['existingDeployment']['contractAddress']
 setupcheck()
 # Only now create private material. Preserve original roles, password, snapshots and DB.
 base.mkdir(mode=0o700);syncdir(base.parent);privateState.mkdir(mode=0o700);syncdir(base);syncdir(privateState)
 assert not pathlib.Path(plan['existingDeployment']['inspectionDirectory']).exists()
 save(base/'password',('Aa9!'+secrets.token_urlsafe(48)+'\n').encode())
 passwordJs="import {readLocalStoragePassword} from "+json.dumps((R/'experiments/moriarty-midnight-financial/ledger/launch-local.mjs').as_uri())+";try{await readLocalStoragePassword("+json.dumps(str(base/'password'))+");console.log(JSON.stringify({status:'PASSWORD_VALIDATED',policy:'pinned-installed-sdk',passwordEmitted:false}));}catch{console.error('RECOVERY_PASSWORD_POLICY_FAILED');process.exit(1);}"
 setupcheck();passwordResult=json.loads(run(['/usr/local/bin/node','--input-type=module'],input=passwordJs,timeout=setupDeadline-time.monotonic()));assert passwordResult['status']=='PASSWORD_VALIDATED'
 save(O/'password-validation-observation.json',passwordResult)
 save(base/'plan.json',plan);planhash=sha((base/'plan.json').read_bytes());syncdir(base);syncdir(base.parent)
 setupcheck();activationAllowance=int(setupDeadline-time.monotonic());assert activationAllowance>=1
 argv=['/usr/bin/systemd-run','--user','--unit='+unit.removesuffix('.service'),'--property=Type=exec','--property=TimeoutStartSec='+str(activationAllowance)+'s','--property=MemoryMax=4294967296','--property=MemorySwapMax=0','--property=CPUQuota=200%','--property=RuntimeMaxSec=1206','--property=TimeoutStopSec=5','--property=KillMode=control-group','--property=FinalKillSignal=SIGKILL','--property=SendSIGKILL=yes','--property=UMask=0077','--property=StandardOutput=append:'+str(base/'sdk.stdout'),'--property=StandardError=append:'+str(base/'sdk.stderr'),'--property=ExecStopPost=/usr/bin/timeout --signal=KILL 20s /usr/bin/docker stop -t 5 '+' '.join(names),'--working-directory='+str(R/'experiments/moriarty-midnight-financial'),'/usr/local/bin/node','ledger/launch-local.mjs','--run','--plan',str(base/'plan.json'),'--sha256',planhash]
 save(O/'resolved-launch.json',{'argv':argv,'planSha256':planhash,'deadlineMs':plan['limits']['deadlineMs']})
 setupcheck();assert run(['systemctl','--user','is-active',timer+'.timer'],timeout=setupDeadline-time.monotonic()).strip()=='active';setupcheck();remaining=setupDeadline-time.monotonic();assert remaining>0;run(argv,timeout=remaining);setupcheck()
 state=run(['systemctl','--user','show',unit,'--property=ActiveState,SubState,MainPID,ControlGroup,MemoryMax,MemorySwapMax,CPUQuotaPerSecUSec,ActiveEnterTimestampMonotonic'],timeout=setupDeadline-time.monotonic());setupcheck();fields=dict(line.split('=',1) for line in state.splitlines());assert fields['ActiveState']=='active' and int(fields['MainPID'])>0
 entered=int(fields['ActiveEnterTimestampMonotonic'])/1e6;assert arm<=entered<=setupDeadline
 save(O/'launcher-active.json',{'observed':fields,'elapsedSinceArmingSeconds':time.monotonic()-arm});activated=True;print('Recovery local loan launcher active; public run records:',str(base/'run'),flush=True)
except BaseException as error:
 if armed:
  for cmd in [['/usr/bin/timeout','--signal=KILL','5s','systemctl','--user','kill','--signal=KILL','--kill-whom=all',unit],['/usr/bin/timeout','--signal=KILL','20s','docker','stop','-t','5',*names],['/usr/bin/timeout','--signal=KILL','5s','docker','kill',*names]]:
   try:subprocess.run(cmd,capture_output=True,timeout=22)
   except Exception:pass
 save(O/'setup-failure.json',{'status':'FAILED','errorClass':type(error).__name__,'timerArmed':armed,'launcherAcknowledged':activated,'retryAllowed':False,'scope':'Retain private diagnostics and verify terminal containment independently'})
 raise SystemExit('Setup failed; admitted services stopped or kill attempted. Inspect retained records; no retry.')
