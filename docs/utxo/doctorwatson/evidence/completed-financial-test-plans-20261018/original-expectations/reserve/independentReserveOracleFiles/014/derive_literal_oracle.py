"""Independent literal finance assembly; no product code imported/executed."""
from pathlib import Path
from copy import deepcopy
import json,hashlib,subprocess
P=Path(__file__).resolve().parent
S=P.parent/'utxo_compiler_grammar_coverage_sol'
K=Path('/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018')
BASE='64c6dedff1d99b33c23832330d7911a5fad9ed59'
def sha(b):return hashlib.sha256(b).hexdigest()
def put(name,v):
 p=P/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(v,sort_keys=True,indent=2)+'\n')
freeze=(S/'INPUT-FREEZE.sha256').read_bytes()
assert sha(freeze)=='ce878c845e3b059beff1f441c1f5c5ff2e8cccf24bd250449a5d90039b684e1e'
for line in freeze.decode().splitlines():
 digest,name=line.split('  ',1);b=(S/name).read_bytes();assert sha(b)==digest,name
 d=P/'input-custody'/name;d.parent.mkdir(parents=True,exist_ok=True);d.write_bytes(b)
(P/'input-custody/INPUT-FREEZE.sha256').write_bytes(freeze)
# Resolve public normative sources through the exact immutable Git object only.
# These are read-only git show/rev-parse commands, not product runtime or mutation.
pins=[]
for path in ['crates/mori-core/README.md','docs/CORE-EXECUTION-REPORTS.md','docs/MORIARTY-PRODUCT-CONTRACT.md']:
 b=subprocess.run(['git','show',BASE+':'+path],cwd=K,check=True,capture_output=True).stdout
 blob=subprocess.run(['git','rev-parse',BASE+':'+path],cwd=K,check=True,capture_output=True,text=True).stdout.strip()
 d=P/'basis'/path;d.parent.mkdir(parents=True,exist_ok=True);d.write_bytes(b)
 pins.append({'commit':BASE,'repo_path':path,'git_blob':blob,'copy':'basis/'+path,'bytes':len(b),'sha256':sha(b)})
put('NORMATIVE-PINS.json',{'base':BASE,'files':pins,'scope':'Public grammar and adopted Core1 report/financial semantics; no evaluator implementation read'})
U=lambda n:{'type':'UInt','value':str(n)}
B=lambda b:{'type':'Bool','value':b}
CASH={'network':'local','issuer':'reserve-refund','reference':'CASH','scale':0}
BOND={'network':'local','issuer':'reserve-refund','reference':'BOND','scale':0}
observer={'name':'observer-duty','asset':BOND,'debtor':'observer','creditor':'auditor','amount':'5'}
def duty(n):return {'name':'reserve-duty','asset':CASH,'debtor':'reserve','creditor':'payer','amount':str(n)}
def frame(payer,reserve,merchant,treasury,remaining,grossReserved,grossSpent,refunded,reserveDuty):return {
 'state':{'remaining':U(remaining),'grossReserved':U(grossReserved),'grossSpent':U(grossSpent),'refunded':U(refunded),'marker':U(77)},
 'balances':[
 {'account':'auditor','asset':BOND,'amount':'17'},
 {'account':'observer','asset':BOND,'amount':'11'},
 {'account':'reserve','asset':BOND,'amount':'13'},
 {'account':'merchant','asset':CASH,'amount':str(merchant)},
 {'account':'observer','asset':CASH,'amount':'9'},
 {'account':'payer','asset':CASH,'amount':str(payer)},
 {'account':'reserve','asset':CASH,'amount':str(reserve)},
 {'account':'treasury','asset':CASH,'amount':str(treasury)}],
 'liabilities':[deepcopy(observer)]+([] if reserveDuty is None else [duty(reserveDuty)])}
f0=frame(1000,0,5,7,0,0,0,0,None)
f2=frame(900,100,5,7,0,0,0,0,None)
f3=frame(900,100,5,7,0,0,0,0,100)
f4=frame(900,100,5,7,100,0,0,0,100)
f5=frame(900,100,5,7,100,100,0,0,100)
f6=frame(900,63,42,7,100,100,0,0,100)
f7=frame(900,60,42,10,100,100,0,0,100)
f8=frame(900,60,42,10,100,100,0,0,60)
f9=frame(900,60,42,10,60,100,0,0,60)
f10=frame(900,60,42,10,60,100,40,0,60)
f11=frame(920,40,42,10,60,100,40,0,60)
f12=frame(920,40,42,10,60,100,40,0,40)
f13=frame(920,40,42,10,40,100,40,0,40)
f14=frame(920,40,42,10,40,100,40,20,40)
def move(kind,src,dst,n):return {'kind':kind,'asset':CASH,'from':src,'to':dst,'amount':str(n)}
def de(before,after):return {'kind':'Liability','name':'reserve-duty','asset':CASH,'debtor':'reserve','creditor':'payer','before':str(before),'after':str(after)}
def st(name,before,after):return {'kind':'State','name':name,'before':U(before),'after':U(after)}
def em(name,n):return {'kind':'Emit','name':name,'value':U(n)}
deltas=[[],[],[move('Transfer','payer','reserve',100)],[de(0,100)],
 [st('remaining',0,100)],[st('grossReserved',0,100)],
 [move('Transfer','reserve','merchant',37)],[move('Fee','reserve','treasury',3)],
 [de(100,60)],[st('remaining',100,60)],[st('grossSpent',0,40)],
 [move('Transfer','reserve','payer',20)],[de(60,40)],
 [st('remaining',60,40)],[st('refunded',0,20)],[],
 [em('remaining',40)],[em('gross_reserved',100)],[em('gross_spent',40)],[em('refunded',20)]]
values=[U(40),B(True),U(100),U(100),U(100),U(100),U(37),U(3),U(40),U(60),U(40),U(20),U(20),U(40),U(20),B(True),U(40),U(100),U(40),U(20)]
frames=[f0,f0,f2,f3,f4,f5,f6,f7,f8,f9,f10,f11,f12,f13,f14,f14,f14,f14,f14,f14]
steps=[{'index':i,'value':v,'snapshot':s,'effects':d} for i,(v,s,d) in enumerate(zip(values,frames,deltas))]
history=[];prefixes=[]
for i,d in enumerate(deltas):
 history.extend(deepcopy(d))
 prefixes.append({'index':i,'effects':deepcopy(history)})
projection={'semantics':'moriarty-core/1','action':'run','pre':f0,'post':f14,'steps':steps,'effects':history}
put('financial-execution-projection.expected.json',projection)
put('pre.expected.json',f0);put('post.expected.json',f14)
put('ordered-effects.expected.json',history);put('cumulative-prefix-effects.expected.json',prefixes)
labels=['let spendGross = partAmount + feeAmount','requires reserveAmount >= spendGross + refundAmount','transfer CASH payer->reserve reserveAmount','liability reserve-duty reserveAmount','next remaining += reserveAmount','next grossReserved += reserveAmount','transfer CASH reserve->merchant partAmount','fee CASH reserve->treasury feeAmount','discharge reserve-duty spendGross','next remaining -= spendGross','next grossSpent += spendGross','transfer CASH reserve->payer refundAmount','discharge reserve-duty refundAmount','next remaining -= refundAmount','next refunded += refundAmount','ensures liability_amount(reserve-duty) == remaining','emit remaining','emit gross_reserved','emit gross_spent','emit refunded']
put('SOURCE-PREFIX-ACCOUNTING.json',{'scope':'Documentary accounting observations at each source prefix, not extra Execution fields',
 'prefixes':[{'index':i,'source_description':labels[i],'payer_cumulative_gross_debit':str(100 if i>=2 else 0),'payer_refund_credit':str(20 if i>=11 else 0),
 'payer_net_debit':str((100 if i>=2 else 0)-(20 if i>=11 else 0)),
 'reserve_cumulative_gross_debit':str(60 if i>=11 else 40 if i>=7 else 37 if i>=6 else 0),
 'merchant_net_receipt':str(37 if i>=6 else 0),'treasury_fee_receipt':str(3 if i>=7 else 0),
 'source_state':s['state'],'reserve_duty':'absent' if i<3 else '100 present' if i<8 else '60 present' if i<12 else '40 present'} for i,s in enumerate(frames)]})
put('FINANCIAL-ACCOUNTING.json',{
 'inputs':{'reserveAmount':'100','partAmount':'37','feeAmount':'3','refundAmount':'20'},
 'arithmetic':{'spendGross':'37 +3 =40','source_requires':'100 >=40 +20, true','remaining':'100 -40 -20 =40','grossReserved':'100, never reduced by refund','grossSpent':'40, excludes refund','refunded':'20'},
 'per_asset_totals':{'CASH':{'opening':'1021','closing':'1021'},'BOND':{'opening':'41','closing':'41'}},
 'payer':{'opening':'1000','reservation_gross_debit':'100','refund_credit':'20','cumulative_gross_debit_after_refund':'100','net_debit':'80','closing':'920'},
 'reserve':{'opening_CASH':'0','received_funding':'100','merchant_transfer':'37','fee':'3','refund':'20','cumulative_gross_outgoing':'60','closing_CASH':'40','preserved_BOND':'13'},
 'merchant':{'opening':'5','net_receipt':'37','closing':'42'},
 'treasury':{'opening':'7','fee_receipt':'3','closing':'10'},
 'observer':{'CASH':'9 unchanged','BOND':'11 unchanged'},'auditor':{'BOND':'17 unchanged'},
 'net_equation':'payer net debit80 = merchant net37 + retained fee3 + remaining custody40',
 'gross_turnover_note':'Payer gross debit100 and reserve outgoing60 are separate actor histories; their sum160 is internal movement/fee turnover, not payer net cost or source grossSpent',
 'reserve_duty':{'opening':'absent','created':'100','accrued':'0','discharged_for_partial_spend_and_fee':'40','discharged_for_refund':'20','closing':'40 present','equation':'0 +100 -40 -20 =40'},
 'observer_duty':{'asset':BOND,'debtor':'observer','creditor':'auditor','opening':'5 present','created':'0','accrued':'0','discharged':'0','closing':'5 present'},
 'residual_positive_duties':[observer,duty(40)],
 'important_prefix_relation':'Payments, fee, discharges and bookkeeping are separate source statements. Duty/state/balance equality is asserted only where source specifies it, not silently after every intermediate prefix.',
 'allowance_limit':'remaining/grossReserved/grossSpent/refunded are ordinary application UInt counters. No new Core allowance primitive, authority or authenticated budget policy is asserted.',
 'inapplicable':['mint','burn','dedicated accrual','division/rounding/dust','native/external fee measurement','persistent recovery/cancellation/async event semantics'],
 'preserved':'marker77; all three BOND cells; observer CASH9; observer-duty5; exact AssetIds/parties and complete row order'})
put('WORK-SCOPE.json',{'profile':'moriarty-reference-logical-cost/1',
 'informational_source_tree_units':[4,6,2,2,4,4,2,2,2,4,4,2,2,4,4,4,2,2,2,2],
 'source_expression_occurrences':'40','dispatches':'20','informational_sum':'60',
 'premise':'Normal expression-tree visits follow public Core1 source meaning; no generated Program/arena was read or produced',
 'limits':'Call schedule is unmetered and introduces no work-success policy/cap.60 is not an invented ordinary report field or actual cost observation. Actual metered/wire/compiler/circuit/native work remains unasserted.'})
# Pure expected-data consistency checks; no source parser or execution.
assert len(steps)==20 and len(history)==17 and values[8]==U(40) and values[12]==U(20)
assert 100>=37+3+20 and 100-37-3-20==40 and 1000-100+20==920
assert sum([4,6,2,2,4,4,2,2,2,4,4,2,2,4,4,4,2,2,2,2])==60
for s in [f0]+frames:
 assert len(s['balances'])==8 and len(s['state'])==5
 assert sum(int(r['amount']) for r in s['balances'] if r['asset']==CASH)==1021
 assert sum(int(r['amount']) for r in s['balances'] if r['asset']==BOND)==41
 assert s['balances'][:3]==f0['balances'][:3] and s['balances'][4]==f0['balances'][4]
 assert s['liabilities'][0]==observer and s['state']['marker']==U(77)
assert frames[8]['state']['remaining']==U(100) and frames[8]['liabilities'][1]['amount']=='60'
assert frames[11]['state']['refunded']==U(0) and frames[11]['balances'][5]['amount']=='920'
assert frames[12]['state']['remaining']==U(60) and frames[12]['liabilities'][1]['amount']=='40'
assert f14['state']=={'remaining':U(40),'grossReserved':U(100),'grossSpent':U(40),'refunded':U(20),'marker':U(77)}
assert history==[e for s in steps for e in s['effects']]
print(json.dumps({'prefixes':len(steps),'ordered_effects':len(history),'projection_sha256':sha((P/'financial-execution-projection.expected.json').read_bytes()),'static_checks':'pass; product UNRUN'},sort_keys=True))
