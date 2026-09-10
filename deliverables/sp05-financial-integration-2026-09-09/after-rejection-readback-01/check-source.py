"""Only AST/extracted controlled statements; never imports operational executor."""
import ast,json,pathlib,types,hashlib
O=pathlib.Path(__file__).resolve().parent;R=O.parents[2];text=(O/'execute-once.py').read_text();tree=ast.parse(text)
start=next(i for i,line in enumerate(text.splitlines(),1) if 'activationAllowance=int(activationDeadline-time.monotonic())-1' in line)
stop=next(i for i,line in enumerate(text.splitlines(),1) if "fields=dict(x.split('=',1)" in line)
body=next(n for n in tree.body if isinstance(n,ast.Try)).body;selected=[n for n in body if start<=n.lineno<stop];assert selected
checks=[]
for label,initial,savecost,runcost,calls_expected,pass_expected in [('timely',10,.1,.1,1,True),('record-consumed-reserve',10,2,0,0,False),('late-valid',87,.1,2,1,True),('too-late',88.2,0,0,0,False),('manager-return-late',10,.1,80,1,False)]:
 clock=[initial];saved=[];calls=[]
 def save(name,value):saved.append(value);clock[0]+=savecost
 def run(argv,timeout):calls.append((list(argv),timeout));clock[0]+=runcost
 scope={'time':types.SimpleNamespace(monotonic=lambda:clock[0]),'activationDeadline':90,'argv':['systemd-run','--user','--unit=synthetic','--property=Type=exec'],'save':save,'run':run}
 passed=True
 try:exec(compile(ast.Module(body=selected,type_ignores=[]),'<controlled-activation-statements>','exec'),scope)
 except AssertionError:passed=False
 assert passed==pass_expected and len(calls)==calls_expected,label
 if calls:assert saved[0]['argv']==calls[0][0] and calls[0][1]>=saved[0]['activationAllowanceSeconds'] and '--property=TimeoutStartSec='+str(saved[0]['activationAllowanceSeconds'])+'s' in calls[0][0]
 checks.append({'name':label,'passed':True})
timer=next(n for n in tree.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='timerArgv' for t in n.targets))
argv=eval(compile(ast.Expression(timer.value),'<literal-timer-only>','eval'),{'timer':'moriarty-sp05-after-rejection-readback-stop-01','unit':'moriarty-sp05-after-rejection-readback-01.service','names':['moriarty-midnight-node']})
assert '--on-active=180s' in argv
assert all('indexer' not in a and 'proof-server' not in a for a in argv)
assert '/usr/bin/timeout --signal=KILL 5s' in argv[-1] and '/usr/bin/timeout --signal=KILL 20s' in argv[-1]
assert text.index("A['status']=='ADMITTED'")<text.index('os.umask')
assert text.index("'SDK_SOURCE_DRIFT'")<text.index('os.umask')
assert "bound(receipt['actualReview'])" in text and "{'gpt-6-astra','grok-4.6'}" in text
assert "'timerLaunchAttempted':timerLaunchAttempted,'timerActiveObserved':timerActiveObserved" in text
proposal=json.loads((O/'resource-proposal-01.json').read_text());assert proposal['status']=='PROPOSED' and proposal['submissions']==proposal['proofs']==0 and proposal['dustFeeSpeck']=='0'
a=proposal['priorAccounting'];assert int(a['oldReservedDustSpeck'])+int(a['latestReservedDustSpeck'])==int(a['reservedDustSpeck'])==3300000000000011 and a['reservedSubmissions']==11
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
for name,pin in json.loads((O/'inputs.json').read_text())['files'].items():assert sha(R/name)==pin
ref=proposal['sourceCandidate'];bound=ref!={'path':'UNBOUND','sha256':'UNBOUND'}
if bound:
 assert sha(R/ref['path'])==ref['sha256'];candidate=json.loads((R/ref['path']).read_text())
 for name,pin in candidate['files'].items():assert sha(R/name)==pin,name
print(json.dumps({'status':'PASS_SOURCE_ONLY','activationChecks':checks,'nodeOnlyTimer':True,'boundCandidate':bound,'priorReservedDustSpeck':a['reservedDustSpeck'],'executorSha256':sha(O/'execute-once.py'),'operationalExecutorImported':False,'nodeServicesStarted':False,'submissions':0},indent=2))
