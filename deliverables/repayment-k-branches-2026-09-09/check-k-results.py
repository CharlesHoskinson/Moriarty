from pathlib import Path
import json,hashlib,sys
w=Path(__file__).resolve().parents[2];d=w/'deliverables/repayment-k-branches-2026-09-09';k=w/'experiments/moriarty-language/formal/k';dst=d/'attempt-01';b=k/'.build' if '--retained' not in sys.argv and (k/'.build').exists() else dst
sup=json.loads((d/'execution-supervisor.json').read_text());assert sup['returncode']==0,sup
sys.path.insert(0,str(k));import codec
cases=json.loads((k/'fixtures/branches.json').read_text());src=json.loads((d/'source-observations.json').read_text())['observations'];obs=json.loads((b/'observations.json').read_text());assert len(cases)==len(src)==len(obs)==16
checks=[]
for i,(c,s,o) in enumerate(zip(cases,src,obs),1):
 assert c['id']==s['id']==o['id'];label=f'trace-{i:02d}';receipt=json.loads((b/(label+'.command.json')).read_text());assert receipt['returncode']==0 and not receipt['timedOut']
 packet=codec.admit(json.dumps(c['input']));decoded=codec.decode((b/(label+'.stdout')).read_text(),packet)
 assert decoded==c['expected']==s['result']==o['result'],c['id'];assert o['matchesExpected'] is True
 assert (b/(label+'.input')).read_text()==codec.encode(packet)+'\n'
 checks.append({'id':c['id'],'status':decoded['status'],'completeKSourceIndependentAgreement':True,'inputDigest':codec.digest(packet),'rawOutputSha256':hashlib.sha256((b/(label+'.stdout')).read_bytes()).hexdigest(),'elapsedSeconds':receipt['elapsedSeconds']})
binding=json.loads((b/'binding.json').read_text());assert binding['krunInvocations']==16
actual={str(p.relative_to(b/'moriarty-kompiled')):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted((b/'moriarty-kompiled').rglob('*')) if p.is_file()}
if actual:assert actual==binding['artifacts']
for name,h in binding['sources'].items():assert hashlib.sha256((k/name).read_bytes()).hexdigest()==h
assert 'fixtures/branches.json' in binding['sources'] and 'fixtures/cases.json' not in binding['sources']
compile=json.loads((b/'compile.command.json').read_text());assert compile['returncode']==0 and not compile['timedOut']
summary={'status':'PASS','scope':'16 additional local repayment projection observations; not formal correspondence or ledger acceptance','checks':checks,'prepared':5,'rejected':11,'compileAttempts':1,'krunInvocations':16,'historicalTotalsIncludingThisAllocation':{'compileAttempts':4,'krunInvocations':33},'aggregateSupervisorSeconds':sup['elapsedSeconds'],'compiledArtifactsCurrentlyChecked':len(actual),'selectedSuite':'branches','sourceBinding':binding['sources'],'resultReviews':'pending at execution freeze; see acceptance.json for later disposition'}
# Offline checking does not replace the original execution-result.json.
print(json.dumps({k:v for k,v in summary.items() if k not in ['checks','sourceBinding']},indent=2))
