"""Independent literal financial expectations, not a Mori parser/evaluator."""
from pathlib import Path
from copy import deepcopy
import json,hashlib
P=Path(__file__).resolve().parent
S=P.parent/'core-positive-transfer-compiler-source-20261018/input-only'
K=Path('/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018')
def sha(b):return hashlib.sha256(b).hexdigest()
def put(n,v):
 f=P/n;f.parent.mkdir(parents=True,exist_ok=True);f.write_text(json.dumps(v,sort_keys=True,indent=2)+'\n')
pins=json.loads((S/'INPUT-PINS.json').read_text())
for row in pins['files']:
 b=(S/row['path']).read_bytes();assert sha(b)==row['sha256'] and len(b)==row['bytes']
 d=P/'inputs'/row['path'];d.parent.mkdir(exist_ok=True);d.write_bytes(b)
(P/'inputs/INPUT-PINS.json').write_bytes((S/'INPUT-PINS.json').read_bytes())
norm=[]
for path in ['crates/mori-core/README.md','docs/CORE-EXECUTION-REPORTS.md','docs/MORIARTY-PRODUCT-CONTRACT.md']:
 b=(K/path).read_bytes();d=P/'basis'/path;d.parent.mkdir(parents=True,exist_ok=True);d.write_bytes(b)
 norm.append({'source':str(K/path),'copy':'basis/'+path,'bytes':len(b),'sha256':sha(b)})
put('PROVENANCE.json',{'status':'Input-only bytes supplied/frozen by source author before assertions/expected data; no candidate assertions or outputs read',
 'input_manifest_sha256':sha((S/'INPUT-PINS.json').read_bytes()),'input_source':str(S),'startup_checkout':str(K),
 'normative_language_contracts':norm,
 'method':'Source/input and public documented semantics exposed; independent literal arithmetic/frames; standard library copy/JSON/hash/static arithmetic only',
 'prior_exposure':['Earlier Core1/Core2/financial/Source6/composition/multi-condition/Quint/ACTUS oracle lanes','Public DTOs and earlier incidental residual/InputDecl conversion helper snippets','Current public grammar/report clauses; no current implementation/compiler/test/expected/output bodies'],
 'no_global_blind_claim':True,'actual_calls':'UNRUN by this lane'})
U=lambda n:{'type':'UInt','value':str(n)}
B=lambda b:{'type':'Bool','value':b}
CREDIT={'network':'local','issuer':'print-cooperative','reference':'printing-credit','scale':2}
STAMP={'network':'local','issuer':'city-post','reference':'postage','scale':0}
maintenance={'name':'maintenance','asset':CREDIT,'debtor':'printer','creditor':'coop','amount':'11'}
def snapshot(sponsor,printer,coop,copies,spent):return {
 'state':{'copies_paid':U(copies),'spent':U(spent),'campaign':U(17)},
 'balances':[
  {'account':'observer','asset':STAMP,'amount':'6'},
  {'account':'sponsor','asset':STAMP,'amount':'9'},
  {'account':'coop','asset':CREDIT,'amount':str(coop)},
  {'account':'observer','asset':CREDIT,'amount':'19'},
  {'account':'printer','asset':CREDIT,'amount':str(printer)},
  {'account':'sponsor','asset':CREDIT,'amount':str(sponsor)}],
 'liabilities':[deepcopy(maintenance)]}
pre=snapshot(500,23,4,7,41)
payment=snapshot(372,151,4,7,41)
fee=snapshot(367,151,9,7,41)
copies=snapshot(367,151,9,14,41)
post=snapshot(367,151,9,14,174)
e5={'kind':'Transfer','asset':CREDIT,'from':'sponsor','to':'printer','amount':'128'}
e6={'kind':'Fee','asset':CREDIT,'from':'sponsor','to':'coop','amount':'5'}
e7={'kind':'State','name':'copies_paid','before':U(7),'after':U(14)}
e8={'kind':'State','name':'spent','before':U(41),'after':U(174)}
e10={'kind':'Emit','name':'printDue','value':U(128)}
e11={'kind':'Emit','name':'printGross','value':U(133)}
values=[B(True),U(133),B(True),U(128),U(5),U(128),U(5),U(14),U(174),B(True),U(128),U(133)]
snapshots=[pre,pre,pre,pre,pre,payment,fee,copies,post,post,post,post]
deltas=[[],[],[],[],[],[e5],[e6],[e7],[e8],[],[e10],[e11]]
steps=[{'index':i,'value':v,'snapshot':s,'effects':e} for i,(v,s,e) in enumerate(zip(values,snapshots,deltas))]
effects=[e for delta in deltas for e in delta]
projection={'semantics':'moriarty-core/1','action':'pay','pre':pre,'post':post,'steps':steps,'effects':effects}
put('financial-execution-projection.expected.json',projection)
put('pre.expected.json',pre);put('post.expected.json',post);put('ordered-effects.expected.json',effects)
put('FINANCIAL-ACCOUNTING.json',{
 'scope':'Hand-derived normally accepted local Core1 finance; not a whole Execution/Program/compiler-wire oracle',
 'inputs':{'copies':'7','unit_price':'19','rebate':'5','surcharge':'3'},
 'arithmetic':{'gross':'7 * 19 =133','due':'133 -5 =128','fee_total':'3 +2 =5','payer_gross_debit':'128 +5 =133','copies_paid':'7 +7 =14','spent':'41 +128 +5 =174'},
 'not_to_conflate':'Pre-rebate gross133 and actual payer debit133 happen to be equal for these inputs; due128 and fee5 are independently retained',
 'exact_assets':{'CREDIT':CREDIT,'STAMP':STAMP},
 'per_asset_totals':{'CREDIT':{'opening':'546','closing':'546'},'STAMP':{'opening':'15','closing':'15'}},
 'CREDIT_accounts':{'sponsor':{'opening':'500','payment_debit':'128','fee_debit':'5','gross_debit':'133','closing':'367'},'printer':{'opening':'23','net_receipt':'128','closing':'151'},'coop':{'opening':'4','fee_receipt':'5','closing':'9'},'observer':{'opening':'19','closing':'19','delta':'0'}},
 'STAMP_accounts':{'observer':{'opening':'6','closing':'6','delta':'0'},'sponsor':{'opening':'9','closing':'9','delta':'0'}},
 'liabilities':{'maintenance':{'asset':CREDIT,'debtor':'printer','creditor':'coop','opening':'11 present','created':'0','accrued':'0','discharged':'0','closing':'11 present','positive_residual':'11'}},
 'duty_rule':'No Liability or Discharge statement occurs; receiving a sponsor-paid fee at coop does not implicitly discharge printer maintenance debt',
 'preserved_state':{'campaign':'17 throughout','copies_paid':'7 until ordinal7, then14','spent':'41 until ordinal8, then174'},
 'source_binding':'Specified nonempty scenario state overrides copies_paid/spent declaration initializers0; all three state fields retained',
 'normal_order':['12source statements in order','Transfer128','Fee5','State copies_paid7→14','State spent41→174','Emit printDue128','Emit printGross133'],
 'inapplicable':['mint','burn','dedicated accrual','division/rounding/dust','refund/recovery','asset-scale rescaling of expression arithmetic'],
 'not_asserted':['Native/external fee','Authority/history/authentication','Compiler constraint/IR packing','Full backend financial boundary embedding','Target/SDK/proof/ledger qualification']})
put('SOURCE-STEP-MAP.json',{'role':'Documentary source descriptions, not raw Statement serialization/ExprId assertions',
 'steps':[{'index':i,'source':s} for i,s in enumerate([
 'requires copies >0','let gross = copies * unit_price','requires gross >= rebate','let due = gross - rebate','let fee_total = surcharge +2','transfer CREDIT sponsor->printer due','fee CREDIT sponsor->coop fee_total','next copies_paid = copies_paid + copies','next spent = spent + due + fee_total','ensures copies_paid >= copies','emit printDue(due)','emit printGross(gross)'])]})
put('WORK-SCOPE.json',{'policy':'No new work meter, success policy, cap or target work claim added',
 'existing_public_reference_profile':'moriarty-reference-logical-cost/1',
 'informational_source_tree_arithmetic':{'conditional_premise':'Ordinary source expression trees retain their syntactic distinct nodes for reference interpretation; no generated IR was read',
 'per_statement_expression_nodes':[3,3,3,3,3,1,1,3,5,3,1,1],'statement_dispatches':12,'expression_occurrences':30,'sum':'42'},
 'not_an_expected_field':'The input CALL adds no work-success policy; this informational42 is not inserted into ordinary Execution or compiler artifacts and is not an observed meter',
 'unasserted':['Actual metered consumer output','Compiler/circuit/native/SDK/target cost','Lifetime reservation/authority/cap-refusal semantics']})
# Mechanical equation/frame checks over independently assembled expectations only.
assert 7*19==133 and 133-5==128 and 3+2==5
assert 500-128-5==367 and 23+128==151 and 4+5==9
assert 7+7==14 and 41+128+5==174
assert len(steps)==12 and len(effects)==6
for step in steps:
 s=step['snapshot'];assert set(s)=={'state','balances','liabilities'}
 assert set(s['state'])=={'copies_paid','spent','campaign'}
 assert s['state']['campaign']==U(17) and s['liabilities']==[maintenance]
 assert [r for r in s['balances'] if r['asset']==STAMP]==pre['balances'][:2]
 assert sum(int(r['amount']) for r in s['balances'] if r['asset']==CREDIT)==546
 assert sum(int(r['amount']) for r in s['balances'] if r['asset']==STAMP)==15
assert effects==[e for step in steps for e in step['effects']]
assert steps[-1]['snapshot']==post
assert sum([3,3,3,3,3,1,1,3,5,3,1,1])+12==42
print(json.dumps({'pre_sha256':sha((P/'pre.expected.json').read_bytes()),'projection_sha256':sha((P/'financial-execution-projection.expected.json').read_bytes()),'prefix_count':len(steps),'ordered_effects':len(effects),'gross_debit':'133','static_checks':'passed; no product executed'},sort_keys=True))
