import sys,json,hashlib,tempfile,importlib.util
from pathlib import Path
sys.dont_write_bytecode=True
S=Path(__file__).resolve().parent
W=Path('/home/charl/Moriarty/.worktrees/moriarty-dev-plugin-grok')
sys.path.insert(0,str(S/'candidate/plugins/moriarty-dev/scripts'))
spec=importlib.util.spec_from_file_location('fixture',S/'candidate/plugins/moriarty-dev/tests/test_records.py')
t=importlib.util.module_from_spec(spec);spec.loader.exec_module(t)
old=t.SOURCE_ROOT
for key,value in list(vars(t).items()):
    if isinstance(value,Path) and value.is_relative_to(old):setattr(t,key,W/value.relative_to(old))
def repo():
    temp=tempfile.TemporaryDirectory(prefix='gpt6-probe-',dir=S)
    root=Path(temp.name);t.git_init(root);return temp,root
t.make_repo=repo;t.MARKER=S/'gpt6-no-child-marker'
rows=[]
for case in ['genuine-loan-positive','genuine-atomic-unsupported','loan-as-atomic-prerequisites','loan-as-finalized-prerequisites','missing-prerequisite-review','prerequisite-cycle','publication-added-approval-fields','publication-existing-scope-escalation','publication-closure-escalation','acceptance-unknown-complete-status','acceptance-REVOKED-status','review-unknown-verdict']:
    c=t.GenuineRegisters();c.setUp()
    try:
        c._install_funded_loan_review()
        campaigns=t.load_json(c.campaign_path);loan=campaigns['campaigns']['sp01-loan-swap-grok-01'];details={}
        if case in ['genuine-atomic-unsupported','loan-as-atomic-prerequisites','loan-as-finalized-prerequisites','missing-prerequisite-review','prerequisite-cycle']:
            program=t.require_atomic_prereq(c.program_path,c.sprints_path)
            if case!='genuine-atomic-unsupported':
                t.overlay_supported_prereq(campaigns,program,loan)
                if case=='loan-as-finalized-prerequisites':
                    for sid in ('atomic-prepare','atomic-accept'):
                        st=t.stage_by_id(program,sid);st['acceptedProfile']='finalized-financial-settlement';campaigns['campaigns'][st['campaignRecordId']]['acceptedProfile']=st['acceptedProfile']
                if case=='missing-prerequisite-review':(c.root/loan['review']).unlink()
                if case=='prerequisite-cycle':t.stage_by_id(program,'atomic-prepare')['requires']=['atomic-accept']
                details={'bindingStage':t.load_json(c.root/loan['binding'])['stage'],'reviewScope':t.load_json(W/loan['review'])['scope'],'acceptanceStatus':t.load_json(c.root/loan['acceptance'])['status'],'prerequisiteStages':['atomic-accept','atomic-prepare'],'profile':t.stage_by_id(program,'atomic-accept')['acceptedProfile']}
            t.dump_json(c.program_path,program);t.dump_json(c.campaign_path,campaigns)
        ap=c.root/loan['acceptance'];acc=t.load_json(ap)
        if case.startswith('publication-'):
            pp=c.root/acc['publishedControlPath'];control=t.load_json(pp);subset=control['subsets']['RP01-MC02']
            if case=='publication-added-approval-fields':
                subset['acceptance']['allSprintsAccepted']=True;subset['acceptance']['nativeProofsAccepted']=True
            elif case=='publication-existing-scope-escalation':
                subset['acceptance']['scope']='Full RP01 native proof and financial Preview acceptance'
                subset['acceptance']['networkMilestones']='complete; all SP05 financial settlement gates accepted'
                subset['acceptance']['staticVerification']='missing-forged-verification.json'
            else:
                for row in subset['rowSummaries'].values():row['closureTask']='All financial settlement, native proof and corpus acceptance obligations are closed.'
            t.dump_json(pp,control);acc['publishedControlSha256']=t.sha256_file(pp);t.dump_json(ap,acc)
        if case=='acceptance-unknown-complete-status':acc['status']='complete-NOT_REVIEWED';t.dump_json(ap,acc)
        if case=='acceptance-REVOKED-status':acc['status']='complete-but-REVOKED';t.dump_json(ap,acc)
        if case=='review-unknown-verdict':
            p=c.root/loan['review'];r=t.load_json(p);r['verdict']='NOT_REVIEWED';t.dump_json(p,r)
        snap=t.load_snapshot(str(c.root),'sp01-loan-review',history_reader=t.verified_history())
        act=next(x for x in t.read_actions(str(c.root)) if x['id']=='sp01-loan-review')
        rows.append(dict(case=case,allow=t.assess(snap,act)['allow'],candidateCurrent=snap['candidateCurrent'],entryEligible=snap['entryEligible'],missing=snap['missingEvidence'],**details))
    finally:c.doCleanups()
(S/'gpt6-evidence-probes.json').write_text(json.dumps(rows,indent=2)+'\n')
print(json.dumps(rows,indent=2))
