"""Offline proposal/source/data integrity only; no product evaluation or provider calls."""
from pathlib import Path
import hashlib
import json
import subprocess

ROOT = Path(__file__).resolve().parent
CHECKOUT = Path("/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018")


def sha(data):
    return hashlib.sha256(data).hexdigest()


pins = json.loads((ROOT / "SOURCE-PINS.json").read_text())
assert len(pins["files"]) == 71
for pin in pins["files"]:
    data = (ROOT / "basis" / pin["path"]).read_bytes()
    assert len(data) == pin["bytes"] and sha(data) == pin["sha256"]
    original = subprocess.check_output(
        ["git", "show", pins["commit"] + ":" + pin["path"]], cwd=CHECKOUT
    )
    assert data == original
    blob = subprocess.check_output(
        ["git", "rev-parse", pins["commit"] + ":" + pin["path"]], cwd=CHECKOUT
    ).decode().strip()
    assert blob == pin["git_blob"]
mplr = json.loads((ROOT / "ORIGINAL-MPLR-001-035.json").read_text())
assert [row["id"] for row in mplr] == [f"MPLR-{n:03}" for n in range(1, 36)]
for row in mplr:
    data = (ROOT / "basis" / row["source_path"]).read_bytes()
    assert sha(data) == row["source_sha256"]
    assert row["verbatim_required_behavior"] in data.decode()
primary = (ROOT / "PRIMARY.md").read_bytes()
meta = json.loads((ROOT / "PRIMARY-COMPLETENESS.json").read_text())
assert len(primary) == meta["primary_bytes"] and sha(primary) == meta["primary_sha256"]
assert len(primary) + meta["recommended_wrapper_maximum_bytes"] < 125000
for entry in meta["inline_entries"]:
    data = (ROOT / entry["path"]).read_bytes()
    if entry["mode"] == "whole":
        assert sha(data) == entry["sha256"] and data in primary
    elif entry["mode"] == "exact-source-excerpt":
        assert sha(data) == entry["whole_sha256"]
        excerpt = "\n".join(data.decode().splitlines()[entry["first_line"] - 1:entry["last_line"]]) + "\n"
        assert sha(excerpt.encode()) == entry["sha256"]
        assert excerpt.encode() in primary
    else:
        assert entry["mode"] == "lossless-required-behavior-text-extract"
        assert sha(data) == entry["complete_original_json_sha256"]
for row in mplr:
    assert row["verbatim_required_behavior"].replace("## Required behavior\n\n", "").encode() in primary
staged = ROOT / "basis/crates/fixtures/core-disjoint-product/two-deliveries.mori"
assert staged.read_bytes() == (ROOT / "examples/staged-first.mori").read_bytes()
for name in ["two-programs.plan.proposed.json", "later-credit.plan.proposed.json", "staged-predecessor.plan.proposed.json"]:
    plan = json.loads((ROOT / "examples" / name).read_text())
    paths = [plan["opening"]] + [x["source"] for x in plan["programs"]]
    paths += [x["inputs"] for x in plan["invocations"]]
    paths += [x["event"] for x in plan["operations"] if "event" in x]
    assert all((ROOT / "examples" / path).is_file() for path in paths)
    assert all("scenario" not in invocation for invocation in plan["invocations"])
    assert all("expected" not in invocation for invocation in plan["invocations"])
walk = json.loads((ROOT / "examples/AUTHOR-WORKED-PREFIXES.json").read_text())
assert walk["status"].startswith("AUTHOR-WORKED") and len(walk["prefixes"]) == 15
for prefix in walk["prefixes"]:
    head = prefix["whole_head"]
    assert head["state"]["outside.marker"]["value"] == "77"
    assert head["state"]["future.quota"]["value"] == "9"
    assert next(x for x in head["liabilities"] if x["name"] == "outside.duty")["amount"] == "5"
    assert prefix["closure_reserve"] == "7"
    assert int(prefix["author_remaining_if_this_statement_prefix_is_counted"]) + int(prefix["author_statement_cost_sum"]) == 40
assert [walk["prefixes"][i]["author_statement_cost_sum"] for i in [6, 12, 14]] == ["12", "26", "30"]
print(json.dumps({"offline_integrity": True, "complete_basis_files": 71,
                  "original_MPLR_requirements": 35, "primary_bytes": len(primary),
                  "author_worked_prefixes": 15, "product_execution": False,
                  "independent_oracle_claim": False}, sort_keys=True))
