"""Independent UMP01 literals/arithmetic only: no Mori/ACTUS runtime imports."""
from pathlib import Path
from copy import deepcopy
import json,hashlib
P=Path(__file__).resolve().parent
S=P.parent/'actus-ump01-core-source-candidate-20261018'
def h(b):return hashlib.sha256(b).hexdigest()
def put(n,v):
 p=P/n;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(v,sort_keys=True,indent=2)+'\n')
manifest=(S/'INPUT-ONLY-MANIFEST.json').read_bytes()
assert h(manifest)=='b62e24c0040b319733f49d84f5be133a74df49313f5e251c6b7f886cf78455bd'
M=json.loads(manifest)
for row in M['files']:
 b=(S/row['path']).read_bytes();assert h(b)==row['sha256'] and len(b)==row['bytes']
 dest=P/'input-only'/row['path'];dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(b)
(P/'INPUT-ONLY-MANIFEST.json').write_bytes(manifest)
# Select original inputs excluding results; no upstream result value is displayed
# or used to assemble an expectation.
raw=json.loads((S/'normative/actus-tests-ump.json').read_text())
case=raw['ump01'];operands={k:v for k,v in case.items() if k!='results'}
assert operands==json.loads((S/'inputs/ump01.normative-operands.json').read_text())
whole=(S/'normative/actus-techspecs.tex').read_text()
for row in M['files']:
 if row['path'].startswith('normative/excerpts/'):
  assert (S/row['path']).read_text().strip() in whole,row['path']
# Retain exact normative PAM schedule extension used to exclude fee/reset events.
a=whole.index('\\begin{schedule}{PAM}');b=whole.index('\\end{schedule}',a)+len('\\end{schedule}')
(P/'basis/PAM-SCHEDULE-EXACT.tex').write_text(whole[a:b]+'\n')
U=lambda x:{'type':'UInt','value':str(x)}
B=lambda x:{'type':'Bool','value':x}
A={'network':'actus-reference','issuer':'fixture-only','reference':'USD','scale':0}
T=20130101000000
initial_state={'exchangeRecorded':B(False),'statusTime':U(20121215000000),'nominalInterestRate':U(0),'accruedInterest':U(0)}
def liability():return {'name':'ump01_principal','asset':A,'debtor':'borrower','creditor':'lender','amount':'1000'}
def snap(state,borrower,lender,duties):return {'state':deepcopy(state),'balances':[{'account':'borrower','asset':A,'amount':str(borrower)},{'account':'lender','asset':A,'amount':str(lender)}],'liabilities':deepcopy(duties)}
pre=snap(initial_state,0,2000,[])
transfer=snap(initial_state,1000,1000,[])
created=snap(initial_state,1000,1000,[liability()])
state_time=deepcopy(initial_state);state_time['statusTime']=U(T)
timepost=snap(state_time,1000,1000,[liability()])
state_final=deepcopy(state_time);state_final['exchangeRecorded']=B(True)
post=snap(state_final,1000,1000,[liability()])
e0={'kind':'Transfer','asset':A,'from':'lender','to':'borrower','amount':'1000'}
e1={'kind':'Liability','name':'ump01_principal','asset':A,'debtor':'borrower','creditor':'lender','before':'0','after':'1000'}
e2={'kind':'State','name':'nominalInterestRate','before':U(0),'after':U(0)}
e3={'kind':'State','name':'accruedInterest','before':U(0),'after':U(0)}
e4={'kind':'State','name':'statusTime','before':U(20121215000000),'after':U(T)}
e5={'kind':'State','name':'exchangeRecorded','before':B(False),'after':B(True)}
e6={'kind':'Emit','name':'IED','value':U(T)}
values=[B(True),B(True),B(True),U(1000),U(1000),U(1000),U(0),U(0),U(T),B(True),B(True),U(T)]
snapshots=[pre,pre,pre,pre,transfer,created,created,created,timepost,post,post,post]
deltas=[[],[],[],[],[e0],[e1],[e2],[e3],[e4],[e5],[],[e6]]
node_counts=[2,2,3,1,1,1,1,1,1,1,5,1]
statements=['requires not exchangeRecorded','requires not liability_present("ump01_principal")','requires eventTime == 20130101000000','let principal = 1000','transfer USD lender->borrower principal','liability ump01_principal USD borrower->lender principal','next nominalInterestRate = 0','next accruedInterest = 0','next statusTime = eventTime','next exchangeRecorded = true','ensures liability_present(...) and liability_amount(...) == principal','emit IED(eventTime)']
prefixes=[];cumulative=0;history=[]
for i,(label,value,snapshot,delta,nodes) in enumerate(zip(statements,values,snapshots,deltas,node_counts)):
 cumulative+=1+nodes;history.extend(deepcopy(delta))
 prefixes.append({'source_ordinal':i,'statement_description':label,'typed_result':value,'complete_snapshot':snapshot,'new_effects':delta,'ordered_effect_history':deepcopy(history),'distinct_expression_nodes':nodes,'logical_units':1+nodes,'cumulative_logical_units':cumulative})
assert cumulative==32 and sum(node_counts)==20 and len(history)==7
for row in prefixes:
 assert sum(int(r['amount']) for r in row['complete_snapshot']['balances'])==2000
assert prefixes[4]['complete_snapshot']['liabilities']==[]
assert prefixes[5]['complete_snapshot']['liabilities']==[liability()]
put('CORE-FINANCIAL-WORK.expected.json',{
 'artifact_role':'Independent financial/work projection with full source prefixes; documentary wrapper, not a whole Mori report/Step/Program wire oracle',
 'source_language':'moriarty/1','source_semantics':'moriarty-core/1','action':'initialExchange','logical_profile':'moriarty-reference-logical-cost/1','supplied_cap':None,
 'call_inputs':{'eventTime':U(T)},'asset':A,'subject_bindings':'input-only/inputs/subject-bindings.json',
 'pre':pre,'source_prefixes':prefixes,'post':post,'ordered_effects':history,
 'logical_work':{'statement_dispatches':12,'distinct_expression_occurrences':20,'total':'32','per_statement':[n+1 for n in node_counts],'cumulative':[r['cumulative_logical_units'] for r in prefixes],'metering_scope':'Normally accepted source math only; no execution/rejection/circuit/native cost result'},
 'financial':{'opening_asset_total':'2000','closing_asset_total':'2000','lender_gross_debit':'1000','borrower_net_receipt':'1000','lender_remaining':'1000','explicit_fee':'0','fee_recipient':'inapplicable: no fee statement','reserve_dust':'inapplicable: no rounding/division or reserve rule; lender1000 is preserved balance','opening_principal_duty':'absent','created_principal_duty':'1000','accrued_principal_duty':'0','discharged_principal_duty':'0','closing_principal_duty':'1000 present','residual_positive_duties':[liability()],
 'equation':'opening0 + creation1000 + accrual0 - discharge0 = closing1000; absence is still distinct from present-zero',
 'preserved':'All four declared state fields until their explicit assignments; exact asset and both balance rows; no input/let leaks into state; same-value rate/accrued State effects retained'},
 'time':'Packed decimal calendar label from chosen source input, not Unix seconds, elapsed-time arithmetic or an ACTUS time type',
 'horizon':'2014-01-01T00:00:00 is normative fixture evaluation horizon only, not an additional source call, maturity, repayment or discharge',
 'unasserted':['Raw Program/ExprId/Statement serialization and whole simulate envelopes/stdout/exit','Other ACTUS state variables/full-schema applicability','General UMP behavioral flows/notices/capitalization/nonzero rates','Actual ledger currency mapping, authority, proof or security','External/native fees and costs'],
 'product_execution':'UNRUN'})
put('ACTUS-FINANCIAL-SCHEDULE.expected.json',{
 'artifact_role':'Hand-derived semantic projection; decimal strings here do not assert original upstream result JSON scalar encoding',
 'primary_subject':'tests/actus-tests-ump.json/ump01','operand_source':'input-only/inputs/ump01.normative-operands.json','upstream_result_values_used':False,
 'terms':operands['terms'],'horizon':operands['to'],'dataObserved':{},'eventsObserved':[],
 'role_sign':'RPA = +1','settlement_fx':'1 because settlementCurrency undefined',
 'initial_at_statusDate':{'time':'2012-12-15T00:00:00','notionalPrincipal':'0 (IED is later)','nominalInterestRate':'0 (IED is later)','accruedInterest':'0 (IPNR undefined)','feeAccrued':'0 (FER undefined)','maturityDate':'tmax per UMP no observed PR/PI; no concrete timestamp invented','contractPerformance':'UNRESOLVED: term/default binding not specified in chosen input'},
 'ordered_known_scheduled_financial_events':[{'event_type':'IED','event_time':'2013-01-01T00:00:00','currency':'USD','payoff_in_contract_role_perspective':'-1000','payoff_equation':'FX1 * RPA(+1) * (-1) * NT1000 = -1000','post_notionalPrincipal':'1000','post_nominalInterestRate':'0 (IPNR undefined)','post_accruedInterest':'0 (no explicit IPAC or prior anchor)','post_statusDate':'2013-01-01T00:00:00'}],
 'schedule_basis':{'IED':'UMP same as PAM, exact term date','PR/PI':'No eventsObserved inputs','IPCI':'Undefined IPNR makes capitalization schedule undefined','FP':'Undefined feeRate makes fee schedule undefined','RR/RRF':'No reset anchor/cycle supplied','AD/CE':'No custom analysis/credit-performance observations supplied; no additional financial event invented','MD':'UMP schedule supplies no automatic maturity redemption at fixture horizon'},
 'time_basis':{'statusDate':'2012-12-15T00:00:00','contractDealDate':'2012-12-15T00:00:00','initialExchangeDate':'2013-01-01T00:00:00','to':'2014-01-01T00:00:00','dayCountConvention':'30E360','xDayNotice':'P1M','year_fraction':'UNASSERTED: named convention retained; no numerical year-fraction implementation executed or needed for absent-rate IED','notice':'Retained input term; no observed withdrawal/redemption/notice event or implemented source notice calculation'},
 'principal_at_horizon_under_selected_no_further_events':'1000 outstanding; horizon itself has no payment/discharge',
 'mapped_Mori_effects':'Chosen fixture-only source maps negative RPA payoff to lender->borrower transfer1000 and separately creates borrower->lender duty1000. This is an explicit finite subject/asset embedding, not a universal ACTUS-to-ledger schema.',
 'unknown_state_applicability':['Concrete tmax representation','Contract performance/default binding','Full ACTUS schema-to-Mori state coverage and omitted state/event fields','Currency identity beyond chosen fixture-only binding','General rational/decimal rate and day-count arithmetic'],
 'execution':'No ACTUS or Mori implementation executed'})
print(json.dumps({'source_prefixes':len(prefixes),'ordered_effects':len(history),'logical_work':cumulative,'verified_input_pins':len(M['files']),'raw_results':'Not displayed or used; selected original operands equality checked excluding results','static_financial_checks':'pass'},sort_keys=True))
