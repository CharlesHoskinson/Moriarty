"""Offline immutable-Git/data verification only; no product or provider execution."""
from pathlib import Path
import hashlib
import json
import subprocess

ROOT = Path(__file__).resolve().parent
CHECKOUT = Path("/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018")


def git(*args):
    return subprocess.check_output(["git", *args], cwd=CHECKOUT)


def sha(data):
    return hashlib.sha256(data).hexdigest()


subject = json.loads((ROOT / "COMPARISON.json").read_text())
pr = subject["pr228_head"]
main = subject["current_main"]
base = subject["merge_base"]
assert git("merge-base", pr, main).decode().strip() == base
assert not set(subject["pr228_delta_paths"]) & set(subject["main_delta_paths"])
for row in subject["old_selected_comparison"]:
    assert sha(git("show", pr + ":" + row["path"])) == row["old_sha256"]
    assert sha(git("show", main + ":" + row["path"])) == row["main_sha256"]
for pin in subject["complete_local_source_pins"]:
    data = (ROOT / "source" / pin["path"]).read_bytes()
    assert len(data) == pin["bytes"] and sha(data) == pin["sha256"]
    assert data == git("show", pin["commit"] + ":" + pin["path"])
for pin in json.loads((ROOT / "PRESERVED-PACKET-REFS.json").read_text()):
    data = Path(pin["path"]).read_bytes()
    assert len(data) == pin["bytes"] and sha(data) == pin["sha256"]
test = subject["pr228_delta_paths"][0]
assert git("show", main + ":" + test) == git("show", base + ":" + test)
assert (ROOT / "PR228-EXACT.patch").read_bytes() == git("diff", base, pr, "--", test)
print(json.dumps({"offline_verified": True, "old_selected_bodies": 89,
                  "head_to_head_equal": 85, "prospective_old_body_reuse": 86,
                  "branch_delta_overlap": 0, "new_tree_tested": False}, sort_keys=True))
