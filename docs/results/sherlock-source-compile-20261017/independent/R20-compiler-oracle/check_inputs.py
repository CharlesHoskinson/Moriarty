"""Positive fixed-input schema/typed-width/table/trace arithmetic checks only."""
from pathlib import Path
import json,re,hashlib
P=Path(__file__).resolve().parent

def read(n):return json.loads((P/n).read_text())
def validate(value,schema,path='$'):
 if 'oneOf' in schema:
  matches=0
  for branch in schema['oneOf']:
   try:validate(value,branch,path);matches+=1
   except AssertionError:pass
  assert matches==1,(path,'oneOf',matches)
 if 'const' in schema:assert value==schema['const'] and type(value)is type(schema['const']),(path,'const')
 if 'type' in schema:
  ty=schema['type'];assert {'object':dict,'array':list,'string':str,'null':type(None)}[ty] is type(value),(path,'type')
 if type(value)is dict:
  assert all(k in value for k in schema.get('required',[])),(path,'required')
  if schema.get('additionalProperties') is False:assert set(value)<=set(schema.get('properties',{})),(path,'additionalProperties')
  for k,v in value.items():
   if k in schema.get('properties',{}):validate(v,schema['properties'][k],path+'.'+k)
 if type(value)is list:
  assert len(value)>=schema.get('minItems',0) and len(value)<=schema.get('maxItems',len(value)),(path,'array length')
  for i,v in enumerate(value):
   if 'items' in schema:validate(v,schema['items'],path+'['+str(i)+']')
 if type(value)is str and 'pattern' in schema:assert re.fullmatch(schema['pattern'],value),(path,'pattern')
config=read('inputs/execution-config.json');validate(config,read('contracts/execution-config.schema.json'))
for group in ['bounds','host']:
 for k,v in config[group].items():
  assert re.fullmatch(r'0|[1-9][0-9]*',v)
  limit=(1<<128)-1 if k in ['max_reference_work','max_helper_work','step_cap'] else (1<<32)-1
  assert 0<=int(v)<=limit,(group,k,'declared width')
for k,v in config['layout']['capacities'].items():assert 0<=int(v)<(1<<32)
assert all(int(x)>0 for x in config['host'].values())
assert int(config['fuel']['value'])<=int(config['bounds']['max_reference_work'])
assert int(config['fuel']['value'])<=int(config['layout']['capacities']['value_slots'])
assert int(config['stack']['value'])<=int(config['bounds']['frames'])
contract=(P/'contracts/FIRST-COMPILER-BATCH.md').read_text();blocks=re.findall(r'```mori\n(.*?)\n```',contract,re.S)
assert len(blocks)==2
for name,block in zip(['fixed-quote.mori','fixed-eligibility.mori'],blocks):assert (P/'inputs'/name).read_text()==block+'\n',name
rows=read('normative/03-rows-supplement.json');helper={x['id']:x for x in rows['helper_events']};legacy={x['id']:x for x in rows['legacy_completion_instances']}
results={}
for label,viewfile in [('QUOTE','EXPECTED-QUOTE-PROGRAM-VIEW.json'),('ELIGIBILITY','EXPECTED-ELIGIBILITY-ALPHA-PROGRAM-VIEW.json')]:
 view=read(viewfile);trace=read('EXPECTED-'+label+'-B-EVENT-WORK-PROJECTION.json');run=read('EXPECTED-'+label+'-NORMAL-RUN.json');events=trace['events_projection'];totals=trace['totals']
 assert len(view['expressions'])==len(view['occurrences'])
 assert set(view)=={'entry','expressions','occurrences','occurrence_paths','children','functions','parameters','arms','patterns','constants','constant_fields','names'}
 assert view['names']==sorted(set(view['names']),key=lambda x:x.encode())
 for i,row in enumerate(view['expressions']):
  tag,payload=next(iter(row.items()))
  children=[payload[k] for k in ['value','body','left','right','operand','cond','then_expr','else_expr'] if k in payload]
  assert all(i<int(child)<len(view['expressions']) for child in children)
  if 'binder' in payload:assert int(payload['binder'])<len(view['names'])
 for e in events:
  rule=e['catalog_rule']
  if rule.startswith('b.helper.'):
   normative=helper[rule];assert normative['status']=='Enabled';assert e['helper_visits_delta']==normative['helperVisits'];assert e['work_delta']==normative['deltas']['base'][1] and e['helper_work_delta']==normative['deltas']['base'][2]
  elif rule.startswith('k.machine.'):
   normative=legacy[rule];assert e['work_delta']==normative['deltas']['base'][1] and e['reference_cursor_delta']==normative['deltas']['base'][4]
  else:assert rule=='k.admin.publish.value' and e['work_delta']==e['reference_cursor_delta']==0
 assert sum(e['work_delta'] for e in events)==totals['consumed_work']
 assert sum(e['helper_work_delta'] for e in events)==totals['helper_work']
 assert sum(e['helper_visits_delta'] for e in events)==totals['helper_visits']
 assert sum(e['reference_cursor_delta'] for e in events)==totals['reference_cursor']
 assert totals['consumed_work']==totals['helper_work']+totals['reference_cursor']
 assert len(events)==totals['consumed_work']+1
 assert events[-1]['boundary']=='Terminal' and events[-1]['outcome_tag']=='Halted'
 assert all(e['boundary']=='None' for e in events[:-1])
 assert len(trace['prefix_counters'])==len(events)+1
 assert sum(int(e['alloc_deltas']['data_nodes']) for e in events)==2
 assert sum(int(e['alloc_deltas']['environment_cells']) for e in events)==len(run['final_image_active_projection']['env'])
 assert int(run['final_image_active_projection']['remaining_work'])+totals['consumed_work']==1000
 assert len(view['expressions'])<=int(config['bounds']['code_rows'])
 assert len(view['occurrence_paths'])<=int(config['layout']['capacities']['occurrence_path_slots'])
 assert len(view['constants'])<=int(config['bounds']['constant_nodes'])
 assert len(view['names'])<=int(config['layout']['capacities']['name_rows'])
 results[label]={'schema_and_structural_checks':'PASS','event_projection_count':len(events),'normal_B_work':totals['consumed_work'],'helper_work':totals['helper_work'],'legacy_cursor':totals['reference_cursor']}
report={'status':'PASS_POSITIVE_INPUT_SCHEMA_TYPED_WIDTH_LITERAL_TABLE_AND_MANUAL_PROJECTION_CHECKS','checks':results,'config_bytes':len((P/'inputs/execution-config.json').read_bytes()),'config_sha256':hashlib.sha256((P/'inputs/execution-config.json').read_bytes()).hexdigest(),'sources_exactly_match_final_contract_specimens':True,'all_usize_inputs_fit_32_and_64_bit':True,'no_interpreter_compiler_transition_or_model_executed':True,'unresolved':'RawEligibilityfreshnamebyteextent, targetdescriptor, fullperprefiximages/peripheralStepEventfields and report metadata remain declared limits; these checks are not runtime acceptance.'}
(P/'INPUT-CHECKS.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report))
