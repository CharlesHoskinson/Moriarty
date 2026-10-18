"""Literal independent financial oracle preparation, not a parser/evaluator.
No product imports/execution. All boundary/step/effect/work values below are manual.
SHA256 is applied only to manually transcribed supplied literal program/plan data.
"""
from pathlib import Path
import json,hashlib,copy
W=Path('/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016');R=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017');D=Path(__file__).resolve().parent;F=W/'crates/mori-core/tests/fixtures/financial-observation';E=D/'expected';E.mkdir(exist_ok=True)
def dump(p,x):p.write_text(json.dumps(x,indent=2,ensure_ascii=False)+'\n')
def pin(p):
 p=Path(p);b=p.read_bytes();return dict(path=str(p),bytes=len(b),sha256=hashlib.sha256(b).hexdigest())
def uint(n):return dict(type='UInt',value=str(n))
def boolean(b):return dict(type='Bool',value=b)
def canon(x):return json.dumps(x,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()
# Manually transcribed source IR: state initial literals do not occupy expression rows.
UNIT=dict(network='local',issuer='r151-financial-observation',reference='UNIT',scale=0);AUX=dict(network='local',issuer='r151-financial-observation',reference='AUX',scale=0)
pay=[dict(kind='Transfer',asset='UNIT',**{'from':'reserve'},to='beneficiary',amount=0),dict(kind='Fee',asset='UNIT',**{'from':'reserve'},to='fee_recipient',amount=1),dict(kind='Discharge',name='pay-beneficiary',amount=2),dict(kind='Await',label='release',binders=[dict(name='ready',ty='Bool')],guard=3),dict(kind='Transfer',asset='UNIT',**{'from':'reserve'},to='beneficiary',amount=4),dict(kind='Fee',asset='UNIT',**{'from':'reserve'},to='fee_recipient',amount=5),dict(kind='Discharge',name='pay-beneficiary',amount=6)]
program=dict(language='moriarty/2',semantics='moriarty-core/2',name='RetainedPaymentObservation',states=[dict(name='marker',ty='UInt',initial=uint(77)),dict(name='flag',ty='Bool',initial=boolean(True))],assets=[dict(name='UNIT',identity=UNIT),dict(name='AUX',identity=AUX)],expressions=[dict(kind='Literal',value=uint(30)),dict(kind='Literal',value=uint(2)),dict(kind='Literal',value=uint(30)),dict(kind='Name',name='ready'),dict(kind='Literal',value=uint(10)),dict(kind='Literal',value=uint(1)),dict(kind='Literal',value=uint(10)),dict(kind='Literal',value=uint(0)),dict(kind='Literal',value=uint(0))],actions=[dict(name='pay',inputs=[],statements=pay),dict(name='declareDuties',inputs=[],statements=[dict(kind='Liability',name='auditDuty',asset='AUX',debtor='observer',creditor='fee_recipient',amount=7),dict(kind='Liability',name='pay-beneficiary',asset='UNIT',debtor='reserve',creditor='beneficiary',amount=8)])])
programhash=hashlib.sha256(canon(program)).hexdigest()
plan=dict(format='moriarty-stage-plan/1',program_hash=programhash,actions=[dict(action='pay',statement_count=7,sites=[dict(site=1,ordinal=3,label='release',binders=[dict(name='ready',ty='Bool')],captures=[])],segment_cost=['7','8'],committed_before=['0','7'],plan_total='15'),dict(action='declareDuties',statement_count=2,sites=[],segment_cost=['4'],committed_before=['0'],plan_total='4')]);planhash=hashlib.sha256(canon(plan)).hexdigest()
scenario=json.loads((F/'scenario.json').read_text());pre={k:copy.deepcopy(scenario[k]) for k in ['state','balances','liabilities']}
def boundary(reserve,beneficiary,fee,liability):
 x=copy.deepcopy(pre);x['balances'][2]['amount']=str(beneficiary);x['balances'][3]['amount']=str(fee);x['balances'][5]['amount']=str(reserve);x['liabilities'][1]['amount']=str(liability);return x
b_transfer=boundary(70,30,0,40);b_fee=boundary(68,30,2,40);first=boundary(68,30,2,10);b_transfer2=boundary(58,40,2,10);b_fee2=boundary(57,40,3,10);final=boundary(57,40,3,0)
effects1=[dict(kind='Transfer',asset=UNIT,**{'from':'reserve'},to='beneficiary',amount='30'),dict(kind='Fee',asset=UNIT,**{'from':'reserve'},to='fee_recipient',amount='2'),dict(kind='Liability',name='pay-beneficiary',asset=UNIT,debtor='reserve',creditor='beneficiary',before='40',after='10')]
effects2=[dict(kind='Transfer',asset=UNIT,**{'from':'reserve'},to='beneficiary',amount='10'),dict(kind='Fee',asset=UNIT,**{'from':'reserve'},to='fee_recipient',amount='1'),dict(kind='Liability',name='pay-beneficiary',asset=UNIT,debtor='reserve',creditor='beneficiary',before='10',after='0')]
steps1=[dict(index=0,statement=pay[0],value=uint(30),snapshot=b_transfer,effects=[effects1[0]]),dict(index=1,statement=pay[1],value=uint(2),snapshot=b_fee,effects=[effects1[1]]),dict(index=2,statement=pay[2],value=uint(30),snapshot=first,effects=[effects1[2]])]
steps2=[dict(index=3,statement=pay[3],value=boolean(True),snapshot=first,effects=[]),dict(index=4,statement=pay[4],value=uint(10),snapshot=b_transfer2,effects=[effects2[0]]),dict(index=5,statement=pay[5],value=uint(1),snapshot=b_fee2,effects=[effects2[1]]),dict(index=6,statement=pay[6],value=uint(10),snapshot=final,effects=[effects2[2]])]
idle=dict(site=0,captures=[]);waiting=dict(site=1,captures=[])
segment1=dict(action='pay',frame_pre=idle,frame_post=waiting,execution=dict(semantics='moriarty-core/2',action='pay',pre=pre,post=first,steps=steps1,effects=effects1))
segment2=dict(action='pay',frame_pre=waiting,frame_post=idle,execution=dict(semantics='moriarty-core/2',action='pay',pre=first,post=final,steps=steps2,effects=effects2))
# Exact logical profile literal comes from the public profile contract, not runtime output.
logicalprofile='moriarty-reference-logical-cost/2'
continuation=dict(format='moriarty-continuation/1',program_hash=programhash,plan_hash=planhash,action='pay',site=1,captures=[],logical=dict(profile=logicalprofile,cap=None,committed='7'),boundary=first)
start=dict(Ok=dict(Waiting=dict(advanced=True,segment=segment1,continuation=continuation,attempted='7',residual=first['liabilities'])))
false=dict(Ok=dict(Waiting=dict(advanced=False,segment=None,continuation=continuation,attempted='2',residual=first['liabilities'])))
true=dict(Ok=dict(Complete=dict(segment=segment2,committed='15',attempted='8',residual=[final['liabilities'][0]])))
def exact(n):return {'Exact':str(n)}
def flow(account,td,tr,fd,fr,gd,gr,net):return dict(asset=UNIT,account=account,transfer_debit=exact(td),transfer_receipts=exact(tr),fee_debit=exact(fd),fee_receipts=exact(fr),gross_debit=exact(gd),gross_receipts=exact(gr),net_flow=net)
accounting1={'PublishedSegment':dict(effects=effects1,accounts=[flow('reserve',30,0,2,0,32,0,{'Outflow':'32'}),flow('beneficiary',0,30,0,0,0,30,{'Inflow':'30'}),flow('fee_recipient',0,0,0,2,0,2,{'Inflow':'2'})])}
accounting2={'PublishedSegment':dict(effects=effects2,accounts=[flow('reserve',10,0,1,0,11,0,{'Outflow':'11'}),flow('beneficiary',0,10,0,0,0,10,{'Inflow':'10'}),flow('fee_recipient',0,0,0,1,0,1,{'Inflow':'1'})])}
inputs=dict(action='pay',start_cap=None,resume_cli_action=None,resume_cli_cap=None,events=[dict(format='moriarty-stage-event/1',action='pay',label='release',values=[dict(name='ready',value=boolean(False))]),dict(format='moriarty-stage-event/1',action='pay',label='release',values=[dict(name='ready',value=boolean(True))])])
for name,value in [('PROGRAM',program),('PLAN',plan),('START-RESULT',start),('FALSE-RESULT',false),('TRUE-RESULT',true),('START-ACCOUNTING',accounting1),('FALSE-ACCOUNTING','NoPublishedSegment'),('TRUE-ACCOUNTING',accounting2),('START-PUBLICATION','PublishedSuccessfulSegment'),('FALSE-PUBLICATION','NoAdvancedSegment'),('TRUE-PUBLICATION','PublishedSuccessfulSegment'),('INPUTS',inputs)]:dump(E/(name+'.json'),value)
summary=dict(opening_reserve='100',opening_liability='40',first_payment='30',first_fee='2',first_discharge='30',first_reserve='68',first_liability='10',first_gross_debit='32',first_recipient_receipt='30',first_residual_duty='10',second_payment='10',second_fee='1',second_discharge='10',final_reserve='57',final_liability_present_zero='0',cumulative_gross_debit='43',cumulative_recipient_receipt='40',cumulative_fee_receipt='3',unaffected_audit_duty='5',attempted_by_invocation=['7','2','8'],committed_after_invocation=['7','7','15'],cumulative_attempted='17',committed='15',first_statement_costs=['2','2','2','1'],second_statement_costs=['2','2','2','2'],program_hash=programhash,plan_hash=planhash)
dump(D/'DERIVED-FINANCIAL-FACTS.json',summary)
print(programhash,planhash)
