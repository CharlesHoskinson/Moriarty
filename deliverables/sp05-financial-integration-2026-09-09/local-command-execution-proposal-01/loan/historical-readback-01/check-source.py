"""Inert AST verification; never imports or executes operational module."""
import ast,builtins,json,pathlib,symtable,types
O=pathlib.Path(__file__).resolve().parent;text=(O/'execute-once.py').read_text();tree=ast.parse(text);module=symtable.symtable(text,'execute-once.py','exec');defined={s.get_name() for s in module.get_symbols() if s.is_assigned() or s.is_imported()}|set(dir(builtins));unknown=set()
def names(t):
 for s in t.get_symbols():
  if s.is_referenced() and (t.get_type()=='module' or s.is_global()) and s.get_name() not in defined:unknown.add(s.get_name())
 for child in t.get_children():names(child)
names(module);assert not unknown,unknown
unit='moriarty-sp05-local-loan-historical-readback-01.service';timer='moriarty-sp05-local-loan-historical-readback-stop-01';containers=['moriarty-midnight-node','moriarty-midnight-indexer'];env={'unit':unit,'timer':timer,'names':containers}
timerNode=next(n for n in tree.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='timerArgv' for t in n.targets));argv=eval(compile(ast.Expression(timerNode.value),'<timer-expression>','eval'),env);assert '--on-active=240s' in argv and '--property=TimeoutStartSec=25s' in argv and '--property=TimeoutStopSec=5s' in argv;assert all('proof-server' not in s for s in argv)
start=next(i for i,line in enumerate(text.splitlines(),1) if 'activationAllowance=int' in line);stop=next(i for i,line in enumerate(text.splitlines(),1) if 'fields=dict(' in line);body=next(n for n in tree.body if isinstance(n,ast.Try)).body;selected=[n for n in body if start<=n.lineno<stop];assert selected
checks=[]
for label,now,savecost,runcost,expected in [('timely',10,.1,.1,True),('saving-exhausts',10,2,0,False),('late-success',117,.1,.1,True),('already-late',119.2,0,0,False),('manager-late',10,.1,111,False)]:
 clock=[now];calls=[];records=[]
 def save(name,value):records.append(value);clock[0]+=savecost
 def run(argv,timeout):calls.append(list(argv));clock[0]+=runcost
 scope={'time':types.SimpleNamespace(monotonic=lambda:clock[0],time=lambda:1000+clock[0]),'activationDeadline':120,'argv':['systemd-run','--user','--unit=synthetic','--property=Type=exec'],'save':save,'run':run};passed=True
 try:exec(compile(ast.Module(body=selected,type_ignores=[]),'<activation-only>','exec'),scope)
 except AssertionError:passed=False
 assert passed==expected,label
 if calls:assert records[0]['argv']==calls[0] and '--property=TimeoutStartSec='+str(records[0]['activationAllowanceSeconds'])+'s' in calls[0]
 checks.append({'case':label,'passed':True})
call=next(n for n in ast.walk(tree) if isinstance(n,ast.Call) and isinstance(n.func,ast.Name) and n.func.id=='save' and n.args and isinstance(n.args[0],ast.Constant) and n.args[0].value=='attempt.json');record=eval(compile(ast.Expression(call.args[1]),'<attempt-dict>','eval'),{'sha':lambda p:'a'*64,'O':pathlib.Path('/inert'),'timerArgv':argv,'argv':['inert']});assert record['admissionSha256']=='a'*64 and record['submissionsAllowed']==0
assert "'--property=RemainAfterExit=yes'" in text and "'cleanupActions':cleanup" in text and "run(['docker','start',*names]" in text
p=json.loads((O/'resource-proposal-01.json').read_text());assert p['submissions']==p['proofs']==0 and p['servicesSeconds']==240 and p['totalSeconds']==300 and p['storage']['minimumHostAvailableMemoryBytes']==20*1024**3
print(json.dumps({'status':'PASS_SOURCE_ONLY','activation':checks,'undefinedGlobals':sorted(unknown),'attemptRecordConstructed':True,'containers':containers,'timerTotalMaxSeconds':271,'operationalModuleImported':False},indent=2))
