"""Independent local JSON relation checks; no derivation/subject imports."""
import hashlib,json
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parent
source=R/'dedicated-quint-20261018/batch09-nested-patterns/SAFE-NESTED-PATTERN-INPUTS.json'
cases=json.loads(source.read_text())['cases']
api=json.loads((R/'generic-nominal-let-if-source-successor-20261018/PUBLIC-API.json').read_text())
def enmap(e):
    assert [x['name_id'] for x in e]==sorted(set(x['name_id'] for x in e))
    return {x['name_id']:x['value'] for x in e}
def en(d):return [{'name_id':k,'value':v} for k,v in sorted(d.items())]
def normv(v):
    if 'Nominal'not in v:return v
    n=v['Nominal'];return {'Nominal':{'type_id':n['typeId'],'ctor_id':n['ctorId'],'fields':[normv(x) for x in n['fields']]}}
def source_pattern(p):
    if isinstance(p,list):return [source_pattern(x) for x in p]
    if isinstance(p,dict):return {({'typeId':'type_id','ctorId':'ctor_id'}.get(k,k)):source_pattern(v) for k,v in p.items()}
    return p
def tag(category,name,payload):
    assert list(payload)==api[category+'_payload_order'][name]
def vtype(v):return 0 if 'UInt'in v else 1 if 'Bool'in v else v['Nominal']['type_id']
def structural_match(pattern,value,path=()):
    # Independent structural DATA relation: preorder ordered binding/path rows.
    if 'Wildcard'in pattern:return []
    if 'Bind'in pattern:return [(pattern['Bind'],list(path),value)]
    p=pattern['Constructor'];v=value.get('Nominal')
    if v is None or (p['type_id'],p['ctor_id'],len(p['fields']))!=(v['type_id'],v['ctor_id'],len(v['fields'])):return None
    out=[]
    for i,(child,val) in enumerate(zip(p['fields'],v['fields'])):
        matched=structural_match(child,val,path+(i,))
        if matched is None:return None
        out+=matched
    return out
def C(name,**payload):return {name:payload}
def F(name,**payload):return {name:payload}
results=[]
for case in cases:
    p=D/'expected'/(case['caseId']+'.K.json');d=json.loads(p.read_text());states=d['states'];P=states[0]['program'];nodes={x['id']:x for x in P['expressions']};fn={x['name_id']:x for x in P['functions']};layouts={(a['type_id'],b['ctor_id']):b['field_types'] for a in P['declarations'] for b in a['ctors']}
    assert d['schema']=='independent-nominal-full-state/1' and d['profile']=='K-uncharged-nominal-let-if-successor'
    assert [x['id'] for x in P['expressions']]==[x['exprId'] for x in case['expressions']]
    assert P['declarations']==[{'type_id':a['typeId'],'recursive_group':a['recursiveGroup'],'ctors':[{'type_id':a['typeId'],'ctor_id':b['ctorId'],'source_order':b['sourceOrder'],'field_types':b['fieldTypes']} for b in a['constructors']]} for a in case['declarations']]
    assert P['functions']==[{'name_id':a['name'],'parameters':[{'name_id':b['name'],'type_id':b['typeId']} for b in a['parameters']],'return_type_id':a['returnType'],'body':a['bodyExpr']} for a in case['functions']]
    for x,y in zip(P['expressions'],case['expressions']):
        assert x['occurrence']==y['occurrence']
        if y['kind']=='Match':
            arms=x['kind']['Match']['arms'];assert len(arms)==len(y['arms'])
            assert [a['source_arm_id'] for a in arms]==[a['sourceArmId'] for a in y['arms']]
            assert [a['body'] for a in arms]==[a['bodyExpr'] for a in y['arms']]
            assert [a['pattern'] for a in arms]==[source_pattern(a['pattern']) for a in y['arms']]
            assert x['kind']['Match']['scrutinee']==y['scrutineeExpr']
        elif y['kind']=='Literal':assert x['kind']=={'Literal':normv(y['value'])}
        elif y['kind']=='Variable':assert x['kind']=={'Variable':{'name_id':y['name']}}
        elif y['kind']=='Let':assert x['kind']=={'Let':{'binder':y['binder'],'bound_expr':y['valueExpr'],'body_expr':y['bodyExpr']}}
        elif y['kind']=='Constructor':assert x['kind']=={'Construct':{'type_id':y['typeId'],'ctor_id':y['ctorId'],'fields':y['fields']}}
        elif y['kind']=='Call':assert x['kind']=={'Call':{'name_id':y['function'],'arguments':y['args']}}
        else:assert y['kind']=='Binary' and y['op']=='Add' and x['kind']=={'Add':{'left':y['leftExpr'],'right':y['rightExpr']}}
    initial=en({e['name']:normv(e['value']) for e in case['initial_environment']})
    assert states[0]['environment']==states[-1]['environment']==initial
    assert states[0]['control']==C('ngEval',expr_id=case['entry'])
    assert states[0]['frames_top_first']==states[-1]['frames_top_first']==[]
    assert states[0]['events']==[] and d['observerCuts']==list(range(len(states)))
    assert len(states)<=513
    for ordinal,s in enumerate(states):
        assert set(s)=={'program','control','environment','frames_top_first','events'} and s['program']==P
        assert len(s['events'])==ordinal and s['events']==states[-1]['events'][:ordinal]
        enmap(s['environment'])
        for category,items in [('control',[s['control']]),('frame',s['frames_top_first']),('event',s['events'][-1:])]:
            for item in items:
                assert len(item)==1;name,payload=next(iter(item.items()));tag(category,name,payload)
        for frame in s['frames_top_first']:
            name,f=next(iter(frame.items()))
            if 'saved_environment'in f:enmap(f['saved_environment'])
            if name in ['ngArguments','ngConstructorFields']:
                field=name=='ngConstructorFields';idx=f['constructor_expr_id'] if field else f['call_expr_id'];source_ids=nodes[idx]['kind']['Construct']['fields'] if field else nodes[idx]['kind']['Call']['arguments'];cursor=f['field_cursor'] if field else f['argument_cursor']
                assert f['active_expr_id']==source_ids[cursor] and f['remaining_expr_ids']==source_ids[cursor+1:] and len(f['evaluated_prefix'])==cursor
                if field:assert f['field_types']==layouts[f['type_id'],f['ctor_id']]
    selected=[]
    for i,(a,b) in enumerate(zip(states,states[1:])):
        name,e=next(iter(b['events'][-1].items()));old=a['environment'];af=a['frames_top_first'];bf=b['frames_top_first'];ac=a['control'];bc=b['control']
        def unchanged():assert bf==af and b['environment']==old
        if name in ['ngLiteralEvaluated','ngVariableRead']:
            node=nodes[e['expr_id']];assert ac==C('ngEval',expr_id=e['expr_id']) and node['occurrence']==e['occurrence']
            v=node['kind']['Literal'] if name=='ngLiteralEvaluated' else enmap(old)[e['name_id']]
            assert e['value']==v and bc==C('ngReturn',value=v);unchanged()
        elif name in ['ngConstructorFieldReturned','ngArgumentReturned']:
            field=name=='ngConstructorFieldReturned';frame_name='ngConstructorFields' if field else 'ngArguments';f=af[0][frame_name];v=ac['ngReturn']['value'];cursor=f['field_cursor'] if field else f['argument_cursor'];prefix=f['evaluated_prefix']+[v]
            assert e['value']==v and e['prefix_after']==prefix and e['completed_expr_id']==f['active_expr_id']
            assert e['saved_environment']==f['saved_environment']==b['environment']
            if field:
                assert e['completed_expr_occurrence']==nodes[f['active_expr_id']]['occurrence'] and e['field_cursor']==cursor
                assert vtype(v)==f['field_types'][cursor]
            else:assert e['argument_cursor']==cursor
            remaining=f['remaining_expr_ids']
            if remaining:
                nf=dict(f);nf['active_expr_id']=remaining[0];nf['remaining_expr_ids']=remaining[1:];nf['evaluated_prefix']=prefix;nf['field_cursor' if field else 'argument_cursor']=cursor+1
                assert bf==[{frame_name:nf}]+af[1:] and bc==C('ngEval',expr_id=remaining[0]) and e['remaining_expr_ids_after']==remaining[1:]
            else:
                assert bf==af[1:] and e['remaining_expr_ids_after']==[]
                if field:assert bc==C('ngMaterialize',constructor_expr_id=f['constructor_expr_id'],constructor_occurrence=f['constructor_occurrence'],type_id=f['type_id'],ctor_id=f['ctor_id'],ordered_values=prefix)
                else:assert bc==C('ngApply',call_expr_id=f['call_expr_id'],call_occurrence=f['call_occurrence'],function_name_id=f['function_name_id'],arguments=prefix)
        elif name=='ngFunctionEntered':
            f=fn[e['function_name_id']];bound=en({p['name_id']:v for p,v in zip(f['parameters'],e['arguments'])})
            assert e['bound_environment']==bound==b['environment'] and bc==C('ngEval',expr_id=f['body'])
            assert af==bf[1:] and bf[0]['ngFunctionReturn']['saved_environment']==old
        elif name=='ngFunctionReturned':
            f=af[0]['ngFunctionReturn'];v=ac['ngReturn']['value'];assert bc==ac and e['value']==v and vtype(v)==f['return_type_id']
            assert bf==af[1:] and e['restored_environment']==b['environment']==f['saved_environment']
        elif name=='ngMatchStarted':
            n=nodes[e['match_expr_id']];m=n['kind']['Match'];assert ac==C('ngEval',expr_id=n['id']) and e['source_arms']==m['arms']
            assert bc==C('ngEval',expr_id=m['scrutinee']) and bf[1:]==af and b['environment']==old
        elif name=='ngScrutineeReturned':
            f=af[0]['ngScrutinee'];v=ac['ngReturn']['value'];assert bf==af[1:] and e['value']==v
            assert bc==C('ngChoose',match_expr_id=f['match_expr_id'],match_occurrence=f['match_occurrence'],scrutinee_value=v,remaining_source_arms=f['source_arms'],saved_environment=f['saved_environment'])
            assert b['environment']==f['saved_environment']
        elif name in ['ngArmMiss','ngArmSelected']:
            choose=ac['ngChoose'];arm=choose['remaining_source_arms'][0];matched=structural_match(arm['pattern'],choose['scrutinee_value'])
            assert e['source_arm_id']==arm['source_arm_id']
            if name=='ngArmMiss':
                assert matched is None and bf==af and b['environment']==old
                nxt=dict(choose);nxt['remaining_source_arms']=choose['remaining_source_arms'][1:];assert bc=={'ngChoose':nxt}
            else:
                assert matched is not None
                bindings=[{'name_id':n,'match_expr_id':choose['match_expr_id'],'source_arm_id':arm['source_arm_id'],'field_path':path,'value':v} for n,path,v in matched]
                assert e['bindings']==bindings
                new=enmap(choose['saved_environment']);new.update({n:v for n,path,v in matched})
                assert e['body_environment']==b['environment']==en(new) and bc==C('ngEval',expr_id=arm['body'])
                assert bf[1:]==af and bf[0]['ngArmScope']['saved_environment']==choose['saved_environment']
                selected.append({'stateOrdinal':i+1,'match':choose['match_expr_id'],'arm':arm['source_arm_id'],'bindings':bindings})
        elif name in ['ngArmBodyReturned','ngLetBodyReturned']:
            f=af[0]['ngArmScope' if name=='ngArmBodyReturned' else 'ngLetScope'];assert bc==ac and bf==af[1:] and b['environment']==f['saved_environment']==e['restored_environment']
        elif name=='ngLetBound':
            f=af[0]['ngLetValue'];v=ac['ngReturn']['value'];extended=enmap(f['saved_environment']);extended[f['binder']]=v
            assert e['bound_occurrence']==nodes[f['bound_expr']]['occurrence'] and e['bound_value']==v
            assert b['environment']==e['extended_environment']==en(extended) and bc==C('ngEval',expr_id=f['body_expr'])
        elif name=='ngConstructorBuilt':
            material=ac['ngMaterialize'];v={'Nominal':{'type_id':material['type_id'],'ctor_id':material['ctor_id'],'fields':material['ordered_values']}}
            assert bc==C('ngReturn',value=v) and e['value']==v;unchanged()
        elif name=='ngAdded':
            f=af[0]['ngAddRight'];right=ac['ngReturn']['value'];v={'UInt':f['left_value']['UInt']+right['UInt']}
            assert e['left_value']==f['left_value'] and e['right_value']==right and e['result']==v and bc==C('ngReturn',value=v)
            assert bf==af[1:] and b['environment']==f['saved_environment']
        elif name=='ngFinished':
            assert ac==C('ngReturn',value=e['value']) and bc==C('ngHalted',value=e['value']) and af==bf==[] and b['environment']==old
        else:
            assert name in ['ngLetStarted','ngConstructorStarted','ngCallStarted','ngAddStarted','ngAddRightRequested']
    final=states[-1]['control']['ngHalted']['value']
    if case['caseId'].startswith('structural-mutual'):
        original={x['exprId']:x for x in case['expressions']};seedA=normv(original[7]['value']);seedB=normv(original[8]['value'])
        expected={'Nominal':{'type_id':61,'ctor_id':610,'fields':[{'Nominal':{'type_id':60,'ctor_id':600,'fields':[seedA,seedB,{'UInt':9}]}},{'UInt':9}]}}
        assert final==expected
    else:
        fields=final['Nominal']['fields'];assert fields==[{'UInt':4},{'UInt':9}] if 'first-arm' in case['caseId'] else fields==[{'UInt':5},{'UInt':5}]
    results.append({'caseId':case['caseId'],'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'states':len(states),'events':len(states)-1,'fullPayloadAndEveryPrefixShape':True,'independentPatternBindingChecks':True,'callerAndSourceIdentityChecks':True,'selectedArmWitnesses':selected})
(D/'PRIVATE-INDEPENDENT-WIRE-PROOF.json').write_text(json.dumps({'status':'PASS_LOCAL_DATA_RELATION_ONLY','noSubjectOrPeerImplementationExecution':True,'cases':results},indent=2)+'\n')
print(json.dumps({'status':'PASS_LOCAL_DATA_RELATION_ONLY','documents':len(results)}))
