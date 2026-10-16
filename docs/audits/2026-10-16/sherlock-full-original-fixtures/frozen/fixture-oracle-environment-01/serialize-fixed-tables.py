"""Serialize fixed hand-authored rows. This file contains no transition evaluator.
K terms below are text constants. expand() only substitutes named text constants;
rows, rule lines, environments, stack membership and fuel endpoints are supplied.
"""
from pathlib import Path
import json,hashlib
R=Path(__file__).parent
base=json.loads((R/'pinned/models/sherlock/oracles/normal-prefixes.json').read_text())
S={
'M':'.Map','e0':'0 |-> uiU(0)','e1':'0 |-> uiU(1)','e2':'0 |-> uiU(2)','e3':'0 |-> uiU(3)','e4':'0 |-> uiU(4)','e5':'0 |-> uiU(5)','e10':'0 |-> uiU(10)',
'l0':'uiLiteral(uiU(0))','l1':'uiLiteral(uiU(1))','l2':'uiLiteral(uiU(2))','l4':'uiLiteral(uiU(4))','l5':'uiLiteral(uiU(5))','l10':'uiLiteral(uiU(10))','l99':'uiLiteral(uiU(99))',
'bt':'uiLiteral(uiB(true))','bf':'uiLiteral(uiB(false))','var':'uiVar(0)',
'ps':'uiParam(0, uiUIntT(), uiNoParams())',
'v0':'uiValue(uiU(0), uiNoValues())','v1':'uiValue(uiU(1), uiNoValues())','v2':'uiValue(uiU(2), uiNoValues())','v3':'uiValue(uiU(3), uiNoValues())','v4':'uiValue(uiU(4), uiNoValues())',
'scope':'uiScopeF(.Map)',
'rM':'uiReturnF(uiBoolT(), .Map)','r1':'uiReturnF(uiBoolT(), 0 |-> uiU(1))','r2':'uiReturnF(uiBoolT(), 0 |-> uiU(2))','r3':'uiReturnF(uiBoolT(), 0 |-> uiU(3))','r4':'uiReturnF(uiBoolT(), 0 |-> uiU(4))',
'RU10':'uiReturnF(uiUIntT(), 0 |-> uiU(10))',
}
def expand(text):
    while '{' in text:
        text=text.format_map(S)
    return text
# All term aliases are fixed expressions or fixed frame text, never results of a
# state transition, lookup, arithmetic evaluator, or program parser.
for key,text in [
 ('notfalse','uiUnary(uiNotOp(), {bf})'),('add52','uiBinary(uiAdd(), {var}, {l2})'),
 ('boolif','uiIf({notfalse}, {add52}, {l99})'),('boolentry','uiLet(0, {l5}, {boolif})'),
 ('boollet','uiLetF(0, {boolif}, .Map)'),('boolifF','uiIfF({add52}, {l99}, {e5})'),
 ('notF','uiUnaryF(uiNotOp())'),('add5F','uiLeftF(uiAdd(), {l2}, {e5})'),('right5F','uiRightF(uiAdd(), uiU(5))'),
 ('fun5','uiFun(5, {ps}, uiUIntT(), {var})'),('code5','uiFunction({fun5}, uiNoFuns())'),
 ('call5','uiCall(5, uiExpr({l2}, uiNoExprs()))'),('calleradd','uiBinary(uiAdd(), {call5}, {var})'),('callerentry','uiLet(0, {l10}, {calleradd})'),
 ('callerlet','uiLetF(0, {calleradd}, .Map)'),('callerleft','uiLeftF(uiAdd(), {var}, {e10})'),('callerargs','uiArgsF(5, uiNoValues(), uiNoExprs(), {e10})'),('callerright','uiRightF(uiAdd(), uiU(2))'),
 ('eq','uiBinary(uiEq(), {var}, {l0})'),('sub','uiBinary(uiSub(), {var}, {l1})'),
 ('call6sub','uiCall(6, uiExpr({sub}, uiNoExprs()))'),('call7sub','uiCall(7, uiExpr({sub}, uiNoExprs()))'),
 ('body6','uiIf({eq}, {bt}, {call7sub})'),('body7','uiIf({eq}, {bf}, {call6sub})'),
 ('fun6','uiFun(6, {ps}, uiBoolT(), {body6})'),('fun7','uiFun(7, {ps}, uiBoolT(), {body7})'),
 ('codemut','uiFunction({fun6}, uiFunction({fun7}, uiNoFuns()))'),('mutentry','uiCall(6, uiExpr({l4}, uiNoExprs()))'),
 ('if4','uiIfF({bt}, {call7sub}, {e4})'),('if3','uiIfF({bf}, {call6sub}, {e3})'),('if2','uiIfF({bt}, {call7sub}, {e2})'),('if1','uiIfF({bf}, {call6sub}, {e1})'),('if0','uiIfF({bt}, {call7sub}, {e0})'),
 ('eq4','uiLeftF(uiEq(), {l0}, {e4})'),('eq3','uiLeftF(uiEq(), {l0}, {e3})'),('eq2','uiLeftF(uiEq(), {l0}, {e2})'),('eq1','uiLeftF(uiEq(), {l0}, {e1})'),('eq0','uiLeftF(uiEq(), {l0}, {e0})'),
 ('q4','uiRightF(uiEq(), uiU(4))'),('q3','uiRightF(uiEq(), uiU(3))'),('q2','uiRightF(uiEq(), uiU(2))'),('q1','uiRightF(uiEq(), uiU(1))'),('q0','uiRightF(uiEq(), uiU(0))'),
 ('arg6M','uiArgsF(6, uiNoValues(), uiNoExprs(), .Map)'),('arg7e4','uiArgsF(7, uiNoValues(), uiNoExprs(), {e4})'),('arg6e3','uiArgsF(6, uiNoValues(), uiNoExprs(), {e3})'),('arg7e2','uiArgsF(7, uiNoValues(), uiNoExprs(), {e2})'),('arg6e1','uiArgsF(6, uiNoValues(), uiNoExprs(), {e1})'),
 ('sub4','uiLeftF(uiSub(), {l1}, {e4})'),('sub3','uiLeftF(uiSub(), {l1}, {e3})'),('sub2','uiLeftF(uiSub(), {l1}, {e2})'),('sub1','uiLeftF(uiSub(), {l1}, {e1})'),
 ('s4','uiRightF(uiSub(), uiU(4))'),('s3','uiRightF(uiSub(), uiU(3))'),('s2','uiRightF(uiSub(), uiU(2))'),('s1','uiRightF(uiSub(), uiU(1))'),
]:
    S[key]=expand(text)
# Columns are exact incoming rule line, complete control, environment alias,
# top-first frame aliases. A blank frame column denotes uiNoFrames().
BOOL_ROWS='''
- | uiEval({boolentry}) | M |
114 | uiPush({boollet}, uiEval({l5}), .Map) | M |
135 | uiEval({l5}) | M | boollet
102 | uiRet(uiU(5)) | M | boollet
145 | uiEval({boolif}) | e5 | scope
123 | uiPush({boolifF}, uiEval({notfalse}), {e5}) | e5 | scope
135 | uiEval({notfalse}) | e5 | boolifF,scope
120 | uiPush({notF}, uiEval({bf}), {e5}) | e5 | boolifF,scope
135 | uiEval({bf}) | e5 | notF,boolifF,scope
102 | uiRet(uiB(false)) | e5 | notF,boolifF,scope
157 | uiPrimResult(uiPrimValue(uiB(true))) | e5 | boolifF,scope
223 | uiRet(uiB(true)) | e5 | boolifF,scope
161 | uiEval({add52}) | e5 | scope
117 | uiPush({add5F}, uiEval({var}), {e5}) | e5 | scope
135 | uiEval({var}) | e5 | add5F,scope
108 | uiRet(uiU(5)) | e5 | add5F,scope
149 | uiEval({l2}) | e5 | right5F,scope
102 | uiRet(uiU(2)) | e5 | right5F,scope
153 | uiPrimResult(uiPrimValue(uiU(7))) | e5 | scope
223 | uiRet(uiU(7)) | e5 | scope
195 | uiRet(uiU(7)) | M |
'''
CALLER_ROWS='''
- | uiEval({callerentry}) | M |
114 | uiPush({callerlet}, uiEval({l10}), .Map) | M |
135 | uiEval({l10}) | M | callerlet
102 | uiRet(uiU(10)) | M | callerlet
145 | uiEval({calleradd}) | e10 | scope
117 | uiPush({callerleft}, uiEval({call5}), {e10}) | e10 | scope
135 | uiEval({call5}) | e10 | callerleft,scope
131 | uiPush({callerargs}, uiEval({l2}), {e10}) | e10 | callerleft,scope
135 | uiEval({l2}) | e10 | callerargs,callerleft,scope
102 | uiRet(uiU(2)) | e10 | callerargs,callerleft,scope
181 | uiInvoke(5, {v2}) | e10 | callerleft,scope
200 | uiEnter({fun5}, {v2}) | e10 | callerleft,scope
204 | uiBound(uiBindings({e2}), uiUIntT(), {var}) | e10 | callerleft,scope
208 | uiPush({RU10}, uiEval({var}), {e2}) | e10 | callerleft,scope
135 | uiEval({var}) | e2 | RU10,callerleft,scope
108 | uiRet(uiU(2)) | e2 | RU10,callerleft,scope
185 | uiRet(uiU(2)) | e10 | callerleft,scope
149 | uiEval({var}) | e10 | callerright,scope
108 | uiRet(uiU(10)) | e10 | callerright,scope
153 | uiPrimResult(uiPrimValue(uiU(12))) | e10 | scope
223 | uiRet(uiU(12)) | e10 | scope
195 | uiRet(uiU(12)) | M |
'''
MUTUAL_ROWS='''
- | uiEval({mutentry}) | M |
131 | uiPush({arg6M}, uiEval({l4}), .Map) | M |
135 | uiEval({l4}) | M | arg6M
102 | uiRet(uiU(4)) | M | arg6M
181 | uiInvoke(6, {v4}) | M |
200 | uiEnter({fun6}, {v4}) | M |
204 | uiBound(uiBindings({e4}), uiBoolT(), {body6}) | M |
208 | uiPush({rM}, uiEval({body6}), {e4}) | M |
135 | uiEval({body6}) | e4 | rM
123 | uiPush({if4}, uiEval({eq}), {e4}) | e4 | rM
135 | uiEval({eq}) | e4 | if4,rM
117 | uiPush({eq4}, uiEval({var}), {e4}) | e4 | if4,rM
135 | uiEval({var}) | e4 | eq4,if4,rM
108 | uiRet(uiU(4)) | e4 | eq4,if4,rM
149 | uiEval({l0}) | e4 | q4,if4,rM
102 | uiRet(uiU(0)) | e4 | q4,if4,rM
153 | uiPrimResult(uiPrimValue(uiB(false))) | e4 | if4,rM
223 | uiRet(uiB(false)) | e4 | if4,rM
165 | uiEval({call7sub}) | e4 | rM
131 | uiPush({arg7e4}, uiEval({sub}), {e4}) | e4 | rM
135 | uiEval({sub}) | e4 | arg7e4,rM
117 | uiPush({sub4}, uiEval({var}), {e4}) | e4 | arg7e4,rM
135 | uiEval({var}) | e4 | sub4,arg7e4,rM
108 | uiRet(uiU(4)) | e4 | sub4,arg7e4,rM
149 | uiEval({l1}) | e4 | s4,arg7e4,rM
102 | uiRet(uiU(1)) | e4 | s4,arg7e4,rM
153 | uiPrimResult(uiPrimValue(uiU(3))) | e4 | arg7e4,rM
223 | uiRet(uiU(3)) | e4 | arg7e4,rM
181 | uiInvoke(7, {v3}) | e4 | rM
200 | uiEnter({fun7}, {v3}) | e4 | rM
204 | uiBound(uiBindings({e3}), uiBoolT(), {body7}) | e4 | rM
208 | uiPush({r4}, uiEval({body7}), {e3}) | e4 | rM
135 | uiEval({body7}) | e3 | r4,rM
123 | uiPush({if3}, uiEval({eq}), {e3}) | e3 | r4,rM
135 | uiEval({eq}) | e3 | if3,r4,rM
117 | uiPush({eq3}, uiEval({var}), {e3}) | e3 | if3,r4,rM
135 | uiEval({var}) | e3 | eq3,if3,r4,rM
108 | uiRet(uiU(3)) | e3 | eq3,if3,r4,rM
149 | uiEval({l0}) | e3 | q3,if3,r4,rM
102 | uiRet(uiU(0)) | e3 | q3,if3,r4,rM
153 | uiPrimResult(uiPrimValue(uiB(false))) | e3 | if3,r4,rM
223 | uiRet(uiB(false)) | e3 | if3,r4,rM
165 | uiEval({call6sub}) | e3 | r4,rM
131 | uiPush({arg6e3}, uiEval({sub}), {e3}) | e3 | r4,rM
135 | uiEval({sub}) | e3 | arg6e3,r4,rM
117 | uiPush({sub3}, uiEval({var}), {e3}) | e3 | arg6e3,r4,rM
135 | uiEval({var}) | e3 | sub3,arg6e3,r4,rM
108 | uiRet(uiU(3)) | e3 | sub3,arg6e3,r4,rM
149 | uiEval({l1}) | e3 | s3,arg6e3,r4,rM
102 | uiRet(uiU(1)) | e3 | s3,arg6e3,r4,rM
153 | uiPrimResult(uiPrimValue(uiU(2))) | e3 | arg6e3,r4,rM
223 | uiRet(uiU(2)) | e3 | arg6e3,r4,rM
181 | uiInvoke(6, {v2}) | e3 | r4,rM
200 | uiEnter({fun6}, {v2}) | e3 | r4,rM
204 | uiBound(uiBindings({e2}), uiBoolT(), {body6}) | e3 | r4,rM
208 | uiPush({r3}, uiEval({body6}), {e2}) | e3 | r4,rM
135 | uiEval({body6}) | e2 | r3,r4,rM
123 | uiPush({if2}, uiEval({eq}), {e2}) | e2 | r3,r4,rM
135 | uiEval({eq}) | e2 | if2,r3,r4,rM
117 | uiPush({eq2}, uiEval({var}), {e2}) | e2 | if2,r3,r4,rM
135 | uiEval({var}) | e2 | eq2,if2,r3,r4,rM
108 | uiRet(uiU(2)) | e2 | eq2,if2,r3,r4,rM
149 | uiEval({l0}) | e2 | q2,if2,r3,r4,rM
102 | uiRet(uiU(0)) | e2 | q2,if2,r3,r4,rM
153 | uiPrimResult(uiPrimValue(uiB(false))) | e2 | if2,r3,r4,rM
223 | uiRet(uiB(false)) | e2 | if2,r3,r4,rM
165 | uiEval({call7sub}) | e2 | r3,r4,rM
131 | uiPush({arg7e2}, uiEval({sub}), {e2}) | e2 | r3,r4,rM
135 | uiEval({sub}) | e2 | arg7e2,r3,r4,rM
117 | uiPush({sub2}, uiEval({var}), {e2}) | e2 | arg7e2,r3,r4,rM
135 | uiEval({var}) | e2 | sub2,arg7e2,r3,r4,rM
108 | uiRet(uiU(2)) | e2 | sub2,arg7e2,r3,r4,rM
149 | uiEval({l1}) | e2 | s2,arg7e2,r3,r4,rM
102 | uiRet(uiU(1)) | e2 | s2,arg7e2,r3,r4,rM
153 | uiPrimResult(uiPrimValue(uiU(1))) | e2 | arg7e2,r3,r4,rM
223 | uiRet(uiU(1)) | e2 | arg7e2,r3,r4,rM
181 | uiInvoke(7, {v1}) | e2 | r3,r4,rM
200 | uiEnter({fun7}, {v1}) | e2 | r3,r4,rM
204 | uiBound(uiBindings({e1}), uiBoolT(), {body7}) | e2 | r3,r4,rM
208 | uiPush({r2}, uiEval({body7}), {e1}) | e2 | r3,r4,rM
135 | uiEval({body7}) | e1 | r2,r3,r4,rM
123 | uiPush({if1}, uiEval({eq}), {e1}) | e1 | r2,r3,r4,rM
135 | uiEval({eq}) | e1 | if1,r2,r3,r4,rM
117 | uiPush({eq1}, uiEval({var}), {e1}) | e1 | if1,r2,r3,r4,rM
135 | uiEval({var}) | e1 | eq1,if1,r2,r3,r4,rM
108 | uiRet(uiU(1)) | e1 | eq1,if1,r2,r3,r4,rM
149 | uiEval({l0}) | e1 | q1,if1,r2,r3,r4,rM
102 | uiRet(uiU(0)) | e1 | q1,if1,r2,r3,r4,rM
153 | uiPrimResult(uiPrimValue(uiB(false))) | e1 | if1,r2,r3,r4,rM
223 | uiRet(uiB(false)) | e1 | if1,r2,r3,r4,rM
165 | uiEval({call6sub}) | e1 | r2,r3,r4,rM
131 | uiPush({arg6e1}, uiEval({sub}), {e1}) | e1 | r2,r3,r4,rM
135 | uiEval({sub}) | e1 | arg6e1,r2,r3,r4,rM
117 | uiPush({sub1}, uiEval({var}), {e1}) | e1 | arg6e1,r2,r3,r4,rM
135 | uiEval({var}) | e1 | sub1,arg6e1,r2,r3,r4,rM
108 | uiRet(uiU(1)) | e1 | sub1,arg6e1,r2,r3,r4,rM
149 | uiEval({l1}) | e1 | s1,arg6e1,r2,r3,r4,rM
102 | uiRet(uiU(1)) | e1 | s1,arg6e1,r2,r3,r4,rM
153 | uiPrimResult(uiPrimValue(uiU(0))) | e1 | arg6e1,r2,r3,r4,rM
223 | uiRet(uiU(0)) | e1 | arg6e1,r2,r3,r4,rM
181 | uiInvoke(6, {v0}) | e1 | r2,r3,r4,rM
200 | uiEnter({fun6}, {v0}) | e1 | r2,r3,r4,rM
204 | uiBound(uiBindings({e0}), uiBoolT(), {body6}) | e1 | r2,r3,r4,rM
208 | uiPush({r1}, uiEval({body6}), {e0}) | e1 | r2,r3,r4,rM
135 | uiEval({body6}) | e0 | r1,r2,r3,r4,rM
123 | uiPush({if0}, uiEval({eq}), {e0}) | e0 | r1,r2,r3,r4,rM
135 | uiEval({eq}) | e0 | if0,r1,r2,r3,r4,rM
117 | uiPush({eq0}, uiEval({var}), {e0}) | e0 | if0,r1,r2,r3,r4,rM
135 | uiEval({var}) | e0 | eq0,if0,r1,r2,r3,r4,rM
108 | uiRet(uiU(0)) | e0 | eq0,if0,r1,r2,r3,r4,rM
149 | uiEval({l0}) | e0 | q0,if0,r1,r2,r3,r4,rM
102 | uiRet(uiU(0)) | e0 | q0,if0,r1,r2,r3,r4,rM
153 | uiPrimResult(uiPrimValue(uiB(true))) | e0 | if0,r1,r2,r3,r4,rM
223 | uiRet(uiB(true)) | e0 | if0,r1,r2,r3,r4,rM
161 | uiEval({bt}) | e0 | r1,r2,r3,r4,rM
102 | uiRet(uiB(true)) | e0 | r1,r2,r3,r4,rM
185 | uiRet(uiB(true)) | e1 | r2,r3,r4,rM
185 | uiRet(uiB(true)) | e2 | r3,r4,rM
185 | uiRet(uiB(true)) | e3 | r4,rM
185 | uiRet(uiB(true)) | e4 | rM
185 | uiRet(uiB(true)) | M |
'''
# These fuel domain lists and terminal tuples are independent hand counts. The
# serializer maps those literals to the existing representation mechanically.
FUEL_BOOL=[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19]
FUEL_CALLER=[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]
FUEL_MUTUAL=[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100,101,102,103,104,105,106,107,108,109,110,111,112,113,114,115,116,117,118,119]
CASES=[
 ('F06bool','uiBoolLetOrder.ui','uiNoFuns()',S['boolentry'],20,'uiU(7)',BOOL_ROWS,FUEL_BOOL,[[20,20,0,'uiDone(uiU(7))',80],[21,20,1,'uiDone(uiU(7))',80]],3,'Frozen bool/let order fixture','20 charges: let declaration/push/literal/bind (4), if declaration/push (2), unary declaration/push/literal/result/publication-to-ret (5), true branch choice (1), binary declaration/push/variable/left-to-right/literal/result/to-ret (7), lexical scope restoration (1).'),
 ('F07caller','uiCallerLocal.ui',S['code5'],S['callerentry'],21,'uiU(12)',CALLER_ROWS,FUEL_CALLER,[[21,21,0,'uiDone(uiU(12))',80],[22,21,1,'uiDone(uiU(12))',80]],3,'Frozen caller-local restoration fixture','21 charges: initial let bind (4), binary declaration/push (2), argument declaration/push/literal/invoke (4), lookup/bind/return declaration/push (4), callee variable (1), caller restoration (1), left-to-right/right variable/result/to-ret (4), lexical scope restoration (1).'),
 ('F08mutual','uiMutualEven4.ui',S['codemut'],S['mutentry'],120,'uiB(true)',MUTUAL_ROWS,FUEL_MUTUAL,[[120,120,0,'uiDone(uiB(true))',80],[121,120,1,'uiDone(uiB(true))',80]],7,'Frozen mutual even-four fixture','120 charges: entry argument/call setup (8); four nonbase descents with false comparison/branch (10), strict subtraction argument evaluation/invoke (10), lookup/bind/return declaration/push (4), total 4*24; base comparison/true branch/literal (11); five checked return restorations (5). 8+96+11+5=120.'),
]
programs=[]; traces=[]
for cid,fixture,code,entry,n,value,table,fuels,termrows,maxdepth,title,note in CASES:
    fields={'case_id':cid,'code_k':code,'entry_k':entry,'initial_arguments':'Explicit literals in entry_k; no external arguments.','stack_limit':64,'exact_completion_fuel':n}
    digest=hashlib.sha256(json.dumps(fields,sort_keys=True,separators=(',',':')).encode()).hexdigest()
    src=R/'pinned/models/sherlock/k/fixtures'/fixture
    p={**fields,'input_sha256':digest,'sha256_convention':'SHA256 of UTF-8 compact sorted-key JSON of the six preceding source fields, excluding digest/convention and metadata.','title':title,'expected_value_k':value,'derivation':note,'original_fixture':{'path':'models/sherlock/k/fixtures/'+fixture,'sha256':hashlib.sha256(src.read_bytes()).hexdigest(),'initial_fuel':1000,'stack_limit':64,'bytes_preserved_in_pinned_copy':True},'expected_maximum_depth':maxdepth}
    programs.append(p)
    states=[]
    for i,line in enumerate(table.strip().splitlines()):
        rule,control,env,stack=[v.strip() for v in line.split(' |', 3)]
        frames=[S[k.strip()] for k in stack.split(',')] if stack else []
        states.append({'charged_steps':i,'remaining_fuel':n-i,'depth':len(frames),'control_k':expand(control),'environment_k':S[env],'frames_top_first_k':frames,'via_rule_start_line':None if rule=='-' else int(rule)})
    assert len(states)==n+1,(cid,len(states),n)
    assert max(s['depth'] for s in states)==maxdepth
    prefixes=[[f,f,0,'uiExhausted()',84] for f in fuels]+termrows
    assert [x[0] for x in prefixes]==list(range(n+2))
    initial={'control_k':f'uiRun({code}, {entry}, {n}, 64)','environment_k':'.Map','frames_top_first_k':[],'depth':0,'stack_limit':0,'code_k':'uiNoFuns()','remaining_fuel':0,'initial_fuel':0,'output_k':'uiPending()','charged_steps':0}
    traces.append({'case_id':cid,'input_sha256':digest,'fixed_state_fields':{'code_ref':{'file':'fixture-normal-programs.json','case_id':cid,'input_sha256':digest},'initial_fuel':n,'stack_limit':64,'output_k':'uiPending()'},'initial_configuration':initial,'startup':{'rule_start_line':74,'charge':0,'post_state_index':0},'states':states,'publication':{'pre_state_index':n,'rule_start_line':80,'charge':0,'post_overrides':{'control_k':'.K','output_k':f'uiDone({value})'}},'ordinary_fuel_prefixes':prefixes,'original_fixture_run':{'initial_fuel':1000,'stack_limit':64,'boundary_state_index':n,'remaining_fuel':1000-n,'consumed_work':n,'published_output_k':f'uiDone({value})','publication_rule_start_line':80,'reconstruction':'Use the same complete literal states and fixed code; replace initial_fuel by 1000 and remaining_fuel by 1000-charged_steps in every state. Startup control uses the unchanged original fixture run; final publication changes only control/output as above.'}})
common={k:base[k] for k in ('profile','source')}
common['baseline_commit']='c9f7cf6a712c7e6069cdc7a3ecdd86fb79817429'
pdoc={'format':'sherlock-literal-normal-inputs/1','status':'hand-authored source preparation; specified-only/unexecuted',**common,'programs':programs,'scope':{'case_ids':['F06bool','F07caller','F08mutual'],'coverage_status':'Three original frozen fixture full-state A tables only. Existing eight normal programs and 216 ordinary prefixes unchanged. No B, finance, theorem, circuit, native or target claim.'}}
tdoc={'format':'sherlock-literal-normal-prefixes/1','status':'hand-derived source expectations; specified-only/unexecuted',**common,'representation':base['representation'],'traces':traces,'coverage_status':'Provisional independent source expectations for three original fixtures only; no subject execution or review claim.'}
for filename,document in [('fixture-normal-programs.json',pdoc),('fixture-normal-prefixes.json',tdoc)]:
    (R/filename).write_text(json.dumps(document,indent=2,ensure_ascii=False)+'\n')
print(json.dumps({'status':'literal serialization only','programs':len(programs),'states':sum(len(t['states']) for t in traces),'ordinary_fuel_prefixes':sum(len(t['ordinary_fuel_prefixes']) for t in traces),'cases':[{'case_id':p['case_id'],'exact_completion_fuel':p['exact_completion_fuel'],'maximum_depth':p['expected_maximum_depth'],'input_sha256':p['input_sha256']} for p in programs]},indent=2))
