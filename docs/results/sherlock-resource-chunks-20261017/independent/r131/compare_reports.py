#!/usr/bin/env python3
"""Ordinary exact JSON data comparison against independently sealed R131 goldens.
No model, Machine, compiler, production reporting or provider imports/calls.
Object member order is not meaningful JSON; array order, type and presence are exact.
"""
import argparse
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPORTS = {name: HERE / ("EXPECTED-" + name.upper() + "-REPORT.json")
           for name in ("whole", "first", "second", "cumulative")}

class Difference(ValueError):
    pass

def exact(expected, actual, path=""):
    if type(expected) is not type(actual):
        raise Difference(f"{path or '/'}: expected {type(expected).__name__}, got {type(actual).__name__}")
    if isinstance(expected, dict):
        if expected.keys() != actual.keys():
            raise Difference(f"{path or '/'}: missing={sorted(expected.keys()-actual.keys())}, extra={sorted(actual.keys()-expected.keys())}")
        for key, value in expected.items():
            escaped = key.replace("~", "~0").replace("/", "~1")
            exact(value, actual[key], path + "/" + escaped)
    elif isinstance(expected, list):
        if len(expected) != len(actual):
            raise Difference(f"{path or '/'}: expected {len(expected)} entries, got {len(actual)}")
        for index, (left, right) in enumerate(zip(expected, actual)):
            exact(left, right, path + "/" + str(index))
    elif expected != actual:
        raise Difference(f"{path or '/'}: expected {expected!r}, got {actual!r}")

def compare(name, actual):
    expected = json.loads(REPORTS[name].read_text())
    exact(expected, actual)

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("report", choices=REPORTS)
    parser.add_argument("actual", type=Path)
    args = parser.parse_args()
    try:
        compare(args.report, json.loads(args.actual.read_text()))
    except (Difference, ValueError, OSError) as error:
        parser.exit(1, str(error) + "\n")
    print("PASS: complete selected report JSON equals frozen " + args.report + " expectation")

if __name__ == "__main__":
    main()
