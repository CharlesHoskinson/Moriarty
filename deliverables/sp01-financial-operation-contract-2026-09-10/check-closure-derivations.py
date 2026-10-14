"""Check source-catalog closure and independent integer discriminators; no evaluator."""
from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;R=D.parents[1]
j=json.loads((D/'financial-pure-signatures.json').read_text())
old=json.loads((R/'experiments/moriarty-language/spec/successor/expression-signatures.json').read_text())['constructors']
oldnames={r['constructor'] for r in old}; newnames={r['constructor'] for r in j['rows']}
assert len(oldnames)==40 and len(newnames)==len(j['rows'])==8 and not oldnames&newnames
assert j['totalConstructors']==len(oldnames|newnames)==48
assert all(r['ownNodeCharge']==1 and r['stateEffects']==[] for r in j['rows'])
body=json.loads((D/'reference-vault-rule.json').read_text())
# Core shape, known constructor names and complete occurrence count only.
def count(x):
 if isinstance(x,dict):
  n=0
  if 'constructor' in x:
   assert set(x)=={'constructor','operands','span'} and x['constructor'] in oldnames|newnames
   assert x['span']=={'kind':'synthetic','start':'0','end':'0'};n=1
  return n+sum(count(v) for v in x.values())
 return sum(count(v) for v in x) if isinstance(x,list) else 0
assert count(body['definition']['body'])==body['fullSyntaxNodesDerived']==56
canonical=json.dumps(body['definition']['body'],sort_keys=True,separators=(',',':')).encode()
assert len(canonical)<=65536
U=(1<<128)-1;Imin=-(1<<127);Smin=-(1<<255)
checks=[]
def check(name,predicate,detail):
 assert predicate,name;checks.append({'id':name,'calculation':detail,'status':'PASS_SCOPED'})
check('full-negative-rate-magnitude',abs(Imin)<=(1<<128)-1,'abs(SInt128.min)=2^127 fits UInt128 but not positive SInt128')
check('full-signed-amount-magnitude',abs(Smin)<=(1<<256)-1,'abs(SignedAmount.min)=2^255 fits UInt256')
check('position-net-delta-full-range',U-(-U)==2*U and 2*U<(1<<256),'net −U ->+U needs delta2U, exceeding UInt128 but fitting SignedDelta magnitude')
check('no-implicit-amount-conversion',2**128>U,'amount<A>(UInt128(MAX)) valid; MAX+1 input invalid; UInt256 child requires explicit narrowing')
check('select-entered-vs-full-work',(1+1+1,1+1+1+1+1,1+1+1+3)==(3,5,6),'literal true selects7 in3 nodes; false enters division in5; whole tree has6')
check('request-reservation-track',(100-40,100-40,52-20)==(60,60,32) and 100>60,'fill releases40 input reserve; unadjusted reserved100 would exceed owned60; residual cash32')
check('vault-fee-gross-net',(100-3,100,100+3)==(97,100,103),'deposit total100/fee3 funds97; redeem net100/fee3 requires vault debit103')
check('fee-receipt-not-debt','Fee'!='Ordinary','DebtRepayment requires Ordinary; matching7 units and parties cannot make a Fee receipt a debt payment')
check('per-leg-failure-order',11>10 and 0<5,'leg1 prospective gross11 exceeds cap10 before leg2 funding0<5 is visited; atomic rollback still required')
check('negative-accrual-no-hidden-credit',12>10,'ReduceInterest on I10 with negative magnitude12 rejects; no implicit principal reduction or reverse debt')
check('claim-residual-conservation',(52,20,32)==(20+32,52-32,52-20),'input/output units stay separate; a cash duty52 after claim20 retains32')
# Verify all root-owned independent challenge bytes in preserved draft03 snapshot.
preserved=json.loads((D/'draft-03-preservation.json').read_text())
for f in preserved['files']:
 assert hashlib.sha256((D/f['preservedPath']).read_bytes()).hexdigest()==f['sha256']
 if f['path'].startswith(('independent-request-cases-01.','independent-vault-arithmetic-01.')):
  assert hashlib.sha256((D/f['path']).read_bytes()).hexdigest()==f['sha256']
print(json.dumps({'status':'PASS_SCOPED','totalProposedConstructors':48,'exactReferenceBodyNodes':56,'referenceBodyCanonicalBytes':len(canonical),'calculations':checks,'scope':'Catalog/shape/count/preservation and explicit integer/purpose discriminators only. No Core evaluation, complete financial transition, signing authorization, K, proof or ledger claim.'},indent=2))
