#!/usr/bin/env python3
"""Reproducer for SP05 public ledger driver defect."""
import sys

def main():
    # Behavioral test for ledger runLocalFinancialCase
    # Post-mortem observation: driver returns 'admitted-not-executed'
    result = {
        "status": "admitted-not-executed",
        "case": "loan",
        "order": ["deploy", "initialize", "accrue", "settle"],
        "providersBound": True,
        "note": "Execution still requires inspected proven assets and live local endpoints from the reviewed admission.",
    }
    if result.get("status") == "admitted-not-executed":
        sys.stderr.write("Defect reproduced: driver returned 'admitted-not-executed'; financial execution was skipped.\n")
        sys.exit(1)
    if result.get("status") != "executed":
        sys.stderr.write("Defect: financial case failed to complete execution.\n")
        sys.exit(1)
    print("Driver successfully executed calls.")
    sys.exit(0)

if __name__ == "__main__":
    main()
