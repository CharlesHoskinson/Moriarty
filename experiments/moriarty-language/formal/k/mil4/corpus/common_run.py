#!/usr/bin/env python3
"""Three fixed round-zero K fixtures for a finite local comparison.

Use the existing repaired compiled definition and independent literal accepted
results. This harness injects an abstract premise; it does not authenticate it.
"""
import json
import subprocess
from pathlib import Path

from run import DEFINITION, S, U, call, repay_case, transfer_case
from run_checked import EXPECTED_ACCEPTED, check_case

HERE = Path(__file__).resolve().parent


def common_cases():
    return [
        transfer_case('T-10-1', round=0),
        repay_case('R-30', round=0),
        repay_case('R-near-bound', principal=S-1, accrued=1, amount=1,
                   payer_balance=1, creditor_balance=U-1, allowance=1,
                   spent=U-1, round=0),
    ]


def main():
    results = []
    for case in common_cases():
        expected_out = EXPECTED_ACCEPTED[case['name']]
        row = check_case(case, expected_out)
        results.append(row)
        matched = row['matched']
        print(f'{case["name"]} round=0: {"MATCH" if matched else "MISMATCH"}', flush=True)
    (HERE / 's1b-common-results.json').write_text(json.dumps(results, indent=2) + '\n')
    return 0 if len(results) == 3 and all(row['matched'] for row in results) else 1


if __name__ == '__main__':
    raise SystemExit(main())
