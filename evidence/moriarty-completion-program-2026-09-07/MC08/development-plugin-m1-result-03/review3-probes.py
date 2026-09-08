import sys, json, hashlib, tempfile
from pathlib import Path
S=Path('/home/charl/.local/state/moriarty/moriarty-dev-plugin-20260908/successor-admission-01')
W=Path('/home/charl/Moriarty/.worktrees/moriarty-dev-plugin-grok')
sys.dont_write_bytecode=True
sys.path[:0]=[str(S/'candidate/plugins/moriarty-dev/scripts'),str(W/'plugins/moriarty-dev/tests')]
import test_records as t

def repo():
    temp=tempfile.TemporaryDirectory(prefix='review3-',dir=S)
    root=Path(temp.name); t.git_init(root)
    return temp,root
t.make_repo=repo
t.MARKER=S/'review3-command-must-not-run'
rows=[]
for case in ['actual-master-positive','publication-rewritten','publication-stale-control','prerequisite-positive','prerequisite-no-proof','prerequisite-blocked-control','review-unknown-verdict','resource-unknown-field','master-authority-wrong','historical-exact','historical-dot-alias','missing-charges','pass-scope-wrong','pass-stopped-control']:
    c=t.GenuineRegisters();c.setUp()
    try:
        c._install_funded_loan_review()
        account=c.root/t.CURRENT_ACCOUNTING_REL
        account.write_bytes(t.MASTER_BUDGET_SRC.read_bytes())
        pb=c.root/'.moriarty-dev/pass-binding.json';pb.write_bytes(t.PASS_BINDING_SRC.read_bytes())
        pl=c.root/'.moriarty-dev/pass-ledger.json';pl.write_bytes(t.PASS_LEDGER_SRC.read_bytes())
        t.attach_current_accounting(c.root,t.load_json(c.campaign_path),extra={'passBinding':'.moriarty-dev/pass-binding.json','passLedger':'.moriarty-dev/pass-ledger.json'})
        camp=t.load_json(c.campaign_path);loan=camp['campaigns']['sp01-loan-swap-grok-01']
        detail={}
        if case.startswith('publication-'):
            ap=c.root/loan['acceptance'];a=t.load_json(ap);p=c.root/a['publishedControlPath']
            p.write_text('arbitrary replacement: all financial requirements accepted\n')
            if case=='publication-rewritten':
                a['publishedControlSha256']=hashlib.sha256(p.read_bytes()).hexdigest();t.dump_json(ap,a)
            detail={'changedPath':a['publishedControlPath'],'candidateHashUnchanged':loan['candidateHash']}
        if case.startswith('prerequisite-'):
            program=t.load_json(c.program_path);t.stage_by_id(program,'rp01-mc02')['requires']=['atomic-accept'];t.dump_json(c.program_path,program)
            sprints=t.load_json(c.sprints_path)
            for sprint in sprints['sprints']:
                for gate in sprint.get('entryGates',[]):
                    if 'SP01.6' in gate.get('tasks',[]):gate['requires']=['atomic-accept']
            t.dump_json(c.sprints_path,sprints)
            st=t.stage_by_id(program,'atomic-prepare');cr=camp['campaigns'][st['campaignRecordId']]
            if case=='prerequisite-no-proof':
                cr.clear();cr.update(candidateHash=st['candidateHash'],status='complete')
                detail={'prerequisiteCampaign':st['campaignRecordId'],'retained':cr}
            if case=='prerequisite-blocked-control':cr['status']='blocked'
            t.dump_json(c.campaign_path,camp)
        if case=='review-unknown-verdict':
            p=c.root/loan['review'];d=t.load_json(p);d['verdict']='NOT_REVIEWED';t.dump_json(p,d)
        if case=='resource-unknown-field':
            p=c.root/loan['resourceAmendment'];d=t.load_json(p);d['unknown_permission']=True;t.dump_json(p,d)
        if case=='master-authority-wrong':
            d=t.load_json(account);d['authority_sha256']='0'*64;t.dump_json(account,d)
        if case=='missing-charges':
            d=t.load_json(account)
            for key in ['charges','reserved','externalPackageCharges']:d.pop(key,None)
            t.dump_json(account,d)
        if case.startswith('historical-'):
            p=c.root/t.BUDGET_REL;p.write_bytes(account.read_bytes())
            loan['currentAccounting']['path']=('./' if case.endswith('dot-alias') else '')+t.BUDGET_REL;t.dump_json(c.campaign_path,camp)
        if case=='pass-scope-wrong':
            d=t.load_json(pb);d['scope']='Unrelated service only';d['worktree']='/unrelated';d['parentCandidate']='0'*64;t.dump_json(pb,d)
        if case=='pass-stopped-control':
            d=t.load_json(pl);d['stopped']=True;t.dump_json(pl,d)
        snap=t.load_snapshot(str(c.root),'sp01-loan-review',history_reader=t.verified_history())
        act=next(x for x in t.read_actions(str(c.root)) if x['id']=='sp01-loan-review')
        rows.append(dict(case=case,allow=t.assess(snap,act)['allow'],candidateCurrent=snap['candidateCurrent'],entryEligible=snap['entryEligible'],resourceAdmitted=snap['resourceAdmitted'],missing=snap['missingEvidence'],**detail))
    finally:c.doCleanups()
(S/'review3-probes.json').write_text(json.dumps(rows,indent=2)+'\n')
print(json.dumps(rows,indent=2))
