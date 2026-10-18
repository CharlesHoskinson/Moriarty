#!/usr/bin/env python3
"""K10 compiled-C2 native wrapper operator (SOURCE ONLY, unexecuted).

Runs the already-compiled c2.c2.mint UPLC program, never a rebuilt
predicate, through the pinned Aiken 1.1.24 `uplc eval --cbor`
interface: exactly seven sequential calls, one per generated 2649-byte
PlutusV3 ScriptContext term. T1 must accept with `(con unit ())`;
T2..T7 must refuse only as exact structured EvaluationFailure (the
validator crashed / exited prematurely) with strict Costs. Any other
shape -- parse error, OutOfExError, signal, timeout, budget exhaustion
or unexplained trailing bytes -- fails closed and is preserved.

Finite guard labels come from the source-frozen EXPECTATIONS and the
claim.issue conjunct order plus the coherent positive control; the
runtime trace does not instrument named guards, and a typed-decode
failure could also surface as EvaluationFailure. No ledger, wallet,
key, signing, proof, setup, RNG or network effect exists here. The
fabricated 2222../555.. context, zero fee and unbound 2030 expiry stay
LOCAL FABRICATED / UNBOUND and support no Preprod claim. Local VM
cpu/mem are ExBudget::default() costs, not qualified ledger ExUnits.
"""
import hashlib, json, os, re, resource, signal, stat, subprocess, sys, time

BASE = "/home/charl/moriarty-eutxo-validation-2026-10-12"
LOCAL = BASE + "/current-cardano-k2-k10-compiled-wrapper-local-20261016"
OPERATOR_DIR = BASE + "/current-cardano-k2-k10-compiled-wrapper-operator-materialized-20261016"
RECORDS = BASE + "/current-cardano-k2-k10-compiled-wrapper-root-records-20261016"
AUTH_PATH = RECORDS + "/AUTH.json"
DATAONLY = BASE + "/aiken-fixed-compiler-release-discovery-20261015/v1.1.24-data-only"
GEN = BASE + "/current-cardano-k2-k10-public-inputs-generated-20261016"
CTX = "contexts/"
BINARY_SHA = "4257cc8be37cbde5bf7b5dd111e43a1277f45352fb2433ad7a7e647209aa7677"
BINARY_ENTRY_PATH = "compiler-data/aiken-x86_64-unknown-linux-musl/aiken"
RELEASE_DECLARED = "https://github.com/aiken-lang/aiken/releases/tag/v1.1.24"
RELEASE_TARGET_DECLARED = "bacbeb35fbc9f1db93cd2e1e93fb1d93053acb4a"
POLICY = "6a4b67e7d46dbada1b4f49dd324018324b60b13bebbeb6f33d9d59c1"
CAPS = {"wallSecondsPerCall": 60, "killAfterSeconds": 10,
        "parentWaitSecondsPerCall": 80, "cleanupWaitSeconds": 10,
        "cpuSoftSeconds": 30, "cpuHardSeconds": 35,
        "addressSpaceBytes": 8589934592, "coreSize": 0,
        "fileSizeBytes": 16777216, "maxStdoutBytes": 16777216,
        "maxStderrBytes": 16777216, "maxContextBytes": 8192,
        "plannedAggregateSeconds": 420, "stagingFinalReserveSeconds": 180,
        "wholeOperatorWallSeconds": 600, "wholeOperatorKillAfterSeconds": 10}
# Frozen from R2 EXPECTATIONS (source order). Guard labels are source
# expectations from claim.ak conjunct order, not runtime instrumentation.
EXPECTED = [
    ("K10-T1-positive-genuine-issue", CTX + "K10-T1-positive-genuine-issue.uplc.txt", "ACCEPT_UNIT", "none"),
    ("K10-T2-n1-body-digest-pairing-reject", CTX + "K10-T2-n1-body-digest-pairing-reject.uplc.txt", "SEMANTIC_EVALUATION_FAILURE", "pairing inside groth16.verify_digest (statement/body digest)"),
    ("K10-T3-n2-context-digest-pairing-reject", CTX + "K10-T3-n2-context-digest-pairing-reject.uplc.txt", "SEMANTIC_EVALUATION_FAILURE", "pairing inside groth16.verify_digest (statement/context digest)"),
    ("K10-T4-n3-proof-ac-swap-pairing-reject", CTX + "K10-T4-n3-proof-ac-swap-pairing-reject.uplc.txt", "SEMANTIC_EVALUATION_FAILURE", "pairing inside groth16.verify_digest (proof A/C swap)"),
    ("K10-T5-quantity-two-reject", CTX + "K10-T5-quantity-two-reject.uplc.txt", "SEMANTIC_EVALUATION_FAILURE", "qty == 1"),
    ("K10-T6-token-name-bitflip-reject", CTX + "K10-T6-token-name-bitflip-reject.uplc.txt", "SEMANTIC_EVALUATION_FAILURE", "name == envelope_name(raw)"),
    ("K10-T7-vk-digest-bitflip-reject", CTX + "K10-T7-vk-digest-bitflip-reject.uplc.txt", "SEMANTIC_EVALUATION_FAILURE", "env.vk_digest == blake2b_256(vk)")]
# role -> (absolute path, bytes, sha256): tool, its data manifest, and
# exactly the nine generated data inputs. All values from INPUT-INDEX.
PINS = {
    "aikenBinary": (DATAONLY + "/compiler-data/aiken-x86_64-unknown-linux-musl/aiken", 38299808, BINARY_SHA),
    "dataOnlyManifest": (DATAONLY + "/DATA-ONLY-MANIFEST.json", 1554399, "8adde843b7552bc7810f4be9cfca73693b8cc5f11fbb9e07d586901fd656d487"),
    "generatedManifest": (GEN + "/manifest.json", 10588, "10e83e23d0ab6c6f7d37cb6898e23fb473394eac5e42350dfdbd589a74ef5f11"),
    "scriptHex": (GEN + "/script.hex", 5541, "7068c0673286e9d506183106f39e34d24872c9c794f323d0113988ba7ff3139f"),
    "contextT1": (GEN + "/" + EXPECTED[0][1], 2649, "bad16e8d89d5ad47bc94965874ef3b243bee7ccb983a68f34499598b0d577b1e"),
    "contextT2": (GEN + "/" + EXPECTED[1][1], 2649, "055581434c12cdbf013cf594a8640159c02b47bcc63fe8637108959518a0b827"),
    "contextT3": (GEN + "/" + EXPECTED[2][1], 2649, "959e5a6995cee975720b7971bfbfce2a114773fa5ae103ad56815a4fea204446"),
    "contextT4": (GEN + "/" + EXPECTED[3][1], 2649, "dc8c8b9641b7a778893ec5baaad538cceb106265319cfedb1c7175436f6ccaa0"),
    "contextT5": (GEN + "/" + EXPECTED[4][1], 2649, "0a6d1a41b376790d097e05fab15a68a07c5b9ee5e7c7b977d261d420872ba98c"),
    "contextT6": (GEN + "/" + EXPECTED[5][1], 2649, "516e694891494b254ca5ba4420d91c27bd6945c7025189dc5c76c0bd903aeceb"),
    "contextT7": (GEN + "/" + EXPECTED[6][1], 2649, "1a1c4f864cad63c7f3eee9e691560e11008eef66ed46b16d48c4bb13a23cd212")}
INPUT_ROLES = ["generatedManifest", "scriptHex"] + ["contextT%d" % i for i in range(1, 8)]
AUTH_KEYS = {"format", "status", "attempts", "retries", "maxAikenCalls", "localRoot",
             "materializedOperatorDir", "rootRecordsDir", "operatorSha256",
             "inputAuditAttestation", "pins", "caps", "expectedCases"}
OPERATOR_FILES = ["runner/operator.py", "AUTH-CANDIDATE.json", "EXECUTION-PROPOSAL.json", "README.md", "DESIGN-POSITIONS.json"]
NEG_RE = re.compile(r"^\nError\n-----\n\n"
                    r"the validator crashed / exited prematurely\n\n"
                    r"\nCosts\n-----\ncpu: ([0-9]+)\nmemory: ([0-9]+)\n"
                    r"(?:\nLogs\n----\n(.*))?$", re.DOTALL)

class Fail(Exception):
    pass

def sha_bytes(b):
    return hashlib.sha256(b).hexdigest()

def is_int(x):
    return type(x) is int

def read_regular(path, want_bytes=None, max_bytes=None):
    st = os.lstat(path)
    if stat.S_ISLNK(st.st_mode) or not stat.S_ISREG(st.st_mode):
        raise Fail("not a regular non-symlink file: " + path)
    if want_bytes is not None and st.st_size != want_bytes:
        raise Fail("size mismatch: " + path)
    if max_bytes is not None and st.st_size > max_bytes:
        raise Fail("size exceeds bounded read cap: " + path)
    with open(path, "rb") as f:
        return f.read(max_bytes + 1) if max_bytes is not None else f.read()

def check_pin(role):
    path, n, want = PINS[role]
    data = read_regular(path, n)
    if sha_bytes(data) != want:
        raise Fail("sha256 mismatch: " + role)
    return data

def check_all_pins():
    for role in PINS:
        check_pin(role)
    return True

def operator_file_hashes():
    out = {}
    for rel in OPERATOR_FILES:
        out[rel] = sha_bytes(read_regular(OPERATOR_DIR + "/" + rel))
    return out

def load_auth(argv):
    if len(argv) != 2 or argv[1] != AUTH_PATH:
        raise Fail("usage: operator.py " + AUTH_PATH)
    try:
        auth = json.loads(read_regular(AUTH_PATH).decode("utf-8"))
    except Fail:
        raise
    except Exception as e:
        raise Fail("AUTH is not valid JSON: " + str(e))
    if not isinstance(auth, dict) or set(auth) != AUTH_KEYS:
        raise Fail("AUTH exact key set mismatch (13 top-level fields)")
    for k in ("format", "status", "localRoot", "materializedOperatorDir", "rootRecordsDir", "operatorSha256", "inputAuditAttestation"):
        if type(auth[k]) is not str:
            raise Fail("AUTH field must be a string: " + k)
    if auth["format"] != "cardano-k2-k10-compiled-wrapper-native-auth/1":
        raise Fail("AUTH format mismatch")
    if auth["status"] != "AUTHORIZED_EXACT_ONE_RUN":
        raise Fail("AUTH status is not an exact one-run authorisation (candidate never runs)")
    if auth["inputAuditAttestation"] != "ROOT_ATTESTS_FRESH_ACTUAL_INPUT_AUDIT_ACCEPTED_BEFORE_THIS_AUTH":
        raise Fail("AUTH lacks root attestation that the fresh actual-input audit accepted before this AUTH")
    if not (is_int(auth["attempts"]) and auth["attempts"] == 1):
        raise Fail("AUTH attempts must be integer 1")
    if not (is_int(auth["retries"]) and auth["retries"] == 0):
        raise Fail("AUTH retries must be integer 0")
    if not (is_int(auth["maxAikenCalls"]) and auth["maxAikenCalls"] == 7):
        raise Fail("AUTH maxAikenCalls must be integer 7")
    if auth["localRoot"] != LOCAL or auth["materializedOperatorDir"] != OPERATOR_DIR or auth["rootRecordsDir"] != RECORDS:
        raise Fail("AUTH root mismatch")
    caps = auth["caps"]
    if not isinstance(caps, dict) or set(caps) != set(CAPS) or any(not is_int(v) for v in caps.values()) or caps != CAPS:
        raise Fail("AUTH caps mismatch (exact keys, integer non-bool values, exact values)")
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
    cases = auth["expectedCases"]
    want = [{"id": i, "contextRelativePath": c, "expectedOutcome": o, "expectedGuard": g} for i, c, o, g in EXPECTED]
    if not isinstance(cases, list) or len(cases) != len(want):
        raise Fail("AUTH expectedCases must list exactly 7 cases")
    for e in cases:
        if not isinstance(e, dict) or set(e) != {"id", "contextRelativePath", "expectedOutcome", "expectedGuard"}:
            raise Fail("AUTH expectedCases entry exact key set mismatch")
        if any(type(e[k]) is not str for k in e):
            raise Fail("AUTH expectedCases entry values must be strings")
    if cases != want:
        raise Fail("AUTH expectedCases mismatch")
    self_path = os.path.abspath(__file__)
    if os.path.dirname(os.path.dirname(self_path)) != OPERATOR_DIR or os.path.basename(self_path) != "operator.py":
        raise Fail("operator is not at <materializedOperatorDir>/runner/operator.py")
    if auth["operatorSha256"] != sha_bytes(read_regular(self_path)):
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
                          "the binary was not shown rebuilt from indexed source, and no --version probe was run"}

def check_generated_manifest():
    doc = json.loads(check_pin("generatedManifest").decode("utf-8"))
    if not isinstance(doc, dict) or doc.get("format") != "cardano-k2-k10-public-inputs/1":
        raise Fail("generated manifest format mismatch")
    if doc.get("policyIdHex") != POLICY or doc.get("scriptHashIndependentlyRecomputedHex") != POLICY or doc.get("scriptHashClaimedHex") != POLICY:
        raise Fail("generated manifest policy / claimed / independently recomputed hash identity mismatch")
    if doc.get("scriptCompiledBytes") != 2770 or doc.get("scriptInnerFlatBytes") != 2767:
        raise Fail("generated manifest compiled/inner byte counts mismatch (2770/2767)")
    cases = doc.get("cases")
    if not isinstance(cases, list) or [c.get("id") for c in cases if isinstance(c, dict)] != [e[0] for e in EXPECTED]:
        raise Fail("generated manifest case ids do not equal the frozen seven in order")
    if [c.get("contextFile") for c in cases if isinstance(c, dict)] != [e[1] for e in EXPECTED]:
        raise Fail("generated manifest context files do not equal the frozen seven in order")
    return {"policyIdHex": POLICY, "compiledBytes": 2770, "innerFlatBytes": 2767,
            "hashBasis": "blake2b-224(0x03 || 2770 CBOR compiled bytes), one 590acf CBOR byte string, inner 2767 flat bytes not hashed; ordinary Pallas/Aiken inspection evidence, not a compile theorem"}

def read_context(role):
    data = read_regular(PINS[role][0], PINS[role][1], CAPS["maxContextBytes"])
    if sha_bytes(data) != PINS[role][2]:
        raise Fail("context sha256 mismatch: " + role)
    try:
        text = data.decode("utf-8")
    except Exception as e:
        raise Fail("context is not strict UTF-8: " + role + ": " + str(e))
    if text.endswith("\n"):
        text = text[:-1]  # exactly one trailing newline stripped
    if not text or "\n" in text or "\r" in text:
        raise Fail("context must be one non-empty line after exact newline strip: " + role)
    return text

def write_exclusive(path, data, mode=0o644):
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, mode)
    try:
        os.write(fd, data)
    finally:
        os.close(fd)

def stage_tool_and_inputs():
    # Root pre-stages the nine inputs read-only under LOCAL/inputs; this
    # operator only adds the fixed tool copy and fresh capture dirs, and
    # refuses any pre-existing tool/capture/logs/receipts path.
    inputs_src = {"generatedManifest": "manifest.json", "scriptHex": "script.hex"}
    for role in INPUT_ROLES:
        rel = inputs_src.get(role, EXPECTED[int(role[-1]) - 1][1])
        staged = LOCAL + "/inputs/" + rel
        data = read_regular(staged, PINS[role][1])
        if sha_bytes(data) != PINS[role][2]:
            raise Fail("staged input bytes differ from pin: " + role)
        if os.lstat(staged).st_mode & 0o222:
            raise Fail("staged input is writable; inputs must be read-only: " + role)
    for p in (LOCAL + "/tool", LOCAL + "/capture", LOCAL + "/logs", LOCAL + "/receipts"):
        if os.path.lexists(p):
            raise Fail("fresh local path already exists; refusing to overwrite: " + p)
    os.makedirs(LOCAL + "/tool")
    os.makedirs(LOCAL + "/capture")
    os.makedirs(LOCAL + "/logs")
    os.makedirs(LOCAL + "/receipts")
    tool = LOCAL + "/tool/aiken"
    write_exclusive(tool, check_pin("aikenBinary"), 0o600)
    os.chmod(tool, 0o555)
    if sha_bytes(read_regular(tool)) != BINARY_SHA or os.lstat(tool).st_mode & 0o777 != 0o555:
        raise Fail("staged tool hash/mode mismatch")
    if os.lstat(PINS["aikenBinary"][0]).st_mode & 0o777 != 0o644:
        raise Fail("source binary is no longer mode 644 data")
    return tool

def verify_custody(operator_before):
    check_all_pins()
    tool = LOCAL + "/tool/aiken"
    if sha_bytes(read_regular(tool)) != BINARY_SHA or os.lstat(tool).st_mode & 0o777 != 0o555:
        raise Fail("staged tool custody changed")
    if operator_file_hashes() != operator_before:
        raise Fail("operator source files changed during the run")
    return True

def bwrap_argv(term):
    return ["/usr/bin/timeout", "--signal=TERM", "--kill-after=" + str(CAPS["killAfterSeconds"]) + "s",
            str(CAPS["wallSecondsPerCall"]) + "s",
            "/usr/bin/prlimit", "--cpu=%d:%d" % (CAPS["cpuSoftSeconds"], CAPS["cpuHardSeconds"]),
            "--as=%d" % CAPS["addressSpaceBytes"], "--core=0", "--fsize=%d" % CAPS["fileSizeBytes"], "--",
            "/usr/bin/bwrap", "--unshare-all", "--die-with-parent", "--new-session", "--clearenv",
            "--setenv", "PATH", "/tool", "--setenv", "HOME", "/tmp", "--setenv", "TMPDIR", "/tmp",
            "--setenv", "NO_COLOR", "1", "--setenv", "LC_ALL", "C", "--setenv", "TZ", "UTC",
            "--ro-bind", LOCAL + "/tool/aiken", "/tool/aiken",
            "--ro-bind", LOCAL + "/inputs", "/inputs",
            "--bind", LOCAL + "/capture", "/capture",
            "--tmpfs", "/tmp", "--dev", "/dev", "--proc", "/proc", "--chdir", "/capture",
            "/tool/aiken", "uplc", "eval", "--cbor", "/inputs/script.hex", term]

def _signal_group(proc, sig):
    try:
        os.killpg(proc.pid, sig)
        return True
    except Exception:
        return False

def run_call(case, term, result):
    cid = case[0]
    out_path, err_path = LOCAL + "/logs/" + cid + ".stdout.log", LOCAL + "/logs/" + cid + ".stderr.log"
    argv = bwrap_argv(term)
    result["aikenLaunchAttempts"] += 1  # intent counted before Popen
    attempt = result["aikenLaunchAttempts"]
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    t0 = time.monotonic()
    with open(out_path, "wb") as out, open(err_path, "wb") as err:
        try:
            proc = subprocess.Popen(argv, stdin=subprocess.DEVNULL, stdout=out, stderr=err,
                                    cwd="/", env={"PATH": "/usr/bin:/bin"}, start_new_session=True)
        except Exception as e:
            result["incompleteCall"] = {"id": cid, "launchAttempt": attempt,
                                        "nativeExecutableStarted": "not_measured", "outcome": "spawn_failed_before_return"}
            raise Fail(cid + ": launch spawn failed after attempt was counted: " + str(e))
        try:
            rc = proc.wait(timeout=CAPS["parentWaitSecondsPerCall"])
        except subprocess.TimeoutExpired:
            actions = []
            if _signal_group(proc, signal.SIGTERM):
                actions.append("SIGTERM to timeout-leader process group")
            try:
                rc = proc.wait(timeout=CAPS["cleanupWaitSeconds"])
            except subprocess.TimeoutExpired:
                if _signal_group(proc, signal.SIGKILL):
                    actions.append("SIGKILL to timeout-leader process group")
                try:
                    rc = proc.wait(timeout=CAPS["cleanupWaitSeconds"])
                except subprocess.TimeoutExpired:
                    result["incompleteCall"] = {"id": cid, "launchAttempt": attempt,
                                                "nativeExecutableStarted": "not_measured",
                                                "outcome": "cleanup_wait_exhausted", "cleanupActions": actions}
                    raise Fail(cid + ": bounded cleanup wait exhausted; no hard-kill claim is made for uninterruptible (D-state) processes")
            result["incompleteCall"] = {"id": cid, "launchAttempt": attempt,
                                        "nativeExecutableStarted": "not_measured",
                                        "outcome": "parent_wait_exceeded", "cleanupActions": actions}
            raise Fail(cid + ": parent wait bound exceeded; signalled the timeout-leader process group; bwrap descendants are tied by --die-with-parent and its PID namespace, not by killpg alone")
    wall = time.monotonic() - t0
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    cpu = (after.ru_utime + after.ru_stime) - (before.ru_utime + before.ru_stime)
    for p, cap in ((out_path, CAPS["maxStdoutBytes"]), (err_path, CAPS["maxStderrBytes"])):
        if os.path.getsize(p) > cap:
            raise Fail(cid + ": log exceeds the 16 MiB cap: " + p)
    return {"id": cid, "argv": argv, "exitCode": rc, "wallSeconds": wall, "launchAttempt": attempt,
            "nativeExecutableStarted": "not_measured", "waitedChildrenCpuSecondsDelta": cpu,
            "stdout": out_path, "stderr": err_path, "stdoutBytes": os.path.getsize(out_path),
            "stderrBytes": os.path.getsize(err_path),
            "stdoutSha256": sha_bytes(read_regular(out_path)), "stderrSha256": sha_bytes(read_regular(err_path))}

def _costs(cpu_s, mem_s, label):
    cpu, mem = int(cpu_s), int(mem_s)
    if cpu <= 0 or mem <= 0:
        raise Fail(label + ": VM cpu/mem must be positive integers")
    return {"cpu": cpu, "mem": mem, "meaning": "local Aiken ExBudget::default() VM cost only; not qualified node/ledger ExUnits, fee, min-ADA or tx-fit"}

def validate_call(step, case):
    cid, _, outcome, guard = case
    raw_out, raw_err = read_regular(step["stdout"]), read_regular(step["stderr"])
    if outcome == "ACCEPT_UNIT":
        if step["exitCode"] != 0:
            raise Fail(cid + ": expected exit 0")
        if raw_err != b"":
            raise Fail(cid + ": expected empty stderr on success")
        try:
            doc = json.loads(raw_out.decode("utf-8"))
        except Exception as e:
            raise Fail(cid + ": stdout is not strict JSON: " + str(e))
        if not isinstance(doc, dict) or set(doc) != {"result", "cpu", "mem"}:
            raise Fail(cid + ": stdout JSON keys are not exactly result/cpu/mem")
        if doc["result"] != "(con unit ())":
            raise Fail(cid + ": result is not exactly (con unit ())")
        if not is_int(doc["cpu"]) or not is_int(doc["mem"]):
            raise Fail(cid + ": cpu/mem must be integers, not booleans")
        return {"observed": "ACCEPT_UNIT", "result": doc["result"], "costs": _costs(str(doc["cpu"]), str(doc["mem"]), cid)}
    if step["exitCode"] != 1:
        raise Fail(cid + ": expected exit exactly 1 for semantic refusal (any other nonzero/signal is not a refusal)")
    if raw_out != b"":
        raise Fail(cid + ": expected empty stdout on refusal")
    try:
        text = raw_err.decode("utf-8")
    except Exception as e:
        raise Fail(cid + ": stderr is not strict UTF-8: " + str(e))
    m = NEG_RE.match(text)
    if not m or m.end() != len(text):
        raise Fail(cid + ": stderr is not the exact eval.rs Error/EvaluationFailure/Costs shape (OutOfExError, syntax, crash and trailing content fail closed)")
    logs = m.group(3)
    if logs is not None and (not logs.endswith("\n") or len(logs.encode()) > CAPS["maxStderrBytes"]):
        raise Fail(cid + ": optional Logs section is malformed or unbounded")
    return {"observed": "SEMANTIC_EVALUATION_FAILURE", "errorText": "the validator crashed / exited prematurely",
            "costs": _costs(m.group(1), m.group(2), cid), "logsPresent": logs is not None,
            "expectedGuard": guard,
            "guardAttribution": "source-expected guard from claim.issue conjunct order plus the coherent positive control and minimal changed case; finite-predicate support only -- the runtime does not instrument named guards, and a typed-decode failure could also surface as EvaluationFailure"}

def main(argv):
    result = {"format": "cardano-k2-k10-compiled-wrapper-native-result/1", "status": "FAILED",
              "aikenLaunchAttempts": 0, "aikenCallsReturned": 0, "nativeExecutablesStarted": "not_measured",
              "attempts": 1, "retries": 0, "calls": [], "policyIdHex": POLICY}
    created = False
    custody_before = None
    try:
        load_auth(argv)
        check_all_pins()
        result["hostInputsUnchangedBefore"] = True
        result["releaseIdentityDeclared"] = parse_release_identity()
        result["generatedManifestIdentity"] = check_generated_manifest()
        custody_before = operator_file_hashes()
        terms = [read_context("contextT%d" % i) for i in range(1, 8)]
        stage_tool_and_inputs()
        created = True
        result["stagedCustodyVerifiedAtStaging"] = verify_custody(custody_before)
        for case, term in zip(EXPECTED, terms):
            step = run_call(case, term, result)
            step["validation"] = validate_call(step, case)
            result["calls"].append(step)
            result["aikenCallsReturned"] = len(result["calls"])
        if len(result["calls"]) != 7:
            raise Fail("exactly seven calls must return")
        result["status"] = "COMPLETED_AWAITING_FRESH_ACTUAL_RESULT_AUDIT"
    except Exception as e:
        result["error"] = str(e)
    if created:
        try:
            result["custodyUnchangedAfter"] = verify_custody(custody_before)
        except Exception as e:
            result["custodyUnchangedAfter"] = False
            result["custodyPostcheckError"] = str(e)
            if result["status"].startswith("COMPLETED"):
                result["status"] = "FAILED"
        try:
            write_exclusive(LOCAL + "/receipts/RESULT.json", (json.dumps(result, indent=2, sort_keys=True) + "\n").encode())
        except Exception as e:
            result["status"] = "FAILED"
            result["resultWriteError"] = str(e)
    else:
        result["resultFileWritten"] = False
        result["resultFileNote"] = "no RESULT.json written because the fresh local staging was not created (AUTH/pin/input failure); the stdout JSON below is the terminal record for root to capture"
    print(json.dumps(result, sort_keys=True))
    return 0 if result["status"].startswith("COMPLETED") else 1

if __name__ == "__main__":
    sys.exit(main(sys.argv))
