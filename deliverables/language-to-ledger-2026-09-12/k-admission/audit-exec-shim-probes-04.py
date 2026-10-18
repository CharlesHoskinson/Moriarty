import pathlib,json,hashlib,os,subprocess,importlib.util,sys
E=pathlib.Path(__file__).parent
W=pathlib.Path('/home/charl/Moriarty/.worktrees/lifecycle-k'); H=W/'.moriarty-dev'
def sha(p):return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
spec=importlib.util.spec_from_file_location('shim',H/'k-macro05-trace106-diagnostic.py');s=importlib.util.module_from_spec(spec);spec.loader.exec_module(s)
p=s.load_pins();freeze=json.loads((E/'root-exec-shim-freeze-04.json').read_text()); hashes={str(x):sha(W/x) for x in freeze['files']};assert hashes==freeze['files']
files={}
def record(path,expected=None):
 path=pathlib.Path(path); actual=sha(path);files[str(path)]={'sha256':actual,'bytes':path.stat().st_size,'expectedMatch':expected is None or expected==actual};assert expected is None or expected==actual
for f,h in hashes.items():record(W/f,h)
for x in p['selectedExecutables']+p['retained']:record(x['path'],x['sha256'])
for name in ('requisites','ownership','kPackage'):record(H/p['manifests'][name],p['manifests'][name+'SHA256'])
b=json.loads(pathlib.Path(p['binding']['path']).read_text())
for name,h in b['sources'].items():record(pathlib.Path(p['binding']['kRoot'])/name,h)
for name,h in b['artifacts'].items():record(pathlib.Path(p['binding']['kompiled'])/name,h)
km=json.loads((H/p['manifests']['kPackage']).read_text()); kr=pathlib.Path(p['recordedArgv'][0]).parent.parent
for x in km['files']:record(x['path'],x['sha256'])
actual={str(x) for x in kr.rglob('*') if x.is_file()};mapped={x['path'] for x in km['files']};assert actual==mapped
req=json.loads((H/p['manifests']['requisites']).read_text());own=json.loads((H/p['manifests']['ownership']).read_text())
for obj,keys in [(req,['observation','batchOrderControl']),(own,['hostEvidence','nixMetadataObservation'])]:
 for key in keys:record(obj[key+'Path'],obj[key+'SHA256'])
record(H/own['hostEvidencePointer'],own['hostEvidencePointerSHA256'])
he=s.load_host_evidence(own);hostbad=[]
for path,entry in he.items():
 st=pathlib.Path(path).lstat()
 if st.st_uid!=entry['hostUid'] or st.st_dev!=entry['lstatDevice'] or st.st_ino!=entry['lstatInode'] or (st.st_mode&0o7777)!=entry['mode'] or os.access(path,os.W_OK):hostbad.append(path)
# No native execution: the only final exec is a Python callback.
code="import importlib.util,json,pathlib; h=pathlib.Path("+repr(str(H))+"); spec=importlib.util.spec_from_file_location('shim',h/'k-macro05-trace106-diagnostic.py'); s=importlib.util.module_from_spec(spec); spec.loader.exec_module(s); r=s.run_shim(execve=lambda *a:None); r['uidMap']=pathlib.Path('/proc/self/uid_map').read_text(); print(json.dumps(r))"
argv=['/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin/node','/home/charl/foreman/skills/foreman/runtime/dist/foreman-launch.js','--timeout','20','--grace','5','--require-containment','strong','--','/usr/bin/python3.14','-I','-B','-c',code]
r=subprocess.run(argv,cwd=W,capture_output=True,timeout=55);(E/'audit-exec-shim-preflight-04.stdout').write_bytes(r.stdout);(E/'audit-exec-shim-preflight-04.stderr').write_bytes(r.stderr);assert r.returncode==0,(r.returncode,r.stderr)
# Enumerate actual PATH-selected tools without executing any of them.
dirs=[str(kr/'bin-unwrapped')]+list(reversed(p['wrapperPathDirs']))+p['childEnv']['PATH'].split(':')
selected=[]
for name in ['dirname','basename','mktemp','date','rm','cat','cp','fold','uname','grep','head','cut','sed','java','llvm-krun','kore-print','python3']:
 path=s.resolve_on_path(name,dirs);selected.append({'name':name,'path':str(path),'resolved':str(path.resolve()) if path else None,'explicitSelected':any(i['path']==str(path) for i in p['selectedExecutables']),'sha256':sha(path) if path else None})
# Source-exact proofs of omitted runtime ownership checks and HOME loading.
body=(H/'k-macro05-trace106-diagnostic.py').read_text();checkjava=(kr/'lib/kframework/checkJava').read_text()
result={'frozenFiles':hashes,'candidateHash':freeze['candidateHash'],'closurePaths':len(files),'allHashesMatch':True,'kPackageSetExact':True,'kFiles':len(mapped),'hostOwnershipPaths':len(he),'hostOwnershipDrift':hostbad,'strongPreflight':{'exitCode':r.returncode,'stdoutSHA256':hashlib.sha256(r.stdout).hexdigest(),'stderrSHA256':hashlib.sha256(r.stderr).hexdigest(),'result':json.loads(r.stdout)},'actualToolSelections':selected,'envOwnershipBinding':s.bind_path_for_ownership('/usr/bin/env',he),'dbOwnershipReferenced': '/nix/var/nix/db' in body,'homeClasspathInSelectedLoader':'$HOME/.local/lib/kframework/java/*' in checkjava,'homeNailgunInSelectedLoader':'$HOME/.kserver' in checkjava,'homePathsPresent':{x:pathlib.Path(x).exists() for x in ['/home/charl/.kserver','/home/charl/.local/lib/kframework/java']},'sourceStable':hashes=={x:sha(W/x) for x in hashes},'nativeInvoked':False}
(E/'audit-exec-shim-closure-04.json').write_text(json.dumps(files,indent=2)+'\n');(E/'audit-exec-shim-probes-04.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
