#!/usr/bin/env python3
"""Rejection holdout runner. Usage: rejections.py local|stagenet

For each case in cases/rejections whose `scope` matches, either runs the real
verifier on a mutated artifact and requires a REJECT verdict, or exits 2 with a
precise `UNIMPLEMENTED: needs <entrypoint>` message.

Verdict rule (from the security requirements, never from current behavior):
  ACCEPT  = verifier exit 0 AND JSON verifies == true  -> case FAILS
  REJECT  = (verifier exit 0 AND verifies == false) OR typed decoder refusal (exit 1, empty stdout)
  anything else (signal, exit >1, unparsable success) -> case FAILS
Exit status 0 alone is never the oracle.
"""
import hashlib
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
HOLDOUT = HERE.parent
REPO = HOLDOUT.parent.parent
CFG = json.loads((HOLDOUT / "config" / "parent-verifier.json").read_text())


def find_exe():
    env = os.environ.get(CFG["verifierEnv"])
    cands = [env] if env else [str(REPO / c) for c in CFG["verifierCandidates"]]
    for c in cands:
        if c and os.path.isfile(c) and os.access(c, os.X_OK):
            return c
    return None


def find_dir(envname, rels):
    env = os.environ.get(envname)
    for c in ([env] if env else [str(REPO / r) for r in rels]):
        if c and os.path.isdir(c):
            return Path(c)
    return None


def find_srs():
    env = os.environ.get(CFG["srs14Env"])
    for c in ([env] if env else [str(REPO / r) for r in CFG["srs14Candidates"]]):
        if c and os.path.isfile(c):
            return Path(c)
    return None


def unimplemented_reasons(case, exe, fx, srs):
    need = []
    if not exe:
        need.append("the real parent verifier executable exposing `sherlock-verify-parent --vk --proof --instance --srs14` "
                    "(Source45 interface; set DW_PARENT_VERIFIER or build target/release/recl9)")
    files = CFG["fixtureFiles"]
    required = ["vk", "proof", "instance"] + (["wrongValidChildVk"] if case["mutation"].get("needsFixture") == "wrongValidChildVk" else [])
    missing = []
    if fx is None:
        missing = [files[k] for k in required]
    else:
        missing = [files[k] for k in required if not (fx / files[k]).is_file()]
    if missing:
        need.append("a genuine A102->A80 k20 parent artifact set (" + ", ".join(missing) +
                    f") in a directory named by DW_PARENT_FIXTURE_DIR")
    if not srs:
        need.append("the official pinned SRS14 file (sha256 " + CFG["srs14Sha256"][:16] + "..., set DW_SRS14)")
    elif hashlib.sha256(srs.read_bytes()).hexdigest() != CFG["srs14Sha256"]:
        need.append("the SRS14 file at " + str(srs) + " does not match the official pin")
    return need


def call(exe, vk, proof, inst, srs):
    cmd = [exe, *CFG["invocation"], "--vk", str(vk), "--proof", str(proof), "--instance", str(inst), "--srs14", str(srs)]
    r = subprocess.run(cmd, capture_output=True, text=True, timeout=CFG["timeoutSeconds"])
    return r


def verdict(r):
    if r.returncode == 0:
        try:
            doc = json.loads(r.stdout.strip().splitlines()[-1])
        except Exception:
            return "ERROR", "exit 0 with unparsable output"
        if doc.get("kind") != CFG["outputKind"] or not isinstance(doc.get("verifies"), bool):
            return "ERROR", f"exit 0 with unexpected report shape {sorted(doc)[:6]}"
        return ("ACCEPT" if doc["verifies"] else "REJECT"), json.dumps({k: doc.get(k) for k in ("verifies", "stdlib_verifies", "pairing_verifies", "failure")})
    if r.returncode == 1 and r.stdout.strip() == "" and r.stderr.strip():
        return "REJECT", "typed decoder refusal (exit 1)"
    return "ERROR", f"exit {r.returncode}: {r.stderr[:160]!r}"


def hexscalar_swap(inst, i, j):
    inst[i], inst[j] = inst[j], inst[i]


def mutate(m, work, fx):
    vk, proof, inst = (work / "vk.bin"), (work / "proof.bin"), (work / "instance.json")
    shutil.copy(fx / CFG["fixtureFiles"]["vk"], vk)
    shutil.copy(fx / CFG["fixtureFiles"]["proof"], proof)
    shutil.copy(fx / CFG["fixtureFiles"]["instance"], inst)
    op = m["op"]
    if op == "proof-xor-byte":
        b = bytearray(proof.read_bytes()); b[m["offset"]] ^= m["mask"]; proof.write_bytes(bytes(b))
    elif op == "proof-append-byte":
        proof.write_bytes(proof.read_bytes() + bytes([m["value"]]))
    elif op == "vk-append-byte":
        vk.write_bytes(vk.read_bytes() + bytes([m["value"]]))
    elif op == "vk-substitute":
        shutil.copy(fx / CFG["fixtureFiles"]["wrongValidChildVk"], vk)
    elif op in ("scalar-set", "scalar-flip-low-bit", "scalar-swap"):
        doc = json.loads(inst.read_text())
        assert len(doc) == 28 and all(len(s) == 64 for s in doc), "fixture instance is not the 28-scalar array"
        if op == "scalar-set":
            assert doc[m["index"]] == m["from"], f"fixture scalar {m['index']} is not the expected precondition value"
            doc[m["index"]] = m["to"]
        elif op == "scalar-flip-low-bit":
            s = doc[m["index"]]; first = int(s[:2], 16) ^ 1; doc[m["index"]] = f"{first:02x}" + s[2:]
        else:
            i, j = m["indices"]
            assert doc[i] != doc[j], "swap of equal scalars would not mutate the statement"
            hexscalar_swap(doc, i, j)
        inst.write_text(json.dumps(doc))
    else:
        raise ValueError(op)
    return vk, proof, inst


def main(scope):
    cases = []
    for d in sorted((HOLDOUT / "cases" / "rejections").iterdir()):
        c = json.loads((d / "case.json").read_text())
        if c["scope"] == scope:
            cases.append(c)
    if not cases:
        print(f"FAIL: no {scope} rejection cases")
        return 1
    exe = find_exe()
    fx = find_dir(CFG["fixtureDirEnv"], CFG["fixtureDirCandidates"])
    srs = find_srs()
    results = {"pass": [], "fail": [], "unimplemented": []}
    control_done = False
    for c in cases:
        if scope == "stagenet" or c["target"].get("kind") != "local-verifier":
            results["unimplemented"].append((c["id"], c["unimplemented"]))
            continue
        need = unimplemented_reasons(c, exe, fx, srs)
        if need:
            results["unimplemented"].append((c["id"], "needs " + "; and needs ".join(need)))
            continue
        with tempfile.TemporaryDirectory(prefix="dw-rej-") as t:
            work = Path(t)
            if not control_done:
                shutil.copy(fx / CFG["fixtureFiles"]["vk"], work / "c.vk")
                r = call(exe, work / "c.vk", fx / CFG["fixtureFiles"]["proof"], fx / CFG["fixtureFiles"]["instance"], srs)
                v, detail = verdict(r)
                if v != "ACCEPT":
                    print(f"FAIL: genuine control parent was not accepted ({v}: {detail}); the verifier cannot be trusted to reject")
                    return 1
                control_done = True
            vk, proof, inst = mutate(c["mutation"], work, fx)
            r = call(exe, vk, proof, inst, srs)
            v, detail = verdict(r)
            (results["pass"] if v == "REJECT" else results["fail"]).append((c["id"], f"{v} {detail}"))
    for cid, msg in results["pass"]:
        print(f"PASS {cid}: {msg}")
    for cid, msg in results["fail"]:
        print(f"FAIL {cid}: {msg}")
    groups = {}
    for cid, msg in results["unimplemented"]:
        groups.setdefault(msg.removeprefix("UNIMPLEMENTED: "), []).append(cid)
    for msg, ids in groups.items():
        print(f"UNIMPLEMENTED: {msg}\n  affects {len(ids)} case(s): {', '.join(ids)}")
    print(f"summary {scope}: pass={len(results['pass'])} fail={len(results['fail'])} unimplemented={len(results['unimplemented'])}")
    if results["fail"]:
        return 1
    if results["unimplemented"]:
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1]))
