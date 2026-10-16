"""Literal-data transcription only: no parser/evaluator/compiler/subject imports.
Every source expression/statement, step value, financial snapshot, effect, frame
and work count is chosen explicitly from the pinned existing Core/2 contract.
Helper functions merely construct existing JSON record shapes and hashes.
"""
from pathlib import Path
import copy,hashlib,json
R=Path(__file__).resolve().parent

def write(name,data):
    p=R/name;p.parent.mkdir(parents=True,exist_ok=True)
    assert not p.exists(),f'new-only output exists: {p}'
    p.write_text(data if isinstance(data,str) else json.dumps(data,indent=2,ensure_ascii=False)+'\n')
def U(n): return {'type':'UInt','value':str(n)}
def B(b): return {'type':'Bool','value':b}
def name(n): return {'kind':'Name','name':n}
def literal(v): return {'kind':'Literal','value':v}
def binary(op,a,b): return {'kind':'Binary','op':op,'lhs':a,'rhs':b}
def statement(kind,**fields): return {'kind':kind,**fields}
def nv(n,v): return {'name':n,'value':v}
def asset(ref): return {'network':'oracle-local','issuer':'financial-core2-oracle','reference':ref,'scale':0}
PAY=asset('PAY'); CREDIT=asset('CREDIT'); OPS=asset('OPS')
def balance(a,id,n):return {'account':a,'asset':id,'amount':str(n)}
def duty(n,id,d,c,amount):return {'name':n,'asset':id,'debtor':d,'creditor':c,'amount':str(amount)}
def snapshot(state,balances,liabilities):return {'state':state,'balances':balances,'liabilities':liabilities}
def move(kind,id,f,t,n):return {'kind':kind,'asset':id,'from':f,'to':t,'amount':str(n)}
def liability(n,id,d,c,before,after):return {'kind':'Liability','name':n,'asset':id,'debtor':d,'creditor':c,'before':str(before),'after':str(after)}
def state(n,b,a):return {'kind':'State','name':n,'before':b,'after':a}
def emit(n,v):return {'kind':'Emit','name':n,'value':v}
def frame(site,captures):return {'site':site,'captures':captures}
IDLE=frame(0,[])
def canonical_hash(data):return hashlib.sha256(json.dumps(data,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()).hexdigest()

invoice_source='''language "moriarty/2";
agreement InstallmentInvoice {
  asset PAY = { network: "oracle-local", issuer: "financial-core2-oracle", reference: "PAY", scale: 0 };
  state paid: UInt = 0;
  state marker: Bool = true;
  action settle(invoice: UInt, upfront: UInt) {
    liability("invoiceDue", PAY, "buyer", "vendor", invoice);
    transfer(PAY, "buyer", "vendor", upfront);
    fee(PAY, "buyer", "collector", 2);
    discharge("invoiceDue", upfront);
    next paid = paid + upfront;
    let remaining = invoice - upfront;
    await acceptance(accepted: Bool) when accepted;
    transfer(PAY, "buyer", "vendor", remaining);
    fee(PAY, "buyer", "collector", 1);
    discharge("invoiceDue", remaining);
    next paid = paid + remaining;
    emit "remainingDue"(liability_amount("invoiceDue"));
    ensures paid == invoice;
  }
}
'''
grant_source='''language "moriarty/2";
agreement ConditionalGrantTranches {
  asset CREDIT = { network: "oracle-local", issuer: "financial-core2-oracle", reference: "CREDIT", scale: 0 };
  asset OPS = { network: "oracle-local", issuer: "financial-core2-oracle", reference: "OPS", scale: 0 };
  state allocated: UInt = 6;
  state marker: Bool = false;
  action allocate(first: UInt, second: UInt) {
    let opening = allocated;
    liability("grantDue", CREDIT, "vault", "recipient", first + second);
    transfer(CREDIT, "vault", "recipient", first);
    discharge("grantDue", first);
    next allocated = allocated + first;
    await review(approved: Bool, supplement: UInt) when approved and supplement == 2;
    liability("grantDue", CREDIT, "vault", "recipient", supplement);
    fee(OPS, "vault", "auditor", 1);
    next allocated = allocated + supplement;
    await release(released: Bool) when released;
    transfer(CREDIT, "vault", "recipient", second + supplement);
    discharge("grantDue", second + supplement);
    next allocated = allocated + second;
    emit "openingAllocation"(opening);
    emit "dutyPresent"(liability_present("grantDue"));
    ensures allocated == opening + first + supplement + second;
  }
}
'''
# Expression IDs transcribed manually from postorder append in the existing parser.
IE=[name('invoice'),name('upfront'),literal(U(2)),name('upfront'),name('paid'),name('upfront'),binary('Add',4,5),
 name('invoice'),name('upfront'),binary('Sub',7,8),name('accepted'),name('remaining'),literal(U(1)),name('remaining'),
 name('paid'),name('remaining'),binary('Add',14,15),{'kind':'LiabilityAmount','name':'invoiceDue'},name('paid'),name('invoice'),binary('Eq',18,19)]
IS=[statement('Liability',name='invoiceDue',asset='PAY',debtor='buyer',creditor='vendor',amount=0),
 statement('Transfer',asset='PAY',**{'from':'buyer','to':'vendor'},amount=1),
 statement('Fee',asset='PAY',**{'from':'buyer','to':'collector'},amount=2),
 statement('Discharge',name='invoiceDue',amount=3),statement('Next',state='paid',value=6),statement('Let',name='remaining',value=9),
 statement('Await',label='acceptance',binders=[{'name':'accepted','ty':'Bool'}],guard=10),
 statement('Transfer',asset='PAY',**{'from':'buyer','to':'vendor'},amount=11),
 statement('Fee',asset='PAY',**{'from':'buyer','to':'collector'},amount=12),statement('Discharge',name='invoiceDue',amount=13),
 statement('Next',state='paid',value=16),statement('Emit',name='remainingDue',value=17),statement('Ensures',predicate=20)]
GE=[name('allocated'),name('first'),name('second'),binary('Add',1,2),name('first'),name('first'),name('allocated'),name('first'),binary('Add',6,7),
 name('approved'),name('supplement'),literal(U(2)),binary('Eq',10,11),binary('And',9,12),name('supplement'),literal(U(1)),
 name('allocated'),name('supplement'),binary('Add',16,17),name('released'),name('second'),name('supplement'),binary('Add',20,21),
 name('second'),name('supplement'),binary('Add',23,24),name('allocated'),name('second'),binary('Add',26,27),name('opening'),
 {'kind':'LiabilityPresent','name':'grantDue'},name('allocated'),name('opening'),name('first'),binary('Add',32,33),
 name('supplement'),binary('Add',34,35),name('second'),binary('Add',36,37),binary('Eq',31,38)]
GS=[statement('Let',name='opening',value=0),
 statement('Liability',name='grantDue',asset='CREDIT',debtor='vault',creditor='recipient',amount=3),
 statement('Transfer',asset='CREDIT',**{'from':'vault','to':'recipient'},amount=4),statement('Discharge',name='grantDue',amount=5),
 statement('Next',state='allocated',value=8),
 statement('Await',label='review',binders=[{'name':'approved','ty':'Bool'},{'name':'supplement','ty':'UInt'}],guard=13),
 statement('Liability',name='grantDue',asset='CREDIT',debtor='vault',creditor='recipient',amount=14),
 statement('Fee',asset='OPS',**{'from':'vault','to':'auditor'},amount=15),statement('Next',state='allocated',value=18),
 statement('Await',label='release',binders=[{'name':'released','ty':'Bool'}],guard=19),
 statement('Transfer',asset='CREDIT',**{'from':'vault','to':'recipient'},amount=22),statement('Discharge',name='grantDue',amount=25),
 statement('Next',state='allocated',value=28),statement('Emit',name='openingAllocation',value=29),statement('Emit',name='dutyPresent',value=30),
 statement('Ensures',predicate=39)]
IP={'language':'moriarty/2','semantics':'moriarty-core/2','name':'InstallmentInvoice',
 'states':[{'name':'paid','ty':'UInt','initial':U(0)},{'name':'marker','ty':'Bool','initial':B(True)}],
 'assets':[{'name':'PAY','identity':PAY}],'expressions':IE,'actions':[{'name':'settle','inputs':[{'name':'invoice','ty':'UInt'},{'name':'upfront','ty':'UInt'}],'statements':IS}]}
GP={'language':'moriarty/2','semantics':'moriarty-core/2','name':'ConditionalGrantTranches',
 'states':[{'name':'allocated','ty':'UInt','initial':U(6)},{'name':'marker','ty':'Bool','initial':B(False)}],
 'assets':[{'name':'CREDIT','identity':CREDIT},{'name':'OPS','identity':OPS}],
 'expressions':GE,'actions':[{'name':'allocate','inputs':[{'name':'first','ty':'UInt'},{'name':'second','ty':'UInt'}],'statements':GS}]}
def slot(n,ty,origin,ordinal):return {'name':n,'ty':ty,'origin':origin,'ordinal':ordinal}
ICLAY=[slot('invoice','UInt','Input',0),slot('upfront','UInt','Input',1),slot('remaining','UInt','Let',5)]
GCLAY1=[slot('first','UInt','Input',0),slot('second','UInt','Input',1),slot('opening','UInt','Let',0)]
GCLAY2=GCLAY1+[slot('approved','Bool','Event',5),slot('supplement','UInt','Event',5)]
IH=canonical_hash(IP); GH=canonical_hash(GP)
IPL={'format':'moriarty-stage-plan/1','program_hash':IH,'actions':[{'action':'settle','statement_count':13,
 'sites':[{'site':1,'ordinal':6,'label':'acceptance','binders':[{'name':'accepted','ty':'Bool'}],'captures':ICLAY}],
 'segment_cost':['17','18'],'committed_before':['0','17'],'plan_total':'35'}]}
GPL={'format':'moriarty-stage-plan/1','program_hash':GH,'actions':[{'action':'allocate','statement_count':16,
 'sites':[{'site':1,'ordinal':5,'label':'review','binders':[{'name':'approved','ty':'Bool'},{'name':'supplement','ty':'UInt'}],'captures':GCLAY1},
 {'site':2,'ordinal':9,'label':'release','binders':[{'name':'released','ty':'Bool'}],'captures':GCLAY2}],
 'segment_cost':['15','15','28'],'committed_before':['0','15','30'],'plan_total':'58'}]}
IPH=canonical_hash(IPL); GPH=canonical_hash(GPL)
# Every financial boundary below is an independent literal. Helpers only fill
# unchanged declared fields; no balance/statement transition is calculated.
def inv_snap(paid,buyer,collector,vendor,owed):
 return snapshot({'marker':B(True),'paid':U(paid)},[balance('buyer',PAY,buyer),balance('collector',PAY,collector),balance('observer',PAY,17),balance('vendor',PAY,vendor)],[duty('invoiceDue',PAY,'buyer','vendor',owed)])
I0=inv_snap(0,100,3,5,5)
I1=inv_snap(0,100,3,5,45)
I2=inv_snap(0,85,3,20,45)
I3=inv_snap(0,83,5,20,45)
I4=inv_snap(0,83,5,20,30)
I5=inv_snap(15,83,5,20,30)
I6=inv_snap(15,58,5,45,30)
I7=inv_snap(15,57,6,45,30)
I8=inv_snap(15,57,6,45,5)
I9=inv_snap(40,57,6,45,5)
def grant_snap(allocated,creditvault,recipient,opsvault,auditor,owed):
 return snapshot({'allocated':U(allocated),'marker':B(False)},[
 balance('observer',CREDIT,11),balance('recipient',CREDIT,recipient),balance('vault',CREDIT,creditvault),
 balance('auditor',OPS,auditor),balance('observer',OPS,13),balance('vault',OPS,opsvault)],
 [] if owed is None else [duty('grantDue',CREDIT,'vault','recipient',owed)])
G0=grant_snap(6,90,1,9,4,None)
G1=grant_snap(6,90,1,9,4,30)
G2=grant_snap(6,78,13,9,4,30)
G3=grant_snap(6,78,13,9,4,18)
G4=grant_snap(18,78,13,9,4,18)
G5=grant_snap(18,78,13,9,4,20)
G6=grant_snap(18,78,13,8,5,20)
G7=grant_snap(20,78,13,8,5,20)
G8=grant_snap(20,58,33,8,5,20)
G9=grant_snap(20,58,33,8,5,0)
G10=grant_snap(38,58,33,8,5,0)
# Literal per-statement ordered effect objects.
IL=liability('invoiceDue',PAY,'buyer','vendor',5,45)
IT0=move('Transfer',PAY,'buyer','vendor',15); IF0=move('Fee',PAY,'buyer','collector',2)
ID0=liability('invoiceDue',PAY,'buyer','vendor',45,30); IN0=state('paid',U(0),U(15))
IT1=move('Transfer',PAY,'buyer','vendor',25); IF1=move('Fee',PAY,'buyer','collector',1)
ID1=liability('invoiceDue',PAY,'buyer','vendor',30,5); IN1=state('paid',U(15),U(40)); IE1=emit('remainingDue',U(5))
GL0=liability('grantDue',CREDIT,'vault','recipient',0,30); GT0=move('Transfer',CREDIT,'vault','recipient',12)
GD0=liability('grantDue',CREDIT,'vault','recipient',30,18); GN0=state('allocated',U(6),U(18))
GL1=liability('grantDue',CREDIT,'vault','recipient',18,20); GF1=move('Fee',OPS,'vault','auditor',1); GN1=state('allocated',U(18),U(20))
GT2=move('Transfer',CREDIT,'vault','recipient',20); GD2=liability('grantDue',CREDIT,'vault','recipient',20,0)
GN2=state('allocated',U(20),U(38)); GE20=emit('openingAllocation',U(6)); GE21=emit('dutyPresent',B(True))
def step(index,statements,value,snap,effects):return {'index':index,'statement':statements[index],'value':value,'snapshot':snap,'effects':effects}
IST0=[step(0,IS,U(40),I1,[IL]),step(1,IS,U(15),I2,[IT0]),step(2,IS,U(2),I3,[IF0]),step(3,IS,U(15),I4,[ID0]),step(4,IS,U(15),I5,[IN0]),step(5,IS,U(25),I5,[])]
IST1=[step(6,IS,B(True),I5,[]),step(7,IS,U(25),I6,[IT1]),step(8,IS,U(1),I7,[IF1]),step(9,IS,U(25),I8,[ID1]),step(10,IS,U(40),I9,[IN1]),step(11,IS,U(5),I9,[IE1]),step(12,IS,B(True),I9,[])]
GST0=[step(0,GS,U(6),G0,[]),step(1,GS,U(30),G1,[GL0]),step(2,GS,U(12),G2,[GT0]),step(3,GS,U(12),G3,[GD0]),step(4,GS,U(18),G4,[GN0])]
GST1=[step(5,GS,B(True),G4,[]),step(6,GS,U(2),G5,[GL1]),step(7,GS,U(1),G6,[GF1]),step(8,GS,U(20),G7,[GN1])]
GST2=[step(9,GS,B(True),G7,[]),step(10,GS,U(20),G8,[GT2]),step(11,GS,U(20),G9,[GD2]),step(12,GS,U(38),G10,[GN2]),step(13,GS,U(6),G10,[GE20]),step(14,GS,B(True),G10,[GE21]),step(15,GS,B(True),G10,[])]
IC=[nv('invoice',U(40)),nv('upfront',U(15)),nv('remaining',U(25))]
GC1=[nv('first',U(12)),nv('second',U(18)),nv('opening',U(6))]
GC2=GC1+[nv('approved',B(True)),nv('supplement',U(2))]
def execution(action,pre,post,steps,effects):return {'semantics':'moriarty-core/2','action':action,'pre':pre,'post':post,'steps':steps,'effects':effects}
def segment(action,pre,post,exe):return {'action':action,'frame_pre':pre,'frame_post':post,'execution':exe}
ISEG0=segment('settle',IDLE,frame(1,IC),execution('settle',I0,I5,IST0,[IL,IT0,IF0,ID0,IN0]))
ISEG1=segment('settle',frame(1,IC),IDLE,execution('settle',I5,I9,IST1,[IT1,IF1,ID1,IN1,IE1]))
GSEG0=segment('allocate',IDLE,frame(1,GC1),execution('allocate',G0,G4,GST0,[GL0,GT0,GD0,GN0]))
GSEG1=segment('allocate',frame(1,GC1),frame(2,GC2),execution('allocate',G4,G7,GST1,[GL1,GF1,GN1]))
GSEG2=segment('allocate',frame(2,GC2),IDLE,execution('allocate',G7,G10,GST2,[GT2,GD2,GN2,GE20,GE21]))
def continuation(action,site,captures,boundary,committed,cap,ph,plh):return {'format':'moriarty-continuation/1','program_hash':ph,'plan_hash':plh,'action':action,'site':site,'captures':captures,'logical':{'profile':'moriarty-reference-logical-cost/2','cap':str(cap),'committed':str(committed)},'boundary':boundary}
ICON=continuation('settle',1,IC,I5,17,35,IH,IPH)
GCON1=continuation('allocate',1,GC1,G4,15,58,GH,GPH)
GCON2=continuation('allocate',2,GC2,G7,30,58,GH,GPH)
def waiting(advanced,seg,c,attempted,residual):return {'Waiting':{'advanced':advanced,'segment':seg,'continuation':c,'attempted':str(attempted),'residual':residual}}
def complete(seg,committed,attempted,residual):return {'Complete':{'segment':seg,'committed':str(committed),'attempted':str(attempted),'residual':residual}}
IW=waiting(True,ISEG0,ICON,17,[duty('invoiceDue',PAY,'buyer','vendor',30)])
IFALSE=waiting(False,None,ICON,2,[duty('invoiceDue',PAY,'buyer','vendor',30)])
IDONE=complete(ISEG1,35,18,[duty('invoiceDue',PAY,'buyer','vendor',5)])
GW1=waiting(True,GSEG0,GCON1,15,[duty('grantDue',CREDIT,'vault','recipient',18)])
GFALSE1=waiting(False,None,GCON1,6,[duty('grantDue',CREDIT,'vault','recipient',18)])
GW2=waiting(True,GSEG1,GCON2,15,[duty('grantDue',CREDIT,'vault','recipient',20)])
GFALSE2=waiting(False,None,GCON2,2,[duty('grantDue',CREDIT,'vault','recipient',20)])
GDONE=complete(GSEG2,58,28,[])
def event(action,label,values):return {'format':'moriarty-stage-event/1','action':action,'label':label,'values':values}
IEF=event('settle','acceptance',[nv('accepted',B(False))]); IET=event('settle','acceptance',[nv('accepted',B(True))])
GEF1=event('allocate','review',[nv('approved',B(False)),nv('supplement',U(2))]); GET1=event('allocate','review',[nv('approved',B(True)),nv('supplement',U(2))])
GEF2=event('allocate','release',[nv('released',B(False))]); GET2=event('allocate','release',[nv('released',B(True))])
ISC={'state':{},'inputs':{'invoice':U(40),'upfront':U(15)},'balances':I0['balances'],'liabilities':I0['liabilities']}
GSC={'state':{},'inputs':{'first':U(12),'second':U(18)},'balances':G0['balances'],'liabilities':[]}

def invocation(identifier,operation,expected,**fields):return {'id':identifier,'operation':operation,**fields,'expected':expected}
def resume(identifier,from_id,c,e,outcome):return invocation(identifier,'resume',{'input':c,'event':e,'outcome':outcome},continuation_from=from_id,cli_action=None,cli_cap=None)
IINV=[invocation('start','start',IW),resume('not-yet-accepted','start',ICON,IEF,IFALSE),resume('accepted','not-yet-accepted',ICON,IET,IDONE)]
GINV=[invocation('start','start',GW1),resume('review-not-ready','start',GCON1,GEF1,GFALSE1),resume('review-ready','review-not-ready',GCON1,GET1,GW2),resume('release-not-ready','review-ready',GCON2,GEF2,GFALSE2),resume('release-ready','release-not-ready',GCON2,GET2,GDONE)]
# Pure input files keep actual execution selection separate from expected data.
write('inputs/installment-invoice.mori',invoice_source)
write('inputs/conditional-grant-tranches.mori',grant_source)
write('inputs/invoice-scenario.json',ISC);write('inputs/grant-scenario.json',GSC)
for path,data in [('invoice-false-event.json',IEF),('invoice-true-event.json',IET),('grant-review-false-event.json',GEF1),('grant-review-true-event.json',GET1),('grant-release-false-event.json',GEF2),('grant-release-true-event.json',GET2)]:write('inputs/'+path,data)
input_cases=[{'case_id':'F01','source':'inputs/installment-invoice.mori','action':'settle','scenario':'inputs/invoice-scenario.json','start_cap':'35',
 'sequence':[{'id':'start','operation':'start'},{'id':'not-yet-accepted','operation':'resume','continuation_from':'start','event':'inputs/invoice-false-event.json','cli_action':None,'cli_cap':None},{'id':'accepted','operation':'resume','continuation_from':'not-yet-accepted','event':'inputs/invoice-true-event.json','cli_action':None,'cli_cap':None}]},
 {'case_id':'F02','source':'inputs/conditional-grant-tranches.mori','action':'allocate','scenario':'inputs/grant-scenario.json','start_cap':'58',
 'sequence':[{'id':'start','operation':'start'},{'id':'review-not-ready','operation':'resume','continuation_from':'start','event':'inputs/grant-review-false-event.json','cli_action':None,'cli_cap':None},{'id':'review-ready','operation':'resume','continuation_from':'review-not-ready','event':'inputs/grant-review-true-event.json','cli_action':None,'cli_cap':None},{'id':'release-not-ready','operation':'resume','continuation_from':'review-ready','event':'inputs/grant-release-false-event.json','cli_action':None,'cli_cap':None},{'id':'release-ready','operation':'resume','continuation_from':'release-not-ready','event':'inputs/grant-release-true-event.json','cli_action':None,'cli_cap':None}]}]
write('inputs.json',{'format':'sherlock-independent-core2-financial-inputs/1','status':'source-only; specified-only/unexecuted','source_profile':'moriarty/2','semantics':'moriarty-core/2','production_consumer':'mori_core::stage::load/start/resume','cases':input_cases,'target':None,'target_status':'unsupplied; oracle-local is a declared fictional asset-domain input, not a network/target observation'})
expected_cases=[{'case_id':'F01','source_sha256':hashlib.sha256(invoice_source.encode()).hexdigest(),'program_ir':IP,'program_hash':IH,'stage_plan':IPL,'plan_hash':IPH,'invocations':IINV,
 'work_schedule':{'profile':'moriarty-reference-logical-cost/2','successful_segment_costs':['17','18'],'committed_before':['0','17'],'plan_total':'35','statement_costs_on_execution':['2','2','2','2','4','4','2','2','2','2','4','2','4'],'await_entry_charges':[{'statement_index':6,'charge':'1'}],'false_guard_attempts':[{'site':1,'attempted':'2','committed_unchanged':'17'}]}},
 {'case_id':'F02','source_sha256':hashlib.sha256(grant_source.encode()).hexdigest(),'program_ir':GP,'program_hash':GH,'stage_plan':GPL,'plan_hash':GPH,'invocations':GINV,
 'work_schedule':{'profile':'moriarty-reference-logical-cost/2','successful_segment_costs':['15','15','28'],'committed_before':['0','15','30'],'plan_total':'58','statement_costs_on_execution':['2','4','2','2','4','6','2','2','4','2','4','4','4','2','2','10'],'await_entry_charges':[{'statement_index':5,'charge':'1'},{'statement_index':9,'charge':'1'}],'false_guard_attempts':[{'site':1,'attempted':'6','committed_unchanged':'15'},{'site':2,'attempted':'2','committed_unchanged':'30'}]}}]
write('expected.json',{'format':'sherlock-independent-core2-financial-expectations/1','status':'hand-derived source-only; specified-only/unexecuted','predicate':'Later admitted same-production-consumer observations for both cases equal these complete typed literals; final-value agreement is insufficient.',
 'representation':'Externally tagged Waiting/Complete is the existing StageOutcome serialization. New financial view has no wire ABI: Root must bind these same fields to typed expectations without discarding fields. Resume expected object retains full input continuation and event separately from outcome.',
 'publication':'All Step snapshots/effects in a successful segment are segment-local tentative observations until that segment publishes. A published Waiting/Complete segment is a successful local prefix. A false guard has segment:null and no added prefix. No technical proof cut creates a financial commit.',
 'cases':expected_cases,
 'unmeasured':{key:{'value':None,'status':'unmeasured','reason':reason} for key,reason in [
 ('preparation_work','Parsing/validation/planning/hashing/encoding are outside the reference logical-cost profile.'),('host_time_memory_disk','No subject execution or measurement performed.'),('helper_or_sherlock_A_B_work','Core/2 logical work is not Sherlock K-A fuel or a B helper schedule.'),('circuit_rows_constraints_witness','No circuit representation or execution selected.'),('DUST','No target/native action or fee measurement performed.'),('service_fee','Only source fee effects are specified; no service price/fee exists in this input.'),('proof_cost','No proof generation/verification performed.'),('recovery_reservation','No recovery policy/reservation specified by these sources.'),('target_identity','No qualified target tuple supplied; declared fictional asset IDs do not identify a live network.')]},
 'acceptance':'none','security_judgment':'none; Opus-owned','remaining_coverage':['No rejected/tentative-failure observation authored; security/hostile cases remain with Opus.','No actual compiler/lowering/target observation, universal Commit/Yield/Await lowering, full grammar correspondence or K/Quint/Lean/Clean/circuit/native result.','No authenticated event origin/consumption/history or settlement qualification; existing local stage labels remain controlling.'], 'existing_cli_labels':{'qualification':'local-reference-stage','authentication':'none','consumption':'untracked'}, 'existing_cli_labels_scope':'These are existing CLI/report qualifications, not added fields or a new wire ABI on the borrowed typed financial view.'})
# Accounting is supplemental literal arithmetic tied to the ordered effects;
# the projection view does not synthesize these aggregates.
accounting={'format':'sherlock-independent-core2-financial-accounting/1','status':'hand-derived, unexecuted; source fee units are financial atomic units, not work/DUST',
 'cases':[
 {'case_id':'F01','published_segments':[
  {'invocation':'start','gross_debits':[{'asset':PAY,'account':'buyer','amount':'17'}],'fee_debits':[{'asset':PAY,'account':'buyer','amount':'2'}],'transfer_receipts':[{'asset':PAY,'account':'vendor','amount':'15'}],'fee_receipts':[{'asset':PAY,'account':'collector','amount':'2'}],'duty_evolution':[{'name':'invoiceDue','opening':'5','created_or_accrued':'40','discharged':'15','closing':'30'}]},
  {'invocation':'accepted','gross_debits':[{'asset':PAY,'account':'buyer','amount':'26'}],'fee_debits':[{'asset':PAY,'account':'buyer','amount':'1'}],'transfer_receipts':[{'asset':PAY,'account':'vendor','amount':'25'}],'fee_receipts':[{'asset':PAY,'account':'collector','amount':'1'}],'duty_evolution':[{'name':'invoiceDue','opening':'30','created_or_accrued':'0','discharged':'25','closing':'5'}]}],
  'cumulative':{'gross_debits':[{'asset':PAY,'account':'buyer','amount':'43'}],'fee_debits':[{'asset':PAY,'account':'buyer','amount':'3'}],'transfer_receipts':[{'asset':PAY,'account':'vendor','amount':'40'}],'fee_receipts':[{'asset':PAY,'account':'collector','amount':'3'}],'net_balance_changes':[{'asset':PAY,'account':'buyer','signed_amount':'-43'},{'asset':PAY,'account':'collector','signed_amount':'3'},{'asset':PAY,'account':'observer','signed_amount':'0'},{'asset':PAY,'account':'vendor','signed_amount':'40'}],'duty_evolution':[{'name':'invoiceDue','opening':'5','created_or_accrued':'40','discharged':'40','closing':'5'}],'residual':[duty('invoiceDue',PAY,'buyer','vendor',5)],'preserved_state':{'marker':B(True)},'preserved_balances':[balance('observer',PAY,17)]}},
 {'case_id':'F02','published_segments':[
  {'invocation':'start','gross_debits':[{'asset':CREDIT,'account':'vault','amount':'12'}],'fee_debits':[],'transfer_receipts':[{'asset':CREDIT,'account':'recipient','amount':'12'}],'fee_receipts':[],'duty_evolution':[{'name':'grantDue','opening':'0','opening_presence':'absent','created_or_accrued':'30','discharged':'12','closing':'18'}]},
  {'invocation':'review-ready','gross_debits':[{'asset':OPS,'account':'vault','amount':'1'}],'fee_debits':[{'asset':OPS,'account':'vault','amount':'1'}],'transfer_receipts':[],'fee_receipts':[{'asset':OPS,'account':'auditor','amount':'1'}],'duty_evolution':[{'name':'grantDue','opening':'18','created_or_accrued':'2','discharged':'0','closing':'20'}]},
  {'invocation':'release-ready','gross_debits':[{'asset':CREDIT,'account':'vault','amount':'20'}],'fee_debits':[],'transfer_receipts':[{'asset':CREDIT,'account':'recipient','amount':'20'}],'fee_receipts':[],'duty_evolution':[{'name':'grantDue','opening':'20','created_or_accrued':'0','discharged':'20','closing':'0','closing_presence':'present'}]}],
  'cumulative':{'gross_debits':[{'asset':CREDIT,'account':'vault','amount':'32'},{'asset':OPS,'account':'vault','amount':'1'}],'fee_debits':[{'asset':OPS,'account':'vault','amount':'1'}],'transfer_receipts':[{'asset':CREDIT,'account':'recipient','amount':'32'}],'fee_receipts':[{'asset':OPS,'account':'auditor','amount':'1'}],'net_balance_changes':[{'asset':CREDIT,'account':'observer','signed_amount':'0'},{'asset':CREDIT,'account':'recipient','signed_amount':'32'},{'asset':CREDIT,'account':'vault','signed_amount':'-32'},{'asset':OPS,'account':'auditor','signed_amount':'1'},{'asset':OPS,'account':'observer','signed_amount':'0'},{'asset':OPS,'account':'vault','signed_amount':'-1'}],'duty_evolution':[{'name':'grantDue','opening':'0','opening_presence':'absent','created_or_accrued':'32','discharged':'32','closing':'0','closing_presence':'present'}],'residual':[],'retained_present_zero':[duty('grantDue',CREDIT,'vault','recipient',0)],'preserved_state':{'marker':B(False)},'preserved_balances':[balance('observer',CREDIT,11),balance('observer',OPS,13)]}}],
 'false_observations':'No financial balance/effect/duty or committed-work delta; attempted dispatch/guard work is reported separately and is not erased or added to committed semantic prefix work.',
 'liability_presence':'Opening absent zero is not a present-zero row. Full discharge leaves the existing row with amount0; positive-only residual omits it.'}
write('accounting.json',accounting)
print(json.dumps({'status':'serialized-source-literals-only','cases':2,'invocations':8,'published_segments':5,'steps':29,'effects':22,'source_hashes':{'F01':expected_cases[0]['source_sha256'],'F02':expected_cases[1]['source_sha256']},'program_hashes':{'F01':IH,'F02':GH},'plan_hashes':{'F01':IPH,'F02':GPH}},indent=2))
