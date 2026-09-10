# PROPOSED ONLY. Never import/execute for checks: use AST extraction.
import pathlib,json,hashlib,os,subprocess,time,shlex,shutil,urllib.request,socket,datetime
R=pathlib.Path('/home/charl/Moriarty/.worktrees/sp05-adverse-integration');D=R/'deliverables/sp05-financial-integration-2026-09-09';O=D/'local-stale-loan-01'
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
P=json.loads(bound({'path':str((O/'resource-proposal-01.json').relative_to(R)),'sha256':A['proposalSha256']}).read_text())
assert P['sourceCandidate']['path']!='UNBOUND' and P['sourceCandidate']['sha256']!='UNBOUND'
assert A['sourceCandidate']==P['sourceCandidate'];candidate=bound(A['sourceCandidate'])
source=json.loads(candidate.read_text());assert isinstance(source['files'],dict) and source['files']
for p,digest in source['files'].items():bound({'path':p,'sha256':digest})
sdkManifestRelative=str((D/'stale-loan-rejection-01/sdk-source-manifest.json').relative_to(R))
assert sdkManifestRelative in source['files']
sdkPins=json.loads(bound({'path':sdkManifestRelative,'sha256':source['files'][sdkManifestRelative]}).read_text());assert len(sdkPins)==9
sdkRoot=pathlib.Path('/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules')
for name,digest in sdkPins.items():
 sdkFile=pathlib.Path(name);assert sdkFile.is_absolute() and sdkFile.is_relative_to(sdkRoot) and '..' not in sdkFile.parts
 for q in [sdkFile,*sdkFile.parents]:assert not q.is_symlink()
 assert sdkFile.is_file() and sha(sdkFile)==digest,'SDK_SOURCE_DRIFT'
for name in ['execute-once.py','public-preflight.mjs','commands-01.md','fixed-public-inputs.json','resource-proposal-01.json']:assert str((O/name).relative_to(R)) in files
for name in ['launch-local.mjs','integrate-local.mjs','stale-loan-plan.mjs','stale-loan-rejection.mjs','providers.mjs','receipt.mjs','finalized-financial-state.mjs']:
 assert 'experiments/moriarty-midnight-financial/ledger/'+name in source['files']
for group,field,digest,verdicts in [('sourceReviews','candidateSha256',A['sourceCandidate']['sha256'],['PASS_SCOPED','APPROVED']),('resourceVotes','proposalSha256',A['proposalSha256'],['APPROVE','APPROVED','APPROVE_BOUNDED_RUN'])]:
 refs=A[group];assert len(refs)==2 and {r['reviewer'] for r in refs}=={'gpt-6-astra','grok-4.6'}
 for ref in refs:
  receipt=json.loads(bound(ref).read_text());assert receipt['reviewer']==ref['reviewer'] and receipt['verdict'] in verdicts and receipt[field]==digest;bound(receipt['actualReview'])
fixed=json.loads((O/'fixed-public-inputs.json').read_text())
for key,value in list(fixed.items()):
 if key.endswith('File'):
  digest=fixed[key[:-4]+('Hash' if key.endswith('TransactionFile') else 'Sha256')]
  fixed[key]=str(bound({'path':value,'sha256':digest}))
for relative,digest in [('full-build-01/gpt6-build-result-review.json','f8baed4d44ac753ae856fabf7fae98b059a28203707437d34e1f6f96de6601dc'),('grok-build-review-02/review.json','0109d4b2f5995393ec63d0ece04bb0fefa60417011bd3d74c19a6b64ea4f3a69')]:
 rel=str((D/relative).relative_to(R));assert json.loads(bound({'path':rel,'sha256':digest}).read_text())['verdict']=='APPROVED'
probe=D/'local-execution-04/committee-probe.mjs';assert files.get(str(probe.relative_to(R)))=='1baf836d99dab191a8af61b91ee54c43f1c6037ae812a28e2e051338adca5c29'
assert P['allocationId']==A['allocationId']=='sp05-stale-loan-01' and P['attempts']==1 and P['submissions']==1
assert P['dustFeeSpeck']=='1000000000000000' and P['grossByLogicalAsset']=={'USD_TEST_ASSET':'0'}
assert P['proofAccounting']['contractProveEntriesMax']==P['proofAccounting']['walletFinalizeProveEntriesMax']==1
assert P['timeSeconds']=={'serviceReadinessMax':120,'walletOperationMax':1200,'cleanupReserved':31,'totalMax':1350,'independentShutdownStart':1300}
assert P['historicalCharges']['reservedSubmissions']==10 and P['historicalCharges']['reservedDustSpeck']=='3000000000000010'
unit=P['nodeService']['unit'];diagnostic=P['diagnosticService']['unit'];timer=P['outerShutdown']['unit'];base=pathlib.Path(P['privatePreparation']['base'])
assert unit=='moriarty-sp05-stale-loan-01.service' and diagnostic=='moriarty-sp05-stale-loan-preflight-01.service' and timer=='moriarty-sp05-stale-loan-stop-01'
assert str(base)=='/home/charl/.local/state/moriarty/sp05-stale-loan-20260910-01'
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
for name in [unit,diagnostic,timer+'.timer',timer+'.service']:assert run(['systemctl','--user','show',name,'--property=LoadState','--value']).strip()=='not-found'
for path in [base,*base.parents]:assert not path.is_symlink()
assert not base.exists() and not (O/'attempt.json').exists()
mem=dict(x.split(':',1) for x in pathlib.Path('/proc/meminfo').read_text().splitlines());assert int(mem['MemAvailable'].split()[0])*1024>=P['storage']['minimumHostAvailableMemoryBytes'];assert shutil.disk_usage(base.parent).free>=P['storage']['minimumFreeBytes']
for c in json.loads(run(['docker','inspect',*names])):
 pin=next(p for p in P['containers'] if '/'+p['name']==c['Name']);h=c['HostConfig'];assert c['State']['Status']=='exited' and c['Image']=='sha256:'+pin['imageSha256'];assert h['Memory']==pin['memory'] and h['MemorySwap']==pin['memorySwapTotal'] and h['NanoCpus']==pin['nanoCpus'] and h['RestartPolicy']['Name']=='no'
timerArgv=['/usr/bin/systemd-run','--user','--unit='+timer,'--on-active=1300s','--timer-property=AccuracySec=1s','--property=Type=oneshot','--property=TimeoutStartSec=25s','--property=TimeoutStopSec=5s','--property=ExecStopPost=/usr/bin/timeout --signal=KILL 5s /usr/bin/docker kill '+' '.join(names),'/usr/bin/bash','-c','/usr/bin/timeout --signal=KILL 5s /usr/bin/systemctl --user kill --signal=KILL --kill-whom=all '+unit+' '+diagnostic+'; /usr/bin/timeout --signal=KILL 20s /usr/bin/docker stop -t 5 '+' '.join(names)]
save(O/'attempt.json',{'schema':'moriarty.local-stale-loan-attempt/1','status':'CONSUMED','allocationId':P['allocationId'],'admissionSha256':sha(O/'execution-admission.json'),'timerArgv':timerArgv,'retryAllowed':False,'priorCharges':P['historicalCharges']})
try:
 timerLaunchAttempted=True;run(timerArgv)
 obj=shlex.split(run(['busctl','--user','call','org.freedesktop.systemd1','/org/freedesktop/systemd1','org.freedesktop.systemd1.Manager','GetUnit','s',timer+'.timer']))[1]
 def prop(key):return int(shlex.split(run(['busctl','--user','get-property','org.freedesktop.systemd1',obj,'org.freedesktop.systemd1.Timer',key]))[1])
 nextUS=prop('NextElapseUSecMonotonic');accuracy=prop('AccuracyUSec');assert 0<accuracy<=1000000
 arm=nextUS/1e6-1300;setupDeadline=arm+120;shutdownWallMs=int((time.time()+nextUS/1e6-time.monotonic())*1000)
 assert run(['systemctl','--user','is-active',timer+'.timer']).strip()=='active';timerActiveObserved=True;setupcheck()
 save(O/'timer-armed.json',{'argv':timerArgv,'armingMonotonicSeconds':arm,'nextElapseMonotonicMicroseconds':nextUS,'accuracyMicroseconds':accuracy,'setupDeadlineMonotonicSeconds':setupDeadline,'shutdownWallMsProjection':shutdownWallMs})
 setupcheck();run(['docker','start','moriarty-midnight-node'],timeout=setupDeadline-time.monotonic());setupcheck()
 while True:
  setupcheck()
  try:
   genesis=jsonpost(P['network']['node'],{'jsonrpc':'2.0','id':1,'method':'chain_getBlockHash','params':[0]})['result'];break
  except Exception:time.sleep(min(1,max(0,setupDeadline-time.monotonic())))
 assert genesis=='0xe72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846'
 setupcheck();run(['docker','start','moriarty-midnight-indexer','moriarty-midnight-proof-server'],timeout=setupDeadline-time.monotonic());setupcheck()
 while True:
  setupcheck()
  try:
   indexed=jsonpost(P['network']['indexer'],{'query':'{ block(offset: null) { height hash timestamp } }'});assert indexed.get('data',{}).get('block')
   with socket.create_connection(('127.0.0.1',16300),timeout=min(5,setupDeadline-time.monotonic())):pass
   break
  except Exception:time.sleep(min(1,max(0,setupDeadline-time.monotonic())))
 setupcheck();deadlineMs=min(int(time.time()*1000)+1200000,shutdownWallMs-5000)
 fixed.update(snapshotDirectory=str(base/'snapshot'),inspectionDirectory=str(base/'inspection'))
 plan={'schema':'moriarty.local-financial-launch/1','kind':'loan','build':{'receiptPath':'/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/loan-output/build/build-receipt.json','receiptSha256':'51ee2d4d60216464a9ace67966ba0ab253699844187aeb5652b46dc6e9ca5bf7','sourceManifestHash':'a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'},'networkConfig':{k:P['network'][k] for k in ['networkId','node','indexer','indexerWS','proofServer']},'wallet':{'seedFile':P['privatePreparation']['seedFile'],'stateDirectory':P['privatePreparation']['walletStateDirectory'],'expectedAddress':P['roles']['expectedAddress']},'roles':{'firstAddress':P['roles']['firstAddress'],'secondAddress':P['roles']['secondAddress'],'secretsFile':P['privatePreparation']['originalRolesFile']},'privateState':{'directory':P['privatePreparation']['originalStore'],'passwordFile':P['privatePreparation']['existingPasswordFile']},'networkTag':genesis[2:],'expectedProtocolVersion':1000000,'limits':{'allocationId':P['allocationId'],'deadlineMs':deadlineMs,'submissions':1,'dustFee':P['dustFeeSpeck'],'grossByLogicalAsset':P['grossByLogicalAsset']},'outputDirectory':str(base/'run'),'existingStaleLoan':fixed}
 common=['--property=MemoryMax=4294967296','--property=MemorySwapMax=0','--property=CPUQuota=200%','--property=KillMode=control-group','--property=FinalKillSignal=SIGKILL','--property=SendSIGKILL=yes','--property=UMask=0077']
 diagnosticArgv=['/usr/bin/systemd-run','--user','--unit='+diagnostic.removesuffix('.service'),'--property=Type=exec','--quiet','--pipe','--wait','--property=TimeoutStopSec=1',*common,'--working-directory='+str(R),'/usr/local/bin/node',str(O/'public-preflight.mjs')]
 setupcheck();input=json.dumps({'plan':plan,'setupDeadlineMs':int((time.time()+setupDeadline-time.monotonic())*1000)})
 try:output=activate(diagnosticArgv,'resolved-preflight.json',input=input,isDiagnostic=True)
 except subprocess.CalledProcessError as error:output=error.stdout or ''
 assert len(output.encode())<=65536;lines=output.splitlines()
 try:public=json.loads(lines[-1])
 except Exception:public={'status':'PUBLIC_PREFLIGHT_FAILED','code':'PUBLIC_OUTPUT_INVALID'}
 save(O/'public-preflight-result.json',public)
 assert public['status']=='SETTLED_LOAN_PUBLIC_VERIFIED' and public['current']['stateSha256']==fixed['settledStateSha256'] and public['current']['blockHeight']>=fixed['minimumCurrentBlockHeight']
 assert len(public['history'])==4
 for row,stage in zip(public['history'],['deploy','initialize','accrue','settle']):assert row=={'stage':stage,'txId':fixed[stage+'TxId'],'rawSha256':fixed[stage+'TransactionHash']}
 setupcheck();base.mkdir(mode=0o700);syncdir(base.parent);syncdir(base)
 for name in ['run','snapshot','inspection']:assert not (base/name).exists() and not (base/name).is_symlink()
 save(base/'plan.json',plan);planhash=sha(base/'plan.json')
 argv=['/usr/bin/systemd-run','--user','--unit='+unit.removesuffix('.service'),'--property=Type=exec',*common,'--property=RuntimeMaxSec=1206','--property=TimeoutStopSec=5','--property=StandardOutput=append:'+str(base/'sdk.stdout'),'--property=StandardError=append:'+str(base/'sdk.stderr'),'--property=ExecStopPost=/usr/bin/timeout --signal=KILL 20s /usr/bin/docker stop -t 5 '+' '.join(names),'--working-directory='+str(R/'experiments/moriarty-midnight-financial'),'/usr/local/bin/node','ledger/launch-local.mjs','--run','--plan',str(base/'plan.json'),'--sha256',planhash]
 setupcheck();assert run(['systemctl','--user','is-active',timer+'.timer'],timeout=setupDeadline-time.monotonic()).strip()=='active'
 activate(argv,'resolved-launch.json')
 fields=dict(x.split('=',1) for x in run(['systemctl','--user','show',unit,'--property=ActiveState,SubState,MainPID,ControlGroup,ActiveEnterTimestampMonotonic'],timeout=setupDeadline-time.monotonic()).splitlines());setupcheck();assert fields['ActiveState']=='active' and int(fields['MainPID'])>0 and arm<=int(fields['ActiveEnterTimestampMonotonic'])/1e6<=setupDeadline
 save(O/'launcher-active.json',{'observed':fields,'planSha256':planhash,'deadlineMs':deadlineMs,'elapsedSinceArmingSeconds':time.monotonic()-arm})
 print('Fixed local stale-loan launcher active. Inspect public candidate/outcome/result; stop on UNKNOWN. Independent containment must precede timer cancellation.')
except BaseException as error:
 if timerLaunchAttempted:
  for cmd in [['/usr/bin/timeout','--signal=KILL','5s','systemctl','--user','kill','--signal=KILL','--kill-whom=all',unit,diagnostic],['/usr/bin/timeout','--signal=KILL','20s','docker','stop','-t','5',*names],['/usr/bin/timeout','--signal=KILL','5s','docker','kill',*names]]:
   try:subprocess.run(cmd,capture_output=True,timeout=22)
   except Exception:pass
 save(O/'setup-failure.json',{'status':'FAILED','errorClass':type(error).__name__,'timerLaunchAttempted':timerLaunchAttempted,'timerActiveObserved':timerActiveObserved,'retryAllowed':False,'scope':'Verify both cgroups and all three containers actually stopped; no retry or refund.'})
 raise SystemExit('Stale-loan setup failed; retain evidence and inspect containment. No retry.')
