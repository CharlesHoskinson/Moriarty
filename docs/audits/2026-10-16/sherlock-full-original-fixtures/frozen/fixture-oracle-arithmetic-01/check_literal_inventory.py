"""Read-only literal/schema inventories. Does not simulate the subject or its rules."""
from pathlib import Path
from collections import Counter
import hashlib,json,datetime
R=Path(__file__).parent
W=Path('/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016')
pins=json.loads((R/'INPUT-PINS.json').read_text())
unchanged={p:hashlib.sha256((W/p).read_bytes()).hexdigest()==m['sha256'] for p,m in pins['inputs'].items()}
assert all(unchanged.values()),unchanged
p=json.loads((R/'newprograms.json').read_text()); t=json.loads((R/'prefixes.json').read_text()); o=json.loads((R/'original-fixture-runs.json').read_text())
expected_counts={131:4,135:22,102:9,181:4,200:4,204:4,208:4,123:4,117:10,108:10,149:10,153:10,223:10,165:3,161:1,185:4}
expected_lines=[131,135,102,181,200,204,208,135]+[123,135,117,135,108,149,102,153,223,165,117,135,108,149,131,135,117,135,108,149,102,153,223,181,200,204,208,135]*3+[123,135,117,135,108,149,102,153,223,161,102,185]+[153,223,185]*3
required_state={'charged_steps','remaining_fuel','depth','control_k','environment_k','frames_top_first_k','via_rule_start_line'}
for inp,trace,orig in zip(p['programs'],t['traces'],o['traces']):
 source={k:inp[k] for k in ['case_id','code_k','entry_k','initial_arguments','stack_limit','exact_completion_fuel']}
 assert hashlib.sha256(json.dumps(source,sort_keys=True,separators=(',',':')).encode()).hexdigest()==inp['input_sha256']==trace['input_sha256']==orig['input_sha256']
 assert trace['case_id']==orig['case_id']==inp['case_id']
 assert len(trace['states'])==len(orig['states'])==114
 assert [s['via_rule_start_line'] for s in trace['states'][1:]]==expected_lines
 assert Counter(s['via_rule_start_line'] for s in trace['states'][1:])==expected_counts
 for i,(s,full) in enumerate(zip(trace['states'],orig['states'])):
  assert set(s)==required_state and set(full)==required_state
  assert s['charged_steps']==i and s['remaining_fuel']==113-i and s['depth']==len(s['frames_top_first_k'])
  assert full==dict(s,remaining_fuel=1000-i)
  assert s['control_k'].count('(')==s['control_k'].count(')')
  assert all(frame.count('(')==frame.count(')') for frame in s['frames_top_first_k'])
 assert [i for i,s in enumerate(trace['states']) if s['depth']==9]==[96,97,98,99]
 assert trace['states'][113]['control_k']=='uiRet(uiU(6))'
 assert trace['states'][113]['environment_k']=='.Map' and trace['states'][113]['frames_top_first_k']==[]
 assert len(trace['ordinary_fuel_prefixes'])==115
 for fuel,row in enumerate(trace['ordinary_fuel_prefixes']):
  assert row==([fuel,fuel,0,'uiExhausted()',84] if fuel<113 else [fuel,113,fuel-113,'uiDone(uiU(6))',80])
 assert trace['startup']=={'rule_start_line':74,'charge':0,'post_state_index':0}
 assert trace['publication']=={'pre_state_index':113,'rule_start_line':80,'charge':0,'post_overrides':{'control_k':'.K','output_k':'uiDone(uiU(6))'}}
 assert orig['states'][-1]['remaining_fuel']==887
 assert orig['fixed_state_fields']['initial_fuel']==1000 and orig['fixed_state_fields']['stack_limit']==64
 assert orig['initial_configuration']['control_k']==inp['original_fixture']['run_k']
old=[json.loads((W/'models/sherlock/oracles'/f).read_text()) for f in ['normal-prefixes.json','extra-normal-prefixes.json']]
assert sum(len(x['traces']) for x in old)==8
assert sum(len(t['ordinary_fuel_prefixes']) for x in old for t in x['traces'])==216
report={'status':'PASS: literal structural inventory only; no subject execution or independent audit','created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'inputs_unchanged_since_pin':unchanged,'existing_cases_untouched':8,'existing_prefixes_untouched':216,'new_cases':2,'new_ordinary_prefixes':230,'new_states':228,'original_fuel1000_runs':2,'charged_work_each':113,'original_remaining_fuel_each':887,'maximum_depth_each':9,'scope':'provisional source-only K-A literal expectations; no B, finance, circuit, native, correspondence, public ABI or adoption claim'}
(R/'LITERAL-INVENTORY.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
