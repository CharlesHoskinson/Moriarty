#!/usr/bin/env python3
"""OPX0 / OPX7: read-only guarded manifest and pin checks for the four-piece VS measurement (O'').
NOT RUN BY ITS AUTHOR.

Usage: python3 -I preflight.py pre|post OWN PRELAUNCH_JSON M T PT R1T

Reads only; writes nothing anywhere. Prints one JSON summary on stdout. Exit 0 only if every
check holds; 1 otherwise; 2 on usage.

  pre   T must be absent (OPX0).   post   T must be a directory (OPX7).

Checks (all pins are literals in this file or in OWN/data, which the operator manifest pins):
  P1 pinstore: PRELAUNCH_JSON (root-written) names this T with targetAbsent true, the operator
     manifest OWN/ROOT-SOURCE-MANIFEST.json by bytes and sha256, the candidate manifest
     M/ROOT-SOURCE-MANIFEST.json by bytes and sha256 equal to CANDIDATE_MANIFEST below, and the
     15 step files in LAUNCH_ORDER by path and sha256. No hash placeholder is accepted: every
     value must be 64 lowercase hex and equal the file actually present.
  P2 operator manifest: root == OWN; its rows are exactly OWN_FILES; every row's bytes and
     sha256 match OWN/<path> and its "original" (SRC/<path>, the author source); the walk of OWN
     finds exactly OWN_FILES plus ROOT-SOURCE-MANIFEST.json and nothing else.
  P3 candidate manifest: 41 rows, each matching M/<path> and its "original"; the 32 package
     rows equal data/candidate-32.sha256 and the 9 document rows equal data/m-docs-10.sha256.
  P4 candidate tree M/candidate/aiken: exactly the 32 files, sub-directories exactly lib,
     lib/pvc2 and validators (4 directories counting the root), no other entry; validators exactly
     c2, s2p, vi, vs1, vs2, vs3, vs4 (vs.ak and s2.ak absent); no build, aiken.lock, plutus.json.
  P5 expectations: M/EXPECTATIONS.json is 80cf1601...; its 125 + 22 names and fail flags equal the
     test declarations parsed from the placed-to-be shard4_test.ak and shard4_cost_test.ak; only
     modules pvc2/shard4_test and pvc2/shard4_cost_test contain the two --match-tests strings.
     A disagreement is an EXPECTATIONS transcription error (its own authority clause), never a
     candidate result: STOP before T exists.
  P6 parent evidence (attempt 2, raw, not adopted) and the R1 label control, by pinned digest.
"""
import hashlib
import json
import os
import re
import stat
import sys

SRC = "/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-four-way-vs-harness-repair-operator-muse-render-plan-r2-20261016/rendered-candidate"
LAUNCH_ORDER = ["OPX0", "OPX1", "OPX1t", "OPX1p", "OPX2a", "OPX3", "OPX3p", "OPX4", "OPX5", "OPX6",
                "OPX2", "OPX2p", "OPX2x", "OPX7", "OPX7f"]
OWN_FILES = (["steps/%s.sh" % s for s in LAUNCH_ORDER] + [
    "tools/preflight.py", "tools/freeze.py", "tools/tree_check.py", "tools/test_compare4.py",
    "tools/hash_chain4.py", "tools/label_search4.py", "tools/size_report4.py", "tools/evaluate4.py",
    "data/candidate-32.sha256", "data/m-docs-10.sha256", "data/stdlib-38.sha256",
    "data/parent-evidence-9.sha256",
    "REPORT.md", "SOURCE-CHANGE-MAP.json", "ROOT-PLACEMENT-INSTRUCTIONS.json",
    "EXECUTION-PROPOSAL.json", "READ-COVERAGE.json", "DESIGN-POSITIONS.json",
    "OWN-OUTPUT-ALLOWLIST.json",
])
ROOT_OWNED_IN_OWN = ["ROOT-SOURCE-MANIFEST.json"]
PRELAUNCH_FORMAT = "root-four-way-vs-harness-repair-prelaunch/1"
OPERATOR_MANIFEST_FORMAT = "root-four-way-vs-harness-repair-materialization/1"
CANDIDATE_MANIFEST = {"bytes": 14506, "sha256": "54e0fc5629068371d2e1c2ff77945abe4394c8cd2632dbfcf5be0ac2dde07a47"}
CANDIDATE_ROWS = 41
EXPECTATIONS_SHA256 = "80cf1601480be0c415649ac1b071326b0097f89c2132279b3636a38c15df8393"
R1_APPLIED_SHA256 = "aa6d462782defd28c6cd5ec98a5f5687678159ab0449bba626fd2f87f338cc77"
TEST_MODULES = {"pvc2/shard4_test": "lib/pvc2/shard4_test.ak", "pvc2/shard4_cost_test": "lib/pvc2/shard4_cost_test.ak"}
MATCH_STRINGS = ["pvc2/shard4_test", "pvc2/shard4_cost_test"]
SUBDIRS = ["lib", "lib/pvc2", "validators"]
VALIDATORS = ["c2.ak", "s2p.ak", "vi.ak", "vs1.ak", "vs2.ak", "vs3.ak", "vs4.ak"]
HEX64 = re.compile(r"[0-9a-f]{64}")
TEST_DECL = re.compile(r"^test ([a-z_][a-z0-9_]*)\(\)( fail)? \{$")


def sha256_of(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def regular(path):
    try:
        return stat.S_ISREG(os.lstat(path).st_mode)
    except OSError:
        return False


def read_sums(path):
    out = {}
    with open(path, encoding="ascii") as f:
        for line in f.read().splitlines():
            h, sep, p = line.partition("  ")
            if not sep or not HEX64.fullmatch(h) or p in out:
                raise ValueError("bad checksum line in %s: %r" % (path, line))
            out[p] = h
    return out


def walk(base):
    files, dirs, other = [], [], []
    for d, ds, ns in os.walk(base, followlinks=False):
        for n in ds:
            p = os.path.join(d, n)
            (dirs if stat.S_ISDIR(os.lstat(p).st_mode) else other).append(os.path.relpath(p, base))
        for n in ns:
            p = os.path.join(d, n)
            (files if stat.S_ISREG(os.lstat(p).st_mode) else other).append(os.path.relpath(p, base))
    return sorted(files), sorted(dirs), sorted(other)


class Checker:
    def __init__(self):
        self.problems = []
        self.notes = []

    def need(self, cond, msg):
        if not cond:
            self.problems.append(msg)
        return cond

    def file_is(self, path, sha, size=None, label=None):
        label = label or path
        if not self.need(regular(path), "%s: missing or not a regular file" % label):
            return False
        ok = self.need(sha256_of(path) == sha, "%s: sha256 differs" % label)
        if size is not None:
            ok = self.need(os.path.getsize(path) == size, "%s: byte count differs" % label) and ok
        return ok


def load_json(c, path, label):
    try:
        with open(path, "rb") as f:
            return json.loads(f.read())
    except (OSError, ValueError) as e:
        c.need(False, "%s unreadable: %s" % (label, e))
        return None


def check_pinstore(c, pre, own, m, t):
    if pre is None:
        return
    c.need(pre.get("format") == PRELAUNCH_FORMAT, "P1 PRE-LAUNCH format differs")
    c.need(pre.get("target") == t, "P1 PRE-LAUNCH target differs from T")
    c.need(pre.get("targetAbsent") is True, "P1 PRE-LAUNCH targetAbsent is not true")
    om = pre.get("operatorManifest") or {}
    opath = os.path.join(own, "ROOT-SOURCE-MANIFEST.json")
    c.need(om.get("path") == opath, "P1 operatorManifest.path differs")
    if c.need(isinstance(om.get("sha256"), str) and HEX64.fullmatch(om["sha256"]) is not None,
              "P1 operatorManifest.sha256 is not 64 lowercase hex (placeholder refused)"):
        c.file_is(opath, om["sha256"], om.get("bytes") if isinstance(om.get("bytes"), int) else -1,
                  "P1 operator manifest")
    cm = pre.get("candidateManifest") or {}
    c.need(cm.get("path") == os.path.join(m, "ROOT-SOURCE-MANIFEST.json"), "P1 candidateManifest.path differs")
    c.need(cm.get("sha256") == CANDIDATE_MANIFEST["sha256"] and cm.get("bytes") == CANDIDATE_MANIFEST["bytes"],
           "P1 candidateManifest pin differs from the freeze pin")
    steps = pre.get("steps")
    if c.need(isinstance(steps, list) and [s.get("id") for s in steps] == LAUNCH_ORDER,
              "P1 PRE-LAUNCH steps are not exactly the launch order"):
        for s in steps:
            p = os.path.join(own, "steps", s["id"] + ".sh")
            c.need(s.get("path") == p, "P1 step path differs: %s" % s["id"])
            if c.need(isinstance(s.get("sha256"), str) and HEX64.fullmatch(s["sha256"]) is not None,
                      "P1 step sha256 is not 64 lowercase hex: %s" % s["id"]):
                c.file_is(p, s["sha256"], None, "P1 step " + s["id"])


def check_operator_manifest(c, own, man):
    if man is None:
        return
    c.need(man.get("format") == OPERATOR_MANIFEST_FORMAT, "P2 operator manifest format differs")
    c.need(man.get("root") == own, "P2 operator manifest root differs from OWN")
    rows = man.get("files") or []
    paths = [r.get("path") for r in rows]
    c.need(sorted(paths) == sorted(OWN_FILES) and len(paths) == len(set(paths)),
           "P2 operator manifest rows are not exactly the allowlisted files")
    for r in rows:
        p = r.get("path")
        if p not in OWN_FILES:
            continue
        sha, size = r.get("sha256"), r.get("bytes")
        if not c.need(isinstance(sha, str) and HEX64.fullmatch(sha) is not None and isinstance(size, int),
                      "P2 row without a literal digest: %s" % p):
            continue
        c.file_is(os.path.join(own, p), sha, size, "P2 OWN/" + p)
        c.need(r.get("original") == os.path.join(SRC, p), "P2 original path differs: %s" % p)
        c.file_is(os.path.join(SRC, p), sha, size, "P2 SRC/" + p)
    files, _, other = walk(own)
    c.need(files == sorted(OWN_FILES + ROOT_OWNED_IN_OWN), "P2 OWN holds files outside the allowlist + root manifest")
    c.need(other == [], "P2 OWN holds non-regular entries")
    allow = load_json(c, os.path.join(own, "OWN-OUTPUT-ALLOWLIST.json"), "P2 allowlist")
    if allow is not None:
        c.need(sorted(allow.get("files") or []) == sorted(OWN_FILES), "P2 OWN-OUTPUT-ALLOWLIST differs from OWN_FILES")


def check_candidate_manifest(c, own, m):
    mpath = os.path.join(m, "ROOT-SOURCE-MANIFEST.json")
    if not c.file_is(mpath, CANDIDATE_MANIFEST["sha256"], CANDIDATE_MANIFEST["bytes"], "P3 candidate manifest"):
        return None
    man = load_json(c, mpath, "P3 candidate manifest")
    if man is None:
        return None
    c.need(man.get("root") == m, "P3 candidate manifest root differs from M")
    rows = man.get("files") or []
    c.need(len(rows) == CANDIDATE_ROWS, "P3 candidate manifest row count differs")
    for r in rows:
        c.file_is(os.path.join(m, r.get("path", "")), r.get("sha256"), r.get("bytes"), "P3 M/%s" % r.get("path"))
        c.file_is(r.get("original", ""), r.get("sha256"), r.get("bytes"), "P3 original of %s" % r.get("path"))
    cand = read_sums(os.path.join(own, "data/candidate-32.sha256"))
    docs = read_sums(os.path.join(own, "data/m-docs-10.sha256"))
    c.need(docs.get("ROOT-SOURCE-MANIFEST.json") == CANDIDATE_MANIFEST["sha256"], "P3 m-docs-10 manifest row differs")
    got_c = {r["path"][len("candidate/aiken/"):]: r["sha256"] for r in rows if r.get("path", "").startswith("candidate/aiken/")}
    got_d = {r["path"]: r["sha256"] for r in rows if not r.get("path", "").startswith("candidate/aiken/")}
    c.need(got_c == cand and len(cand) == 32, "P3 package rows differ from data/candidate-32.sha256")
    want_d = {k: v for k, v in docs.items() if k != "ROOT-SOURCE-MANIFEST.json"}
    c.need(got_d == want_d and len(want_d) == 9, "P3 document rows differ from data/m-docs-10.sha256")
    return cand


def check_candidate_tree(c, m, cand):
    base = os.path.join(m, "candidate", "aiken")
    files, dirs, other = walk(base)
    c.need(cand is not None and files == sorted(cand), "P4 candidate/aiken files are not exactly the 32 rows")
    c.need(dirs == SUBDIRS, "P4 candidate/aiken sub-directories are not exactly lib, lib/pvc2, validators")
    c.need(len(dirs) + 1 == 4, "P4 directory count including the root is not 4")
    c.need(other == [], "P4 candidate/aiken holds non-regular entries")
    try:
        c.need(sorted(os.listdir(os.path.join(base, "validators"))) == VALIDATORS, "P4 validators set differs")
    except OSError as e:
        c.need(False, "P4 validators unreadable: %s" % e)
    for n in ["validators/vs.ak", "validators/s2.ak", "build", "aiken.lock", "plutus.json"]:
        c.need(not os.path.lexists(os.path.join(base, n)), "P4 present but must be absent: %s" % n)


def parse_tests(c, path, label):
    out = []
    with open(path, encoding="utf-8") as f:
        for i, line in enumerate(f.read().split("\n"), 1):
            if line.startswith("test"):
                mt = TEST_DECL.match(line)
                if c.need(mt is not None, "P5 unparsed test-like line %s:%d" % (label, i)):
                    out.append((mt.group(1), mt.group(2) is not None))
    return out


def check_expectations(c, m):
    epath = os.path.join(m, "EXPECTATIONS.json")
    if not c.file_is(epath, EXPECTATIONS_SHA256, None, "P5 EXPECTATIONS.json"):
        return
    e = load_json(c, epath, "P5 EXPECTATIONS.json")
    if e is None:
        return
    tm, cm = e.get("shard4TestModule") or {}, e.get("shard4CostModule") or {}
    exp = {tm.get("module"): [(t["name"], bool(t["fail"])) for t in tm.get("tests", [])],
           cm.get("module"): [(n, False) for n in cm.get("names", [])]}
    c.need(sorted(exp) == sorted(TEST_MODULES), "P5 EXPECTATIONS module names differ")
    c.need(len(exp.get("pvc2/shard4_test", [])) == tm.get("count") == 125, "P5 shard4_test count differs")
    c.need(sum(f for _, f in exp.get("pvc2/shard4_test", [])) == tm.get("failCount") == 9, "P5 shard4_test fail count differs")
    c.need(len(exp.get("pvc2/shard4_cost_test", [])) == cm.get("count") == 22 and cm.get("failCount") == 0,
           "P5 shard4_cost_test count differs")
    per = e.get("perModule") or {}
    c.need(per.get("pvc2/shard4_test") == [125, 9] and per.get("pvc2/shard4_cost_test") == [22, 0],
           "P5 perModule differs")
    c.need((e.get("totals") or {}).get("new") == {"tests": 147, "failTests": 9}, "P5 totals.new differs")
    names = [n for v in exp.values() for n, _ in v]
    c.need(len(names) == len(set(names)) == 147, "P5 expected names are not 147 unique")
    base = os.path.join(m, "candidate", "aiken")
    for mod, rel in TEST_MODULES.items():
        got = parse_tests(c, os.path.join(base, rel), rel)
        c.need(sorted(got) == sorted(exp.get(mod, [])), "P5 %s declarations differ from EXPECTATIONS names/fail flags" % mod)
        if got != exp.get(mod, []):
            c.notes.append("P5 %s declaration order differs from EXPECTATIONS order (membership equal: %s)"
                           % (mod, sorted(got) == sorted(exp.get(mod, []))))
    mods = []
    for d, _, ns in os.walk(os.path.join(base, "lib")):
        for n in ns:
            if n.endswith(".ak"):
                mods.append(os.path.relpath(os.path.join(d, n), os.path.join(base, "lib"))[:-3])
    mods += [n[:-3] for n in VALIDATORS]
    hit = sorted(md for md in mods if any(s in md for s in MATCH_STRINGS))
    c.need(hit == sorted(MATCH_STRINGS), "P5 --match-tests strings select modules other than the two test modules: %s" % hit)
    c.notes.append("P5 module names in package: %d; filter-selected: %s" % (len(mods), hit))


def check_parent(c, own, pt, r1t):
    sums = read_sums(os.path.join(own, "data/parent-evidence-9.sha256"))
    c.need(len(sums) == 9, "P6 parent evidence list is not 9 rows")
    for rel, h in sums.items():
        c.file_is(os.path.join(pt, rel), h, None, "P6 PT/" + rel)
    c.file_is(os.path.join(r1t, "a2", "plutus.applied.json"), R1_APPLIED_SHA256, None, "P6 R1 label control")


def main():
    if len(sys.argv) != 8 or sys.argv[1] not in ("pre", "post"):
        sys.stderr.write("usage: preflight.py pre|post OWN PRELAUNCH_JSON M T PT R1T\n")
        sys.exit(2)
    mode, own, prelaunch, m, t, pt, r1t = sys.argv[1:8]
    c = Checker()
    if mode == "pre":
        c.need(not os.path.lexists(t), "P0 T exists (pre mode requires it absent)")
    else:
        c.need(os.path.isdir(t) and not os.path.islink(t), "P0 T is not a directory (post mode)")
    check_pinstore(c, load_json(c, prelaunch, "P1 PRE-LAUNCH"), own, m, t)
    check_operator_manifest(c, own, load_json(c, os.path.join(own, "ROOT-SOURCE-MANIFEST.json"), "P2 operator manifest"))
    cand = check_candidate_manifest(c, own, m)
    check_candidate_tree(c, m, cand)
    check_expectations(c, m)
    check_parent(c, own, pt, r1t)
    doc = {"format": "four-way-vs-operator/preflight/1", "mode": mode, "ok": not c.problems,
           "operatorManifestSha256": sha256_of(os.path.join(own, "ROOT-SOURCE-MANIFEST.json"))
           if regular(os.path.join(own, "ROOT-SOURCE-MANIFEST.json")) else None,
           "problems": c.problems, "notes": c.notes}
    print(json.dumps(doc, indent=1))
    sys.exit(0 if not c.problems else 1)


if __name__ == "__main__":
    main()
