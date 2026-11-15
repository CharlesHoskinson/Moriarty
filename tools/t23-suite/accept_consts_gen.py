#!/usr/bin/env python3
"""Generate accept_consts.json: the hash constants that accept_args.py needs, read from the pinned
crate sources that the Sherlock reference code uses.

- BabyBear Poseidon2, width 16 (`P2` of ARCHITECTURE 1.11 FP-4): p3-baby-bear 0.4.3
  `src/poseidon2.rs` (`default_babybear_poseidon2_16`, the version in the stark-backend Cargo.lock).
- Midnight Poseidon over BLS12-381 Fr, width 3, 8 full + 60 partial rounds (`PoseidonFr_4` and the
  Compact `transientHash`): midnight-circuits 7.3.0 `src/hash/poseidon/constants/blstrs.rs`
  (the version that `sherlock-lfr/src/poseidon.rs` names).

The output records the sha256 of each source file, so a later crate change is visible.

usage: accept_consts_gen.py [REGISTRY_SRC_DIR] > accept_consts.json
"""
import glob
import hashlib
import json
import os
import re
import sys

BB_FILE = "p3-baby-bear-0.4.3/src/poseidon2.rs"
MC_FILE = "midnight-circuits-7.3.0/src/hash/poseidon/constants/blstrs.rs"


def registry():
    if len(sys.argv) > 1:
        return sys.argv[1]
    hits = sorted(glob.glob(os.path.expanduser("~/.cargo/registry/src/*/")))
    for h in hits:
        if os.path.exists(os.path.join(h, BB_FILE)) and os.path.exists(os.path.join(h, MC_FILE)):
            return h
    raise SystemExit("no cargo registry with both crates")


def bb_consts(src):
    out = {}
    for name in ("BABYBEAR_RC16_EXTERNAL_INITIAL", "BABYBEAR_RC16_EXTERNAL_FINAL", "BABYBEAR_RC16_INTERNAL"):
        i = src.index(f"pub const {name}")
        j = src.index("]);", i)
        vals = [int(x, 16) for x in re.findall(r"0x([0-9a-fA-F]+)", src[i:j])]
        out[name] = vals
    ext_i, ext_f, inter = (out["BABYBEAR_RC16_EXTERNAL_INITIAL"], out["BABYBEAR_RC16_EXTERNAL_FINAL"],
                           out["BABYBEAR_RC16_INTERNAL"])
    assert len(ext_i) == 64 and len(ext_f) == 64 and len(inter) == 13
    return {"external_initial": [ext_i[16 * r:16 * r + 16] for r in range(4)],
            "external_final": [ext_f[16 * r:16 * r + 16] for r in range(4)],
            "internal": inter}


def fq(limbs):
    return sum(int(l.replace("_", ""), 16) << (64 * k) for k, l in enumerate(limbs))


def mc_consts(src):
    raws = re.findall(r"from_raw\(\[\s*0x([0-9a-f_]+),\s*0x([0-9a-f_]+),\s*0x([0-9a-f_]+),\s*0x([0-9a-f_]+),?\s*\]\)", src)
    vals = [fq(r) for r in raws]
    m = src.index("const MDS")
    n_rc = len(re.findall(r"from_raw", src[:m]))
    assert n_rc == 204 and len(vals) == 213, (n_rc, len(vals))
    rc = [[str(vals[3 * r + i]) for i in range(3)] for r in range(68)]
    mds = [[str(vals[204 + 3 * i + j]) for j in range(3)] for i in range(3)]
    return {"round_constants": rc, "mds": mds}


def main():
    reg = registry()
    bb_path, mc_path = os.path.join(reg, BB_FILE), os.path.join(reg, MC_FILE)
    bb_src, mc_src = open(bb_path).read(), open(mc_path).read()
    out = {
        "schema": "sherlock-accept-consts/1",
        "sources": {
            BB_FILE: hashlib.sha256(bb_src.encode()).hexdigest(),
            MC_FILE: hashlib.sha256(mc_src.encode()).hexdigest(),
        },
        "babybear_poseidon2_16": bb_consts(bb_src),
        "midnight_poseidon_bls12381": mc_consts(mc_src),
    }
    json.dump(out, sys.stdout, indent=1)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
