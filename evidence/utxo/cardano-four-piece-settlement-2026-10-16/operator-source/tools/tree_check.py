#!/usr/bin/env python3
"""Run-tree hygiene for the four-piece VS measurement (O''). NOT RUN BY ITS AUTHOR.

Usage: python3 -I tree_check.py RUN_DIR DATA_DIR OUT_JSON STAGE [--forbid-generated]
           [--require-lock] [--plutus SHA_RECEIPT A2_COPY]

Reads RUN_DIR (never follows links) and the two pinned lists in DATA_DIR; writes OUT_JSON with
mode "x". Exit 0 when every invariant holds, 1 otherwise, 2 on usage.

Always enforced (category "hygiene" unless marked "fetch"):
  * the 71 placed files are present with their pinned sha256: 32 package files
    (candidate-32.sha256), 38 stdlib files under build/packages/aiken-lang-stdlib
    (stdlib-38.sha256) and the 77-byte build/packages/packages.toml;
  * any other regular file is one of aiken.lock, build/aiken-compile.lock, plutus.json;
  * no symlink, fifo, socket or device anywhere;
  * the 19 placed directories exist; build/packages holds exactly aiken-lang-stdlib and
    packages.toml ("fetch");
  * if aiken.lock exists: exactly one [[packages]] entry, aiken-lang/stdlib v2.2.0, and no
    "aiken-lang/stdlib@" etag line ("fetch").
Stage flags:
  --forbid-generated   none of the three generated files may exist (placement);
  --require-lock       aiken.lock and build/aiken-compile.lock must exist (a type check ran clean);
  --plutus R C         plutus.json must exist, equal the sha in receipt R and be byte-equal to C.
Extra directories are recorded, not gated (no parent observation of aiken's directory set).
A missing lock after a failed type check is therefore not flagged (FINDINGS PSR-09 lesson).
"""
import hashlib
import json
import os
import stat
import sys

PACKAGES_TOML_SHA256 = "42ca23893e5c0f43598c15a1c699fa2bcbb451999c054351f67dae174d49ffb6"
PARENT_LOCK_SHA256 = "ec5d6a58b5c5b7786e8e0b3b7fbc76736aea5739bcc6e6e9633cffa87130da83"
GENERATED = ["aiken.lock", "build/aiken-compile.lock", "plutus.json"]
STDLIB = "build/packages/aiken-lang-stdlib/"
STDLIB_DIRS = [".github", ".github/workflows", "lib", "lib/aiken", "lib/aiken/collection",
               "lib/aiken/crypto", "lib/aiken/crypto/bls12_381", "lib/aiken/math",
               "lib/aiken/primitive", "lib/cardano", "lib/cardano/address",
               "lib/cardano/governance", "lib/cardano/transaction"]
PLACED_DIRS = (["lib", "lib/pvc2", "validators", "build", "build/packages", "build/packages/aiken-lang-stdlib"]
               + [STDLIB + d for d in STDLIB_DIRS])


def sha256_of(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def read_sums(path, prefix=""):
    out = {}
    with open(path, encoding="ascii") as f:
        for line in f.read().splitlines():
            h, sep, p = line.partition("  ")
            if not sep or len(h) != 64:
                raise ValueError("bad checksum line: %r" % line)
            out[prefix + p] = h
    return out


def main():
    a = sys.argv[1:]
    if len(a) < 4:
        sys.stderr.write(__doc__)
        sys.exit(2)
    run, data, out, stage = a[:4]
    flags = a[4:]
    forbid = "--forbid-generated" in flags
    need_lock = "--require-lock" in flags
    plutus = None
    if "--plutus" in flags:
        i = flags.index("--plutus")
        if len(flags) < i + 3:
            sys.stderr.write("--plutus needs SHA_RECEIPT A2_COPY\n")
            sys.exit(2)
        plutus = (flags[i + 1], flags[i + 2])
    known = {"--forbid-generated", "--require-lock", "--plutus"}
    rest = [f for f in flags if f.startswith("--")]
    if any(f not in known for f in rest):
        sys.stderr.write("unknown flag\n")
        sys.exit(2)

    expected = read_sums(os.path.join(data, "candidate-32.sha256"))
    expected.update(read_sums(os.path.join(data, "stdlib-38.sha256"), STDLIB))
    expected["build/packages/packages.toml"] = PACKAGES_TOML_SHA256
    problems = []

    def bad(cat, msg):
        problems.append({"category": cat, "problem": msg})

    files, dirs, other = {}, [], []
    for d, ds, ns in os.walk(run, followlinks=False):
        for n in ds:
            p = os.path.join(d, n)
            (dirs if stat.S_ISDIR(os.lstat(p).st_mode) else other).append(os.path.relpath(p, run))
        for n in ns:
            p = os.path.join(d, n)
            st = os.lstat(p)
            rel = os.path.relpath(p, run)
            if stat.S_ISREG(st.st_mode):
                files[rel] = {"bytes": st.st_size, "sha256": sha256_of(p)}
            else:
                other.append(rel)
    if len(expected) != 71:
        bad("hygiene", "pinned placement list is not 71 rows")
    for rel, h in sorted(expected.items()):
        if rel not in files:
            bad("hygiene", "placed file missing: " + rel)
        elif files[rel]["sha256"] != h:
            bad("hygiene", "placed file changed: " + rel)
    extra = sorted(set(files) - set(expected))
    for rel in extra:
        if rel not in GENERATED:
            bad("fetch" if rel.startswith("build/packages/") else "hygiene", "unexpected file: " + rel)
    for rel in other:
        bad("hygiene", "non-regular entry: " + rel)
    for d in PLACED_DIRS:
        if d not in dirs:
            bad("hygiene", "placed directory missing: " + d)
    extra_dirs = sorted(set(dirs) - set(PLACED_DIRS))
    for d in extra_dirs:
        if d.startswith("build/packages/") and not d.startswith(STDLIB):
            bad("fetch", "extra package directory: " + d)
        elif d.startswith(STDLIB):
            bad("fetch", "extra directory inside the pinned stdlib: " + d)
    try:
        top = sorted(os.listdir(os.path.join(run, "build", "packages")))
    except OSError:
        top = None
    if top != ["aiken-lang-stdlib", "packages.toml"]:
        bad("fetch", "build/packages entries are not exactly aiken-lang-stdlib and packages.toml: %s" % top)

    present = [g for g in GENERATED if g in files]
    if forbid and present:
        bad("hygiene", "generated files present at placement: %s" % present)
    if need_lock:
        for g in ("aiken.lock", "build/aiken-compile.lock"):
            if g not in files:
                bad("hygiene", "required after a clean type check but absent: " + g)
    lock = None
    if "aiken.lock" in files:
        with open(os.path.join(run, "aiken.lock"), encoding="utf-8", errors="replace") as f:
            text = f.read()
        lines = text.split("\n")
        lock = {
            "sha256": files["aiken.lock"]["sha256"],
            "equalsParentAttempt2Lock": files["aiken.lock"]["sha256"] == PARENT_LOCK_SHA256,
            "packagesEntries": sum(1 for l in lines if l.strip() == "[[packages]]"),
        }
        if lock["packagesEntries"] != 1:
            bad("fetch", "aiken.lock does not hold exactly one [[packages]] entry")
        if 'name = "aiken-lang/stdlib"' not in text or 'version = "v2.2.0"' not in text:
            bad("fetch", "aiken.lock does not name aiken-lang/stdlib v2.2.0")
        if "aiken-lang/stdlib@" in text:
            bad("fetch", "aiken.lock carries an etag line")
    if plutus is not None:
        if "plutus.json" not in files:
            bad("hygiene", "plutus.json required but absent")
        else:
            with open(plutus[0], encoding="ascii") as f:
                want = f.read().split()[0]
            if files["plutus.json"]["sha256"] != want:
                bad("hygiene", "plutus.json differs from its recorded sha256")
            with open(os.path.join(run, "plutus.json"), "rb") as f1, open(plutus[1], "rb") as f2:
                if f1.read() != f2.read():
                    bad("hygiene", "plutus.json differs from the read-only a2 copy")

    doc = {
        "format": "four-way-vs-operator/tree-check/1",
        "stage": stage,
        "ok": not problems,
        "problems": problems,
        "categories": sorted({p["category"] for p in problems}),
        "fileCount": len(files),
        "directoryCountBelowRoot": len(dirs),
        "directoryCountWithRoot": len(dirs) + 1,
        "generatedPresent": present,
        "extraDirectories": extra_dirs,
        "aikenLock": lock,
        "flags": {"forbidGenerated": forbid, "requireLock": need_lock, "plutus": plutus is not None},
        "files": [dict(path=k, **v) for k, v in sorted(files.items())],
    }
    with open(out, "x", encoding="utf-8") as f:
        json.dump(doc, f, indent=1)
        f.write("\n")
    print("tree %s: %d files, %d dirs below root, ok=%s %s" % (stage, len(files), len(dirs), not problems,
                                                              [p["problem"] for p in problems]))
    sys.exit(0 if not problems else 1)


if __name__ == "__main__":
    main()
