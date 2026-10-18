#!/usr/bin/env python3
"""K9 native-consumer operator (SOURCE ONLY, unexecuted by its author).

Minimal adaptation of the reviewed R2 operator bd7487...3040a4: stable
bounds, bwrap containment, strict typed AUTH and exact telemetry parsing.
The one learned repair: a plain `test ... fail` annotation maps to
on_failure=succeed_eventually (test_like.rs:25-32; def_test_fail.snap:51);
only `fail once` maps to succeed_immediately, and a normal test maps to
fail_immediately. All 7 K9 tests are normal, so all expect
fail_immediately. Counts derive from len(EXPECTED_TESTS), never a
forgotten hard-coded 10. One exact authorised run only. No network,
setup, proving, RNG, key, wallet, signing or ledger effect exists here.
The spent K8 run is never restarted and its counters are never reset.
"""
import hashlib, json, os, re, resource, signal, stat, subprocess, sys, time

BASE = "/home/charl/moriarty-eutxo-validation-2026-10-12"
LOCAL = BASE + "/current-cardano-k2-k9-native-consumer-local-20261016"
OPERATOR_DIR = BASE + "/current-cardano-k2-k9-native-consumer-operator-materialized-20261016"
RECORDS = BASE + "/current-cardano-k2-k9-native-consumer-root-records-20261016"
AUTH_PATH = RECORDS + "/AUTH.json"
DATAONLY = BASE + "/aiken-fixed-compiler-release-discovery-20261015/v1.1.24-data-only"
PROD = "/home/charl/Moriarty/.worktrees/utxo/experiments/cardano-four-piece-settlement"
K9SRC = BASE + "/current-cardano-k2-k9-genuine-c2-consumer-candidate-20261016"
K9ADOPT = BASE + "/current-cardano-k2-k9-genuine-c2-source-root-records-20261016"
FIXDIR = BASE + "/current-cardano-k2-k8-public-fixture-generated-20261016"
STDPIN = BASE + "/aiken-stdlib-v220-source-20261015"
BINARY_SHA = "4257cc8be37cbde5bf7b5dd111e43a1277f45352fb2433ad7a7e647209aa7677"
BINARY_ENTRY_PATH = "compiler-data/aiken-x86_64-unknown-linux-musl/aiken"
RELEASE_DECLARED = "https://github.com/aiken-lang/aiken/releases/tag/v1.1.24"
RELEASE_TARGET_DECLARED = "bacbeb35fbc9f1db93cd2e1e93fb1d93053acb4a"
PACKAGES_TOML = b'[[packages]]\nname = "aiken-lang/stdlib"\nversion = "v2.2.0"\nsource = "github"\n'
PACKAGES_TOML_SHA = "42ca23893e5c0f43598c15a1c699fa2bcbb451999c054351f67dae174d49ffb6"
CAPS = {"wallSecondsPerAikenStep": 600, "killAfterSeconds": 10,
        "parentWaitSecondsPerAikenStep": 620, "cleanupWaitSecondsPerAikenStep": 20,
        "cpuSoftSeconds": 300, "cpuHardSeconds": 310, "addressSpaceBytes": 8589934592,
        "coreSize": 0, "maxStdoutBytes": 16777216, "maxStderrBytes": 16777216,
        "plannedAggregateSeconds": 1280, "prepFinalReserveSeconds": 120,
        "wholeOperatorWallSeconds": 1400, "wholeOperatorKillAfterSeconds": 10}
# Frozen before any execution from EXPECTATIONS.json and the pinned test
# source: 1 genuine positive True, N1/N2/N3 False through the pairing,
# and 3 structural Bool False guards. Every test is normal (no `fail`).
EXPECTED_TESTS = [
    ("k9_positive_claim_issue_returns_true", False),
    ("k9_n1_body_rederived_digest_claim_issue_rejected", False),
    ("k9_n2_context_rederived_digest_claim_issue_rejected", False),
    ("k9_n3_proof_ac_swapped_claim_issue_rejected", False),
    ("k9_wrong_minted_quantity_claim_issue_rejected", False),
    ("k9_mismatched_envelope_name_claim_issue_rejected", False),
    ("k9_wrong_vk_digest_claim_issue_rejected", False)]
EXPECTED_ANNOTATIONS = ["normal"] * len(EXPECTED_TESTS)

def expected_on_failure(annotation):
    # Literal map from the actual parser (test_like.rs:25-32,51):
    # no annotation -> FailImmediately; `fail once` -> SucceedImmediately;
    # plain `fail` -> SucceedEventually (def_test_fail.snap:51).
    return {"normal": "fail_immediately",
            "fail_once": "succeed_immediately",
            "fail": "succeed_eventually"}[annotation]

# role -> (absolute path, bytes, sha256), every value from INPUT-INDEX.json.
PINS = {
    "aikenBinary": (DATAONLY + "/compiler-data/aiken-x86_64-unknown-linux-musl/aiken", 38299808, BINARY_SHA),
    "dataOnlyManifest": (DATAONLY + "/DATA-ONLY-MANIFEST.json", 1554399, "8adde843b7552bc7810f4be9cfca73693b8cc5f11fbb9e07d586901fd656d487"),
    "prodAikenToml": (PROD + "/aiken.toml", 583, "07abb4793916465256dda02b9dd75d9930dd3f5888fb7fb1af0c490ababeac3f"),
    "prodAikenLock": (PROD + "/aiken.lock", 268, "ec5d6a58b5c5b7786e8e0b3b7fbc76736aea5739bcc6e6e9633cffa87130da83"),
    "prodClaim": (PROD + "/lib/pvc2/claim.ak", 4954, "d304fe59e15fdfe61df532dd0a05cb4a12a5de953f411896492de2ea0d8bb882"),
    "prodCore": (PROD + "/lib/pvc2/core.ak", 19555, "2f4fbf3b5a9c6f09afaa0bf9342a36e0d547ee993e82b854682d05c148f60cfa"),
    "prodCatalogue": (PROD + "/lib/pvc2/catalogue.ak", 6062, "d9d988a766bd1e4470bc16f8d5cbf8240200048e2396c06050ab940902263efe"),
    "prodTypes": (PROD + "/lib/pvc2/types.ak", 5681, "84284d308e9cd15b4227f005e36c56f34b565a45e4c1e083717d049d4ec51645"),
    "prodBytes": (PROD + "/lib/pvc2/bytes.ak", 5077, "9ad68722715a2438b689d45c8b910f2aff9428196e2bf7340555c1258e74e32f"),
    "prodGroth16": (PROD + "/lib/pvc2/groth16.ak", 3995, "1cc3da602d84ccfa70215e26108b76da05814c47b74a981528a4adc7bfd4068a"),
    "k8Fixture": (FIXDIR + "/lib/pvc2/k8_fixture.ak", 5269, "1b9e2a9a2eb6a2b47915bfc9aacd3b68a59d76f189f98da5615b4c653923ef64"),
    "k8FixtureManifest": (FIXDIR + "/k8-fixture-manifest.json", 2939, "3b4a46407af427d2d9caa3b2ca01d898b4502ba8aada77fc6c0de5f66a4f38f0"),
    "k9Builder": (K9SRC + "/lib/pvc2/k9_consumer_builder.ak", 6041, "239fb66552d2cd416cacb0eb11047385b761c1920246c7ca51638473831bbd1c"),
    "k9Test": (K9SRC + "/lib/pvc2/k9_genuine_c2_consumer_test.ak", 5445, "8d26629c383f2d26cdc37b6fc7e5b378deef13023f25f8014422606b41d814b2"),
    "k9Expectations": (K9SRC + "/EXPECTATIONS.json", 5917, "58a2d28f458e53863f0a1cf6400d4041a3266dc20395a97111e9649d7012556b"),
    "k9SourceAdoption": (K9ADOPT + "/SOURCE-ADOPTION.json", 19761, "4d73968396180b1f2acf4a8468f08a167e23efb04ee57ad77bd84c3b70f419c3"),
    "stdlibPin": (STDPIN + "/SOURCE-PIN.json", 363, "a26daa087fae05f9b35a06ed382d20e92e4d1ea8b76b59947134b7e4b2197266"),
    "stdlibManifest": (STDPIN + "/SOURCE-MANIFEST.json", 6164, "e44ed44783d17f4cb0b58c59f98a75bc09fc64c793dd25f3ab56b4097cdf1b1c"),
}
AUTH_KEYS = {"format", "status", "attempts", "retries", "maxAikenCalls", "localRoot",
             "materializedOperatorDir", "rootRecordsDir", "operatorSha256", "pins", "expectedTests", "caps"}
STAGED = []  # (absolute staged path, sha256) for the cheap post-run staged check

class Fail(Exception):
    pass

def sha_bytes(b):
    return hashlib.sha256(b).hexdigest()

def read_regular(path, want_bytes=None):
    st = os.lstat(path)
    if stat.S_ISLNK(st.st_mode) or not stat.S_ISREG(st.st_mode):
        raise Fail("not a regular non-symlink file: " + path)
    if want_bytes is not None and st.st_size != want_bytes:
        raise Fail("size mismatch: " + path)
    with open(path, "rb") as f:
        return f.read()

def check_pin(role):
    path, n, want = PINS[role]
    data = read_regular(path, n)
    if sha_bytes(data) != want:
        raise Fail("sha256 mismatch: " + role)
    return data

def check_all_pins():
    return {role: check_pin(role) and PINS[role][2] for role in PINS}

def is_int(x):
    return type(x) is int

def load_auth(argv):
    if len(argv) != 2 or argv[1] != AUTH_PATH:
        raise Fail("usage: operator.py " + AUTH_PATH)
    raw = read_regular(AUTH_PATH)
    try:
        auth = json.loads(raw.decode("utf-8"))
    except Exception as e:
        raise Fail("AUTH is not valid JSON: " + str(e))
    if not isinstance(auth, dict) or set(auth) != AUTH_KEYS:
        raise Fail("AUTH exact key set mismatch (12 top-level fields)")
    for k in ("format", "status", "localRoot", "materializedOperatorDir", "rootRecordsDir", "operatorSha256"):
        if type(auth[k]) is not str:
            raise Fail("AUTH field must be a string: " + k)
    if auth["format"] != "cardano-k2-k9-native-consumer-auth/1":
        raise Fail("AUTH format mismatch")
    if auth["status"] != "AUTHORIZED_EXACT_ONE_RUN":
        raise Fail("AUTH status is not an exact one-run authorisation")
    if not (is_int(auth["attempts"]) and auth["attempts"] == 1):
        raise Fail("AUTH attempts must be integer 1")
    if not (is_int(auth["retries"]) and auth["retries"] == 0):
        raise Fail("AUTH retries must be integer 0")
    if not (is_int(auth["maxAikenCalls"]) and auth["maxAikenCalls"] == 2):
        raise Fail("AUTH maxAikenCalls must be integer 2")
    if auth["localRoot"] != LOCAL or auth["materializedOperatorDir"] != OPERATOR_DIR:
        raise Fail("AUTH root mismatch")
    if auth["rootRecordsDir"] != RECORDS:
        raise Fail("AUTH records dir mismatch")
    caps = auth["caps"]
    if not isinstance(caps, dict) or set(caps) != set(CAPS):
        raise Fail("AUTH caps exact key set mismatch")
    if any(not is_int(v) for v in caps.values()):
        raise Fail("AUTH caps values must be integers, not booleans")
    if caps != CAPS:
        raise Fail("AUTH caps mismatch")
    pins = auth["pins"]
    if not isinstance(pins, dict) or set(pins) != set(PINS):
        raise Fail("AUTH pins exact role set mismatch")
    for role, (p, n, s) in PINS.items():
        e = pins[role]
        if not isinstance(e, dict) or set(e) != {"path", "bytes", "sha256"}:
            raise Fail("AUTH pin entry exact key set mismatch: " + role)
        if type(e["path"]) is not str or not is_int(e["bytes"]) or type(e["sha256"]) is not str:
            raise Fail("AUTH pin entry types mismatch: " + role)
        if e != {"path": p, "bytes": n, "sha256": s}:
            raise Fail("AUTH pin entry mismatch: " + role)
    tests = auth["expectedTests"]
    if not isinstance(tests, list) or len(tests) != len(EXPECTED_TESTS):
        raise Fail("AUTH expectedTests must list exactly %d tests" % len(EXPECTED_TESTS))
    for e in tests:
        if not isinstance(e, dict) or set(e) != {"name", "fail"}:
            raise Fail("AUTH expectedTests entry exact key set mismatch")
        if type(e["name"]) is not str or type(e["fail"]) is not bool:
            raise Fail("AUTH expectedTests entry types mismatch (name str, fail bool)")
    want_tests = [{"name": n, "fail": f} for n, f in EXPECTED_TESTS]
    if tests != want_tests:
        raise Fail("AUTH expectedTests mismatch")
    self_path = os.path.abspath(__file__)
    if os.path.dirname(os.path.dirname(self_path)) != OPERATOR_DIR or os.path.basename(self_path) != "operator.py":
        raise Fail("operator is not at <materializedOperatorDir>/runner/operator.py")
    digest = sha_bytes(read_regular(self_path))
    if auth["operatorSha256"] != digest:
        raise Fail("AUTH operatorSha256 does not match this file (placeholder or stale hash)")
    return auth

def parse_release_identity():
    manifest = json.loads(check_pin("dataOnlyManifest").decode("utf-8"))
    if not isinstance(manifest, dict):
        raise Fail("DATA-ONLY-MANIFEST is not a JSON object")
    if manifest.get("release") != RELEASE_DECLARED or manifest.get("releaseTarget") != RELEASE_TARGET_DECLARED:
        raise Fail("DATA-ONLY-MANIFEST declared release/releaseTarget mismatch")
    files = manifest.get("files")
    if not isinstance(files, list):
        raise Fail("DATA-ONLY-MANIFEST files is not a list")
    entries = [e for e in files if isinstance(e, dict) and e.get("path") == BINARY_ENTRY_PATH]
    if len(entries) != 1:
        raise Fail("DATA-ONLY-MANIFEST must contain exactly one aiken binary entry")
    e = entries[0]
    if set(e) != {"path", "bytes", "sha256", "mode"} or not is_int(e["bytes"]) or type(e["sha256"]) is not str:
        raise Fail("DATA-ONLY-MANIFEST binary entry shape/types mismatch")
    if e["bytes"] != PINS["aikenBinary"][1] or e["sha256"] != BINARY_SHA or e["mode"] != "0644 data only":
        raise Fail("DATA-ONLY-MANIFEST binary entry does not match the pinned binary")
    return {"declaredRelease": RELEASE_DECLARED, "declaredReleaseTarget": RELEASE_TARGET_DECLARED,
            "binaryEntryPath": BINARY_ENTRY_PATH, "binaryBytes": e["bytes"], "binarySha256": e["sha256"],
            "provenance": "manifest-declared only; same-origin digests are not an independent signature, "
                          "the binary was not shown rebuilt from indexed source, and no --version was run"}

def derive_tests(test_bytes):
    # Source guard, run before any execution: each name and fail flag (all
    # False here) must equal the frozen expectations, and each literal
    # annotation must map through expected_on_failure above. A plain
    # `fail` is succeed_eventually, never succeed_immediately.
    text = test_bytes.decode("utf-8")
    found, modes = [], []
    for m in re.finditer(r"(?m)^test ([a-z0-9_]+)\(\)( fail once| fail)? \{", text):
        ann = {None: "normal", " fail once": "fail_once", " fail": "fail"}[m.group(2)]
        found.append((m.group(1), ann != "normal"))
        modes.append(expected_on_failure(ann))
    if found != EXPECTED_TESTS or len(found) != len(EXPECTED_TESTS):
        raise Fail("derived test names/fail flags do not equal the pinned %d (all normal)" % len(EXPECTED_TESTS))
    if modes != [expected_on_failure(a) for a in EXPECTED_ANNOTATIONS]:
        raise Fail("derived on_failure modes do not equal the frozen annotation map")
    return found

def write_exclusive(path, data, mode=0o644):
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, mode)
    try:
        os.write(fd, data)
    finally:
        os.close(fd)

def copy_pinned(role, dest):
    data = check_pin(role)
    write_exclusive(dest, data)
    if sha_bytes(read_regular(dest)) != PINS[role][2]:
        raise Fail("copied bytes differ: " + role)
    STAGED.append((dest, PINS[role][2]))

def stage_project(work):
    # Exact import closure: 6 unchanged production AK (claim, core,
    # catalogue, types, bytes, groth16) + adopted k8_fixture + K9 builder
    # and test = 9 AK, plus verbatim aiken.toml/aiken.lock = 11 files.
    # The K8 test module, validators/c2.ak and other parent test modules
    # are deliberately excluded. No production byte is changed.
    os.makedirs(work + "/lib/pvc2")
    os.makedirs(work + "/build/packages/aiken-lang-stdlib")
    copy_pinned("prodAikenToml", work + "/aiken.toml")
    copy_pinned("prodAikenLock", work + "/aiken.lock")
    copy_pinned("prodClaim", work + "/lib/pvc2/claim.ak")
    copy_pinned("prodCore", work + "/lib/pvc2/core.ak")
    copy_pinned("prodCatalogue", work + "/lib/pvc2/catalogue.ak")
    copy_pinned("prodTypes", work + "/lib/pvc2/types.ak")
    copy_pinned("prodBytes", work + "/lib/pvc2/bytes.ak")
    copy_pinned("prodGroth16", work + "/lib/pvc2/groth16.ak")
    copy_pinned("k8Fixture", work + "/lib/pvc2/k8_fixture.ak")
    copy_pinned("k9Builder", work + "/lib/pvc2/k9_consumer_builder.ak")
    copy_pinned("k9Test", work + "/lib/pvc2/k9_genuine_c2_consumer_test.ak")
    manifest = json.loads(check_pin("stdlibManifest").decode("utf-8"))
    entries = manifest.get("files")
    if not isinstance(entries, list) or len(entries) != 38:
        raise Fail("stdlib manifest does not list exactly 38 files")
    for e in entries:
        rel = e["path"]
        if rel.startswith("/") or ".." in rel.split("/"):
            raise Fail("unsafe stdlib path: " + rel)
        src = STDPIN + "/source/" + rel
        data = read_regular(src, e["bytes"])
        if sha_bytes(data) != e["sha256"]:
            raise Fail("stdlib file hash mismatch: " + rel)
        dest = work + "/build/packages/aiken-lang-stdlib/" + rel
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        write_exclusive(dest, data)
        STAGED.append((dest, e["sha256"]))
    write_exclusive(work + "/build/packages/packages.toml", PACKAGES_TOML)
    if sha_bytes(PACKAGES_TOML) != PACKAGES_TOML_SHA:
        raise Fail("packages.toml hash mismatch")
    STAGED.append((work + "/build/packages/packages.toml", PACKAGES_TOML_SHA))

def verify_staged():
    for path, want in STAGED:
        if sha_bytes(read_regular(path)) != want:
            raise Fail("staged bytes differ after run: " + path)
    tool = LOCAL + "/tool/aiken"
    if sha_bytes(read_regular(tool)) != BINARY_SHA:
        raise Fail("staged tool bytes differ after run")
    if os.lstat(tool).st_mode & 0o777 != 0o555:
        raise Fail("staged tool is no longer mode 555")
    return True

def bwrap_argv(aiken_args):
    return ["/usr/bin/timeout", "--signal=TERM", "--kill-after=" + str(CAPS["killAfterSeconds"]) + "s",
            str(CAPS["wallSecondsPerAikenStep"]) + "s",
            "/usr/bin/prlimit", "--cpu=300:310", "--as=8589934592", "--core=0",
            "--fsize=16777216", "--",
            "/usr/bin/bwrap", "--unshare-all", "--die-with-parent", "--new-session",
            "--clearenv", "--setenv", "PATH", "/tool", "--setenv", "HOME", "/tmp",
            "--setenv", "TMPDIR", "/tmp", "--setenv", "NO_COLOR", "1",
            "--setenv", "LC_ALL", "C", "--setenv", "TZ", "UTC",
            "--ro-bind", LOCAL + "/tool/aiken", "/tool/aiken",
            "--bind", LOCAL + "/work", "/work", "--tmpfs", "/tmp",
            "--dev", "/dev", "--proc", "/proc", "--chdir", "/work",
            "/tool/aiken"] + aiken_args

def _signal_group(proc, sig):
    try:
        os.killpg(proc.pid, sig)
        return True
    except Exception:
        return False

def run_step(name, aiken_args, logs, result):
    out_path, err_path = logs + "/" + name + ".stdout.log", logs + "/" + name + ".stderr.log"
    argv = bwrap_argv(aiken_args)
    result["aikenLaunchAttempts"] += 1  # spawn attempt counted before Popen
    attempt = result["aikenLaunchAttempts"]
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    t0 = time.monotonic()
    with open(out_path, "wb") as out, open(err_path, "wb") as err:
        try:
            proc = subprocess.Popen(argv, stdin=subprocess.DEVNULL, stdout=out, stderr=err,
                                    cwd="/", env={"PATH": "/usr/bin:/bin"}, start_new_session=True)
        except Exception as e:
            result["incompleteStep"] = {"name": name, "launchAttempt": attempt,
                                        "nativeExecutableStarted": "not_measured",
                                        "outcome": "spawn_failed_before_return"}
            raise Fail(name + ": launch spawn failed after attempt was counted: " + str(e))
        try:
            rc = proc.wait(timeout=CAPS["parentWaitSecondsPerAikenStep"])
        except subprocess.TimeoutExpired:
            actions = []
            if _signal_group(proc, signal.SIGTERM):
                actions.append("SIGTERM to timeout-leader process group")
            try:
                rc = proc.wait(timeout=10)
            except subprocess.TimeoutExpired:
                if _signal_group(proc, signal.SIGKILL):
                    actions.append("SIGKILL to timeout-leader process group")
                try:
                    rc = proc.wait(timeout=10)
                except subprocess.TimeoutExpired:
                    result["incompleteStep"] = {"name": name, "launchAttempt": attempt,
                                                "nativeExecutableStarted": "not_measured",
                                                "outcome": "cleanup_wait_exhausted", "cleanupActions": actions}
                    raise Fail(name + ": bounded cleanup wait exhausted; no hard-kill claim is made "
                                     "for uninterruptible (D-state) processes")
            result["incompleteStep"] = {"name": name, "launchAttempt": attempt,
                                        "nativeExecutableStarted": "not_measured",
                                        "outcome": "parent_wait_exceeded", "cleanupActions": actions}
            raise Fail(name + ": parent wait bound exceeded; signalled the timeout-leader process group "
                             "(" + ", ".join(actions) + "); bwrap runs in a new session, so its descendants "
                             "are tied by --die-with-parent and the bwrap PID namespace, not by killpg alone")
    wall = time.monotonic() - t0
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    cpu = (after.ru_utime + after.ru_stime) - (before.ru_utime + before.ru_stime)
    for p, cap in ((out_path, CAPS["maxStdoutBytes"]), (err_path, CAPS["maxStderrBytes"])):
        if os.path.getsize(p) > cap:
            raise Fail(name + ": log exceeds the 16 MiB cap: " + p)
    return {"name": name, "argv": argv, "exitCode": rc, "wallSeconds": wall,
            "launchAttempt": attempt, "nativeExecutableStarted": "not_measured",
            "waitedChildrenCpuSecondsDelta": cpu, "stdout": out_path, "stderr": err_path,
            "stdoutBytes": os.path.getsize(out_path), "stderrBytes": os.path.getsize(err_path)}

def _check_summary(summary, label):
    n = len(EXPECTED_TESTS)
    if not isinstance(summary, dict) or set(summary) != {"total", "passed", "failed", "kind"}:
        raise Fail(label + " summary exact key set mismatch")
    if any(not is_int(summary[k]) for k in ("total", "passed", "failed")):
        raise Fail(label + " summary values must be integers, not booleans")
    if (summary["total"], summary["passed"], summary["failed"]) != (n, n, 0):
        raise Fail(label + " summary is not total=%d passed=%d failed=0" % (n, n))
    kind = summary["kind"]
    if not isinstance(kind, dict) or set(kind) != {"unit", "property"}:
        raise Fail(label + " summary kind exact key set mismatch")
    if not is_int(kind["unit"]) or not is_int(kind["property"]) or (kind["unit"], kind["property"]) != (n, 0):
        raise Fail(label + " summary kind is not unit=%d property=0" % n)

def validate_test_output(step):
    # Exact FinishedTests JSON from telemetry/json.rs (aiken 1.1.24): whole
    # stdout must be one complete JSON document; any other event delegates
    # to the Terminal listener and would pollute stdout, failing closed.
    # Whole JSON is preserved on failure; no bare-string PASS is accepted.
    n = len(EXPECTED_TESTS)
    raw = read_regular(step["stdout"])
    if not raw.strip():
        raise Fail("step2 stdout is empty; JSON test output missing (missing JSON never means pass)")
    try:
        doc = json.loads(raw.decode("utf-8"))
    except Exception as e:
        raise Fail("step2 stdout is not one complete JSON document (truncated, polluted or non-JSON): " + str(e))
    if not isinstance(doc, dict) or set(doc) != {"seed", "summary", "modules"}:
        raise Fail("step2 JSON top-level keys are not exactly seed/summary/modules")
    if not is_int(doc["seed"]) or not (0 <= doc["seed"] <= 4294967295):
        raise Fail("step2 JSON seed is not a u32 integer")
    _check_summary(doc["summary"], "top-level")
    modules = doc["modules"]
    if not isinstance(modules, list) or len(modules) != 1:
        raise Fail("step2 JSON must contain exactly one module")
    module = modules[0]
    if not isinstance(module, dict) or set(module) != {"name", "summary", "tests"}:
        raise Fail("step2 JSON module exact key set mismatch")
    if module["name"] != "pvc2/k9_genuine_c2_consumer_test":
        raise Fail("step2 JSON module is not pvc2/k9_genuine_c2_consumer_test")
    _check_summary(module["summary"], "module")
    tests = module["tests"]
    if not isinstance(tests, list) or len(tests) != n:
        raise Fail("step2 JSON module must contain exactly %d test rows" % n)
    titles, units = [], []
    for row, (want_name, want_fail), ann in zip(tests, EXPECTED_TESTS, EXPECTED_ANNOTATIONS):
        if not isinstance(row, dict) or not {"title", "status", "on_failure", "execution_units"} <= set(row):
            raise Fail("step2 JSON test row is missing required fields: " + want_name)
        if not set(row) <= {"title", "status", "on_failure", "execution_units", "traces"}:
            raise Fail("step2 JSON test row has unexpected fields (no extras allowed): " + want_name)
        if row["title"] != want_name:
            raise Fail("step2 JSON test titles are not the pinned %d in source order "
                       "(missing, extra, duplicated or reordered row at %s)" % (n, want_name))
        if row["status"] != "pass":
            raise Fail("step2 JSON test did not pass: " + want_name)
        want_on_failure = expected_on_failure(ann)  # all K9 normal -> fail_immediately
        if row["on_failure"] != want_on_failure:
            raise Fail("step2 JSON on_failure mismatch for " + want_name + ": want " + want_on_failure)
        eu = row["execution_units"]
        if not isinstance(eu, dict) or set(eu) != {"mem", "cpu"}:
            raise Fail("step2 JSON execution_units exact key set mismatch: " + want_name)
        if not is_int(eu["mem"]) or not is_int(eu["cpu"]) or eu["mem"] < 0 or eu["cpu"] < 0:
            raise Fail("step2 JSON execution_units must be non-negative integers, not booleans: " + want_name)
        if "traces" in row and (not isinstance(row["traces"], list) or
                                any(type(t) is not str for t in row["traces"])):
            raise Fail("step2 JSON traces must be a list of strings: " + want_name)
        titles.append(row["title"])
        units.append({"title": want_name, "mem": eu["mem"], "cpu": eu["cpu"]})
    if titles != [nm for nm, _ in EXPECTED_TESTS] or len(set(titles)) != n:
        raise Fail("step2 JSON test titles do not equal the pinned %d exactly once each" % n)
    return {"jsonParsed": True, "schema": "aiken-1.1.24 telemetry/json.rs FinishedTests",
            "seed": doc["seed"], "module": module["name"], "testTitles": titles,
            "summary": doc["summary"], "moduleSummary": module["summary"], "executionUnits": units,
            "executionUnitsMeaning": "per-test Aiken evaluation units from the run itself; "
                                     "not an on-chain budget and not capped here beyond the step caps"}

def main(argv):
    result = {"format": "cardano-k2-k9-native-consumer-result/1", "status": "FAILED",
              "aikenLaunchAttempts": 0, "aikenCallsReturned": 0,
              "nativeExecutablesStarted": "not_measured", "attempts": 1, "retries": 0, "steps": []}
    local_created = False
    staging_complete = False
    try:
        load_auth(argv)
        check_all_pins()
        result["hostInputsUnchangedBefore"] = True
        result["releaseIdentityDeclared"] = parse_release_identity()
        # Real adopted format root-K9-genuine-C2-consumer-source-adoption/1:
        # check only fields that record actually contains. This is a
        # SOURCE adoption; it grants no native authority by itself.
        adoption = json.loads(check_pin("k9SourceAdoption").decode("utf-8"))
        if adoption.get("sourceReady") is not True:
            raise Fail("K9 source adoption is not sourceReady")
        if adoption.get("nativeAuthorityGranted") is not False:
            raise Fail("K9 source adoption unexpectedly grants native authority")
        if adoption.get("compiled") is not False or adoption.get("executed") is not False:
            raise Fail("K9 source adoption unexpectedly claims compilation/execution")
        # NOTE: a fresh K8 actual audit is required by root BEFORE any
        # runtime AUTH is materialised (see EXECUTION-PROPOSAL.json). That
        # future record does not exist yet, so this operator deliberately
        # has no predicate guard over it and invents no hash for it.
        derive_tests(check_pin("k9Test"))
        if os.path.lexists(LOCAL):
            raise Fail("fresh local root already exists; refusing to overwrite: " + LOCAL)
        os.makedirs(LOCAL + "/tool")
        os.makedirs(LOCAL + "/logs")
        os.makedirs(LOCAL + "/receipts")
        local_created = True
        staged = LOCAL + "/tool/aiken"
        write_exclusive(staged, check_pin("aikenBinary"), 0o600)
        if sha_bytes(read_regular(staged)) != BINARY_SHA:
            raise Fail("staged binary hash mismatch")
        os.chmod(staged, 0o555)
        if os.lstat(PINS["aikenBinary"][0]).st_mode & 0o777 != 0o644:
            raise Fail("source binary is no longer mode 644 data")
        stage_project(LOCAL + "/work")
        result["stagedInputsVerifiedAtStaging"] = verify_staged()
        staging_complete = True
        s1 = run_step("step1-typecheck", ["check", "--skip-tests", "--trace-filter", "all",
                                          "--trace-level", "silent"], LOCAL + "/logs", result)
        result["steps"].append(s1)
        result["aikenCallsReturned"] = len(result["steps"])
        if s1["exitCode"] != 0:
            raise Fail("step1 aiken check --skip-tests exited nonzero; step2 not started")
        s2 = run_step("step2-k9-tests", ["check", "--trace-filter", "all", "--trace-level", "silent",
                                         "--match-tests", "pvc2/k9_genuine_c2_consumer_test"], LOCAL + "/logs", result)
        result["steps"].append(s2)
        result["aikenCallsReturned"] = len(result["steps"])
        if s2["exitCode"] != 0:
            raise Fail("step2 targeted aiken check exited nonzero; its output is not validated as a pass")
        result["outputValidation"] = validate_test_output(s2)
        result["status"] = "COMPLETED_AWAITING_FRESH_ACTUAL_AUDIT"
    except Exception as e:
        result["error"] = str(e)
    if local_created:
        try:
            check_all_pins()
            result["hostInputsUnchangedAfter"] = True
        except Exception as e:
            result["hostInputsUnchangedAfter"] = False
            result["hostPostcheckError"] = str(e)
            if result["status"].startswith("COMPLETED"):
                result["status"] = "FAILED"
                result["postcheckError"] = "host pin postcheck failed: " + str(e)
        if staging_complete:
            try:
                result["stagedInputsUnchangedAfter"] = verify_staged()
            except Exception as e:
                result["stagedInputsUnchangedAfter"] = False
                result["stagedPostcheckError"] = str(e)
                if result["status"].startswith("COMPLETED"):
                    result["status"] = "FAILED"
                    result["postcheckError"] = "staged postcheck failed: " + str(e)
    write_failed = False
    if local_created:
        try:
            write_exclusive(LOCAL + "/receipts/RESULT.json",
                            (json.dumps(result, indent=2, sort_keys=True) + "\n").encode())
        except Exception as e:
            write_failed = True
            result["status"] = "FAILED"
            result["resultWriteError"] = "RESULT.json write failed in the fresh local root: " + str(e)
    else:
        result["resultFileWritten"] = False
        result["resultFileNote"] = ("no RESULT.json was written because this run did not create the "
                                    "fresh local root (early AUTH/pin/freshness failure or a pre-existing "
                                    "root, whose bytes are never written by this operator); the stdout "
                                    "JSON below is the terminal record for root to capture")
    print(json.dumps(result, sort_keys=True))
    return 0 if result["status"].startswith("COMPLETED") and not write_failed else 1

if __name__ == "__main__":
    sys.exit(main(sys.argv))
