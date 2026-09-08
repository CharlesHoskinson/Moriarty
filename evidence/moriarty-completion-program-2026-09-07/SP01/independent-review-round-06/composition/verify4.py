from pathlib import Path
import json,hashlib,subprocess,shutil,tempfile
S=Path(__file__).resolve().parent;B=json.loads((S/'binding.json').read_text());W=Path(B['worktree']);R=Path(B['sourceRoot']);sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
for p,h in B['inputs'].items():assert sha(S/p)==h,('input',p)
P=json.loads((S/'old-domain-pins.json').read_text())
for e in P['sources']:assert sha(R/e['path'])==e['sha256'],e['path']
receipt=json.loads((S/'model-correction-worker-receipt.json').read_text());assert receipt['exitCode']==0 and not receipt['unauthorizedGitActivity'] and not receipt['storageStop'];assert 'grok-4.6-build' in receipt['modelUsage']
files={p:sha(W/p) for p in B['ownedFiles']};total=sum((W/p).stat().st_size for p in files);assert total<=B['resources']['ownedBytesLimit']
rel=Path('experiments/moriarty-language/spec/successor/financial-fragments');seed=S/'candidate-02'/rel/'composition.json';old=S/'retained-atomic-fixture.json';pins=S/'old-domain-pins.json';source=W/rel/'composition.json';commands=[]
def run(args,name,expected=0):
 q=subprocess.run(args,cwd=W,capture_output=True,timeout=45);(S/(name+'.stdout')).write_bytes(q.stdout);(S/(name+'.stderr')).write_bytes(q.stderr);commands.append({'argv':[str(x) for x in args],'exitCode':q.returncode,'stdoutSha256':sha(S/(name+'.stdout')),'stderrSha256':sha(S/(name+'.stderr'))});assert q.returncode==expected,(name,q.stderr.decode(),q.stdout.decode()[-1500:]);return q
assert sha(seed)=='3ed9403616dc6cb6eb843ef94812481f168c525e2de375c80b5eac253827db4b';assert sha(old)=='2c44dcd0b364afae95213568aa74dd0e75dbc4264b4164772ba0593efbc1a560'
run(['python3',str(W/rel/'composition-model.test.py'),'--candidate',str(source),'--old-fixture',str(old),'--old-pins',str(pins)],'root-model-tests')
with tempfile.TemporaryDirectory(prefix='composition-root-') as t:
 for n in [1,2]:
  out=Path(t)/str(n);run(['python3',str(W/rel/'composition-model.py'),'--seed',str(seed),'--old-fixture',str(old),'--old-pins',str(pins),'--output',str(out)],'root-generate-'+str(n));assert out.read_bytes()==source.read_bytes(),'not reproducible'
for p,h in files.items():assert sha(W/p)==h,('owned drift',p)
for e in P['sources']:assert sha(R/e['path'])==e['sha256'],e['path']
report={'candidateSha256':hashlib.sha256(json.dumps(files,sort_keys=True,separators=(',',':')).encode()).hexdigest(),'files':files,'totalBytes':total,'commands':commands,'authorReceipt':'model-correction-worker-receipt.json','scope':'Root tests and deterministic reproduction passed. Finite design only; independent semantic review required; no runtime/K/proof/network/semantic-freeze acceptance.'}
O=S/'candidate-03';O.mkdir(exist_ok=False)
for p in files:q=O/p;q.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(W/p,q)
(S/'candidate-03-freeze.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
