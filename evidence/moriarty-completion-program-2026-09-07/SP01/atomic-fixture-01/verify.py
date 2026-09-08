from pathlib import Path
import json,hashlib,subprocess,tempfile,shutil
S=Path(__file__).resolve().parent;B=json.loads((S/'binding.json').read_text());W=Path(B['worktree'])
for p,h in B['inputs'].items(): assert hashlib.sha256((W/p).read_bytes()).hexdigest()==h,p
with tempfile.TemporaryDirectory(prefix='moriarty-fixture-red-') as scratch:
 testfile=Path(scratch)/'fixture.test.mjs';shutil.copyfile(W/'experiments/moriarty-atomic-fixture/fixture.test.mjs',testfile)
 red=subprocess.run(['/usr/local/bin/node','--test',str(testfile)],capture_output=True,timeout=30)
 (S/'corrected-red.stdout').write_bytes(red.stdout);(S/'corrected-red.stderr').write_bytes(red.stderr)
 assert red.returncode!=0 and b'missing generate.mjs export buildFixture' in red.stdout, 'corrected tests must fail substantively without implementation'
subprocess.run(['/usr/local/bin/node','--test','experiments/moriarty-atomic-fixture/fixture.test.mjs'],cwd=W,check=True)
a=subprocess.check_output(['/usr/local/bin/node','experiments/moriarty-atomic-fixture/generate.mjs'],cwd=W)
b=subprocess.check_output(['/usr/local/bin/node','experiments/moriarty-atomic-fixture/generate.mjs'],cwd=W)
assert a==b,'non-deterministic capture'
assert len(a)<4*1024*1024,'fixture exceeds 4MiB'
(S/'fixture.json').write_bytes(a)
subprocess.run(['/usr/local/bin/node','experiments/moriarty-atomic-fixture/verify.mjs',str(S/'fixture.json')],cwd=W,check=True)
j=json.loads(a)
expected=json.loads((W/'evidence/moriarty-completion-program-2026-09-07/SP01/loan-swap-subset-01/traces.json').read_text())
assert len(j['states'])==3 and len(j['steps'])==2
assert j['scope']=='simulation-only'
for key,value in expected['examples']['loan']['initialState'].items():
 if key!='note':assert j['states'][0]['body'][key]==value,key
for n,rid in enumerate(['loan-accrue','loan-settle']):
 row=expected['positiveRows'][rid];step=j['steps'][n];result=step['result'];body=result['candidate']['body'];state=body['after']['body']
 assert result['kind']=='Simulation'
 assert step['input']['state']==j['states'][n]
 assert body['after']==j['states'][n+1]
 assert step['input']['action']==row['action']
 for key in ['revision','remaining','episodeStatus','agreementStatus','remainingNotional']:assert state[key]==row[key],key
 for key,ref in [('values','afterValues'),('obligations','retainedObligations')]:assert state[key]==row[ref],key
 for key,ref in [('writes','orderedWrites'),('effects','orderedEffects'),('obligationDelta','obligationDelta')]:assert body[key]==row[ref],key
assert 5000000000*8*31//(100*365)==33972602
assert 500000000+33972602==533972602
for c in j['commitments']:
 encoded=json.dumps(c['preimage'],sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()
 assert encoded.hex()==c['canonicalHex'],c['label']
 assert hashlib.sha256(c['domain'].encode()+b'\x00'+encoded).hexdigest()==c['digest'],c['label']
for p,h in B['inputs'].items(): assert hashlib.sha256((W/p).read_bytes()).hexdigest()==h,p
print(json.dumps({'status':'pass','fixtureSha256':hashlib.sha256(a).hexdigest(),'fixtureBytes':len(a),'commitments':len(j['commitments']),'scope':'Two deterministic local captures, Node fixture controls, independent Python canonical preimage hashes. No native proof or public transaction.'}))
