#!/usr/bin/env python3
"""Serialize independently hand-listed K-A states; this is not an interpreter.

Only stdlib serialization/hash/file operations occur below. Source expressions,
states, rule locators, frame stacks, counters and budget rows are literal data.
String interpolation expands names for repeated immutable K text only. No
subject implementation is imported, evaluated, invoked or used for expectations.
"""
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
BASELINE = "30b65d925d1fc8210cb3521e7091aba787a69141"
SOURCE = {"path": "models/sherlock/k/main.k", "sha256": "b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41"}
EMPTY = ".Map"
WENV = "0 |-> uiU(1) 1 |-> uiU(2) 2 |-> uiU(3)"
TEN_B = "uiBinary(uiMul(), uiLiteral(uiU(10)), uiVar(1))"
HUNDRED_C = "uiBinary(uiMul(), uiLiteral(uiU(100)), uiVar(2))"
INNER = f"uiBinary(uiAdd(), uiVar(0), {TEN_B})"
WEIGHTED = f"uiBinary(uiAdd(), {INNER}, {HUNDRED_C})"
WFUN = f"uiFun(17, uiParam(0, uiUIntT(), uiParam(1, uiUIntT(), uiParam(2, uiUIntT(), uiNoParams()))), uiUIntT(), {WEIGHTED})"
WCODE = f"uiFunction({WFUN}, uiNoFuns())"
WENTRY = "uiCall(17, uiExpr(uiLiteral(uiU(1)), uiExpr(uiLiteral(uiU(2)), uiExpr(uiLiteral(uiU(3)), uiNoExprs()))))"
WVS = "uiValue(uiU(1), uiValue(uiU(2), uiValue(uiU(3), uiNoValues())))"
WA0 = "uiArgsF(17, uiNoValues(), uiExpr(uiLiteral(uiU(2)), uiExpr(uiLiteral(uiU(3)), uiNoExprs())), .Map)"
WA1 = "uiArgsF(17, uiValue(uiU(1), uiNoValues()), uiExpr(uiLiteral(uiU(3)), uiNoExprs()), .Map)"
WA2 = "uiArgsF(17, uiValue(uiU(1), uiValue(uiU(2), uiNoValues())), uiNoExprs(), .Map)"
WR = "uiReturnF(uiUIntT(), .Map)"
WLO = f"uiLeftF(uiAdd(), {HUNDRED_C}, {WENV})"
WLI = f"uiLeftF(uiAdd(), {TEN_B}, {WENV})"
WRI = "uiRightF(uiAdd(), uiU(1))"
WL10 = f"uiLeftF(uiMul(), uiVar(1), {WENV})"
WR10 = "uiRightF(uiMul(), uiU(10))"
WRO = "uiRightF(uiAdd(), uiU(21))"
WL100 = f"uiLeftF(uiMul(), uiVar(2), {WENV})"
WR100 = "uiRightF(uiMul(), uiU(100))"

# Columns: charged_steps, remaining_fuel, depth, incoming rule start,
# control, environment, complete top-first frame list. Every row is hand-listed.
WSTATES = [
    (0, 38, 0, None, f"uiEval({WENTRY})", EMPTY, []),
    (1, 37, 0, 131, f"uiPush({WA0}, uiEval(uiLiteral(uiU(1))), .Map)", EMPTY, []),
    (2, 36, 1, 135, "uiEval(uiLiteral(uiU(1)))", EMPTY, [WA0]),
    (3, 35, 1, 102, "uiRet(uiU(1))", EMPTY, [WA0]),
    (4, 34, 1, 177, "uiEval(uiLiteral(uiU(2)))", EMPTY, [WA1]),
    (5, 33, 1, 102, "uiRet(uiU(2))", EMPTY, [WA1]),
    (6, 32, 1, 177, "uiEval(uiLiteral(uiU(3)))", EMPTY, [WA2]),
    (7, 31, 1, 102, "uiRet(uiU(3))", EMPTY, [WA2]),
    (8, 30, 0, 181, f"uiInvoke(17, {WVS})", EMPTY, []),
    (9, 29, 0, 200, f"uiEnter({WFUN}, {WVS})", EMPTY, []),
    (10, 28, 0, 204, f"uiBound(uiBindings({WENV}), uiUIntT(), {WEIGHTED})", EMPTY, []),
    (11, 27, 0, 208, f"uiPush({WR}, uiEval({WEIGHTED}), {WENV})", EMPTY, []),
    (12, 26, 1, 135, f"uiEval({WEIGHTED})", WENV, [WR]),
    (13, 25, 1, 117, f"uiPush({WLO}, uiEval({INNER}), {WENV})", WENV, [WR]),
    (14, 24, 2, 135, f"uiEval({INNER})", WENV, [WLO, WR]),
    (15, 23, 2, 117, f"uiPush({WLI}, uiEval(uiVar(0)), {WENV})", WENV, [WLO, WR]),
    (16, 22, 3, 135, "uiEval(uiVar(0))", WENV, [WLI, WLO, WR]),
    (17, 21, 3, 108, "uiRet(uiU(1))", WENV, [WLI, WLO, WR]),
    (18, 20, 3, 149, f"uiEval({TEN_B})", WENV, [WRI, WLO, WR]),
    (19, 19, 3, 117, f"uiPush({WL10}, uiEval(uiLiteral(uiU(10))), {WENV})", WENV, [WRI, WLO, WR]),
    (20, 18, 4, 135, "uiEval(uiLiteral(uiU(10)))", WENV, [WL10, WRI, WLO, WR]),
    (21, 17, 4, 102, "uiRet(uiU(10))", WENV, [WL10, WRI, WLO, WR]),
    (22, 16, 4, 149, "uiEval(uiVar(1))", WENV, [WR10, WRI, WLO, WR]),
    (23, 15, 4, 108, "uiRet(uiU(2))", WENV, [WR10, WRI, WLO, WR]),
    (24, 14, 3, 153, "uiPrimResult(uiPrimValue(uiU(20)))", WENV, [WRI, WLO, WR]),
    (25, 13, 3, 223, "uiRet(uiU(20))", WENV, [WRI, WLO, WR]),
    (26, 12, 2, 153, "uiPrimResult(uiPrimValue(uiU(21)))", WENV, [WLO, WR]),
    (27, 11, 2, 223, "uiRet(uiU(21))", WENV, [WLO, WR]),
    (28, 10, 2, 149, f"uiEval({HUNDRED_C})", WENV, [WRO, WR]),
    (29, 9, 2, 117, f"uiPush({WL100}, uiEval(uiLiteral(uiU(100))), {WENV})", WENV, [WRO, WR]),
    (30, 8, 3, 135, "uiEval(uiLiteral(uiU(100)))", WENV, [WL100, WRO, WR]),
    (31, 7, 3, 102, "uiRet(uiU(100))", WENV, [WL100, WRO, WR]),
    (32, 6, 3, 149, "uiEval(uiVar(2))", WENV, [WR100, WRO, WR]),
    (33, 5, 3, 108, "uiRet(uiU(3))", WENV, [WR100, WRO, WR]),
    (34, 4, 2, 153, "uiPrimResult(uiPrimValue(uiU(300)))", WENV, [WRO, WR]),
    (35, 3, 2, 223, "uiRet(uiU(300))", WENV, [WRO, WR]),
    (36, 2, 1, 153, "uiPrimResult(uiPrimValue(uiU(321)))", WENV, [WR]),
    (37, 1, 1, 223, "uiRet(uiU(321))", WENV, [WR]),
    (38, 0, 0, 185, "uiRet(uiU(321))", EMPTY, []),
]
WPREFIXES = [
    [0, 0, 0, "uiExhausted()", 84],
    [1, 1, 0, "uiExhausted()", 84],
    [2, 2, 0, "uiExhausted()", 84],
    [3, 3, 0, "uiExhausted()", 84],
    [4, 4, 0, "uiExhausted()", 84],
    [5, 5, 0, "uiExhausted()", 84],
    [6, 6, 0, "uiExhausted()", 84],
    [7, 7, 0, "uiExhausted()", 84],
    [8, 8, 0, "uiExhausted()", 84],
    [9, 9, 0, "uiExhausted()", 84],
    [10, 10, 0, "uiExhausted()", 84],
    [11, 11, 0, "uiExhausted()", 84],
    [12, 12, 0, "uiExhausted()", 84],
    [13, 13, 0, "uiExhausted()", 84],
    [14, 14, 0, "uiExhausted()", 84],
    [15, 15, 0, "uiExhausted()", 84],
    [16, 16, 0, "uiExhausted()", 84],
    [17, 17, 0, "uiExhausted()", 84],
    [18, 18, 0, "uiExhausted()", 84],
    [19, 19, 0, "uiExhausted()", 84],
    [20, 20, 0, "uiExhausted()", 84],
    [21, 21, 0, "uiExhausted()", 84],
    [22, 22, 0, "uiExhausted()", 84],
    [23, 23, 0, "uiExhausted()", 84],
    [24, 24, 0, "uiExhausted()", 84],
    [25, 25, 0, "uiExhausted()", 84],
    [26, 26, 0, "uiExhausted()", 84],
    [27, 27, 0, "uiExhausted()", 84],
    [28, 28, 0, "uiExhausted()", 84],
    [29, 29, 0, "uiExhausted()", 84],
    [30, 30, 0, "uiExhausted()", 84],
    [31, 31, 0, "uiExhausted()", 84],
    [32, 32, 0, "uiExhausted()", 84],
    [33, 33, 0, "uiExhausted()", 84],
    [34, 34, 0, "uiExhausted()", 84],
    [35, 35, 0, "uiExhausted()", 84],
    [36, 36, 0, "uiExhausted()", 84],
    [37, 37, 0, "uiExhausted()", 84],
    [38, 38, 0, "uiDone(uiU(321))", 80],
    [39, 38, 1, "uiDone(uiU(321))", 80],
]

F2 = "0 |-> uiU(2)"
F1 = "0 |-> uiU(1)"
F0 = "0 |-> uiU(0)"
COND = "uiBinary(uiEq(), uiVar(0), uiLiteral(uiU(0)))"
DECREMENT = "uiBinary(uiSub(), uiVar(0), uiLiteral(uiU(1)))"
RECUR = f"uiCall(18, uiExpr({DECREMENT}, uiNoExprs()))"
MULTIPLY = f"uiBinary(uiMul(), uiVar(0), {RECUR})"
FACT = f"uiIf({COND}, uiLiteral(uiU(1)), {MULTIPLY})"
FFUN = f"uiFun(18, uiParam(0, uiUIntT(), uiNoParams()), uiUIntT(), {FACT})"
FCODE = f"uiFunction({FFUN}, uiNoFuns())"
FENTRY = "uiCall(18, uiExpr(uiLiteral(uiU(2)), uiNoExprs()))"
V2 = "uiValue(uiU(2), uiNoValues())"
V1 = "uiValue(uiU(1), uiNoValues())"
V0 = "uiValue(uiU(0), uiNoValues())"
FAE = "uiArgsF(18, uiNoValues(), uiNoExprs(), .Map)"
FA2 = f"uiArgsF(18, uiNoValues(), uiNoExprs(), {F2})"
FA1 = f"uiArgsF(18, uiNoValues(), uiNoExprs(), {F1})"
FRE = "uiReturnF(uiUIntT(), .Map)"
FR2 = f"uiReturnF(uiUIntT(), {F2})"
FR1 = f"uiReturnF(uiUIntT(), {F1})"
FI2 = f"uiIfF(uiLiteral(uiU(1)), {MULTIPLY}, {F2})"
FI1 = f"uiIfF(uiLiteral(uiU(1)), {MULTIPLY}, {F1})"
FI0 = f"uiIfF(uiLiteral(uiU(1)), {MULTIPLY}, {F0})"
FE2 = f"uiLeftF(uiEq(), uiLiteral(uiU(0)), {F2})"
FE1 = f"uiLeftF(uiEq(), uiLiteral(uiU(0)), {F1})"
FE0 = f"uiLeftF(uiEq(), uiLiteral(uiU(0)), {F0})"
FER2 = "uiRightF(uiEq(), uiU(2))"
FER1 = "uiRightF(uiEq(), uiU(1))"
FER0 = "uiRightF(uiEq(), uiU(0))"
FM2 = f"uiLeftF(uiMul(), {RECUR}, {F2})"
FM1 = f"uiLeftF(uiMul(), {RECUR}, {F1})"
FMR2 = "uiRightF(uiMul(), uiU(2))"
FMR1 = "uiRightF(uiMul(), uiU(1))"
FS2 = f"uiLeftF(uiSub(), uiLiteral(uiU(1)), {F2})"
FS1 = f"uiLeftF(uiSub(), uiLiteral(uiU(1)), {F1})"
FSR2 = "uiRightF(uiSub(), uiU(2))"
FSR1 = "uiRightF(uiSub(), uiU(1))"

FSTATES = [
    (0, 82, 0, None, f"uiEval({FENTRY})", EMPTY, []),
    (1, 81, 0, 131, f"uiPush({FAE}, uiEval(uiLiteral(uiU(2))), .Map)", EMPTY, []),
    (2, 80, 1, 135, "uiEval(uiLiteral(uiU(2)))", EMPTY, [FAE]),
    (3, 79, 1, 102, "uiRet(uiU(2))", EMPTY, [FAE]),
    (4, 78, 0, 181, f"uiInvoke(18, {V2})", EMPTY, []),
    (5, 77, 0, 200, f"uiEnter({FFUN}, {V2})", EMPTY, []),
    (6, 76, 0, 204, f"uiBound(uiBindings({F2}), uiUIntT(), {FACT})", EMPTY, []),
    (7, 75, 0, 208, f"uiPush({FRE}, uiEval({FACT}), {F2})", EMPTY, []),
    (8, 74, 1, 135, f"uiEval({FACT})", F2, [FRE]),
    (9, 73, 1, 123, f"uiPush({FI2}, uiEval({COND}), {F2})", F2, [FRE]),
    (10, 72, 2, 135, f"uiEval({COND})", F2, [FI2, FRE]),
    (11, 71, 2, 117, f"uiPush({FE2}, uiEval(uiVar(0)), {F2})", F2, [FI2, FRE]),
    (12, 70, 3, 135, "uiEval(uiVar(0))", F2, [FE2, FI2, FRE]),
    (13, 69, 3, 108, "uiRet(uiU(2))", F2, [FE2, FI2, FRE]),
    (14, 68, 3, 149, "uiEval(uiLiteral(uiU(0)))", F2, [FER2, FI2, FRE]),
    (15, 67, 3, 102, "uiRet(uiU(0))", F2, [FER2, FI2, FRE]),
    (16, 66, 2, 153, "uiPrimResult(uiPrimValue(uiB(false)))", F2, [FI2, FRE]),
    (17, 65, 2, 223, "uiRet(uiB(false))", F2, [FI2, FRE]),
    (18, 64, 1, 165, f"uiEval({MULTIPLY})", F2, [FRE]),
    (19, 63, 1, 117, f"uiPush({FM2}, uiEval(uiVar(0)), {F2})", F2, [FRE]),
    (20, 62, 2, 135, "uiEval(uiVar(0))", F2, [FM2, FRE]),
    (21, 61, 2, 108, "uiRet(uiU(2))", F2, [FM2, FRE]),
    (22, 60, 2, 149, f"uiEval({RECUR})", F2, [FMR2, FRE]),
    (23, 59, 2, 131, f"uiPush({FA2}, uiEval({DECREMENT}), {F2})", F2, [FMR2, FRE]),
    (24, 58, 3, 135, f"uiEval({DECREMENT})", F2, [FA2, FMR2, FRE]),
    (25, 57, 3, 117, f"uiPush({FS2}, uiEval(uiVar(0)), {F2})", F2, [FA2, FMR2, FRE]),
    (26, 56, 4, 135, "uiEval(uiVar(0))", F2, [FS2, FA2, FMR2, FRE]),
    (27, 55, 4, 108, "uiRet(uiU(2))", F2, [FS2, FA2, FMR2, FRE]),
    (28, 54, 4, 149, "uiEval(uiLiteral(uiU(1)))", F2, [FSR2, FA2, FMR2, FRE]),
    (29, 53, 4, 102, "uiRet(uiU(1))", F2, [FSR2, FA2, FMR2, FRE]),
    (30, 52, 3, 153, "uiPrimResult(uiPrimValue(uiU(1)))", F2, [FA2, FMR2, FRE]),
    (31, 51, 3, 223, "uiRet(uiU(1))", F2, [FA2, FMR2, FRE]),
    (32, 50, 2, 181, f"uiInvoke(18, {V1})", F2, [FMR2, FRE]),
    (33, 49, 2, 200, f"uiEnter({FFUN}, {V1})", F2, [FMR2, FRE]),
    (34, 48, 2, 204, f"uiBound(uiBindings({F1}), uiUIntT(), {FACT})", F2, [FMR2, FRE]),
    (35, 47, 2, 208, f"uiPush({FR2}, uiEval({FACT}), {F1})", F2, [FMR2, FRE]),
    (36, 46, 3, 135, f"uiEval({FACT})", F1, [FR2, FMR2, FRE]),
    (37, 45, 3, 123, f"uiPush({FI1}, uiEval({COND}), {F1})", F1, [FR2, FMR2, FRE]),
    (38, 44, 4, 135, f"uiEval({COND})", F1, [FI1, FR2, FMR2, FRE]),
    (39, 43, 4, 117, f"uiPush({FE1}, uiEval(uiVar(0)), {F1})", F1, [FI1, FR2, FMR2, FRE]),
    (40, 42, 5, 135, "uiEval(uiVar(0))", F1, [FE1, FI1, FR2, FMR2, FRE]),
    (41, 41, 5, 108, "uiRet(uiU(1))", F1, [FE1, FI1, FR2, FMR2, FRE]),
    (42, 40, 5, 149, "uiEval(uiLiteral(uiU(0)))", F1, [FER1, FI1, FR2, FMR2, FRE]),
    (43, 39, 5, 102, "uiRet(uiU(0))", F1, [FER1, FI1, FR2, FMR2, FRE]),
    (44, 38, 4, 153, "uiPrimResult(uiPrimValue(uiB(false)))", F1, [FI1, FR2, FMR2, FRE]),
    (45, 37, 4, 223, "uiRet(uiB(false))", F1, [FI1, FR2, FMR2, FRE]),
    (46, 36, 3, 165, f"uiEval({MULTIPLY})", F1, [FR2, FMR2, FRE]),
    (47, 35, 3, 117, f"uiPush({FM1}, uiEval(uiVar(0)), {F1})", F1, [FR2, FMR2, FRE]),
    (48, 34, 4, 135, "uiEval(uiVar(0))", F1, [FM1, FR2, FMR2, FRE]),
    (49, 33, 4, 108, "uiRet(uiU(1))", F1, [FM1, FR2, FMR2, FRE]),
    (50, 32, 4, 149, f"uiEval({RECUR})", F1, [FMR1, FR2, FMR2, FRE]),
    (51, 31, 4, 131, f"uiPush({FA1}, uiEval({DECREMENT}), {F1})", F1, [FMR1, FR2, FMR2, FRE]),
    (52, 30, 5, 135, f"uiEval({DECREMENT})", F1, [FA1, FMR1, FR2, FMR2, FRE]),
    (53, 29, 5, 117, f"uiPush({FS1}, uiEval(uiVar(0)), {F1})", F1, [FA1, FMR1, FR2, FMR2, FRE]),
    (54, 28, 6, 135, "uiEval(uiVar(0))", F1, [FS1, FA1, FMR1, FR2, FMR2, FRE]),
    (55, 27, 6, 108, "uiRet(uiU(1))", F1, [FS1, FA1, FMR1, FR2, FMR2, FRE]),
    (56, 26, 6, 149, "uiEval(uiLiteral(uiU(1)))", F1, [FSR1, FA1, FMR1, FR2, FMR2, FRE]),
    (57, 25, 6, 102, "uiRet(uiU(1))", F1, [FSR1, FA1, FMR1, FR2, FMR2, FRE]),
    (58, 24, 5, 153, "uiPrimResult(uiPrimValue(uiU(0)))", F1, [FA1, FMR1, FR2, FMR2, FRE]),
    (59, 23, 5, 223, "uiRet(uiU(0))", F1, [FA1, FMR1, FR2, FMR2, FRE]),
    (60, 22, 4, 181, f"uiInvoke(18, {V0})", F1, [FMR1, FR2, FMR2, FRE]),
    (61, 21, 4, 200, f"uiEnter({FFUN}, {V0})", F1, [FMR1, FR2, FMR2, FRE]),
    (62, 20, 4, 204, f"uiBound(uiBindings({F0}), uiUIntT(), {FACT})", F1, [FMR1, FR2, FMR2, FRE]),
    (63, 19, 4, 208, f"uiPush({FR1}, uiEval({FACT}), {F0})", F1, [FMR1, FR2, FMR2, FRE]),
    (64, 18, 5, 135, f"uiEval({FACT})", F0, [FR1, FMR1, FR2, FMR2, FRE]),
    (65, 17, 5, 123, f"uiPush({FI0}, uiEval({COND}), {F0})", F0, [FR1, FMR1, FR2, FMR2, FRE]),
    (66, 16, 6, 135, f"uiEval({COND})", F0, [FI0, FR1, FMR1, FR2, FMR2, FRE]),
    (67, 15, 6, 117, f"uiPush({FE0}, uiEval(uiVar(0)), {F0})", F0, [FI0, FR1, FMR1, FR2, FMR2, FRE]),
    (68, 14, 7, 135, "uiEval(uiVar(0))", F0, [FE0, FI0, FR1, FMR1, FR2, FMR2, FRE]),
    (69, 13, 7, 108, "uiRet(uiU(0))", F0, [FE0, FI0, FR1, FMR1, FR2, FMR2, FRE]),
    (70, 12, 7, 149, "uiEval(uiLiteral(uiU(0)))", F0, [FER0, FI0, FR1, FMR1, FR2, FMR2, FRE]),
    (71, 11, 7, 102, "uiRet(uiU(0))", F0, [FER0, FI0, FR1, FMR1, FR2, FMR2, FRE]),
    (72, 10, 6, 153, "uiPrimResult(uiPrimValue(uiB(true)))", F0, [FI0, FR1, FMR1, FR2, FMR2, FRE]),
    (73, 9, 6, 223, "uiRet(uiB(true))", F0, [FI0, FR1, FMR1, FR2, FMR2, FRE]),
    (74, 8, 5, 161, "uiEval(uiLiteral(uiU(1)))", F0, [FR1, FMR1, FR2, FMR2, FRE]),
    (75, 7, 5, 102, "uiRet(uiU(1))", F0, [FR1, FMR1, FR2, FMR2, FRE]),
    (76, 6, 4, 185, "uiRet(uiU(1))", F1, [FMR1, FR2, FMR2, FRE]),
    (77, 5, 3, 153, "uiPrimResult(uiPrimValue(uiU(1)))", F1, [FR2, FMR2, FRE]),
    (78, 4, 3, 223, "uiRet(uiU(1))", F1, [FR2, FMR2, FRE]),
    (79, 3, 2, 185, "uiRet(uiU(1))", F2, [FMR2, FRE]),
    (80, 2, 1, 153, "uiPrimResult(uiPrimValue(uiU(2)))", F2, [FRE]),
    (81, 1, 1, 223, "uiRet(uiU(2))", F2, [FRE]),
    (82, 0, 0, 185, "uiRet(uiU(2))", EMPTY, []),
]
FPREFIXES = [
    [0, 0, 0, "uiExhausted()", 84],
    [1, 1, 0, "uiExhausted()", 84],
    [2, 2, 0, "uiExhausted()", 84],
    [3, 3, 0, "uiExhausted()", 84],
    [4, 4, 0, "uiExhausted()", 84],
    [5, 5, 0, "uiExhausted()", 84],
    [6, 6, 0, "uiExhausted()", 84],
    [7, 7, 0, "uiExhausted()", 84],
    [8, 8, 0, "uiExhausted()", 84],
    [9, 9, 0, "uiExhausted()", 84],
    [10, 10, 0, "uiExhausted()", 84],
    [11, 11, 0, "uiExhausted()", 84],
    [12, 12, 0, "uiExhausted()", 84],
    [13, 13, 0, "uiExhausted()", 84],
    [14, 14, 0, "uiExhausted()", 84],
    [15, 15, 0, "uiExhausted()", 84],
    [16, 16, 0, "uiExhausted()", 84],
    [17, 17, 0, "uiExhausted()", 84],
    [18, 18, 0, "uiExhausted()", 84],
    [19, 19, 0, "uiExhausted()", 84],
    [20, 20, 0, "uiExhausted()", 84],
    [21, 21, 0, "uiExhausted()", 84],
    [22, 22, 0, "uiExhausted()", 84],
    [23, 23, 0, "uiExhausted()", 84],
    [24, 24, 0, "uiExhausted()", 84],
    [25, 25, 0, "uiExhausted()", 84],
    [26, 26, 0, "uiExhausted()", 84],
    [27, 27, 0, "uiExhausted()", 84],
    [28, 28, 0, "uiExhausted()", 84],
    [29, 29, 0, "uiExhausted()", 84],
    [30, 30, 0, "uiExhausted()", 84],
    [31, 31, 0, "uiExhausted()", 84],
    [32, 32, 0, "uiExhausted()", 84],
    [33, 33, 0, "uiExhausted()", 84],
    [34, 34, 0, "uiExhausted()", 84],
    [35, 35, 0, "uiExhausted()", 84],
    [36, 36, 0, "uiExhausted()", 84],
    [37, 37, 0, "uiExhausted()", 84],
    [38, 38, 0, "uiExhausted()", 84],
    [39, 39, 0, "uiExhausted()", 84],
    [40, 40, 0, "uiExhausted()", 84],
    [41, 41, 0, "uiExhausted()", 84],
    [42, 42, 0, "uiExhausted()", 84],
    [43, 43, 0, "uiExhausted()", 84],
    [44, 44, 0, "uiExhausted()", 84],
    [45, 45, 0, "uiExhausted()", 84],
    [46, 46, 0, "uiExhausted()", 84],
    [47, 47, 0, "uiExhausted()", 84],
    [48, 48, 0, "uiExhausted()", 84],
    [49, 49, 0, "uiExhausted()", 84],
    [50, 50, 0, "uiExhausted()", 84],
    [51, 51, 0, "uiExhausted()", 84],
    [52, 52, 0, "uiExhausted()", 84],
    [53, 53, 0, "uiExhausted()", 84],
    [54, 54, 0, "uiExhausted()", 84],
    [55, 55, 0, "uiExhausted()", 84],
    [56, 56, 0, "uiExhausted()", 84],
    [57, 57, 0, "uiExhausted()", 84],
    [58, 58, 0, "uiExhausted()", 84],
    [59, 59, 0, "uiExhausted()", 84],
    [60, 60, 0, "uiExhausted()", 84],
    [61, 61, 0, "uiExhausted()", 84],
    [62, 62, 0, "uiExhausted()", 84],
    [63, 63, 0, "uiExhausted()", 84],
    [64, 64, 0, "uiExhausted()", 84],
    [65, 65, 0, "uiExhausted()", 84],
    [66, 66, 0, "uiExhausted()", 84],
    [67, 67, 0, "uiExhausted()", 84],
    [68, 68, 0, "uiExhausted()", 84],
    [69, 69, 0, "uiExhausted()", 84],
    [70, 70, 0, "uiExhausted()", 84],
    [71, 71, 0, "uiExhausted()", 84],
    [72, 72, 0, "uiExhausted()", 84],
    [73, 73, 0, "uiExhausted()", 84],
    [74, 74, 0, "uiExhausted()", 84],
    [75, 75, 0, "uiExhausted()", 84],
    [76, 76, 0, "uiExhausted()", 84],
    [77, 77, 0, "uiExhausted()", 84],
    [78, 78, 0, "uiExhausted()", 84],
    [79, 79, 0, "uiExhausted()", 84],
    [80, 80, 0, "uiExhausted()", 84],
    [81, 81, 0, "uiExhausted()", 84],
    [82, 82, 0, "uiDone(uiU(2))", 80],
    [83, 82, 1, "uiDone(uiU(2))", 80],
]

PROGRAMS = [
    {
        "case_id": "N07", "code_k": WCODE, "entry_k": WENTRY,
        "initial_arguments": "Explicit literals in entry_k; no external arguments.",
        "stack_limit": 8, "exact_completion_fuel": 38,
        "title": "Three source-ordered arguments with asymmetric decimal weights",
        "expected_value_k": "uiU(321)",
        "derivation": "Call 17 binds a=1, b=2, c=3 after source-ordered argument evaluation. The body (a + 10*b) + 100*c yields 321. Helpers append/find/bind and checked arithmetic are unmetered in A. 12 charges reach the body, 25 evaluate it, one restores the caller. See EXTRA-NORMAL-DERIVATION.md.",
    },
    {
        "case_id": "N08", "code_k": FCODE, "entry_k": FENTRY,
        "initial_arguments": "Explicit literals in entry_k; no external arguments.",
        "stack_limit": 8, "exact_completion_fuel": 82,
        "title": "Named self-recursive factorial two with suspended multiplication",
        "expected_value_k": "uiU(2)",
        "derivation": "Function 18 recursively calls the same source-table function on n-1 until n=0; no source unrolling. Equality selects false, false, true. Calls retain caller-return and pending multiplication frames. Base return restores n=1 at76, next return n=2 at79, outer return empty at82. 8+28+28+12+3+3=82 charges. See EXTRA-NORMAL-DERIVATION.md.",
    },
]

SOURCE_FIELDS = ("case_id", "code_k", "entry_k", "initial_arguments", "stack_limit", "exact_completion_fuel")
for program in PROGRAMS:
    source_fields = {key: program[key] for key in SOURCE_FIELDS}
    program["input_sha256"] = hashlib.sha256(json.dumps(source_fields, sort_keys=True, separators=(",", ":")).encode("utf-8")).hexdigest()
    program["sha256_convention"] = "SHA256 of UTF-8 compact sorted-key JSON of the six source fields case_id/code_k/entry_k/initial_arguments/stack_limit/exact_completion_fuel, excluding digest/convention and metadata."

INPUTS = {
    "format": "sherlock-literal-normal-inputs/1",
    "status": "hand-authored source preparation; specified-only/unexecuted",
    "profile": "sherlock-k-a/1", "baseline_commit": BASELINE, "source": SOURCE,
    "programs": PROGRAMS,
    "scope": {
        "purpose": "Add two normal complete-state/prefix expectations beyond unchanged N01-N06; independent audit and consumer wiring remain separate.",
        "coverage_status": "Two new A programs only; all frozen-fixture, D01-D36, whole-grammar, Candidate B, native and target obligations remain as recorded in normal-programs.json.",
        "admission": "Ordinary valid finite examples. No new closed-body or unique-function-table admission condition; K's source-ordered first-matching-function lookup is unchanged.",
        "future_b": {"profile": "sherlock-helper-microstep/1", "status": "not derived or frozen here", "exact_fuel": None, "exact_helper_work": None},
    },
}

REPRESENTATION = {
    "state_reconstruction": "Each state overlays fixed_state_fields. code_ref resolves exactly one immutable source record by case_id/input_sha256; no other state field is omitted. Frame array is top-first; fold right using uiFrame(frame,rest), ending uiNoFrames(). Environment_k is a K Map; whitespace juxtaposes distinct entries. Map order has no semantic meaning. Pure function subterms are normalized within their charged rule; this does not create another A fuel event.",
    "edges": "State i is the full pre-state for the next edge and full post-state of the preceding edge. via_rule_start_line identifies the rule taking i-1 to i in pinned main.k; startup/publication are separate zero-charge edges.",
    "ordinary_prefix_columns": ["initial_fuel", "boundary_state_index", "remaining_fuel", "published_output_k", "publication_rule_start_line"],
    "ordinary_prefix_reconstruction": "For each five-element literal prefix row, boundary state fields come from states[boundary_state_index], replacing initial_fuel and remaining_fuel by the literal row values; consumed_work equals boundary_state_index. The complete charged sequence is states[0..boundary_state_index] with initial_fuel from the row and remaining_fuel=initial_fuel-charged_steps. Code/env/frames/depth/control/output are otherwise unchanged. Before startup the uiRun fuel argument is replaced by this row's initial_fuel; configured initial_fuel and remaining_fuel are both zero. Startup sets both counters to the row's initial_fuel, loads code/stack_limit, and preserves empty env/frames, zero depth and pending output. Publication replaces control_k by .K and output_k by published_output_k, preserving every other boundary field. These are ordinary finite-budget prefixes of valid programs. No attempted-work diagnostic is defined by K A.",
    "administrative_charge": 0, "machine_rule_charge": 1,
    "helper_charge": "unmetered in A; helper visits/work not measured and never filled with zero",
    "prefix_extent": "Every integer initial fuel from zero through exact completion fuel plus one is explicitly listed for every case. For any larger budget the same terminal state is reached after the listed exact completion count, with the surplus remaining; no further charges occur.",
}
TRACES = []
for program, rows, prefixes in [(PROGRAMS[0], WSTATES, WPREFIXES), (PROGRAMS[1], FSTATES, FPREFIXES)]:
    case_id = program["case_id"]
    digest = program["input_sha256"]
    completion = program["exact_completion_fuel"]
    states = [dict(zip(("charged_steps", "remaining_fuel", "depth", "via_rule_start_line", "control_k", "environment_k", "frames_top_first_k"), row)) for row in rows]
    TRACES.append({
        "case_id": case_id, "input_sha256": digest,
        "fixed_state_fields": {"code_ref": {"file": "extra-normal-programs.json", "case_id": case_id, "input_sha256": digest}, "initial_fuel": completion, "stack_limit": 8, "output_k": "uiPending()"},
        "initial_configuration": {"control_k": f"uiRun({program['code_k']}, {program['entry_k']}, {completion}, 8)", "environment_k": ".Map", "frames_top_first_k": [], "depth": 0, "stack_limit": 0, "code_k": "uiNoFuns()", "remaining_fuel": 0, "initial_fuel": 0, "output_k": "uiPending()", "charged_steps": 0},
        "startup": {"rule_start_line": 74, "charge": 0, "post_state_index": 0},
        "states": states,
        "publication": {"pre_state_index": completion, "rule_start_line": 80, "charge": 0, "post_overrides": {"control_k": ".K", "output_k": f"uiDone({program['expected_value_k']})"}},
        "ordinary_fuel_prefixes": prefixes,
    })
PREFIXES = {"format": "sherlock-literal-normal-prefixes/1", "status": "hand-derived source expectations; specified-only/unexecuted", "profile": "sherlock-k-a/1", "baseline_commit": BASELINE, "source": SOURCE, "representation": REPRESENTATION, "traces": TRACES}

if __name__ == "__main__":
    for name, packet in [("extra-normal-programs.json", INPUTS), ("extra-normal-prefixes.json", PREFIXES)]:
        (HERE / name).write_text(json.dumps(packet, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
