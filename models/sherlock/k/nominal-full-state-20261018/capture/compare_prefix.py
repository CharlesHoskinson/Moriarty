"""Compare one whole independent K prefix after Root's durable actual freeze."""
import argparse
import hashlib
import json
from pathlib import Path

from decode_prefix import pinned, save


def pin(path):
    raw = Path(path).read_bytes()
    return {"path": str(path), "bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest()}


def difference(left, right, path=""):
    if type(left) is not type(right):
        return path + ":kind"
    if type(left) is dict:
        if left.keys() != right.keys():
            return path + ":keys"
        for key in left:
            result = difference(left[key], right[key], path + "/" + key.replace("~", "~0").replace("/", "~1"))
            if result is not None:
                return result
    elif type(left) is list:
        if len(left) != len(right):
            return path + ":length"
        for index, (a, b) in enumerate(zip(left, right)):
            result = difference(a, b, path + "/" + str(index))
            if result is not None:
                return result
    elif left != right:
        return path + ":value"
    return None


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("manifest", type=Path)
    args = cli.parse_args()
    m = json.loads(args.manifest.read_bytes())
    # Root fsyncs these actual outputs AND all prior raw receipts/streams before
    # selecting this child. This consumer checks actual identities before ANY
    # expected file open; it never creates/fills an expected header or cell.
    root_freeze = json.loads(Path(m["root_actual_freeze"]).read_bytes())
    if pin(m["actual_wire_output"]) != root_freeze["actual_wire"] or pin(m["relation_output"]) != root_freeze["relation"]:
        raise ValueError("Actual outputs differ from Root durable freeze")
    actual = json.loads(pinned(root_freeze["actual_wire"]))
    relation = json.loads(pinned(root_freeze["relation"]))
    if actual["observerCuts"] != list(range(len(actual["states"]))):
        raise ValueError("Actual-only observer ordinals differ")
    # The original expected path/hash were independently prefrozen, selected
    # from SAFE metadata and never read by the renderer/relation author.
    expected = json.loads(pinned(m["independent_expected"]))
    mismatch = difference(actual, expected)
    result = {"status": "PASS" if mismatch is None and relation["status"] == "PASS" else "FAIL",
        "complete_K_prefix_typed_equal": mismatch is None, "mismatch_path_and_kind": mismatch,
        "frozen_primary_boundary_relation": relation["status"],
        "actual_wire": root_freeze["actual_wire"], "relation": root_freeze["relation"],
        "independent_expected": m["independent_expected"],
        "comparison": "whole six-key wire; strict scalar kinds/values, ordered arrays, unordered object key sets",
        "expected_filled_or_normalized_from_actual": False,
        "scope": "one bounded actual compiler-input K prefix and partial declared primary relation; no clock/work/physical-B/native equivalence"}
    save(m["comparison_output"], result)
    print(json.dumps(result))
    return 0 if result["status"] == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
