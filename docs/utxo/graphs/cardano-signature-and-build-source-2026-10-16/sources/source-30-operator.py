#!/usr/bin/env python3
"""Two-phase operator for the genuine compiled-VI experiment (SOURCE ONLY).

Phase A (one attempt, zero retries): at most ONE offline Rust helper build
(every compiler subprocess is reported as part of that single build, never
disguised as one native op), then exactly TWO helper calls, one fresh key
and one signature per scheme, each positively Core-verified once inside
the helper. Phase A stops at
COMPLETED_AWAITING_FRESH_ACTUAL_PUBLIC_INPUT_AUDIT. Ordinary
runner/build_inputs.py then runs as a separate data-only step, a fresh
audit of the actual produced public inputs must accept them, and only a
new Phase B AUTH may authorize Phase B: exactly eight sequential
already-compiled VI evaluations, zero retries, using the pinned argv
/tool/aiken uplc eval --cbor /inputs/script.hex '<single context term>'
with no --flat/--debug or guessed flag.

This candidate refuses every PENDING_UNMATERIALIZED AUTH, source, tool or
build pin. It therefore cannot run as shipped: source completeness is not
operator readiness. No wallet, existing key, proving key, SRS, provider
file, /w tree or host-home scan is ever touched; only exact public named
inputs in the AUTH are read. Old K6/K10 authorities are spent and are
never reused or rerun.
"""
import hashlib, json, os, re, resource, signal, stat, subprocess, sys, time

PENDING = "PENDING_UNMATERIALIZED"
CAPS_A = {"phaseAWallSeconds": 480, "buildWallSeconds": 300, "helperWallSecondsEach": 30, "phaseAReserveSeconds": 120, "killAfterSeconds": 10, "parentWaitSeconds": 80, "cleanupWaitSeconds": 10, "cpuSoftSeconds": 30, "cpuHardSeconds": 35, "addressSpaceBytes": 8589934592, "coreSize": 0, "fileSizeBytes": 16777216, "maxStdoutBytes": 16777216, "maxStderrBytes": 1048576}
CAPS_B = {"phaseBWallSeconds": 600, "perCallWallSeconds": 60, "reserveSeconds": 120, "killAfterSeconds": 10, "parentWaitSecondsPerCall": 80, "cleanupWaitSeconds": 10, "cpuSoftSeconds": 30, "cpuHardSeconds": 35, "addressSpaceBytes": 8589934592, "coreSize": 0, "fileSizeBytes": 16777216, "maxStdoutBytes": 16777216, "maxStderrBytes": 16777216, "maxContextBytes": 262144}
EXPECTED = [
    ("VI-P1-schnorr-positive", "ACCEPT_UNIT", "none"),
    ("VI-P2-ecdsa-positive", "ACCEPT_UNIT", "none"),
    ("VI-N1-schnorr-signature-bitflip", "SEMANTIC_EVALUATION_FAILURE", "intent.verify Schnorr builtin"),
    ("VI-N2-ecdsa-signature-bitflip", "SEMANTIC_EVALUATION_FAILURE", "intent.verify ECDSA builtin"),
    ("VI-N3-stale-seq-nonce", "SEMANTIC_EVALUATION_FAILURE", "nonce/seq binding in rebuilt signing message"),
    ("VI-N4-domain-predicate-bitflip", "SEMANTIC_EVALUATION_FAILURE", "domain/2 target_hash binding"),
    ("VI-N5-window-exceeds-intent", "SEMANTIC_EVALUATION_FAILURE", "intent.window_ok"),
    ("VI-N6-native-coauthority-absent", "SEMANTIC_EVALUATION_FAILURE", "settle_intent_native"),
]
NEG_RE = re.compile(r"^\nError\n-----\n\nthe validator crashed / exited prematurely\n\n\nCosts\n-----\ncpu: ([0-9]+)\nmemory: ([0-9]+)\n(?:\nLogs\n----\n(.*))?$", re.DOTALL)

class Fail(Exception): pass
def sha(b): return hashlib.sha256(b).hexdigest()
def is_int(x): return type(x) is int
def read_regular(path, want=None, cap=None):
    st = os.lstat(path)
    if stat.S_ISLNK(st.st_mode) or not stat.S_ISREG(st.st_mode): raise Fail("not a regular non-symlink file: " + path)
    if want is not None and st.st_size != want: raise Fail("size mismatch: " + path)
    if cap is not None and st.st_size > cap: raise Fail("size over cap: " + path)
    with open(path, "rb") as f: return f.read()

def load_auth(path):
    try: auth = json.loads(read_regular(path, cap=1048576).decode("utf-8"))
    except Fail: raise
    except Exception as e: raise Fail("AUTH is not valid JSON: " + str(e))
    if not isinstance(auth, dict): raise Fail("AUTH must be an object")
    blob = json.dumps(auth)
    if PENDING in blob or "PLACEHOLDER" in blob: raise Fail("AUTH contains PENDING_UNMATERIALIZED pins; candidate operator refuses unmaterialized authority, source or build pins")
    if auth.get("attempts") != 1 or auth.get("retries") != 0: raise Fail("AUTH must be exactly one attempt and zero retries")
    phase = auth.get("phase")
    if phase == "A":
        if auth.get("status") != "AUTHORIZED_EXACT_ONE_PHASE_A_PRODUCE": raise Fail("Phase A AUTH status mismatch")
        if auth.get("maxRustBuilds") != 1 or auth.get("maxHelperCalls") != 2: raise Fail("Phase A AUTH must allow exactly one build and two helper calls")
        if auth.get("entropyDevice") != "/dev/urandom": raise Fail("Phase A AUTH entropy device mismatch")
        if auth.get("caps") != CAPS_A: raise Fail("Phase A AUTH caps mismatch")
    elif phase == "B":
        if auth.get("status") != "AUTHORIZED_EXACT_ONE_PHASE_B_EVALUATE_8": raise Fail("Phase B AUTH status mismatch")
        if auth.get("maxAikenCalls") != 8: raise Fail("Phase B AUTH must allow exactly eight VI calls")
        if auth.get("inputAuditAttestation") != "ROOT_ATTESTS_FRESH_ACTUAL_PUBLIC_INPUT_AUDIT_ACCEPTED_BEFORE_THIS_AUTH": raise Fail("Phase B requires the fresh actual public-input audit attestation")
        if auth.get("caps") != CAPS_B: raise Fail("Phase B AUTH caps mismatch")
        cases = auth.get("expectedCases")
        want = [{"id": i, "expectedOutcome": o, "expectedGuard": g} for i, o, g in EXPECTED]
        if cases != want: raise Fail("Phase B AUTH expectedCases mismatch")
    else: raise Fail("AUTH phase must be A or B; phases never auto-chain")
    for role, pin in auth.get("pins", {}).items():
        if not isinstance(pin, dict) or set(pin) != {"path", "bytes", "sha256"}: raise Fail("pin shape mismatch: " + role)
        data = read_regular(pin["path"], pin["bytes"])
        if sha(data) != pin["sha256"]: raise Fail("pin hash mismatch: " + role)
    return auth

def bwrap(tool_argv, local, writable_rel, caps, wall_key):
    return ["/usr/bin/timeout", "--signal=TERM", "--kill-after=%ds" % caps["killAfterSeconds"], "%ds" % caps[wall_key],
            "/usr/bin/prlimit", "--cpu=%d:%d" % (caps["cpuSoftSeconds"], caps["cpuHardSeconds"]), "--as=%d" % caps["addressSpaceBytes"], "--core=0", "--fsize=%d" % caps["fileSizeBytes"], "--",
            "/usr/bin/bwrap", "--unshare-all", "--die-with-parent", "--new-session", "--clearenv",
            "--setenv", "PATH", "/tool:/usr/bin:/bin", "--setenv", "HOME", "/tmp", "--setenv", "TMPDIR", "/tmp", "--setenv", "RUST_BACKTRACE", "0", "--setenv", "NO_COLOR", "1", "--setenv", "LC_ALL", "C", "--setenv", "TZ", "UTC",
            "--ro-bind", local + "/tool", "/tool", "--ro-bind", local + "/inputs", "/inputs", "--bind", local + "/" + writable_rel, "/" + writable_rel,
            "--tmpfs", "/tmp", "--dev", "/dev", "--proc", "/proc", "--chdir", "/" + writable_rel] + tool_argv

def run_counted(argv, result, kind, out_path, err_path, caps):
    result[kind + "LaunchAttempts"] = result.get(kind + "LaunchAttempts", 0) + 1  # intent before Popen
    t0 = time.monotonic()
    with open(out_path, "wb") as out, open(err_path, "wb") as err:
        try: proc = subprocess.Popen(argv, stdin=subprocess.DEVNULL, stdout=out, stderr=err, cwd="/", env={"PATH": "/usr/bin:/bin"}, start_new_session=True)
        except Exception as e:
            result["incompleteCall"] = {"kind": kind, "outcome": "spawn_failed_before_return"}; raise Fail(kind + " spawn failed after attempt counted: " + str(e))
        try: rc = proc.wait(timeout=caps["parentWaitSeconds"] if "parentWaitSeconds" in caps else caps["parentWaitSecondsPerCall"])
        except subprocess.TimeoutExpired:
            try: os.killpg(proc.pid, signal.SIGTERM)
            except Exception: pass
            try: rc = proc.wait(timeout=caps["cleanupWaitSeconds"])
            except subprocess.TimeoutExpired:
                result["incompleteCall"] = {"kind": kind, "outcome": "cleanup_wait_exhausted"}; raise Fail(kind + " cleanup wait exhausted; no hard-kill claim for D-state processes")
            result["incompleteCall"] = {"kind": kind, "outcome": "parent_wait_exceeded"}; raise Fail(kind + " parent wait exceeded")
    return {"argv": argv, "exitCode": rc, "wallSeconds": time.monotonic() - t0, "stdoutBytes": os.path.getsize(out_path), "stderrBytes": os.path.getsize(err_path), "stdoutSha256": sha(read_regular(out_path)), "stderrSha256": sha(read_regular(err_path))}

def validate_positive(step):
    if step["exitCode"] != 0: raise Fail("positive: expected exit 0")
    return step
def validate_call(step, case, local):
    raw_out, raw_err = read_regular(local + "/logs/%s.stdout.log" % case[0]), read_regular(local + "/logs/%s.stderr.log" % case[0])
    if case[1] == "ACCEPT_UNIT":
        if step["exitCode"] != 0 or raw_err != b"": raise Fail(case[0] + ": expected exit 0 and empty stderr")
        doc = json.loads(raw_out.decode("utf-8"))
        if set(doc) != {"result", "cpu", "mem"} or doc["result"] != "(con unit ())" or not is_int(doc["cpu"]) or not is_int(doc["mem"]) or doc["cpu"] <= 0 or doc["mem"] <= 0: raise Fail(case[0] + ": positive parser mismatch")
        return {"observed": "ACCEPT_UNIT", "costs": {"cpu": doc["cpu"], "mem": doc["mem"], "meaning": "local Aiken VM cost only; not qualified ledger ExUnits or fees"}}
    if step["exitCode"] != 1 or raw_out != b"": raise Fail(case[0] + ": negative must be exit exactly 1 with empty stdout; bad decode, budget, signal and timeout shapes are not crypto rejects")
    m = NEG_RE.match(raw_err.decode("utf-8"))
    if not m: raise Fail(case[0] + ": stderr is not the exact semantic EvaluationFailure shape")
    return {"observed": "SEMANTIC_EVALUATION_FAILURE", "costs": {"cpu": int(m.group(1)), "mem": int(m.group(2))}, "expectedGuard": case[2], "guardAttribution": "SOURCE EXPECTED only; the runtime does not instrument named guards"}

def main(argv):
    result = {"format": "moriarty-genuine-vi-experiment-result/1", "status": "FAILED", "attempts": 1, "retries": 0, "calls": []}
    try:
        if len(argv) != 2: raise Fail("usage: operator.py <AUTH.json>")
        auth = load_auth(argv[1]); local = auth["localRoot"]
        if auth["phase"] == "A":
            result["buildLaunchAttempts"] = 0; result["helperLaunchAttempts"] = 0
            # Exactly one build invocation is launched by this operator;
            # cargo/rustc subprocesses inside it are transitive effects of
            # that build and are reported as such, never as one native op.
            build = run_counted(bwrap(["/tool/cargo", "build", "--offline", "--locked", "--release", "-p", "moriarty-vi-signer"], local, "capture", CAPS_A, "buildWallSeconds"), result, "build", local + "/logs/build.stdout.log", local + "/logs/build.stderr.log", CAPS_A)
            result["build"] = dict(build, transitiveCompilerSubprocesses="reported as effects of the single cargo build; count not measured as native ops")
            if build["exitCode"] != 0: raise Fail("helper build failed")
            for scheme in ("schnorr_bip340", "ecdsa_secp256k1_sha256"):
                step = run_counted(bwrap(["/tool/moriarty-vi-signer", scheme], local, "capture", CAPS_A, "helperWallSecondsEach"), result, "helper", local + "/logs/helper-%s.stdout.log" % scheme, local + "/logs/helper-%s.stderr.log" % scheme, CAPS_A)
                if step["exitCode"] != 0: raise Fail("helper failed for " + scheme + "; no entropy or key-generation retry")
                out = json.loads(read_regular(local + "/logs/helper-%s.stdout.log" % scheme).decode("utf-8"))
                if out.get("core_verify", {}).get("status") != "ok": raise Fail("helper Core verify not honestly ok for " + scheme)
                result["calls"].append(dict(step, scheme=scheme, producerOutputSha256=sha(json.dumps(out, sort_keys=True).encode())))
            if result["helperLaunchAttempts"] != 2: raise Fail("exactly two helper calls required")
            result["status"] = "COMPLETED_AWAITING_FRESH_ACTUAL_PUBLIC_INPUT_AUDIT"
        else:
            result["aikenLaunchAttempts"] = 0; result["aikenCallsReturned"] = 0; result["aikenCallsValidated"] = 0
            script = read_regular(local + "/inputs/script.hex").decode("utf-8").strip()
            _ = script  # script is passed by pinned path /inputs/script.hex, never by value
            for case in EXPECTED:
                term = read_regular(local + "/inputs/contexts/%s.uplc.txt" % case[0], cap=CAPS_B["maxContextBytes"]).decode("utf-8")
                if term.endswith("\n"): term = term[:-1]
                if not term or "\n" in term: raise Fail("context must be one line: " + case[0])
                step = run_counted(bwrap(["/tool/aiken", "uplc", "eval", "--cbor", "/inputs/script.hex", term], local, "capture", CAPS_B, "perCallWallSeconds"), result, "aiken", local + "/logs/%s.stdout.log" % case[0], local + "/logs/%s.stderr.log" % case[0], CAPS_B)
                result["calls"].append(step); result["aikenCallsReturned"] = len(result["calls"])  # raw returned step appended BEFORE validation
                step["validationStatus"] = "PENDING_VALIDATION"; step["validationError"] = None
                try: step["validation"] = validate_call(step, case, local)
                except Exception:
                    step["validationStatus"] = "FAILED_VALIDATION"; step["validationError"] = str(sys.exc_info()[1]); raise
                step["validationStatus"] = "VALIDATED"; result["aikenCallsValidated"] = sum(1 for c in result["calls"] if c.get("validationStatus") == "VALIDATED")
            if result["aikenCallsReturned"] != 8 or result["aikenCallsValidated"] != 8: raise Fail("exactly eight returned and eight validated calls required")
            result["status"] = "COMPLETED_AWAITING_FRESH_ACTUAL_RESULT_AUDIT"
    except Exception as e: result["error"] = str(e)
    print(json.dumps(result, sort_keys=True))
    return 0 if result["status"].startswith("COMPLETED") else 1

if __name__ == "__main__": sys.exit(main(sys.argv))
