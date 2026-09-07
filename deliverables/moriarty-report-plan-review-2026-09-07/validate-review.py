"""Validate this report/roadmap change; does not run product or proof code."""
from pathlib import Path
import csv,hashlib,json,re,subprocess,datetime
root=Path(__file__).resolve().parents[2];out=Path(__file__).resolve().parent
errors=[];checks={}
def check(name,condition,detail=None):
 checks[name]={'passed':bool(condition),'detail':detail}
 if not condition:errors.append(name)
receipt=json.loads((root/'raw/reports/unified-2026-09-07/receipt.json').read_text())
for row in receipt['files']:
 p=root/row['snapshot'];check('source-hash:'+p.name,hashlib.sha256(p.read_bytes()).hexdigest()==row['sha256'])
rows=list(csv.DictReader((root/'evidence/source-inventory.csv').open()));check('source-ids-unique',len({r['source_id'] for r in rows})==len(rows))
for row in rows:
 if int(row['source_id'][4:])>=70:check('inventory-hash:'+row['source_id'],hashlib.sha256((root/row['local_path']).read_bytes()).hexdigest()==row['sha256'])
graph=json.loads((out/'extraction.json').read_text());ids={n['id'] for n in graph['nodes']}
check('graph-node-ids',len(ids)==len(graph['nodes']) and all(re.fullmatch('[a-z0-9_]+',x) for x in ids))
check('graph-endpoints',all(e['source'] in ids and e['target'] in ids and e['source']!=e['target'] for e in graph['edges']))
check('graph-hyperedges',all(set(e['nodes'])<=ids for e in graph['hyperedges']))
check('graph-source-provenance',all(Path(x['source_file']).exists() and x.get('source_location') for x in graph['nodes']+graph['edges']))
check('graph-confidence',all(e['confidence_score']==1 if e['confidence']=='EXTRACTED' else e['confidence_score'] in [.95,.85,.75,.65,.55] if e['confidence']=='INFERRED' else .1<=e['confidence_score']<=.3 for e in graph['edges']))
for name in ['intents','pcd','defi']:
 d=json.loads((out/(name+'.graph.json')).read_text());check('fragment-retained:'+name,all(n['id'] in ids for n in d['nodes']))
 reportpath=root/'raw/reports/unified-2026-09-07'/('PCD.md' if name=='pcd' else name+'.md');review=(out/(name+'.review.md')).read_text();intervals=[(int(a),int(b)) for a,b in re.findall(r'\|\s*(\d+)[–-](\d+)\s*\|',review,re.M)]
 # Review coverage partitions are checked independently of document-root graph nodes.
 covered={i for a,b in intervals for i in range(a,b+1)};total=len(reportpath.read_text().splitlines());missing=[i for i in range(1,total+1) if i not in covered]
 check('review-line-coverage:'+name,not missing,{'lines':total,'intervals':len(intervals),'missing':missing[:30]})
reg=json.loads((root/'openspec/moriarty-completion-program.json').read_text());by={p['id']:p for p in reg['packages']}
def visit(id,trail):
 if id in trail:raise ValueError('dependency cycle')
 for dep in by[id]['dependencies']:visit(dep,trail+[id])
try:
 for id in by:visit(id,[])
 check('package-dag',True)
except Exception as e:check('package-dag',False,str(e))
check('eight-packages',set(by)=={f'MC{i:02}' for i in range(1,9)})
check('current-reviewer',reg['auditModels']==['claude-fable-5-1','gpt-6-astra'] and reg['auditEfforts']['claude-fable-5-1']=='medium')
# Publication authority changed after the preserved report-review packets.
check('publication-policy',reg['publicationPolicy']['mode']=='github-publication-authorized' and reg['publicationPolicy']['authority']=='raw/assignments/moriarty-github-cleanup-2026-09-07.md')
stages={x['id']:x for x in reg['reportReconciliation']['stageAdmission']['stages']}
check('stage-ids-unique',len(stages)==len(reg['reportReconciliation']['stageAdmission']['stages']))
def visit_stage(id,trail):
 if id in trail:raise ValueError('stage dependency cycle')
 for dep in stages[id]['requires']:visit_stage(dep,trail+[id])
try:
 for id in stages:visit_stage(id,[])
 check('stage-dag-resolvable',True)
except Exception as e:check('stage-dag-resolvable',False,str(e))
for id,subset in [('i2','rp01-mc02'),('f2','rp01-mc03')]:check('subset-gate:'+id,subset in stages[id]['requires'] and 'atomic-accept' in stages[id]['requires'])
check('null-campaigns-not-admitted',not reg['reportReconciliation']['stageAdmission']['dispatchEnabled'] and all(x['campaignRecordId'] is None for x in stages.values()))
try:
 import jsonschema
 jsonschema.validate(reg['reportReconciliation']['stageAdmission'],json.loads((root/'openspec/report-stage-admission.schema.json').read_text()))
 check('stage-json-schema',True)
except Exception as e:check('stage-json-schema',False,str(e))
check('no-execution-admission',reg['currentAdmission']['status']=='not-admitted')
roadmap=(root/'ROADMAP.md').read_text();check('roadmap-package-coverage',all(re.search(r'^### '+id+r':',roadmap,re.M) for id in by))
check('roadmap-wiki-links','wiki/index.md' in roadmap and 'wiki/research-journal.md' in roadmap)
# Check links introduced by this change, not unrelated historical broken links.
changed=subprocess.check_output(['git','diff','--name-only'],cwd=root,text=True).splitlines();untracked=subprocess.check_output(['git','ls-files','--others','--exclude-standard'],cwd=root,text=True).splitlines();bad=[]
for path in changed+untracked:
 p=root/path
 if p.suffix!='.md' or not p.exists() or path.startswith('raw/') or '/audits/' in path:continue
 if path in untracked:text=p.read_text()
 else:
  diff=subprocess.check_output(['git','diff','--unified=0','--',path],cwd=root,text=True);text='\n'.join(x[1:] for x in diff.splitlines() if x.startswith('+') and not x.startswith('+++'))
 for link in re.findall(r'(?<!!)\[[^\]]*\]\(([^)]+)\)',text):
  dest=link.split('#',1)[0].strip('<>')
  if not dest or re.match(r'[a-zA-Z]+:',dest):continue
  if not (p.parent/dest).exists():bad.append({'file':path,'target':link})
check('introduced-local-links',not bad,bad)
validation=json.loads((out/'openspec-validation.json').read_text());check('openspec-strict',len(validation)==8 and all(x['exitCode']==0 for x in validation))
result={'observedAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),'scope':'document hashes, graph, review coverage, local links, package plan schema; no product/proof/network execution','checks':checks,'passed':not errors,'failed':errors};(out/'validation.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'passed':not errors,'failed':errors,'checks':len(checks),'details':[checks[e] for e in errors]},indent=2));raise SystemExit(bool(errors))
