from pathlib import Path
import json,hashlib,subprocess
S=Path(__file__).resolve().parent;B=json.loads((S/'binding.json').read_text());W=Path(B['worktree']);F=json.loads((S/'candidate-01-freeze.json').read_text());h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
for p,d in {**F['inputs'],**F['ownedFiles']}.items():assert h(W/p)==d,p
assert not subprocess.check_output(['git','diff','--name-only'],cwd=W,text=True),'tracked edits'
j=json.loads((W/B['ownedFiles'][0]).read_text());old=json.loads((W/'experiments/moriarty-native-ivc-r3/episode.json').read_text());assert {x['path']:x['sha256'] for x in j['inputs'] if 'sha256' in x}==B['inputs'];assert j['status']=='pending-review' and j['candidateHash'] is None and j['reviews']==[]
assert j['fixedRows']['fieldNames']==old['encoding']['financial'];rows=j['fixedRows']['orderedRows'];assert len(rows)==3
for i,row in enumerate(rows):
 assert row['financial']==old['rows'][i]['financial'];assert row['phase']==i and int(row['revision'])==i and int(row['remaining'])==2-i
num=5000000000*8*31;den=100*365;q,r=divmod(num,den);a=j['arithmetic'];assert int(a['interestFloor'])==q and a['interestRemainder']==str(r)+'/'+str(den);assert int(a['settleTotal'])==500000000+q
assert j['encoding']['limbCounts']=={'financialLimbs':22,'digestLimbs':32,'privateLimbs':54,'phaseCells':1}
digests=j['commitmentBindings']['digests'];assert [x['label'] for x in digests]==old['encoding']['digests']
for d in digests:
 if 'retainedHash' in d:assert all(d['retainedHash']==row['hashes'][d['label']] for row in old['rows'])
 assert d['closureTask'] and d['currentCounterpart']
assert j['acceptanceBlockers'] and j['closureTasks'];assert j['workConservation']['residualNotional']=='4500000000 remains Outstanding'
print(json.dumps({'status':'pass','inputHashes':len(B['inputs']),'financialRows':3,'financialFieldsPerRow':11,'digestLabels':8,'independentArithmetic':'Python exact integers','scope':'Static pinned-source/row/arithmetic/shape checks only; not full field-map review, evaluator/native/circuit/proof/network execution'}))
