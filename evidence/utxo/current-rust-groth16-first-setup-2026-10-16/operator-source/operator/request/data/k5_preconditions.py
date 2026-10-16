"""K5 root-precondition check for steps P1 and P0 (clause K5-1). PROPOSED SOURCE; never executed by its author.

Run only as:
  /usr/bin/python3 -I k5_preconditions.py p1|p0 <RR5> <OWN> <TK> <T5> <K4H expected-KH3-10-names.txt>

It checks the root records whose exact schema ROOT-PLACEMENT-INSTRUCTIONS.json defines: RR5/K5-OPERATOR-PINS.sha256,
RR5/K5-MATERIALIZATION-PIN.sha256 with the manifest it names, RR5/K5-PRECONDITIONS.json, RR5/K5-AUTHORIZATION.json
and RR5/K5-PRE-LAUNCH.json. The manifest it names covers exactly the 26 candidate files and has no entry for
itself or for any RR5 record; its final bytes are bound only by RR5/K5-MATERIALIZATION-PIN.sha256. It also checks
the files they name and these public K4H outputs:
TK/receipts/{KH-binaries-pre.sha256, KH-binaries-post.sha256, KH6.json, KH3-ok-names.txt, stage-K4H-final.json},
TK/logs/{KH3,KH5,KH6}.exit and TK/ZH2.exit. It reads nothing under TK/w. Every file must lie under BASE with
no link in its path, be a regular non-link file of at most 1 MiB, and is opened with O_NOFOLLOW. A missing,
malformed, placeholder or mismatched value is refused; nothing is defaulted.

On PASS it prints one "<sha256>  <absolute path>" line per root record, K4H receipt and review file it verified
(P0 keeps them as receipts/P0-root-pins.sha256; every later step re-verifies them) and exits 0. Otherwise it
prints every problem to stderr and exits 1, which is BLOCKED(prerequisite) under clause K5-5. Exit 2 is a usage
or interpreter-level refusal. It writes no file.
"""
import hashlib
import json
import os
import re
import stat
import sys

BASE = "/home/charl/moriarty-eutxo-validation-2026-10-12"
M5 = BASE + "/current-cardano-k2-k5-setup-operator-materialized-20261016"
FIXED = {
    "RR5": BASE + "/current-cardano-k2-k5-root-records-20261016",
    "OWN": M5 + "/operator/request",
    "TK": BASE + "/current-cardano-k2-k4h-hostile-local-execution-20261016",
    "T5": BASE + "/current-cardano-k2-k5-single-party-test-setup-20261016",
    "names": BASE + "/current-cardano-k2-k4h-hostile-materialized-20261016/operator/request/data/expected-KH3-10-names.txt",
}
MAX = 1 << 20
STEPS = ["P1", "P0", "Q5", "K5a", "K5b", "K5M", "K5g", "H5", "Z51", "Z52"]
CLAUSES = ["K5-1", "K5-2", "K5-4M", "K5-5", "K5-7", "K5-8", "K5-8A"]
DV_PREFIX = "b04371a2"
K4H_RECEIPTS = ["KH-binaries-pre.sha256", "KH-binaries-post.sha256", "KH6.json", "KH3-ok-names.txt",
                "stage-K4H-final.json"]
K4H_EXITS = ["logs/KH3.exit", "logs/KH5.exit", "logs/KH6.exit", "ZH2.exit"]
MANIFEST_PATHS = ["DESIGN-POSITIONS.json", "EXECUTION-PROPOSAL.json", "EXPECTATIONS.json",
                  "OWN-OUTPUT-ALLOWLIST.json", "READ-COVERAGE.json", "REPORT.md",
                  "ROOT-PLACEMENT-INSTRUCTIONS.json", "SOURCE-CHANGE-MAP.json",
                  "operator/request/OPERATOR-STEPS.md", "operator/request/data/expected-r1cs.txt",
                  "operator/request/data/k5-abs.sha256", "operator/request/data/k5-in-4.sha256",
                  "operator/request/data/k5-operator-files.paths", "operator/request/data/k5_preconditions.py",
                  "operator/request/data/k5g_gate.py", "operator/request/data/probe-k5.sh",
                  "operator/request/steps/H5.sh", "operator/request/steps/K5M.sh",
                  "operator/request/steps/K5a.sh", "operator/request/steps/K5b.sh",
                  "operator/request/steps/K5g.sh", "operator/request/steps/P0.sh",
                  "operator/request/steps/P1.sh", "operator/request/steps/Q5.sh",
                  "operator/request/steps/Z51.sh", "operator/request/steps/Z52.sh"]
PRE_KEYS = ["format", "k4hTarget", "k4hActualResultReview", "k4hReceipts", "releaseMoriK2Sha256", "k5SourceReview",
            "k5PositionTally", "dvK21R"]
REVIEW_KEYS = ["reportPath", "reportSha256", "closed", "kh3OkCount", "kh6Verdict", "kh5BinariesUnchanged",
               "kh3FinishedWithoutCompiling", "relationSecurityFinding", "k4hTreeReadForK5Permitted"]
REVIEW_VALUES = [("closed", True), ("kh3OkCount", 10), ("kh6Verdict", "PASS"), ("kh5BinariesUnchanged", True),
                 ("kh3FinishedWithoutCompiling", True), ("relationSecurityFinding", False),
                 ("k4hTreeReadForK5Permitted", True)]
AUTH_VALUES = {
    "format": "root-K5-single-party-test-setup-authorization/1",
    "target": FIXED["T5"],
    "steps": STEPS,
    "attempts": 1,
    "retries": 0,
    "relation": "invoice/partial",
    "setupInvocations": 1,
    "trustProfile": "single-party-test-setup/1",
    "custodyClause": "DV-K2-1-R",
    "proveVerifyPairing": False,
    "publishKeyMaterial": False,
    "networkWalletLedger": False,
}
HEX64 = re.compile(r"[0-9a-f]{64}")
REL_PATH = re.compile(r"[A-Za-z0-9_][A-Za-z0-9_.-]*(/[A-Za-z0-9_][A-Za-z0-9_.-]*)*")


class Refused(Exception):
    pass


def refuse_duplicates(pairs):
    out = {}
    for k, v in pairs:
        if k in out:
            raise Refused("duplicate key %r" % k)
        out[k] = v
    return out


def no_constant(name):
    raise Refused("non-finite number %s" % name)


def no_float(text):
    raise Refused("non-integer number %s" % text)


def strict_object(data, what):
    value = json.loads(data.decode("utf-8"), object_pairs_hook=refuse_duplicates, parse_constant=no_constant,
                       parse_float=no_float)
    if not isinstance(value, dict):
        raise Refused("%s is not a JSON object" % what)
    return value


def canonical(value):
    return json.dumps(value, sort_keys=False, separators=(",", ":"))


def exact_keys(obj, keys, what):
    if not isinstance(obj, dict) or sorted(obj) != sorted(keys):
        got = sorted(obj) if isinstance(obj, dict) else type(obj).__name__
        raise Refused("%s has keys %r; expected exactly %r" % (what, got, sorted(keys)))


def hex64(value, what):
    if not (isinstance(value, str) and HEX64.fullmatch(value)) or len(set(value)) == 1:
        raise Refused("%s is not a 64-hex SHA-256 (placeholders refused)" % what)
    return value


def no_link_path(path):
    if os.path.normpath(path) != path or not path.startswith(BASE + "/"):
        raise Refused("%s is not a normalized absolute path under %s" % (path, BASE))
    cur = BASE
    for part in [""] + path[len(BASE) + 1:].split("/")[:-1]:
        cur = cur + "/" + part if part else cur
        st = os.lstat(cur)
        if stat.S_ISLNK(st.st_mode) or not stat.S_ISDIR(st.st_mode):
            raise Refused("%s is a link or not a directory" % cur)


def read(path, pins):
    """Bytes of a regular non-link file of at most 1 MiB; records its SHA-256 in `pins`."""
    no_link_path(path)
    st = os.lstat(path)
    if not stat.S_ISREG(st.st_mode):
        raise Refused("%s is not a regular non-link file" % path)
    if st.st_size > MAX:
        raise Refused("%s exceeds 1 MiB" % path)
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC)
    with os.fdopen(fd, "rb") as f:
        fs = os.fstat(f.fileno())
        if (fs.st_dev, fs.st_ino) != (st.st_dev, st.st_ino):
            raise Refused("%s changed between lstat and open" % path)
        data = f.read(MAX + 1)
    if len(data) != st.st_size:
        raise Refused("%s changed while it was read" % path)
    pins[path] = hashlib.sha256(data).hexdigest()
    return data


def lines_of(data, what):
    text = data.decode("ascii")
    if not text.endswith("\n"):
        raise Refused("%s lacks a final newline" % what)
    return text[:-1].split("\n")


def main(argv):
    if len(argv) != 7 or argv[1] not in ("p1", "p0"):
        print("usage: k5_preconditions.py p1|p0 <RR5> <OWN> <TK> <T5> <expected-KH3-10-names.txt>", file=sys.stderr)
        return 2
    mode = argv[1]
    given = dict(zip(["RR5", "OWN", "TK", "T5", "names"], argv[2:]))
    problems = []
    pins = {}      # root records, K4H receipts and review files: printed on PASS
    scratch = {}   # operator files: already pinned by RR5/K5-OPERATOR-PINS.sha256, not repeated
    state = {}
    for name, want in FIXED.items():
        if given[name] != want:
            problems.append("argument %s is %r; expected %r" % (name, given[name], want))
    if problems:
        for p in problems:
            print("REFUSED: " + p, file=sys.stderr)
        return 1
    rr5, own, tk, t5 = FIXED["RR5"], FIXED["OWN"], FIXED["TK"], FIXED["T5"]

    def check(name, fn):
        try:
            fn()
        except Exception as e:  # every failure is a refusal; none is dropped
            problems.append("%s: %s: %s" % (name, type(e).__name__, e))

    def require(cond, message):
        if not cond:
            problems.append(message)

    def file_pin(path, sha, what):
        hex64(sha, what + " SHA-256")
        if not isinstance(path, str):
            raise Refused("%s path is not a string" % what)
        read(path, pins)
        require(pins[path] == sha, "%s %s does not have the declared SHA-256" % (what, path))

    def t5_state():
        if mode == "p1":
            require(not os.path.lexists(t5), "T5 already exists; K5 is one attempt in a fresh directory")
        else:
            st = os.lstat(t5)
            require(stat.S_ISDIR(st.st_mode) and not stat.S_ISLNK(st.st_mode), "T5 is a link or not a directory")

    def operator_pins():
        rel = lines_of(read(own + "/data/k5-operator-files.paths", scratch), "k5-operator-files.paths")
        for p in rel:
            if not REL_PATH.fullmatch(p) or ".." in p.split("/"):
                raise Refused("operator path %r is not a plain relative path" % p)
        want = {own + "/" + p for p in rel}
        require(len(want) == len(rel), "k5-operator-files.paths repeats a path")
        got = {}
        for line in lines_of(read(rr5 + "/K5-OPERATOR-PINS.sha256", pins), "K5-OPERATOR-PINS.sha256"):
            m = re.fullmatch(r"([0-9a-f]{64})  (/\S+)", line)
            if m is None:
                raise Refused("malformed operator pin line %r" % line)
            hex64(m.group(1), "operator pin for " + m.group(2))
            if m.group(2) in got:
                raise Refused("operator pin repeats %s" % m.group(2))
            got[m.group(2)] = m.group(1)
        if set(got) != want:
            raise Refused("operator pins: missing %r, extra %r" % (sorted(want - set(got)), sorted(set(got) - want)))
        for path, sha in sorted(got.items()):
            read(path, scratch)
            require(scratch[path] == sha, "operator file %s differs from its root pin" % path)
        state["operator"] = got

    def materialization():
        lines = lines_of(read(rr5 + "/K5-MATERIALIZATION-PIN.sha256", pins), "K5-MATERIALIZATION-PIN.sha256")
        m = re.fullmatch(r"([0-9a-f]{64})  (/\S+)", lines[0]) if len(lines) == 1 else None
        if m is None or m.group(2) != M5 + "/ROOT-SOURCE-MANIFEST.json":
            raise Refused("K5-MATERIALIZATION-PIN.sha256 is not one line naming M5/ROOT-SOURCE-MANIFEST.json")
        sha = hex64(m.group(1), "materialization manifest pin")
        manifest = strict_object(read(m.group(2), pins), "materialization manifest")
        require(pins[m.group(2)] == sha, "the materialization manifest differs from its root pin")
        exact_keys(manifest, ["format", "root", "files"], "materialization manifest")
        require(manifest.get("format") == "root-K5-source-materialization/1", "manifest format differs")
        require(manifest.get("root") == M5, "manifest root differs from M5")
        files = manifest.get("files")
        if not isinstance(files, list):
            raise Refused("manifest files is not a list")
        by_path = {}
        for e in files:
            exact_keys(e, ["path", "sha256"], "manifest entry")
            if not REL_PATH.fullmatch(e["path"]) or ".." in e["path"].split("/"):
                raise Refused("manifest path %r is not a plain relative path" % (e["path"],))
            hex64(e["sha256"], "manifest entry for " + e["path"])
            if e["path"] in by_path:
                raise Refused("manifest repeats %s" % e["path"])
            by_path[e["path"]] = e["sha256"]
        require(set(by_path) == set(MANIFEST_PATHS),
                "manifest coverage is not exactly the 26 candidate files: missing %r, extra %r"
                % (sorted(set(MANIFEST_PATHS) - set(by_path)), sorted(set(by_path) - set(MANIFEST_PATHS))))
        for path, pin in sorted(state.get("operator", {}).items()):
            entry = "operator/request/" + path[len(own) + 1:]
            require(by_path.get(entry) == pin, "manifest entry %s differs from the operator pin" % entry)
        state["manifest"] = sha

    def preconditions():
        pre = strict_object(read(rr5 + "/K5-PRECONDITIONS.json", pins), "K5-PRECONDITIONS.json")
        exact_keys(pre, PRE_KEYS, "K5-PRECONDITIONS.json")
        require(pre["format"] == "root-K5-preconditions/1", "preconditions format differs")
        require(pre["k4hTarget"] == tk, "k4hTarget is not the K4H run directory")
        rv = pre["k4hActualResultReview"]
        exact_keys(rv, REVIEW_KEYS, "k4hActualResultReview")
        file_pin(rv["reportPath"], rv["reportSha256"], "K4H actual-result review report")
        for key, want in REVIEW_VALUES:
            require(canonical(rv[key]) == canonical(want), "k4hActualResultReview.%s is %s; K5 requires %s"
                    % (key, canonical(rv[key]), canonical(want)))
        rc = pre["k4hReceipts"]
        exact_keys(rc, K4H_RECEIPTS, "k4hReceipts")
        for name in K4H_RECEIPTS:
            file_pin(tk + "/receipts/" + name, rc[name], "K4H receipt")
        hex64(pre["releaseMoriK2Sha256"], "releaseMoriK2Sha256")
        sr = pre["k5SourceReview"]
        exact_keys(sr, ["reportPath", "reportSha256", "blockingFindings", "operatorPinsSha256"], "k5SourceReview")
        file_pin(sr["reportPath"], sr["reportSha256"], "K5 source/effect review report")
        require(type(sr["blockingFindings"]) is int and sr["blockingFindings"] == 0,
                "the K5 source/effect review has blocking findings")
        require(sr["operatorPinsSha256"] == pins.get(rr5 + "/K5-OPERATOR-PINS.sha256"),
                "the K5 source/effect review does not name these operator pins")
        tl = pre["k5PositionTally"]
        exact_keys(tl, ["path", "sha256", "clauses"], "k5PositionTally")
        file_pin(tl["path"], tl["sha256"], "K5 position tally")
        exact_keys(tl["clauses"], CLAUSES, "k5PositionTally.clauses")
        for c in CLAUSES:
            n = tl["clauses"][c]
            require(type(n) is int and n >= 2, "clause %s has %r agreeing positions; two are required" % (c, n))
        dv = pre["dvK21R"]
        exact_keys(dv, ["textSha256", "positions"], "dvK21R")
        hex64(dv["textSha256"], "dvK21R.textSha256")
        require(dv["textSha256"].startswith(DV_PREFIX), "dvK21R.textSha256 does not begin b04371a2")
        require(type(dv["positions"]) is int and dv["positions"] >= 2, "DV-K2-1-R lacks two positions")
        state["pre"] = pre

    def k4h_outputs():
        pre = state["pre"]
        before = read(tk + "/receipts/KH-binaries-pre.sha256", pins)
        after = read(tk + "/receipts/KH-binaries-post.sha256", pins)
        require(before == after, "KH-binaries-post.sha256 differs from KH-binaries-pre.sha256")
        names = {}
        for line in lines_of(before, "KH-binaries-pre.sha256"):
            m = re.fullmatch(r"([0-9a-f]{64})  (mori-k2|deps/hostile_tier-[0-9a-f]+)", line)
            if m is None or m.group(2) in names:
                raise Refused("KH-binaries-pre.sha256 line %r is malformed or repeated" % line)
            names[m.group(2)] = m.group(1)
        require(len(names) == 2 and "mori-k2" in names, "KH-binaries-pre.sha256 is not mori-k2 plus one hostile_tier")
        require(names.get("mori-k2") == pre["releaseMoriK2Sha256"],
                "the KH2 mori-k2 hash differs from the root's releaseMoriK2Sha256")
        kh6 = strict_object(read(tk + "/receipts/KH6.json", pins), "KH6.json")
        require(kh6.get("verdict") == "PASS", "KH6.json verdict is %r" % kh6.get("verdict"))
        ok = lines_of(read(tk + "/receipts/KH3-ok-names.txt", pins), "KH3-ok-names.txt")
        expected = lines_of(read(FIXED["names"], pins), "expected-KH3-10-names.txt")
        require(len(ok) == 10 and sorted(ok) == sorted(expected), "KH3-ok-names.txt is not the ten expected names")
        for e in K4H_EXITS:
            require(read(tk + "/" + e, pins) == b"0\n", "TK/%s is not 0" % e)

    def authorization():
        auth = strict_object(read(rr5 + "/K5-AUTHORIZATION.json", pins), "K5-AUTHORIZATION.json")
        for key, want in AUTH_VALUES.items():
            require(key in auth and canonical(auth[key]) == canonical(want),
                    "K5-AUTHORIZATION.json %s is %s; required %s" % (key, canonical(auth.get(key)), canonical(want)))
        pre = state.get("pre")
        bound = {
            "operatorPinsSha256": pins.get(rr5 + "/K5-OPERATOR-PINS.sha256"),
            "preconditionsSha256": pins.get(rr5 + "/K5-PRECONDITIONS.json"),
            "materializationManifestSha256": state.get("manifest"),
            "dvK21RTextSha256": pre["dvK21R"]["textSha256"] if pre else None,
        }
        for key, want in bound.items():
            require(want is not None and auth.get(key) == want, "K5-AUTHORIZATION.json %s is not bound to %r" % (key, want))
        state["auth"] = pins[rr5 + "/K5-AUTHORIZATION.json"]

    def pre_launch():
        pl = strict_object(read(rr5 + "/K5-PRE-LAUNCH.json", pins), "K5-PRE-LAUNCH.json")
        for key, want in (("format", "root-K5-pre-launch/1"), ("targetAbsent", True),
                          ("operatorFilesRehashedBeforeP1", True), ("authorizationSha256", state.get("auth"))):
            require(want is not None and key in pl and canonical(pl[key]) == canonical(want),
                    "K5-PRE-LAUNCH.json %s is %s; required %s" % (key, canonical(pl.get(key)), canonical(want)))

    check("T5", t5_state)
    check("operator pins", operator_pins)
    check("materialization", materialization)
    check("preconditions", preconditions)
    check("K4H public outputs", k4h_outputs)
    check("authorization", authorization)
    check("pre-launch", pre_launch)
    if problems:
        for p in problems:
            print("REFUSED: " + p, file=sys.stderr)
        print("BLOCKED(prerequisite): %d root precondition problem(s); nothing was created or copied" % len(problems),
              file=sys.stderr)
        return 1
    for path in sorted(pins):
        print("%s  %s" % (pins[path], path))
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv))
    except Exception as e:  # interpreter-level refusal
        print("REFUSED: %s: %s" % (type(e).__name__, e), file=sys.stderr)
        sys.exit(2)
