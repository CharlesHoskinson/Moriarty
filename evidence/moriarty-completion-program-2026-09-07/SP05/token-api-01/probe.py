from pathlib import Path
import json,hashlib,subprocess,shutil
S=Path(__file__).resolve().parent; B=json.loads((S/'binding.json').read_text()); h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
for p,digest in B['compilerFiles'].items():assert h(Path(p))==digest
src=S/'upstream-token-interfaces.compact';assert h(src)==B['source']['sha256']
out=Path('/tmp/compiled');command=[next(p for p in B['compilerFiles'] if p.endswith('/compactc')),'--skip-zk',str(src),str(out)]
r=subprocess.run(command,capture_output=True,text=True,timeout=95);print(r.stdout,end='');print(r.stderr,end='');assert r.returncode==0, 'compiler failed '+str(r.returncode)
files={str(p.relative_to(out)):{'bytes':p.stat().st_size,'sha256':h(p)} for p in out.rglob('*') if p.is_file()}; assert sum(p['bytes'] for p in files.values())<20*1024*1024
shutil.copytree(out,S/'compiled')
for p,digest in B['compilerFiles'].items():assert h(Path(p))==digest
(S/'compiled-manifest.json').write_text(json.dumps({'status':'compiler-exit-zero','sourceSha256':h(src),'outputs':files,'scope':B['scope']},indent=2)+'\n')
print(json.dumps({'status':'compiler-exit-zero','outputFiles':len(files),'scope':B['scope']}))
