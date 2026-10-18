#!/usr/bin/env python3
"""Inspect manually authored literal data and fixed source only.
No parser, compiler, owner, admission, runtime or producer imports/execution.
This checks the listed source predicates; it does not run actual admission.
"""
import hashlib
import json
from pathlib import Path
P=Path(__file__).resolve().parent
OLD=P.parent/'152-financial-observation-independent-expectations'

def load(base,name):return json.loads((base/name).read_text())
def same(x,y):
    assert type(x) is type(y)
    if isinstance(x,dict):
        assert x.keys()==y.keys()
        for k in x:same(x[k],y[k])
    elif isinstance(x,list):
        assert len(x)==len(y)
        for a,b in zip(x,y):same(a,b)
    else:assert x==y

def without_identity_values(value):
    if isinstance(value,dict):
        return {k:('BOUND-'+k if k in ('program_hash','plan_hash') else without_identity_values(v)) for k,v in value.items()}
    if isinstance(value,list):return [without_identity_values(v) for v in value]
    return value

def snapshots(value):
    out=[]
    if isinstance(value,dict):
        if set(value)=={'state','balances','liabilities'}:out.append(value)
        for v in value.values():out.extend(snapshots(v))
    elif isinstance(value,list):
        for v in value:out.extend(snapshots(v))
    return out

def checks():
    old_source=(OLD/'inputs/retained-payment.mori').read_bytes()
    source=(P/'inputs/retained-payment.mori').read_bytes()
    addition=b'''  action declareDuties() {
    liability("auditDuty", AUX, "observer", "fee_recipient", 0);
    liability("pay-beneficiary", UNIT, "reserve", "beneficiary", 0);
  }
'''
    assert old_source.endswith(b'}\n')
    assert source==old_source[:-2]+addition+b'}\n'
    assert b'liability(' not in old_source
    new=load(P,'EXPECTED.json');old=load(OLD,'EXPECTED.json')
    prog=new['program_context']['program'];prior=old['program_context']['program']
    assert len(prog['actions'])==2 and len(prog['expressions'])==9
    assert prog['actions'][0]==prior['actions'][0]
    assert prog['expressions'][:7]==prior['expressions']
    for field in ('language','semantics','name','states','assets'):
        same(prog[field],prior[field])
    assert prog['expressions'][7:]==[{'kind':'Literal','value':{'type':'UInt','value':'0'}}]*2
    ids={a['name']:a['identity'] for a in prog['assets']}
    # Source-wide entries are explicit in the fixed source; the two existing
    # unchanged production registries scan all actions using these exact fields.
    entries=[]
    for action in prog['actions']:
        for stmt in action['statements']:
            if stmt['kind']=='Liability':
                entries.append((stmt['name'],(ids[stmt['asset']],stmt['debtor'],stmt['creditor'])))
    assert [x[0] for x in entries]==['auditDuty','pay-beneficiary']
    registry={}
    for name,identity in entries:
        assert name not in registry or registry[name]==identity
        registry[name]=identity
    assert registry=={
      'auditDuty':(ids['AUX'],'observer','fee_recipient'),
      'pay-beneficiary':(ids['UNIT'],'reserve','beneficiary')}
    def asset_key(asset):return tuple(asset[k] for k in ['network','issuer','reference','scale'])
    # Complete fixed canonical boundary predicates on every expected Snapshot,
    # not normalizing/reordering any actual value.
    all_snaps=snapshots(new)+snapshots(load(P,'EXPECTED-STATE-PROJECTIONS.json'))
    assert all_snaps
    for snap in all_snaps:
        assert list(snap['state'])==['flag','marker']
        assert snap['state']=={'flag':{'type':'Bool','value':True},'marker':{'type':'UInt','value':'77'}}
        assert len(snap['balances'])==6 and len(snap['liabilities'])==2
        keys=[(asset_key(b['asset']),b['account']) for b in snap['balances']]
        assert keys==sorted(keys) and len(set(keys))==len(keys)
        assert [b['amount'] for b in snap['balances'][:2]]==['13','9']
        assert snap['balances'][4]['amount']=='11'
        names=[l['name'] for l in snap['liabilities']]
        assert names==['auditDuty','pay-beneficiary'] and names==sorted(names)
        assert snap['liabilities'][0]['amount']=='5'
        for row in snap['balances']:
            assert row['asset'] in ids.values()
            assert row['amount'].isdigit() and 0<=int(row['amount'])<(1<<128)
        for row in snap['liabilities']:
            assert registry[row['name']]==(row['asset'],row['debtor'],row['creditor'])
            assert row['amount'].isdigit() and 0<=int(row['amount'])<(1<<128)
    pay=prog['actions'][0]
    assert pay['name']=='pay' and pay['inputs']==[]
    for stmt in pay['statements']:
        if stmt['kind'] in ('Transfer','Fee'):
            for party in ('from','to'):
                assert any(b['account']==stmt[party] and b['asset']==ids[stmt['asset']] for b in new['initial']['balances'])
    # Fixed-source static domains and selected action trace-cell predicate.
    assert len(prog['states'])+len(prog['assets'])+len(prog['actions'])==6<=256
    assert len(prog['expressions'])==9<=8192
    assert [len(a['statements']) for a in prog['actions']]==[7,2]
    assert all(len(a['statements'])<=1024 for a in prog['actions'])
    assert all(a['identity']['scale']==0 for a in prog['assets'])
    assert len(new['initial']['balances'])<=4096 and len(new['initial']['liabilities'])<=1024
    assert 11*(7+2)==99<=65536
    plan=new['program_context']['plan'];oldplan=old['program_context']['plan']
    same(plan['actions'][0],oldplan['actions'][0])
    assert plan['actions'][1]=={'action':'declareDuties','statement_count':2,'sites':[],
        'segment_cost':['4'],'committed_before':['0'],'plan_total':'4'}
    assert len(plan['actions'])==2
    site=plan['actions'][0]['sites'][0]
    assert site=={'site':1,'ordinal':3,'label':'release','binders':[{'name':'ready','ty':'Bool'}],'captures':[]}
    # Source aliases preserve exact scenario+event fields. Amount/work equations
    # are rechecked explicitly; retaining old literal cells is not itself proof.
    assert new['accounting_cross_checks']['first_reserve']==str(100-30-2)
    assert new['accounting_cross_checks']['first_liability']==str(40-30)
    assert new['accounting_cross_checks']['first_gross_debit']==str(30+2)
    assert new['accounting_cross_checks']['final_reserve']==str(100-30-2-10-1)
    assert new['accounting_cross_checks']['cumulative_gross_debit']==str(30+2+10+1)
    assert [int(new['work'][k]['attempted']) for k in ('start','false_await','true_resume')]==[2+2+2+1,1+1,2+2+2+2]
    assert int(new['work']['final_committed_total'])==7+8
    assert int(new['work']['chosen_invocation_attempt_total'])==7+2+8
    # Complete retained domain comparison, apart from explicitly replaced identities.
    retained=['initial','committed_first_prefix','final_complete','published_source_steps','observations','work',
      'local_failure_observation','accounting_cross_checks','selected_positive_expected_gaps','local_failure_input_gaps']
    for field in retained:same(without_identity_values(new[field]),without_identity_values(old[field]))
    byte_identical=['EXPECTED-WORK.json','EXPECTED-STATE-PROJECTIONS.json']
    for phase in ('START','FALSE-AWAIT','TRUE-RESUME'):
        for kind in ('ACCOUNTING','TENTATIVE-PUBLICATION'):
            byte_identical.append('EXPECTED-'+phase+'-'+kind+'.json')
        same(without_identity_values(load(P,'EXPECTED-'+phase+'-RESULT.json')),
             without_identity_values(load(OLD,'EXPECTED-'+phase+'-RESULT.json')))
    for filename in byte_identical:assert (P/filename).read_bytes()==(OLD/filename).read_bytes()
    identities=new['program_context']
    assert identities['program_hash']!=old['program_context']['program_hash']
    assert identities['plan_hash']!=old['program_context']['plan_hash']
    for label in ('start','false_await'):
        c=new['observations'][label]['result']['Ok']['Waiting']['continuation']
        assert c['program_hash']==identities['program_hash'] and c['plan_hash']==identities['plan_hash']
    return {'status':'SOURCE_PREDICATES_CHECKED_NOT_ACTUAL_ADMISSION_EXECUTED',
      'source_delta':'Only appended unselected declareDuties with two exact identity entries; pay source prefix byte-identical',
      'sourcewide_registry_entries_checked':len(registry),'expected_snapshot_occurrences_checked':len(all_snaps),
      'selected_trace_cells':'99 <= 65536','fixed_static_declarations':6,'fixed_expression_rows':9,
      'source_plan_actions':2,'selected_pay_action':'unchanged complete rows, plan, work, state/effects and ordering apart from identity bindings',
      'retained_complete_EXPECTED_subtrees':retained,'byte_identical_financial_work_fixture_files':byte_identical,
      'not_executed':['production parser/compiler/admission','stage::load/start/resume','financial projection','Machine/model/evaluator','SDK/native/provider']}

if __name__=='__main__':
    result=checks()
    (P/'INPUT-PREMISE-AND-RETENTION-CHECKS.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result,indent=2))
