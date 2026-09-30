"""Bounded bridge arithmetic probes in Aeon; no network or ledger calls."""

import json
import sys

from loguru import logger
from aeon.facade.driver import AeonConfig, AeonDriver
from aeon.facade.trust import compute_trust_report
from aeon.synthesis.uis.api import SilentSynthesisUI


CASES = {
    "withhold_while_unknown": """
def withhold (sent:Int | sent > 0)
             (delivered:Int | delivered >= 0 && delivered <= sent)
             (status:Int | status = 1)
             : {refund:Int | refund >= 0 && delivered + refund <= sent} := 0;
""",
    "timeout_alone_full_refund": """
def refund_on_timeout (sent:Int | sent > 0)
                      (delivered:Int | delivered >= 0 && delivered <= sent)
                      (timed_out:Bool)
                      : {refund:Int | refund >= 0 && delivered + refund <= sent} :=
    if timed_out then sent else 0;
""",
    "unknown_status_full_refund": """
def refund_unknown (sent:Int | sent > 0)
                   (delivered:Int | delivered >= 0 && delivered <= sent)
                   (status:Int | status = 1)
                   : {refund:Int | refund >= 0 && delivered + refund <= sent} := sent;
""",
    "observed_zero_does_not_prove_nonreceipt": """
def refund_on_empty_observation (sent:Int | sent > 0)
                                (actual_delivery:Int | actual_delivery >= 0 && actual_delivery <= sent)
                                (observed_delivery:Int | observed_delivery = 0)
                                (status:Int | status = 1)
                                : {refund:Int | refund >= 0 && actual_delivery + refund <= sent} := sent;
""",
    "partial_delivery_full_refund": """
def refund_partial (sent:Int | sent > 0)
                   (delivered:Int | delivered > 0 && delivered <= sent)
                   : {refund:Int | refund >= 0 && delivered + refund <= sent} := sent;
""",
    "refund_remaining_after_partial": """
def refund_remaining (sent:Int | sent > 0)
                     (delivered:Int | delivered >= 0 && delivered <= sent)
                     : {refund:Int | refund >= 0 && delivered + refund <= sent} :=
    sent - delivered;
""",
    "full_refund_with_proven_zero_delivery": """
def refund_zero (sent:Int | sent > 0)
                (delivered:Int | delivered = 0)
                : {refund:Int | refund >= 0 && delivered + refund <= sent} := sent;
""",
}


def main():
    logger.remove()
    rows = []
    for name, source in CASES.items():
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
        rows.append({
            "case": name,
            "accepted": not errors,
            "errors": [type(error).__name__ + ": " + str(error) for error in errors],
            "trust": trust,
            "source": source.strip(),
        })
    json.dump(rows, sys.stdout, indent=2)
    print()


if __name__ == "__main__":
    main()
