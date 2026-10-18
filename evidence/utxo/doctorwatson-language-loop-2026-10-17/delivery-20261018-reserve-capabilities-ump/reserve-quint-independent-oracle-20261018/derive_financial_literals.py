"""Independent finite reserve literals. No Mori/Quint imports or execution."""
from pathlib import Path
from copy import deepcopy
import json,hashlib
P=Path(__file__).resolve().parent
I=Path('/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017')
def put(n,v):
 f=P/n;f.parent.mkdir(parents=True,exist_ok=True);f.write_text(json.dumps(v,sort_keys=True,indent=2)+'\n')
pins=[]
for name in ['examples/core/continuations/reserve.mori','examples/core/continuations/reserve-scenario.json','docs/CORE-STAGED-TESTS.md','docs/CORE-CONTINUATIONS.md']:
 b=(I/name).read_bytes();out=P/'basis'/name;out.parent.mkdir(parents=True,exist_ok=True);out.write_bytes(b)
 pins.append({'source':str(I/name),'copy':'basis/'+name,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()})
put('INPUT-SOURCE-PINS.json',{'checkout_commit':'e4d7a2b5 parent-reported; no Git query','files':pins})
A={'network':'local','issuer':'continuation-reserve-oracle','reference':'UNIT','scale':0}
U=lambda n:{'type':'UInt','value':str(n)}
def duty(n):return {'name':'allocation','asset':A,'debtor':'reserve','creditor':'target','amount':str(n)}
def snap(basis,allocated,balances,liability):return {'state':{'basis':U(basis),'allocated':U(allocated),'marker':U(77)},
 'balances':[{'account':a,'asset':A,'amount':str(v)} for a,v in zip(['observer','reserve','target','treasury'],balances)],'liabilities':[duty(liability)]}
s0=snap(40,0,[42,100,0,0],5)
s1=snap(35,0,[42,100,0,0],5)
s2=snap(35,0,[42,100,0,0],47)
s3=snap(35,42,[42,100,0,0],47)
s4=snap(35,42,[42,58,42,0],47)
s5=snap(35,42,[42,58,42,0],5)
s6=snap(78,42,[42,58,42,0],5)
s7=snap(78,42,[42,56,42,2],5)
e1={'kind':'State','name':'basis','before':U(40),'after':U(35)}
e2={'kind':'Liability','name':'allocation','asset':A,'debtor':'reserve','creditor':'target','before':'5','after':'47'}
e4={'kind':'State','name':'allocated','before':U(0),'after':U(42)}
e5={'kind':'Transfer','asset':A,'from':'reserve','to':'target','amount':'42'}
e6={'kind':'Liability','name':'allocation','asset':A,'debtor':'reserve','creditor':'target','before':'47','after':'5'}
e7={'kind':'State','name':'basis','before':U(35),'after':U(78)}
e10={'kind':'Fee','asset':A,'from':'reserve','to':'treasury','amount':'2'}
e11={'kind':'Emit','name':'settled_quote','value':U(43)}
cap1=[{'name':'extra','value':U(2)},{'name':'held','value':U(42)}]
cap2=cap1+[{'name':'quote','value':U(43)}]
put('INDEPENDENT-FINANCIAL-WORK.json',{
 'status':'Independently derived finite reserve facts; raw/shared projection pending exact schema-only contract',
 'asset':A,'action':'allocate','extra':'2','initial':s0,
 'source_statement_prefixes':[
 {'index':0,'statement':'let held = basis + extra','value':U(42),'snapshot':s0,'new_effects':[],'ordinary_work':'4'},
 {'index':1,'statement':'next basis = basis - 5','value':U(35),'snapshot':s1,'new_effects':[e1],'ordinary_work':'4'},
 {'index':2,'statement':'liability allocation += held','value':U(42),'snapshot':s2,'new_effects':[e2],'ordinary_work':'2'},
 {'index':3,'statement':'await approval entry','emitted_step':False,'snapshot':s2,'new_effects':[],'ordinary_work':'1'},
 {'index':3,'statement':'true approval guard quote43 >= held42','value':{'type':'Bool','value':True},'snapshot':s2,'new_effects':[],'ordinary_work':'4'},
 {'index':4,'statement':'next allocated = held','value':U(42),'snapshot':s3,'new_effects':[e4],'ordinary_work':'2'},
 {'index':5,'statement':'transfer reserve->target held42','value':U(42),'snapshot':s4,'new_effects':[e5],'ordinary_work':'2'},
 {'index':6,'statement':'discharge allocation held42','value':U(42),'snapshot':s5,'new_effects':[e6],'ordinary_work':'2'},
 {'index':7,'statement':'next basis = basis + quote','value':U(78),'snapshot':s6,'new_effects':[e7],'ordinary_work':'4'},
 {'index':8,'statement':'await settlement entry','emitted_step':False,'snapshot':s6,'new_effects':[],'ordinary_work':'1'},
 {'index':8,'statement':'true settlement guard ready','value':{'type':'Bool','value':True},'snapshot':s6,'new_effects':[],'ordinary_work':'2'},
 {'index':9,'statement':'requires quote43 <= 50','value':{'type':'Bool','value':True},'snapshot':s6,'new_effects':[],'ordinary_work':'4'},
 {'index':10,'statement':'fee reserve->treasury extra2','value':U(2),'snapshot':s7,'new_effects':[e10],'ordinary_work':'2'},
 {'index':11,'statement':'emit settled_quote quote43','value':U(43),'snapshot':s7,'new_effects':[e11],'ordinary_work':'2'}],
 'five_invocations':[
 {'name':'start','event':None,'post':s2,'effects':[e1,e2],'site':1,'captures':cap1,'attempted':'11','committed':'11'},
 {'name':'approval-quote41','event':{'quote':'41'},'post':s2,'effects':[],'site':1,'captures':cap1,'attempted':'4','committed':'11','guard':'41 >= 42 false'},
 {'name':'approval-quote43','event':{'quote':'43'},'post':s6,'effects':[e4,e5,e6,e7],'site':2,'captures':cap2,'attempted':'15','committed':'26','guard':'43 >= 42 true'},
 {'name':'settlement-false','event':{'ready':False},'post':s6,'effects':[],'site':2,'captures':cap2,'attempted':'2','committed':'26'},
 {'name':'settlement-true','event':{'ready':True},'post':s7,'effects':[e10,e11],'site':0,'captures':[],'attempted':'10','committed':'36'}],
 'ordered_complete_effect_history':[e1,e2,e4,e5,e6,e7,e10,e11],
 'logical':{'profile':'moriarty-reference-logical-cost/2','successful_segment_cost':['11','15','10'],'committed_before':['0','11','26'],'plan_total':'36','attempted':['11','4','15','2','10'],'committed':['11','11','26','26','36'],'cumulative_attempted':['11','15','30','32','42'],'sum_attempted':'42','false_work_not_committed':'6'},
 'financial':{'asset_total_every_prefix':'142','reserve_gross_debit':'44','reserve_transfer_debit':'42','retained_fee':'2','target_net_receipt':'42','treasury_net_receipt':'2','observer_preserved':'42','reserve_final':'56','liability_opening':'5 present','created':'42','accrued':'0','discharged':'42','closing':'5 present','residual':'5','absence_vs_zero':'No selected prefix has absent or zero allocation; opening/closing are both present5; do not normalize to absent/zero','no_dust_policy':'Integral units; reserve56 is remaining balance, not inferred dust/refund amount'},
 'inapplicable':['mint','burn','division/rounding','interest/accrual schedule','refund/recovery','external fee/proof/ledger settlement'],
 'projection_limits':'Shared financial/model projection must explicitly omit source raw Steps/values/ExprIds/profiles/hashes where its declared DTO does not carry them; no full native correspondence asserted.'})
for s in [s0,s1,s2,s3,s4,s5,s6,s7]:
 assert sum(int(r['amount']) for r in s['balances'])==142
 assert s['state']['marker']==U(77) and s['balances'][0]['amount']=='42'
assert 5+42-42==5 and 100-42-2==56 and 11+4+15+2+10==42
print('Independent source finance/work arithmetic frozen locally; no product execution. Exact projection contract pending.')
