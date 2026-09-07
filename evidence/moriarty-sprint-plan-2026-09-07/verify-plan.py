"""Recompute plan checks without executing any product or native campaign."""
from pathlib import Path
import json, tempfile, shutil, subprocess, re, hashlib
r=Path(__file__).resolve().parents[2];e=Path(__file__).resolve().parent
results={}
for name,cmd in [('plan',['python3','openspec/sprints/verify.py']),('openspec',['openspec','validate','--all','--strict'])]:
 p=subprocess.run(cmd,cwd=r,capture_output=True,text=True);(e/(name+'-final.txt')).write_text(p.stdout+p.stderr);results[name]={'command':cmd,'exit_code':p.returncode}
 if p.returncode:raise SystemExit(p.stdout+p.stderr)
coverage=json.loads((r/'openspec/sprints/coverage.json').read_text())
files=[r/'openspec/sprints/verify.py',r/'openspec/sprints/sprints.json',r/'openspec/sprints/coverage.json',r/'openspec/moriarty-completion-program.json']
files+=list((r/'openspec/sprints').glob('sp*.md'))+list((r/'openspec/changes').glob('mc*/specs/*/spec.md'))+[r/p for p in coverage['sourceInventories']]
mutations=[]
with tempfile.TemporaryDirectory(prefix='moriarty-plan-check-') as temp:
 root=Path(temp)
 for p in files:
  q=root/p.relative_to(r);q.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,q)
 def reject(name,path,mutate):
  p=root/path;original=p.read_bytes();j=json.loads(original);mutate(j);p.write_text(json.dumps(j));result=subprocess.run(['python3','openspec/sprints/verify.py'],cwd=root,capture_output=True,text=True);p.write_bytes(original)
  assert result.returncode!=0,name
  mutations.append({'case':name,'exit_code':result.returncode,'diagnostic':(result.stdout+result.stderr).strip()[:700]})
 reject('cyclic sprint completion dependency','openspec/sprints/sprints.json',lambda j:j['sprints'][0]['completionRequires'].append('SP12'))
 reject('missing OpenSpec requirement','openspec/sprints/coverage.json',lambda j:j['requirements'].pop())
 reject('missing financial family','openspec/sprints/coverage.json',lambda j:j['targetFamilies'].pop())
 reject('plan attempts to admit dispatch','openspec/sprints/sprints.json',lambda j:j.update(dispatchEnabled=True))
 reject('wrong DeFi denominator','openspec/sprints/coverage.json',lambda j:j['targetFamilies'][3].update(count=71))
 reject('missing task entry gate','openspec/sprints/sprints.json',lambda j:j['sprints'][0]['entryGates'][0]['tasks'].clear())
 reject('missing task prerequisite','openspec/sprints/sprints.json',lambda j:j['sprints'][3]['entryGates'][0]['requires'].clear())
 reject('cyclic RP stage dependency','openspec/moriarty-completion-program.json',lambda j:j['reportReconciliation']['stageAdmission']['stages'][0]['requires'].append('release'))
 reject('missing primary requirement owner','openspec/sprints/coverage.json',lambda j:j['requirements'][0].update(primaryClosingSprint='SP99'))
results['rejection_controls']=mutations
links=[]
paths=[r/'ROADMAP.md',r/'openspec/MORIARTY-COMPLETION-PROGRAM.md']+list((r/'openspec/sprints').glob('*.md'))+list((r/'openspec/changes/bounded-language-completion-sprints').glob('*.md'))
for p in paths:
 for target in re.findall(r'\]\(([^)]+)\)',p.read_text()):
  if '://' in target or target.startswith('#'):continue
  dest=target.split('#')[0]
  if not dest:continue
  assert (p.parent/dest).exists(),(p,target)
  links.append((str(p.relative_to(r)),target))
results['navigation']={'checked_links':len(links),'missing':0}
manifest=json.loads((e/'audits/candidate.json').read_text())
assert all(hashlib.sha256((r/p).read_bytes()).hexdigest()==h for p,h in manifest['files'].items())
results['candidate_sha256']=manifest['candidate_sha256']
results['scope']='Plan structure, pinned inventory consistency, navigation and negative controls only. No language implementation or proof campaign executed.'
(e/'verification-final.json').write_text(json.dumps(results,indent=2)+'\n');print(json.dumps(results,indent=2))
