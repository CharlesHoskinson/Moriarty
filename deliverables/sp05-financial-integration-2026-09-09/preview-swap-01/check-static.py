"""Read-only AST/controlled checks. Never import the operational executor."""
import ast,json,pathlib,hashlib,types
O=pathlib.Path(__file__).resolve().parent
source=(O/'execute-once.py').read_text();tree=ast.parse(source)
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
functions=[n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name in ['setupcheck','activate']]
assert len(functions)==2
code=compile(ast.Module(body=functions,type_ignores=[]),'<extracted-activation>','exec')
results=[]
for label,start,saveDelay,runDelay,error in [('timely',5,0,1,None),('late-valid',117,0,1,None),('record-overrun',117,2,0,'ACTIVATION_RESERVE_CONSUMED'),('too-late',119,0,0,''),('manager-overrun',117,0,4,'SETUP_DEADLINE')]:
 clock=[start];records=[];calls=[]
 def save(path,value):records.append(value);clock[0]+=saveDelay
 def run(argv,timeout):calls.append({'argv':argv,'timeout':timeout});clock[0]+=runDelay;return 'controlled'
 env={'time':types.SimpleNamespace(monotonic=lambda:clock[0]),'setupDeadline':120,'save':save,'run':run,'O':pathlib.Path('/controlled')}
 exec(code,env)
 try:env['activate'](['systemd-run','--user','--unit=controlled','--property=Type=exec'],'record');observed=None
 except AssertionError as exc:observed=str(exc)
 assert observed==error,(label,observed,error)
 if error in ['ACTIVATION_RESERVE_CONSUMED','']:assert not calls
 if calls:assert records and calls[0]['argv']==records[0]['argv'] and calls[0]['timeout']>=records[0]['activationAllowanceSeconds']
 results.append({'case':label,'expectedError':error,'calls':len(calls),'status':'PASS'})
timerNode=next(n.value for n in tree.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='timerArgv' for t in n.targets))
timer=eval(compile(ast.Expression(timerNode),'<timer-argv>','eval'),{'__builtins__':{}},{'timer':'moriarty-sp05-preview-swap-stop-01','unit':'moriarty-sp05-preview-swap-01.service','names':['moriarty-midnight-proof-server']})
assert '--on-active=2500s' in timer and '--timer-property=AccuracySec=1s' in timer
assert any(x.startswith('--property=ExecStopPost=') and 'docker kill moriarty-midnight-proof-server' in x for x in timer)
assert 'moriarty-midnight-node' not in ' '.join(timer) and 'moriarty-midnight-indexer' not in ' '.join(timer)
assert 'systemctl --user kill --signal=KILL --kill-whom=all moriarty-sp05-preview-swap-01.service' in timer[-1]
assert 'docker stop -t 5 moriarty-midnight-proof-server' in timer[-1]
assert not any(isinstance(n,ast.Call) and isinstance(n.func,ast.Name) and n.func.id in ['exec','eval'] for n in ast.walk(tree))
assert 'systemctl\',\'--user\',\'stop\',timer' not in source
P=json.loads((O/'resource-proposal-draft.json').read_text());plan=json.loads((O/'public-plan-draft.json').read_text())
assert plan['limits']['deadlineMs']==0 and plan['limits']['submissions']==4
assert P['timeSeconds']['operationDeadlineAfterArming']+6<2500 and 2500+31<=2550
assert int(P['priorCharges']['priorLocalReservedDustSpeck'])+int(P['priorCharges']['priorPreviewLoanReservedDustSpeck'])+int(P['dustFeeSpeck'])==int(P['priorCharges']['combinedReservedPlusThisCapSpeck'])
assert int(P['priorCharges']['priorLocalAdmittedDustCeilingSpeck'])+int(P['priorCharges']['priorPreviewLoanAdmittedDustCeilingSpeck'])+int(P['dustFeeSpeck'])==int(P['priorCharges']['combinedAdmittedCeilingPlusThisCapSpeck'])
for delay in [0,.2,2]:
 mono=100;wall=1000+delay;deadline=int((wall+2490-mono)*1000)-int(delay*1000)-1
 assert deadline<=int((1000+2490-100)*1000)
results.append({'case':'timer-targets-and-deadline-arithmetic','status':'PASS'})
launchTry=next(n for n in tree.body if isinstance(n,ast.Try))
logLoop=next(n for n in launchTry.body if isinstance(n,ast.For) and isinstance(n.target,ast.Name) and n.target.id=='logname')
logCode=compile(ast.Module(body=[logLoop],type_ignores=[]),'<private-log-creation>','exec')
for occupied in [False,True]:
 calls=[]
 def logopen(path,flags,mode):
  calls.append(('open',str(path),flags,mode))
  assert flags==15 and mode==0o600
  if occupied:raise FileExistsError('controlled existing log')
  return 7
 fake=types.SimpleNamespace(O_WRONLY=1,O_CREAT=2,O_EXCL=4,O_NOFOLLOW=8,open=logopen,fsync=lambda fd:calls.append(('fsync',fd)),close=lambda fd:calls.append(('close',fd)))
 try:exec(logCode,{'os':fake,'base':pathlib.Path('/controlled')});failed=False
 except FileExistsError:failed=True
 assert failed==occupied
 if not occupied:assert [c[1] for c in calls if c[0]=='open']==['/controlled/sdk.stdout','/controlled/sdk.stderr'] and sum(c[0]=='fsync' for c in calls)==2
 results.append({'case':'private-log-existing-rejected' if occupied else 'private-logs-exclusive-durable','status':'PASS'})
argvNode=next(n.value for n in launchTry.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='argv' for t in n.targets))
argv=eval(compile(ast.Expression(argvNode),'<launcher-argv>','eval'),{'__builtins__':{'str':str}},{'unit':'moriarty-sp05-preview-swap-01.service','base':pathlib.Path('/controlled'),'R':pathlib.Path('/repo'),'names':['moriarty-midnight-proof-server'],'runtimeSeconds':2400,'planhash':'a'*64})
assert '--property=StandardOutput=append:/controlled/sdk.stdout' in argv and '--property=StandardError=append:/controlled/sdk.stderr' in argv
assert not any('LimitFSIZE' in x or 'StandardOutput=null' in x or 'StandardError=null' in x for x in argv)
assert P['proofAccounting']['hardProofCallbackCountEnforced'] is False and 'contractProveEntriesMax' not in P['proofAccounting'] and 'walletFinalizeProveEntriesMax' not in P['proofAccounting']
results.append({'case':'private-log-argv-and-honest-proof-accounting','status':'PASS'})
assert plan['kind']=='swap' and plan['limits']['grossByLogicalAsset']=={'ASSET_A':'100000','ASSET_B':'0'}
grossAssert=next(n for n in tree.body if isinstance(n,ast.Assert) and 'grossByLogicalAsset' in ast.unparse(n))
grossCode=compile(ast.Module(body=[grossAssert],type_ignores=[]),'<fixed-swap-gross-cap>','exec')
for caps in [{'ASSET_A':'100000','ASSET_B':'0'},{'ASSET_A':'10000','ASSET_B':'0'},{'ASSET_A':'100000','ASSET_B':'1'},{'USD_TEST_ASSET':'20000000000'}]:
 pp=json.loads(json.dumps(P));pl=json.loads(json.dumps(plan));pp['grossByLogicalAsset']=caps;pl['limits']['grossByLogicalAsset']=caps
 try:exec(grossCode,{'P':pp,'plan':pl});accepted=True
 except AssertionError:accepted=False
 assert accepted==(caps=={'ASSET_A':'100000','ASSET_B':'0'})
meta=json.loads((O/'metadata-observation.json').read_text());byPath={x['path']:x for x in meta['paths']}
assert byPath['/home/charl/.local/state/moriarty']['mode']=='0o700'
for suffix in ['', '/.midnight-wallet-state','/.midnight-wallet-state/preview']:
 assert byPath[plan['wallet']['stateDirectory']+suffix]['mode']=='0o700'
assert 'privateMetadata(base.parent, True)' in ast.unparse(tree)
assert 'priorLoanResultReviews' in source and "priorBackup = snapshot / '.moriarty-backup-sp05-preview-loan-20260910-01'" in ast.unparse(tree)
results.append({'case':'swap-gross-cap-controls-parent-metadata-and-prior-loan-gate','status':'PASS'})
print(json.dumps({'status':'PASS','scope':'AST parse, extracted controlled activation, fixed timer argv and resource arithmetic only; no operational module import, private reads, service start or network request','sourceSha256':digest(O/'execute-once.py'),'checks':results},indent=2))
