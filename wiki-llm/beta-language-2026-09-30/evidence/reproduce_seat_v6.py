"""Independent argv-only consumer reproduction; never execute a worker command string."""
from pathlib import Path
import subprocess,json,sys,hashlib
seat=sys.argv[1];project=Path('/tmp/moriarty-beta-developers-v3-20260930')/seat;cli=Path(sys.argv[2]) if len(sys.argv)>2 else project/'node_modules/.bin/mori';results=[]
def run(args):
 r=subprocess.run([str(cli),*args],cwd=project,capture_output=True,text=True)
 try:body=json.loads(r.stdout)
 except json.JSONDecodeError:body=None
 return {'argv':['mori',*args],'exit_code':r.returncode,'status':body.get('status') if isinstance(body,dict) else None,'diagnostics':body.get('diagnostics') if isinstance(body,dict) else None,'body':body,'stderr':r.stderr}
for file in sorted(project.rglob('*.mori')):
 if 'node_modules' in file.relative_to(project).parts:continue
 rel=str(file.relative_to(project));x=run(['check',rel,'--json']);results.append(x)
for file in sorted(project.rglob('mori.tests.json')):
 if 'node_modules' in file.relative_to(project).parts:continue
 results.append(run(['test',str(file.parent.relative_to(project))]))
out=Path('/home/charl/research/moriarty-beta-2026-09-30')/(seat+'-parent-reproduction-v6.json');out.write_text(json.dumps({'seat':seat,'mode':'independent argv-only source checks and all explicit case manifests','results':results},indent=2)+'\n')
print(json.dumps({'seat':seat,'source_checks':[{'source':x['argv'][2],'exit':x['exit_code'],'status':x['status']} for x in results if x['argv'][1]=='check'],'project_cases':[{'project':x['argv'][2],'exit':x['exit_code'],'status':x['status'],'cases':x['body'].get('cases') if isinstance(x['body'],dict) else None} for x in results if x['argv'][1]=='test']},indent=2))
