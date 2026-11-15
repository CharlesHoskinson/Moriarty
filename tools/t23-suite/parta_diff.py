#!/usr/bin/env python3
"""T23 Part A: the per-call diff of a Preview rerun against the T18 (a) run of 2026-11-14.

Reads the Calls tables of the two PREVIEW.md files (report.py output) and prints a JSON list of
{program, call, circuit, old, new, changed}. The 2026-11-14 notes.json overrides are applied to the old verdicts
only (they are the explanations recorded then); the rerun's verdicts are raw.
A call is changed when its result bucket (pass, refused-as-native, mismatch, blocked/not realizable, n/a) changed;
a wording-only change of the verdict label is kept apart (labelChanged). With OLD_STATE and NEW_STATE (harness
state dirs) each row also carries the raw harness verdict and the error text (node error codes 138, 194, 214).
Usage: parta_diff.py OLD_PREVIEW.md NEW_PREVIEW.md OUT.json [OLD_STATE NEW_STATE]
"""
import json
import re
import sys


def calls(md):
    out, on = {}, False
    for line in open(md):
        if line.startswith("## Calls"):
            on = True
            continue
        if on and line.startswith("## "):
            break
        if on and line.startswith("| `"):
            c = [x.strip() for x in line.strip().strip("|").split("|")]
            prog, call = c[0].strip("`"), c[1].strip("`")
            out[(prog, call)] = {"program": prog, "call": call, "circuit": c[2].strip("`"), "result": c[3], "tx": c[4].strip("`"),
                                 "block": c[5], "words": c[7], "verdict": c[-1]}
    return out


def bucket(v):
    if v.startswith("PASS (refused"):
        return "refused-as-native"
    if v.startswith("PASS"):
        return "pass"
    if v.startswith("MISMATCH"):
        return "mismatch"
    if v.startswith("n/a"):
        return "n/a"
    return "blocked/not realizable"


def raw(state_dir, prog, call):
    import pathlib
    f = pathlib.Path(state_dir) / f"{prog}.json"
    if not f.exists():
        return None
    c = json.loads(f.read_text()).get("calls", {}).get(call)
    if not c:
        return None
    err = str(c.get("error") or "")
    codes = sorted(set(re.findall(r"\b(?:error|Custom error:?)\s*:?\s*(\d{3})\b", err)))
    return {"verdict": c.get("verdict"), "error": err[:240] or None, "nodeErrorCodes": codes}


def norm(e):
    """Error class: the message without hex, numbers and addresses."""
    return re.sub(r"[0-9a-fA-F]{8,}|\d+", "#", e or "")[:120]


def main():
    old, new = calls(sys.argv[1]), calls(sys.argv[2])
    sd = sys.argv[4:6] if len(sys.argv) >= 6 else None
    rows = []
    for k in sorted(set(old) | set(new)):
        o, n = old.get(k), new.get(k)
        if o and n and sd:
            # the rerun has no notes.json overrides: carry the 2026-11-14 label when the raw harness verdict and
            # the error class are the same as then (e.g. a `$` circuit id refused, a coin the wallet lacks)
            ro, rn = raw(sd[0], *k), raw(sd[1], *k)
            if ro and rn and ro["verdict"] == rn["verdict"] and norm(ro["error"]) == norm(rn["error"]) and n["verdict"] != o["verdict"] \
                    and not o["verdict"].startswith("BLOCKED"):
                n = {**n, "verdict": o["verdict"], "carried": True}
        rows.append({"program": k[0], "call": k[1], "circuit": (n or o)["circuit"], "old": o and o["verdict"], "new": n and n["verdict"],
                     "oldBucket": o and bucket(o["verdict"]), "newBucket": n and bucket(n["verdict"]),
                     "carried20261114Label": bool(n and n.get("carried")), "new_tx": n and n["tx"], "new_block": n and n["block"], "words": n and n["words"],
                     "labelChanged": (o and o["verdict"]) != (n and n["verdict"]),
                     "changed": bool(n) and (o and bucket(o["verdict"])) != bucket(n["verdict"]),
                     "oldRaw": raw(sd[0], *k) if sd else None, "newRaw": raw(sd[1], *k) if sd else None})
    tot = lambda key: {b: sum(1 for r in rows if r[key] == b) for b in ("pass", "refused-as-native", "mismatch", "blocked/not realizable", "n/a", None)}
    json.dump({"rows": rows, "old": tot("oldBucket"), "new": tot("newBucket"), "changed": sum(r["changed"] for r in rows)},
              open(sys.argv[3], "w"), indent=1)
    print(json.dumps({"old": tot("oldBucket"), "new": tot("newBucket"), "changed": sum(r["changed"] for r in rows)}))


if __name__ == "__main__":
    main()
