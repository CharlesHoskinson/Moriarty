#!/usr/bin/env python3
"""PROPOSED E0r2 copy + identity tool (fresh-reviewer r2 plumbing; stdlib only; UNREVIEWED).

Successor of the reviewer's tools/copy_verify.py, for identity/copy plumbing only:
- every expected digest comes from the caller's authorization record via --expect and
  is compared with bytes; nothing is taken from a candidate flag or claimed field;
- BUILD-MANIFEST and Core-closure identities are RECOMPUTED from the planned bytes
  (and, with --copy, from the copied tree), and a mismatch is a STOP;
- the frozen reviewer packet (REVIEW-MANIFEST.json files) is copied unchanged to
  review/, and the r2 plumbing (R2-MANIFEST.json files) is added beside it under
  review/ without overwriting any frozen path.

Usage:
  python3 -I copy_verify_r2.py --dry-run CHECKOUT CANDIDATE PREV_REVIEW R2_DIR --expect K=SHA ...
  python3 -I copy_verify_r2.py --copy    CHECKOUT CANDIDATE PREV_REVIEW R2_DIR DEST --expect K=SHA ...
Required keys: source_manifest build_manifest core_closure review_manifest r2_manifest.
Exit 0 = PASS, 2 = usage, 3 = STOP. --dry-run writes nothing.
"""
import hashlib
import json
import os
import re
import resource
import shutil
import sys

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

KEYS = ("source_manifest", "build_manifest", "core_closure", "review_manifest", "r2_manifest")


def usage(msg):
    print(json.dumps({"verdict": "USAGE", "error": msg}))
    sys.exit(2)


args = sys.argv[1:]
if "--expect" not in args:
    usage("--expect K=SHA ... is required")
cut = args.index("--expect")
positional, pairs = args[:cut], args[cut + 1:]
if not positional or positional[0] not in ("--dry-run", "--copy"):
    usage("first argument must be --dry-run or --copy")
mode = positional[0]
need = 5 if mode == "--copy" else 4
if len(positional) - 1 != need:
    usage(f"{mode} takes {need} paths")
checkout, candidate, prev, r2 = positional[1:5]
dest = positional[5] if mode == "--copy" else None
expect = {}
for p in pairs:
    k, _, v = p.partition("=")
    if k not in KEYS or not re.fullmatch(r"[0-9a-f]{64}", v) or k in expect:
        usage(f"bad or repeated --expect {p!r}")
    expect[k] = v
if set(expect) != set(KEYS):
    usage(f"missing --expect keys {sorted(set(KEYS) - set(expect))}")


def sha(b):
    return hashlib.sha256(b).hexdigest()


def read(p):
    with open(p, "rb") as f:
        return f.read()


def compact(obj):
    return json.dumps(obj, sort_keys=True, separators=(",", ":")).encode()


stops = []
rec = {"format": "r2-e0-identity/1", "mode": mode, "expect": expect}

# 1. Manifests: digest of the actual bytes must equal the authorization record.
manifest_paths = {"source_manifest": os.path.join(candidate, "SOURCE-MANIFEST.json"),
                  "review_manifest": os.path.join(prev, "REVIEW-MANIFEST.json"),
                  "r2_manifest": os.path.join(r2, "R2-MANIFEST.json")}
manifests = {}
for k, p in manifest_paths.items():
    b = read(p)
    manifests[k] = json.loads(b)
    rec[k + "_sha256"] = sha(b)
    if sha(b) != expect[k]:
        stops.append(f"{k} digest {sha(b)} != expected {expect[k]}")

# 2. Plan (src_abs, dest_rel, expected_sha). The pin list is read only after its bytes
#    are verified against the (already digest-checked) SOURCE-MANIFEST entry.
sm = manifests["source_manifest"]
sm_index = {f["path"]: f["sha256"] for f in sm["files"]}
pins_path = os.path.join(candidate, "CORE-CLOSURE-PINS.json")
if sm_index.get("CORE-CLOSURE-PINS.json") != sha(read(pins_path)):
    stops.append("CORE-CLOSURE-PINS.json is not the SOURCE-MANIFEST-pinned file")
pins = json.loads(read(pins_path))
plan = []
for f in pins["files"]:
    plan.append((os.path.join(checkout, f["path"]), "moriarty/" + f["path"], f["sha256"]))
for f in sm["files"]:
    plan.append((os.path.join(candidate, f["path"]), "r1cs/" + f["path"], f["sha256"]))
plan.append((manifest_paths["source_manifest"], "r1cs/SOURCE-MANIFEST.json", expect["source_manifest"]))
for f in manifests["review_manifest"]["files"]:
    plan.append((os.path.join(prev, f["path"]), "review/" + f["path"], f["sha256"]))
frozen_review = {"review/" + f["path"] for f in manifests["review_manifest"]["files"]}
for f in manifests["r2_manifest"]["files"]:
    plan.append((os.path.join(r2, f["path"]), "review/" + f["path"], f["sha256"]))
plan.append((manifest_paths["r2_manifest"], "review/R2-MANIFEST.json", expect["r2_manifest"]))
dests = [d for _, d, _ in plan]
dupes = sorted({d for d in dests if dests.count(d) > 1})
if dupes:
    stops.append(f"destination collisions (r2 may not overwrite frozen paths): {dupes}")
if any(os.path.isabs(d) or ".." in d.split("/") for d in dests):
    stops.append("a planned destination is absolute or escapes the layout")

# 3. Every source byte equals its pin.
bad = []
for src, d, exp in plan:
    if not os.path.isfile(src) or os.path.islink(src):
        bad.append({"dest": d, "problem": "missing or not a regular file"})
    elif sha(read(src)) != exp:
        bad.append({"dest": d, "problem": "sha256 differs"})
if bad:
    stops.append(f"{len(bad)} source files differ from their pins")
rec["source_mismatches"] = bad
rec["counts"] = {top: sum(1 for d in dests if d.startswith(top + "/")) for top in ("moriarty", "r1cs", "review")}
rec["counts"]["review_frozen"] = len(frozen_review)
rec["counts"]["review_r2"] = rec["counts"]["review"] - len(frozen_review)

# 4. Copy (only with --copy and only if nothing has stopped so far).
if mode == "--copy" and not stops:
    if os.path.lexists(dest):
        stops.append("destination exists; refusing")
    else:
        for src, d, exp in plan:
            out = os.path.join(dest, d)
            os.makedirs(os.path.dirname(out), exist_ok=True)
            shutil.copyfile(src, out)
            if sha(read(out)) != exp:
                stops.append(f"post-copy mismatch {d}")
        rec["copied"] = True


def tree_files(top):
    """Planned relative files under top/ (dry-run) or the files actually present (copy)."""
    if mode == "--copy" and rec.get("copied"):
        base = os.path.join(dest, top)
        out = []
        for dp, dns, fns in os.walk(base):
            for fn in fns:
                p = os.path.join(dp, fn)
                if os.path.islink(p) or not os.path.isfile(p):
                    stops.append(f"not a regular file in copy: {p}")
                out.append(os.path.relpath(p, base).replace(os.sep, "/"))
        return sorted(out)
    return sorted(d[len(top) + 1:] for d in dests if d.startswith(top + "/"))


def tree_bytes(top, rel):
    if mode == "--copy" and rec.get("copied"):
        return read(os.path.join(dest, top, rel))
    src = {d: s for s, d, _ in plan}[f"{top}/{rel}"]
    return read(src)


# 5. Identity recomputation (from planned bytes, or from the copy).
if not bad:
    mfiles = tree_files("moriarty")
    rows = [{"path": p, "sha256": sha(tree_bytes("moriarty", p)), "bytes": len(tree_bytes("moriarty", p))} for p in mfiles]
    closure = sha(compact(rows))
    rec["core_closure_recomputed"] = closure
    rec["core_closure_files"] = len(mfiles)
    if closure != expect["core_closure"]:
        stops.append(f"recomputed Core closure {closure} != expected")
    if closure != pins.get("closure_sha256") or len(mfiles) != len(pins["files"]):
        stops.append("recomputed Core closure differs from CORE-CLOSURE-PINS.json (cross-check)")
    if mfiles != sorted(f["path"] for f in pins["files"]):
        stops.append("moriarty/ file set differs from the pin list")

    bm_bytes = tree_bytes("r1cs", "BUILD-MANIFEST.json")
    rec["build_manifest_recomputed"] = sha(bm_bytes)
    if sha(bm_bytes) != expect["build_manifest"]:
        stops.append(f"BUILD-MANIFEST digest {sha(bm_bytes)} != expected")
    if sm.get("build_manifest_sha256") != sha(bm_bytes):
        stops.append("SOURCE-MANIFEST build_manifest_sha256 differs from the BUILD-MANIFEST bytes (cross-check)")
    bm = json.loads(bm_bytes)
    listed = []
    for e in bm["files"]:
        b = tree_bytes("r1cs", e["path"])
        if sha(b) != e["sha256"] or len(b) != e["bytes"]:
            stops.append(f"BUILD-MANIFEST entry {e['path']} differs from the bytes")
        listed.append(e["path"])
    r1 = tree_files("r1cs")
    rule = sorted([p for p in r1 if p.startswith("source/mori-r1cs/src/")] + ["source/mori-r1cs/Cargo.toml"])
    rec["build_manifest_entries"] = len(listed)
    if sorted(listed) != rule:
        stops.append("BUILD-MANIFEST entry set differs from Cargo.toml + src/** of the copied crate")

# 6. Informational currentness (bytes above are the STOP criteria).
def git_head(path):
    try:
        g = os.path.join(path, ".git")
        gd = open(g).read().split("gitdir:", 1)[1].strip() if os.path.isfile(g) else g
        head = open(os.path.join(gd, "HEAD")).read().strip()
        if not head.startswith("ref:"):
            return head
        ref = head.split(" ", 1)[1]
        for base in (gd, os.path.join(gd, open(os.path.join(gd, "commondir")).read().strip()) if os.path.exists(os.path.join(gd, "commondir")) else gd):
            if os.path.exists(os.path.join(base, ref)):
                return open(os.path.join(base, ref)).read().strip()
    except OSError:
        return None
    return None


rec["checkout_head_informational"] = git_head(checkout)
rec["pins_head"] = pins.get("head")

# 7. Hygiene of the copy.
if mode == "--copy" and rec.get("copied"):
    cfg = [os.path.join(dp, d) for dp, dns, _ in os.walk(dest) for d in dns if d == ".cargo"]
    if cfg:
        stops.append(f".cargo directories inside the layout: {cfg}")
    rec["rust_toolchain_files_in_layout"] = sorted(os.path.relpath(os.path.join(dp, f), dest)
                                                  for dp, _, fns in os.walk(dest) for f in fns if f.startswith("rust-toolchain"))

rec["stops"] = stops
rec["verdict"] = "PASS" if not stops else "STOP"
print(json.dumps(rec, indent=1, sort_keys=True))
sys.exit(0 if not stops else 3)
