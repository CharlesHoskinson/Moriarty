from pathlib import Path
import hashlib,json,runpy
repo=Path('/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016')
qa=runpy.run_path(str(repo/'models/sherlock/qa/check.py'))
artifact_path=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/quint-01/quint-observations.json')
artifact=json.loads(artifact_path.read_text())
rows=[]
for case,fuel,field,record,expected_index in [('N08',82,'records',1,214),('N01',2,'terminal',None,2),('N01',1,'initial',None,1)]:
 difference=dict(case_id=case,initial_fuel=fuel,path=f'/runs/{expected_index}/{field}/control_k')
 if record is not None:difference['record_index']=record
 context=qa['layer_raw_context'](artifact,difference)
 assert context['raw_trace_index']==expected_index
 assert context['raw_trace_reference']==artifact['raw_trace_references'][expected_index]
 states=artifact['raw_traces'][expected_index]['itf']['states']
 if record is not None:
  assert context['raw_before_state']==states[record] and context['raw_after_state']==states[record+1]
 else:
  index=0 if field=='initial' else 2
  assert context['raw_state']==states[index]
  if field=='terminal':assert states[3]['lastRule']=={'#bigint':'0'} and context['raw_state']!=states[3]
 rows.append(dict(case_id=case,initial_fuel=fuel,raw_trace_index=expected_index,raw_state_indices=context['raw_state_indices']))
print(json.dumps(dict(status='pass',scope='read-only raw-state locator on existing actual artifact',artifact_sha256=hashlib.sha256(artifact_path.read_bytes()).hexdigest(),cases=rows),indent=2))
