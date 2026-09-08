from pathlib import Path
import json,hashlib,subprocess,shutil,os
S=Path(__file__).resolve().parent;B=json.loads((S/'binding.json').read_text());W=Path(B['worktree']);sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
for p,h in B['inputs'].items():assert sha(W/p)==h,('input',p)
for p,h in B['externalInputs'].items():assert sha(Path(p))==h,('runtime',p)
r=json.loads((S/'completion-worker-receipt.json').read_text());assert r['exitCode']==0 and not r['unauthorizedGitActivity'] and not r['storageStop'];assert 'grok-4.6-build' in r['modelUsage']
files={p:sha(W/p) for p in B['ownedFiles']};total=sum((W/p).stat().st_size for p in files);assert total<=B['resources']['ownedBytesLimit'];commands=[];C=W/'experiments/moriarty-midnight-financial/custody';node=B['toolchain']['node']
def run(args,name,timeout=90,env=None):
 q=subprocess.run(args,cwd=W,capture_output=True,timeout=timeout,env=env);(S/(name+'.stdout')).write_bytes(q.stdout);(S/(name+'.stderr')).write_bytes(q.stderr);commands.append({'argv':[str(x) for x in args],'exitCode':q.returncode,'stdoutSha256':sha(S/(name+'.stdout')),'stderrSha256':sha(S/(name+'.stderr'))});assert q.returncode==0,(name,q.stderr.decode(),q.stdout.decode()[-1500:]);return q
for n in [1,2]:
 out=S/('root-gen-'+str(n));assert not out.exists();run([node,str(C/'generate.mjs'),'--output-dir',str(out)],'root-generate-'+str(n));
 for p in ['loan.compact','swap.compact']:assert (out/p).read_bytes()==(C/p).read_bytes(),p
build=S/'build/root-verify';assert not build.exists();run([node,str(C/'build.mjs'),'--output-dir',str(build),'--skip-zk'],'root-compile',timeout=210)
buildbytes=sum(p.stat().st_size for p in (S/'build').rglob('*') if p.is_file() and not p.is_symlink());assert buildbytes<=B['resources']['buildBytes'],buildbytes
env=dict(os.environ);env['MORIARTY_CUSTODY_ARTIFACTS']=str(build)
q=run([node,'--test','--test-reporter=tap','--test-timeout=90000',str(C/'runtime.test.mjs')],'root-runtime-tests',env=env);assert b'# tests 17' in q.stdout and b'# fail 0' in q.stdout
for p,h in files.items():assert sha(W/p)==h,('owned drift',p)
for p,h in B['inputs'].items():assert sha(W/p)==h,('input drift',p)
for p,h in B['externalInputs'].items():assert sha(Path(p))==h,('runtime drift',p)
artifacts={str(p.relative_to(build)):sha(p) for p in build.rglob('*') if p.is_file() and not p.is_symlink()}
report={'candidateSha256':hashlib.sha256(json.dumps(files,sort_keys=True,separators=(',',':')).encode()).hexdigest(),'files':files,'totalBytes':total,'commands':commands,'buildArtifacts':artifacts,'buildBytes':buildbytes,'authorReceipt':'completion-worker-receipt.json','scope':'Root generated twice, compiled fresh with skip-zk, ran17compiled-runtime tests. Synthetic context only. No ledger/payer/proof/wallet/Preview/sprint acceptance; independent GPT6 review required.'}
O=S/'candidate-01';O.mkdir(exist_ok=False)
for p in files:q=O/p;q.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(W/p,q)
(S/'candidate-01-freeze.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'candidateSha256':report['candidateSha256'],'totalBytes':total,'commands':commands,'buildBytes':buildbytes,'scope':report['scope']},indent=2))
