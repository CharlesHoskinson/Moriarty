# One-off execution of local-execution-04/commands-02.md, only after its admission.
import pathlib,json,hashlib,subprocess,time,datetime,os,secrets,urllib.request,shlex
R=pathlib.Path('/home/charl/Moriarty/.worktrees/sp05-deadline-review');E=pathlib.Path('/home/charl/Moriarty/.worktrees/sp05-financial-integration');O=R/'deliverables/sp05-financial-integration-2026-09-09/local-execution-04'
P=json.loads((O/'resource-proposal-02.json').read_text());A=json.loads((O/'execution-admission.json').read_text());assert A['status']=='ADMITTED'
sha=lambda b:hashlib.sha256(b).hexdigest()
assert sha((O/'resource-proposal-02.json').read_bytes())==A['proposalSha256']
for f in A['files']:assert sha((R/f['path']).read_bytes())==f['sha256'],f['path']
for f in json.loads((R/'deliverables/sp05-financial-integration-2026-09-09/launch-source-04/candidate-02.json').read_text())['files']:assert sha((E/f['path']).read_bytes())==f['sha256'],f['path']
unit=P['nodeService']['unit'];timer=P['outerShutdown']['unit'];names=['moriarty-midnight-proof-server','moriarty-midnight-indexer','moriarty-midnight-node'];base=pathlib.Path(P['privatePreparation']['base']);armed=False;activated=False
os.umask(0o077)
def run(argv,timeout=10):return subprocess.run(argv,capture_output=True,text=True,timeout=timeout,check=True).stdout

def syncdir(p):
 fd=os.open(p,os.O_RDONLY)
 try:os.fsync(fd)
 finally:os.close(fd)
def save(p,v):
 raw=(json.dumps(v,indent=2)+'\n').encode() if not isinstance(v,bytes) else v
 fd=os.open(p,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600)
 try:os.write(fd,raw);os.fsync(fd)
 finally:os.close(fd)
 syncdir(p.parent)
def jsonpost(url,payload,timeout=5):
 req=urllib.request.Request(url,data=json.dumps(payload).encode(),headers={'Content-Type':'application/json'})
 with urllib.request.urlopen(req,timeout=timeout) as response:
  b=response.read(65537);assert len(b)<=65536;return json.loads(b)
def rpc(method,params=[]):
 v=jsonpost(P['network']['node'],{'jsonrpc':'2.0','id':1,'method':method,'params':params});assert 'error' not in v;return v['result']
def setupcheck():assert time.monotonic()<setupDeadline,'SETUP_DEADLINE'
binding=json.loads((O/'old-native-binding.json').read_text());original=P['reconciliation']
assert binding['canonical'] is True and binding['nativeTransactionHash']==original['nativeTransactionHash'] and binding['identifiers']==original['identifiers']
assert binding['earliestIntentTtlMs']==datetime.datetime.fromisoformat(original['earliestNativeIntentTtl'].replace('Z','+00:00')).timestamp()*1000
raw=R/'deliverables/sp05-financial-integration-2026-09-09/local-execution-03/run-public/public-transactions'/(original['nativeTransactionHash']+'.bin')
assert sha(raw.read_bytes())==binding['rawSha256']==original['nativeTransactionHash']
assert time.time()>datetime.datetime.fromisoformat(P['reconciliation']['earliestNativeIntentTtl'].replace('Z','+00:00')).timestamp(),'PRE_TTL_HOST_STOP'
for name in [unit,timer+'.timer',timer+'.service']:assert run(['systemctl','--user','show',name,'--property=LoadState','--value']).strip()=='not-found'
assert not base.exists() and not (O/'attempt.json').exists()
mem=dict(x.split(':',1) for x in pathlib.Path('/proc/meminfo').read_text().splitlines());assert int(mem['MemAvailable'].split()[0])*1024>=20*1024**3
import shutil
assert shutil.disk_usage(base.parent).free>=P['storage']['minimumFreeBytes']
containers=json.loads(run(['docker','inspect',*names]))
for x in containers:
 pin=next(c for c in P['containers'] if '/'+c['name']==x['Name']);h=x['HostConfig'];assert x['State']['Status']=='exited' and x['Image']=='sha256:'+pin['imageSha256'];assert h['Memory']==pin['memory'] and h['MemorySwap']==pin['memorySwapTotal'] and h['NanoCpus']==pin['nanoCpus'] and h['RestartPolicy']['Name']=='no'
old=R/'deliverables/sp05-financial-integration-2026-09-09/local-execution-02'
timerArgv=[s.replace('local-stop-01','local-stop-03').replace('local-loan-01','local-loan-03') for s in json.loads((old/'timer-armed.json').read_text())['argv']]
save(O/'attempt.json',{'schema':'moriarty.local-loan-attempt/2','allocationId':'sp05-local-loan-03','createdAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),'admissionSha256':sha((O/'execution-admission.json').read_bytes()),'timerArgv':timerArgv,'retryAllowed':False,'priorAttempt':'../local-execution-03/attempt-result.json','earlierAttempt':'../local-execution-02/attempt-result.json','priorReservedSubmissions':2,'priorReservedDustSpeck':'600000000000002','otherHistoricalCharges':'retained separately, not zeroed'})
try:
 run(timerArgv);armed=True
 obj=shlex.split(run(['busctl','--user','call','org.freedesktop.systemd1','/org/freedesktop/systemd1','org.freedesktop.systemd1.Manager','GetUnit','s',timer+'.timer']))[1]
 def prop(key):return int(shlex.split(run(['busctl','--user','get-property','org.freedesktop.systemd1',obj,'org.freedesktop.systemd1.Timer',key]))[1])
 nextUS=prop('NextElapseUSecMonotonic');accuracy=prop('AccuracyUSec');assert 0<accuracy<=1000000
 arm=nextUS/1e6-1300;setupDeadline=arm+120;shutdownWallMs=int((time.time()+nextUS/1e6-time.monotonic())*1000)
 assert run(['systemctl','--user','is-active',timer+'.timer']).strip()=='active';setupcheck()
 save(O/'timer-armed.json',{'armingMonotonicSeconds':arm,'nextElapseMonotonicMicroseconds':nextUS,'accuracyMicroseconds':accuracy,'shutdownWallMsProjection':shutdownWallMs,'setupDeadlineMonotonicSeconds':setupDeadline,'argv':timerArgv})
 run(['docker','start','moriarty-midnight-node'])
 while True:
  setupcheck()
  try:genesis=rpc('chain_getBlockHash',[0]);break
  except Exception:time.sleep(min(1,max(0,setupDeadline-time.monotonic())))
 expected=json.loads((old/'live-chain-preflight.json').read_text());assert genesis==expected['genesis']
 run(['docker','start','moriarty-midnight-indexer','moriarty-midnight-proof-server']);setupcheck()
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
 verification=json.loads(run(['/usr/local/bin/node',str(E/'experiments/moriarty-midnight-network/verify-local.mjs')],timeout=max(1,setupDeadline-time.monotonic())))
 txhash=expected['protocolTransaction']['hash'];gql='{ transactions(offset: { hash: "'+txhash+'" }) { hash protocolVersion block { height hash } ... on RegularTransaction { transactionResult { status } } } }'
 txs=jsonpost(P['network']['indexer'],{'query':gql})['data']['transactions'];assert len(txs)==1;tx=txs[0];assert tx==expected['protocolTransaction'];assert type(tx['protocolVersion']) is int
 save(O/'live-chain-preflight.json',{'genesis':genesis,'protocolTransaction':tx,'verification':verification,'indexedLatestBeforeLaunch':data['data']['block'],'elapsedSinceArmingSeconds':time.monotonic()-arm})

 samples=[]
 while True:
  setupcheck()
  p=subprocess.run(['/usr/local/bin/node','/tmp/moriarty-committee-probe.mjs',P['network']['node'],P['network']['indexer']],capture_output=True,text=True,timeout=min(16,max(1,setupDeadline-time.monotonic())))
  try:sample=json.loads(p.stdout.strip().splitlines()[-1])
  except Exception:sample={'status':'NOT_READY','error':'PROBE_OUTPUT_INVALID'}
  samples.append(sample);save(O/('committee-sample-'+str(len(samples)).zfill(3)+'.json'),sample)
  if p.returncode==0 and sample.get('status')=='READY' and sample['indexer']['hash']==sample['hash'][2:] and sample['indexer']['height']==sample['height']:break
  if time.monotonic()<setupDeadline:time.sleep(min(1,setupDeadline-time.monotonic()))
 save(O/'committee-readiness.json',{'samples':samples,'ready':sample,'elapsedSinceArmingSeconds':time.monotonic()-arm})
 original=P['reconciliation'];native=R/'deliverables/sp05-financial-integration-2026-09-09/local-execution-03/native-time-inspection.json';inspection=json.loads(native.read_text())
 assert inspection['canonical'] and inspection['nativeTransactionHash']==original['nativeTransactionHash']
 earliest=min(datetime.datetime.fromisoformat(x['ttl'].replace('Z','+00:00')).timestamp()*1000 for x in inspection['intents'])
 assert earliest==datetime.datetime.fromisoformat(original['earliestNativeIntentTtl'].replace('Z','+00:00')).timestamp()*1000 and sample['timestampMs']>earliest
 qry='{ latest:block(offset:null){height hash timestamp} oldHash:transactions(offset:{hash:"'+original['nativeTransactionHash']+'"}){hash} first:transactions(offset:{identifier:"'+original['identifiers'][0]+'"}){hash} second:transactions(offset:{identifier:"'+original['identifiers'][1]+'"}){hash} }'
 observation=jsonpost(P['network']['indexer'],{'query':qry});assert not observation.get('errors');d=observation['data']
 assert d['latest']['hash']==sample['hash'][2:] and d['latest']['height']==sample['height']
 assert d['oldHash']==[] and d['first']==[] and d['second']==[]
 pool=rpc('author_pendingExtrinsics');assert pool==[]
 save(O/'old-transaction-reconciliation.json',{'observedAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),'originalTransactionHash':original['nativeTransactionHash'],'originalIdentifiers':original['identifiers'],'earliestIntentTtlMs':earliest,'finalizedHash':sample['hash'],'finalizedHeight':sample['height'],'finalizedTimestampMs':sample['timestampMs'],'indexedObservations':d,'pendingPoolCount':0,'result':'NOT_OBSERVED_THROUGH_SYNCHRONIZED_POST_TTL_FINALIZED_TIP','journalChanged':False,'scope':'No old inclusion observed; exact original bytes past TTL. Not a refund, cancellation, financial settlement or proof of arbitrary indexer completeness.'})
 setupcheck();base.mkdir(mode=0o700);syncdir(base.parent);privateState=base/'contract-state';privateState.mkdir(mode=0o700);syncdir(base);syncdir(privateState)
 save(base/'roles.json',{'firstSecret':secrets.token_hex(32),'secondSecret':secrets.token_hex(32)});save(base/'password',(secrets.token_hex(32)+'\n').encode())
 priorBase=pathlib.Path('/home/charl/.local/state/moriarty/sp05-local-loan-20260909-01');plan=json.loads((priorBase/'plan.json').read_text())
 plan['roles']['secretsFile']=str(base/'roles.json');plan['privateState']={'directory':str(privateState),'passwordFile':str(base/'password')};plan['limits']['allocationId']='sp05-local-loan-03';plan['limits']['deadlineMs']=min(int(time.time()*1000)+1200000,shutdownWallMs);plan['networkTag']=genesis[2:];plan['expectedProtocolVersion']=tx['protocolVersion'];plan['outputDirectory']=str(base/'run');save(base/'plan.json',plan);planhash=sha((base/'plan.json').read_bytes());syncdir(base);syncdir(base.parent)
 argv=['/usr/bin/systemd-run','--user','--unit='+unit.removesuffix('.service'),'--property=Type=exec','--property=MemoryMax=4294967296','--property=MemorySwapMax=0','--property=CPUQuota=200%','--property=RuntimeMaxSec=1206','--property=TimeoutStopSec=5','--property=KillMode=control-group','--property=FinalKillSignal=SIGKILL','--property=SendSIGKILL=yes','--property=UMask=0077','--property=StandardOutput=append:'+str(base/'sdk.stdout'),'--property=StandardError=append:'+str(base/'sdk.stderr'),'--property=ExecStopPost=/usr/bin/timeout --signal=KILL 20s /usr/bin/docker stop -t 5 '+' '.join(names),'--working-directory='+str(E/'experiments/moriarty-midnight-financial'),'/usr/local/bin/node','ledger/launch-local.mjs','--run','--plan',str(base/'plan.json'),'--sha256',planhash]
 save(O/'resolved-launch.json',{'argv':argv,'planSha256':planhash,'deadlineMs':plan['limits']['deadlineMs']})
 setupcheck();assert run(['systemctl','--user','is-active',timer+'.timer'],timeout=setupDeadline-time.monotonic()).strip()=='active';setupcheck();remaining=setupDeadline-time.monotonic();assert remaining>0;run(argv,timeout=remaining);setupcheck()
 state=run(['systemctl','--user','show',unit,'--property=ActiveState,SubState,MainPID,ControlGroup,MemoryMax,MemorySwapMax,CPUQuotaPerSecUSec,ActiveEnterTimestampMonotonic']);fields=dict(line.split('=',1) for line in state.splitlines());assert fields['ActiveState']=='active' and int(fields['MainPID'])>0
 entered=int(fields['ActiveEnterTimestampMonotonic'])/1e6;assert arm<=entered<=setupDeadline
 save(O/'launcher-active.json',{'observed':fields,'elapsedSinceArmingSeconds':time.monotonic()-arm});activated=True;print('Successor local loan launcher active; public run records:',str(base/'run'),flush=True)
except BaseException as error:
 if armed:
  for cmd in [['/usr/bin/timeout','--signal=KILL','5s','systemctl','--user','kill','--signal=KILL','--kill-whom=all',unit],['/usr/bin/timeout','--signal=KILL','20s','docker','stop','-t','5',*names],['/usr/bin/timeout','--signal=KILL','5s','docker','kill',*names]]:
   try:subprocess.run(cmd,capture_output=True,timeout=22)
   except Exception:pass
 save(O/'setup-failure.json',{'status':'FAILED','errorClass':type(error).__name__,'timerArmed':armed,'launcherAcknowledged':activated,'retryAllowed':False,'scope':'Retain private diagnostics and verify terminal containment independently'})
 raise SystemExit('Setup failed; admitted services stopped or kill attempted. Inspect retained records; no retry.')
