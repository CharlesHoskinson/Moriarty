from pathlib import Path
import json,hashlib,re,subprocess,sys
v=Path('/home/charl/Moriarty-aeon-study');r=Path('/home/charl/research/aeon-moriarty-2026-09-19')
results={}
# Hash-bound same-text votes, no inferred consensus.
p=r/'review';sha=hashlib.sha256((p/'CONSENSUS-CANDIDATE-v4.md').read_bytes()).hexdigest();votes=[]
for vendor in ['astra','fable']:
 for i in [1,2,3]:
  j=json.loads((p/f'{vendor}-{i}-consensus-v4.json').read_text());q=j
  if vendor=='fable':
   assert j['is_error'] is False and j['modelUsage']['claude-fable-5-1']['canonicalModel']=='claude-fable-5-1'
   t=j['result'];q=json.loads(t[t.find('{'):t.rfind('}')+1])
  assert q['candidate_sha256']==sha and q['verdict']=='ENDORSE' and not q['blocking_changes'];assert q.get('evidence_that_would_change_vote');votes.append(f'{vendor}-{i}')
results['consensus']={'hash':sha,'endorsements':votes}
# Exact transaction byte results.
operations=[]
for f in r.glob('*result.json'):
 if f.name.startswith(('ingest-apss-','save-final-research','save-apss-navigation','ingest-certified-basis','save-diataxis-metadata','save-roadmap-delivery')):
  j=json.loads(f.read_text());assert j['status']=='complete',(f,j.get('status'));operations.append(j['operation_id'])
results['vault_operations']=operations
# Requirement coverage and scenarios.
p=v/'openspec/changes/permissionless-provable-intention';spec=(p/'specs/permissionless-intention/spec.md').read_text();trace=(p/'requirements.md').read_text();ids=re.findall(r'### Requirement: ([A-Z]+-\d+)',spec);assert len(ids)==len(set(ids));assert all(x in trace for x in ids);assert len(re.findall(r'#### Scenario:',spec))>=2*len(ids)
results['requirements']={'count':len(ids),'ids':ids,'scenarios':len(re.findall(r'#### Scenario:',spec))}
# Diataxis files and relative links excluding preserved archives and external references.
paths=list((v/'wiki/research/apss').rglob('*.md'))+[v/'docs/MORIARTY-PRODUCT-CONTRACT.md',v/'ROADMAP.md',v/'openspec/README.md']+list((v/'docs').glob('ROADMAP-RECONCILIATION-2026-09-19.md'))+list(p.rglob('*.md'));bad=[]
for f in paths:
 text=f.read_text()
 text=re.sub(r'```.*?```','',text,flags=re.S)
 for target in re.findall(r'\]\(([^)]+)\)',text):
  if re.match(r'[a-z]+:|#|/',target):continue
  t=target.split('#')[0]
  if not (f.parent/t).exists():bad.append([str(f.relative_to(v)),target])
assert not bad,bad
for section in ['applications','permission','solvers','settlement','certified-basis']:
 for form in ['explanation','reference','how-to','tutorial']:
  f=v/f'wiki/research/apss/{section}/{form}.md';assert f.exists() and 'diataxis:' in f.read_text()
results['diataxis_notes']=20;results['apss_markdown_pages']=len(list((v/'wiki/research/apss').rglob('*.md')));results['relative_link_errors']=bad
# Preservation of original main roadmap content and original app source code.
a=v/'deliverables/aeon-study-2026-09-19/ROADMAP-before-audit.md';orig=subprocess.check_output(['git','show','1896217a28553e0254b0ba319422ac4c0b39ac0d:ROADMAP.md'],cwd=v);assert a.read_bytes()==orig
results['historical_roadmap_sha256']=hashlib.sha256(orig).hexdigest()
changed=subprocess.check_output(['git','diff','--name-only'],cwd=v,text=True).splitlines();assert not any(x.startswith('experiments/') for x in changed)
results['product_implementation_changes']=False
(r/'delivery-validation.json').write_text(json.dumps(results,indent=2));print(json.dumps(results,indent=2))
