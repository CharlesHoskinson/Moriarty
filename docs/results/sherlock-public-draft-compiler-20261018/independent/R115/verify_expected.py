"""Static consistency of handwritten expectation tables, never subject execution."""
from pathlib import Path
import json,re,hashlib
P=Path(__file__).resolve().parent
v=json.loads((P/'EXPECTED.json').read_text());assert set(v)=={'chain_total','branch_total'}
fields={'entry','expressions','occurrences','occurrence_paths','children','functions','parameters','arms','patterns','constants','constant_fields','names'}
counts={}
for case,filename,n,paths in [('chain_total','chain-total.mori',13,21),('branch_total','branch-total.mori',17,35)]:
 e=v[case];assert set(e)=={'source_digest','source','preparation','program_view'}
 assert e['source_digest']==e['source']['source_digest']==hashlib.sha256((P/'inputs'/filename).read_bytes()).hexdigest()
 view=e['program_view'];prep=e['preparation'];assert set(view)==fields and view['entry']=='0'
 assert len(view['expressions'])==len(view['occurrences'])==len(prep['provenance']['expressions'])==n
 assert len(view['occurrence_paths'])==paths
 assert view['names']==sorted(set(view['names']),key=lambda s:s.encode())
 assert len(view['functions'])==len(view['parameters'])==len(prep['functions'])==len(prep['types']['function_returns'])==1
 assert prep['types']=={'function_returns':['UInt'],'entry_type':'UInt'}
 assert view['constant_fields']==[]
 def deref_slice(s,table):
  assert set(s)=={'start','len'}
  start=int(s['start']);length=int(s['len']);assert start+length<=len(table)
  if length==0:assert start==0
  return table[start:start+length]
 for i,(row,occ,source) in enumerate(zip(view['expressions'],view['occurrences'],prep['provenance']['expressions'])):
  assert set(occ)=={'owner_is_function','function','path'} and occ['function']=='0'
  path=deref_slice(occ['path'],view['occurrence_paths'])
  assert path==source['source']['child_path']
  assert occ['owner_is_function']==(source['source']['function_index'] is not None)
  tag,payload=next(iter(row.items()));assert len(row)==1;children=[]
  if tag=='Literal':assert 0<=int(payload['constant'])<len(view['constants'])
  elif tag=='Var':assert 0<=int(payload['binder'])<len(view['names'])
  elif tag=='Binary':children=[payload['left'],payload['right']]
  elif tag=='Unary':children=[payload['operand']]
  elif tag=='Call':
   assert 0<=int(payload['function_name'])<len(view['names']);children=deref_slice(payload['args'],view['children'])
  elif tag=='Match':
   children=[payload['scrutinee']]+[arm['body'] for arm in deref_slice(payload['arms'],view['arms'])]
  else:raise AssertionError(tag)
  for pos,child in enumerate(children):
   child=int(child);assert i<child<n
   target=view['occurrences'][child]
   assert target['owner_is_function']==occ['owner_is_function'] and target['function']==occ['function']
   assert deref_slice(target['path'],view['occurrence_paths'])==path+[str(pos)]
 for f in view['functions']:
  assert view['occurrences'][int(f['body'])]['owner_is_function']
  assert view['occurrences'][int(f['body'])]['path']=={'start':'0','len':'0'}
  assert len(deref_slice(f['params'],view['parameters']))==1
 for arm in view['arms']:
  assert set(arm)=={'pattern','body'} and 0<=int(arm['pattern'])<len(view['patterns']) and 0<=int(arm['body'])<n
 for row in view['patterns']:
  if row=='PNil':continue
  tag,payload=next(iter(row.items()));assert tag in ['PLeaf','PBranch','PCons']
  for name in payload.values():assert 0<=int(name)<len(view['names'])
 for constant in view['constants']:
  tag,payload=next(iter(constant.items()));assert payload['marker_free'] is True
  if tag=='U128':assert re.fullmatch(r'0|[1-9][0-9]*',payload['n']) and int(payload['n'])<1<<128
  else:assert tag=='Nil' and set(payload)=={'marker_free'}
 counts[case]={'expressions':n,'path_slots':paths,'all_twelve_fields_present':True,'source_type_owner_occurrence_join':'exact all rows','static_field_gaps':0}
report={'status':'PASS_HANDWRITTEN_EXPECTATION_STATIC_REFERENCES_AND_JOINS','cases':counts,'subject_execution':False,'limits':'Static data consistency and independently derived math only; no observed parser/checker/bridge/admission/compiler/runtime result'}
(P/'STATIC-CHECKS.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report))
