"""Source-only AST checks. Never imports the operational executor."""
import ast,pathlib,json,types
O=pathlib.Path(__file__).resolve().parent
s=(O/'execute-once.py').read_text();tree=ast.parse(s)
assert "[['--case','loan'],['--case','swap']]" in s
functions=[n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name in ['activate','setupcheck']]
code=compile(ast.Module(body=functions,type_ignores=[]),'<controlled-activation>','exec');rows=[]
for diagnostic in [False,True]:
 for label,start,delay,fail in [('timely',5,0,False),('late',117,0,False),('durable-delay',117,2,True),('expired',120,0,True)]:
  clock=[start];saved=[];calls=[]
  def save(path,value):saved.append(value);clock[0]+=delay
  def run(argv,timeout,input=None):calls.append(argv);return 'inert'
  e={'time':types.SimpleNamespace(monotonic=lambda:clock[0]),'setupDeadline':120,'save':save,'run':run,'O':pathlib.Path('/inert')};exec(code,e)
  try:e['activate'](['systemd-run','--user','--unit=inert','--property=Type=exec'],'record',isDiagnostic=diagnostic);failed=False
  except AssertionError:failed=True
  assert failed==fail
  if calls:assert calls[0]==saved[0]['argv'] and saved[0]['activationAllowanceSeconds']<=120-clock[0]
  if fail:assert not calls
  rows.append({'case':label,'diagnostic':diagnostic,'passed':True})
timerNode=next(n.value for n in tree.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='timerArgv' for t in n.targets))
launchTry=next(n for n in tree.body if isinstance(n,ast.Try))
argvNode=next(n.value for n in launchTry.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='argv' for t in n.targets))
for kind in ['loan','swap']:
 unit='moriarty-sp05-local-'+kind+'-completion-01.service';diagnostic='moriarty-sp05-local-'+kind+'-completion-preflight-01.service';timer='moriarty-sp05-local-'+kind+'-completion-stop-01';names=['moriarty-midnight-proof-server','moriarty-midnight-indexer','moriarty-midnight-node']
 scope={'unit':unit,'diagnostic':diagnostic,'timer':timer,'names':names,'common':[],'base':pathlib.Path('/inert'),'R':pathlib.Path('/repo'),'planhash':'a'*64}
 ta=eval(compile(ast.Expression(timerNode),'<timer>','eval'),{'__builtins__':{}},scope)
 assert ta[3]=='--on-active=1300s' and '--property=TimeoutStartSec=25s' in ta and '--property=TimeoutStopSec=5s' in ta
 assert any('ExecStopPost=' in x and '5s /usr/bin/docker kill ' in x for x in ta) and unit+' '+diagnostic in ta[-1]
 argv=eval(compile(ast.Expression(argvNode),'<argv>','eval'),{'__builtins__':{'str':str}},scope)
 assert '--property=RemainAfterExit=yes' in argv and '--property=RuntimeMaxSec=1206' in argv
 assert 'ledger/launch-local.mjs' in argv and argv[-5]=='--run'
 assert '--property=StandardOutput=append:/inert/sdk.stdout' in argv and '--property=StandardError=append:/inert/sdk.stderr' in argv
assert "fields['InvocationID']" in s and "fields['LoadState']=='loaded'" in s
assert 'timerActiveObserved' in s and "'cleanupActions':cleanup" in s
assert "'stop',timer" not in s
print(json.dumps({'status':'PASS','controlledActivationCases':rows,'closedCaseArgvCount':2,'timerCleanupSeconds':31,'scope':'AST/extracted inert activation only; no executor import, private reads, services or network.'},indent=2))
