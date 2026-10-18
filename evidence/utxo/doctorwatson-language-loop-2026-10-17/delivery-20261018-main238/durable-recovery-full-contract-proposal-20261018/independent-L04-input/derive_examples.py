"""Independent proposed economic examples; no Mori implementation imported/executed."""
from pathlib import Path
from copy import deepcopy
import hashlib,json,shutil
P=Path(__file__).resolve().parent
I=Path('/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017')
def put(n,v):
 p=P/n;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(v,sort_keys=True,indent=2)+'\n')
def sha(b):return hashlib.sha256(b).hexdigest()
provenance=[]
for name in ['AGENTS.md','docs/MORIARTY-PRODUCT-CONTRACT.md','docs/CORE-CONTINUATIONS.md','docs/CORE-STAGED-TESTS.md','crates/mori-core/README.md','openspec/changes/partial-and-conditional-transactions/requirements.md','openspec/changes/partial-and-conditional-transactions/specs/partial-conditional-transactions/spec.md','examples/core/continuations/escrow.mori','examples/core/continuations/delivery.mori']:
 b=(I/name).read_bytes();dst=P/'basis'/name;dst.parent.mkdir(parents=True,exist_ok=True);dst.write_bytes(b)
 provenance.append({'repo_path':name,'read_only_checkout':str(I),'copy':'basis/'+name,'bytes':len(b),'sha256':sha(b)})
put('SOURCE-PROVENANCE.json',{'checkout_commit':'74a909 (parent-reported current source; no Git query performed)',
 'records':provenance,'read_scope':'Product/EARS, public Core2 source examples and public grammar/report/work/API docs only. No proposed recovery runtime, test assertion, expected JSON or actual recovery result body read.',
 'previous_exposure':['Multi-condition source and independently derived reports','Earlier Core1/Core2/R3/Source6/composition oracle custody','Earlier public DTO inspection with incidental residual/helper and InputDecl-to-StageBinder snippets','Unrelated historical result counts shown by public continuation documentation; no new recovery outputs used'],
 'independence':'Source-exposed and post-adoption; not blind or wholly implementation-unexposed. Economic inputs/policies below newly chosen by custodian, not recovered original scenarios.'})
U=lambda n:{'type':'UInt','value':str(n)}
def asset(issuer,ref):return {'network':'local','issuer':issuer,'reference':ref,'scale':0}
def duty(name,a,debtor,creditor,n):return {'name':name,'asset':a,'debtor':debtor,'creditor':creditor,'amount':str(n)}
def effect(kind,a,src,dst,n):return {'kind':kind,'asset':a,'from':src,'to':dst,'amount':str(n)}
def deffect(d,b,a):return {'kind':'Liability',**{k:v for k,v in d.items() if k!='amount'},'before':str(b),'after':str(a)}
def frame(marker,assets,balances,duties):
 rows=[{'account':account,'asset':assets[alias],'amount':str(n)} for alias,ac in balances.items() for account,n in ac.items()]
 rows.sort(key=lambda r:(r['asset']['network'],r['asset']['issuer'],r['asset']['reference'],r['asset']['scale'],r['account']))
 return {'state':{'marker':U(marker)},'balances':rows,'liabilities':sorted(deepcopy(duties),key=lambda r:r['name'])}
def scenario(snapshot,inputs):return {**deepcopy(snapshot),'inputs':{k:U(v) for k,v in inputs.items()}}
for kind in ['custody-refund','forward-compensation']:
 issuer='independent-recovery-'+kind
 assets={'PAY':asset(issuer,'PAY'),'BOND':asset(issuer,'BOND')}
 pay,bond=assets['PAY'],assets['BOND']
 audit=duty('audit',bond,'observer','auditor',4)
 is_refund=kind=='custody-refund';marker=77 if is_refund else 91
 if is_refund:
  initial_bal={'PAY':{'buyer':200,'escrow':5,'merchant':11,'observer':9,'treasury':3},'BOND':{'auditor':0,'escrow':7,'observer':13}}
  afterfee=deepcopy(initial_bal);afterfee['PAY']['buyer']-=4;afterfee['PAY']['treasury']+=4
  funded=deepcopy(afterfee);funded['PAY']['buyer']-=60;funded['PAY']['escrow']+=60
  newd=duty('custody-refund',pay,'escrow','buyer',60)
  recovered=deepcopy(funded);recovered['PAY']['escrow']-=60;recovered['PAY']['buyer']+=60
  closing=deepcopy(newd);closing['amount']='0'
  fee=effect('Fee',pay,'buyer','treasury',4); first=effect('Transfer',pay,'buyer','escrow',60);remedy=effect('Transfer',pay,'escrow','buyer',60)
  initial_inputs={'principal':60,'retainedFee':4};remedy_inputs={'refundAmount':60}
  action='fund';recover_action='refundControlled'
  src=f'''language "moriarty/2";
agreement ProposedCustodyRefund {{
  asset PAY = {{ network: "local", issuer: "{issuer}", reference: "PAY", scale: 0 }};
  asset BOND = {{ network: "local", issuer: "{issuer}", reference: "BOND", scale: 0 }};
  state marker: UInt = 77;
  action recordAudit(auditAmount: UInt) {{
    liability("audit", BOND, "observer", "auditor", auditAmount);
  }}
  action fund(principal: UInt, retainedFee: UInt) {{
    fee(PAY, "buyer", "treasury", retainedFee);
    transfer(PAY, "buyer", "escrow", principal);
    liability("custody-refund", PAY, "escrow", "buyer", principal);
    await acceptance(accepted: Bool) when accepted;
    transfer(PAY, "escrow", "merchant", principal);
    discharge("custody-refund", principal);
  }}
  action refundControlled(refundAmount: UInt) {{
    transfer(PAY, "escrow", "buyer", refundAmount);
    discharge("custody-refund", refundAmount);
  }}
}}
'''
  event={'format':'moriarty-stage-event/1','action':'fund','label':'acceptance','values':[{'name':'accepted','value':{'type':'Bool','value':False}}]}
  econ={'opening_PAY_total':'228','opening_BOND_total':'20','gross_buyer_funding_and_fee':'64','refund_gross_escrow_debit':'60','refund_buyer_receipt':'60','buyer_net_debit_after_refund':'4','retained_fee':'4','fee_recipient_opening':'3','fee_recipient_closing':'7','merchant_net_receipt':'0','controlled_principal_refunded':'60','preserved_escrow_PAY':'5','unrelated_audit_BOND_duty':'4','refund_duty_created':'60','refund_duty_discharged':'60','refund_duty_closing':'present-zero','remedy_residual':[]}
 else:
  initial_bal={'PAY':{'buyer':200,'supplier':8,'reserve':30,'treasury':2,'observer':9},'BOND':{'auditor':0,'reserve':7,'observer':13}}
  afterfee=deepcopy(initial_bal);afterfee['PAY']['buyer']-=3;afterfee['PAY']['treasury']+=3
  funded=deepcopy(afterfee);funded['PAY']['buyer']-=40;funded['PAY']['supplier']+=40
  newd=duty('remedy',pay,'reserve','buyer',20)
  recovered=deepcopy(funded);recovered['PAY']['reserve']-=12;recovered['PAY']['buyer']+=12
  closing=deepcopy(newd);closing['amount']='8'
  fee=effect('Fee',pay,'buyer','treasury',3);first=effect('Transfer',pay,'buyer','supplier',40);remedy=effect('Transfer',pay,'reserve','buyer',12)
  initial_inputs={'price':40,'retainedFee':3,'remedy':20};remedy_inputs={'compensation':12}
  action='purchase';recover_action='compensate'
  src=f'''language "moriarty/2";
agreement ProposedForwardCompensation {{
  asset PAY = {{ network: "local", issuer: "{issuer}", reference: "PAY", scale: 0 }};
  asset BOND = {{ network: "local", issuer: "{issuer}", reference: "BOND", scale: 0 }};
  state marker: UInt = 91;
  action recordAudit(auditAmount: UInt) {{
    liability("audit", BOND, "observer", "auditor", auditAmount);
  }}
  action purchase(price: UInt, retainedFee: UInt, remedy: UInt) {{
    fee(PAY, "buyer", "treasury", retainedFee);
    transfer(PAY, "buyer", "supplier", price);
    liability("remedy", PAY, "reserve", "buyer", remedy);
    await fulfilment(accepted: Bool) when accepted;
    discharge("remedy", remedy);
  }}
  action compensate(compensation: UInt) {{
    transfer(PAY, "reserve", "buyer", compensation);
    discharge("remedy", compensation);
  }}
}}
'''
  event={'format':'moriarty-stage-event/1','action':'purchase','label':'fulfilment','values':[{'name':'accepted','value':{'type':'Bool','value':False}}]}
  econ={'opening_PAY_total':'249','opening_BOND_total':'20','original_payment_gross':'40','original_payment_recipient_net':'40','retained_fee':'3','original_buyer_gross_payment_and_fee':'43','new_compensation_reserve_gross':'12','new_compensation_buyer_net':'12','buyer_net_debit_after_compensation':'31','supplier_original_opening':'8','supplier_closing_preserving_payment':'48','reserve_PAY_opening':'30','reserve_PAY_closing':'18','fee_recipient_opening':'2','fee_recipient_closing':'5','unrelated_audit_BOND_duty':'4','remedy_opening':'absent','remedy_created':'20','remedy_discharged':'12','remedy_closing':'8','not_refunded':'Original supplier payment40 remains a historical transfer; compensation12 is new reserve spending, not its inverse.'}
 directory=P/kind;directory.mkdir(exist_ok=True)
 (directory/'primitive-actions.proposed.mori').write_text(src)
 s0=frame(marker,assets,initial_bal,[audit]);s1=frame(marker,assets,afterfee,[audit]);s2=frame(marker,assets,funded,[audit]);s3=frame(marker,assets,funded,[audit,newd]);s4=frame(marker,assets,recovered,[audit,newd]);s5=frame(marker,assets,recovered,[audit,closing])
 put(kind+'/start.scenario.json',scenario(s0,initial_inputs))
 put(kind+'/condition-not-met.event.json',event)
 # Documentary only: input for an ordinary primitive body, not a recovery-continuation wire encoding.
 put(kind+'/remedy-body.scenario.json',scenario(s3,remedy_inputs))
 create=deffect(newd,0,int(newd['amount']));discharge=deffect(newd,int(newd['amount']),int(closing['amount']))
 residual=[d for d in s5['liabilities'] if int(d['amount'])>0]
 prefixes=[{'label':'opening','snapshot':s0,'new_effects':[]},
 {'label':'fee retained','snapshot':s1,'new_effects':[fee]},
 {'label':'funding/payment committed','snapshot':s2,'new_effects':[first]},
 {'label':'explicit remedy duty created; await entered','snapshot':s3,'new_effects':[create]},
 {'label':'condition not met; no financial transition','snapshot':s3,'new_effects':[]},
 {'label':'proposed new remedy transfer','snapshot':s4,'new_effects':[remedy]},
 {'label':'proposed explicit duty discharge','snapshot':s5,'new_effects':[discharge]}]
 put(kind+'/financial-expectations.proposed.json',{
  'artifact_role':'Custodian-authored proposed normative economic example; this wrapper is documentary, not a Mori wire schema',
  'status':'PROPOSED financial policy; no actual recovery implementation/adoption/consumer observation claimed',
  'assets':assets,'chosen_inputs':initial_inputs,'chosen_remedy_body_inputs':remedy_inputs,
  'action_source':'primitive-actions.proposed.mori','ordinary_start_action':action,'ordinary_remedy_body_action':recover_action,
  'financial_policy':'Return only still-controlled custody principal60, retain already charged fee4 and unrelated duty' if is_refund else 'Pay new compensation12 from separately controlled reserve against expressly declared remedy20; retain original completed payment40 and fee3; leave remedy8',
  'all_prefix_frames':prefixes,'complete_initial':s0,'complete_pending':s3,'proposed_successful_recovery_frame':s5,
  'ordered_history':[fee,first,create,remedy,discharge],
  'opening_duties':s0['liabilities'],'created_duties':[newd],
  'discharged_amount':str(int(newd['amount'])-int(closing['amount'])),
  'closing_duties':s5['liabilities'],'residual_positive_duties':residual,
  'accounting':econ,
  'duty_evolution':{'audit_BOND':'4 + creation0 + accrual0 - discharge0 =4',
  newd['name']:f"absent + creation{newd['amount']} + accrual0 - discharge{int(newd['amount'])-int(closing['amount'])} ={closing['amount']}; present-zero retained when zero"},
  'work':{'known_profile_for_existing_ordinary_primitives':'moriarty-reference-logical-cost/2',
   'per_ordinary_statement_work':['fee2','funding/payment transfer2','liability2','await entry1','false guard2','remedy transfer2','remedy discharge2'],
   'start':'fee dispatch1+Name1 + transfer dispatch1+Name1 + liability dispatch1+Name1 + await entry1 =7',
   'normal_false_observation':'dispatch1+guard Name1 =2; committed stays7',
   'ordinary_remedy_body':'transfer dispatch1+Name1 + discharge dispatch1+Name1 =4, conditional on ordinary body acceptance',
   'separate_ordinary_action_committed':'4 for that standalone action only; not a recovery append policy',
   'arithmetic_sum_of_chosen_operation_work':'7+2+4=13; NOT a normative recovery lifetime counter',
   'recovery_overhead':'UNSPECIFIED','recovery_cumulative_attempted':'UNSPECIFIED','recovery_cumulative_committed':'UNSPECIFIED','failure_attempted':'UNSPECIFIED','external_or_native_fees':'UNSPECIFIED'},
  'condition_not_met_outcome':{'financial_pre':s3,'financial_post':s3,'effects':[],'known_attempted':'2','known_committed':'7','liabilities_retained':s3['liabilities'],
    'meaning':'Local false Bool only; not authenticated failure/nonexecution/cancellation evidence'},
  'failure_before_any_new_financial_effect':{'explicit_case_premise':'No remedy transfer, fee, or discharge has occurred. This is a documentary boundary, not an assumed runtime atomicity/refusal rule.',
    'financial_pre':s3,'financial_post':s3,'new_effects':[],'duties_retained':s3['liabilities'],
    'attempted':'UNSPECIFIED','recovery_status_wire':'UNSPECIFIED','admission_or_error_precedence':'UNSPECIFIED'},
  'failure_after_remedy_transfer_before_discharge':{'explicit_case_premise':'Only if the selected future recovery phase model permits this prefix to commit; current /2 recovery phase policy not selected',
    'conditional_financial_post':s4,'conditional_new_effects':[remedy],'duties_retained':s4['liabilities'],
    'required_rule':'Do not infer discharge from payment or replay the transfer as if absent; outcome/next remedy policy remains unspecified',
    'actual_reachability':'UNSPECIFIED','attempted':'UNSPECIFIED','committed':'UNSPECIFIED','terminal_or_late_event_relation':'UNSPECIFIED'},
  'unresolved':['Recovery transition binding/consumption of original pending frame and ordinary body','Terminal/cancelled/recovered versus still-pending result and treatment of later original accepted event','Recovery report/DTO/semantic-version identifiers, fields and byte rendering','Recovery dispatch/overhead/attempted and committed lifetime convention, cap/refusal policy','Per-phase atomicity, failure fees and partial-effect retention','Evidence/correlation/finality premises; no timeout-only inference','Authority/signatures/asset-control validation are unassessed, not established by balances'],
  'specific_pending_frame_gap':'Original pending custody action still captures principal60 and its source suffix pays merchant60; original compensation action still captures remedy20 while proposed residual is8. No existing ordinary remedy action alone is asserted to close/update either pending frame or alter that suffix. A recovery binding/late-event rule is required.',
  'preservation':'All other exact-asset balance cells, marker, unrelated BOND audit duty4 and earlier committed effect history retained',
  'inapplicable':['mint','burn','dedicated accrual','division/rounding/dust','implicit netting of duty into asset supply'],
  'no_product_execution':True})
 # Exact per-asset conservation and prefix frame preservation checks only.
 def totals(s):
  out={}
  for row in s['balances']:
   key=tuple(row['asset'][k] for k in ['network','issuer','reference','scale']);out[key]=out.get(key,0)+int(row['amount'])
  return out
 for s in [s1,s2,s3,s4,s5]:assert totals(s)==totals(s0)
 assert s0['state']==s5['state'];assert s0['liabilities'][0]==s5['liabilities'][0]
 assert [r for r in s0['balances'] if r['asset']==bond]==[r for r in s5['balances'] if r['asset']==bond]
 assert int(newd['amount'])-(int(newd['amount'])-int(closing['amount']))==int(closing['amount'])
 if is_refund:assert int(s5['liabilities'][1]['amount'])==0
 else:assert int(s5['liabilities'][1]['amount'])==8
print('Two independent proposed economic examples assembled; all per-asset prefix equations passed. Product execution UNRUN.')
