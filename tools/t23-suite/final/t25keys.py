#!/usr/bin/env python3
"""gpu-leafl-t25 key check against the T25 key table.

usage: t25keys.py extract LOG            -> prints the key record (one 'name<TAB>value' per line)
       t25keys.py check EXPECTED LOG...  -> exit 0 if every key LOG gives exactly EXPECTED, else exit 3
       t25keys.py checkcall EXPECTED LOG... -> the same for `rslchain call` logs (T10CALL fields only)
       t25keys.py checksuite EXPECTED BUCKETS LOG -> T23 final run: one `rslchain key` log of any
                                            suite call, exit 0 if equal, else exit 3

Fields come from the RSLKEY, RSLCHAIN and T10CALL JSON lines and the T7KEY / T7CKEY lines of a
`t7_chain rslchain key` log (T10CALL fields also from a `rslchain call` log). A field missing in
a log is a mismatch.

checksuite (T23 final run) splits the T25 record of t02 in three:
- per call, never compared: prog_d and image_d (the program is bound by prog_d, rsl-uni RSLU-2);
- per R_SL bucket: rsl_key_leaf and the planner key_root (RSLKEY, RSLCHAIN and T10CALL key_root).
  The bucket comes from the RSLKEY line. Plain must equal the T25 record (1832b29f, 659d6ff6).
  Any other bucket (Sha) has no T25 record: its first call writes one row to BUCKETS
  (bucket, rsl_key_leaf, key_root, log), and every later call of that bucket must equal it.
  The lead compares the BUCKETS files of all boxes on muninn;
- universal, equal to T25 for every call: every other field of EXPECTED (key_root_without_rsl,
  keys_d, l_pre_hash, lfr_pre_hash, x, the lfr shape, n1_pre_hash_equals_universal_n1, rsl_key_leaf_idx,
  the T7KEY / T7CKEY lines).
"""
import json
import os
import sys

JSON_FIELDS = {
    "RSLKEY": ["key_root", "rsl_key_leaf", "rsl_key_leaf_idx", "n1_pre_hash_equals_universal_n1"],
    "RSLCHAIN": ["key_root", "key_root_without_rsl", "keys_d", "rsl_key_leaf", "prog_d", "image_d",
                 "n1_pre_hash_equals_universal_n1"],
    "T10CALL": ["key_root", "keys_d", "l_pre_hash", "lfr_pre_hash", "x", "prog_d", "lfr_airs",
                "lfr_widths", "lfr_public_words"],
}
CALL_TAGS = ("T10CALL",)


def extract(path, call=False):
    rec = {}
    with open(path, errors="replace") as f:
        for line in f:
            line = line.rstrip("\n")
            tag, _, rest = line.partition(" ")
            if tag in ("T7KEY", "T7CKEY") and not call:
                rec[f"{tag}:{rest}"] = "line"
            elif tag in JSON_FIELDS and (not call or tag in CALL_TAGS):
                try:
                    d = json.loads(rest)
                except ValueError:
                    continue
                for k in JSON_FIELDS[tag]:
                    if k in d:
                        rec[f"{tag}.{k}"] = json.dumps(d[k], sort_keys=True)
    return rec


PER_CALL = ("RSLCHAIN.prog_d", "RSLCHAIN.image_d", "T10CALL.prog_d")
PER_BUCKET = ("RSLKEY.key_root", "RSLKEY.rsl_key_leaf", "RSLCHAIN.key_root", "RSLCHAIN.rsl_key_leaf",
              "T10CALL.key_root")


def bucket_of(path):
    with open(path, errors="replace") as f:
        for line in f:
            if line.startswith("RSLKEY "):
                try:
                    return json.loads(line[len("RSLKEY "):]).get("bucket")
                except ValueError:
                    return None
    return None


def read_tsv(path):
    exp = {}
    with open(path) as f:
        for line in f:
            k, _, v = line.rstrip("\n").partition("\t")
            if k:
                exp[k] = v
    return exp


def checksuite(expected, buckets, log):
    exp = read_tsv(expected)
    got = extract(log)
    b = bucket_of(log)
    diff = []
    if b is None:
        diff.append(("RSLKEY.bucket", "<a bucket>", "<absent>"))
    universal = {k: v for k, v in exp.items() if k not in PER_CALL and k not in PER_BUCKET}
    for k in sorted(set(universal) | {g for g in got if g not in PER_CALL and g not in PER_BUCKET}):
        if universal.get(k) != got.get(k):
            diff.append((k, universal.get(k, "<absent>"), got.get(k, "<absent>")))
    leaf = got.get("RSLKEY.rsl_key_leaf")
    root = got.get("RSLKEY.key_root")
    same = all(got.get(k) == leaf for k in ("RSLCHAIN.rsl_key_leaf",)) and \
        all(got.get(k) == root for k in ("RSLCHAIN.key_root", "T10CALL.key_root"))
    if not same:
        diff.append(("bucket fields", "RSLKEY = RSLCHAIN = T10CALL", "differ inside the log"))
    note = ""
    if b == "Plain":
        for k in PER_BUCKET:
            if exp.get(k) != got.get(k):
                diff.append((k, exp.get(k, "<absent>"), got.get(k, "<absent>")))
    elif b is not None:
        rec = {}
        if os.path.exists(buckets):
            with open(buckets) as f:
                for line in f:
                    p = line.rstrip("\n").split("\t")
                    if len(p) >= 3 and not line.startswith("#"):
                        rec[p[0]] = (p[1], p[2])
        if b in rec:
            if rec[b] != (leaf, root):
                diff.append((f"bucket {b} leaf/key_root", "/".join(rec[b]), f"{leaf}/{root}"))
        elif not diff and leaf and root:
            with open(buckets, "a") as f:
                f.write(f"{b}\t{leaf}\t{root}\t{log}\n")
            note = f" (first {b} call: bucket record written)"
    if diff:
        print(f"MISMATCH {log} bucket {b}")
        for k, w, g in diff:
            print(f"  {k}\n    T25  {w}\n    got  {g}")
        return 3
    print(f"T25SUITE EQUAL {log} bucket {b} ({len(universal)} universal fields){note}")
    return 0


def main():
    if len(sys.argv) == 5 and sys.argv[1] == "checksuite":
        return checksuite(sys.argv[2], sys.argv[3], sys.argv[4])
    if len(sys.argv) >= 3 and sys.argv[1] == "extract":
        for k, v in sorted(extract(sys.argv[2]).items()):
            print(f"{k}\t{v}")
        return 0
    if len(sys.argv) >= 4 and sys.argv[1] in ("check", "checkcall"):
        call = sys.argv[1] == "checkcall"
        exp = {}
        with open(sys.argv[2]) as f:
            for line in f:
                k, _, v = line.rstrip("\n").partition("\t")
                if k:
                    exp[k] = v
        bad = 0
        for log in sys.argv[3:]:
            want = {k: v for k, v in exp.items() if not call or k.startswith(CALL_TAGS)}
            got = extract(log, call)
            diff = [(k, want.get(k, "<absent>"), got.get(k, "<absent>"))
                    for k in sorted(set(want) | set(got)) if want.get(k) != got.get(k)]
            if diff:
                bad = 1
                print(f"MISMATCH {log}")
                for k, w, g in diff:
                    print(f"  {k}\n    T25  {w}\n    got  {g}")
            else:
                print(f"T25KEYS EQUAL {log} ({len(want)} fields)")
        return 3 if bad else 0
    print(__doc__)
    return 2


if __name__ == "__main__":
    sys.exit(main())
