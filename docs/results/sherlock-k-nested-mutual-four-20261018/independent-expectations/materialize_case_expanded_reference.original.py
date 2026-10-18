"""Independent case-expanded DATA derivation from safe source/public contracts.

This does not import/read/execute the K/Quint/Clean/Rust subject or its adapter.
There is deliberately no generic AST evaluator/step dispatcher. Explicit source
callbacks below spell the four selected positive paths; form macros materialize
their full before/after continuation states and declaration-named event payloads.
"""
import copy,json,hashlib
from pathlib import Path
D=Path(__file__).resolve().parent
R=D.parent
INPUT=R/'dedicated-quint-20261018/batch09-nested-patterns/SAFE-NESTED-PATTERN-INPUTS.json'
API_PATH=R/'generic-nominal-let-if-source-successor-20261018/PUBLIC-API.json'
api=json.loads(API_PATH.read_text());inputs=json.loads(INPUT.read_text())
def cp(x):return copy.deepcopy(x)
def pin(p):
    b=p.read_bytes();return {'path':str(p),'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()}
def value(v):
    if 'Nominal'not in v:return cp(v)
    n=v['Nominal'];return nominal(n['typeId'],n['ctorId'],*[value(f) for f in n['fields']])
def nominal(t,k,*fields):return {'Nominal':{'type_id':t,'ctor_id':k,'fields':list(fields)}}
def uint(n):return {'UInt':n}
def field(v,i):return v['Nominal']['fields'][i]
def ctor(v):return v['Nominal']['ctor_id']
def env(d):return [{'name_id':n,'value':v} for n,v in sorted(d.items())]
def vtype(v):return 0 if 'UInt'in v else 1 if 'Bool'in v else v['Nominal']['type_id']
def pat(p):
    if 'Constructor'not in p:return cp(p)
    c=p['Constructor'];return {'Constructor':{'type_id':c['typeId'],'ctor_id':c['ctorId'],'fields':[pat(f) for f in c['fields']]}}
def program(case):
    expressions=[]
    for x in case['expressions']:
        k=x['kind']
        if k=='Literal':payload={'Literal':value(x['value'])}
        elif k=='Variable':payload={'Variable':{'name_id':x['name']}}
        elif k=='Let':payload={'Let':{'binder':x['binder'],'bound_expr':x['valueExpr'],'body_expr':x['bodyExpr']}}
        elif k=='Constructor':payload={'Construct':{'type_id':x['typeId'],'ctor_id':x['ctorId'],'fields':x['fields']}}
        elif k=='Call':payload={'Call':{'name_id':x['function'],'arguments':x['args']}}
        elif k=='Match':payload={'Match':{'scrutinee':x['scrutineeExpr'],'arms':[{'source_arm_id':a['sourceArmId'],'pattern':pat(a['pattern']),'body':a['bodyExpr']} for a in x['arms']]}}
        else:
            assert k=='Binary' and x['op']=='Add'
            payload={'Add':{'left':x['leftExpr'],'right':x['rightExpr']}}
        expressions.append({'id':x['exprId'],'occurrence':x['occurrence'],'kind':payload})
    return {'declarations':[{'type_id':d['typeId'],'recursive_group':d['recursiveGroup'],'ctors':[{'type_id':d['typeId'],'ctor_id':c['ctorId'],'source_order':c['sourceOrder'],'field_types':c['fieldTypes']} for c in d['constructors']]} for d in case['declarations']], 'expressions':expressions,'functions':[{'name_id':f['name'],'parameters':[{'name_id':p['name'],'type_id':p['typeId']} for p in f['parameters']],'return_type_id':f['returnType'],'body':f['bodyExpr']} for f in case['functions']],'entry':case['entry']}

class Tape:
    def __init__(self,case):
        self.case=case;self.program=program(case);self.nodes={e['exprId']:e for e in case['expressions']};self.functions={f['name']:f for f in case['functions']}
        self.layouts={(d['typeId'],c['ctorId']):c['fieldTypes'] for d in case['declarations'] for c in d['constructors']}
        self.current={e['name']:value(e['value']) for e in case['initial_environment']};self.frames=[];self.events=[];self.states=[];self.control=None
    def tag(self,category,name,*payload):
        keys=api[category+'_payload_order'][name];assert len(keys)==len(payload),(name,keys,payload)
        return {name:dict(zip(keys,payload))}
    def C(self,name,*p):return self.tag('control',name,*p)
    def F(self,name,*p):return self.tag('frame',name,*p)
    def E(self,name,*p):return self.tag('event',name,*p)
    def emit(self,control,event=None):
        self.control=control
        if event is not None:self.events.append(event)
        self.states.append(cp({'program':self.program,'control':control,'environment':env(self.current),'frames_top_first':self.frames,'events':self.events}))
    def expect_eval(self,i):assert self.control==self.C('ngEval',i)
    def expect_return(self,v):assert self.control==self.C('ngReturn',v)
    def literal(self,i):
        self.expect_eval(i);e=self.nodes[i];assert e['kind']=='Literal';v=value(e['value'])
        self.emit(self.C('ngReturn',v),self.E('ngLiteralEvaluated',i,e['occurrence'],v));return v
    def variable(self,i):
        self.expect_eval(i);e=self.nodes[i];assert e['kind']=='Variable';v=cp(self.current[e['name']])
        self.emit(self.C('ngReturn',v),self.E('ngVariableRead',i,e['occurrence'],e['name'],v));return v
    def let(self,i,bound_script,body_script):
        self.expect_eval(i);e=self.nodes[i];assert e['kind']=='Let';old=cp(self.current);occ=e['occurrence'];b=e['valueExpr'];body=e['bodyExpr'];name=e['binder']
        self.frames.insert(0,self.F('ngLetValue',i,occ,name,b,body,env(old)))
        self.emit(self.C('ngEval',b),self.E('ngLetStarted',i,occ,name,b,body,env(old)))
        v=bound_script();self.expect_return(v);self.frames[0]=self.F('ngLetScope',i,occ,name,env(old));self.current=cp(old);self.current[name]=v
        self.emit(self.C('ngEval',body),self.E('ngLetBound',i,occ,name,b,self.nodes[b]['occurrence'],v,body,env(old),env(self.current)))
        result=body_script();self.expect_return(result);self.frames.pop(0);self.current=old
        self.emit(self.C('ngReturn',result),self.E('ngLetBodyReturned',i,occ,name,result,env(old)));return result
    def construct(self,i,child_scripts):
        self.expect_eval(i);e=self.nodes[i];assert e['kind']=='Constructor';ids=e['fields'];t=e['typeId'];k=e['ctorId'];occ=e['occurrence'];types=self.layouts[t,k];old=cp(self.current);prefix=[]
        assert len(ids)==len(child_scripts)==len(types)
        if ids:
            self.frames.insert(0,self.F('ngConstructorFields',i,occ,t,k,types,0,ids[0],[],ids[1:],env(old)))
            control=self.C('ngEval',ids[0])
        else:control=self.C('ngMaterialize',i,occ,t,k,[])
        self.emit(control,self.E('ngConstructorStarted',i,occ,t,k,ids,types,env(old)))
        for cursor,script in enumerate(child_scripts):
            v=script();self.expect_return(v);assert vtype(v)==types[cursor];prefix.append(v);self.current=cp(old)
            remaining=ids[cursor+2:] if cursor+1<len(ids) else []
            if cursor+1<len(ids):
                self.frames[0]=self.F('ngConstructorFields',i,occ,t,k,types,cursor+1,ids[cursor+1],cp(prefix),remaining,env(old));control=self.C('ngEval',ids[cursor+1])
            else:self.frames.pop(0);control=self.C('ngMaterialize',i,occ,t,k,cp(prefix))
            self.emit(control,self.E('ngConstructorFieldReturned',i,occ,cursor,ids[cursor],self.nodes[ids[cursor]]['occurrence'],v,cp(prefix),remaining,env(old)))
        result=nominal(t,k,*prefix);self.current=old
        self.emit(self.C('ngReturn',result),self.E('ngConstructorBuilt',i,occ,t,k,result,env(old)));return result
    def call(self,i,arg_scripts,body_script):
        self.expect_eval(i);e=self.nodes[i];assert e['kind']=='Call';ids=e['args'];name=e['function'];fn=self.functions[name];occ=e['occurrence'];old=cp(self.current);args=[]
        assert ids and len(ids)==len(arg_scripts)==len(fn['parameters'])
        self.frames.insert(0,self.F('ngArguments',i,occ,name,0,ids[0],[],ids[1:],env(old)))
        self.emit(self.C('ngEval',ids[0]),self.E('ngCallStarted',i,occ,name,ids,env(old)))
        for cursor,script in enumerate(arg_scripts):
            v=script();self.expect_return(v);assert vtype(v)==fn['parameters'][cursor]['typeId'];args.append(v);self.current=cp(old)
            remaining=ids[cursor+2:] if cursor+1<len(ids) else []
            if cursor+1<len(ids):
                self.frames[0]=self.F('ngArguments',i,occ,name,cursor+1,ids[cursor+1],cp(args),remaining,env(old));control=self.C('ngEval',ids[cursor+1])
            else:self.frames.pop(0);control=self.C('ngApply',i,occ,name,cp(args))
            self.emit(control,self.E('ngArgumentReturned',i,occ,cursor,ids[cursor],v,cp(args),remaining,env(old)))
        self.current={p['name']:v for p,v in zip(fn['parameters'],args)}
        self.frames.insert(0,self.F('ngFunctionReturn',i,occ,name,fn['returnType'],env(old)))
        self.emit(self.C('ngEval',fn['bodyExpr']),self.E('ngFunctionEntered',i,occ,name,args,env(self.current),fn['returnType']))
        result=body_script();self.expect_return(result);assert vtype(result)==fn['returnType'];self.frames.pop(0);self.current=old
        self.emit(self.C('ngReturn',result),self.E('ngFunctionReturned',i,occ,name,result,env(old)));return result
    def match(self,i,scrutinee_script,chosen_index,bindings,body_script):
        self.expect_eval(i);e=self.nodes[i];assert e['kind']=='Match';old=cp(self.current);occ=e['occurrence'];scr=e['scrutineeExpr'];arms=next(x for x in self.program['expressions'] if x['id']==i)['kind']['Match']['arms']
        self.frames.insert(0,self.F('ngScrutinee',i,occ,arms,env(old)))
        self.emit(self.C('ngEval',scr),self.E('ngMatchStarted',i,occ,scr,arms,env(old)))
        v=scrutinee_script();self.expect_return(v);self.frames.pop(0);self.current=cp(old)
        self.emit(self.C('ngChoose',i,occ,v,arms,env(old)),self.E('ngScrutineeReturned',i,occ,v,arms,env(old)))
        for arm_index in range(chosen_index):
            self.emit(self.C('ngChoose',i,occ,v,arms[arm_index+1:],env(old)),self.E('ngArmMiss',i,occ,arms[arm_index]['source_arm_id'],v))
        arm=arms[chosen_index];arm_id=arm['source_arm_id'];witnesses=[]
        for name,path,bound_value in bindings:
            witnesses.append({'name_id':name,'match_expr_id':i,'source_arm_id':arm_id,'field_path':path,'value':bound_value});self.current[name]=bound_value
        self.frames.insert(0,self.F('ngArmScope',i,occ,arm_id,env(old)))
        self.emit(self.C('ngEval',arm['body']),self.E('ngArmSelected',i,occ,arm_id,witnesses,arm['body'],env(self.current)))
        result=body_script();self.expect_return(result);self.frames.pop(0);self.current=old
        self.emit(self.C('ngReturn',result),self.E('ngArmBodyReturned',i,occ,arm_id,result,env(old)));return result
    def add(self,i,left_script,right_script):
        self.expect_eval(i);e=self.nodes[i];assert e['kind']=='Binary' and e['op']=='Add';left=e['leftExpr'];right=e['rightExpr'];occ=e['occurrence'];old=cp(self.current)
        self.frames.insert(0,self.F('ngAddLeft',i,occ,right,env(old)))
        self.emit(self.C('ngEval',left),self.E('ngAddStarted',i,occ,left,right,env(old)))
        a=left_script();self.expect_return(a);self.current=cp(old);self.frames[0]=self.F('ngAddRight',i,occ,a,env(old))
        self.emit(self.C('ngEval',right),self.E('ngAddRightRequested',i,occ,a,right,env(old)))
        b=right_script();self.expect_return(b);assert 'UInt'in a and 'UInt'in b;result=uint(a['UInt']+b['UInt']);self.frames.pop(0);self.current=old
        self.emit(self.C('ngReturn',result),self.E('ngAdded',i,occ,a,b,result));return result

# Complete case-expanded source paths. These callbacks name the exact original
# input expression IDs, source arms and binder paths; there is no generic AST
# dispatch or peer-machine execution.
def mutual_case(t):
    def A():
        seed=cp(t.current[10])
        if ctor(seed)==500:return t.match(100,lambda:t.variable(101),0,[],lambda:t.construct(102,[]))
        assert ctor(seed)==501
        return t.match(100,lambda:t.variable(101),1,[(11,[0],field(seed,0)),(12,[1],field(seed,1))],lambda:t.construct(103,[lambda:t.variable(104),lambda:t.call(105,[lambda:t.variable(106)],B)]))
    def B():
        seed=cp(t.current[10])
        if ctor(seed)==510:return t.match(200,lambda:t.variable(201),0,[],lambda:t.construct(202,[]))
        assert ctor(seed)==511
        return t.match(200,lambda:t.variable(201),1,[(11,[0],field(seed,0)),(12,[1],field(seed,1))],lambda:t.construct(203,[lambda:t.call(204,[lambda:t.variable(206)],A),lambda:t.variable(205)]))
    def pack():return t.construct(300,[lambda:t.variable(301),lambda:t.variable(302),lambda:t.variable(303)])
    def inner():
        return t.let(2,lambda:t.call(3,[lambda:t.call(4,[lambda:t.literal(7)],A),lambda:t.call(5,[lambda:t.literal(8)],B),lambda:t.variable(6)],pack),lambda:t.construct(9,[lambda:t.variable(10),lambda:t.variable(11)]))
    return t.let(0,lambda:t.literal(1),inner)

def schedule_case(t):
    def Sum():
        seed=cp(t.current[10]);tag=ctor(seed)
        if tag==21:return t.match(200,lambda:t.variable(201),0,[],lambda:t.literal(202))
        bindings=[(11,[0],field(seed,0)),(12,[1],field(seed,1))]
        if tag==22:return t.match(200,lambda:t.variable(201),1,bindings,lambda:t.add(203,lambda:t.variable(204),lambda:t.call(205,[lambda:t.variable(206)],Sum)))
        assert tag==23
        return t.match(200,lambda:t.variable(201),2,bindings,lambda:t.add(207,lambda:t.call(208,[lambda:t.variable(209)],Sum),lambda:t.call(210,[lambda:t.variable(211)],Sum)))
    def First():
        seed=cp(t.current[10]);assert ctor(seed)==23
        if ctor(field(seed,0))==22:
            return t.match(100,lambda:t.variable(101),0,[(11,[0,0],field(field(seed,0),0))],lambda:t.variable(102))
        assert ctor(field(seed,0))==21
        return t.match(100,lambda:t.variable(101),1,[(12,[1],field(seed,1))],lambda:t.call(103,[lambda:t.variable(104)],Sum))
    return t.let(0,lambda:t.literal(1),lambda:t.construct(2,[lambda:t.call(3,[lambda:t.variable(5)],First),lambda:t.call(4,[lambda:t.variable(6)],Sum)]))

summary=[]
for case in inputs['cases']:
    t=Tape(case);initial=cp(t.current);t.emit(t.C('ngEval',case['entry']))
    result=mutual_case(t) if case['caseId'].startswith('structural-mutual') else schedule_case(t)
    assert t.current==initial and t.frames==[];t.expect_return(result)
    t.emit(t.C('ngHalted',result),t.E('ngFinished',result))
    assert len(t.states)<=513
    out={'schema':'independent-nominal-full-state/1','profile':'K-uncharged-nominal-let-if-successor','caseId':case['caseId'],'states':t.states,'terminalStepContract':'ngStep at final state is unchanged; not an extra advancing record','observerCuts':list(range(len(t.states)))}
    dest=D/'expected'/(case['caseId']+'.K.json');dest.parent.mkdir(exist_ok=True);dest.write_text(json.dumps(out,separators=(',',':'))+'\n')
    summary.append({'caseId':case['caseId'],'oracle':pin(dest),'snapshots':len(t.states),'events':len(t.events),'terminalValue':result})
(D/'PRIVATE-DERIVATION-SUMMARY.json').write_text(json.dumps({'status':'INDEPENDENT_SPECIFIED_ONLY_NO_SUBJECT_EXECUTION','source':pin(INPUT),'cases':summary},indent=2)+'\n')
