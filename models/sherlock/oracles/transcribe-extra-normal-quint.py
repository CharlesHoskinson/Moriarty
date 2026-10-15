#!/usr/bin/env python3
"""Literal structural serializer only: no evaluator imports or transition execution."""
import argparse
import json
import pathlib
import re

class KParser:
    def __init__(self, text):
        self.tokens = re.findall(r'\|->|\.Map|\.K|[A-Za-z_][A-Za-z_0-9]*|-?\d+|[(),]', text)
        assert re.sub(r'\s+', '', text) == ''.join(self.tokens)
        self.i = 0
    def pop(self, expected=None):
        value = self.tokens[self.i]; self.i += 1
        assert expected is None or value == expected, (value, expected)
        return value
    def term(self):
        if self.tokens[self.i] == '.Map':
            self.pop(); return ('map', ())
        if re.fullmatch(r'-?\d+', self.tokens[self.i]):
            value = int(self.pop())
            if self.i < len(self.tokens) and self.tokens[self.i] == '|->':
                entries = []
                while True:
                    self.pop('|->'); entries.append((value, self.term()))
                    if self.i >= len(self.tokens) or not re.fullmatch(r'-?\d+', self.tokens[self.i]): break
                    value = int(self.pop())
                return ('map', tuple(entries))
            return value
        name = self.pop()
        if name in ('.K', 'true', 'false'): return (name, ())
        self.pop('('); args = []
        while self.tokens[self.i] != ')':
            args.append(self.term())
            if self.tokens[self.i] == ',': self.pop(',')
            else: break
        self.pop(')'); return (name, tuple(args))
    def parse(self):
        result = self.term(); assert self.i == len(self.tokens); return result

def parse(text): return KParser(text).parse()
def seq(term, cons, nil):
    if term[0] == nil: assert not term[1]; return []
    assert term[0] == cons
    return [term[1][0]] + seq(term[1][1], cons, nil)
def qlist(items): return 'List(' + ', '.join(items) + ')'
def rec(**fields): return '{' + ', '.join(k + ': ' + str(v) for k, v in fields.items()) + '}'
def con(name, value): return name + '(' + str(value) + ')'
OPS = {'uiAdd':'Add', 'uiSub':'Sub', 'uiMul':'Mul', 'uiEq':'Eq', 'uiLt':'Lt', 'uiConsOp':'ConsOp', 'uiBranchOp':'BranchOp', 'uiLeafOp':'LeafOp', 'uiNot':'NotOp'}
TYPES = {'uiUIntT':'UIntType','uiBoolT':'BoolType','uiListT':'ListType','uiTreeT':'TreeType'}
def ty(term): return TYPES[term[0]]
def value(term):
    name, args = term
    if name == 'uiU': return con('UInt',args[0])
    if name == 'uiB': return con('BoolValue', str(args[0][0] == 'true').lower())
    raise ValueError(term)
def env(term):
    assert term[0] == 'map'
    return 'Map(' + ', '.join(str(k) + ' -> ' + value(v) for k,v in term[1]) + ')'

class Transcription:
    def __init__(self, source):
        self.ids = {}; self.terms = []
        self.entry = parse(source['entry_k']); self.functions = seq(parse(source['code_k']), 'uiFunction','uiNoFuns')
        self.intern(self.entry)
        for function in self.functions: self.intern(function[1][3])
    def intern(self, term):
        if term in self.ids: return self.ids[term]
        eid = len(self.terms); self.ids[term] = eid; self.terms.append(term)
        name,args = term
        children = {'uiLiteral':lambda:[], 'uiVar':lambda:[], 'uiBinary':lambda:list(args[1:]), 'uiIf':lambda:list(args), 'uiCall':lambda:seq(args[1],'uiExpr','uiNoExprs')}[name]()
        for child in children: self.intern(child)
        return eid
    def eid(self, term): return self.ids[term]
    def expr(self, term):
        name,a=term
        if name=='uiLiteral': return con('Literal',value(a[0]))
        if name=='uiVar': return con('Variable',a[0])
        if name=='uiBinary': return con('Binary',rec(op=OPS[a[0][0]],left=self.eid(a[1]),right=self.eid(a[2])))
        if name=='uiIf': return con('If',rec(condition=self.eid(a[0]),yes=self.eid(a[1]),no=self.eid(a[2])))
        if name=='uiCall': return con('Call',rec(name=a[0],arguments=qlist([str(self.eid(e)) for e in seq(a[1],'uiExpr','uiNoExprs')])))
        raise ValueError(term)
    def function(self, term):
        assert term[0]=='uiFun'; a=term[1]; params=[]; p=a[1]
        while p[0]!='uiNoParams':
            assert p[0]=='uiParam'; n,t,p=p[1]; params.append(rec(name=n,ty=ty(t)))
        return rec(name=a[0],parameters=qlist(params),returnType=ty(a[2]),body=self.eid(a[3]))
    def program(self):
        return rec(expressions='Map(' + ',\n      '.join(str(i)+' -> '+self.expr(t) for i,t in enumerate(self.terms)) + ')',functions=qlist([self.function(f) for f in self.functions]),entry=self.eid(self.entry))
    def frame(self, term):
        name,a=term
        if name=='uiArgsF': return con('ArgsFrame',rec(name=a[0],values=qlist([value(v) for v in seq(a[1],'uiValue','uiNoValues')]),remaining=qlist([str(self.eid(e)) for e in seq(a[2],'uiExpr','uiNoExprs')]),environment=env(a[3])))
        if name=='uiReturnF': return con('ReturnFrame',rec(ty=ty(a[0]),environment=env(a[1])))
        if name=='uiLeftF': return con('LeftFrame',rec(op=OPS[a[0][0]],right=self.eid(a[1]),environment=env(a[2])))
        if name=='uiRightF': return con('RightFrame',rec(op=OPS[a[0][0]],left=value(a[1])))
        if name=='uiIfF': return con('IfFrame',rec(yes=self.eid(a[0]),no=self.eid(a[1]),environment=env(a[2])))
        raise ValueError(term)
    def control(self, term):
        name,a=term
        if name=='.K': return 'EmptyControl'
        if name=='uiEval': return con('Eval',self.eid(a[0]))
        if name=='uiRet': return con('Return',value(a[0]))
        if name=='uiPush':
            assert a[1][0]=='uiEval'
            return con('Push',rec(frame=self.frame(a[0]),expression=self.eid(a[1][1][0]),environment=env(a[2])))
        if name=='uiInvoke': return con('Invoke',rec(name=a[0],values=qlist([value(v) for v in seq(a[1],'uiValue','uiNoValues')])))
        if name=='uiEnter': return con('Enter',rec(found=con('FoundFunction',self.function(a[0])),values=qlist([value(v) for v in seq(a[1],'uiValue','uiNoValues')])))
        if name=='uiBound':
            assert a[0][0]=='uiBindings'
            return con('Bound',rec(bindings=con('Bindings',env(a[0][1][0])),ty=ty(a[1]),expression=self.eid(a[2])))
        if name=='uiPrimResult':
            assert a[0][0]=='uiPrimValue'
            return con('PrimitiveResult',con('PrimitiveValue',value(a[0][1][0])))
        raise ValueError(term)
    def outcome(self, term):
        name,a=term
        if name=='uiPending': return 'Pending'
        if name=='uiDone': return con('Done',value(a[0]))
        if name=='uiExhausted': return 'Exhausted'
        raise ValueError(term)
    def snapshot(self, prefix, state, fixed, overrides=None):
        data={**fixed,**state,**(overrides or {})}
        return rec(program=prefix+'Program',control=self.control(parse(data['control_k'])),environment=env(parse(data['environment_k'])),frames=qlist([self.frame(parse(f)) for f in data['frames_top_first_k']]),stackLimit=data['stack_limit'],remaining=data['remaining_fuel'],initial=data['initial_fuel'],outcome=self.outcome(parse(data['output_k'])))

def main():
    ap=argparse.ArgumentParser(); ap.add_argument('--repo',type=pathlib.Path,required=True); ap.add_argument('--out',type=pathlib.Path,required=True); a=ap.parse_args()
    inputs=json.loads((a.repo/'models/sherlock/oracles/extra-normal-programs.json').read_text())
    traces=json.loads((a.repo/'models/sherlock/oracles/extra-normal-prefixes.json').read_text())
    lines=['module kProfileExtraNormalTest {','  import kProfileMachine.* from "./kProfileMachine"','','  // Additive literal transcription of extra-normal-programs/prefixes.json only.','  // Clock: sherlock-k-a/1; charged machine rules, free startup/publication.','  // A helpers are unmetered; helper visits/work are neither measured nor zero.','  // Immutable ExprIds share structurally equal syntax; recursive calls retain names.','  // Nine frame forms belong to the unchanged imported model; frames are top first.','  // Source-only normal examples: no B, financial, theorem, native or target claim.','']
    for source,trace in zip(inputs['programs'],traces['traces'],strict=True):
        assert source['case_id']==trace['case_id']; p=source['case_id'].lower(); t=Transcription(source); fixed=trace['fixed_state_fields']; states=trace['states']
        lines += ['  // '+source['case_id']+': '+source['title']+'.','  pure val '+p+'Program: Program = '+t.program()]
        for i,state in enumerate(states):
            assert i==state['charged_steps'] and state['depth']==len(state['frames_top_first_k'])
            lines.append('  pure val '+p+'State'+str(i)+': Snapshot = '+t.snapshot(p,state,fixed))
        pub=trace['publication']; published=t.snapshot(p,states[pub['pre_state_index']],fixed,pub['post_overrides'])
        lines += ['  pure val '+p+'Published: Snapshot = '+published,'','  run '+p+'ExactTraceTest = initProgram('+p+'Program, '+str(source['exact_completion_fuel'])+', '+str(source['stack_limit'])+')','    .expect(machine == '+p+'State0 and lastRule == '+str(trace['startup']['rule_start_line'])+' and lastCharge == '+str(trace['startup']['charge'])+')']
        for i,state in enumerate(states[1:],1):
            lines += ['    .then(step)','    .expect(machine == '+p+'State'+str(i)+' and lastRule == '+str(state['via_rule_start_line'])+' and lastCharge == 1)']
        lines += ['    .then(step)','    .expect(machine == '+p+'Published and lastRule == '+str(pub['rule_start_line'])+' and lastCharge == '+str(pub['charge'])+')','']
        for fuel,boundary,remaining,output,rule in trace['ordinary_fuel_prefixes']:
            snapshot=t.snapshot(p,states[boundary],fixed,dict(initial_fuel=fuel,remaining_fuel=remaining,control_k='.K',output_k=output))
            lines += ['  run '+p+'Fuel'+str(fuel)+'PrefixTest = initProgram('+p+'Program, '+str(fuel)+', '+str(source['stack_limit'])+')']
            if boundary: lines.append('    .then('+str(boundary)+'.reps(i => step))')
            lines += ['    .then(step)','    .expect(machine == '+snapshot+' and lastRule == '+str(rule)+' and lastCharge == 0)','']
    lines += ['}','']; a.out.write_text('\n'.join(lines))
if __name__=='__main__': main()
