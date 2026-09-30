"""Aeon advisory refinement probe for an encoded signing request.

These small integer tags stand for canonical identifiers/digests. The experiment
checks only the type-level relationship, not any cryptographic implementation.
"""

import json
from pathlib import Path

from aeon.facade.driver import AeonConfig, AeonDriver
from aeon.facade.trust import compute_trust_report
from aeon.synthesis.uis.api import SilentSynthesisUI


OUT = Path(__file__).resolve().parent

SIGNER = """
def authorize
  (domain:{d:Int | d = 1})
  (path:{p:Int | p = 7})
  (digest:{h:Int | h = 314159})
  (recipient:{r:Int | r = 42})
  (value:{v:Int | v > 0 && v <= 100})
  (epoch:{e:Int | e = 5})
  (nonce:{n:Int | n = 9})
  : {s:Int | s = 1} := 1;
"""

VALID_ARGS = [1, 7, 314159, 42, 100, 5, 9]
CASES = {"valid_request": (VALID_ARGS, True)}
for label, index, replacement in [
    ("wrong_domain", 0, 2),
    ("wrong_path", 1, 8),
    ("wrong_payload_digest", 2, 314160),
    ("wrong_recipient", 3, 43),
    ("value_above_cap", 4, 101),
    ("zero_value", 4, 0),
    ("wrong_key_epoch", 5, 6),
    ("wrong_nonce", 6, 10),
]:
    args = VALID_ARGS.copy()
    args[index] = replacement
    CASES[label] = (args, False)


def check(source):
    driver = AeonDriver(
        AeonConfig(
            synthesizer="smt",
            synthesis_ui=SilentSynthesisUI(),
            synthesis_budget=0,
            no_main=True,
            strict_decidable=True,
        )
    )
    errors = list(driver.parse(aeon_code=source))
    trust = []
    if not errors:
        trust = [
            {"name": item.display, "kind": item.kind}
            for item in compute_trust_report(driver.core).items
        ]
    return {
        "accepted": not errors,
        "errors": [type(error).__name__ for error in errors],
        "trust": trust,
    }


results = []
for name, (args, expected) in CASES.items():
    source = SIGNER + f"\ndef attempt : {{s:Int | s = 1}} := authorize {' '.join(map(str, args))};\n"
    (OUT / f"{name}.ae").write_text(source)
    actual = check(source)
    results.append({"case": name, "expected_acceptance": expected, **actual})

DYNAMIC_CASES = {
    "unbounded_dynamic_value": (
        SIGNER
        + "def attempt (raw:Int) : {s:Int | s = 1} := authorize 1 7 314159 42 raw 5 9;\n",
        False,
    ),
    "bounded_dynamic_value": (
        SIGNER
        + "def attempt (value:{v:Int | v > 0 && v <= 100}) : {s:Int | s = 1} := authorize 1 7 314159 42 value 5 9;\n",
        True,
    ),
}
for name, (source, expected) in DYNAMIC_CASES.items():
    (OUT / f"{name}.ae").write_text(source)
    actual = check(source)
    results.append({"case": name, "expected_acceptance": expected, **actual})

PHASE_CASES = {
    "signed_not_broadcast": (
        "def signed : {p:Int | p = 1} := 1;\n"
        "def require_broadcast (receipt:{p:Int | p = 2}) : Int := 1;\n"
        "def result : Int := require_broadcast signed;\n",
        False,
    ),
    "signed_not_finalized": (
        "def signed : {p:Int | p = 1} := 1;\n"
        "def require_finalized (proof:{p:Int | p = 3}) : Int := 1;\n"
        "def result : Int := require_finalized signed;\n",
        False,
    ),
    "finalized_evidence_token": (
        "def observed : {p:Int | p = 3} := 3;\n"
        "def require_finalized (proof:{p:Int | p = 3}) : Int := 1;\n"
        "def result : Int := require_finalized observed;\n",
        True,
    ),
    "broadcast_not_finalized": (
        "def broadcast : {p:Int | p = 2} := 2;\n"
        "def require_finalized (proof:{p:Int | p = 3}) : Int := 1;\n"
        "def result : Int := require_finalized broadcast;\n",
        False,
    ),
    "untrusted_native_finality_promise": (
        'def observed : {p:Int | p = 3} := native "1";\n'
        "def require_finalized (proof:{p:Int | p = 3}) : Int := 1;\n"
        "def result : Int := require_finalized observed;\n",
        True,
    ),
}
for name, (source, expected) in PHASE_CASES.items():
    (OUT / f"{name}.ae").write_text(source)
    actual = check(source)
    results.append({"case": name, "expected_acceptance": expected, **actual})

(OUT / "results.json").write_text(json.dumps(results, indent=2) + "\n")
print(json.dumps(results, indent=2))
assert all(result["accepted"] == result["expected_acceptance"] for result in results)
assert any(
    item["kind"] == "native"
    for result in results
    if result["case"] == "untrusted_native_finality_promise"
    for item in result["trust"]
)
