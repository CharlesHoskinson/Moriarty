from pathlib import Path
import json,hashlib,csv,subprocess,collections,decimal
S=Path(__file__).resolve().parent;B=json.loads((S/'binding.json').read_text());W=Path(B['worktree']);R=Path(B['sourceRoot']);P=W/'experiments/moriarty-language/spec/successor'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
for p,h in B['inputs'].items():assert sha(R/p)==h,('input changed',p)
files={p:sha(W/p) for p in B['ownedFiles']};total=sum((W/p).stat().st_size for p in files);assert total<=700*1024,total
m=json.loads((P/'challenge-map.json').read_text());t=json.loads((P/'challenge-traces.json').read_text());th=json.loads((P/'theorem-ledger.json').read_text())
assert m['status']=='specified-only'
inv=m['inventories'];assert len(inv['actusFixtures'])==277;assert len(inv['actusTaxonomy'])==32;assert len(inv['defiRows'])==72;assert len(inv['defiActions'])==24
# Additional exact identity/field checks follow once the author representation is terminal.
expected={}
for f in sorted((R/'repos/actusfrf/actus-tests/tests').glob('*.json')):
 for key,v in json.loads(f.read_text()).items():expected[str(f.relative_to(R))+'#'+key]={'key':key,'type':v['terms']['contractType'],'resultCount':len(v['results']),'sourceSha256':sha(f)}
assert len(expected)==277;assert len(set(v['type'] for v in expected.values()))==18
actual={r['qualifiedId']:r for r in inv['actusFixtures']};assert set(actual)==set(expected);assert len(actual)==277
for key,e in expected.items():
 r=actual[key];assert (r['key'],r['contractType'],r['resultCount'],r['sha256'])==(e['key'],e['type'],e['resultCount'],e['sourceSha256']),key
for name,path in [('actusTaxonomy','evidence/moriarty-design-sprint-2026-09-06/actus-32-requirements.csv'),('defiRows','evidence/moriarty-design-sprint-2026-09-06/defi-72-requirements.csv'),('defiActions','deliverables/defi-language-design-2026-09-07/action-targets.csv')]:
 rows=list(csv.DictReader((R/path).open()))
 for old,new in zip(rows,inv[name],strict=True):
  assert all(new[k]==v for k,v in old.items()),(name,old)
for f,n in [('heldouts',3),('intent-cases',8),('DeFi-regressions',8)]:assert sum(r['family']==f for r in m['challenges'])==n
assert {r['id'] for r in m['compositionOperators']}=={'sequential','disjoint-parallel','shared-state-interleaving','atomic-synchronization','asynchronous-messaging'}
assert {r['name'] for r in m['candidateProducts']}=={'dYdX Chain','Osmosis','DeepBookV3','THORChain','Velocity','Kamino','Euler V2','Term Finance','UMA Optimistic Oracle','Nexus Mutual','Lightning Network','Balancer V3'}
for r in m['candidateProducts']:assert r['behaviorIds']==[] and r['sourceDefinedExpectedTraceIds']==[]
for r in m['challenges']+m['compositionOperators']+m['candidateProducts']:
 for src in r['source']:assert sha(R/src['path'])==src['sha256']
assert {r['id'] for r in th['theorems']}=={'type-preservation','asset-indexed-accounting','authority-safety','frame-noninterference','assume-guarantee-composition','structural-associativity','obligation-preservation','conservative-extension'}
for r in th['theorems']:assert r['status']=='proposed' and r['axioms']==[] and r['mechanizedEvidence']==[]

required={'NAM19-capitalization','accepted-refinance','pending-redemption','exact-output-swap','partial-fill-batch','refinance','recurring-payments','delegated-rebalancing','contingent-claim','cross-domain-recovery','concentrated-liquidity','iterative-invariant-AMM','ordered-redemption','bad-debt','shared-vault-accounting-hooks','asynchronous-settlement','margin-funding','conditional-payoff-insurance'}
assert len(m['challenges'])==19
assert {r['id'] for r in m['challenges']}==required
for r in m['challenges']+m['compositionOperators']:
 for k in ['id','family','source','sourceGap','representationPath','observationMap','boundedFootprint','environmentAssumptions','positiveTraceId','invalidMutationIds','implementationStatus','ownerPackages','closureSprint','closureTask']:assert k in r,(r.get('id'),k)
assert len(m['compositionOperators'])==5
assert len(m['candidateProducts'])==12
traces={x['id']:x for x in t['traces']};mutations={x['id']:x for x in t['mutations']}
assert len(traces)==len(t['traces']);assert len(mutations)==len(t['mutations'])
for r in m['challenges']+m['compositionOperators']:
 assert r['positiveTraceId'] in traces,r['id'];assert r['invalidMutationIds'],r['id']
 for k in r['invalidMutationIds']:assert k in mutations,(r['id'],k)
# Preserve mathematical evidence separate from source fixture finite decimals.
decimal.getcontext().prec=45;D=decimal.Decimal
capitalized=D(5000)*D('.08')*D(90)/D(365)+D(5000)*D('.1105679012345679')*D(91)/D(365)
assert abs(D('5236.461356333502')-(D(5000)+capitalized))<D('0.000000000001')
q=subprocess.run(['git','diff','--check'],cwd=W,capture_output=True,text=True);assert q.returncode==0,q.stdout+q.stderr
candidate=hashlib.sha256(json.dumps(files,sort_keys=True,separators=(',',':')).encode()).hexdigest()
report={'candidateSha256':candidate,'files':files,'ownedBytes':total,'inputCount':len(B['inputs']),'inventoryCounts':{k:len(v) for k,v in inv.items()},'challengeCount':len(m['challenges']),'operatorCount':len(m['compositionOperators']),'traceCount':len(traces),'mutationCount':len(mutations),'independentNam19Accrual':str(capitalized),'scope':'Independent structural/source checks; economic adequacy requires source inspection and GPT6 review. No execution or proof.'}
(S/'candidate-01-freeze.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
