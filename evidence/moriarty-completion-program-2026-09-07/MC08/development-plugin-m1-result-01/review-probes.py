"""Offline M1 regression probes. Synthetic fixture is not production authority.
Run: PYTHONDONTWRITEBYTECODE=1 python3 review-probes.py [worktree]
Writes only disposable /tmp fixtures. Never executes registered commands.
"""
import sys,json,hashlib,tempfile,subprocess
from pathlib import Path
W=Path(sys.argv[1] if len(sys.argv)>1 else '/home/charl/Moriarty/.worktrees/moriarty-dev-plugin-grok').resolve()
sys.path[:0]=[str(W/'plugins/moriarty-dev/scripts'),str(W/'plugins/moriarty-dev/tests')]
from test_records import GenuineRegisters,verified_history,load_json,dump_json,stage_by_id
from moriarty_dev.records import load_snapshot,read_actions
from moriarty_dev.policy import assess
results=[]
def run(name,mutate):
 f=GenuineRegisters()
 try:
  f.setUp();f._install_admitted_sp05();mutate(f)
  s=load_snapshot(f.root,'sp05-ledger-implement',history_reader=verified_history())
  a=next(x for x in read_actions(f.root) if x['id']=='sp05-ledger-implement')
  d=assess(s,a)
  results.append(dict(name=name,passed=d['allow'] is False,decision=d,missing=s['missingEvidence']))
 except Exception as e:results.append(dict(name=name,passed=False,exception=type(e).__name__))
 finally:f.doCleanups()
def edit(f,p,fn):
 q=f.root/p;d=load_json(q);fn(d);dump_json(q,d)
def campaign(f,fn):edit(f,f.campaign_path,lambda d:fn(d['campaigns']['sp05-ledger-driver-01']))
def binding(f,value):
 q=f.root/f.binding;dump_json(q,value);campaign(f,lambda d:d.update(bindingSha256=hashlib.sha256(q.read_bytes()).hexdigest()))
def action(f,**kw):edit(f,'.moriarty-dev/actions.json',lambda d:d['actions'][0].update(**kw))
run('genuine RP01 design campaign cannot admit SP05 implementation',lambda f:action(f,admissionRef='campaign:sp01-loan-swap-grok-01'))
run('wrong campaign stage/owner',lambda f:campaign(f,lambda d:d.update(stage='f0a',owner='MC99')))
run('missing action-stage campaign linkage',lambda f:edit(f,f.program_path,lambda d:stage_by_id(d,'i2').update(campaignRecordId=None)))
run('null prerequisite candidate/profile/campaign',lambda f:edit(f,f.program_path,lambda d:stage_by_id(d,'atomic-accept').update(candidateHash=None,acceptedProfile=None,campaignRecordId=None)))
run('blocked transitive prerequisite',lambda f:edit(f,f.program_path,lambda d:stage_by_id(d,'atomic-prepare').update(status='blocked')))
run('unknown binding schema',lambda f:binding(f,{'schema':'unknown/999','candidateHash':f.candidate}))
run('empty binding object',lambda f:binding(f,{}))
run('wrong binding schema type returns unresolved, not exception',lambda f:binding(f,{'schema':[]}))
run('unknown catalog key',lambda f:edit(f,'.moriarty-dev/actions.json',lambda d:d.update(unknown=True)))
run('unsupported program version',lambda f:edit(f,f.program_path,lambda d:d.update(schemaVersion=999)))
run('unknown nested program key',lambda f:edit(f,f.program_path,lambda d:stage_by_id(d,'i2').update(unknown=True)))
run('omitted gate requires',lambda f:edit(f,f.sprints_path,lambda d:[g.pop('requires',None) for s in d['sprints'] for g in s.get('entryGates',[]) if 'SP05.2' in g['tasks']]))
run('empty implementation commandRef',lambda f:action(f,commandRef=''))
run('empty argv',lambda f:edit(f,'commands.json',lambda d:d['commands']['driver'].update(argv=[])))
run('unknown command key',lambda f:edit(f,'commands.json',lambda d:d['commands']['driver'].update(unknown=True)))
run('conflicting duplicate action id',lambda f:edit(f,'.moriarty-dev/actions.json',lambda d:d['actions'].append(dict(d['actions'][0],admissionRef='campaign:missing'))))
run('unsupported campaign status',lambda f:campaign(f,lambda d:d.update(status='complete-but-revoked')))
def symlink_program(f):
 t=tempfile.TemporaryDirectory(dir='/tmp');f.addCleanup(t.cleanup);p=Path(t.name)/'outside.json';p.write_bytes(f.program_path.read_bytes());f.program_path.unlink();f.program_path.symlink_to(p)
run('escaping fixed program symlink',symlink_program)
def alias_secret(f):
 p=f.root/'credentials.json';p.write_bytes((f.root/f.resource).read_bytes());q=f.root/f.resource;q.unlink();q.symlink_to(p)
run('resource alias to artificial credentials-named file',alias_secret)
for k,v in [('authority','missing-authority.json'),('scope','unrelated capability'),('allocationSeconds',0),('allocationSeconds',True)]:
 run('resource '+k+'='+repr(v),lambda f,k=k,v=v:edit(f,f.resource,lambda d:d.update({k:v})))
run('empty resource',lambda f:dump_json(f.root/f.resource,{}))
run('negative resource',lambda f:dump_json(f.root/f.resource,{'allocationSeconds':-1}))
run('changed candidate bytes',lambda f:f.candidate_path.write_text('changed\n'))
run('missing candidate bytes',lambda f:f.candidate_path.unlink())
f=GenuineRegisters()
try:
 f.setUp();f._install_admitted_sp05()
 def git(*args):return subprocess.run(['git',*args],cwd=f.root,check=True,capture_output=True,text=True).stdout.strip()
 git('add','.');git('-c','user.name=Fixture','-c','user.email=fixture@example.invalid','commit','-m','fixture')
 with tempfile.TemporaryDirectory(dir='/tmp') as t:
  linked=Path(t)/'linked';git('worktree','add','--detach',str(linked),'HEAD');seen=[]
  def history(lineage):
   seen.append(lineage);return verified_history(2 if not seen or lineage['repository']==seen[0]['repository'] else 0)(lineage)
  a=read_actions(f.root)[0];main=load_snapshot(f.root,a['id'],history_reader=history);other=load_snapshot(linked,a['id'],history_reader=history)
  results.append(dict(name='stable real linked-worktree lineage',passed=seen[0]['repository']==seen[1]['repository'],mainAllow=assess(main,a)['allow'],linkedAllow=assess(other,a)['allow']))
 s=load_snapshot(f.root,'sp05-ledger-implement',history_reader=lambda lineage:verified_history()(lineage)|{'unexpected':True})
 results.append(dict(name='closed history reader result',passed=not assess(s,read_actions(f.root)[0])['allow']))
 # Positive controls concern pure policy only, not fixture admission authority.
 base=dict(authorityCurrent=True,entryEligible=True,candidateCurrent=True,resourceAdmitted=True,sameDefectFailures=2,adminCycles=0,adminSeconds=0,primaryActive=False,reproducerVerified=False,approachChanged=False,nextActionId=None,missingEvidence=[])
 a=read_actions(f.root)[0]
 results.append(dict(name='control broad retry denied',passed=not assess(base,a)['allow']))
 results.append(dict(name='control admitted reproducer remains possible',passed=assess(base,dict(a,kind='reproduce'))['allow']))
 for failures in (0,2):
  snap=dict(base,sameDefectFailures=failures,primaryActive=True,reproducerVerified=True,approachChanged=True)
  results.append(dict(name='repair cannot bypass primary '+str(failures),passed=not assess(snap,dict(a,kind='repair'))['allow']))
finally:f.doCleanups()
print(json.dumps(dict(candidateScope='M1 only; fixture not authority',passed=sum(x['passed'] for x in results),total=len(results),results=results),indent=2))
raise SystemExit(0 if all(x['passed'] for x in results) else 1)
