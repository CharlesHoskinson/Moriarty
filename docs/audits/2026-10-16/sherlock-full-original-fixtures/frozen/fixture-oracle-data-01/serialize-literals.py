"""Serialize handwritten, fixed-fixture literal tables; no machine evaluator.

All controls, environments, frame sequences and rule edges below were derived
from the pinned source before this script runs. Constructors below format K
text only. No routine chooses a rule, interprets an expression, executes a
program or traverses a subject transition relation.
"""
from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).parent
W = Path('/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016')
BASELINE = 'c9f7cf6a712c7e6069cdc7a3ecdd86fb79817429'
SOURCE = {'path': 'models/sherlock/k/main.k', 'sha256': 'b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41'}
PROFILE = 'sherlock-k-a/1'
PFILE = 'fixture-normal-programs.json'
NIL = 'uiNil()'
EMPTY = '.Map'
NC = 'uiNoCases()'
NF = 'uiNoFuns()'
NE = 'uiNoExprs()'
NV = 'uiNoValues()'
U0, U1, U2, U3, U4, U5, U6, U9 = [f'uiU({i})' for i in [0,1,2,3,4,5,6,9]]

# Text-formatting abbreviations, not semantic operations.
def ev(x): return f'uiEval({x})'
def ret(x): return f'uiRet({x})'
def literal(x): return f'uiLiteral({x})'
def push(fr, c, rho): return f'uiPush({fr}, {c}, {rho})'
def vals(x): return f'uiValue({x}, {NV})'
def call(n, x): return f'uiCall({n}, uiExpr({x}, {NE}))'
def args(n, rho): return f'uiArgsF({n}, {NV}, {NE}, {rho})'
def invoke(n, x): return f'uiInvoke({n}, {vals(x)})'
def enter(fun, x): return f'uiEnter({fun}, {vals(x)})'
def bound(rho, body): return f'uiBound(uiBindings({rho}), uiUIntT(), {body})'
def rf(rho): return f'uiReturnF(uiUIntT(), {rho})'
def sf(rho): return f'uiScopeF({rho})'
def mf(cs, rho): return f'uiMatchF({cs}, {rho})'
def lf(op, right, rho): return f'uiLeftF({op}, {right}, {rho})'
def right(op, val): return f'uiRightF({op}, {val})'
def prim(val): return f'uiPrimResult(uiPrimValue({val}))'
def cases(val, cs, rho): return f'uiCases({val}, {cs}, {rho})'
def result(match, val, exp, rest, rho):
    return f'uiCaseResult({match}, {val}, {exp}, {rest}, {rho})'
def matched(rho): return f'uiMatched({rho})'

# F03 source literals. L1/L2/L3/L4 identify the four separately entered
# callee environments; B1/B2/B3 identify their successful cons-case scopes.
V3 = f'uiCons({U3}, {NIL})'
V23 = f'uiCons({U2}, {V3})'
V123 = f'uiCons({U1}, {V23})'
L1, L2, L3, L4 = [f'0 |-> {v}' for v in [V123, V23, V3, NIL]]
B1 = f'{L1} 1 |-> {U1} 2 |-> {V23}'
B2 = f'{L2} 1 |-> {U2} 2 |-> {V3}'
B3 = f'{L3} 1 |-> {U3} 2 |-> {NIL}'
LCALL = call(3, 'uiVar(2)')
LADD = f'uiBinary(uiAdd(), uiVar(1), {LCALL})'
LCONS = f'uiCase(uiPCons(1, 2), {LADD}, {NC})'
LCASES = f'uiCase(uiPNil(), {literal(U0)}, {LCONS})'
LBODY = f'uiMatch(uiVar(0), {LCASES})'
LFUN = f'uiFun(3, uiParam(0, uiListT(), uiNoParams()), uiUIntT(), {LBODY})'
LCODE = f'uiFunction({LFUN}, {NF})'
LENTRY = call(3, literal(V123))

# These are explicit saved frame arrays, top first. Array construction merely
# expands the handwritten suffix names; it does not push/pop by a rule.
S0 = []
S1 = [rf(EMPTY)]
S2 = [sf(L1)] + S1
S3 = [right('uiAdd()', U1)] + S2
S4 = [rf(B1)] + S3
S5 = [sf(L2)] + S4
S6 = [right('uiAdd()', U2)] + S5
S7 = [rf(B2)] + S6
S8 = [sf(L3)] + S7
S9 = [right('uiAdd()', U3)] + S8
S10 = [rf(B3)] + S9
S11 = [sf(L4)] + S10

# Each tuple is (source rule start line, complete control, complete env,
# complete top-first frames), with no inferred control/env/frame fields.
LIST_ROWS = [
 (None, ev(LENTRY), EMPTY, S0),                                      # 0
 (131, push(args(3,EMPTY), ev(literal(V123)), EMPTY), EMPTY, S0),     # 1
 (135, ev(literal(V123)), EMPTY, [args(3,EMPTY)]),                    # 2
 (102, ret(V123), EMPTY, [args(3,EMPTY)]),                           # 3
 (181, invoke(3,V123), EMPTY, S0),                                  # 4
 (200, enter(LFUN,V123), EMPTY, S0),                                # 5
 (204, bound(L1,LBODY), EMPTY, S0),                                 # 6
 (208, push(rf(EMPTY), ev(LBODY), L1), EMPTY, S0),                    # 7
 (135, ev(LBODY), L1, S1),                                         # 8
 (126, push(mf(LCASES,L1),ev('uiVar(0)'),L1), L1, S1),               # 9
 (135, ev('uiVar(0)'), L1, [mf(LCASES,L1)]+S1),                     # 10
 (108, ret(V123), L1, [mf(LCASES,L1)]+S1),                          # 11
 (173, cases(V123,LCASES,L1), L1, S1),                              # 12
 (214, result('uiNoMatch()',V123,literal(U0),LCONS,L1), L1, S1),    # 13
 (217, cases(V123,LCONS,L1), L1, S1),                               # 14
 (214, result(matched(B1),V123,LADD,NC,L1), L1, S1),                # 15
 (220, push(sf(L1),ev(LADD),B1), L1, S1),                           # 16
 (135, ev(LADD), B1, S2),                                         # 17
 (117, push(lf('uiAdd()',LCALL,B1),ev('uiVar(1)'),B1), B1, S2),      # 18
 (135, ev('uiVar(1)'), B1, [lf('uiAdd()',LCALL,B1)]+S2),            # 19
 (108, ret(U1), B1, [lf('uiAdd()',LCALL,B1)]+S2),                   # 20
 (149, ev(LCALL), B1, S3),                                        # 21
 (131, push(args(3,B1),ev('uiVar(2)'),B1), B1, S3),                 # 22
 (135, ev('uiVar(2)'), B1, [args(3,B1)]+S3),                       # 23
 (108, ret(V23), B1, [args(3,B1)]+S3),                             # 24
 (181, invoke(3,V23), B1, S3),                                    # 25
 (200, enter(LFUN,V23), B1, S3),                                  # 26
 (204, bound(L2,LBODY), B1, S3),                                   # 27
 (208, push(rf(B1),ev(LBODY),L2), B1, S3),                          # 28
 (135, ev(LBODY), L2, S4),                                        # 29
 (126, push(mf(LCASES,L2),ev('uiVar(0)'),L2), L2, S4),               # 30
 (135, ev('uiVar(0)'), L2, [mf(LCASES,L2)]+S4),                     # 31
 (108, ret(V23), L2, [mf(LCASES,L2)]+S4),                           # 32
 (173, cases(V23,LCASES,L2), L2, S4),                               # 33
 (214, result('uiNoMatch()',V23,literal(U0),LCONS,L2), L2, S4),     # 34
 (217, cases(V23,LCONS,L2), L2, S4),                                # 35
 (214, result(matched(B2),V23,LADD,NC,L2), L2, S4),                 # 36
 (220, push(sf(L2),ev(LADD),B2), L2, S4),                           # 37
 (135, ev(LADD), B2, S5),                                         # 38
 (117, push(lf('uiAdd()',LCALL,B2),ev('uiVar(1)'),B2), B2, S5),      # 39
 (135, ev('uiVar(1)'), B2, [lf('uiAdd()',LCALL,B2)]+S5),            # 40
 (108, ret(U2), B2, [lf('uiAdd()',LCALL,B2)]+S5),                   # 41
 (149, ev(LCALL), B2, S6),                                        # 42
 (131, push(args(3,B2),ev('uiVar(2)'),B2), B2, S6),                 # 43
 (135, ev('uiVar(2)'), B2, [args(3,B2)]+S6),                       # 44
 (108, ret(V3), B2, [args(3,B2)]+S6),                              # 45
 (181, invoke(3,V3), B2, S6),                                     # 46
 (200, enter(LFUN,V3), B2, S6),                                   # 47
 (204, bound(L3,LBODY), B2, S6),                                   # 48
 (208, push(rf(B2),ev(LBODY),L3), B2, S6),                          # 49
 (135, ev(LBODY), L3, S7),                                        # 50
 (126, push(mf(LCASES,L3),ev('uiVar(0)'),L3), L3, S7),               # 51
 (135, ev('uiVar(0)'), L3, [mf(LCASES,L3)]+S7),                     # 52
 (108, ret(V3), L3, [mf(LCASES,L3)]+S7),                            # 53
 (173, cases(V3,LCASES,L3), L3, S7),                                # 54
 (214, result('uiNoMatch()',V3,literal(U0),LCONS,L3), L3, S7),      # 55
 (217, cases(V3,LCONS,L3), L3, S7),                                 # 56
 (214, result(matched(B3),V3,LADD,NC,L3), L3, S7),                  # 57
 (220, push(sf(L3),ev(LADD),B3), L3, S7),                           # 58
 (135, ev(LADD), B3, S8),                                         # 59
 (117, push(lf('uiAdd()',LCALL,B3),ev('uiVar(1)'),B3), B3, S8),      # 60
 (135, ev('uiVar(1)'), B3, [lf('uiAdd()',LCALL,B3)]+S8),            # 61
 (108, ret(U3), B3, [lf('uiAdd()',LCALL,B3)]+S8),                   # 62
 (149, ev(LCALL), B3, S9),                                        # 63
 (131, push(args(3,B3),ev('uiVar(2)'),B3), B3, S9),                 # 64
 (135, ev('uiVar(2)'), B3, [args(3,B3)]+S9),                       # 65
 (108, ret(NIL), B3, [args(3,B3)]+S9),                             # 66
 (181, invoke(3,NIL), B3, S9),                                    # 67
 (200, enter(LFUN,NIL), B3, S9),                                  # 68
 (204, bound(L4,LBODY), B3, S9),                                   # 69
 (208, push(rf(B3),ev(LBODY),L4), B3, S9),                          # 70
 (135, ev(LBODY), L4, S10),                                       # 71
 (126, push(mf(LCASES,L4),ev('uiVar(0)'),L4), L4, S10),              # 72
 (135, ev('uiVar(0)'), L4, [mf(LCASES,L4)]+S10),                    # 73
 (108, ret(NIL), L4, [mf(LCASES,L4)]+S10),                          # 74
 (173, cases(NIL,LCASES,L4), L4, S10),                              # 75
 (214, result(matched(L4),NIL,literal(U0),LCONS,L4), L4, S10),      # 76
 (220, push(sf(L4),ev(literal(U0)),L4), L4, S10),                    # 77
 (135, ev(literal(U0)), L4, S11),                                  # 78
 (102, ret(U0), L4, S11),                                         # 79
 (195, ret(U0), L4, S10),                                         # 80
 (185, ret(U0), B3, S9),                                          # 81
 (153, prim(U3), B3, S8),                                         # 82: 3+0
 (223, ret(U3), B3, S8),                                          # 83
 (195, ret(U3), L3, S7),                                          # 84
 (185, ret(U3), B2, S6),                                          # 85
 (153, prim(U5), B2, S5),                                         # 86: 2+3
 (223, ret(U5), B2, S5),                                          # 87
 (195, ret(U5), L2, S4),                                          # 88
 (185, ret(U5), B1, S3),                                          # 89
 (153, prim(U6), B1, S2),                                         # 90: 1+5
 (223, ret(U6), B1, S2),                                          # 91
 (195, ret(U6), L1, S1),                                          # 92
 (185, ret(U6), EMPTY, S0),                                       # 93
]

# F04 source literals and all saved environments/frames.
LEAF4 = f'uiLeaf({U4})'
LEAF5 = f'uiLeaf({U5})'
TREE = f'uiBranch({LEAF4}, {LEAF5})'
T0 = f'0 |-> {TREE}'
TB = f'{T0} 1 |-> {LEAF4} 2 |-> {LEAF5}'
T4 = f'0 |-> {LEAF4}'
T5 = f'0 |-> {LEAF5}'
T4B = f'{T4} 1 |-> {U4}'
T5B = f'{T5} 1 |-> {U5}'
TCALL1 = call(4, 'uiVar(1)')
TCALL2 = call(4, 'uiVar(2)')
TADD = f'uiBinary(uiAdd(), {TCALL1}, {TCALL2})'
TBRANCH = f'uiCase(uiPBranch(1, 2), {TADD}, {NC})'
TCASES = f'uiCase(uiPLeaf(1), uiVar(1), {TBRANCH})'
TBODY = f'uiMatch(uiVar(0), {TCASES})'
TFUN = f'uiFun(4, uiParam(0, uiTreeT(), uiNoParams()), uiUIntT(), {TBODY})'
TCODE = f'uiFunction({TFUN}, {NF})'
TENTRY = call(4, literal(TREE))
TSTACK1 = [rf(EMPTY)]
TSTACK2 = [sf(T0)] + TSTACK1
TLEFT = [lf('uiAdd()', TCALL2, TB)] + TSTACK2
TLEFTRET = [rf(TB)] + TLEFT
TLEFTSCOPE = [sf(T4)] + TLEFTRET
TRIGHT = [right('uiAdd()',U4)] + TSTACK2
TRIGHTRET = [rf(TB)] + TRIGHT
TRIGHTSCOPE = [sf(T5)] + TRIGHTRET

TREE_ROWS = [
 (None, ev(TENTRY), EMPTY, S0),                                       # 0
 (131, push(args(4,EMPTY),ev(literal(TREE)),EMPTY), EMPTY, S0),        # 1
 (135, ev(literal(TREE)), EMPTY, [args(4,EMPTY)]),                     # 2
 (102, ret(TREE), EMPTY, [args(4,EMPTY)]),                            # 3
 (181, invoke(4,TREE), EMPTY, S0),                                   # 4
 (200, enter(TFUN,TREE), EMPTY, S0),                                 # 5
 (204, bound(T0,TBODY), EMPTY, S0),                                  # 6
 (208, push(rf(EMPTY),ev(TBODY),T0), EMPTY, S0),                       # 7
 (135, ev(TBODY), T0, TSTACK1),                                      # 8
 (126, push(mf(TCASES,T0),ev('uiVar(0)'),T0), T0, TSTACK1),           # 9
 (135, ev('uiVar(0)'), T0, [mf(TCASES,T0)]+TSTACK1),                  # 10
 (108, ret(TREE), T0, [mf(TCASES,T0)]+TSTACK1),                       # 11
 (173, cases(TREE,TCASES,T0), T0, TSTACK1),                           # 12
 (214, result('uiNoMatch()',TREE,'uiVar(1)',TBRANCH,T0), T0,TSTACK1), # 13
 (217, cases(TREE,TBRANCH,T0), T0, TSTACK1),                          # 14
 (214, result(matched(TB),TREE,TADD,NC,T0), T0, TSTACK1),             # 15
 (220, push(sf(T0),ev(TADD),TB), T0, TSTACK1),                        # 16
 (135, ev(TADD), TB, TSTACK2),                                      # 17
 (117, push(lf('uiAdd()',TCALL2,TB),ev(TCALL1),TB), TB, TSTACK2),     # 18
 (135, ev(TCALL1), TB, TLEFT),                                      # 19
 (131, push(args(4,TB),ev('uiVar(1)'),TB), TB, TLEFT),                # 20
 (135, ev('uiVar(1)'), TB, [args(4,TB)]+TLEFT),                      # 21
 (108, ret(LEAF4), TB, [args(4,TB)]+TLEFT),                          # 22
 (181, invoke(4,LEAF4), TB, TLEFT),                                 # 23
 (200, enter(TFUN,LEAF4), TB, TLEFT),                               # 24
 (204, bound(T4,TBODY), TB, TLEFT),                                 # 25
 (208, push(rf(TB),ev(TBODY),T4), TB, TLEFT),                        # 26
 (135, ev(TBODY), T4, TLEFTRET),                                    # 27
 (126, push(mf(TCASES,T4),ev('uiVar(0)'),T4), T4, TLEFTRET),         # 28
 (135, ev('uiVar(0)'), T4, [mf(TCASES,T4)]+TLEFTRET),                # 29
 (108, ret(LEAF4), T4, [mf(TCASES,T4)]+TLEFTRET),                    # 30
 (173, cases(LEAF4,TCASES,T4), T4, TLEFTRET),                        # 31
 (214, result(matched(T4B),LEAF4,'uiVar(1)',TBRANCH,T4),T4,TLEFTRET),# 32
 (220, push(sf(T4),ev('uiVar(1)'),T4B), T4, TLEFTRET),               # 33
 (135, ev('uiVar(1)'), T4B, TLEFTSCOPE),                            # 34
 (108, ret(U4), T4B, TLEFTSCOPE),                                  # 35
 (195, ret(U4), T4, TLEFTRET),                                     # 36
 (185, ret(U4), TB, TLEFT),                                        # 37
 (149, ev(TCALL2), TB, TRIGHT),                                    # 38
 (131, push(args(4,TB),ev('uiVar(2)'),TB), TB, TRIGHT),              # 39
 (135, ev('uiVar(2)'), TB, [args(4,TB)]+TRIGHT),                    # 40
 (108, ret(LEAF5), TB, [args(4,TB)]+TRIGHT),                        # 41
 (181, invoke(4,LEAF5), TB, TRIGHT),                               # 42
 (200, enter(TFUN,LEAF5), TB, TRIGHT),                             # 43
 (204, bound(T5,TBODY), TB, TRIGHT),                               # 44
 (208, push(rf(TB),ev(TBODY),T5), TB, TRIGHT),                      # 45
 (135, ev(TBODY), T5, TRIGHTRET),                                  # 46
 (126, push(mf(TCASES,T5),ev('uiVar(0)'),T5), T5, TRIGHTRET),       # 47
 (135, ev('uiVar(0)'), T5, [mf(TCASES,T5)]+TRIGHTRET),              # 48
 (108, ret(LEAF5), T5, [mf(TCASES,T5)]+TRIGHTRET),                  # 49
 (173, cases(LEAF5,TCASES,T5), T5, TRIGHTRET),                      # 50
 (214, result(matched(T5B),LEAF5,'uiVar(1)',TBRANCH,T5),T5,TRIGHTRET),# 51
 (220, push(sf(T5),ev('uiVar(1)'),T5B), T5, TRIGHTRET),             # 52
 (135, ev('uiVar(1)'), T5B, TRIGHTSCOPE),                          # 53
 (108, ret(U5), T5B, TRIGHTSCOPE),                                # 54
 (195, ret(U5), T5, TRIGHTRET),                                   # 55
 (185, ret(U5), TB, TRIGHT),                                      # 56
 (153, prim(U9), TB, TSTACK2),                                    # 57: 4+5
 (223, ret(U9), TB, TSTACK2),                                     # 58
 (195, ret(U9), T0, TSTACK1),                                     # 59
 (185, ret(U9), EMPTY, S0),                                       # 60
]

# F05: both constructor fields evaluated in source order, nested right call.
C2 = f'uiCons({U2}, {NIL})'
C12 = f'uiCons({U1}, {C2})'
CINNER = f'uiBinary(uiConsOp(), {literal(U2)}, {literal(NIL)})'
CENTRY = f'uiBinary(uiConsOp(), {literal(U1)}, {CINNER})'
CLEFT1 = lf('uiConsOp()', CINNER, EMPTY)
CRIGHT1 = right('uiConsOp()', U1)
CLEFT2 = lf('uiConsOp()', literal(NIL), EMPTY)
CRIGHT2 = right('uiConsOp()', U2)
CONSTRUCT_ROWS = [
 (None, ev(CENTRY), EMPTY, []),                                      # 0
 (117, push(CLEFT1,ev(literal(U1)),EMPTY), EMPTY, []),                # 1
 (135, ev(literal(U1)), EMPTY, [CLEFT1]),                            # 2
 (102, ret(U1), EMPTY, [CLEFT1]),                                   # 3
 (149, ev(CINNER), EMPTY, [CRIGHT1]),                               # 4
 (117, push(CLEFT2,ev(literal(U2)),EMPTY), EMPTY, [CRIGHT1]),          # 5
 (135, ev(literal(U2)), EMPTY, [CLEFT2,CRIGHT1]),                    # 6
 (102, ret(U2), EMPTY, [CLEFT2,CRIGHT1]),                            # 7
 (149, ev(literal(NIL)), EMPTY, [CRIGHT2,CRIGHT1]),                   # 8
 (102, ret(NIL), EMPTY, [CRIGHT2,CRIGHT1]),                           # 9
 (153, prim(C2), EMPTY, [CRIGHT1]),                                 # 10
 (223, ret(C2), EMPTY, [CRIGHT1]),                                  # 11
 (153, prim(C12), EMPTY, []),                                      # 12
 (223, ret(C12), EMPTY, []),                                       # 13
]

# Fixed counts, values and prefix rows were hand-derived. Serialization uses
# enumeration only to expand already fixed row positions and fuel subtractions.
SPECS = [
 ('F03','uiListsum123.ui',LCODE,LENTRY,93,U6,11,LIST_ROWS,
  'Full frozen list[1,2,3] sum fixture',
  '8+3*(9+4+8)+10+3*4=93. Source-order nil-arm miss/cons-arm match retained three times. Results unwind 3+0=3, 2+3=5, 1+5=6.'),
 ('F04','uiTreesum45.ui',TCODE,TENTRY,60,U9,5,TREE_ROWS,
  'Full frozen tree(leaf4,leaf5) sum fixture',
  '8+9+2+18+1+18+2+2=60. Pending right-tree call is retained throughout the first leaf; 4+5=9.'),
 ('F05','uiConstructList.ui',NF,CENTRY,13,C12,2,CONSTRUCT_ROWS,
  'Full frozen constructor list[1,2] fixture',
  'Outer 7-step binary skeleton plus inner 7-step skeleton replacing one 1-step literal: 7+7-1=13. Inner cons(2,nil) completes before outer cons(1,inner).'),
]

original = json.loads((W/'models/sherlock/oracles/normal-prefixes.json').read_text())
programs = []
traces = []
for cid, filename, code, entry, count, expected, maxdepth, rows, title, derivation in SPECS:
    assert len(rows) == count+1
    assert max(len(row[3]) for row in rows) == maxdepth
    assert rows[-1][1:] == (ret(expected), EMPTY, [])
    source_fields = dict(case_id=cid, code_k=code, entry_k=entry,
        initial_arguments='Explicit literals in entry_k; no external arguments.',
        stack_limit=64, exact_completion_fuel=count)
    digest = hashlib.sha256(json.dumps(source_fields,sort_keys=True,separators=(',',':')).encode()).hexdigest()
    fixture_path = 'models/sherlock/k/fixtures/'+filename
    fixture_digest = hashlib.sha256((W/fixture_path).read_bytes()).hexdigest()
    p = dict(source_fields, input_sha256=digest,
        sha256_convention='SHA256 of UTF-8 compact sorted-key JSON of the six preceding source fields, excluding digest/convention and metadata.',
        title=title, expected_value_k=expected, derivation=derivation,
        original_fixture=dict(path=fixture_path,sha256=fixture_digest,initial_fuel=1000,stack_limit=64),
        expected_maximum_frame_depth=maxdepth)
    programs.append(p)
    initial = dict(control_k=f'uiRun({code}, {entry}, {count}, 64)',
        environment_k=EMPTY,frames_top_first_k=[],depth=0,stack_limit=0,
        code_k=NF,remaining_fuel=0,initial_fuel=0,output_k='uiPending()',charged_steps=0)
    states = [dict(charged_steps=i,remaining_fuel=count-i,depth=len(frames),
        control_k=control,environment_k=rho,frames_top_first_k=frames,
        via_rule_start_line=line) for i,(line,control,rho,frames) in enumerate(rows)]
    output = f'uiDone({expected})'
    # Literal ordinary budgets are a source-independent projection of the fixed
    # snapshot indices, never a re-evaluation of expressions or main.k.
    prefixes = [[i,i,0,'uiExhausted()',84] for i in range(count)] + [
        [count,count,0,output,80],[count+1,count,1,output,80]]
    trace = dict(case_id=cid,input_sha256=digest,
        fixed_state_fields=dict(code_ref=dict(file=PFILE,case_id=cid,input_sha256=digest),
            initial_fuel=count,stack_limit=64,output_k='uiPending()'),
        initial_configuration=initial,
        startup=dict(rule_start_line=74,charge=0,post_state_index=0),
        states=states,
        publication=dict(pre_state_index=count,rule_start_line=80,charge=0,
            post_overrides=dict(control_k='.K',output_k=output)),
        ordinary_fuel_prefixes=prefixes)
    trace['original_fixture_run'] = dict(
        source_path=fixture_path,source_sha256=fixture_digest,
        initial_configuration=dict(initial,control_k=f'uiRun({code}, {entry}, 1000, 64)'),
        fixed_state_overrides=dict(initial_fuel=1000,stack_limit=64),
        state_fuel_columns=['state_index','remaining_fuel'],
        state_fuel_overlays=[[i,1000-i] for i in range(count+1)],
        state_reconstruction='Resolve every states[i] as above, overlay initial_fuel=1000 and the explicit state_fuel_overlays remaining_fuel. Startup and publication are the same separate zero-charge edges. These snapshots preserve every original source fixture field; no ordinary fuel-prefix budget is substituted into the frozen fixture.',
        publication=dict(pre_state_index=count,rule_start_line=80,charge=0,
            post_overrides=dict(control_k='.K',output_k=output)),
        terminal=dict(control_k='.K',environment_k=EMPTY,frames_top_first_k=[],depth=0,
            code_ref=dict(file=PFILE,case_id=cid,input_sha256=digest),
            stack_limit=64,initial_fuel=1000,remaining_fuel=1000-count,
            charged_steps=count,output_k=output))
    traces.append(trace)

common = dict(status='hand-derived source expectations; provisional specified-only/unexecuted',
              profile=PROFILE,baseline_commit=BASELINE,source=SOURCE)
program_pack = dict(format='sherlock-literal-normal-inputs/1',**common,programs=programs,
    scope='Full original F03/F04/F05 fixture source expectations only. Existing normal/extra-normal cases and prefixes remain immutable. Helpers remain unmetered in profile A; no B, finance, execution, correspondence, proof or acceptance result.')
prefix_pack = dict(format='sherlock-literal-normal-prefixes/1',**common,
    representation=original['representation'],traces=traces,
    coverage_status='Three full original data fixtures specified independently before producer execution; no execution or review claim.')
for name, value in [(PFILE,program_pack),('fixture-normal-prefixes.json',prefix_pack)]:
    (ROOT/name).write_text(json.dumps(value,indent=2)+'\n')

# Syntax/shape assertions only; no comparison with subject code or output.
for trace in traces:
    n = len(trace['states'])-1
    assert len(trace['ordinary_fuel_prefixes']) == n+2
    assert [row[0] for row in trace['ordinary_fuel_prefixes']] == list(range(n+2))
    assert trace['ordinary_fuel_prefixes'][-2][3] == trace['publication']['post_overrides']['output_k']
    assert trace['states'][-1]['remaining_fuel'] == 0
    for state in trace['states']:
        assert state['depth'] == len(state['frames_top_first_k'])
        assert state['remaining_fuel'] + state['charged_steps'] == n
    for row in trace['ordinary_fuel_prefixes']:
        assert row[0] - row[1] == row[2]
print(json.dumps({'status':'serialized fixed handwritten tables; syntax/shape assertions only',
    'programs':len(programs),'states':sum(len(t['states']) for t in traces),
    'ordinary_prefixes':sum(len(t['ordinary_fuel_prefixes']) for t in traces),
    'completion_work':{p['case_id']:p['exact_completion_fuel'] for p in programs}},indent=2))
