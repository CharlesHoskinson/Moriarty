"""Synthetic grammar repros only; no K/model/actual/golden data is loaded."""
import argparse
import importlib.util
import json
import sys
from pathlib import Path


def module(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    loaded = importlib.util.module_from_spec(spec)
    sys.modules[name] = loaded
    spec.loader.exec_module(loaded)
    return loaded


def ast_tuple(value):
    if type(value) is int:
        return value
    return value.name, tuple(ast_tuple(child) for child in value.args)


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("--original", type=Path, required=True)
    cli.add_argument("--candidate", type=Path, required=True)
    cli.add_argument("--repros", type=Path, required=True)
    args = cli.parse_args()
    original = module(args.original, "original_complete_image_grammar")
    candidate = module(args.candidate, "candidate_complete_image_grammar")
    cases = json.loads(args.repros.read_bytes())["cases"]
    checked = []
    for case in cases:
        try:
            old = original.parse(case["text"])
        except original.FormatError:
            if not case["old_rejects"]:
                raise AssertionError("Original unexpectedly rejected: " + case["id"])
        else:
            if case["old_rejects"]:
                raise AssertionError("Original did not reproduce rejection: " + case["id"])
        try:
            new = candidate.parse(case["text"])
        except candidate.FormatError:
            if case["accept_candidate"]:
                raise AssertionError("Candidate rejected valid framing: " + case["id"])
        else:
            if not case["accept_candidate"]:
                raise AssertionError("Candidate consumed forbidden tail: " + case["id"])
            # Every non-nullary positive case declares the same complete tree.
            expected = ("lcNoCells", ()) if case["id"] == "legacy-nullary" else (
                "lcCell", (17, ("lcNoCells", ())))
            if ast_tuple(new) != expected:
                raise AssertionError("Complete tree changed: " + case["id"])
            if not case["old_rejects"] and ast_tuple(old) != ast_tuple(new):
                raise AssertionError("Legacy parse changed: " + case["id"])
        checked.append(case["id"])
    print(json.dumps({"status": "PASS_SYNTHETIC_COMPLETE_TREE_EMPTY_CONTINUATION_GRAMMAR",
                      "cases": checked, "count": len(checked),
                      "original_rejection_witnesses": sum(c["old_rejects"] and c["accept_candidate"] for c in cases),
                      "model_steps_or_actual_or_expected_documents_read": 0}))


if __name__ == "__main__":
    main()
