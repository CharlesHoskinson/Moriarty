import sys,json,tempfile,hashlib
from pathlib import Path
S=Path(__file__).resolve().parent
W=Path('/home/charl/Moriarty/.worktrees/moriarty-dev-plugin-grok')
sys.path[:0]=[str(W/'plugins/moriarty-dev/scripts'),str(W/'plugins/moriarty-dev/tests')]
import test_records as t
from moriarty_dev.records import load_snapshot,read_actions
from moriarty_dev.policy import assess
results=[]
with tempfile.TemporaryDirectory(dir=S,prefix='review2-fixture-') as d:
 r=Path(d);t.git_init(r); pp,sp,cp=t.copy_genuine_registers(r)
 t.write_integer_budget(r);t.write_commands(r);a=t.loan_action('review','commands.json#driver');t.write_actions(r,[a])
 camp=t.load_json(cp)['campaigns']['sp01-loan-swap-grok-01']
 paths={'actions':r/'.moriarty-dev/actions.json','campaigns':cp,'program':pp,'sprints':sp,'manifest':r/camp['candidateManifest'],'acceptance':r/camp['acceptance'],'review':r/camp['review'],'budget':r/t.BUDGET_REL,'binding':r/camp['binding'],'resource':r/camp['resourceAmendment']}
 originals={p:p.read_bytes() for p in paths.values()}
 def check(name,fn=None):
  for p,b in originals.items():p.write_bytes(b)
  if fn:fn()
  try:
   aa=read_actions(r)[0]; ss=load_snapshot(r,aa['id'],history_reader=t.verified_history());dd=assess(ss,aa)
   results.append({'probe':name,'allow':dd['allow'],'authority':ss['authorityCurrent'],'candidate':ss['candidateCurrent'],'entry':ss['entryEligible'],'resource':ss['resourceAdmitted'],'next':ss['nextActionId'],'missing':ss['missingEvidence']})
  except Exception as e:results.append({'probe':name,'exception':type(e).__name__})
 def edit(key,fn):
  p=paths[key];j=t.load_json(p);fn(j);t.dump_json(p,j)
 def act(k,v):edit('actions',lambda j:j['actions'][0].__setitem__(k,v))
 def record(k,v):edit('campaigns',lambda j:j['campaigns']['sp01-loan-swap-grok-01'].__setitem__(k,v))
 def stage(k,v):edit('program',lambda j:t.stage_by_id(j,'rp01-mc02').__setitem__(k,v))
 check('positive-author-integer-budget')
 check('action-wrong-candidate',lambda:act('candidate','0'*64))
 check('action-wrong-evidence-profile',lambda:act('evidenceProfile','finalized-financial-settlement'))
 check('action-unrelated-capability',lambda:act('capability','native-recursive-financial-proof'))
 check('stage-wrong-candidate',lambda:stage('candidateHash','0'*64))
 check('stage-blocked',lambda:stage('status','blocked'))
 check('stage-null-profile',lambda:stage('acceptedProfile',None))
 check('campaign-null-profile',lambda:record('acceptedProfile',None))
 check('campaign-unknown-field',lambda:record('revoked',True))
 check('review-blocked',lambda:edit('review',lambda j:j.__setitem__('verdict','BLOCKED')))
 check('review-missing-scope',lambda:edit('review',lambda j:j.pop('scope')))
 check('review-wrong-provider',lambda:edit('review',lambda j:j.__setitem__('reviewer','author-grok')))
 check('acceptance-revoked',lambda:edit('acceptance',lambda j:j.__setitem__('status','revoked')))
 check('acceptance-null-candidate',lambda:edit('acceptance',lambda j:j.__setitem__('candidateHash',None)))
 check('manifest-missing-inputs',lambda:edit('manifest',lambda j:j.pop('inputs')))
 check('manifest-unknown-schema',lambda:edit('manifest',lambda j:j.__setitem__('schema','unsupported/999')))
 check('stage-admission-unknown-schema',lambda:edit('program',lambda j:j['reportReconciliation']['stageAdmission'].__setitem__('schema','unsupported/999')))
 check('program-unknown-field',lambda:edit('program',lambda j:j.__setitem__('revoked',True)))
 check('resource-unknown-schema',lambda:edit('resource',lambda j:j.__setitem__('schema','unsupported/999')))
 check('budget-schema-only',lambda:t.dump_json(paths['budget'],{'schema':'moriarty.supervised-accounting/1'}))
 def prerequisite(field,value):
  def f(j):
   st=t.stage_by_id(j,'rp01-mc02');st['requires']=['atomic-accept']
   t.stage_by_id(j,'atomic-prepare')[field]=value
   for sprint in t.load_json(sp)['sprints']:
    pass
  edit('program',f)
  def g(j):
   for sprint in j['sprints']:
    for gate in sprint.get('entryGates',[]):
     if 'SP01.6' in gate['tasks']:gate['requires']=['atomic-accept']
  edit('sprints',g)
 check('prerequisite-missing-campaign',lambda:prerequisite('campaignRecordId','absent-campaign'))
 check('prerequisite-wrong-candidate',lambda:prerequisite('candidateHash','0'*64))
 check('prerequisite-blocked-control',lambda:prerequisite('status','blocked'))
 def rebase_payload():
  owned=t.load_json(paths['manifest'])['ownedFiles'];p=next(p for p in owned if p.endswith('/design.md'));target=r/p; old=target.read_bytes();target.write_bytes(old+b'\nreview mutation\n')
  edit('acceptance',lambda j:j['unchangedPayloadHashes'].__setitem__(p,t.sha256_file(target)))
  return target,old
 for p,b in originals.items():p.write_bytes(b)
 target,old=rebase_payload()
 try:
  ss=load_snapshot(r,a['id'],history_reader=t.verified_history());results.append({'probe':'changed-payload-acceptance-override','allow':assess(ss,a)['allow'],'candidate':ss['candidateCurrent'],'missing':ss['missingEvidence']})
 finally:target.write_bytes(old)
 (S/'review2-probes.json').write_text(json.dumps(results,indent=2)+'\n')
 print(json.dumps(results,indent=2))
