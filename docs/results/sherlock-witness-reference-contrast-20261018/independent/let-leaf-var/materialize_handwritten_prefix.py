"""Explicit independently hand-derived A39 rows, data expansion only."""
import copy,hashlib,json
from pathlib import Path
D=Path(__file__).resolve().parent
def c(x):return copy.deepcopy(x)
def U(n):return {'UInt':n}
def V(n):return {'Var':n}
def Ev(e):return {'Eval':e}
def Ret(v):return {'Return':v}
def Push(f,n,e):return {'Push':{'frame':f,'next':n,'environment':e}}
leaf={'Tree':{'Leaf':3}}
unary={'Unary':{'op':'Leaf','operand':V('x')}}
let={'Let':{'name':'x','value':{'Literal':U(3)},'body':unary}}
arms=[{'pattern':{'Var':'whole'},'body':V('whole')}]
entry={'Match':{'value':let,'arms':arms}}
program={'functions':[],'entry':entry}
matchF={'Match':{'arms':arms,'saved_environment':[]}}
letF={'Let':{'name':'x','body':unary,'saved_environment':[]}}
scope={'Scope':{'saved_environment':[]}}
unaryF={'Unary':{'op':'Leaf'}}
xe=[{'name':'x','value':U(3)}]
we=[{'name':'whole','value':leaf}]
rows=[]
def row(rule,control,env,frames):rows.append((rule,c(control),c(env),c(frames)))
# The rule number belongs to the transition entering this row. Every charged
# transition below names the exact standalone K-A normative source line.
row(None,Ev(entry),[],[])
row(126,Push(matchF,Ev(let),[]),[],[])
row(135,Ev(let),[],[matchF])
row(114,Push(letF,Ev({'Literal':U(3)}),[]),[],[matchF])
row(135,Ev({'Literal':U(3)}),[],[letF,matchF])
row(102,Ret(U(3)),[],[letF,matchF])
row(145,Ev(unary),xe,[scope,matchF])
row(120,Push(unaryF,Ev(V('x')),xe),xe,[scope,matchF])
row(135,Ev(V('x')),xe,[unaryF,scope,matchF])
row(108,Ret(U(3)),xe,[unaryF,scope,matchF])
row(157,{'PrimitiveResult':{'Ok':leaf}},xe,[scope,matchF])
row(223,Ret(leaf),xe,[scope,matchF])
row(195,Ret(leaf),[],[matchF])
row(173,{'Cases':{'value':leaf,'arms':arms,'saved_environment':[]}},[],[])
row(214,{'CaseResult':{'bindings':we,'value':leaf,'body':V('whole'),'remaining_arms':[],'saved_environment':[]}},[],[])
row(220,Push(scope,Ev(V('whole')),we),[],[])
row(135,Ev(V('whole')),we,[scope])
row(108,Ret(leaf),we,[scope])
row(195,Ret(leaf),[],[])
row(80,None,[],[])
snapshots=[];records=[];prefix=[];fuel=4096;work=0
for i,(rule,control,environment,frames) in enumerate(rows):
    if i:
        charge=0 if rule==80 else 1;fuel-=charge;work+=charge
        records.append({'rule_start_line':rule,'charge':charge,'before_snapshot':i-1,'after_snapshot':i})
    snapshots.append({'program':program,'control':control,'environment':environment,'frames_top_first':frames,'stack_limit':64,'remaining_fuel':fuel,'initial_fuel':4096,'outcome':{'Done':leaf} if control is None else 'Pending'})
    prefix.append(work)
old=json.loads((D.parent/'UIREQ035-independent-reference-prefix-oracle-20261018/expectations/factorial3-complete-A39-prefix.json').read_text())
out={k:old[k] for k in ['profile','comparison_domain','snapshot_format','record_representation','B_absolute_work']}
out.update({'complete_program':program,'snapshots':snapshots,'records':records,'prefix_work_A':prefix,'charged_A_work':work,'A_event_count':len(records),'terminal_outcome':{'Done':leaf},'logical_depth_peak_A':max(len(s['frames_top_first']) for s in snapshots)})
p=D/'expectations/let-leaf-pattern-var-complete-A39-prefix.json';p.parent.mkdir(exist_ok=True);p.write_text(json.dumps(out,indent=2)+'\n')
assert len(snapshots)==len(records)+1
assert all(s['initial_fuel']-s['remaining_fuel']==w for s,w in zip(snapshots,prefix))
assert snapshots[-1]['environment']==[] and snapshots[-1]['frames_top_first']==[]
proof={'status':'PASS_DATA_INTEGRITY_ONLY','snapshots':len(snapshots),'records':len(records),'chargedAWork':work,'allEightSnapshotFields':True,'exactFullProgramEverySnapshot':True,'topFirstFramesAndSavedEnvironments':True,'newEvaluatorOrSubjectRun':False}
(D/'DATA-PROOF.json').write_text(json.dumps(proof,indent=2)+'\n')
