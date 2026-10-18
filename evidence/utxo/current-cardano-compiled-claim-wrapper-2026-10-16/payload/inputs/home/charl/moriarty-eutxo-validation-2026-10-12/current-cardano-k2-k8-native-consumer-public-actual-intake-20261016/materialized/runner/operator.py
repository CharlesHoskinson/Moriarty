#!/usr/bin/env python3
"""K8 native-consumer operator R2 (REVISED PROPOSED SOURCE, unexecuted by its author).

Repairs F1-F6 from the fresh 5c72e3 source review without changing any
production function: exact Aiken FinishedTests JSON parsing, strict nested
AUTH typing, honest process-group / bwrap cleanup with finite whole-operator
budgets, spawn-attempt counting, staged/host postchecks and manifest-declared
release identity. One exact authorised run only. No network, setup, proving,
RNG, key, wallet, signing or ledger effect is implemented here.
"""
import hashlib, json, os, re, resource, shutil, signal, stat, subprocess, sys, time

BASE = "/home/charl/moriarty-eutxo-validation-2026-10-12"
LOCAL = BASE + "/current-cardano-k2-k8-native-consumer-local-20261016"
OPERATOR_DIR = BASE + "/current-cardano-k2-k8-native-consumer-operator-materialized-20261016"
RECORDS = BASE + "/current-cardano-k2-k8-native-consumer-root-records-20261016"
AUTH_PATH = RECORDS + "/AUTH.json"
DATAONLY = BASE + "/aiken-fixed-compiler-release-discovery-20261015/v1.1.24-data-only"
PROD = "/home/charl/Moriarty/.worktrees/utxo/experiments/cardano-four-piece-settlement"
K8SRC = BASE + "/current-cardano-k2-k8-aiken-genuine-proof-muse-source-20261016/candidate"
FIXDIR = BASE + "/current-cardano-k2-k8-public-fixture-generated-20261016"
ROOTREC = BASE + "/current-cardano-k2-k8-public-fixture-generation-root-records-20261016"
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
EXPECTED_TESTS = [
    ("k8_positive_verify_digest", False), ("k8_n1_body_rederived_digest_rejected", False),
    ("k8_n2_context_rederived_digest_rejected", False), ("k8_n3_proof_ac_swapped_rejected", False),
    ("k8_scalars_are_be128_halves_of_statement_digest", False),
    ("k8_n1_scalars_are_be128_halves_of_n1_digest", False),
    ("k8_n2_scalars_are_be128_halves_of_n2_digest", False),
    ("k8_byte_profile_matches_groth16_layout", False),
    ("k8_negative_cases_are_distinct_from_positive", False), ("k8_short_vk_is_refused", True)]
# role -> (absolute path, bytes, sha256). Every existing byte is pinned here and in AUTH.
PINS = {
    "aikenBinary": (DATAONLY + "/compiler-data/aiken-x86_64-unknown-linux-musl/aiken", 38299808, BINARY_SHA),
    "dataOnlyManifest": (DATAONLY + "/DATA-ONLY-MANIFEST.json", 1554399, "8adde843b7552bc7810f4be9cfca73693b8cc5f11fbb9e07d586901fd656d487"),
    "prodAikenToml": (PROD + "/aiken.toml", 583, "07abb4793916465256dda02b9dd75d9930dd3f5888fb7fb1af0c490ababeac3f"),
    "prodAikenLock": (PROD + "/aiken.lock", 268, "ec5d6a58b5c5b7786e8e0b3b7fbc76736aea5739bcc6e6e9633cffa87130da83"),
    "prodGroth16": (PROD + "/lib/pvc2/groth16.ak", 3995, "1cc3da602d84ccfa70215e26108b76da05814c47b74a981528a4adc7bfd4068a"),
    "prodBytes": (PROD + "/lib/pvc2/bytes.ak", 5077, "9ad68722715a2438b689d45c8b910f2aff9428196e2bf7340555c1258e74e32f"),
    "prodTypes": (PROD + "/lib/pvc2/types.ak", 5681, "84284d308e9cd15b4227f005e36c56f34b565a45e4c1e083717d049d4ec51645"),
    "k8Fixture": (FIXDIR + "/lib/pvc2/k8_fixture.ak", 5269, "1b9e2a9a2eb6a2b47915bfc9aacd3b68a59d76f189f98da5615b4c653923ef64"),
    "k8FixtureManifest": (FIXDIR + "/k8-fixture-manifest.json", 2939, "3b4a46407af427d2d9caa3b2ca01d898b4502ba8aada77fc6c0de5f66a4f38f0"),
    "k8Test": (K8SRC + "/lib/pvc2/k8_genuine_proof_test.ak", 4782, "d25c3811132ee99dab4bba9862ec87147931c80cd516e6eae5940e2088551eec"),
    "k8ActualAdoption": (ROOTREC + "/ACTUAL-ADOPTION.json", 867, "5e650d2398ba1bdbe3702307a103cf661bd153220d0fddfc822b038ce851f0b5"),
    "k8GenerationResult": (ROOTREC + "/RESULT.json", 4361, "67cc14890c3280787e21d9e4846904d7d687539ea75056537b2e24d4db09798a"),
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
        raise Fail("AUTH exact key set mismatch")
    for k in ("format", "status", "localRoot", "materializedOperatorDir", "rootRecordsDir", "operatorSha256"):
        if type(auth[k]) is not str:
            raise Fail("AUTH field must be a string: " + k)
    if auth["format"] != "cardano-k2-k8-native-consumer-auth/1":
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
        raise Fail("AUTH expectedTests must list exactly 10 tests")
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
    text = test_bytes.decode("utf-8")
    found = [(m.group(1), bool(m.group(2))) for m in
             re.finditer(r"(?m)^test ([a-z0-9_]+)\(\)( fail)? \{", text)]
    if found != EXPECTED_TESTS or len(found) != 10 or sum(1 for _, f in found if f) != 1:
        raise Fail("derived test names/fail flags do not equal the pinned 10 (9 normal, 1 fail)")
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
    os.makedirs(work + "/lib/pvc2")
    os.makedirs(work + "/build/packages/aiken-lang-stdlib")
    copy_pinned("prodAikenToml", work + "/aiken.toml")  # byte-verbatim, no alias renaming
    copy_pinned("prodAikenLock", work + "/aiken.lock")  # byte-verbatim
    copy_pinned("prodGroth16", work + "/lib/pvc2/groth16.ak")
    copy_pinned("prodBytes", work + "/lib/pvc2/bytes.ak")
    copy_pinned("prodTypes", work + "/lib/pvc2/types.ak")
    copy_pinned("k8Fixture", work + "/lib/pvc2/k8_fixture.ak")
    copy_pinned("k8Test", work + "/lib/pvc2/k8_genuine_proof_test.ak")
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
    if not isinstance(summary, dict) or set(summary) != {"total", "passed", "failed", "kind"}:
        raise Fail(label + " summary exact key set mismatch")
    if any(not is_int(summary[k]) for k in ("total", "passed", "failed")):
        raise Fail(label + " summary values must be integers, not booleans")
    if (summary["total"], summary["passed"], summary["failed"]) != (10, 10, 0):
        raise Fail(label + " summary is not total=10 passed=10 failed=0")
    kind = summary["kind"]
    if not isinstance(kind, dict) or set(kind) != {"unit", "property"}:
        raise Fail(label + " summary kind exact key set mismatch")
    if not is_int(kind["unit"]) or not is_int(kind["property"]) or (kind["unit"], kind["property"]) != (10, 0):
        raise Fail(label + " summary kind is not unit=10 property=0")

def validate_test_output(step):
    # Exact FinishedTests JSON from telemetry/json.rs (aiken 1.1.24): whole-file
    # parse only; non-FinishedTests events delegate to the Terminal listener and
    # would pollute stdout, which must fail closed rather than be stripped away.
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
    if module["name"] != "pvc2/k8_genuine_proof_test":
        raise Fail("step2 JSON module is not pvc2/k8_genuine_proof_test")
    _check_summary(module["summary"], "module")
    tests = module["tests"]
    if not isinstance(tests, list) or len(tests) != 10:
        raise Fail("step2 JSON module must contain exactly 10 test rows")
    titles, units = [], []
    for row, (want_name, want_fail) in zip(tests, EXPECTED_TESTS):
        if not isinstance(row, dict) or not {"title", "status", "on_failure", "execution_units"} <= set(row):
            raise Fail("step2 JSON test row is missing required fields: " + want_name)
        if not set(row) <= {"title", "status", "on_failure", "execution_units", "traces"}:
            raise Fail("step2 JSON test row has unexpected fields: " + want_name)
        if row["title"] != want_name:
            raise Fail("step2 JSON test titles are not the pinned 10 in source-definition order "
                       "(missing, extra, duplicated or reordered row at " + want_name + ")")
        if row["status"] != "pass":
            raise Fail("step2 JSON test did not pass: " + want_name)
        want_on_failure = "succeed_immediately" if want_fail else "fail_immediately"
        if row["on_failure"] != want_on_failure:
            raise Fail("step2 JSON on_failure mismatch for " + want_name + ": want " + want_on_failure)
        eu = row["execution_units"]
        if not isinstance(eu, dict) or set(eu) != {"mem", "cpu"}:
            raise Fail("step2 JSON execution_units exact key set mismatch: " + want_name)
        if not is_int(eu["mem"]) or not is_int(eu["cpu"]) or eu["mem"] < 0 or eu["cpu"] < 0:
            raise Fail("step2 JSON execution_units must be non-negative integers: " + want_name)
        if "traces" in row and (not isinstance(row["traces"], list) or
                                any(type(t) is not str for t in row["traces"])):
            raise Fail("step2 JSON traces must be a list of strings: " + want_name)
        titles.append(row["title"])
        units.append({"title": want_name, "mem": eu["mem"], "cpu": eu["cpu"]})
    if titles != [n for n, _ in EXPECTED_TESTS] or len(set(titles)) != 10:
        raise Fail("step2 JSON test titles do not equal the pinned 10 exactly once each")
    return {"jsonParsed": True, "schema": "aiken-1.1.24 telemetry/json.rs FinishedTests",
            "seed": doc["seed"], "module": module["name"], "testTitles": titles,
            "summary": doc["summary"], "moduleSummary": module["summary"], "executionUnits": units,
            "executionUnitsMeaning": "per-test Aiken evaluation units from the run itself; "
                                     "not an on-chain budget and not capped here beyond the step caps"}

def main(argv):
    result = {"format": "cardano-k2-k8-native-consumer-result/1", "status": "FAILED",
              "aikenLaunchAttempts": 0, "aikenCallsReturned": 0,
              "nativeExecutablesStarted": "not_measured", "attempts": 1, "retries": 0, "steps": []}
    local_created = False
    staging_complete = False
    try:
        load_auth(argv)
        check_all_pins()
        result["hostInputsUnchangedBefore"] = True
        result["releaseIdentityDeclared"] = parse_release_identity()
        adoption = json.loads(check_pin("k8ActualAdoption").decode("utf-8"))
        if adoption.get("ordinaryFixturePrerequisiteClosed") is not True:
            raise Fail("K8 ordinary fixture prerequisite is not closed")
        if adoption.get("nativeExecutionAuthorityGranted") is not False:
            raise Fail("K8 adoption unexpectedly grants native authority")
        if adoption.get("pairingExecuted") is not False or adoption.get("sourceCompiled") is not False:
            raise Fail("K8 adoption unexpectedly claims pairing/compilation")
        derive_tests(check_pin("k8Test"))
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
        s2 = run_step("step2-k8-tests", ["check", "--trace-filter", "all", "--trace-level", "silent",
                                         "--match-tests", "pvc2/k8_genuine_proof_test"], LOCAL + "/logs", result)
        result["steps"].append(s2)
        result["aikenCallsReturned"] = len(result["steps"])
        if s2["exitCode"] != 0:
            raise Fail("step2 targeted aiken check exited nonzero; its output is not validated as a pass")
        result["outputValidation"] = validate_test_output(s2)
        result["status"] = "COMPLETED_AWAITING_FRESH_ACTUAL_AUDIT"
    except Exception as e:
        result["error"] = str(e)
    if local_created:
        # Terminal postchecks: capture failures, preserve any actual test
        # evidence already recorded, and never qualify a run whose postcheck failed.
        try:
            check_all_pins()
            result["hostInputsUnchangedAfter"] = True
        except Exception as e:
            result["hostInputsUnchangedAfter"] = False
            result["hostPostcheckError"] = str(e)
            if result["status"].startswith("COMPLETED"):
                result["status"] = "FAILED"
                result["postcheckError"] = "host 14-pin postcheck failed: " + str(e)
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
                                    "JSON above is the terminal record for root to capture")
    print(json.dumps(result, sort_keys=True))
    return 0 if result["status"].startswith("COMPLETED") and not write_failed else 1

if __name__ == "__main__":
    sys.exit(main(sys.argv))
