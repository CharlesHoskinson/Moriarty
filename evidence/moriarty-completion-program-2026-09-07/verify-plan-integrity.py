from pathlib import Path
from urllib.parse import unquote
import hashlib,json,re,csv,datetime,argparse
r=Path(__file__).resolve().parents[2]; e=r/'evidence/moriarty-completion-program-2026-09-07'; parser=argparse.ArgumentParser(); parser.add_argument('--candidate', default='plan-candidate-05.json'); args=parser.parse_args(); candidate=json.loads((e/args.candidate).read_text()); program=json.loads((r/'openspec/moriarty-completion-program.json').read_text())
errors=[]
reservations=program.get('resourceReservations',{})
if reservations:
 for field,expected in [('minutes',480),('workerDispatches',24),('previewSubmissions',24),('grossTNight',1000)]:
  if sum(row[field] for row in reservations.values()) != expected: errors.append('reservation sum '+field)
 extension=program.get('extensionOwnership',{})
 if len(extension.get('paths',[]))!=len(set(extension.get('paths',[]))):errors.append('duplicate extension ownership path')
ids={p['id'] for p in program['packages']}; visiting=set(); done=set(); tasks=0; links=0
assert len(ids)==len(program['packages'])==8
by_id={p['id']:p for p in program['packages']}
def visit(pid):
 if pid in visiting: errors.append('dependency cycle '+pid); return
 if pid in done:return
 visiting.add(pid)
 for dep in by_id[pid]['dependencies']:
  if dep not in ids:errors.append('unknown dependency '+dep)
  else:visit(dep)
 visiting.remove(pid);done.add(pid)
for pid in ids:visit(pid)
for f in candidate['files']:
 p=r/f['path']
 if hashlib.sha256(p.read_bytes()).hexdigest()!=f['sha256']:errors.append('candidate drift '+f['path'])
for p in program['packages']:
 for key in ['acceptance','tasks']:
  if not (r/p[key]).is_file(): errors.append('missing '+p[key])
 s=(r/p['tasks']).read_text();tasks+=len(re.findall(r'^- \[ \]',s,re.M))
 if re.search(r'^- \[[xX]\]',s,re.M):errors.append('premature checked task '+p['id'])
 for cmd in p['commands']:
  if cmd not in s:errors.append('command absent from tasks '+cmd)
for f in candidate['files']+ [{'path':p} for p in ['AGENTS.md','docs/MORIARTY_ROADMAP.md','evidence/moriarty-completion-program-2026-09-07/README.md']]:
 p=r/f['path']
 if p.suffix!='.md':continue
 for link in re.findall(r'\]\(([^)]+)\)',p.read_text()):
  link=link.strip('<>')
  if '://' in link or link.startswith('#'):continue
  target=unquote(link.split('#')[0])
  if not (p.parent/target).exists():errors.append('broken link '+f['path']+' -> '+link)
  links+=1
with (r/'evidence/source-inventory.csv').open(newline='') as f: source_rows=list(csv.DictReader(f))
source_ids=[row['source_id'] for row in source_rows]
if len(set(source_ids))!=len(source_ids):errors.append('duplicate source ID')
for row in source_rows:
 if row['source_id'] not in ['SRC-0063','SRC-0064','SRC-0065']:continue
 if hashlib.sha256((r/row['local_path']).read_bytes()).hexdigest()!=row['sha256']:errors.append('source digest drift '+row['source_id'])
receipt={'observed_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'candidate_sha256':candidate['candidate_sha256'],'command':'python3 evidence/moriarty-completion-program-2026-09-07/verify-plan-integrity.py --candidate '+args.candidate,'scope':'structural plan/candidate/DAG/link/task/command/source checks only; no implementation predicate','packages':len(ids),'candidate_files':len(candidate['files']),'unchecked_tasks':tasks,'relative_links_checked':links,'errors':errors,'passed':not errors}
(e/args.candidate.replace('plan-candidate-','plan-integrity-')).write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt,indent=2)); raise SystemExit(bool(errors))
