#!/usr/bin/env python3
"""T23 post-T25 settle: the t10a `--args` file of one program, checked against the public wrap statement of the keys.

    settle_args.py SOURCE.json WRAP.public.json OUT-ledger-args.json [--old OLD-ledger-args.json] [--anchor-only]

SOURCE is a prove result (`t23-prove-result/1`, also `.partial`; t7_chain rslchain) or a T10 `ledger-args.json`
(chain.sh): both carry c_p, input_pub and outcome_pub. anchor = c_p (FP-4, t23.accept_words). claim_pub is
recomputed (transientHash) and compared with the source's own words, and lanes 10, 20, 34 and 39 of WRAP.public.json
(the 1,026 hex lanes of a wrap made on the same key dir) must hold anchor, claim_pub, claim_pub and outcome_pub.
`--anchor-only` checks lane 10 only: for the T10 stand-in A, whose key-k21 wrap statement is the keygen fixture
(its claim lanes are not the args.json claim; the anchor lane is). `--old` compares the anchor with an earlier ledger's args (equal anchor = that ledger could be reused for the call;
a new wrap key needs a new ledger anyway, the key is installed at deploy).
Prints booleans only; never prints a word. OUT is written 0600 and is byte-equal in form to T17's ledger-args.json.
"""
import json
import os
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import accept_args as aa  # noqa: E402


def main(argv):
    anchor_only = "--anchor-only" in argv
    argv = [a for a in argv if a != "--anchor-only"]
    if len(argv) not in (4, 6) or (len(argv) == 6 and argv[4] != "--old"):
        raise SystemExit(__doc__)
    src = json.loads(pathlib.Path(argv[1]).read_text())
    w = {k: int(src[k]) for k in ("c_p", "input_pub", "outcome_pub")}
    w["claim_pub"] = aa.transient_hash([w["c_p"], w["input_pub"], w["outcome_pub"]])
    w["anchor"] = w["c_p"]
    lanes = [int(h, 16) for h in json.loads(pathlib.Path(argv[2]).read_text())]
    rep = {"source_anchor_is_c_p": int(src.get("anchor", w["c_p"])) == w["c_p"],
           "source_claim_pub_equal": int(src["claim_pub"]) == w["claim_pub"] if "claim_pub" in src else None,
           "wrap_lanes": len(lanes)}
    if anchor_only:
        rep["wrap_lane_10_anchor_equal"] = len(lanes) == aa.CALL_VECTOR_LEN and lanes[aa.LANE_ANCHOR] == w["anchor"]
        lanes_ok = rep["wrap_lane_10_anchor_equal"]
    else:
        try:
            aa.check_public(w, {"anchor": src.get("anchor", w["c_p"]), "call_vector": lanes,
                                **({"claim_pub": src["claim_pub"]} if "claim_pub" in src else {})})
            rep["wrap_lanes_10_20_34_39_equal"] = True
        except aa.Mismatch as e:
            rep["wrap_lanes_10_20_34_39_equal"] = False
            rep["mismatch"] = str(e).split(" is ")[0].split(":")[0]  # lane and field name only, never the words
        lanes_ok = rep["wrap_lanes_10_20_34_39_equal"]
    if len(argv) == 6:
        old = json.loads(pathlib.Path(argv[5]).read_text())
        rep["anchor_equals_old"] = int(old["anchor"]) == w["anchor"]
        rep["claim_args_equal_old"] = all(int(old[k]) == w[k] for k in ("c_p", "input_pub", "outcome_pub"))
    ok = rep["source_anchor_is_c_p"] and rep["source_claim_pub_equal"] is not False and lanes_ok
    if ok:
        out = pathlib.Path(argv[3])
        out.parent.mkdir(parents=True, exist_ok=True)
        fd = os.open(out, os.O_WRONLY | os.O_CREAT | os.O_TRUNC, 0o600)
        with os.fdopen(fd, "wb") as f:
            f.write(aa.ledger_args_bytes(w))
    rep["written"] = ok
    print(json.dumps(rep))
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
