from pathlib import Path
import json,hashlib,subprocess,shutil
S=Path(__file__).resolve().parent;B=json.loads((S/'binding.json').read_text());W=Path(B['worktree'])
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
for p,h in B['inputs'].items():assert sha(W/p)==h,('changed input',p)
files={p:sha(W/p) for p in B['ownedFiles']};total=sum((W/p).stat().st_size for p in files);assert total<=B['resources']['ownedBytesLimit'],total
q=subprocess.run(['/usr/local/bin/node','--test','--test-reporter=tap','experiments/moriarty-midnight-financial/tests/differential.test.mjs'],cwd=W,capture_output=True,timeout=60)
(S/'financial-tests2.stdout').write_bytes(q.stdout);(S/'financial-tests2.stderr').write_bytes(q.stderr);assert q.returncode==0,q.stdout.decode()+q.stderr.decode()
assert b'# fail 0' in q.stdout and b'# tests 0' not in q.stdout
# Exact independent integer oracle; no production evaluator/kernel is involved.
interest,remainder=divmod(5000000000*8*31,100*365)
assert interest==33972602 and remainder==27000
loan={'initialPrincipal':'5000000000','principalPaid':'500000000','interestPaid':str(interest),'totalPaid':str(500000000+interest),'remainingPrincipal':'4500000000','borrowerAfter':str(20000000000-500000000-interest)}
num=10000*997*2000000;den=1000000*1000+10000*997;output,swapRemainder=divmod(num,den)
assert output==19743
swap={'grossInput':'10000','economicFee':'30','output':str(output),'poolAAfter':'1010000','poolBAfter':str(2000000-output),'traderAAfter':'90000','traderBAfter':str(output),'quotientRemainder':str(swapRemainder),'denominator':str(den)}
# Field-level validation against the author fixture format follows source inspection;
# this command records its limits and never treats the independent constants alone as comparison acceptance.
for p,h in files.items():assert sha(W/p)==h,('test mutated owned file',p)
for p,h in B['inputs'].items():assert sha(W/p)==h,('changed input',p)
report={'status':'tests-and-independent-oracle-captured','candidateSha256':hashlib.sha256(json.dumps(files,sort_keys=True,separators=(',',':')).encode()).hexdigest(),'files':files,'ownedBytes':total,'testOutputSha256':sha(S/'financial-tests2.stdout'),'independentOracle':{'loan':loan,'loanInterestRemainderNumerator':str(remainder),'loanInterestRemainderDenominator':'36500','swap':swap},'scope':'Node test execution and independently derived numeric constants. Root field-by-field inspection and independent GPT6 review required. No receipt/ledger/proof/network acceptance.'}
(S/'candidate-01-freeze.json').write_text(json.dumps(report,indent=2)+'\n')
O=S/'candidate-01';O.mkdir(exist_ok=False)
for p in files:t=O/p;t.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(W/p,t)
print(json.dumps(report,indent=2))
