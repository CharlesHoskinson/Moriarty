#!/usr/bin/env python3
"""OPX4 falsifiers for the four-piece chain (O''). NOT RUN BY ITS AUTHOR.

Usage: python3 -I hash_chain4.py A2_DIR RECEIPTS_DIR OUT_JSON PARENT_CHAIN_JSON

Chain (acyclic; 9 applications, one parameter each, each from the preceding blueprint):
  U : C2, VS1, VS2, VS4 hashed unparameterized
  B1 = apply(U, vi, C2)                        -> VI
  B2 = apply(B1, vs3, C2); B3 = apply(B2, vs3, VI) -> VS3
  B4..B9 = apply s2p with C2, VI, VS1, VS2, VS3, VS4 in declaration order -> S2' (derived last)
  final: the six other hashes re-derived from B9.

Reads the ten blueprints in A2_DIR, the tool receipts OPX4 wrote in RECEIPTS_DIR and the pinned
attempt-2 chain.json (c2f06fa4..., raw parent observation, used for PS4-5/PS4-6 comparisons only).
Writes OUT_JSON with mode "x". Exit 0 only if every falsifier passes, 1 otherwise, 2 on usage or pin.

Falsifiers (FAILED(hash-chain) when any holds):
  * a blueprint whose title set is not exactly the 16 expected titles (no vs.*, no s2.*), or whose
    handlers of one validator differ in compiledCode, hash or parameters;
  * a parameter count other than the stage's remaining count (vi 1, vs3 2, s2p 6, others 0 in U),
    or parameter titles other than the declared tail;
  * an application that changes a non-target validator, or leaves its target's code unchanged;
  * a tool hash different from the blueprint hash field at the stage where the validator is fully
    applied, or in B9, or a final re-derivation that differs;
  * parameter bytes other than 581c || the source validator's tool hash, or a parameter consumed
    before its source validator was fully applied (cycle);
  * seven hashes not all 56 lowercase hex and pairwise distinct, any equal to a test identifier
    (#b1, #b2, #b3, #c1..#c4 x 28), a parameter left in B9, S2' equal to attempt 2's S2' or R1's S2.
Recorded predictions (not falsifiers): C2 and VI hashes equal attempt 2's; VI's B1 compiled-code
sha256 equals attempt 2's (PS4-5); C2's compiled code equals attempt 2's.
"""
import hashlib
import json
import os
import re
import sys

STAGES = [
    ("U", "plutus.unapplied.json", None),
    ("B1", "b1-vi.json", "vi"),
    ("B2", "b2-vs3-c2.json", "vs3"),
    ("B3", "b3-vs3.json", "vs3"),
    ("B4", "b4-s2p-c2.json", "s2p"),
    ("B5", "b5-s2p-vi.json", "s2p"),
    ("B6", "b6-s2p-vs1.json", "s2p"),
    ("B7", "b7-s2p-vs2.json", "s2p"),
    ("B8", "b8-s2p-vs3.json", "s2p"),
    ("B9", "b9-applied.json", "s2p"),
]
TITLES = {
    "c2": ["c2.c2.mint", "c2.c2.spend", "c2.c2.else"],
    "vi": ["vi.vi.withdraw", "vi.vi.else"],
    "vs1": ["vs1.vs1.withdraw", "vs1.vs1.else"],
    "vs2": ["vs2.vs2.withdraw", "vs2.vs2.else"],
    "vs3": ["vs3.vs3.withdraw", "vs3.vs3.else"],
    "vs4": ["vs4.vs4.withdraw", "vs4.vs4.else"],
    "s2p": ["s2p.s2p.mint", "s2p.s2p.spend", "s2p.s2p.else"],
}
DECLARED = {"c2": [], "vi": ["c2_hash"], "vs1": [], "vs2": [], "vs3": ["c2_hash", "vi_hash"], "vs4": [],
            "s2p": ["c2_hash", "vi_hash", "vs1_hash", "vs2_hash", "vs3_hash", "vs4_hash"]}
APPLIED_AT = {"c2": ("U", "hash-c2.txt"), "vs1": ("U", "hash-vs1.txt"), "vs2": ("U", "hash-vs2.txt"),
              "vs4": ("U", "hash-vs4.txt"), "vi": ("B1", "hash-vi.txt"), "vs3": ("B3", "hash-vs3.txt"),
              "s2p": ("B9", "hash-s2p.txt")}
FINAL = {g: "hash-%s-final.txt" % g for g in ["c2", "vi", "vs1", "vs2", "vs3", "vs4"]}
# (receipt, consuming stage, source validator), in application order B1..B9.
PARAMS = [
    ("param-vi-1-c2.hex", "B1", "c2"),
    ("param-vs3-1-c2.hex", "B2", "c2"),
    ("param-vs3-2-vi.hex", "B3", "vi"),
    ("param-s2p-1-c2.hex", "B4", "c2"),
    ("param-s2p-2-vi.hex", "B5", "vi"),
    ("param-s2p-3-vs1.hex", "B6", "vs1"),
    ("param-s2p-4-vs2.hex", "B7", "vs2"),
    ("param-s2p-5-vs3.hex", "B8", "vs3"),
    ("param-s2p-6-vs4.hex", "B9", "vs4"),
]
TEST_IDS = {x * 28 for x in ["b1", "b2", "b3", "c1", "c2", "c3", "c4"]}
PARENT_CHAIN_SHA256 = "c2f06fa4f80c351adc88a179a8b33c723d1674831221cc83b78b0f5324e5ba34"
PARENT = {"c2": "6a4b67e7d46dbada1b4f49dd324018324b60b13bebbeb6f33d9d59c1",
          "vi": "fc2266228a6a5072946ba7bef649be56fd984ee851d50e57977c1f64",
          "vs": "790f51b3b77456069b1cc36275979a4ce04b79064fb051705f9c5ed0",
          "s2p": "def07bb08595fdc35e4eb2311d39a1fb81861b9b4f6d5c13f1831599"}
PARENT_VI_B1_CODE = "593dc0539cf74427aba74ae61a6469218b476d26687738a5440cd924ffb00aad"
PARENT_C2_CODE = "ac9096ea103e078bd2f2f70970866bc0804ab21123d4a055b9ff696eed9e2486"
R1_S2 = "338293e4fcb37e76c1a7e309cd33f76bd8fc5594aec140386fbf1b7e"
HEX56 = re.compile(r"[0-9a-f]{56}")
ORDER = [s[0] for s in STAGES]


def read_line(path):
    with open(path, encoding="ascii") as f:
        text = f.read()
    if not text.endswith("\n") or text.count("\n") != 1:
        raise ValueError("not exactly one line: " + path)
    return text[:-1]


def groups(bp_path, problems, tag):
    with open(bp_path, "rb") as f:
        raw = f.read()
    d = json.loads(raw)
    vals = d.get("validators", [])
    titles = sorted(v.get("title") for v in vals)
    if titles != sorted(t for ts in TITLES.values() for t in ts):
        problems.append("%s: title set differs: %s" % (tag, titles))
    out = {}
    for g, ts in TITLES.items():
        entries = [v for v in vals if v.get("title") in ts]
        snaps = set()
        for v in entries:
            params = v.get("parameters") or []
            snaps.add((v.get("compiledCode"), v.get("hash"), len(params), tuple(p.get("title") for p in params)))
        if len(snaps) != 1:
            problems.append("%s: handlers of %s differ or are absent" % (tag, g))
            out[g] = None
            continue
        code, h, n, ptitles = snaps.pop()
        out[g] = {"code": code, "hash": h, "nparams": n, "ptitles": ptitles,
                  "codeSha256": hashlib.sha256((code or "").encode()).hexdigest(),
                  "codeBytes": len(code or "") // 2}
    return out, hashlib.sha256(raw).hexdigest()


def remaining():
    rem = {"U": {g: len(DECLARED[g]) for g in TITLES}}
    for i in range(1, len(STAGES)):
        prev, cur, target = STAGES[i - 1][0], STAGES[i][0], STAGES[i][2]
        rem[cur] = dict(rem[prev])
        rem[cur][target] -= 1
    return rem


def main():
    if len(sys.argv) != 5:
        sys.stderr.write("usage: hash_chain4.py A2_DIR RECEIPTS_DIR OUT_JSON PARENT_CHAIN_JSON\n")
        sys.exit(2)
    a2, rc, out, pchain = sys.argv[1:5]
    with open(pchain, "rb") as f:
        praw = f.read()
    if hashlib.sha256(praw).hexdigest() != PARENT_CHAIN_SHA256:
        sys.stderr.write("parent chain.json digest differs from pin\n")
        sys.exit(2)
    pdoc = json.loads(praw)
    notes = []
    if pdoc.get("hashes") != PARENT or pdoc["compiledCodeSha256"]["B1"]["vi"] != PARENT_VI_B1_CODE \
            or pdoc["compiledCodeSha256"]["U"]["c2"] != PARENT_C2_CODE:
        notes.append("parent chain.json disagrees with this tool's transcribed parent constants")

    problems = []
    snap, shas = {}, {}
    for tag, name, _ in STAGES:
        try:
            snap[tag], shas[tag] = groups(os.path.join(a2, name), problems, tag)
        except (OSError, ValueError) as e:
            problems.append("%s: unreadable blueprint %s: %s" % (tag, name, e))
            snap[tag], shas[tag] = {}, None
    rem = remaining()
    for tag, _, _ in STAGES:
        for g in TITLES:
            s = snap[tag].get(g)
            if s is None:
                continue
            if s["nparams"] != rem[tag][g]:
                problems.append("%s: %s has %d parameters, expected %d" % (tag, g, s["nparams"], rem[tag][g]))
            want = tuple(DECLARED[g][len(DECLARED[g]) - rem[tag][g]:])
            if s["ptitles"] != want:
                problems.append("%s: %s parameter titles %s, expected %s" % (tag, g, s["ptitles"], want))
    for i in range(1, len(STAGES)):
        prev, cur, target = STAGES[i - 1][0], STAGES[i][0], STAGES[i][2]
        for g in TITLES:
            a, b = snap[prev].get(g), snap[cur].get(g)
            if a is None or b is None:
                continue
            if g == target:
                if a["code"] == b["code"]:
                    problems.append("%s: applying to %s left its compiledCode unchanged" % (cur, g))
            elif a != b:
                problems.append("%s: %s changed although only %s was applied" % (cur, g, target))

    hashes = {}
    for g, (tag, receipt) in APPLIED_AT.items():
        try:
            h = read_line(os.path.join(rc, receipt))
        except (OSError, ValueError) as e:
            problems.append("receipt %s: %s" % (receipt, e))
            continue
        hashes[g] = h
        s = snap[tag].get(g)
        if s is None or s["hash"] != h:
            problems.append("%s: blueprint hash field of %s differs from tool output %s" % (tag, g, receipt))
        f = snap["B9"].get(g)
        if f is None or f["hash"] != h:
            problems.append("B9: hash field of %s differs from %s" % (g, receipt))
    for g, receipt in FINAL.items():
        try:
            if read_line(os.path.join(rc, receipt)) != hashes.get(g):
                problems.append("final re-derivation %s differs from %s" % (receipt, APPLIED_AT[g][1]))
        except (OSError, ValueError) as e:
            problems.append("receipt %s: %s" % (receipt, e))

    params = {}
    for name, stage, src in PARAMS:
        try:
            params[name] = read_line(os.path.join(rc, name))
        except (OSError, ValueError) as e:
            problems.append("receipt %s: %s" % (name, e))
            continue
        if src in hashes and params[name] != "581c" + hashes[src]:
            problems.append("%s is not 581c || %s hash" % (name, src))
        if ORDER.index(APPLIED_AT[src][0]) >= ORDER.index(stage):
            problems.append("%s consumed at %s before %s was fully applied (cycle)" % (name, stage, src))
    for src in ["c2", "vi"]:
        vals = {params.get(n) for n, _, s in PARAMS if s == src}
        if len(vals) != 1:
            problems.append("parameter bytes for %s differ across applications" % src)

    for g, h in hashes.items():
        if not HEX56.fullmatch(h):
            problems.append("%s hash is not 56 lowercase hex" % g)
        if h in TEST_IDS:
            problems.append("%s hash equals a test identifier" % g)
    if len(hashes) != 7 or len(set(hashes.values())) != 7:
        problems.append("the seven hashes are not all present and pairwise distinct")
    if hashes.get("s2p") in (PARENT["s2p"], R1_S2):
        problems.append("S2' hash equals attempt 2's S2' or R1's S2")
    for g in TITLES:
        s = snap["B9"].get(g)
        if s is not None and s["nparams"] != 0:
            problems.append("B9: %s still has parameters" % g)

    vi_b1 = (snap.get("B1", {}).get("vi") or {}).get("codeSha256")
    c2_u = (snap.get("U", {}).get("c2") or {}).get("codeSha256")
    doc = {
        "format": "four-way-vs-operator/opx4-hash-chain/1",
        "ok": not problems,
        "problems": problems,
        "notes": notes,
        "hashes": hashes,
        "parametersCborHex": params,
        "blueprintSha256": shas,
        "compiledCodeSha256": {tag: {g: (s or {}).get("codeSha256") for g, s in snap[tag].items()} for tag in snap},
        "predictions": {
            "c2HashEqualsAttempt2": hashes.get("c2") == PARENT["c2"],
            "viHashEqualsAttempt2": hashes.get("vi") == PARENT["vi"],
            "PS4-5 viB1CompiledCodeSha256EqualsAttempt2": vi_b1 == PARENT_VI_B1_CODE,
            "c2CompiledCodeEqualsAttempt2": c2_u == PARENT_C2_CODE,
            "s2pDiffersFromAttempt2 (falsifier)": hashes.get("s2p") not in (None, PARENT["s2p"]),
        },
        "qualifier": ("Tool-derived hashes of locally compiled bytes (M-HASH open): not independently "
                      "recomputed, not deployed, not registered, not signer anchors. VS1, VS2 and VS4 hashes "
                      "are shared by every deployment (N-5). Attempt-2 values are raw parent observations."),
        "notGenuine": "vi_t #b1, vs_t #b2, foreign #b3 and vs1_t..vs4_t #c1..#c4 (x 28) are test identifiers, never script hashes.",
    }
    with open(out, "x", encoding="utf-8") as f:
        json.dump(doc, f, indent=1)
        f.write("\n")
    print("OPX4 chain ok=%s problems=%d" % (not problems, len(problems)))
    sys.exit(0 if not problems else 1)


if __name__ == "__main__":
    main()
