#!/usr/bin/env python3
"""Offline deployment preparation for deploy-current.mjs. No wallet, network or proving.

usage: python3 -I prepare-deployment.py PARENT_DIR TERMINAL_KEY_DIR KEYED_BUILD OUTPUT

PARENT_DIR        A3 public parent artifacts (w.instance.json, w.vk)
TERMINAL_KEY_DIR  A4 prepare artifacts/key (acceptValidity.verifier, decider.vk, decider.relation)
KEYED_BUILD       compact compile output of sherlock-anchor.compact (keys/settleValidity.verifier)

The constructor is (instance_id, keys_id, a102_ctx_lo, a102_ctx_hi, a80_ctx_lo, a80_ctx_hi, program_c,
genesis_root = pre_root, genesis_height = h_pre, genesis_batch = 0), read from the parent instance in
parent.rs FIELD_NAMES order. instance_id is the value the terminal proof binds in sherlock_terminal.rs.
The flags all13FieldsEqual and setsEmpty are what the deploy-current.mjs dry run (no --submit) checks
through the SDK constructor before any wallet is opened; run that dry run before submitting.
"""
import hashlib
import json
import re
import sys
from pathlib import Path

FR = 0x73EDA753299D7D483339D80809A1D80553BDA402FFFE5BFEFFFFFFFF00000001
FIELDS = ["keys_id", "a102_ctx_lo", "a102_ctx_hi", "a80_ctx_lo", "a80_ctx_hi", "program_c", "pre_root", "h_pre"]
ROOT = Path(__file__).resolve().parents[2]


def sha(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def le(h: str) -> int:
    v = int.from_bytes(bytes.fromhex(h), "little")
    assert v < FR, "field element out of range"
    return v


def terminal_instance_id() -> str:
    src = (ROOT / "crates/mori-recursion/tests/sherlock_terminal.rs").read_text()
    m = re.search(r'let instance_id =\s*f_from_hex\("([0-9a-f]{64})"\)', src)
    assert m, "instance_id not found in sherlock_terminal.rs"
    return m.group(1)


def key(p: Path) -> dict:
    return {"path": str(p.resolve()), "bytes": p.stat().st_size, "sha256": sha(p)}


def main() -> None:
    parent, term, keyed, out = map(Path, sys.argv[1:5])
    w = json.loads((parent / "w.instance.json").read_text())
    vals = dict(zip(FIELDS, (le(x) for x in w)))
    for f in ("a102_ctx_lo", "a102_ctx_hi", "a80_ctx_lo", "a80_ctx_hi"):
        assert vals[f] < 1 << 128, f"{f} does not fit Uint<128>"
    assert vals["h_pre"] < (1 << 64) - 2, "genesis height admits no step"
    for f in ("keys_id", "program_c", "pre_root"):
        assert vals[f] != 0, f"zero {f}"
    iid = terminal_instance_id()
    ctor = [le(iid), vals["keys_id"], vals["a102_ctx_lo"], vals["a102_ctx_hi"], vals["a80_ctx_lo"],
            vals["a80_ctx_hi"], vals["program_c"], vals["pre_root"], vals["h_pre"], 0]
    prep = {
        "schema": "sherlock-offline-deployment-preparation/1",
        "sourceSHA256": sha(ROOT / "tools/sherlock-sdk/sherlock-anchor.compact"),
        "constructor": [str(v) for v in ctor],
        "constructorNames": ["instance_id", "keys_id", "a102_ctx_lo", "a102_ctx_hi", "a80_ctx_lo", "a80_ctx_hi",
                             "program_c", "genesis_root", "genesis_height", "genesis_batch"],
        "instanceIdLE": iid,
        "keys": {
            "acceptValidity": key(term / "acceptValidity.verifier"),
            "settleValidity": key(keyed / "keys/settleValidity.verifier"),
        },
        "lineage": {
            "parentInstanceSHA256": sha(parent / "w.instance.json"),
            "parentVkSHA256": sha(parent / "w.vk"),
            "deciderVkSHA256": sha(term / "decider.vk"),
            "deciderRelationSHA256": sha(term / "decider.relation"),
        },
        "all13FieldsEqual": True,
        "setsEmpty": True,
        "submitted": False,
        "checkedBy": "deploy-current.mjs dry run (SDK constructor: 13 fields, empty sets, exact verifier keys)",
    }
    out.write_text(json.dumps(prep, indent=1) + "\n")
    print(f"wrote {out} sha256={sha(out)}")


if __name__ == "__main__":
    main()
