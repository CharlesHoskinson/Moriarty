"""Check hand-written expected arithmetic only, NOT acceptance or signing."""
import json
from pathlib import Path
p=Path(__file__).parent
h={x['id']:x for x in json.loads((p/'independent-histories-01.json').read_text())['histories']}
checks=[]
def eq(name,actual,expected):
 if actual!=expected:raise AssertionError((name,actual,expected))
 checks.append({'name':name,'actual':actual,'expected':expected})
loan=h['H-LOAN']
for s in loan['states']:
 d=s['debt'];eq(s['id']+'.debt',d['created']+d['accrued']-d['repaid']-d['writtenOff']-d['credited'],d['outstanding'])
 eq(s['id']+'.cash',sum(s['balances'].values()),100)
for a,b in zip(loan['states'],loan['states'][1:]):eq(b['id']+'.work',a['work'][0]-b['illustrativeCharge'],b['work'][0])
swap=h['H-SWAP-CANCEL']
for s in swap['states']:
 for asset,total in [('A',1100000),('B',2000000)]:eq(s['id']+'.total.'+asset,sum(v for k,v in s['balances'].items() if k.endswith('.'+asset)),total)
 eq(s['id']+'.net',s['traderReceiptsB']-s['traderFeesB'],s['traderNetB'])
a,b,c=swap['states'];eq('cancel.deficit',max(0,swap['authority']['traderNetGoalB']-b['traderNetB']),c['duty']['amount'])
eq('cancel.balance',c['poolFreeB']+c['poolReservedB'],c['balances']['pool.B']);eq('cancel.reserved',c['poolReservedB'],c['duty']['amount']);eq('cancel.recovery',b['remainingWork'][1]-c['illustrativeRecoveryCharge'],c['remainingWork'][1]);eq('insufficient29',29<30,True)
gross=fees=wallet=0
for i,s in enumerate(h['H-REFUND']['steps']):
 gross+=s['debit'];fees+=s['fee'];wallet+=s['refund']-s['debit']-s['fee']
 eq(f'refund{i}.gross',gross,s['grossConsumed']);eq(f'refund{i}.remaining',100-gross,s['remainingGross']);eq(f'refund{i}.wallet',wallet,s['walletDelta'])
eq('gross.exhausted',gross+1>100,True);eq('fee.exhausted',fees+1>5,True)
x=h['H-PARTITION'];ch=x['children'];steps=x['branchSteps'];j=x['joined'];pa=x['parent']
eq('partition.ordinary',sum(c['ordinary'] for c in ch),pa['ordinary']-x['splitCharge']);eq('partition.recovery',sum(c['recovery'] for c in ch),pa['recovery']);eq('partition.gross',sum(c['grossRemaining'] for c in ch),pa['grossRemaining']);eq('partition.duty',sum(c['duty'] for c in ch),pa['duty'])
eq('join.ordinary',sum(c['ordinary'] for c in ch)-sum(s['ordinaryCharge'] for s in steps)-x['joinCharge'],j['ordinary']);eq('join.grossRemaining',pa['grossRemaining']-sum(s['newGross'] for s in steps),j['grossRemaining']);eq('join.cumulative',pa['sharedPrefixGross']+sum(s['newGross'] for s in steps),j['cumulativeGross']);eq('join.duty',sum(c['duty'] for c in ch),j['duty'])
eq('exact.overdelivery',19744==19743,False);eq('minimum.overdelivery',19744>=19743,True);eq('fee.only.net',0-30,-30)
print(json.dumps({'status':'PASS_ARITHMETIC_ONLY','checks':checks,'count':len(checks),'limits':'No canonical encoding/hash/signature, currentness, financial operation or history validator executed. Rejection labels remain specified expectations.'},indent=2))
