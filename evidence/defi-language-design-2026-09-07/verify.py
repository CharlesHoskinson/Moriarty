"""Read-only checks for the source-extension migration and research artifacts."""
from pathlib import Path
import csv,json,hashlib,subprocess,re
r=Path(__file__).resolve().parents[2];e=r/'evidence/defi-language-design-2026-09-07';d=r/'deliverables/defi-language-design-2026-09-07'
h=lambda b:hashlib.sha256(b).hexdigest()
checks=[]
def check(name,value):
 checks.append({'check':name,'passed':bool(value)})
 if not value: raise AssertionError(name)
migration=json.loads((e/'extension-migration.json').read_text());base=migration['base_commit']
for f in migration['source_renames']:
 before=subprocess.check_output(['git','show',base+':'+f['old']],cwd=r);after=(r/f['new']).read_bytes();check(f['new']+' exact source bytes retained',h(before)==h(after)==f['sha256'])
bound='experiments/moriarty-language/spec/bounds.json';check('registered bounds bytes unchanged',subprocess.check_output(['git','show',base+':'+bound],cwd=r)==(r/bound).read_bytes())
old=subprocess.check_output(['git','show',base+':evidence/source-inventory.csv'],cwd=r);new=(r/'evidence/source-inventory.csv').read_bytes();check('original source inventory bytes retained as prefix',new.startswith(old))
rows=list(csv.DictReader((r/'evidence/source-inventory.csv').open()));check('source IDs unique',len({x['source_id'] for x in rows})==len(rows));source=json.loads((r/'wiki/meta/ledgers/source-ledger.json').read_text());mapping=json.loads((e/'source-map.json').read_text())
for key,m in mapping.items():
 row=next(x for x in rows if x['source_id']==m['source_id']);check(key+' capsule-file digest (not original payload)',h((r/row['local_path']).read_bytes())==row['sha256']);check(key+' portable mapping',source['legacy_source_ids'][m['source_id']]==m['portable_id'])
actions=list(csv.DictReader((d/'action-targets.csv').open()));check('target IDs unique',len({x['target_id'] for x in actions})==len(actions));check('target references resolve',all(k in mapping or k=='existing-reports' for a in actions for k in a['source_keys'].split(';')));check('every target has a distinguishing test and disposition',all(a['distinguishing_test'] and a['implementation_disposition'] for a in actions))
graph=json.loads((d/'research-graph.json').read_text());ids={x['id'] for x in graph['nodes']};check('graph node IDs unique',len(ids)==len(graph['nodes']));check('graph endpoints resolve',all(x['source'] in ids and x['target'] in ids for x in graph['edges']))
active=subprocess.run(['git','grep','-n',r'\.moriarty\b','--','experiments/moriarty-language','README.md','ROADMAP.md','openspec/changes'],cwd=r,capture_output=True,text=True)
check('no old extension in active language readers and guide references',active.returncode==1)
for p in [r/'README.md',r/'ROADMAP.md',r/'openspec/DEFI-LANGUAGE-DESIGN-2026-09-07.md',*d.glob('*.md')]:
 for target in sorted(set(re.findall(r'\]\(([^)]+)\)',p.read_text()))):
  if '://' in target or target.startswith('#'):continue
  target=target.split('#')[0]
  check(str(p.relative_to(r))+' link '+target,(p.parent/target).exists())
check('language suite passed', 'ℹ pass 78' in (e/'language-tests.txt').read_text() and 'ℹ fail 0' in (e/'language-tests.txt').read_text())
print(json.dumps({'scope':'Source-path migration and research artifact structure; no new semantic correctness, usability, K or proof result','checks':checks,'passed':len(checks),'source_records':len(rows),'action_targets':len(actions)},indent=2))
