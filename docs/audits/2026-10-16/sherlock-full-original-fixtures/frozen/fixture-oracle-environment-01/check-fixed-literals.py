"""Ordinary literal integrity checks only. No subject evaluator is imported/run."""
from pathlib import Path
import hashlib,json,re
R=Path(__file__).parent; W=Path('/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016')
pins=json.loads((R/'input-pins.json').read_text())
# Authoritative source pins are checked against the private copies. Original
# fixtures and all four original literal oracle files are also byte-preserved W.
preserved=[]
for row in pins['inputs']:
    data=(R/'pinned'/row['path']).read_bytes()
    assert len(data)==row['bytes']
    assert hashlib.sha256(data).hexdigest()==row['sha256']
    if row['path'].startswith(('models/sherlock/k/','models/sherlock/oracles/')):
        assert (W/row['path']).read_bytes()==data
        preserved.append(row['path'])
original_programs=[json.loads((R/'pinned/models/sherlock/oracles'/name).read_text()) for name in ['normal-programs.json','extra-normal-programs.json']]
original_traces=[json.loads((R/'pinned/models/sherlock/oracles'/name).read_text()) for name in ['normal-prefixes.json','extra-normal-prefixes.json']]
assert sum(len(d['programs']) for d in original_programs)==8
assert sum(len(t['ordinary_fuel_prefixes']) for d in original_traces for t in d['traces'])==216
pdoc=json.loads((R/'fixture-normal-programs.json').read_text()); tdoc=json.loads((R/'fixture-normal-prefixes.json').read_text())
assert pdoc['profile']==tdoc['profile']=='sherlock-k-a/1'
assert pdoc['source']==tdoc['source']
assert pdoc['baseline_commit']==tdoc['baseline_commit']=='c9f7cf6a712c7e6069cdc7a3ecdd86fb79817429'
assert [p['case_id'] for p in pdoc['programs']]==['F06bool','F07caller','F08mutual']
assert [t['case_id'] for t in tdoc['traces']]==['F06bool','F07caller','F08mutual']
main=(R/'pinned/models/sherlock/k/main.k').read_text().splitlines()
used={74,80,84}; summaries=[]
for p,t,n,maxdepth,expected in zip(pdoc['programs'],tdoc['traces'],[20,21,120],[3,3,7],['uiU(7)','uiU(12)','uiB(true)']):
    fields={k:p[k] for k in ('case_id','code_k','entry_k','initial_arguments','stack_limit','exact_completion_fuel')}
    digest=hashlib.sha256(json.dumps(fields,sort_keys=True,separators=(',',':')).encode()).hexdigest()
    assert digest==p['input_sha256']==t['input_sha256']==t['fixed_state_fields']['code_ref']['input_sha256']
    assert t['fixed_state_fields']=={'code_ref':{'file':'fixture-normal-programs.json','case_id':p['case_id'],'input_sha256':digest},'initial_fuel':n,'stack_limit':64,'output_k':'uiPending()'}
    assert p['exact_completion_fuel']==n and p['expected_maximum_depth']==maxdepth and p['expected_value_k']==expected
    fixture=(R/'pinned'/p['original_fixture']['path']).read_text()
    source_run=f"uiRun({p['code_k']}, {p['entry_k']}, 1000, 64)"
    assert re.sub(r'\s+','',source_run)==re.sub(r'\s+','',fixture)
    assert t['initial_configuration']=={'control_k':f"uiRun({p['code_k']}, {p['entry_k']}, {n}, 64)",'environment_k':'.Map','frames_top_first_k':[],'depth':0,'stack_limit':0,'code_k':'uiNoFuns()','remaining_fuel':0,'initial_fuel':0,'output_k':'uiPending()','charged_steps':0}
    assert t['startup']=={'rule_start_line':74,'charge':0,'post_state_index':0}
    states=t['states']; assert len(states)==n+1
    assert [s['charged_steps'] for s in states]==list(range(n+1))
    assert [s['remaining_fuel'] for s in states]==list(range(n,-1,-1))
    assert max(s['depth'] for s in states)==maxdepth
    assert states[0]['control_k']==f"uiEval({p['entry_k']})" and states[0]['via_rule_start_line'] is None
    assert states[-1]['control_k']==f'uiRet({expected})' and states[-1]['environment_k']=='.Map' and states[-1]['frames_top_first_k']==[]
    for s in states:
        assert s['depth']==len(s['frames_top_first_k'])<=64
        for term in [s['control_k'],s['environment_k'],*s['frames_top_first_k']]:
            assert '{' not in term and '}' not in term
            balance=0
            for c in term:
                balance += (c=='(')-(c==')'); assert balance>=0
            assert balance==0
        if s['via_rule_start_line'] is not None:
            used.add(s['via_rule_start_line']);assert main[s['via_rule_start_line']-1].lstrip().startswith('rule <k>')
    assert t['publication']=={'pre_state_index':n,'rule_start_line':80,'charge':0,'post_overrides':{'control_k':'.K','output_k':f'uiDone({expected})'}}
    prefixes=t['ordinary_fuel_prefixes']; assert [row[0] for row in prefixes]==list(range(n+2))
    assert prefixes[:-2]==[[f,f,0,'uiExhausted()',84] for f in range(n)]
    assert prefixes[-2:]==[[n,n,0,f'uiDone({expected})',80],[n+1,n,1,f'uiDone({expected})',80]]
    assert t['original_fixture_run']['remaining_fuel']==1000-n and t['original_fixture_run']['consumed_work']==n
    summaries.append({'case_id':p['case_id'],'exact_completion_fuel':n,'literal_states':len(states),'ordinary_fuel_prefixes':len(prefixes),'maximum_depth':maxdepth,'original_fixture_remaining_fuel':1000-n,'input_sha256':digest})
# Distinctive predeclared environment checkpoints, independently written as
# literal assertions; these do not evaluate any main.k transition.
a,b,c=tdoc['traces']
assert a['states'][8]['depth']==3 and a['states'][19]['frames_top_first_k']==['uiScopeF(.Map)']
assert b['states'][13]['environment_k']=='0 |-> uiU(10)' and b['states'][14]['environment_k']=='0 |-> uiU(2)' and b['states'][16]['environment_k']=='0 |-> uiU(10)'
assert c['states'][104]['frames_top_first_k']==['uiReturnF(uiBoolT(), 0 |-> uiU(1))','uiReturnF(uiBoolT(), 0 |-> uiU(2))','uiReturnF(uiBoolT(), 0 |-> uiU(3))','uiReturnF(uiBoolT(), 0 |-> uiU(4))','uiReturnF(uiBoolT(), .Map)']
assert [c['states'][i]['environment_k'] for i in [116,117,118,119,120]]==['0 |-> uiU(1)','0 |-> uiU(2)','0 |-> uiU(3)','0 |-> uiU(4)','.Map']
labels={'source':pdoc['source'],'profile':pdoc['profile'],'label_convention':'Exact main.k rule start line; source has no explicit rule attributes.','rules':[{'rule_start_line':i,'source_first_line':main[i-1],'charge':0 if i in {74,80,84} else 1} for i in sorted(used)]}
(R/'rule-source-labels.json').write_text(json.dumps(labels,indent=2)+'\n')
report={'status':'ordinary literal integrity assertions passed; subject execution UNRUN','cases':summaries,'original_normal_cases_preserved':8,'original_normal_prefixes_preserved':216,'total_literal_states':164,'total_ordinary_fuel_prefixes':167,'preserved_worktree_input_paths':preserved,'limitations':'Only lexical reconstruction, digest/schema/count/alias/selected fixed checkpoint assertions. No program interpretation, rule execution, K/host/circuit/Lean correspondence, security or result-review claim.'}
(R/'literal-check-report.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
