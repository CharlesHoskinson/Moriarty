"""Independent literal assembly only. No Mori imports, parsing or execution."""
from pathlib import Path
from copy import deepcopy
import json, hashlib, shutil
ROOT=Path(__file__).resolve().parent
SOURCE=ROOT.parent/'core2-next-stage-input-custody'
def sha(b):return hashlib.sha256(b).hexdigest()
def pretty(v):return (json.dumps(v,sort_keys=True,indent=2,ensure_ascii=True)+'\n').encode('ascii')
def canonical(v):return json.dumps(v,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode('utf-8')
def put(n,v): (ROOT/n).write_bytes(pretty(v))
# Check immutable input custody before any literal expectation publication.
for line in (SOURCE/'FREEZE.sha256').read_text().splitlines():
    digest,name=line.split('  ',1)
    assert sha((SOURCE/name).read_bytes())==digest,name
for path in sorted(SOURCE.rglob('*')):
    if path.is_file():
        dest=ROOT/'input-custody'/path.relative_to(SOURCE)
        dest.parent.mkdir(parents=True,exist_ok=True)
        shutil.copyfile(path,dest)
U=lambda n:{'type':'UInt','value':str(n)}
B=lambda b:{'type':'Bool','value':b}
A={'network':'local','issuer':'multi-condition-input','reference':'PAY','scale':0}
binders=[{'name':'documentObserved','ty':'Bool'},{'name':'approvedAmount','ty':'UInt'}]
expressions=[
 {'kind':'Name','name':'amount'},
 {'kind':'Name','name':'amount'},
 {'kind':'Name','name':'documentObserved'},
 {'kind':'Name','name':'approvedAmount'},
 {'kind':'Name','name':'amount'},
 {'kind':'Binary','op':'Ge','lhs':3,'rhs':4},
 {'kind':'Binary','op':'And','lhs':2,'rhs':5},
 {'kind':'Name','name':'amount'},
 {'kind':'Name','name':'amount'},
]
statements=[
 {'kind':'Transfer','asset':'PAY','from':'alice','to':'escrow','amount':0},
 {'kind':'Liability','name':'delivery','asset':'PAY','debtor':'escrow','creditor':'bob','amount':1},
 {'kind':'Await','label':'release','binders':binders,'guard':6},
 {'kind':'Transfer','asset':'PAY','from':'escrow','to':'bob','amount':7},
 {'kind':'Discharge','name':'delivery','amount':8},
]
program={'language':'moriarty/2','semantics':'moriarty-core/2','name':'MultiConditionDelivery',
 'states':[{'name':'marker','ty':'UInt','initial':U(77)}],
 'assets':[{'name':'PAY','identity':A}], 'expressions':expressions,
 'actions':[{'name':'deliver','inputs':[{'name':'amount','ty':'UInt'}],'statements':statements}]}
pbytes=canonical(program); ph=sha(pbytes)
site={'site':1,'ordinal':2,'label':'release','binders':binders,
 'captures':[{'name':'amount','ty':'UInt','origin':'Input','ordinal':0}]}
plan={'format':'moriarty-stage-plan/1','program_hash':ph,'actions':[{'action':'deliver','statement_count':5,
 'sites':[site],'segment_cost':['5','10'],'committed_before':['0','5'],'plan_total':'15'}]}
lbytes=canonical(plan); lh=sha(lbytes)
put('multi-condition.program.expected.json',program)
put('multi-condition.plan.expected.json',plan)
(ROOT/'multi-condition.program.canonical.bin').write_bytes(pbytes)
(ROOT/'multi-condition.plan.canonical.bin').write_bytes(lbytes)
put('LOCAL-IDENTITIES.json',{'qualification':'local-reference-stage','authentication':'none','consumption':'untracked',
 'program_hash':ph,'program_preimage_bytes':len(pbytes),'program_preimage':'multi-condition.program.canonical.bin',
 'plan_hash':lh,'plan_preimage_bytes':len(lbytes),'plan_preimage':'multi-condition.plan.canonical.bin',
 'basis':'Verbatim designated-owner M03 F4 in input-custody normative CORE-STAGED-TESTS; filled full Program/Plan compact recursive lexical-key UTF-8 JSON; no LF, prefix or framing'})
def duty(n):return {'name':'delivery','asset':deepcopy(A),'debtor':'escrow','creditor':'bob','amount':str(n)}
def snap(amounts,liabilities):return {'state':{'marker':U(77)},
 'balances':[{'account':a,'asset':deepcopy(A),'amount':str(n)} for a,n in zip(['alice','bob','escrow','observer'],amounts)],
 'liabilities':deepcopy(liabilities)}
initial=snap([20,2,3,9],[])
funding=snap([10,2,13,9],[])
waiting=snap([10,2,13,9],[duty(10)])
delivered=snap([10,12,3,9],[duty(10)])
final=snap([10,12,3,9],[duty(0)])
e0={'kind':'Transfer','asset':A,'from':'alice','to':'escrow','amount':'10'}
e1={'kind':'Liability','name':'delivery','asset':A,'debtor':'escrow','creditor':'bob','before':'0','after':'10'}
e3={'kind':'Transfer','asset':A,'from':'escrow','to':'bob','amount':'10'}
e4={'kind':'Liability','name':'delivery','asset':A,'debtor':'escrow','creditor':'bob','before':'10','after':'0'}
cap=[{'name':'amount','value':U(10)}]
idle={'site':0,'captures':[]}; pending={'site':1,'captures':cap}
continuation={'format':'moriarty-continuation/1','program_hash':ph,'plan_hash':lh,
 'action':'deliver','site':1,'captures':cap,
 'logical':{'profile':'moriarty-reference-logical-cost/2','cap':None,'committed':'5'},'boundary':waiting}
def step(i,value,snapshot,effects):return {'index':i,'statement':statements[i],
 'value':value,'snapshot':snapshot,'effects':effects}
startsteps=[step(0,U(10),funding,[e0]),step(1,U(10),waiting,[e1])]
endsteps=[step(2,B(True),waiting,[]),step(3,U(10),delivered,[e3]),step(4,U(10),final,[e4])]
def segment(pre,post,steps,fpre,fpost):return {'action':'deliver','frame_pre':fpre,'frame_post':fpost,
 'execution':{'semantics':'moriarty-core/2','action':'deliver','pre':pre,'post':post,'steps':steps,
 'effects':[deepcopy(e) for s in steps for e in s['effects']]}}
q={'qualification':'local-reference-stage','authentication':'none','consumption':'untracked'}
start={**q,'status':'waiting','advanced':True,'segment':segment(initial,waiting,startsteps,idle,pending),
 'continuation':continuation,'attempted':'5','committed':'5','residual':[duty(10)]}
false={**q,'status':'waiting','advanced':False,'segment':None,'continuation':continuation,
 'attempted':'6','committed':'5','residual':[duty(10)]}
complete={**q,'status':'complete','segment':segment(waiting,final,endsteps,pending,idle),
 'attempted':'10','committed':'15','residual':[]}
reports={'start':start,'document-only':deepcopy(false),'amount-only':deepcopy(false),'both':complete}
for name,report in reports.items():
    put(name+'.report.expected.json',report)
    (ROOT/(name+'.stdout.expected.json')).write_bytes(pretty(report))
    (ROOT/(name+'.stderr.expected.txt')).write_bytes(b'')
put('PROCESS-EXPECTATIONS.json',{'execution':'UNRUN; ordinary accepted positive invocation predictions only',
 'library_pretty_bytes':'UNASSERTED: no transfer from public ASCII stdout contract',
 'invocations':[{'name':n,'exit':0,'stdout':n+'.stdout.expected.json','stderr':n+'.stderr.expected.txt',
 'stdout_encoding':'ASCII, recursive lexical keys, LF, two-space indent, exactly one terminal LF'} for n in reports],
 'runtime_linkage':'Resume exclusively from actual immediately preceding Waiting.continuation. Never feed an expectation into runtime.',
 'false_tuple_policy':'Full independent event tuples; do not accumulate a previous true documentObserved value into another event.'})
put('FINANCE-AND-WORK.json',{
 'basis':'Exact source/scenario/event literals; adopted Core2 report/work/arena/identity clauses; explicit finite Transfer/Liability/Discharge arithmetic premise, not Core1 report-schema inheritance',
 'asset':A,'balance_order':['alice','bob','escrow','observer'],'total_asset_before':'34','total_asset_after':'34',
 'complete_snapshots':{'initial':initial,'after_funding':funding,'after_liability':waiting,'after_delivery':delivered,'after_discharge':final},
 'ordered_effects':[e0,e1,e3,e4],
 'logical':{'profile':'moriarty-reference-logical-cost/2','cap':None,'statement_dispatch':'1',
 'per_statement_node_ids':{'0':[0],'1':[1],'2':[2,3,4,5,6],'3':[7],'4':[8]},
 'cost_derivation':['start: (1+1)+(1+1)+await-entry1=5',
 'each resume guard: dispatch1+distinct nodes2,3,4,5,6=6; both Boolean operands eagerly visited',
 'true suffix: guard6+(1+1)+(1+1)=10'],
 'static_segment_cost':['5','10'],'static_committed_before':['0','5'],'plan_total':'15',
 'attempted_per_invocation':['5','6','6','10'],'committed_per_invocation':['5','5','5','15'],
 'cumulative_attempted':['5','11','17','27'],'sum_attempted':'27','false_attempts_not_committed':'12',
 'not_equated_to':['circuit cost','DUST','native fee','wall time','authenticated lifetime budget']},
 'per_invocation':[
 {'name':'start','event':None,'guard_evaluated':False,'balance_delta':['-10','0','10','0'],
  'transfer_gross_debit':'10','transfer_recipient_credit':'10','explicit_fee':'0','liability_opening':'absent','liability_created':'10','liability_discharged':'0','liability_closing':'10','residual':'10','effects':[e0,e1],'attempted':'5','committed':'5'},
 {'name':'document-only','event':{'documentObserved':True,'approvedAmount':'9'},'guard':'true and (9 >= 10) = false','balance_delta':['0','0','0','0'],
  'transfer_gross_debit':'0','transfer_recipient_credit':'0','explicit_fee':'0','liability_opening':'10','liability_created':'0','liability_discharged':'0','liability_closing':'10','residual':'10','effects':[],'attempted':'6','committed':'5'},
 {'name':'amount-only','event':{'documentObserved':False,'approvedAmount':'12'},'guard':'false and (12 >= 10) = false; eagerly evaluate both','balance_delta':['0','0','0','0'],
  'transfer_gross_debit':'0','transfer_recipient_credit':'0','explicit_fee':'0','liability_opening':'10','liability_created':'0','liability_discharged':'0','liability_closing':'10','residual':'10','effects':[],'attempted':'6','committed':'5'},
 {'name':'both','event':{'documentObserved':True,'approvedAmount':'12'},'guard':'true and (12 >= 10) = true','balance_delta':['0','10','-10','0'],
  'transfer_gross_debit':'10','transfer_recipient_credit':'10','explicit_fee':'0','liability_opening':'10','liability_created':'0','liability_discharged':'10','liability_closing':'present-zero','residual':'0','effects':[e3,e4],'attempted':'10','committed':'15'}],
 'cumulative_finance':{'gross_debits_by_account':{'alice':'10','escrow':'10'},'net_balance_delta':{'alice':'-10','bob':'10','escrow':'0','observer':'0'},
 'internal_transfer_turnover':'20','explicit_fees':'0','bob_net_receipt':'10','opening_liability':'absent','created_liability':'10','discharged_liability':'10','closing_liability':'present-zero',
 'preserved':'marker UInt77; observer9; escrow opening3 remains after onward delivery; all four balance rows and exact asset identity retained',
 'inapplicable':['mint','burn','dedicated interest/accrual','fee recipient','division/rounding/dust','refund/cancellation','external fee/ledger phase'],
 'warning':'Escrow opening3 is simply preserved pre-existing balance; no reserve floor/refund/dust policy is asserted. Liability creation/discharge are separate from token movement.'}
})
# Static internal consistency only: no implementation evidence.
assert len(expressions)==9 and len(statements)==5
assert [s.get('amount',s.get('guard')) for s in statements]==[0,1,6,7,8]
for expr_id,e in enumerate(expressions):
    if e['kind']=='Binary':assert e['lhs']<expr_id and e['rhs']<expr_id
assert reports['document-only']==reports['amount-only']
assert reports['document-only']['continuation']==reports['start']['continuation']
assert reports['both']['segment']['execution']['pre']==reports['start']['continuation']['boundary']
assert sum([5,6,6,10])==27 and 5+10==15
for s in [initial,funding,waiting,delivered,final]:assert sum(int(b['amount']) for b in s['balances'])==34
for report in reports.values():
    seg=report['segment']
    if seg:
        ex=seg['execution'];assert ex['post']==(ex['steps'][-1]['snapshot'] if ex['steps'] else ex['pre'])
        assert ex['effects']==[e for st in ex['steps'] for e in st['effects']]
for n in reports:
    bb=(ROOT/(n+'.stdout.expected.json')).read_bytes();assert bb.isascii() and bb.endswith(b'\n') and not bb.endswith(b'\n\n')
print(json.dumps({'program_hash':ph,'plan_hash':lh,'program_bytes':len(pbytes),'plan_bytes':len(lbytes),'static_checks':'passed; no product executed'},sort_keys=True))
