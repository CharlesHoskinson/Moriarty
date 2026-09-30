"""Research-only Aeon refinement probes for a fixed signed outcome envelope."""

import json
import hashlib
from pathlib import Path

from aeon.facade.driver import AeonConfig, AeonDriver
from aeon.facade.trust import compute_trust_report
from aeon.synthesis.uis.api import SilentSynthesisUI


PREFIX = """
# Integer IDs stand in for canonical domain and recipient names.
# These declarations model facts after signature verification; Aeon does not
# authenticate the envelope or its bytes.
def signedRecipient : {r:Int | r = 42} := 42;
def signedDomain : {d:Int | d = 10} := 10;
def maxGross : {g:Int | g = 100} := 100;
def maxFee : {f:Int | f = 5} := 5;
"""

CASES = {
    "authorized": """
def planRecipient : {r:Int | r = signedRecipient} := 42;
def planDomain : {d:Int | d = signedDomain} := 10;
def planPrincipal : {p:Int | p >= 0} := 94;
def planFee : {f:Int | f >= 0 && f <= maxFee} := 4;
def planGross : {g:Int | g = planPrincipal + planFee && g <= maxGross} := 98;
""",
    "authorized_split": """
def planRecipient : {r:Int | r = signedRecipient} := 42;
def planDomain : {d:Int | d = signedDomain} := 10;
def planPrincipal : {p:Int | p = 94} := 94;
def planFee : {f:Int | f = 4} := 4;
def planGross : {g:Int | g = planPrincipal + planFee} := 98;
def grossWithinCap : {g:Int | g <= maxGross} := planGross;
def feeWithinCap : {f:Int | f <= maxFee} := planFee;
""",
    "recipient_substitution": """
def planRecipient : {r:Int | r = signedRecipient} := 43;
""",
    "domain_substitution": """
def planDomain : {d:Int | d = signedDomain} := 11;
""",
    "fee_cap_violation": """
def planFee : {f:Int | f >= 0 && f <= maxFee} := 6;
""",
    "gross_cap_violation": """
def planPrincipal : {p:Int | p >= 0} := 97;
def planFee : {f:Int | f >= 0 && f <= maxFee} := 4;
def planGross : {g:Int | g = planPrincipal + planFee && g <= maxGross} := 101;
""",
    "gross_cap_violation_split": """
def planPrincipal : {p:Int | p = 97} := 97;
def planFee : {f:Int | f = 4} := 4;
def planGross : {g:Int | g = planPrincipal + planFee} := 101;
def grossWithinCap : {g:Int | g <= maxGross} := planGross;
""",
    "inconsistent_gross": """
def planPrincipal : {p:Int | p >= 0} := 94;
def planFee : {f:Int | f >= 0 && f <= maxFee} := 4;
def planGross : {g:Int | g = planPrincipal + planFee && g <= maxGross} := 97;
""",
    "trusted_native_forge": """
def planRecipient : {r:Int | r = signedRecipient} := native "43";
""",
}


def main():
    results = []
    out = Path(__file__).resolve().parent
    for name, body in CASES.items():
        source = PREFIX + body
        (out / (name + ".ae")).write_text(source)
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
        results.append(
            {
                "case": name,
                "source_sha256": hashlib.sha256(source.encode()).hexdigest(),
                "accepted": not errors,
                "error_types": [type(error).__name__ for error in errors],
                "errors": [str(error)[:1200] for error in errors],
                "trust": trust,
            }
        )
    (out / "results.json").write_text(json.dumps(results, indent=2) + "\n")
    print(json.dumps(results, indent=2))


if __name__ == "__main__":
    main()
