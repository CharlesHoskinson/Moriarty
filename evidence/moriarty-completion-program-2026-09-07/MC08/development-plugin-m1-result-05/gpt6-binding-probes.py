import sys,json,hashlib,tempfile,importlib.util,unittest,io
from pathlib import Path
sys.dont_write_bytecode=True
S=Path(__file__).resolve().parent;W=Path('/home/charl/Moriarty/.worktrees/moriarty-dev-plugin-grok')
sys.path.insert(0,str(S/'candidate/plugins/moriarty-dev/scripts'))
spec=importlib.util.spec_from_file_location('fixture',S/'candidate/plugins/moriarty-dev/tests/test_records.py');t=importlib.util.module_from_spec(spec);spec.loader.exec_module(t)
old=t.SOURCE_ROOT
for key,value in list(vars(t).items()):
 if isinstance(value,Path) and value.is_relative_to(old):setattr(t,key,W/value.relative_to(old))
def repo():
 temp=tempfile.TemporaryDirectory(prefix='gpt6-binding-',dir=S);root=Path(temp.name);t.git_init(root);return temp,root
t.make_repo=repo;t.MARKER=S/'gpt6-no-child-marker'
freeze=json.loads((S/'candidate-freeze.json').read_text())
assert all(hashlib.sha256((S/'candidate'/p).read_bytes()).hexdigest()==h for p,h in freeze['files'].items())
assert all(hashlib.sha256((W/p).read_bytes()).hexdigest()==h for p,h in freeze['sources'].items())
assert hashlib.sha256(json.dumps(freeze['files'],sort_keys=True,separators=(',',':')).encode()).hexdigest()==freeze['candidateHash']
import moriarty_dev.records as r
assert Path(r.__file__).is_relative_to(S/'candidate')
stream=io.StringIO();result=unittest.TextTestRunner(stream=stream,verbosity=1).run(unittest.defaultTestLoader.loadTestsFromModule(t))
(S/'gpt6-frozen-tests.txt').write_text(stream.getvalue())
rows=[]
cases=['positive','positive-prerequisite','atomic-unsupported','atomic-overlay','direct-profile','prerequisite-profile','direct-binding-stage','prerequisite-binding-stage','acceptance-unknown','acceptance-revoked','campaign-unknown','review-unknown','pub-scope','pub-network','pub-extra','pub-closure-escalate','pub-closure-delete','pub-closure-null','pub-static-existing-unrelated','pub-static-failed-receipt','pub-accepted-code-escalate','pub-recordedAt-nonsense','pub-review-different-approved-file','pub-hashstatus-unbound','pub-output-unsupported','bounds-changed-not-rehashed','binding-unknown-status','prerequisite-binding-unknown-status']
for case in cases:
 c=t.GenuineRegisters();c.setUp()
 try:
  c._install_funded_loan_review();campaigns=t.load_json(c.campaign_path);loan=campaigns['campaigns']['sp01-loan-swap-grok-01'];program=t.load_json(c.program_path)
  ap=c.root/loan['acceptance'];acc=t.load_json(ap);pp=c.root/acc['publishedControlPath'];control=t.load_json(pp);sub=control['subsets']['RP01-MC02']
  if case in ('positive-prerequisite','prerequisite-profile','prerequisite-binding-stage','prerequisite-binding-unknown-status'):program=t.require_supported_prereq(c.program_path,c.sprints_path)
  if case in ('atomic-unsupported','atomic-overlay'):
   program=t.require_atomic_prereq(c.program_path,c.sprints_path)
   if case=='atomic-overlay':t.overlay_supported_prereq(campaigns,program,loan)
  if case in ('direct-profile','prerequisite-profile'):
   loan['acceptedProfile']='finalized-financial-settlement';t.stage_by_id(program,'rp01-mc02')['acceptedProfile']=loan['acceptedProfile']
  if case in ('direct-binding-stage','prerequisite-binding-stage','binding-unknown-status','prerequisite-binding-unknown-status','bounds-changed-not-rehashed'):
   bp=c.root/loan['binding'];binding=t.load_json(bp)
   if 'binding-stage' in case:binding['stage']='atomic-accept'
   elif 'unknown-status' in case:binding['status']='REVOKED-NOT-ADMITTED'
   else:
    for rel in binding['inputs']:
     path=c.root/rel
     if path.suffix=='.json':
      obj=t.load_json(path)
      if isinstance(obj,dict) and obj.get('schemaVersion')=='moriarty-bounds/1':obj['semanticProfile']='finalized-financial-settlement';t.dump_json(path,obj)
   if case!='bounds-changed-not-rehashed':t.dump_json(bp,binding);loan['bindingSha256']=t.sha256_file(bp)
  if case=='acceptance-unknown':acc['status']='complete-NOT_REVIEWED'
  if case=='acceptance-revoked':acc['status']='complete-but-REVOKED'
  if case=='campaign-unknown':loan['status']='complete-NOT_REVIEWED'
  if case=='review-unknown':
   revp=c.root/loan['review'];rev=t.load_json(revp);rev['verdict']='NOT_REVIEWED';t.dump_json(revp,rev)
  if case.startswith('pub-'):
   if case=='pub-scope':sub['acceptance']['scope']='Full RP01 native proof and Preview acceptance'
   if case=='pub-network':sub['acceptance']['networkMilestones']='complete'
   if case=='pub-extra':sub['acceptance']['nativeProofsAccepted']=True
   if case=='pub-closure-escalate':
    for row in sub['rowSummaries'].values():row['closureTask']='All obligations closed.'
   if case=='pub-closure-delete':
    for row in sub['rowSummaries'].values():row.pop('closureTask',None)
   if case=='pub-closure-null':
    for row in sub['rowSummaries'].values():row['closureTask']=None
   if case=='pub-static-existing-unrelated':sub['acceptance']['staticVerification']='commands.json'
   if case=='pub-static-failed-receipt':
    receipt=c.root/sub['acceptance']['staticVerification'];obj=t.load_json(receipt);obj['exitCode']=1;obj['scope']='unreviewed';t.dump_json(receipt,obj)
   if case=='pub-accepted-code-escalate':sub['acceptance']['acceptedAtomicCode']='All financial settlement and native recursive proof implementations accepted.'
   if case=='pub-recordedAt-nonsense':sub['acceptance']['recordedAt']='All twelve sprints now accepted'
   if case=='pub-review-different-approved-file':
    newrel='.moriarty-dev/unrelated-approved.json';newp=c.root/newrel;t.dump_json(newp,{'verdict':'APPROVED','scope':'full native proofs','candidateSha256':'0'*64});acc['review']=newrel;sub['acceptance']['review']=newrel;sub['reviews']=[{'path':newrel,'scope':r.DESIGN_SCOPE,'sha256':t.sha256_file(newp),'verdict':'APPROVED'}]
   if case=='pub-hashstatus-unbound':sub['candidateHashStatus']='unbound'
   if case=='pub-output-unsupported':sub['outputs'][0]['status']='complete-native-proofs'
   t.dump_json(pp,control);acc['publishedControlSha256']=t.sha256_file(pp)
  t.dump_json(ap,acc);t.dump_json(c.program_path,program);t.dump_json(c.campaign_path,campaigns)
  snap=t.load_snapshot(str(c.root),'sp01-loan-review',history_reader=t.verified_history());act=next(x for x in t.read_actions(str(c.root)) if x['id']=='sp01-loan-review')
  rows.append({'case':case,'allow':t.assess(snap,act)['allow'],'candidateCurrent':snap['candidateCurrent'],'entryEligible':snap['entryEligible'],'missing':snap['missingEvidence']})
 finally:c.doCleanups()
output={'candidateHash':freeze['candidateHash'],'productionImport':r.__file__,'frozenFilesVerified':True,'pinnedSourcesVerified':True,'suppliedTests':{'run':result.testsRun,'failures':len(result.failures),'errors':len(result.errors)},'resourceFixture':'Synthetic/copy fixture isolates evidence predicates; allow is not live admission.','cases':rows}
(S/'gpt6-binding-probes.json').write_text(json.dumps(output,indent=2)+'\n');print(json.dumps(output,indent=2))
