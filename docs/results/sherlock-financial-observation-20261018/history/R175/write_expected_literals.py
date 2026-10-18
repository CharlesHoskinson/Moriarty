#!/usr/bin/env python3
"""Hand-authored literal expected data, not a parser/evaluator/report implementation.
Only stdlib literal data, copy, canonical JSON identity hashing and file IO.
"""
import copy
import hashlib
import json
from pathlib import Path
P = Path(__file__).resolve().parent

def emit(name, obj):
    (P / name).write_text(json.dumps(obj, indent=2, ensure_ascii=False) + '\n')

def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode()

def H(value):
    return hashlib.sha256(canonical(value)).hexdigest()

def U(value): return {'type':'UInt', 'value':str(value)}
def B(value): return {'type':'Bool', 'value':value}
def A(reference): return {'network':'local','issuer':'r151-financial-observation','reference':reference,'scale':0}
UNIT, AUX = A('UNIT'), A('AUX')
# Literal source table, manually transcribed in parser declaration/occurrence order.
statements = [
 {'kind':'Transfer','asset':'UNIT','from':'reserve','to':'beneficiary','amount':0},
 {'kind':'Fee','asset':'UNIT','from':'reserve','to':'fee_recipient','amount':1},
 {'kind':'Discharge','name':'pay-beneficiary','amount':2},
 {'kind':'Await','label':'release','binders':[{'name':'ready','ty':'Bool'}],'guard':3},
 {'kind':'Transfer','asset':'UNIT','from':'reserve','to':'beneficiary','amount':4},
 {'kind':'Fee','asset':'UNIT','from':'reserve','to':'fee_recipient','amount':5},
 {'kind':'Discharge','name':'pay-beneficiary','amount':6},
]
program = {
 'language':'moriarty/2','semantics':'moriarty-core/2','name':'RetainedPaymentObservation',
 'states':[{'name':'marker','ty':'UInt','initial':U(77)},{'name':'flag','ty':'Bool','initial':B(True)}],
 'assets':[{'name':'UNIT','identity':UNIT},{'name':'AUX','identity':AUX}],
 'expressions':[{'kind':'Literal','value':U(30)},{'kind':'Literal','value':U(2)},
 {'kind':'Literal','value':U(30)},{'kind':'Name','name':'ready'},
 {'kind':'Literal','value':U(10)},{'kind':'Literal','value':U(1)},{'kind':'Literal','value':U(10)}],
 'actions':[{'name':'pay','inputs':[],'statements':statements}],
}
# Corrected input appends an unselected action and two distinct literal occurrences.
# Full prior selected pay rows remain in source occurrence order.
program['expressions'].extend([
 {'kind':'Literal','value':U(0)}, {'kind':'Literal','value':U(0)}])
program['actions'].append({'name':'declareDuties','inputs':[],'statements':[
 {'kind':'Liability','name':'auditDuty','asset':'AUX','debtor':'observer','creditor':'fee_recipient','amount':7},
 {'kind':'Liability','name':'pay-beneficiary','asset':'UNIT','debtor':'reserve','creditor':'beneficiary','amount':8}]})
program_hash=H(program)
plan={'format':'moriarty-stage-plan/1','program_hash':program_hash,'actions':[{
 'action':'pay','statement_count':7,
 'sites':[{'site':1,'ordinal':3,'label':'release','binders':[{'name':'ready','ty':'Bool'}],'captures':[]}],
 'segment_cost':['7','8'],'committed_before':['0','7'],'plan_total':'15'}]}
# This unselected action is planned completely but never run in the pay sequence.
# Two distinct literal-zero statements cost2 each; no awaits or captures.
plan['actions'].append({'action':'declareDuties','statement_count':2,'sites':[],
 'segment_cost':['4'],'committed_before':['0'],'plan_total':'4'})
plan_hash=H(plan)
(P/'EXPECTED-PROGRAM-CANONICAL.json').write_bytes(canonical(program))
(P/'EXPECTED-PLAN-CANONICAL.json').write_bytes(canonical(plan))
emit('EXPECTED-PROGRAM.json',program)
emit('EXPECTED-PLAN.json',plan)
# Every Snapshot row is listed, including unrelated state, asset/account rows and duty.
# Parameter substitution abbreviates only the six independently derived fixed literals below.
def snapshot(reserve, beneficiary, fee, due):
 return {'state':{'flag':B(True),'marker':U(77)},'balances':[
 {'account':'observer','asset':AUX,'amount':'13'},
 {'account':'reserve','asset':AUX,'amount':'9'},
 {'account':'beneficiary','asset':UNIT,'amount':str(beneficiary)},
 {'account':'fee_recipient','asset':UNIT,'amount':str(fee)},
 {'account':'observer','asset':UNIT,'amount':'11'},
 {'account':'reserve','asset':UNIT,'amount':str(reserve)}],
 'liabilities':[{'name':'auditDuty','asset':AUX,'debtor':'observer','creditor':'fee_recipient','amount':'5'},
 {'name':'pay-beneficiary','asset':UNIT,'debtor':'reserve','creditor':'beneficiary','amount':str(due)}]}
s0=snapshot(100,0,0,40)
s1=snapshot(70,30,0,40)
s2=snapshot(68,30,2,40)
s3=snapshot(68,30,2,10)
s4=snapshot(58,40,2,10)
s5=snapshot(57,40,3,10)
s6=snapshot(57,40,3,0)
e0={'kind':'Transfer','asset':UNIT,'from':'reserve','to':'beneficiary','amount':'30'}
e1={'kind':'Fee','asset':UNIT,'from':'reserve','to':'fee_recipient','amount':'2'}
e2={'kind':'Liability','name':'pay-beneficiary','asset':UNIT,'debtor':'reserve','creditor':'beneficiary','before':'40','after':'10'}
e4={'kind':'Transfer','asset':UNIT,'from':'reserve','to':'beneficiary','amount':'10'}
e5={'kind':'Fee','asset':UNIT,'from':'reserve','to':'fee_recipient','amount':'1'}
e6={'kind':'Liability','name':'pay-beneficiary','asset':UNIT,'debtor':'reserve','creditor':'beneficiary','before':'10','after':'0'}
def step(index,value,state,effects):
 return {'index':index,'statement':statements[index],'value':value,'snapshot':state,'effects':effects}
start_execution={'semantics':'moriarty-core/2','action':'pay','pre':s0,'post':s3,
 'steps':[step(0,U(30),s1,[e0]),step(1,U(2),s2,[e1]),step(2,U(30),s3,[e2])],
 'effects':[e0,e1,e2]}
true_execution={'semantics':'moriarty-core/2','action':'pay','pre':s3,'post':s6,
 'steps':[step(3,B(True),s3,[]),step(4,U(10),s4,[e4]),step(5,U(1),s5,[e5]),step(6,U(10),s6,[e6])],
 'effects':[e4,e5,e6]}
idle={'site':0,'captures':[]}; waiting={'site':1,'captures':[]}
start_segment={'action':'pay','frame_pre':idle,'frame_post':waiting,'execution':start_execution}
true_segment={'action':'pay','frame_pre':waiting,'frame_post':idle,'execution':true_execution}
continuation={'format':'moriarty-continuation/1','program_hash':program_hash,'plan_hash':plan_hash,
 'action':'pay','site':1,'captures':[],
 'logical':{'profile':'moriarty-reference-logical-cost/2','cap':None,'committed':'7'},'boundary':s3}
start_outcome={'Waiting':{'advanced':True,'segment':start_segment,'continuation':continuation,'attempted':'7','residual':s3['liabilities']}}
false_outcome={'Waiting':{'advanced':False,'segment':None,'continuation':continuation,'attempted':'2','residual':s3['liabilities']}}
true_outcome={'Complete':{'segment':true_segment,'committed':'15','attempted':'8','residual':[s6['liabilities'][0]]}}
# Complete aggregation cells are literal per-account values, not produced by account_effects.
def exact(n): return {'Exact':str(n)}
def account(name, td,tr,fd,fr,gd,gr,net):
 return {'asset':UNIT,'account':name,'transfer_debit':exact(td),'transfer_receipts':exact(tr),
 'fee_debit':exact(fd),'fee_receipts':exact(fr),'gross_debit':exact(gd),'gross_receipts':exact(gr),'net_flow':net}
start_accounting={'PublishedSegment':{'effects':[e0,e1,e2],'accounts':[
 account('reserve',30,0,2,0,32,0,{'Outflow':'32'}),
 account('beneficiary',0,30,0,0,0,30,{'Inflow':'30'}),
 account('fee_recipient',0,0,0,2,0,2,{'Inflow':'2'})]}}
true_accounting={'PublishedSegment':{'effects':[e4,e5,e6],'accounts':[
 account('reserve',10,0,1,0,11,0,{'Outflow':'11'}),
 account('beneficiary',0,10,0,0,0,10,{'Inflow':'10'}),
 account('fee_recipient',0,0,0,1,0,1,{'Inflow':'1'})]}}
for name,outcome,accounting,pub in [
 ('START',start_outcome,start_accounting,'PublishedSuccessfulSegment'),
 ('FALSE-AWAIT',false_outcome,'NoPublishedSegment','NoAdvancedSegment'),
 ('TRUE-RESUME',true_outcome,true_accounting,'PublishedSuccessfulSegment')]:
 emit('EXPECTED-'+name+'-RESULT.json',{'Ok':outcome})
 emit('EXPECTED-'+name+'-ACCOUNTING.json',accounting)
 emit('EXPECTED-'+name+'-TENTATIVE-PUBLICATION.json',pub)
scenario=json.loads((P/'inputs/scenario.json').read_text())
inv=json.loads((P/'inputs/invocations.json').read_text())
inputs={
 'start':{'Start':{'action':'pay','scenario':scenario,'cap':None}},
 'false_await':{'Resume':{'continuation':continuation,'event':inv['events'][0],'cli_action':None,'cli_cap':None}},
 'true_resume':{'Resume':{'continuation':continuation,'event':inv['events'][1],'cli_action':None,'cli_cap':None}}}
observations={}
for name,outcome,accounting,pub in [('start',start_outcome,start_accounting,'PublishedSuccessfulSegment'),('false_await',false_outcome,'NoPublishedSegment','NoAdvancedSegment'),('true_resume',true_outcome,true_accounting,'PublishedSuccessfulSegment')]:
 observations[name]={'program_ref':'program_context','input':inputs[name],'result':{'Ok':outcome},
 'outcome':outcome,'accounting':accounting,'tentative_publication':pub}
work={'profile':'moriarty-reference-logical-cost/2',
 'start':{'statement_indices':[0,1,2],'statement_costs':['2','2','2'],'await_entry_charge':'1','attempted':'7','committed_before':'0','committed_after':'7','emitted_steps':3},
 'false_await':{'statement_indices_evaluated':[3],'statement_costs':['2'],'attempted':'2','committed_before':'7','committed_after':'7','published_segment':None,'returned_continuation_identical':True},
 'true_resume':{'statement_indices':[3,4,5,6],'statement_costs':['2','2','2','2'],'attempted':'8','committed_before':'7','committed_after':'15','emitted_steps':4},
 'chosen_invocation_attempt_total':'17','final_committed_total':'15',
 'universal_financial_work':{'status':'unavailable','reason':'unselected-universal-financial-schedule'},
 'raw_machine_work':{'status':'unavailable','reason':'not-carried-by-Core2-stage-result'},
 'target_fee_dust_circuit_cost':{'status':'unmeasured','reason':'not-observed-no-target-execution'}}
emit('EXPECTED.json',{
 'format':'r175-independent-typed-financial-expectations/1',
 'representation':'Own typed logical mapping for borrowed non-Serialize InvocationObservation/FinancialObservation; result/accounting/publication files separately match their actual serde declarations. No new production wire is claimed.',
 'program_context':{'program':program,'plan':plan,'program_hash':program_hash,'plan_hash':plan_hash},
 'initial':s0,'committed_first_prefix':s3,'final_complete':s6,
 'published_source_steps':[s1,s2,s3,s3,s4,s5,s6],
 'observations':observations,'work':work,
 'local_failure_observation':{
 'status':'PARAMETRIC_ONLY_NO_DIAGNOSTIC_INPUT_SELECTED',
 'input_committed_prefix':s3,
 'retained_positive_residual':s3['liabilities'],
 'result_constraint':'Err(StageRejection{error:E, attempted:A, committed:7}) for an existing actual resumed invocation at this prefix; E/A must come from independently bound diagnostic input, not invented here.',
 'accounting':'RejectedNoPublishedDelta','tentative_publication':'UnavailableFromStageRejection',
 'raw_error_expected':{'status':'named-gap','id':'LOCAL-FAILURE-ERROR-INPUT'},
 'attempted_work_expected':{'status':'named-gap','id':'LOCAL-FAILURE-ATTEMPTED-WORK-INPUT'},
 'failed_tentative_snapshot':{'status':'unavailable','reason':'StageRejection-has-no-snapshot-field'},
 'failed_tentative_effects':{'status':'unavailable','reason':'StageRejection-has-no-effects-field'},
 'returned_continuation':{'status':'not-returned','reason':'StageRejection-has-no-continuation-field'},
 'captured_invocation_context':'Retain separately exact program/input continuation/event/cli action/cap; no fabricated returned rollback continuation or global ledger rollback.'},
 'accounting_cross_checks':{'first_payment':'30','first_source_fee':'2','first_discharge':'30','first_reserve':'68','first_liability':'10','first_gross_debit':'32','first_receipts':'30',
 'final_reserve':'57','final_liability_row_present':True,'final_liability_amount':'0','cumulative_payment':'40','cumulative_source_fees':'3','cumulative_gross_debit':'43','cumulative_receipts':'40',
 'audit_duty_preserved':'5','UNIT_all_listed_balance_total':'111','AUX_all_listed_balance_total':'22',
 'note':'AccountingAvailability itself reports only current published segment: resume gross11, not cumulative43.'},
 'selected_positive_expected_gaps':[],
 'local_failure_input_gaps':['LOCAL-FAILURE-ERROR-INPUT','LOCAL-FAILURE-ATTEMPTED-WORK-INPUT','LOCAL-FAILURE-EXACT-INVOCATION-CONTEXT'],
})
emit('EXPECTED-WORK.json',work)
print(json.dumps({'program_hash':program_hash,'plan_hash':plan_hash,'source_work':['7','2','8'],'attempt_total':'17','committed':'15'}))
# Retrospective committed/working distinctions from complete successful segment Steps.
# These are logical observations, not a new live tentative-state API or continuation.
state_views=[
 {'id':'initial','committed':s0,'tentative':s0,'committed_work':'0','attempted_segment_work':'0','effects':[]},
 {'id':'start-after-transfer-before-publication','committed':s0,'tentative':s1,'committed_work':'0','attempted_segment_work':'2','effects':[e0]},
 {'id':'start-after-fee-before-publication','committed':s0,'tentative':s2,'committed_work':'0','attempted_segment_work':'4','effects':[e0,e1]},
 {'id':'start-after-discharge-before-await-entry','committed':s0,'tentative':s3,'committed_work':'0','attempted_segment_work':'6','effects':[e0,e1,e2]},
 {'id':'first-paid-await-prefix','committed':s3,'tentative':s3,'committed_work':'7','attempted_segment_work':'7','effects':[]},
 {'id':'false-await-preserves-prefix','committed':s3,'tentative':s3,'committed_work':'7','attempted_segment_work':'2','effects':None},
 {'id':'true-await-guard-before-next-payment','committed':s3,'tentative':s3,'committed_work':'7','attempted_segment_work':'2','effects':[]},
 {'id':'resume-after-transfer-before-publication','committed':s3,'tentative':s4,'committed_work':'7','attempted_segment_work':'4','effects':[e4]},
 {'id':'resume-after-fee-before-publication','committed':s3,'tentative':s5,'committed_work':'7','attempted_segment_work':'6','effects':[e4,e5]},
 {'id':'resume-after-discharge-before-complete-publication','committed':s3,'tentative':s6,'committed_work':'7','attempted_segment_work':'8','effects':[e4,e5,e6]},
 {'id':'complete-published','committed':s6,'tentative':s6,'committed_work':'15','attempted_segment_work':'8','effects':[]},
]
emit('EXPECTED-STATE-PROJECTIONS.json',{
 'representation':'Logical retrospective committed/working views using full successful execution snapshots; not a new production Snapshot serializer or observable failed tentative payload.',
 'views':state_views,
 'effects_null_meaning':'No published segment at false await, not an empty successful execution.',
 'local_failure':'No failed tentative state/effects are returned by StageRejection; preserve input committed prefix separately, keep actual attempted work, and leave missing diagnostic input explicit.'})
