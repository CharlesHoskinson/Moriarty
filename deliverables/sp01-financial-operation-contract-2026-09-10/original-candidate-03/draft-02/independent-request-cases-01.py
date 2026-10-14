"""Independent finite financial arithmetic, not a request interpreter."""
import json,pathlib
p=pathlib.Path(__file__).with_suffix('.json');d=json.loads(p.read_text());c={x['id']:x for x in d['cases']};assert len(c)==13
assert c['RQ02']['expected']['pendingShareEscrow']+c['RQ02']['expected']['fulfilledShares']==100
assert 40*13//10==52
assert c['RQ02']['expected']['reserveCash']+c['RQ02']['expected']['cashEscrow']==1000
x=c['RQ04']['expected'];assert x['pendingShareEscrow']+x['refundedShares']+x['fulfilledShares']==100
assert x['cashAccrued']==x['cashClaimed']+x['cashEscrow']==52
assert x['remainingCashDuty']==32 and x['grossSharesConsumed']==100
assert c['RQ05']['expected']['prospectiveGrossShares']==100+1>c['RQ05']['given']['grossCapShares']
assert 33>c['RQ07']['given']['cashAccrued']-c['RQ07']['given']['cashClaimed']
assert c['RQ08']['given']['reserveCash']<52
assert c['MSG01']['given']['escrowA']==c['MSG01']['expected']['aliceRefundA']+c['MSG01']['expected']['escrowA']
print(json.dumps({'status':'PASS','casesRecorded':len(c),'scope':'selected independent arithmetic/conservation inequalities only; no transition implementation, authorization proof, K or ledger execution'},indent=2))
