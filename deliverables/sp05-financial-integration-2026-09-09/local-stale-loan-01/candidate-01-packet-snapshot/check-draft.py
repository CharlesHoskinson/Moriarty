"""AST and extracted controlled guards only. Never imports the executor."""
import ast,hashlib,json,pathlib,types
O=pathlib.Path(__file__).resolve().parent;R=O.parents[2];executor=O/'execute-once.py';raw=executor.read_bytes();tree=ast.parse(raw,filename=str(executor));proposal=json.loads((O/'resource-proposal-01.json').read_text());fixed=json.loads((O/'fixed-public-inputs.json').read_text())
activate=next(node for node in tree.body if isinstance(node,ast.FunctionDef) and node.name=='activate')
checks=[]
for label,start,recordCost,runCost,expectedCalls,expectedPass in [('timely',10,.1,.1,1,True),('durable-record-reserve-consumed',10,2,0,0,False),('late-valid',117,.1,2,1,True),('too-late-before-record',118.2,0,0,0,False),('late-manager-return',10,.1,110,1,False)]:
 for diagnostic in [False,True]:
  clock=[start];saved=[];called=[]
  def setupcheck():assert clock[0]<120
  def save(path,value):saved.append((str(path),value));clock[0]+=recordCost
  def run(argv,timeout,input=None):called.append((argv,timeout,input));clock[0]+=runCost;return 'CONTROLLED_ONLY'
  scope={'time':types.SimpleNamespace(monotonic=lambda:clock[0]),'setupDeadline':120,'setupcheck':setupcheck,'save':save,'run':run,'O':pathlib.Path('/synthetic/no-io')}
  exec(compile(ast.Module(body=[activate],type_ignores=[]),'<extracted-activation-guard>','exec'),scope)
  passed=True
  try:scope['activate'](['systemd-run','--user','--unit=synthetic','--property=Type=exec'],'resolved.json',input='PUBLIC_SYNTHETIC',isDiagnostic=diagnostic)
  except AssertionError:passed=False
  assert passed==expectedPass and len(called)==expectedCalls,(label,diagnostic)
  if called:
   assert saved and saved[0][1]['argv']==called[0][0]
   allowance=saved[0][1]['activationAllowanceSeconds'];assert '--property=TimeoutStartSec='+str(allowance)+'s' in called[0][0]
   assert called[0][1]>=allowance
   if diagnostic:assert '--property=RuntimeMaxSec='+str(allowance)+'s' in called[0][0]
  checks.append({'name':label,'diagnostic':diagnostic,'passed':True,'calls':len(called),'retainedBeforeDispatch':bool(saved) if called else None})
# One direct outer timer declaration; neither unit nor Docker start is invoked here.
timer=next(n for n in ast.walk(tree) if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='timerArgv' for t in n.targets))
scope={'timer':'moriarty-sp05-stale-loan-stop-01','unit':'moriarty-sp05-stale-loan-01.service','diagnostic':'moriarty-sp05-stale-loan-preflight-01.service','names':['moriarty-midnight-proof-server','moriarty-midnight-indexer','moriarty-midnight-node']}
argv=eval(compile(ast.Expression(timer.value),'<timer-argv-only>','eval'),scope)
assert '--on-active=1300s' in argv
for unit in [scope['unit'],scope['diagnostic']]:assert unit in argv[-1]
for name in scope['names']:assert name in argv[-1] and name in next(s for s in argv if s.startswith('--property=ExecStopPost='))
assert '/usr/bin/timeout --signal=KILL 5s' in argv[-1] and '/usr/bin/timeout --signal=KILL 20s' in argv[-1]
source=raw.decode();assert 'systemctl\',\'--user\',\'stop\',timer' not in source
assert source.index("A['status']=='ADMITTED'")<source.index('os.umask')
assert source.index("P['sourceCandidate']['path']!='UNBOUND'")<source.index('os.umask')
assert "{'gpt-6-astra','grok-4.6'}" in source and "bound(receipt['actualReview'])" in source
assert proposal['status']=='PROPOSED'
ref=proposal['sourceCandidate'];proposalUnbound=ref=={'path':'UNBOUND','sha256':'UNBOUND'}
if not proposalUnbound:
 assert not pathlib.Path(ref['path']).is_absolute() and '..' not in pathlib.Path(ref['path']).parts
 candidatePath=R/ref['path'];assert not candidatePath.is_symlink() and hashlib.sha256(candidatePath.read_bytes()).hexdigest()==ref['sha256']
 candidate=json.loads(candidatePath.read_text());assert candidate['files']
 for name,digest in candidate['files'].items():
  assert not pathlib.Path(name).is_absolute() and '..' not in pathlib.Path(name).parts
  file=R/name;assert not file.is_symlink() and hashlib.sha256(file.read_bytes()).hexdigest()==digest,name
assert "'timerLaunchAttempted':timerLaunchAttempted,'timerActiveObserved':timerActiveObserved" in source
assert source.index('timerLaunchAttempted=True;run(timerArgv)')<source.index('timerActiveObserved=True;setupcheck()')
assert proposal['attempts']==proposal['submissions']==1 and proposal['grossByLogicalAsset']=={'USD_TEST_ASSET':'0'}
assert proposal['historicalCharges']['reservedSubmissions']==10 and proposal['historicalCharges']['reservedDustSpeck']=='3000000000000010'
assert proposal['timeSeconds']['independentShutdownStart']+31<=proposal['timeSeconds']['totalMax']
for key,value in fixed.items():
 if key.endswith('File'):
  assert not pathlib.Path(value).is_absolute() and '..' not in pathlib.Path(value).parts
  pin=fixed[key[:-4]+('Hash' if key.endswith('TransactionFile') else 'Sha256')];assert hashlib.sha256((R/value).read_bytes()).hexdigest()==pin,key
result={'schema':'moriarty.stale-loan-draft-static-checks/1','status':'PASS_SOURCE_ONLY','executorSha256':hashlib.sha256(raw).hexdigest(),'activationChecks':checks,'timerBothCgroupsAndAllThreeContainers':True,'admissionGatesPrecedeOperationalBoundary':True,'fixedPublicInputsMatch':True,'proposalUnbound':proposalUnbound,'operationalScriptImported':False,'servicesStarted':False,'privatePathsRead':False,'submissions':0,'limitations':['AST/extracted controlled checks only, not observed systemd behavior or source/resource approval.','No complete plan/runtime/public-preflight execution occurred.']}
print(json.dumps(result,indent=2))
