"""Serialize hand-derived fixed tables only. No subject imports or transition engine."""
from pathlib import Path
from string import Template
import hashlib,json,copy
R=Path(__file__).parent
W=Path('/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016')
BASE='c9f7cf6a712c7e6069cdc7a3ecdd86fb79817429'
SOURCE={'path':'models/sherlock/k/main.k','sha256':'b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41'}
# Literal source-text abbreviations, independently transcribed from the two pinned fixtures.
common={
 'Z':'uiLiteral(uiU(0))','ONE':'uiLiteral(uiU(1))','THREE':'uiLiteral(uiU(3))','V':'uiVar(0)',
 'e3':'0 |-> uiU(3)','e2':'0 |-> uiU(2)','e1':'0 |-> uiU(1)','e0':'0 |-> uiU(0)','empty':'.Map',
 'Q':'uiBinary(uiEq(), uiVar(0), uiLiteral(uiU(0)))',
 'SUB':'uiBinary(uiSub(), uiVar(0), uiLiteral(uiU(1)))',
 'LQ3':'uiLeftF(uiEq(), uiLiteral(uiU(0)), 0 |-> uiU(3))',
 'LQ2':'uiLeftF(uiEq(), uiLiteral(uiU(0)), 0 |-> uiU(2))',
 'LQ1':'uiLeftF(uiEq(), uiLiteral(uiU(0)), 0 |-> uiU(1))',
 'LQ0':'uiLeftF(uiEq(), uiLiteral(uiU(0)), 0 |-> uiU(0))',
 'LS3':'uiLeftF(uiSub(), uiLiteral(uiU(1)), 0 |-> uiU(3))',
 'LS2':'uiLeftF(uiSub(), uiLiteral(uiU(1)), 0 |-> uiU(2))',
 'LS1':'uiLeftF(uiSub(), uiLiteral(uiU(1)), 0 |-> uiU(1))',
 'RQ3':'uiRightF(uiEq(), uiU(3))','RQ2':'uiRightF(uiEq(), uiU(2))','RQ1':'uiRightF(uiEq(), uiU(1))','RQ0':'uiRightF(uiEq(), uiU(0))',
 'RS3':'uiRightF(uiSub(), uiU(3))','RS2':'uiRightF(uiSub(), uiU(2))','RS1':'uiRightF(uiSub(), uiU(1))',
 'R3':'uiReturnF(uiUIntT(), .Map)','R2':'uiReturnF(uiUIntT(), 0 |-> uiU(3))','R1':'uiReturnF(uiUIntT(), 0 |-> uiU(2))','R0':'uiReturnF(uiUIntT(), 0 |-> uiU(1))',
}
# Fixed 114-row ledger. Columns: incoming source line, control literal, full current map, full top-first stack.
# All source-text aliases expand before JSON publication. Rows do not dispatch on a source expression or rule.
rows=[
(None,'uiEval($ENTRY)','empty',[]),
(131,'uiPush($A3, uiEval($THREE), .Map)','empty',[]),
(135,'uiEval($THREE)','empty',['A3']),
(102,'uiRet(uiU(3))','empty',['A3']),
(181,'uiInvoke($N, uiValue(uiU(3), uiNoValues()))','empty',[]),
(200,'uiEnter($FUN, uiValue(uiU(3), uiNoValues()))','empty',[]),
(204,'uiBound(uiBindings($e3), uiUIntT(), $BODY)','empty',[]),
(208,'uiPush($R3, uiEval($BODY), $e3)','empty',[]),
(135,'uiEval($BODY)','e3',['R3']),
(123,'uiPush($I3, uiEval($Q), $e3)','e3',['R3']),
(135,'uiEval($Q)','e3',['I3','R3']),
(117,'uiPush($LQ3, uiEval($V), $e3)','e3',['I3','R3']),
(135,'uiEval($V)','e3',['LQ3','I3','R3']),
(108,'uiRet(uiU(3))','e3',['LQ3','I3','R3']),
(149,'uiEval($Z)','e3',['RQ3','I3','R3']),
(102,'uiRet(uiU(0))','e3',['RQ3','I3','R3']),
(153,'uiPrimResult(uiPrimValue(uiB(false)))','e3',['I3','R3']),
(223,'uiRet(uiB(false))','e3',['I3','R3']),
(165,'uiEval($REC)','e3',['R3']),
(117,'uiPush($L3, uiEval($V), $e3)','e3',['R3']),
(135,'uiEval($V)','e3',['L3','R3']),
(108,'uiRet(uiU(3))','e3',['L3','R3']),
(149,'uiEval($CALL)','e3',['P3','R3']),
(131,'uiPush($A2, uiEval($SUB), $e3)','e3',['P3','R3']),
(135,'uiEval($SUB)','e3',['A2','P3','R3']),
(117,'uiPush($LS3, uiEval($V), $e3)','e3',['A2','P3','R3']),
(135,'uiEval($V)','e3',['LS3','A2','P3','R3']),
(108,'uiRet(uiU(3))','e3',['LS3','A2','P3','R3']),
(149,'uiEval($ONE)','e3',['RS3','A2','P3','R3']),
(102,'uiRet(uiU(1))','e3',['RS3','A2','P3','R3']),
(153,'uiPrimResult(uiPrimValue(uiU(2)))','e3',['A2','P3','R3']),
(223,'uiRet(uiU(2))','e3',['A2','P3','R3']),
(181,'uiInvoke($N, uiValue(uiU(2), uiNoValues()))','e3',['P3','R3']),
(200,'uiEnter($FUN, uiValue(uiU(2), uiNoValues()))','e3',['P3','R3']),
(204,'uiBound(uiBindings($e2), uiUIntT(), $BODY)','e3',['P3','R3']),
(208,'uiPush($R2, uiEval($BODY), $e2)','e3',['P3','R3']),
(135,'uiEval($BODY)','e2',['R2','P3','R3']),
(123,'uiPush($I2, uiEval($Q), $e2)','e2',['R2','P3','R3']),
(135,'uiEval($Q)','e2',['I2','R2','P3','R3']),
(117,'uiPush($LQ2, uiEval($V), $e2)','e2',['I2','R2','P3','R3']),
(135,'uiEval($V)','e2',['LQ2','I2','R2','P3','R3']),
(108,'uiRet(uiU(2))','e2',['LQ2','I2','R2','P3','R3']),
(149,'uiEval($Z)','e2',['RQ2','I2','R2','P3','R3']),
(102,'uiRet(uiU(0))','e2',['RQ2','I2','R2','P3','R3']),
(153,'uiPrimResult(uiPrimValue(uiB(false)))','e2',['I2','R2','P3','R3']),
(223,'uiRet(uiB(false))','e2',['I2','R2','P3','R3']),
(165,'uiEval($REC)','e2',['R2','P3','R3']),
(117,'uiPush($L2, uiEval($V), $e2)','e2',['R2','P3','R3']),
(135,'uiEval($V)','e2',['L2','R2','P3','R3']),
(108,'uiRet(uiU(2))','e2',['L2','R2','P3','R3']),
(149,'uiEval($CALL)','e2',['P2','R2','P3','R3']),
(131,'uiPush($A1, uiEval($SUB), $e2)','e2',['P2','R2','P3','R3']),
(135,'uiEval($SUB)','e2',['A1','P2','R2','P3','R3']),
(117,'uiPush($LS2, uiEval($V), $e2)','e2',['A1','P2','R2','P3','R3']),
(135,'uiEval($V)','e2',['LS2','A1','P2','R2','P3','R3']),
(108,'uiRet(uiU(2))','e2',['LS2','A1','P2','R2','P3','R3']),
(149,'uiEval($ONE)','e2',['RS2','A1','P2','R2','P3','R3']),
(102,'uiRet(uiU(1))','e2',['RS2','A1','P2','R2','P3','R3']),
(153,'uiPrimResult(uiPrimValue(uiU(1)))','e2',['A1','P2','R2','P3','R3']),
(223,'uiRet(uiU(1))','e2',['A1','P2','R2','P3','R3']),
(181,'uiInvoke($N, uiValue(uiU(1), uiNoValues()))','e2',['P2','R2','P3','R3']),
(200,'uiEnter($FUN, uiValue(uiU(1), uiNoValues()))','e2',['P2','R2','P3','R3']),
(204,'uiBound(uiBindings($e1), uiUIntT(), $BODY)','e2',['P2','R2','P3','R3']),
(208,'uiPush($R1, uiEval($BODY), $e1)','e2',['P2','R2','P3','R3']),
(135,'uiEval($BODY)','e1',['R1','P2','R2','P3','R3']),
(123,'uiPush($I1, uiEval($Q), $e1)','e1',['R1','P2','R2','P3','R3']),
(135,'uiEval($Q)','e1',['I1','R1','P2','R2','P3','R3']),
(117,'uiPush($LQ1, uiEval($V), $e1)','e1',['I1','R1','P2','R2','P3','R3']),
(135,'uiEval($V)','e1',['LQ1','I1','R1','P2','R2','P3','R3']),
(108,'uiRet(uiU(1))','e1',['LQ1','I1','R1','P2','R2','P3','R3']),
(149,'uiEval($Z)','e1',['RQ1','I1','R1','P2','R2','P3','R3']),
(102,'uiRet(uiU(0))','e1',['RQ1','I1','R1','P2','R2','P3','R3']),
(153,'uiPrimResult(uiPrimValue(uiB(false)))','e1',['I1','R1','P2','R2','P3','R3']),
(223,'uiRet(uiB(false))','e1',['I1','R1','P2','R2','P3','R3']),
(165,'uiEval($REC)','e1',['R1','P2','R2','P3','R3']),
(117,'uiPush($L1, uiEval($V), $e1)','e1',['R1','P2','R2','P3','R3']),
(135,'uiEval($V)','e1',['L1','R1','P2','R2','P3','R3']),
(108,'uiRet(uiU(1))','e1',['L1','R1','P2','R2','P3','R3']),
(149,'uiEval($CALL)','e1',['P1','R1','P2','R2','P3','R3']),
(131,'uiPush($A0, uiEval($SUB), $e1)','e1',['P1','R1','P2','R2','P3','R3']),
(135,'uiEval($SUB)','e1',['A0','P1','R1','P2','R2','P3','R3']),
(117,'uiPush($LS1, uiEval($V), $e1)','e1',['A0','P1','R1','P2','R2','P3','R3']),
(135,'uiEval($V)','e1',['LS1','A0','P1','R1','P2','R2','P3','R3']),
(108,'uiRet(uiU(1))','e1',['LS1','A0','P1','R1','P2','R2','P3','R3']),
(149,'uiEval($ONE)','e1',['RS1','A0','P1','R1','P2','R2','P3','R3']),
(102,'uiRet(uiU(1))','e1',['RS1','A0','P1','R1','P2','R2','P3','R3']),
(153,'uiPrimResult(uiPrimValue(uiU(0)))','e1',['A0','P1','R1','P2','R2','P3','R3']),
(223,'uiRet(uiU(0))','e1',['A0','P1','R1','P2','R2','P3','R3']),
(181,'uiInvoke($N, uiValue(uiU(0), uiNoValues()))','e1',['P1','R1','P2','R2','P3','R3']),
(200,'uiEnter($FUN, uiValue(uiU(0), uiNoValues()))','e1',['P1','R1','P2','R2','P3','R3']),
(204,'uiBound(uiBindings($e0), uiUIntT(), $BODY)','e1',['P1','R1','P2','R2','P3','R3']),
(208,'uiPush($R0, uiEval($BODY), $e0)','e1',['P1','R1','P2','R2','P3','R3']),
(135,'uiEval($BODY)','e0',['R0','P1','R1','P2','R2','P3','R3']),
(123,'uiPush($I0, uiEval($Q), $e0)','e0',['R0','P1','R1','P2','R2','P3','R3']),
(135,'uiEval($Q)','e0',['I0','R0','P1','R1','P2','R2','P3','R3']),
(117,'uiPush($LQ0, uiEval($V), $e0)','e0',['I0','R0','P1','R1','P2','R2','P3','R3']),
(135,'uiEval($V)','e0',['LQ0','I0','R0','P1','R1','P2','R2','P3','R3']),
(108,'uiRet(uiU(0))','e0',['LQ0','I0','R0','P1','R1','P2','R2','P3','R3']),
(149,'uiEval($Z)','e0',['RQ0','I0','R0','P1','R1','P2','R2','P3','R3']),
(102,'uiRet(uiU(0))','e0',['RQ0','I0','R0','P1','R1','P2','R2','P3','R3']),
(153,'uiPrimResult(uiPrimValue(uiB(true)))','e0',['I0','R0','P1','R1','P2','R2','P3','R3']),
(223,'uiRet(uiB(true))','e0',['I0','R0','P1','R1','P2','R2','P3','R3']),
(161,'uiEval($BASE)','e0',['R0','P1','R1','P2','R2','P3','R3']),
(102,'uiRet($v0)','e0',['R0','P1','R1','P2','R2','P3','R3']),
(185,'uiRet($v0)','e1',['P1','R1','P2','R2','P3','R3']),
(153,'uiPrimResult(uiPrimValue($v1))','e1',['R1','P2','R2','P3','R3']),
(223,'uiRet($v1)','e1',['R1','P2','R2','P3','R3']),
(185,'uiRet($v1)','e2',['P2','R2','P3','R3']),
(153,'uiPrimResult(uiPrimValue($v2))','e2',['R2','P3','R3']),
(223,'uiRet($v2)','e2',['R2','P3','R3']),
(185,'uiRet($v2)','e3',['P3','R3']),
(153,'uiPrimResult(uiPrimValue(uiU(6)))','e3',['R3']),
(223,'uiRet(uiU(6))','e3',['R3']),
(185,'uiRet(uiU(6))','empty',[]),
]
assert len(rows)==114
# Explicit fixed ordinary budgets. They are data, not a semantic run.
prefixes=[
(0, 0, 0, 'uiExhausted()', 84),
(1, 1, 0, 'uiExhausted()', 84),
(2, 2, 0, 'uiExhausted()', 84),
(3, 3, 0, 'uiExhausted()', 84),
(4, 4, 0, 'uiExhausted()', 84),
(5, 5, 0, 'uiExhausted()', 84),
(6, 6, 0, 'uiExhausted()', 84),
(7, 7, 0, 'uiExhausted()', 84),
(8, 8, 0, 'uiExhausted()', 84),
(9, 9, 0, 'uiExhausted()', 84),
(10, 10, 0, 'uiExhausted()', 84),
(11, 11, 0, 'uiExhausted()', 84),
(12, 12, 0, 'uiExhausted()', 84),
(13, 13, 0, 'uiExhausted()', 84),
(14, 14, 0, 'uiExhausted()', 84),
(15, 15, 0, 'uiExhausted()', 84),
(16, 16, 0, 'uiExhausted()', 84),
(17, 17, 0, 'uiExhausted()', 84),
(18, 18, 0, 'uiExhausted()', 84),
(19, 19, 0, 'uiExhausted()', 84),
(20, 20, 0, 'uiExhausted()', 84),
(21, 21, 0, 'uiExhausted()', 84),
(22, 22, 0, 'uiExhausted()', 84),
(23, 23, 0, 'uiExhausted()', 84),
(24, 24, 0, 'uiExhausted()', 84),
(25, 25, 0, 'uiExhausted()', 84),
(26, 26, 0, 'uiExhausted()', 84),
(27, 27, 0, 'uiExhausted()', 84),
(28, 28, 0, 'uiExhausted()', 84),
(29, 29, 0, 'uiExhausted()', 84),
(30, 30, 0, 'uiExhausted()', 84),
(31, 31, 0, 'uiExhausted()', 84),
(32, 32, 0, 'uiExhausted()', 84),
(33, 33, 0, 'uiExhausted()', 84),
(34, 34, 0, 'uiExhausted()', 84),
(35, 35, 0, 'uiExhausted()', 84),
(36, 36, 0, 'uiExhausted()', 84),
(37, 37, 0, 'uiExhausted()', 84),
(38, 38, 0, 'uiExhausted()', 84),
(39, 39, 0, 'uiExhausted()', 84),
(40, 40, 0, 'uiExhausted()', 84),
(41, 41, 0, 'uiExhausted()', 84),
(42, 42, 0, 'uiExhausted()', 84),
(43, 43, 0, 'uiExhausted()', 84),
(44, 44, 0, 'uiExhausted()', 84),
(45, 45, 0, 'uiExhausted()', 84),
(46, 46, 0, 'uiExhausted()', 84),
(47, 47, 0, 'uiExhausted()', 84),
(48, 48, 0, 'uiExhausted()', 84),
(49, 49, 0, 'uiExhausted()', 84),
(50, 50, 0, 'uiExhausted()', 84),
(51, 51, 0, 'uiExhausted()', 84),
(52, 52, 0, 'uiExhausted()', 84),
(53, 53, 0, 'uiExhausted()', 84),
(54, 54, 0, 'uiExhausted()', 84),
(55, 55, 0, 'uiExhausted()', 84),
(56, 56, 0, 'uiExhausted()', 84),
(57, 57, 0, 'uiExhausted()', 84),
(58, 58, 0, 'uiExhausted()', 84),
(59, 59, 0, 'uiExhausted()', 84),
(60, 60, 0, 'uiExhausted()', 84),
(61, 61, 0, 'uiExhausted()', 84),
(62, 62, 0, 'uiExhausted()', 84),
(63, 63, 0, 'uiExhausted()', 84),
(64, 64, 0, 'uiExhausted()', 84),
(65, 65, 0, 'uiExhausted()', 84),
(66, 66, 0, 'uiExhausted()', 84),
(67, 67, 0, 'uiExhausted()', 84),
(68, 68, 0, 'uiExhausted()', 84),
(69, 69, 0, 'uiExhausted()', 84),
(70, 70, 0, 'uiExhausted()', 84),
(71, 71, 0, 'uiExhausted()', 84),
(72, 72, 0, 'uiExhausted()', 84),
(73, 73, 0, 'uiExhausted()', 84),
(74, 74, 0, 'uiExhausted()', 84),
(75, 75, 0, 'uiExhausted()', 84),
(76, 76, 0, 'uiExhausted()', 84),
(77, 77, 0, 'uiExhausted()', 84),
(78, 78, 0, 'uiExhausted()', 84),
(79, 79, 0, 'uiExhausted()', 84),
(80, 80, 0, 'uiExhausted()', 84),
(81, 81, 0, 'uiExhausted()', 84),
(82, 82, 0, 'uiExhausted()', 84),
(83, 83, 0, 'uiExhausted()', 84),
(84, 84, 0, 'uiExhausted()', 84),
(85, 85, 0, 'uiExhausted()', 84),
(86, 86, 0, 'uiExhausted()', 84),
(87, 87, 0, 'uiExhausted()', 84),
(88, 88, 0, 'uiExhausted()', 84),
(89, 89, 0, 'uiExhausted()', 84),
(90, 90, 0, 'uiExhausted()', 84),
(91, 91, 0, 'uiExhausted()', 84),
(92, 92, 0, 'uiExhausted()', 84),
(93, 93, 0, 'uiExhausted()', 84),
(94, 94, 0, 'uiExhausted()', 84),
(95, 95, 0, 'uiExhausted()', 84),
(96, 96, 0, 'uiExhausted()', 84),
(97, 97, 0, 'uiExhausted()', 84),
(98, 98, 0, 'uiExhausted()', 84),
(99, 99, 0, 'uiExhausted()', 84),
(100, 100, 0, 'uiExhausted()', 84),
(101, 101, 0, 'uiExhausted()', 84),
(102, 102, 0, 'uiExhausted()', 84),
(103, 103, 0, 'uiExhausted()', 84),
(104, 104, 0, 'uiExhausted()', 84),
(105, 105, 0, 'uiExhausted()', 84),
(106, 106, 0, 'uiExhausted()', 84),
(107, 107, 0, 'uiExhausted()', 84),
(108, 108, 0, 'uiExhausted()', 84),
(109, 109, 0, 'uiExhausted()', 84),
(110, 110, 0, 'uiExhausted()', 84),
(111, 111, 0, 'uiExhausted()', 84),
(112, 112, 0, 'uiExhausted()', 84),
(113, 113, 0, 'uiDone(uiU(6))', 80),
(114, 113, 1, 'uiDone(uiU(6))', 80),
]

specimens=[
 {'case_id':'F01factorial','filename':'uiFactorial3.ui','N':'1','OP':'uiMul()','BASE':'uiLiteral(uiU(1))','v0':'uiU(1)','v1':'uiU(1)','v2':'uiU(2)','title':'Full original factorial three fixture','math':'factorial(0)=1; 1*1=1; 2*1=2; 3*2=6.'},
 {'case_id':'F02sum','filename':'uiSum3.ui','N':'2','OP':'uiAdd()','BASE':'uiLiteral(uiU(0))','v0':'uiU(0)','v1':'uiU(1)','v2':'uiU(3)','title':'Full original sum three fixture','math':'sum(0)=0; 1+0=1; 2+1=3; 3+3=6.'},
]

def expand(text,symbols):
    # Pure text macro expansion: no source parsing, value computation, rule dispatch or state evolution.
    while '$' in text: text=Template(text).substitute(symbols)
    return text

normal=json.loads((W/'models/sherlock/oracles/normal-prefixes.json').read_text())
programs={'format':'sherlock-literal-normal-inputs/1','status':'provisional hand-authored source preparation; specified-only/unexecuted; no audit or adoption','profile':'sherlock-k-a/1','baseline_commit':BASE,'baseline_identity_source':'Root observed HEAD; no Git command executed by this owner.','source':SOURCE,'programs':[]}
pack={'format':'sherlock-literal-normal-prefixes/1','status':'provisional hand-derived source expectations; specified-only/unexecuted; no audit or adoption','profile':'sherlock-k-a/1','baseline_commit':BASE,'source':SOURCE,'representation':normal['representation'],'traces':[]}
original={'format':'sherlock-literal-original-fixture-runs/1','status':'provisional hand-derived source expectations; specified-only/unexecuted; no audit or adoption','profile':'sherlock-k-a/1','baseline_commit':BASE,'source':SOURCE,'representation':copy.deepcopy(normal['representation']),'traces':[]}
pack['representation']['initial_configuration_reconstruction']='For each ordinary budget row, replace only the uiRun fuel argument in initial_configuration.control_k with initial_fuel from the row; configured pre-start fuel, initialFuel, code and stackLimit remain their explicit defaults. Startup74 installs that row budget, fixed code and stack limit64.'
original['representation']['prefix_extent']='This file contains the two original fuel1000 runs. Their complete ordinary minimal-budget rows0..114 are in prefixes.json with identical case/input identities.'
original['representation']['original_fixture_run']='Full original fuel1000 and stackLimit64 retained. Each expanded state overlays fixed_state_fields. The initial_configuration control is the original fixture term with whitespace normalized; initial counters/stackLimit/code are their literal configured defaults. Startup74 installs the original code/fuel/limit. Publication80 preserves remaining fuel887 and all boundary fields.'
rule_ids={74:'k.admin.startup.valid',80:'k.admin.publish.value',84:'k.admin.publish.exhausted',102:'k.machine.eval.literal.valid',108:'k.machine.eval.variable.present',117:'k.machine.eval.binary',123:'k.machine.eval.if',131:'k.machine.eval.call.arguments',135:'k.machine.push.accept',149:'k.machine.return.left',153:'k.machine.return.right',161:'k.machine.return.if.true',165:'k.machine.return.if.false',181:'k.machine.return.arguments.final',185:'k.machine.return.call.valid',200:'k.machine.invoke.find',204:'k.machine.enter.bind',208:'k.machine.bound.valid',223:'k.machine.primitive.value'}
catalog=json.loads((W/'models/sherlock/reference/rules.json').read_text())
by_line={x['source']['start_line']:x['id'] for x in catalog['k_rules']}
assert all(by_line[line]==label for line,label in rule_ids.items())
pack['source_rule_labels']={str(k):v for k,v in rule_ids.items()}
original['source_rule_labels']=pack['source_rule_labels']
for specimen in specimens:
    s=dict(common);s.update(specimen)
    s.update({
      'CALL':'uiCall($N, uiExpr($SUB, uiNoExprs()))',
      'REC':'uiBinary($OP, $V, $CALL)',
      'BODY':'uiIf($Q, $BASE, $REC)',
      'FUN':'uiFun($N, uiParam(0, uiUIntT(), uiNoParams()), uiUIntT(), $BODY)',
      'CODE':'uiFunction($FUN, uiNoFuns())','ENTRY':'uiCall($N, uiExpr($THREE, uiNoExprs()))',
      'A3':'uiArgsF($N, uiNoValues(), uiNoExprs(), .Map)',
      'A2':'uiArgsF($N, uiNoValues(), uiNoExprs(), $e3)',
      'A1':'uiArgsF($N, uiNoValues(), uiNoExprs(), $e2)',
      'A0':'uiArgsF($N, uiNoValues(), uiNoExprs(), $e1)',
      'I3':'uiIfF($BASE, $REC, $e3)','I2':'uiIfF($BASE, $REC, $e2)',
      'I1':'uiIfF($BASE, $REC, $e1)','I0':'uiIfF($BASE, $REC, $e0)',
      'L3':'uiLeftF($OP, $CALL, $e3)','L2':'uiLeftF($OP, $CALL, $e2)','L1':'uiLeftF($OP, $CALL, $e1)',
      'P3':'uiRightF($OP, uiU(3))','P2':'uiRightF($OP, uiU(2))','P1':'uiRightF($OP, uiU(1))',
    })
    code=expand('$CODE',s);entry=expand('$ENTRY',s)
    fixture_path='models/sherlock/k/fixtures/'+s['filename']
    fixture_bytes=(W/fixture_path).read_bytes()
    original_term=f'uiRun({code}, {entry}, 1000, 64)'
    assert ''.join(original_term.split())==''.join(fixture_bytes.decode().split())
    record={'case_id':s['case_id'],'code_k':code,'entry_k':entry,'initial_arguments':'Explicit literals in entry_k; no external arguments.','stack_limit':64,'exact_completion_fuel':113}
    digest=hashlib.sha256(json.dumps(record,sort_keys=True,separators=(',',':')).encode()).hexdigest()
    record.update({'input_sha256':digest,'sha256_convention':'SHA256 of UTF-8 compact sorted-key JSON of the six preceding source fields, excluding digest/convention and metadata.','title':s['title'],'expected_value_k':'uiU(6)','derivation':s['math']+' Charged work 8+3*28+12+3*3=113; maximum depth9.','original_fixture':{'path':fixture_path,'sha256':hashlib.sha256(fixture_bytes).hexdigest(),'initial_fuel':1000,'stack_limit':64,'run_k':original_term,'coverage_status':'full literal trace independently specified; unexecuted'}})
    programs['programs'].append(record)
    fixed={'code_ref':{'file':'newprograms.json','case_id':s['case_id'],'input_sha256':digest},'initial_fuel':113,'stack_limit':64,'output_k':'uiPending()'}
    initial={'control_k':f'uiRun({code}, {entry}, 113, 64)','environment_k':'.Map','frames_top_first_k':[],'depth':0,'stack_limit':0,'code_k':'uiNoFuns()','remaining_fuel':0,'initial_fuel':0,'output_k':'uiPending()','charged_steps':0}
    states=[]
    for i,(line,control,env,stack) in enumerate(rows):
        states.append({'charged_steps':i,'remaining_fuel':113-i,'depth':len(stack),'control_k':expand(control,s),'environment_k':expand(s[env],s),'frames_top_first_k':[expand(s[x],s) for x in stack],'via_rule_start_line':line})
    assert max(x['depth'] for x in states)==9
    assert states[-1]['control_k']=='uiRet(uiU(6))' and states[-1]['environment_k']=='.Map' and not states[-1]['frames_top_first_k']
    assert all('$' not in json.dumps(x) for x in states)
    trace={'case_id':s['case_id'],'input_sha256':digest,'fixed_state_fields':fixed,'initial_configuration':initial,'startup':{'rule_start_line':74,'charge':0,'post_state_index':0},'states':states,'publication':{'pre_state_index':113,'rule_start_line':80,'charge':0,'post_overrides':{'control_k':'.K','output_k':'uiDone(uiU(6))'}},'ordinary_fuel_prefixes':[list(x) for x in prefixes]}
    pack['traces'].append(trace)
    full=copy.deepcopy(trace)
    full['fixed_state_fields']['initial_fuel']=1000
    full['initial_configuration']['control_k']=original_term
    for state in full['states']:state['remaining_fuel']=1000-state['charged_steps']
    del full['ordinary_fuel_prefixes']
    full['original_fixture']=record['original_fixture']
    full['expected_terminal_summary']={'charged_work':113,'remaining_fuel':887,'initial_fuel':1000,'stack_limit':64,'control_k':'.K','environment_k':'.Map','frames_top_first_k':[],'depth':0,'output_k':'uiDone(uiU(6))'}
    original['traces'].append(full)
for name,data in [('newprograms.json',programs),('prefixes.json',pack),('original-fixture-runs.json',original)]:
    (R/name).write_text(json.dumps(data,indent=2)+'\n')
print(json.dumps({'status':'literal serialization only; no subject executed','programs':len(programs['programs']),'states_per_trace':[len(t['states']) for t in pack['traces']],'prefixes_per_trace':[len(t['ordinary_fuel_prefixes']) for t in pack['traces']],'maximum_depth':9,'original_remaining_fuel':887}))
