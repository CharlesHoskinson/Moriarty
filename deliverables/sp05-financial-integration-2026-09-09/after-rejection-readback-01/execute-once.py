# Draft operational script: do not import or execute for syntax checks.
import pathlib,json,hashlib,os,subprocess,time,shlex,shutil
R=pathlib.Path('/home/charl/Moriarty/.worktrees/sp05-adverse-integration');D=R/'deliverables/sp05-financial-integration-2026-09-09';O=D/'after-rejection-readback-01'
if not __debug__: raise SystemExit('PYTHON_OPTIMIZATION_FORBIDDEN')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
A=json.loads((O/'execution-admission.json').read_text());assert A['status']=='ADMITTED'
P=json.loads((O/'resource-proposal-01.json').read_text());assert sha(O/'resource-proposal-01.json')==A['proposalSha256']
files={f['path']:f['sha256'] for f in A['files']};assert len(files)==len(A['files'])
def bound(ref):
 p=R/ref['path'];assert not pathlib.Path(ref['path']).is_absolute() and '..' not in pathlib.Path(ref['path']).parts
 for q in [p,*p.parents]:assert not q.is_symlink()
 assert files.get(ref['path'])==ref['sha256'] and sha(p)==ref['sha256'];return p
for f in A['files']:bound(f)
for p in ['resource-proposal-01.json','commands-01.md','execute-once.py','probe.mjs','probe.test.mjs','inputs.json','check-source.py']:assert str((O/p).relative_to(R)) in files
candidate=bound(P['sourceCandidate']);assert A['sourceCandidate']==P['sourceCandidate']
source=json.loads(candidate.read_text())
for p,digest in source['files'].items():bound({'path':p,'sha256':digest})
sdkManifestRelative=str((D/'stale-loan-rejection-01/sdk-source-manifest.json').relative_to(R))
assert sdkManifestRelative in source['files']
sdkPins=json.loads(bound({'path':sdkManifestRelative,'sha256':source['files'][sdkManifestRelative]}).read_text());assert len(sdkPins)==9
sdkRoot=pathlib.Path('/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules')
for name,digest in sdkPins.items():
 sdkFile=pathlib.Path(name);assert sdkFile.is_absolute() and sdkFile.is_relative_to(sdkRoot) and '..' not in sdkFile.parts
 for q in [sdkFile,*sdkFile.parents]:assert not q.is_symlink()
 assert sdkFile.is_file() and sha(sdkFile)==digest,'SDK_SOURCE_DRIFT'
for p,digest in json.loads((O/'inputs.json').read_text())['files'].items():bound({'path':p,'sha256':digest})
for group,field,digest,verdicts in [('sourceReviews','candidateSha256',P['sourceCandidate']['sha256'],['PASS_SCOPED','APPROVED']),('resourceVotes','proposalSha256',A['proposalSha256'],['APPROVED','APPROVE','APPROVE_BOUNDED_RUN'])]:
 refs=A[group];assert len(refs)==2 and {x['reviewer'] for x in refs}=={'gpt-6-astra','grok-4.6'}
 for ref in refs:
  receipt=json.loads(bound(ref).read_text());assert receipt['reviewer']==ref['reviewer'] and receipt['verdict'] in verdicts and receipt[field]==digest;bound(receipt['actualReview'])
assert P['attempts']==1 and P['submissions']==0 and P['servicesSeconds']==180 and P['diagnosticSeconds']==90 and P['totalSeconds']==211
assert P['priorAccounting']['reservedSubmissions']==11 and P['priorAccounting']['reservedDustSpeck']=='3300000000000011' and P['priorAccounting']['admittedDustCeilingSpeck']=='4000000000000010'
unit=P['diagnosticUnit'];timer=P['stopUnit'];names=['moriarty-midnight-node'];timerLaunchAttempted=False;timerActiveObserved=False
assert unit=='moriarty-sp05-after-rejection-readback-01.service' and timer=='moriarty-sp05-after-rejection-readback-stop-01'
os.umask(0o077)
def run(argv,timeout=10):return subprocess.run(argv,capture_output=True,text=True,timeout=timeout,check=True).stdout
def save(name,value):
 p=O/name;raw=(json.dumps(value,indent=2)+'\n').encode();assert len(raw)<=65536
 fd=os.open(p,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600)
 try:
  with os.fdopen(fd,'wb',closefd=False) as f:f.write(raw);f.flush();os.fsync(fd)
 finally:os.close(fd)
 fd=os.open(O,os.O_RDONLY|os.O_DIRECTORY)
 try:os.fsync(fd)
 finally:os.close(fd)
for name in [unit,timer+'.timer',timer+'.service']:assert run(['systemctl','--user','show',name,'--property=LoadState','--value']).strip()=='not-found'
assert not (O/'attempt.json').exists() and not (O/'probe-result.json').exists()
mem=dict(x.split(':',1) for x in pathlib.Path('/proc/meminfo').read_text().splitlines());assert int(mem['MemAvailable'].split()[0])*1024>=P['storage']['minimumHostAvailableMemoryBytes'];assert shutil.disk_usage(O).free>=P['storage']['minimumFreeBytes']
containers=json.loads(run(['docker','inspect',*names,*[x['name'] for x in P['stoppedContainers']]]))
for c in containers:
 assert c['State']['Status']=='exited'
 stopped=next((x for x in P['stoppedContainers'] if c['Name']=='/'+x['name']),None)
 if stopped:assert c['Image']=='sha256:'+stopped['imageSha256'];continue
 pin=next(x for x in P['containers'] if c['Name']=='/'+x['name']);h=c['HostConfig'];assert c['Image']=='sha256:'+pin['imageSha256'] and h['Memory']==pin['memory'] and h['MemorySwap']==pin['memorySwapTotal'] and h['NanoCpus']==pin['nanoCpus'] and h['RestartPolicy']['Name']=='no'
timerArgv=['/usr/bin/systemd-run','--user','--unit='+timer,'--on-active=180s','--timer-property=AccuracySec=1s','--property=Type=oneshot','--property=TimeoutStartSec=25s','--property=TimeoutStopSec=5s','--property=ExecStopPost=/usr/bin/timeout --signal=KILL 5s /usr/bin/docker kill '+' '.join(names),'/usr/bin/bash','-c','/usr/bin/timeout --signal=KILL 5s /usr/bin/systemctl --user kill --signal=KILL --kill-whom=all '+unit+'; /usr/bin/timeout --signal=KILL 20s /usr/bin/docker stop -t 5 '+' '.join(names)]
argv=['/usr/bin/systemd-run','--user','--unit='+unit.removesuffix('.service'),'--property=Type=exec','--property=MemoryMax=4294967296','--property=MemorySwapMax=0','--property=CPUQuota=200%','--property=RuntimeMaxSec=90','--property=TimeoutStopSec=1','--property=KillMode=control-group','--property=FinalKillSignal=SIGKILL','--property=SendSIGKILL=yes','--property=UMask=0077','--property=StandardOutput=null','--property=StandardError=null','--working-directory='+str(R),'/usr/local/bin/node',str(O/'probe.mjs')]
save('attempt.json',{'status':'CONSUMED','admissionSha256':sha(O/'execution-admission.json'),'timerArgv':timerArgv,'diagnosticArgvTemplate':argv,'submissionsAllowed':0,'retryAllowed':False})
try:
 timerLaunchAttempted=True;run(timerArgv)
 obj=shlex.split(run(['busctl','--user','call','org.freedesktop.systemd1','/org/freedesktop/systemd1','org.freedesktop.systemd1.Manager','GetUnit','s',timer+'.timer']))[1]
 def prop(k):return int(shlex.split(run(['busctl','--user','get-property','org.freedesktop.systemd1',obj,'org.freedesktop.systemd1.Timer',k]))[1])
 nextUS=prop('NextElapseUSecMonotonic');accuracy=prop('AccuracyUSec');assert 0<accuracy<=1000000
 arm=nextUS/1e6-180;activationDeadline=arm+90
 assert run(['systemctl','--user','is-active',timer+'.timer']).strip()=='active';timerActiveObserved=True
 save('timer-armed.json',{'argv':timerArgv,'armingMonotonicSeconds':arm,'nextElapseMonotonicMicroseconds':nextUS,'accuracyMicroseconds':accuracy})
 assert time.monotonic()<activationDeadline;run(['docker','start','moriarty-midnight-node'],timeout=max(.001,activationDeadline-time.monotonic()))
 assert time.monotonic()<activationDeadline;activationAllowance=int(activationDeadline-time.monotonic())-1;assert activationAllowance>=1
 argv.insert(4,'--property=TimeoutStartSec='+str(activationAllowance)+'s')
 save('resolved-launch.json',{'argv':argv,'activationDeadlineMonotonicSeconds':activationDeadline,'activationAllowanceSeconds':activationAllowance})
 assert time.monotonic()<activationDeadline;remaining=activationDeadline-time.monotonic();assert remaining>=activationAllowance;run(argv,timeout=remaining);assert time.monotonic()<activationDeadline
 fields=dict(x.split('=',1) for x in run(['systemctl','--user','show',unit,'--property=ActiveState,SubState,MainPID,ActiveEnterTimestampMonotonic']).splitlines());assert fields['ActiveState']=='active' and int(fields['MainPID'])>0 and arm<=int(fields['ActiveEnterTimestampMonotonic'])/1e6<=activationDeadline
 save('diagnostic-active.json',fields)
 print('Read-only diagnostic active; inspect its bounded public result and independent terminal containment.')
except BaseException as e:
 if timerLaunchAttempted:
  for cmd in [['/usr/bin/timeout','--signal=KILL','5s','systemctl','--user','kill','--signal=KILL','--kill-whom=all',unit],['/usr/bin/timeout','--signal=KILL','20s','docker','stop','-t','5',*names],['/usr/bin/timeout','--signal=KILL','5s','docker','kill',*names]]:
   try:subprocess.run(cmd,capture_output=True,timeout=22)
   except Exception:pass
 save('setup-failure.json',{'status':'FAILED','errorClass':type(e).__name__,'retryAllowed':False,'timerLaunchAttempted':timerLaunchAttempted,'timerActiveObserved':timerActiveObserved});raise SystemExit('Diagnostic setup failed; verify terminal containment; no retry.')
