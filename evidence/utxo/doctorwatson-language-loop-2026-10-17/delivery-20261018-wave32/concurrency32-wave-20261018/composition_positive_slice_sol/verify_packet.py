"""Offline source/oracle pin verification; never imports or executes product code."""
from pathlib import Path
import hashlib
import json
import subprocess

ROOT = Path(__file__).resolve().parent
CHECKOUT = Path("/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018")


def digest(data):
    return hashlib.sha256(data).hexdigest()


def main():
    source = json.loads((ROOT / "SOURCE-PINS.json").read_text())
    count = 0
    for pin in source["files"]:
        data = (ROOT / "source" / pin["path"]).read_bytes()
        assert len(data) == pin["bytes"]
        assert digest(data) == pin["sha256"]
        original = subprocess.check_output(
            ["git", "show", source["commit"] + ":" + pin["path"]], cwd=CHECKOUT
        )
        assert data == original
        blob = subprocess.check_output(
            ["git", "rev-parse", source["commit"] + ":" + pin["path"]], cwd=CHECKOUT
        ).decode().strip()
        assert blob == pin["git_blob"]
        count += 1
    oracle = json.loads((ROOT / "ORACLE-AVAILABILITY.json").read_text())
    for pin in oracle["frozen_oracle_pins"]:
        data = Path(pin["path"]).read_bytes()
        assert len(data) == pin["bytes"]
        assert digest(data) == pin["sha256"]
    print(json.dumps({"kind": "offline pins only", "source_files": count,
                      "retained_oracle_pins": len(oracle["frozen_oracle_pins"]),
                      "product_execution": False, "pins_match": True}, sort_keys=True))


if __name__ == "__main__":
    main()
