"""Reduce the accepted gross-cap violation without modifying Aeon."""

import json

from aeon.facade.driver import AeonConfig, AeonDriver
from aeon.synthesis.uis.api import SilentSynthesisUI


CASES = {
    "literal_cap": "def x : {v:Int | v <= 100} := 101;",
    "named_cap": "def cap : {c:Int | c = 100} := 100; def x : {v:Int | v <= cap} := 101;",
    "named_fee": "def cap : {c:Int | c = 5} := 5; def x : {v:Int | v <= cap} := 6;",
    "two_conjuncts_literal": "def x : {v:Int | v = 101 && v <= 100} := 101;",
    "two_conjuncts_reversed": "def x : {v:Int | v <= 100 && v = 101} := 101;",
    "equality_then_false": "def x : {v:Int | v = 101 && false} := 101;",
    "true_then_false": "def x : {v:Int | true && false} := 101;",
    "false_only": "def x : {v:Int | false} := 101;",
    "false_only_pay": "def pay : {v:Int | false} := 101;",
    "literal_cap_pay": "def pay : {v:Int | v <= 100} := 101;",
    "false_only_long": "def planGross : {v:Int | false} := 101;",
    "false_then_true": "def x : {v:Int | false && true} := 101;",
    "parenthesized_conjuncts": "def x : {v:Int | (v = 101) && (v <= 100)} := 101;",
    "inequality_then_false": "def x : {v:Int | v >= 0 && false} := 101;",
    "two_conjuncts_named": "def cap : {c:Int | c = 100} := 100; def x : {v:Int | v = 101 && v <= cap} := 101;",
    "derived_named": "def cap : {c:Int | c = 100} := 100; def a : {x:Int | x >= 0} := 97; def b : {x:Int | x >= 0} := 4; def g : {x:Int | x = a + b && x <= cap} := 101;",
}

rows = []
for name, source in CASES.items():
    d = AeonDriver(AeonConfig(synthesizer="smt", synthesis_ui=SilentSynthesisUI(), synthesis_budget=0, no_main=True, strict_decidable=True))
    errors = list(d.parse(aeon_code=source))
    rows.append({"case": name, "source": source, "accepted": not errors, "errors": [str(e)[:400] for e in errors]})
print(json.dumps(rows, indent=2))
