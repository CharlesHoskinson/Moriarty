"""Exact roster crosswalk and finite arithmetic checks; not a financial interpreter."""
import csv, hashlib, json
from pathlib import Path
D=Path(__file__).resolve().parent
R=D.parents[1]
def read(p):return json.loads(p.read_text())
ops=read(D/'operations.json')['operations']; cross=read(D/'crosswalk.json')
old=read(R/'evidence/moriarty-completion-program-2026-09-07/SP01/independent-review-round-08/successor/candidate-04-partial/experiments/moriarty-language/spec/successor/signing-display-schema.json')
assert [x['operation'] for x in ops]==old['$defs']['CoreOp']['enum']
assert [x['archivedSignature'] for x in ops]==old['x-coreSignatures']
assert len(ops)==len({x['operation'] for x in ops})==38
assert all(x['proposedSignature'] and x['orderedGuards'] and x['updatesAndEffects'] and x['residualObligation'] and x['unresolved'] for x in ops)
source=list(csv.DictReader((R/'deliverables/defi-language-design-2026-09-07/action-targets.csv').open()))
assert [x['sourceRow'] for x in cross['actionTargets']]==source
assert {x['id'] for x in cross['actionTargets']}=={f'DA{x:02}' for x in range(1,25)}
assert all(set(x['candidateOperations'])<=set(old['$defs']['CoreOp']['enum']) for x in cross['actionTargets'])
rules=read(D/'rule-signatures.json')
assert {x for rule in rules['kinds'] for x in rule['operations']}==set(old['$defs']['CoreOp']['enum'])
assert len({rule['kind'] for rule in rules['kinds']})==len(rules['kinds'])
assert all(rule['parameters'] and rule['result'] and rule['fixedConsumer'] for rule in rules['kinds'])
asset=read(R/'openspec/sprints/asset-study.json')
assert [x['sourceRequirement'] for x in cross['assetRequirements']]==asset['requirements']
assert [x['sourceCase'] for x in cross['assetCases']]==asset['cases']
checks=[]
def check(name,predicate,result):
 assert predicate,name
 checks.append({'id':name,'status':'PASS','calculation':result})
# Source-equation arithmetic, not Boolean expected verdict replay.
owned=10;pledged=6;seized=2
check('FC01-pledge-partial-liquidation', (owned-seized,pledged-seized,(owned-seized)-(pledged-seized))==(8,4,4), '10 owned/6 pledged, seize2 ->8 owned/4 pledged/4 free; no debt equation inferred')
check('FC01b-reservation-with-fill', (100-40,100-40,0+52)==(60,60,52) and 100>60, 'fill40 decrements input balance/reservation100 to60, output reservation0 to52; stale reserved100 exceeds owned60 and rejects')
p,i,pay=100,10,7; di=min(pay,i); dp=pay-di
check('FC02-interest-first', (p-dp,i-di)==(100,3), 'P100/I10/pay7 -> dP0/dI7/P100/I3')
check('FC03-capitalization', (100+7)+(10-7)==110, 'P100/I10 capitalize7 ->P107/I3/O110')
check('FC04-fee-net', 19743-30==19713<19743, 'receive19743 fee30 ->19713, below19743 goal')
check('FC05-gross-refund', 100+1>100 and 100-60==40, 'gross100 stays100 despite net40 after refund60; extra1 violates100 cap')
check('FC06-no-double-payment', 20-7==13 and 20-7-7==6, 'single funded7 leavespayer13; consumer must not debit again to6')
check('FC07-receipt-overallocation', 4+4>7 and 4+3==7, 'two allocations4 to receipt7 reject;4+3 fits')
check('FC08-claim-not-payout-choice', 1000000>1, 'resolved entitlement1 cannot support requested1000000')
U=(1<<128)-1; n=10000*997*2000000; den=1000000*1000+10000*997; q,rem=divmod(n,den)
check('MA01-amm', (q,rem)==(19743,162290000) and n==q*den+rem and 0<=rem<den, 'n19940000000000/d1009970000 ->floor19743/ceil19744/remainder162290000')
check('MA02-product-bound', U*U<(1<<256) and U*2>U, 'AmountProduct fitsU128max squared; Amount*UInt128(2) fails its128-bit intermediate')
check('MA03-directed-price', 10000*19743//10000==19743, 'Price<B,A,4>19743 times10000A ->19743B')
check('MA04-signed-rate', (-25//10,-(-(-25)//10))==(-3,-2), '5A timesRate<1>(-5):floor-3A,ceil-2A')
check('MA05-share-directions', (4*3//10,-(-1*10//3),-(-4*3//10),1*10//3)==(1,4,2,3), 'S3/Va10:deposit4->1share;mint1->4assets;withdraw4->2shares;redeem1->3assets')
check('MA07-full-range-net', (U-0,0-U)==(U,-U) and U>(1<<127)-1, 'net endpoints plus/minusUInt128max are valid; SInt128 would wrongly reject; plus/minus2^128 outsideNetAmount')
check('MA06-intermediate-before-division', (U*2)//2==U and U*2>U, 'final quotient fitting does not rescue overflowing Amount*UInt intermediate')
ds=read(D/'dynamic-shares-signature.json')
assert ds['constructor']=='ConstructShares' and ds['preservedConstructorCount']==40 and ds['newProfileConstructorCount']==41
assert ds['evaluatedOperands']==[{'name':'value','type':'UInt128'}]
check('DS01-domain-and-cost', ds['valueDomain']=={'minimum':'0','maximum':str(U)} and ds['ownNodeCost']+1==2, 'ConstructShares permits0..UInt128max as values; parent+ReadArg costs2, no ownership or mint assertion')
# Protect the already-reviewed expression artifact, rather than rewrite it.
c=read(R/'deliverables/sp01-expression-contract-2026-09-10/source-candidate-02.json')
for f in c['files']:
 p=R/f['path']; assert hashlib.sha256(p.read_bytes()).hexdigest()==f['sha256'],str(p)
print(json.dumps({'status':'PASS_SCOPED','inventory':{'archivedOperations':38,'actionTargets':24,'assetRequirements':11,'assetCases':8,'proposedRuleKinds':len(rules['kinds'])},'arithmetic':checks,'protectedExpressionCandidateFilesVerified':len(c['files']),'scope':'Exact inventories, copied source rows, finite arithmetic and protected pins only. No operation interpreter, semantic agreement, policy/signature check, native/K/ledger run or coverage acceptance.'},indent=2))
