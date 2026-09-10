from pathlib import Path
import hashlib,json,re,subprocess
D=Path('deliverables/language-design-2026-09-09');R=Path('raw/sources/language-design-2026-09-09');root=Path.cwd()
issues=[];checked=0
m=json.loads((D/'source-manifest.json').read_text());g=json.loads((D/'research-graph.json').read_text())
for s in m['sources']:
 p=s.get('raw_path',s.get('raw_file',s.get('local_path',s.get('path'))))
 if p:
  p=Path(p);checked+=1
  if not p.exists():issues.append('missing:'+str(p))
  elif hashlib.sha256(p.read_bytes()).hexdigest()!=s['sha256']:issues.append('hash:'+str(p))
 for pk,hk in [('preferred_text_path','preferred_text_sha256'),('text_path','text_sha256'),('extraction_file','extraction_sha256')]:
  if pk in s:
   p=Path(s[pk]);checked+=1
   if not p.exists() or hashlib.sha256(p.read_bytes()).hexdigest()!=s[hk]:issues.append('extract:'+str(p))
ids=[n['id'] for n in g['nodes']]
if len(ids)!=len(set(ids)):issues.append('duplicate graph IDs')
for e in g['edges']:
 if e['source'] not in ids or e['target'] not in ids:issues.append('dangling edge')
for f in json.loads((D/'features.json').read_text()):
 for k in ['disposition','user_visible_purpose','tradeoff_and_finite_resource_safety','current_moriarty_gap','minimal_acceptance_test','confidence']:
  if not f.get(k):issues.append(f['id']+' missing '+k)
 if f['test_status']!='specified-only; not executed':issues.append('test status '+f['id'])
# Check explicit Markdown links only, excluding external references and fragments.
for p in D.glob('*.md'):
 for t in re.findall(r'\]\(([^)]+)\)',p.read_text()):
  if '://' in t or t.startswith('#'):continue
  dest=t.split('#')[0]
  if dest and not (p.parent/dest).exists():issues.append('broken link '+str(p)+': '+dest)
report=(D/'REPORT.md').read_text();defined=set(re.findall(r'^\[\^(\d+)\]:',report,re.M));used=set(re.findall(r'\[\^(\d+)\]',report))
if used-defined:issues.append('missing footnotes '+str(used-defined))
for p in R.rglob('*.pdf'):
 if not p.read_bytes().startswith(b'%PDF-'):issues.append('not pdf '+str(p))
res={'scope':'source/extract hashes, graph IDs/edges, feature fields/status, local Markdown links, report footnotes, PDF signatures','status':'PASS' if not issues else 'FAIL','checked_source_and_extract_hashes':checked,'graph_nodes':len(ids),'graph_edges':len(g['edges']),'issues':issues,'not_checked':['language execution','paper statistical reanalysis','complete proof derivations','developer usability','native/ledger acceptance']}
(D/'CHECKS.json').write_text(json.dumps(res,indent=2)+'\n');print(json.dumps(res));raise SystemExit(bool(issues))
