"""Static custody/schema/citation verification only; never imports or runs Moriarty."""
from pathlib import Path
import hashlib,json,re
OUT=Path(__file__).resolve().parent
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017')
ROOT=R/'durable-recovery-full-readonce-terminal-reviews-20261018'
P=ROOT/'packet'
def sha(b):return hashlib.sha256(b).hexdigest()
def load(n):return json.loads((OUT/n).read_text())
for pin in load('SOURCE-PINS.json')['pins']:
 p=Path(pin['path']);b=p.read_bytes()
 assert len(b)==pin['bytes'] and sha(b)==pin['sha256'],str(p)
primary=(P/'PRIMARY.md').read_bytes();pl=primary.decode().splitlines();body=load('BODY-CUSTODY-CHECKS.json')
assert len(primary)==500104 and sha(primary)==body['primary']['sha256']
assert len(body['checks'])==44 and sum(x['bytes'] for x in body['checks'])==488685
byid={x['id']:x for x in body['checks']}
for x in body['checks']:
 b=primary[x['primary_body_byte_start']:x['primary_body_byte_end_exclusive']]
 assert b==Path(x['original_absolute_path']).read_bytes()
 assert len(b)==x['bytes'] and sha(b)==x['sha256']
 assert primary[:x['primary_body_byte_start']].count(b'\n')+1==x['primary_first_body_line_1based']
 assert x['primary_last_body_line_1based']==x['primary_first_body_line_1based']+len(b.splitlines())-1
 assert len(re.findall(rb'^===== BEGIN MANDATORY '+x['id'].encode()+rb' \| ',primary,re.M))==1
 assert len(re.findall(rb'^===== END MANDATORY '+x['id'].encode()+rb' =====$',primary,re.M))==1
original=(ROOT/'job-b/TERMINAL-REVIEW.json').read_bytes()
assert (OUT/'UNCHANGED-TERMINAL-REVIEW.json').read_bytes()==original
assert len(original)==25735 and sha(original)=='e5ad71e3ba263feb3d60bba6b2a52d21d9b5db7574a3f4f51ae18395d8da2e1f'
review=json.loads(original);task=load('TASK-B.json');receipt=load('PUBLIC-EXPORT-RECEIPT.json')
assert (OUT/'TASK-B.json').read_bytes()==(ROOT/'TASK-B.json').read_bytes()
assert (OUT/'PUBLIC-EXPORT-RECEIPT.json').read_bytes()==(ROOT/'job-b/TERMINAL-EXPORT-RECEIPT.json').read_bytes()
assert task['manifestSha256']==sha((P/'MANIFEST.json').read_bytes())
assert task['schemaSha256']==sha((P/'WHOLE-OPINION.schema.json').read_bytes())
assert task['promptSha256']==sha((P/'PROMPT-B.md').read_bytes())
assert receipt['taskSha256']==sha((OUT/'TASK-B.json').read_bytes())
assert review['candidate']['primary_sha256']==sha(primary)
assert review['candidate']['manifest_sha256']==sha((R/'durable-recovery-full-design-decision-review-20261018/MANIFEST.json').read_bytes())
# This schema uses only the following standard assertions; metadata keywords are not assertions.
def valid(value,s):
 if 'const' in s:assert value==s['const']
 if 'enum' in s:assert value in s['enum']
 if 'type' in s:
  types=s['type'] if isinstance(s['type'],list) else [s['type']]
  predicates={'object':lambda: isinstance(value,dict),'array':lambda:isinstance(value,list),'string':lambda:isinstance(value,str),'boolean':lambda:isinstance(value,bool),'null':lambda:value is None}
  assert any(predicates[t]() for t in types)
 if isinstance(value,dict):
  assert set(s.get('required',[]))<=set(value)
  if s.get('additionalProperties') is False:assert set(value)<=set(s.get('properties',{}))
  for k,v in value.items():
   if k in s.get('properties',{}):valid(v,s['properties'][k])
 if isinstance(value,list):
  assert len(value)>=s.get('minItems',0) and len(value)<=s.get('maxItems',len(value))
  if s.get('uniqueItems'):assert len(set(json.dumps(x,sort_keys=True) for x in value))==len(value)
  for v in value:
   if 'items' in s:valid(v,s['items'])
  if 'contains' in s:
   def matches(x):
    try:valid(x,s['contains']);return True
    except AssertionError:return False
   assert sum(matches(x) for x in value)>=s.get('minContains',1)
 for a in s.get('allOf',[]):valid(value,a)
schema=json.loads((P/'WHOLE-OPINION.schema.json').read_text());valid(review,schema)
anchors=load('SOURCE-ANCHOR-CORRECTIONS.json')
for x in anchors['anchors']+anchors['supplementary_support']:
 a,z=x['primary_lines'];src=byid[x['body_id']];la,lz=x['correct_file_local_lines']
 assert la==a-src['primary_first_body_line_1based']+1 and lz==z-src['primary_first_body_line_1based']+1
 excerpt='\n'.join(pl[a-1:z])+'\n'
 assert x['literal_excerpt']==excerpt and x['excerpt_sha256']==sha(excerpt.encode())
 assert Path(src['original_absolute_path']).read_text().splitlines()[la-1:lz]==pl[a-1:z]
 assert x['source_sha256']==src['sha256']
assert len(anchors['anchors'])==41 and len(anchors['supplementary_support'])==11
occurrences=load('CITATION-OCCURRENCES.json');expected=[]
pathmap={x['path']:x['id'] for x in body['checks']}
for i,f in enumerate(review['findings']):
 for field in ['source_location','finding','consequence','requested_disposition']:
  for m in re.finditer(r'(?P<path>[A-Za-z0-9_./-]+\.(?:json|md|rs)):(?P<ranges>\d+(?:-\d+)?(?:,\d+(?:-\d+)?)*)',f[field]):
   expected.append(('/findings/'+str(i)+'/'+field,m.group(0)))
assert expected==[(x['review_json_pointer'],x['literal_citation']) for x in occurrences['occurrences']]
for x in occurrences['occurrences']:
 assert x['citation_utf8_sha256']==sha(x['literal_citation'].encode())
 for rg in x['ranges']:
  fid=review['findings'][int(x['review_json_pointer'].split('/')[2])]['id']
  assert any(a['finding_id']==fid and a['body_id']==x['body_id'] and a['review_numeric_anchor']==rg for a in anchors['anchors'])
for x in load('FINDING-FIDELITY.json')['findings']:
 f=next(y for y in review['findings'] if y['id']==x['id'])
 assert f['severity']==x['severity'] and f['kind']==x['kind']
 for k,v in x['unchanged_review_fields'].items():
  assert v['literal']==f[k] and v['utf8_sha256']==sha(f[k].encode())
  assert json.dumps(f[k],ensure_ascii=True) in original.decode().splitlines()[v['review_json_line']-1]
ret=load('SCOPE-RETENTION.json');b35=Path(byid['B35']['original_absolute_path']).read_text()
assert len(ret['original35_exact_crosswalk'])==35
for i,x in enumerate(ret['original35_exact_crosswalk'],1):
 assert x['id']==f'MPLR-{i:03}'
 m=re.findall(r'^\| \['+x['id']+r'\]\([^\n]+?\)\s*\|([^\n]+)\|$',b35,re.M)
 assert len(m)==1
 assert [v.strip() for v in m[0].split('|')]==[x['existing_title'],x['existing_phase_owner'],x['existing_consolidation'],x['original_status']]
 assert x['full_normative_body_present_in_PRIMARY'] and x['original_status']=='specified-only'
assert len(ret['full_lifecycle_EARS'])==20 and all(x['acceptance_closed'] is False for x in ret['full_lifecycle_EARS'])
assert ret['full_lifecycle_EARS']==json.loads(Path(byid['B03']['original_absolute_path']).read_text())['requirements']
assert len(ret['owner_prerequisites'])==12
assert [x['id'] for x in ret['owner_prerequisites']]==[x['id'] for x in review['owner_prerequisites']]
for x,y in zip(ret['owner_prerequisites'],review['owner_prerequisites']):
 assert x['review_exact_dependency']==y['interface_gap_or_dependency']
 assert x['mandatory_before_dependent_implementation'] and not x['rule_authored_or_adopted_by_fidelity_lane']
 assert y['mandatory_before_dependent_implementation'] and not y['security_rule_authored_here']
assert ret['review_retained_scope_exact']==review['retained_scope']
result=load('FIDELITY-RESULT.json')
assert result['actual_vote']==review['decision']
assert review['decision']['ordinary_vote']=='AGREE' and review['decision']['selected_alternative']=='A_EXPLICIT_COMPLETE_RECORD'
assert not review['decision']['blocking_ordinary_findings'] and not review['decision']['required_ordinary_changes']
assert [x['severity'] for x in review['findings']]==['minor']*3+['note']*2
assert review['reviewer']['actual_model'] is None and not receipt['actualBackendIdentityAttested']
cor=load('CUSTODY-CORRECTIONS.json')['corrections'];assert cor[0]['recorded']!=task['schemaSha256'] and cor[0]['actual_schema_sha256']==task['schemaSha256']
assert primary.endswith(b'\n') and len(pl)==8311 and primary.count(b'\n')+1==8312
assert len((P/'WHOLE-OPINION.schema.json').read_bytes().splitlines())==543
if (OUT/'INVENTORY.json').exists():
 for x in load('INVENTORY.json')['payloads']:
  b=(OUT/x['path']).read_bytes();assert len(b)==x['bytes'] and sha(b)==x['sha256']
print(json.dumps({'static_check':'PASS','complete_body_matches':44,'numeric_finding_citation_ranges':41,'literal_numeric_citation_occurrences':len(occurrences['occurrences']),'supplementary_anchors':11,'original_MPLR_rows':35,'DR_rows':20,'owner_prerequisites':12,'unchanged_B_bytes':25735,'whole_actual_schema_validation':'PASS','product_execution':False,'vote_or_adoption_created':False},sort_keys=True))
