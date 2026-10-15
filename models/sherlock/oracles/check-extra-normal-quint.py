#!/usr/bin/env python3
"""Independent literal source readback. Does not import serializer or machine code.
Parses emitted Quint literals, reverses their structural projection to K terms,
and compares with frozen JSON. No transitions or arithmetic are evaluated.
"""
import argparse
import hashlib
import json
import pathlib
import re

class Reader:
    def __init__(self, text, quint=False):
        pattern=r'\|->|->|\.Map|\.K|[A-Za-z_][A-Za-z_0-9]*|-?\d+|[(){},:]'
        self.ts=re.findall(pattern,text)
        assert ''.join(self.ts)==re.sub(r'\s+','',text), text
        self.at=0; self.quint=quint
    def take(self, wanted=None):
        token=self.ts[self.at]; self.at+=1
        assert wanted is None or token==wanted,(token,wanted)
        return token
    def peek(self): return self.ts[self.at] if self.at<len(self.ts) else None
    def read(self):
        tok=self.take()
        if tok=='{':
            fields={}
            while self.peek()!='}':
                name=self.take(); self.take(':'); assert name not in fields
                fields[name]=self.read()
                if self.peek()!=',': break
                self.take(',')
            self.take('}'); return fields
        if re.fullmatch(r'-?\d+',tok):
            number=int(tok)
            if self.peek()=='|->':
                entries=[]
                while True:
                    self.take('|->'); entries.append([number,self.read()])
                    if self.peek() is None or not re.fullmatch(r'-?\d+',self.peek()): break
                    number=int(self.take())
                return {'map':sorted(entries)}
            return number
        if tok=='.Map': return {'map':[]}
        if self.peek()!='(':
            return {'name':tok} if self.quint else {'ctor':tok,'args':[]}
        self.take('('); args=[]
        while self.peek()!=')':
            left=self.read()
            if self.peek()=='->':
                self.take('->'); left=[left,self.read()]
            args.append(left)
            if self.peek()!=',': break
            self.take(',')
        self.take(')')
        if self.quint and tok=='Map': return {'map':sorted(args)}
        if self.quint and tok=='List': return args
        return {'ctor':tok,'args':args}
    def whole(self):
        result=self.read(); assert self.at==len(self.ts); return result

def k(text): return Reader(text).whole()
def q(text): return Reader(text,True).whole()
def node(name,*args): return {'ctor':name,'args':list(args)}
def name(term): return term.get('ctor',term.get('name'))
def arg(term): return term['args'][0]
def chain(items,cons,nil):
    result=node(nil)
    for item in reversed(items): result=node(cons,item,result)
    return result

def value(term):
    if name(term)=='UInt': return node('uiU',arg(term))
    if name(term)=='BoolValue': return node('uiB',node(name(arg(term))))
    raise ValueError(term)
def environment(term): return {'map':[[key,value(v)] for key,v in term['map']]}

def vtype(term):
    return node({'UIntType':'uiUIntT','BoolType':'uiBoolT','ListType':'uiListT','TreeType':'uiTreeT'}[name(term)])
def operator(term):
    return node({'Add':'uiAdd','Sub':'uiSub','Mul':'uiMul','Eq':'uiEq','Lt':'uiLt'}[name(term)])

class Reverse:
    def __init__(self,program):
        self.p=program; self.table=dict(program['expressions']['map']); self.used=set(); self.visiting=set()
        assert len(self.table)==len(program['expressions']['map'])
        assert all(isinstance(n,int) for n in self.table)
    def expression(self,eid):
        assert eid not in self.visiting, 'syntax cycle'
        self.used.add(eid); self.visiting.add(eid)
        result=self.expression_form(self.table[eid]); self.visiting.remove(eid); return result
    def expression_form(self,e):
        label=name(e); a=arg(e)
        if label=='Literal': return node('uiLiteral',value(a))
        if label=='Variable': return node('uiVar',a)
        if label=='Binary': return node('uiBinary',operator(a['op']),self.expression(a['left']),self.expression(a['right']))
        if label=='If': return node('uiIf',self.expression(a['condition']),self.expression(a['yes']),self.expression(a['no']))
        if label=='Call': return node('uiCall',a['name'],chain([self.expression(i) for i in a['arguments']],'uiExpr','uiNoExprs'))
        raise ValueError(e)
    def function(self,f):
        params=node('uiNoParams')
        for p in reversed(f['parameters']): params=node('uiParam',p['name'],vtype(p['ty']),params)
        return node('uiFun',f['name'],params,vtype(f['returnType']),self.expression(f['body']))
    def code(self): return chain([self.function(f) for f in self.p['functions']],'uiFunction','uiNoFuns')
    def frame(self,f):
        label=name(f); a=arg(f)
        if label=='ArgsFrame': return node('uiArgsF',a['name'],chain([value(v) for v in a['values']],'uiValue','uiNoValues'),chain([self.expression(i) for i in a['remaining']],'uiExpr','uiNoExprs'),environment(a['environment']))
        if label=='ReturnFrame': return node('uiReturnF',vtype(a['ty']),environment(a['environment']))
        if label=='LeftFrame': return node('uiLeftF',operator(a['op']),self.expression(a['right']),environment(a['environment']))
        if label=='RightFrame': return node('uiRightF',operator(a['op']),value(a['left']))
        if label=='IfFrame': return node('uiIfF',self.expression(a['yes']),self.expression(a['no']),environment(a['environment']))
        raise ValueError(f)
    def control(self,c):
        label=name(c)
        if label=='EmptyControl': return node('.K')
        a=arg(c)
        if label=='Eval': return node('uiEval',self.expression(a))
        if label=='Return': return node('uiRet',value(a))
        if label=='Push': return node('uiPush',self.frame(a['frame']),node('uiEval',self.expression(a['expression'])),environment(a['environment']))
        if label=='Invoke': return node('uiInvoke',a['name'],chain([value(v) for v in a['values']],'uiValue','uiNoValues'))
        if label=='Enter':
            assert name(a['found'])=='FoundFunction'
            return node('uiEnter',self.function(arg(a['found'])),chain([value(v) for v in a['values']],'uiValue','uiNoValues'))
        if label=='Bound':
            assert name(a['bindings'])=='Bindings'
            return node('uiBound',node('uiBindings',environment(arg(a['bindings']))),vtype(a['ty']),self.expression(a['expression']))
        if label=='PrimitiveResult':
            assert name(a)=='PrimitiveValue'
            return node('uiPrimResult',node('uiPrimValue',value(arg(a))))
        raise ValueError(c)
    def output(self,o):
        if name(o)=='Pending': return node('uiPending')
        if name(o)=='Exhausted': return node('uiExhausted')
        if name(o)=='Done': return node('uiDone',value(arg(o)))
        raise ValueError(o)
    def snapshot(self,s,prefix):
        assert s['program']=={'name':prefix+'Program'}
        assert set(s)=={'program','control','environment','frames','stackLimit','remaining','initial','outcome'}
        return dict(code_k=self.code(),control_k=self.control(s['control']),environment_k=environment(s['environment']),frames_top_first_k=[self.frame(f) for f in s['frames']],depth=len(s['frames']),stack_limit=s['stackLimit'],remaining_fuel=s['remaining'],initial_fuel=s['initial'],output_k=self.output(s['outcome']))

def digest(data): return hashlib.sha256(json.dumps(data,sort_keys=True,separators=(',',':')).encode()).hexdigest()
def filehash(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def expected(source,trace,state,overrides=None):
    s={**trace['fixed_state_fields'],**state,**(overrides or {})}
    assert s['code_ref']['case_id']==source['case_id'] and s['code_ref']['input_sha256']==source['input_sha256']
    return dict(code_k=k(source['code_k']),control_k=k(s['control_k']),environment_k=k(s['environment_k']),frames_top_first_k=[k(f) for f in s['frames_top_first_k']],depth=s['depth'],stack_limit=s['stack_limit'],remaining_fuel=s['remaining_fuel'],initial_fuel=s['initial_fuel'],output_k=k(s['output_k']))

def main():
    ap=argparse.ArgumentParser(); ap.add_argument('--repo',type=pathlib.Path,required=True);ap.add_argument('--source',type=pathlib.Path,required=True);ap.add_argument('--report',type=pathlib.Path,required=True); a=ap.parse_args()
    inputs_path=a.repo/'models/sherlock/oracles/extra-normal-programs.json'; traces_path=a.repo/'models/sherlock/oracles/extra-normal-prefixes.json'
    inputs=json.loads(inputs_path.read_text()); traces=json.loads(traces_path.read_text()); raw=a.source.read_text()
    assert 'import kProfileMachine.* from "./kProfileMachine"' in raw
    assert not re.search(r'\b(?:action|var|assume|def|temporal)\s',raw)
    starts=list(re.finditer(r'^  (pure val (\w+): (\w+) = |run (\w+) = )',raw,re.M))
    vals={}; runs={}; line_numbers={}
    for i,m in enumerate(starts):
        body=raw[m.end():starts[i+1].start() if i+1<len(starts) else raw.rfind('\n}')]
        body=re.sub(r'\n\s*//[^\n]*','',body).strip()
        identifier=m[2] or m[4]; assert identifier not in line_numbers
        line_numbers[identifier]=raw[:m.start()].count('\n')+1
        if m[2]: vals[identifier]=q(body)
        else: runs[identifier]=body
    entries=[]; used_runs=set(); case_reports=[]
    def check(label,locator,actual,wanted):
        assert actual==wanted, (label,locator,actual,wanted)
        entries.append(dict(quint_label=label,quint_line=line_numbers.get(label),oracle_locator=locator,equal=True,canonical_sha256=digest(actual),decoded=actual))
    def expectations(body):
        matches=re.findall(r'\.expect\(machine == (.*?) and lastRule == (-?\d+) and lastCharge == (-?\d+)\)',body,re.S)
        assert len(matches)==body.count('.expect(')
        return [(q(s),int(r),int(c)) for s,r,c in matches]
    for ci,(s,t) in enumerate(zip(inputs['programs'],traces['traces'],strict=True)):
        prefix=s['case_id'].lower(); assert s['case_id']==t['case_id']; base=f'traces[{ci}]'; rev=Reverse(vals[prefix+'Program'])
        six={key:s[key] for key in ('case_id','code_k','entry_k','initial_arguments','stack_limit','exact_completion_fuel')}
        assert digest(six)==s['input_sha256']
        check(prefix+'Program',f'programs[{ci}].code_k/entry_k',dict(code_k=rev.code(),entry_k=rev.expression(rev.p['entry'])),dict(code_k=k(s['code_k']),entry_k=k(s['entry_k'])))
        assert rev.used==set(rev.table), 'unused or unreachable ExprId'
        for j,state in enumerate(t['states']):
            check(prefix+'State'+str(j),base+f'.states[{j}]+fixed_state_fields',rev.snapshot(vals[prefix+'State'+str(j)],prefix),expected(s,t,state))
        pub=t['publication']; pubstate=t['states'][pub['pre_state_index']]
        check(prefix+'Published',base+'.publication',rev.snapshot(vals[prefix+'Published'],prefix),expected(s,t,pubstate,pub['post_overrides']))
        rn=prefix+'ExactTraceTest'; body=runs[rn]; used_runs.add(rn)
        assert body.startswith(f'initProgram({prefix}Program, {s["exact_completion_fuel"]}, {s["stack_limit"]})')
        assert body.count('.then(step)')==len(t['states']) and '.reps(' not in body
        ex=expectations(body)
        wanted=[({'name':prefix+'State'+str(j)},t['startup']['rule_start_line'] if j==0 else state['via_rule_start_line'],t['startup']['charge'] if j==0 else 1) for j,state in enumerate(t['states'])]
        wanted.append(({'name':prefix+'Published'},pub['rule_start_line'],pub['charge']))
        assert ex==wanted,(rn,'assertion schedule')
        skeleton=re.sub(r'\.expect\(machine == .*? and lastRule == -?\d+ and lastCharge == -?\d+\)', '.expect(LITERAL)', body, flags=re.S)
        expected_lines=[f'initProgram({prefix}Program, {s["exact_completion_fuel"]}, {s["stack_limit"]})','.expect(LITERAL)']
        for _ in t['states']: expected_lines.extend(['.then(step)','.expect(LITERAL)'])
        assert re.sub(r'\s+','',skeleton)==re.sub(r'\s+','',''.join(expected_lines))
        for pi,(fuel,boundary,remaining,output,rule) in enumerate(t['ordinary_fuel_prefixes']):
            rn=prefix+'Fuel'+str(fuel)+'PrefixTest'; body=runs[rn]; used_runs.add(rn)
            assert body.startswith(f'initProgram({prefix}Program, {fuel}, {s["stack_limit"]})')
            schedule=f'.then({boundary}.reps(i => step))' if boundary else ''
            assert body.count('.then(step)')==1 and body.count('.reps(')==int(boundary>0) and schedule in body
            ex=expectations(body); assert len(ex)==1 and ex[0][1:]==(rule,0)
            skeleton=re.sub(r'\.expect\(machine == .*? and lastRule == -?\d+ and lastCharge == -?\d+\)', '.expect(LITERAL)', body, flags=re.S)
            expected_skeleton=f'initProgram({prefix}Program, {fuel}, {s["stack_limit"]})'+schedule+'.then(step).expect(LITERAL)'
            assert re.sub(r'\s+','',skeleton)==re.sub(r'\s+','',expected_skeleton)
            overrides=dict(initial_fuel=fuel,remaining_fuel=remaining,control_k='.K',output_k=output)
            check(rn,base+f'.ordinary_fuel_prefixes[{pi}]+states[{boundary}]',rev.snapshot(ex[0][0],prefix),expected(s,t,t['states'][boundary],overrides))
        case_reports.append(dict(case_id=s['case_id'],expression_ids=sorted(rev.table),expression_table_k={str(i):rev.expression(i) for i in sorted(rev.table)},state_rows=len(t['states']),charged_edges=len(t['states'])-1,complete_publications=1,prefix_rows=len(t['ordinary_fuel_prefixes']),tests=1+len(t['ordinary_fuel_prefixes']),max_depth=max(st['depth'] for st in t['states']),stack_limit=s['stack_limit'],exact_completion_fuel=s['exact_completion_fuel']))
    assert set(runs)==used_runs and len(runs)==126 and len(vals)==126
    old_test=a.repo/'models/sherlock/quint/kProfileMachineTest.qnt'
    model=a.repo/'models/sherlock/quint/kProfileMachine.qnt'
    assert not set(runs).intersection(re.findall(r'\brun (\w+) =',old_test.read_text()))
    assert filehash(model)=='c62c12dd3a973bcb54b549f45768a356b0fb3a154b193cd76ac80d2d80fa95ca'
    assert filehash(old_test)=='985069bfdfef2f8417a3b7583454b90d84c1764070d77baa608d0add627f6493'
    assert filehash(inputs_path)=='dd85e5d400f24e9be5e2ff017f4b8c141a0bf6e4270053917b735cbe5153f153'
    assert filehash(traces_path)=='acf460094b258910bad0499fb8feda0cd85b680023939800ad714d2cb71bee16'
    report=dict(format='sherlock-extra-normal-quint-literal-readback/1',status='static literal reconstruction passed; model not run',profile=inputs['profile'],source_sha256=filehash(a.source),oracle_sha256={inputs_path.name:filehash(inputs_path),traces_path.name:filehash(traces_path)},unchanged_membership_sha256={model.name:filehash(model),old_test.name:filehash(old_test)},method='Independent Quint literal parser reverses ExprIds and every projected field to K constructor trees; canonical terms compared with independently parsed frozen JSON. No evaluator imports or transitions.',field_map={'program':'code_ref -> source.code_k; entry_k -> finite ExprId table','control':'control_k -> Control, ExprIds expanded back to literal syntax','environment':'environment_k -> Environment; K Map presentation order ignored','frames':'frames_top_first_k -> ordered Frame list with saved fields','depth':'frames.length; also checked against every literal depth','stackLimit':'stack_limit','remaining':'remaining_fuel','initial':'initial_fuel','outcome':'output_k','lastRule':'via_rule_start_line/startup/publication or prefix column 4','lastCharge':'startup/publication charge or one per charged edge'},cases=case_reports,counts=dict(programs=2,state_rows=122,charged_edges=120,complete_publications=2,prefix_rows=124,tests=126,literal_comparisons=len(entries)),entries=entries)
    a.report.write_text(json.dumps(report,indent=2,sort_keys=True)+'\n')
    print(json.dumps({key:report[key] for key in ('status','source_sha256','counts')},indent=2))
if __name__=='__main__': main()
