"""R3 fresh literal AST, plan and expected reports. No Moriarty imports/calls.

Only the eight allowed fixture inputs, original source premise bytes for hashes,
and the three reviewed projection documents are read. No previous oracle or
author expectation is used by this assembly.
"""
from pathlib import Path
import hashlib
import json

OUT = Path(__file__).parent
B = OUT.parent
WT = Path('/home/charl/Moriarty/.worktrees/mori-test-stages-r3-20261016')
INPUT = WT/'models/core-stage-tests'

def sha(data): return hashlib.sha256(data).hexdigest()
def canon(obj): return json.dumps(obj,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()
def write(name,obj): (OUT/name).write_text(json.dumps(obj,indent=2,sort_keys=True)+'\n')
def u(n): return {'type':'UInt','value':str(n)}
def v(name,value): return {'name':name,'value':value}
def flag(value): return {'type':'Bool','value':value}

pins = {'two-deliveries.mori':'b1f89331a3992e8f5135d9af6361a2347d0b45c0be74b0962c63ceb48de640c7',
        'quoted-fee.mori':'20ae6dc774eff97ea77707bcbddd451d4d381f0f779abcee6f035d842bf6252c'}
for name,pin in pins.items():
    if sha((INPUT/name).read_bytes()) != pin: raise SystemExit('ABORT_SOURCE_PIN_MISMATCH '+name)

A = {'network':'local','issuer':'independent-staged-test-oracle','reference':'UNIT','scale':0}
labels = {'qualification':'local-reference-stage','authentication':'none','consumption':'untracked'}

# Hand reconstruction from the two revised sources. The last literal and last
# action are global Program members even though declare is never selected.
td = [
 {'kind':'Transfer','asset':'UNIT','from':'alice','to':'bob','amount':0},
 {'kind':'Next','state':'phase','value':1},
 {'kind':'Await','label':'approval','binders':[{'name':'ok','ty':'Bool'}],'guard':2},
 {'kind':'Transfer','asset':'UNIT','from':'alice','to':'bob','amount':3},
 {'kind':'Await','label':'receipt','binders':[{'name':'seen','ty':'Bool'}],'guard':4},
 {'kind':'Next','state':'phase','value':5},
 {'kind':'Emit','name':'done','value':6}]
qc = [
 {'kind':'Let','name':'payable','value':2},
 {'kind':'Await','label':'quote','binders':[{'name':'agreed','ty':'Bool'}],'guard':3},
 {'kind':'Fee','asset':'UNIT','from':'payer','to':'treasury','amount':4},
 {'kind':'Next','state':'total','value':5},
 {'kind':'Emit','name':'fee','value':6}]
dt = {'kind':'Liability','name':'duty','asset':'UNIT','debtor':'alice','creditor':'bob','amount':7}
dq = {'kind':'Liability','name':'duty','asset':'UNIT','debtor':'payer','creditor':'treasury','amount':7}
tp = {'language':'moriarty/2','semantics':'moriarty-core/2','name':'TwoDeliveries',
 'states':[{'name':'phase','ty':'UInt','initial':u(0)}],
 'assets':[{'name':'UNIT','identity':A}],
 'expressions':[{'kind':'Literal','value':u(7)},{'kind':'Literal','value':u(1)},
 {'kind':'Name','name':'ok'},{'kind':'Literal','value':u(3)},{'kind':'Name','name':'seen'},
 {'kind':'Literal','value':u(2)},{'kind':'Name','name':'phase'},{'kind':'Literal','value':u(0)}],
 'actions':[{'name':'deliver','inputs':[],'statements':td},{'name':'declare','inputs':[],'statements':[dt]}]}
qp = {'language':'moriarty/2','semantics':'moriarty-core/2','name':'QuotedFee',
 'states':[{'name':'total','ty':'UInt','initial':u(0)},{'name':'marker','ty':'UInt','initial':u(9)}],
 'assets':[{'name':'UNIT','identity':A}],
 'expressions':[{'kind':'Name','name':'amount'},{'kind':'Literal','value':u(1)},
 {'kind':'Binary','op':'Add','lhs':0,'rhs':1},{'kind':'Name','name':'agreed'},
 {'kind':'Name','name':'payable'},{'kind':'Name','name':'payable'},{'kind':'Name','name':'payable'},
 {'kind':'Literal','value':u(0)}],
 'actions':[{'name':'charge','inputs':[{'name':'amount','ty':'UInt'}],'statements':qc},
 {'name':'declare','inputs':[],'statements':[dq]}]}
declare_plan = {'action':'declare','statement_count':1,'sites':[],
                'segment_cost':['2'],'committed_before':['0'],'plan_total':'2'}
tl = {'format':'moriarty-stage-plan/1','program_hash':sha(canon(tp)),'actions':[
 {'action':'deliver','statement_count':7,'sites':[
  {'site':1,'ordinal':2,'label':'approval','binders':[{'name':'ok','ty':'Bool'}],'captures':[]},
  {'site':2,'ordinal':4,'label':'receipt','binders':[{'name':'seen','ty':'Bool'}],
   'captures':[{'name':'ok','ty':'Bool','origin':'Event','ordinal':2}]}],
  'segment_cost':['5','5','6'],'committed_before':['0','5','10'],'plan_total':'16'},declare_plan]}
ql = {'format':'moriarty-stage-plan/1','program_hash':sha(canon(qp)),'actions':[
 {'action':'charge','statement_count':5,'sites':[
  {'site':1,'ordinal':1,'label':'quote','binders':[{'name':'agreed','ty':'Bool'}],
   'captures':[{'name':'amount','ty':'UInt','origin':'Input','ordinal':0},
               {'name':'payable','ty':'UInt','origin':'Let','ordinal':0}]}],
  'segment_cost':['5','8'],'committed_before':['0','5'],'plan_total':'13'},declare_plan]}

def duty(debtor,creditor,n): return {'name':'duty','asset':A,'debtor':debtor,'creditor':creditor,'amount':str(n)}
tduty = duty('alice','bob',5)
qduty = duty('payer','treasury',3)
def snap(state,rows,d):
    return {'state':state,'balances':[{'account':name,'asset':A,'amount':str(n)} for name,n in rows],'liabilities':[d]}
t0 = snap({'phase':u(0)},[('alice',20),('bob',0),('observer',11),('treasury',0)],tduty)
t7 = snap({'phase':u(0)},[('alice',13),('bob',7),('observer',11),('treasury',0)],tduty)
t1 = snap({'phase':u(1)},[('alice',13),('bob',7),('observer',11),('treasury',0)],tduty)
t2 = snap({'phase':u(1)},[('alice',10),('bob',10),('observer',11),('treasury',0)],tduty)
t3 = snap({'phase':u(2)},[('alice',10),('bob',10),('observer',11),('treasury',0)],tduty)
q0 = snap({'marker':u(9),'total':u(0)},[('observer',11),('payer',20),('treasury',0)],qduty)
qf = snap({'marker':u(9),'total':u(0)},[('observer',11),('payer',15),('treasury',5)],qduty)
q1 = snap({'marker':u(9),'total':u(5)},[('observer',11),('payer',15),('treasury',5)],qduty)

e7 = {'kind':'Transfer','asset':A,'from':'alice','to':'bob','amount':'7'}
e3 = {'kind':'Transfer','asset':A,'from':'alice','to':'bob','amount':'3'}
phase1 = {'kind':'State','name':'phase','before':u(0),'after':u(1)}
phase2 = {'kind':'State','name':'phase','before':u(1),'after':u(2)}
done = {'kind':'Emit','name':'done','value':u(2)}
fee5 = {'kind':'Fee','asset':A,'from':'payer','to':'treasury','amount':'5'}
total5 = {'kind':'State','name':'total','before':u(0),'after':u(5)}
emit5 = {'kind':'Emit','name':'fee','value':u(5)}
idle = {'site':0,'captures':[]}
approval = {'site':1,'captures':[]}
receipt = {'site':2,'captures':[v('ok',flag(True))]}
quote = {'site':1,'captures':[v('amount',u(4)),v('payable',u(5))]}

def step(i,stmt,value,snapshot,effects):
    return {'index':i,'statement':stmt,'value':value,'snapshot':snapshot,'effects':effects}
def seg(action,frame_pre,frame_post,pre,post,steps,effects):
    return {'action':action,'frame_pre':frame_pre,'frame_post':frame_post,
            'execution':{'semantics':'moriarty-core/2','action':action,'pre':pre,'post':post,'steps':steps,'effects':effects}}
s0 = seg('deliver',idle,approval,t0,t1,[step(0,td[0],u(7),t7,[e7]),step(1,td[1],u(1),t1,[phase1])],[e7,phase1])
s1 = seg('deliver',approval,receipt,t1,t2,[step(2,td[2],flag(True),t1,[]),step(3,td[3],u(3),t2,[e3])],[e3])
s2 = seg('deliver',receipt,idle,t2,t3,[step(4,td[4],flag(True),t2,[]),step(5,td[5],u(2),t3,[phase2]),step(6,td[6],u(2),t3,[done])],[phase2,done])
qs0 = seg('charge',idle,quote,q0,q0,[step(0,qc[0],u(5),q0,[])],[])
qs1 = seg('charge',quote,idle,q0,q1,[step(1,qc[1],flag(True),q0,[]),step(2,qc[2],u(5),qf,[fee5]),step(3,qc[3],u(5),q1,[total5]),step(4,qc[4],u(5),q1,[emit5])],[fee5,total5,emit5])

def cont(program,plan,action,frame,boundary,committed,cap):
    return {'format':'moriarty-continuation/1','program_hash':sha(canon(program)),'plan_hash':sha(canon(plan)),
            'action':action,'site':frame['site'],'captures':frame['captures'],'boundary':boundary,
            'logical':{'profile':'moriarty-reference-logical-cost/2','cap':cap,'committed':str(committed)}}
c1 = cont(tp,tl,'deliver',approval,t1,5,'16')
c2 = cont(tp,tl,'deliver',receipt,t2,10,'16')
cq = cont(qp,ql,'charge',quote,q0,5,None)
def wait(segment,continuation,attempted,d,advanced=True):
    return {**labels,'status':'waiting','advanced':advanced,'segment':segment,'continuation':continuation,
            'attempted':str(attempted),'committed':continuation['logical']['committed'],'residual':[d]}
def complete(segment,attempted,committed,d):
    return {**labels,'status':'complete','segment':segment,'attempted':str(attempted),'committed':str(committed),'residual':[d]}
reports = {
 'two-deliveries-start.json':wait(s0,c1,5,tduty),
 'two-deliveries-observed.json':wait(None,c1,2,tduty,False),
 'two-deliveries-approved.json':wait(s1,c2,5,tduty),
 'two-deliveries-complete.json':complete(s2,6,16,tduty),
 'quoted-fee-start.json':wait(qs0,cq,5,qduty),
 'quoted-fee-complete.json':complete(qs1,8,13,qduty)}
for name,data in reports.items(): write(name,data)
digests = {}
for stem,program,plan in [('two-deliveries',tp,tl),('quoted-fee',qp,ql)]:
    for kind,data in [('program',program),('plan',plan)]:
        name = stem+'-'+kind+'-preimage.json'
        (OUT/name).write_bytes(canon(data))
        digests[name] = sha(canon(data))
write('canonical-digests.json',digests)

inputs = ['two-deliveries.mori','two-deliveries-scenario.json','approval-false.json','approval-true.json','receipt-true.json',
          'quoted-fee.mori','quoted-fee-scenario.json','quote-true.json']
write('input-sha256.json',{name:sha((INPUT/name).read_bytes()) for name in inputs})
write('input-sizes.json',{name:len((INPUT/name).read_bytes()) for name in inputs})
source_paths = ['crates/mori-core/src/stage/'+n+'.rs' for n in ['decode','admit','validate','profile','plan','cost','digest','mod']]
source_paths += ['crates/mori-core/src/ir.rs','crates/mori-core/src/parser.rs','crates/mori-core/src/intent.rs',
                 'crates/mori-core/src/json.rs','crates/mori-core/src/runtime.rs','crates/mori-core/README.md',
                 'docs/CORE-CONTINUATIONS.md','AGENTS.md','docs/MORIARTY-PRODUCT-CONTRACT.md']
write('source-premise-sha256.json',{name:sha((WT/name).read_bytes()) for name in source_paths})
projection_paths = ['core-trace-contract-candidate-r1/CORE-TRACE-REPORT-CONTRACT.md',
                    'core-trace-contract-review-muse-r1/RESULT.md','core-trace-contract-review-opus-r1/RESULT.md']
write('projection-premise-sha256.json',{name:sha((B/name).read_bytes()) for name in projection_paths})

# Structural JSON data-size accounting is literal-data work, not parser or
# runtime admission. It gives no claim that a Moriarty decoder was exercised.
def shape(x,depth=0):
    children = list(x.values()) if isinstance(x,dict) else x if isinstance(x,list) else []
    sub = [shape(y,depth+1) for y in children]
    return (1+sum(a for a,d in sub),max([depth]+[d for a,d in sub]))
sizes = {}
for name,data in [('two-deliveries-first-continuation',c1),('two-deliveries-second-continuation',c2),('quoted-fee-continuation',cq)]:
    nodes,depth = shape(data)
    sizes[name] = {'compact_ascii_bytes':len(canon(data)),'json_value_nodes':nodes,'maximum_value_depth':depth,
                   'basis':'Independent Python literal JSON representation; no consumer decoder called.'}
write('continuation-literal-shape-premises.json',sizes)
write('normal-accounting-and-work.json',{
 'classification':'fresh independent normal expected data; no validity/security judgment',
 'two-deliveries':{'opening':t0,'after-transfer7':t7,'first-boundary':t1,'second-boundary':t2,'complete':t3,
 'effects-by-segment':[[e7,phase1],[e3],[phase2,done]],'segment-work':['5','5','6'],
 'committed-after-segment':['5','10','16'],'false-observation':{'attempted':'2','committed':'5','advanced':False},
 'cumulative-gross-debit-alice':['7','10','10'],'cumulative-fees':['0','0','0'],'cumulative-net-credit-bob':['7','10','10'],
 'liability':{'opening':'5','created':'0','accrued':'0','discharged':'0','closing':'5','present':True},
 'target-action-trace-records':7,'target-action-trace-cells':63,'all-balance-sums':'31'},
 'quoted-fee':{'opening':q0,'first-boundary':q0,'after-fee':qf,'complete':q1,
 'effects-by-segment':[[],[fee5,total5,emit5]],'payable':'5','segment-work':['5','8'],
 'committed-after-segment':['5','13'],'cumulative-gross-debit-payer':['0','5'],
 'cumulative-fees':['0','5'],'cumulative-net-credit-treasury':['0','5'],
 'liability':{'opening':'3','created':'0','accrued':'0','discharged':'0','closing':'3','present':True},
 'target-action-trace-records':7,'target-action-trace-cells':49,'all-balance-sums':'31'},
 'each-program-declare':{'expression-node':7,'expression':u(0),'statement-count':1,'sites':[],
                        'segment-work':['2'],'committed-before':['0'],'plan-total':'2','execution':'UNRUN_NOT_SELECTED'},
 'mint':'0','burn':'0','reserve-effects':[],'rounding-dust':'0',
 'unmeasured':['circuit cost','DUST','service fees','proof cost','recovery reservation','wall-clock','allocations'],
 'execution':'UNRUN','comparisons':'UNRUN'})

write('step-value-premise-map.json',[
 {'report':name,'pointer':f'/segment/execution/steps/{j}/value','source-index':st['index'],
  'kind':st['statement']['kind'],'category':{'Let':1,'Next':4,'Emit':5,'Transfer':6,'Fee':7,'Await':10}[st['statement']['kind']],
  'independently-derived-value':st['value'],'attachment-only-premise':'P-TRACE-PROJ/draft'}
 for name,r in reports.items() if r.get('segment') for j,st in enumerate(r['segment']['execution']['steps'])])
write('actual-call-premises.json',{'profile':'moriarty-tests/2',
 'cases':[{'name':'two-deliveries','source':'two-deliveries.mori','scenario':'two-deliveries-scenario.json',
           'action':'deliver','start-cap':'16','events':['approval-false.json','approval-true.json','receipt-true.json']},
          {'name':'quoted-fee','source':'quoted-fee.mori','scenario':'quoted-fee-scenario.json',
           'action':'charge','start-cap':None,'events':['quote-true.json']}],
 'resume-optional-action':'Some(case.action)','resume-cli-cap':None,
 'chain':'Immediately preceding actual waiting continuation; expectations never supply actual execution state.',
 'core1':'OUT_OF_SCOPE','backend-trace':'OUT_OF_SCOPE','execution':'UNRUN','comparisons':'UNRUN'})
