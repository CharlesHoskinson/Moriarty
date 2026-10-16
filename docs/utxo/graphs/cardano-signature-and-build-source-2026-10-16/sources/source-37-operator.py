#!/usr/bin/env python3
"""Two-phase operator for the genuine compiled-VI experiment (SOURCE ONLY).

Phase A (one attempt, zero retries): typed staging of the exact public
inputs (hash verified before and after copy; staged-set equality
checked), then ONE offline Rust helper build (every compiler
subprocess inside it is a transitive effect of that single build,
never disguised as one native operation, and never counted as one),
then exactly TWO helper calls: one invocation per scheme, each
creating two fresh ephemeral keys (escrow, the debit signer, and
seller, the invoice-discharge creditor required by
thread.required_signers) and producing two genuine signatures. Phase A
stops at COMPLETED_AWAITING_FRESH_ACTUAL_PUBLIC_INPUT_AUDIT. Ordinary
runner/build_inputs.py then runs as a separate data-only step; a fresh
audit of the actual produced public inputs must accept them; only a
new Phase B AUTH may then authorize Phase B: exactly eight sequential
already-compiled VI evaluations, zero retries, using
/tool/bin/aiken uplc eval --cbor /inputs/script.hex '<one context term>'
with no --flat/--debug or guessed flag.

Layout contract (AUTH.localRoot, never /w):
  localRoot/tool/bin/{cargo,rustc,cc,aiken}   pinned tool bodies (ro bind /tool)
  localRoot/inputs/...                        pinned public inputs (ro bind /inputs)
  localRoot/capture/                          the ONLY writable bind (/capture)
  capture/work/moriarty/...                   staged frozen Core workspace
  capture/work/signer/{Cargo.toml,Cargo.lock,src/main.rs}
  capture/work/target/release/moriarty-vi-signer   the one build artifact
Every counting channel keeps launch attempts, returned calls and
validated calls separate, and every returned raw call is appended to
result.calls BEFORE its output is parsed or validated, in both phases.
Phase deadlines use one monotonic clock with a reserve; each launch
wall is min(per-call wall, remaining phase budget); the parent wait is
wall + killAfter + cleanup + 5 seconds. Per-process CPU and address
space caps are per process: transitive compiler subprocess CPU and
uninterruptible D-state death are NOT claimed to be bounded. This
candidate refuses every PENDING_UNMATERIALIZED pin: source
completeness is not operator readiness. Nothing under /w is ever
stat'ed, read, hashed or walked; no wallet, existing key, proving
key, SRS or provider file is touched on any path.
"""
import hashlib, json, os, re, shutil, stat, subprocess, sys, time
from copy import deepcopy
from pathlib import Path

PENDING = "PENDING_UNMATERIALIZED"
CAPS_A = {"phaseWallSeconds": 480, "buildWallSeconds": 300, "helperWallSecondsEach": 30, "reserveSeconds": 120, "killAfterSeconds": 10, "cleanupWaitSeconds": 10, "cpuSoftSeconds": 30, "cpuHardSeconds": 35, "addressSpaceBytes": 8589934592, "coreSize": 0, "fileSizeBytes": 16777216, "maxStdoutBytes": 16777216, "maxStderrBytes": 16777216}
CAPS_B = {"phaseWallSeconds": 600, "perCallWallSeconds": 60, "reserveSeconds": 120, "killAfterSeconds": 10, "cleanupWaitSeconds": 10, "cpuSoftSeconds": 30, "cpuHardSeconds": 35, "addressSpaceBytes": 8589934592, "coreSize": 0, "fileSizeBytes": 16777216, "maxStdoutBytes": 16777216, "maxStderrBytes": 16777216, "maxContextBytes": 262144}
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
ROLE_PATHS_A = {"toolCargo": "tool/bin/cargo", "toolRustc": "tool/bin/rustc", "toolLinker": "tool/bin/cc", "signerCargoToml": "inputs/signer/Cargo.toml", "signerMainRs": "inputs/signer/src/main.rs", "signerCargoLock": "inputs/signer/Cargo.lock", "invoiceMori": "inputs/installment-invoice.mori", "invoiceRequest": "inputs/invoice-partial.request.json"}
ROLE_PATHS_B = {"toolAiken": "tool/bin/aiken", "scriptHex": "inputs/script.hex", "inputsManifest": "inputs/manifest.json"}

class Fail(Exception): pass
def sha(b): return hashlib.sha256(b).hexdigest()
def is_int(x): return type(x) is int
def _no_dup(pairs):
    out = {}
    for k, v in pairs:
        if k in out: raise Fail("duplicate JSON key: " + str(k))
        out[k] = v
    return out
def _no_const(x): raise Fail("non-finite JSON constant: " + x)
def load_strict_bytes(data, what):
    try: return json.loads(data.decode("utf-8"), object_pairs_hook=_no_dup, parse_constant=_no_const)
    except Fail: raise
    except Exception as e: raise Fail(what + " is not strict JSON: " + str(e))
def read_regular(path, want=None, cap=None):
    st = os.lstat(path)
    if stat.S_ISLNK(st.st_mode) or not stat.S_ISREG(st.st_mode): raise Fail("not a regular non-symlink file: " + path)
    if want is not None and st.st_size != want: raise Fail("size mismatch: " + path)
    if cap is not None and st.st_size > cap: raise Fail("size over cap: " + path)
    with open(path, "rb") as f: return f.read()

def check_local_root(auth):
    local = auth.get("localRoot")
    if type(local) is not str or not local.startswith("/") or local == "/w" or local.startswith("/w/"): raise Fail("localRoot must be an absolute non-/w path")
    if ".." in Path(local).parts: raise Fail("localRoot must not contain ..")
    return local
def check_pins(auth, local, role_paths):
    pins = auth.get("pins")
    if not isinstance(pins, dict): raise Fail("AUTH pins must be an object")
    for role, rel in role_paths.items():
        pin = pins.get(role)
        if not isinstance(pin, dict) or set(pin) != {"path", "bytes", "sha256"}: raise Fail("pin shape mismatch: " + role)
        if pin["path"] != local + "/" + rel: raise Fail("pin path is not the typed layout path: " + role)
        if not is_int(pin["bytes"]) or type(pin["sha256"]) is not str: raise Fail("pin field types mismatch: " + role)
        data = read_regular(pin["path"], pin["bytes"])
        if sha(data) != pin["sha256"]: raise Fail("pin hash mismatch: " + role)
    return pins
def check_tools(local, roles):
    for role, rel in roles.items():
        st = os.lstat(local + "/" + rel)
        if stat.S_ISLNK(st.st_mode) or not stat.S_ISREG(st.st_mode) or not (st.st_mode & 0o111): raise Fail("tool is not a regular executable file: " + role)

def load_auth(path):
    auth = load_strict_bytes(read_regular(path, cap=4194304), "AUTH")
    if not isinstance(auth, dict): raise Fail("AUTH must be an object")
    blob = json.dumps(auth)
    if PENDING in blob or "PLACEHOLDER" in blob: raise Fail("AUTH contains PENDING_UNMATERIALIZED pins; candidate operator refuses unmaterialized authority, source or build pins")
    if auth.get("format") != "moriarty-genuine-vi-auth/2": raise Fail("AUTH format mismatch")
    if auth.get("attempts") != 1 or auth.get("retries") != 0 or type(auth.get("attempts")) is bool: raise Fail("AUTH must be exactly one attempt and zero retries")
    local = check_local_root(auth)
    phase = auth.get("phase")
    if phase == "A":
        if auth.get("status") != "AUTHORIZED_EXACT_ONE_PHASE_A_PRODUCE": raise Fail("Phase A AUTH status mismatch")
        if auth.get("maxRustBuilds") != 1 or auth.get("maxHelperCalls") != 2: raise Fail("Phase A AUTH must allow exactly one build and two helper calls")
        if auth.get("entropyDevice") != "/dev/urandom": raise Fail("Phase A AUTH entropy device mismatch")
        if auth.get("caps") != CAPS_A: raise Fail("Phase A AUTH caps mismatch")
        check_pins(auth, local, ROLE_PATHS_A); check_tools(local, {k: v for k, v in ROLE_PATHS_A.items() if k.startswith("tool")})
        files = auth.get("moriartyFiles")
        if not isinstance(files, list) or not (1 <= len(files) <= 256): raise Fail("AUTH moriartyFiles must list the staged Core workspace files")
        seen_in, seen_out = set(), set()
        for e in files:
            if not isinstance(e, dict) or set(e) != {"inputsRel", "stagedRel", "bytes", "sha256"}: raise Fail("moriartyFiles entry shape mismatch")
            if not str(e["inputsRel"]).startswith("inputs/moriarty/") or not str(e["stagedRel"]).startswith("work/moriarty/"): raise Fail("moriartyFiles entry paths outside the typed staging layout")
            if ".." in Path(e["inputsRel"]).parts or ".." in Path(e["stagedRel"]).parts: raise Fail("moriartyFiles entry contains ..")
            if e["inputsRel"] in seen_in or e["stagedRel"] in seen_out: raise Fail("moriartyFiles entry duplicated")
            seen_in.add(e["inputsRel"]); seen_out.add(e["stagedRel"])
            data = read_regular(local + "/" + e["inputsRel"], e["bytes"])
            if sha(data) != e["sha256"]: raise Fail("moriartyFiles hash mismatch: " + e["inputsRel"])
        if "inputs/moriarty/Cargo.toml" not in seen_in or "inputs/moriarty/crates/mori-core/src/intent.rs" not in seen_in: raise Fail("moriartyFiles must include the workspace manifest and Core intent source")
    elif phase == "B":
        if auth.get("status") != "AUTHORIZED_EXACT_ONE_PHASE_B_EVALUATE_8": raise Fail("Phase B AUTH status mismatch")
        if auth.get("maxAikenCalls") != 8: raise Fail("Phase B AUTH must allow exactly eight VI calls")
        if auth.get("inputAuditAttestation") != "ROOT_ATTESTS_FRESH_ACTUAL_PUBLIC_INPUT_AUDIT_ACCEPTED_BEFORE_THIS_AUTH": raise Fail("Phase B requires the fresh actual public-input audit attestation")
        if auth.get("caps") != CAPS_B: raise Fail("Phase B AUTH caps mismatch")
        want = [{"id": i, "expectedOutcome": o, "expectedGuard": g} for i, o, g in EXPECTED]
        if auth.get("expectedCases") != want: raise Fail("Phase B AUTH expectedCases mismatch")
        paths = dict(ROLE_PATHS_B)
        for cid, _, _ in EXPECTED: paths["context:" + cid] = "inputs/contexts/%s.uplc.txt" % cid
        check_pins(auth, local, paths); check_tools(local, {"toolAiken": ROLE_PATHS_B["toolAiken"]})
        manifest = load_strict_bytes(read_regular(local + "/inputs/manifest.json", cap=1048576), "inputs manifest")
        if not isinstance(manifest, dict) or manifest.get("format") != "moriarty-genuine-vi-public-inputs/2": raise Fail("Phase B inputs manifest format mismatch")
        got = [{"id": c.get("id"), "expectedOutcome": c.get("expectedOutcome"), "expectedGuard": c.get("expectedGuard")} for c in manifest.get("cases", []) if isinstance(c, dict)]
        want_manifest = [{"id": i, "expectedOutcome": o, "expectedGuard": g} for (i, o, g), (_, _, _) in zip(EXPECTED, EXPECTED)]
        # Guard strings in the manifest carry the (source-expected only) suffix; compare ids and outcomes exactly.
        if [c["id"] for c in got] != [w["id"] for w in want_manifest] or [c["expectedOutcome"] for c in got] != [w["expectedOutcome"] for w in want_manifest]: raise Fail("Phase B inputs manifest cases mismatch AUTH expectations")
    else: raise Fail("AUTH phase must be A or B; phases never auto-chain")
    return auth

def stage_file(local, inputs_rel, staged_rel, want_bytes, want_sha):
    data = read_regular(local + "/" + inputs_rel, want_bytes)
    if sha(data) != want_sha: raise Fail("staging source hash mismatch: " + inputs_rel)
    dest = Path(local + "/capture/" + staged_rel)
    dest.parent.mkdir(parents=True, exist_ok=True)
    if dest.exists() or dest.is_symlink(): raise Fail("staging destination already exists: " + staged_rel)
    with open(dest, "wb") as f: f.write(data)
    again = read_regular(str(dest), want_bytes)
    if sha(again) != want_sha: raise Fail("staged copy hash mismatch: " + staged_rel)

def bwrap(local, tool_argv, wall):
    return ["/usr/bin/timeout", "--signal=TERM", "--kill-after=%ds" % CAPS_A["killAfterSeconds"], "%ds" % int(wall),
            "/usr/bin/prlimit", "--cpu=%d:%d" % (CAPS_A["cpuSoftSeconds"], CAPS_A["cpuHardSeconds"]), "--as=%d" % CAPS_A["addressSpaceBytes"], "--core=0", "--fsize=%d" % CAPS_A["fileSizeBytes"], "--",
            "/usr/bin/bwrap", "--unshare-all", "--die-with-parent", "--new-session", "--clearenv",
            "--setenv", "PATH", "/tool/bin", "--setenv", "HOME", "/tmp", "--setenv", "TMPDIR", "/tmp", "--setenv", "RUST_BACKTRACE", "0", "--setenv", "NO_COLOR", "1", "--setenv", "LC_ALL", "C", "--setenv", "TZ", "UTC",
            "--setenv", "CARGO_HOME", "/capture/work/cargo-home", "--setenv", "CARGO_TARGET_DIR", "/capture/work/target", "--setenv", "CARGO_NET_OFFLINE", "true", "--setenv", "RUSTC", "/tool/bin/rustc", "--setenv", "CC", "/tool/bin/cc",
            "--ro-bind", local + "/tool", "/tool", "--ro-bind", local + "/inputs", "/inputs", "--bind", local + "/capture", "/capture",
            "--tmpfs", "/tmp", "--dev", "/dev", "--proc", "/proc", "--chdir", "/capture/work"] + tool_argv

def run_counted(argv, result, kind, out_path, err_path, caps, wall, cleanup):
    counts = result["counters"][kind]
    counts["attempts"] += 1  # launch intent recorded BEFORE Popen
    t0 = time.monotonic()
    parent_wait = int(wall) + caps["killAfterSeconds"] + cleanup + 5
    with open(out_path, "wb") as out, open(err_path, "wb") as err:
        try: proc = subprocess.Popen(argv, stdin=subprocess.DEVNULL, stdout=out, stderr=err, cwd="/", env={"PATH": "/usr/bin:/bin"}, start_new_session=True)
        except Exception as e:
            result["incompleteCall"] = {"kind": kind, "outcome": "spawn_failed_before_return"}
            raise Fail(kind + " spawn failed after attempt counted: " + str(e))
        try: rc = proc.wait(timeout=parent_wait)
        except subprocess.TimeoutExpired:
            try: os.killpg(proc.pid, signal.SIGTERM) if False else None
            except Exception: pass
            import signal as _sig
            try: os.killpg(proc.pid, _sig.SIGTERM)
            except Exception: pass
            try: rc = proc.wait(timeout=cleanup)
            except subprocess.TimeoutExpired:
                result["incompleteCall"] = {"kind": kind, "outcome": "cleanup_wait_exhausted"}
                raise Fail(kind + " cleanup wait exhausted; no hard-kill claim for D-state processes")
            result["incompleteCall"] = {"kind": kind, "outcome": "parent_wait_exceeded"}
            raise Fail(kind + " parent wait exceeded")
    step = {"kind": kind, "argv": argv, "exitCode": rc, "wallSeconds": time.monotonic() - t0, "parentWaitSeconds": parent_wait, "stdoutBytes": os.path.getsize(out_path), "stderrBytes": os.path.getsize(err_path), "stdoutSha256": sha(read_regular(out_path)), "stderrSha256": sha(read_regular(err_path)), "validationStatus": "PENDING_VALIDATION"}
    counts["returned"] += 1
    return step

def enforce_output_caps(step, caps):
    if step["stdoutBytes"] > caps["maxStdoutBytes"]: raise Fail(step["kind"] + " stdout exceeds its distinct byte cap")
    if step["stderrBytes"] > caps["maxStderrBytes"]: raise Fail(step["kind"] + " stderr exceeds its distinct byte cap")

def remaining_wall(deadline, reserve):
    return deadline - time.monotonic() - reserve

def validate_call(step, case, local):
    raw_out, raw_err = read_regular(local + "/logs/%s.stdout.log" % case[0]), read_regular(local + "/logs/%s.stderr.log" % case[0])
    if case[1] == "ACCEPT_UNIT":
        if step["exitCode"] != 0 or raw_err != b"": raise Fail(case[0] + ": expected exit 0 and empty stderr")
        doc = load_strict_bytes(raw_out, case[0] + " stdout")
        if not isinstance(doc, dict) or set(doc) != {"result", "cpu", "mem"} or doc["result"] != "(con unit ())" or not is_int(doc["cpu"]) or not is_int(doc["mem"]) or doc["cpu"] <= 0 or doc["mem"] <= 0: raise Fail(case[0] + ": positive parser mismatch")
        return {"observed": "ACCEPT_UNIT", "costs": {"cpu": doc["cpu"], "mem": doc["mem"], "meaning": "local Aiken VM cost only; not qualified ledger ExUnits or fees"}}
    if step["exitCode"] != 1 or raw_out != b"": raise Fail(case[0] + ": negative must be exit exactly 1 with empty stdout; bad decode, budget, signal and timeout shapes are not semantic rejects")
    m = NEG_RE.match(raw_err.decode("utf-8"))
    if not m: raise Fail(case[0] + ": stderr is not the exact semantic EvaluationFailure shape; a malformed context encoding failure is not a cryptographic reject")
    return {"observed": "SEMANTIC_EVALUATION_FAILURE", "costs": {"cpu": int(m.group(1)), "mem": int(m.group(2))}, "expectedGuard": case[2], "guardAttribution": "SOURCE EXPECTED only; the runtime does not instrument named guards"}

def validate_producer(step, scheme, local):
    raw = read_regular(local + "/logs/helper-%s.stdout.log" % scheme)
    doc = load_strict_bytes(raw, "producer output")
    if not isinstance(doc, dict) or doc.get("format") != "moriarty-vi-signer-producer-output/2" or doc.get("scheme") != scheme: raise Fail("helper producer format/scheme mismatch for " + scheme)
    intents = doc.get("intents")
    if not isinstance(intents, list) or [e.get("signer") for e in intents if isinstance(e, dict)] != ["escrow", "seller"]: raise Fail("helper must produce exactly escrow and seller intents for " + scheme)
    for e in intents:
        # The producer's own Core result is recorded as REPORTED internal
        # agreement only; it is not external verification. Genuine
        # verification is the adapter byte reconstruction plus the VI.
        if e.get("core_verify", {}).get("status") != "ok": raise Fail("helper Core verify not honestly ok (reported) for " + scheme)
        sig = e.get("signed_intent", {}).get("signature_hex", "")
        if type(sig) is not str or len(sig) != 128: raise Fail("helper signature shape mismatch for " + scheme)
    return {"producerOutputSha256": sha(raw), "producerCoreVerify": "REPORTED_ONLY_NOT_EXTERNAL_VERIFICATION", "intents": 2}

def main(argv):
    import signal  # noqa: F401  (used by run_counted cleanup path)
    result = {"format": "moriarty-genuine-vi-experiment-result/2", "status": "FAILED", "attempts": 1, "retries": 0, "calls": [], "counters": {"build": {"attempts": 0, "returned": 0, "validated": 0}, "helper": {"attempts": 0, "returned": 0, "validated": 0}, "aiken": {"attempts": 0, "returned": 0, "validated": 0}}}
    try:
        if len(argv) != 2: raise Fail("usage: operator.py <AUTH.json>")
        auth = load_auth(argv[1]); local = auth["localRoot"]
        caps = CAPS_A if auth["phase"] == "A" else CAPS_B
        Path(local + "/logs").mkdir(parents=True, exist_ok=True)
        if auth["phase"] == "A":
            deadline = time.monotonic() + caps["phaseWallSeconds"]
            capture = Path(local + "/capture")
            if capture.exists() and any(capture.iterdir()): raise Fail("capture directory must start empty; refusing to overwrite or mix runs")
            # Pure-Python staging: no native call, no launch attempt counted.
            for e in auth["moriartyFiles"]:
                stage_file(local, e["inputsRel"], e["stagedRel"], e["bytes"], e["sha256"])
            staged = sorted(str(p.relative_to(capture)) for p in (capture / "work" / "moriarty").rglob("*") if p.is_file())
            if staged != sorted(e["stagedRel"] for e in auth["moriartyFiles"]): raise Fail("staged Core workspace set is not exactly the AUTH moriartyFiles set")
            pins = auth["pins"]
            stage_file(local, "inputs/signer/Cargo.toml", "work/signer/Cargo.toml", pins["signerCargoToml"]["bytes"], pins["signerCargoToml"]["sha256"])
            stage_file(local, "inputs/signer/src/main.rs", "work/signer/src/main.rs", pins["signerMainRs"]["bytes"], pins["signerMainRs"]["sha256"])
            stage_file(local, "inputs/signer/Cargo.lock", "work/signer/Cargo.lock", pins["signerCargoLock"]["bytes"], pins["signerCargoLock"]["sha256"])
            lock_text = read_regular(local + "/capture/work/signer/Cargo.lock", cap=4194304).decode("utf-8")
            if 'name = "moriarty-vi-signer"' not in lock_text or 'name = "mori-core"' not in lock_text: raise Fail("signer Cargo.lock does not include the helper and Core packages")
            wall = min(caps["buildWallSeconds"], remaining_wall(deadline, caps["reserveSeconds"]))
            if wall < 5: raise Fail("Phase A aggregate deadline exhausted before the build launch (no attempt counted)")
            # Exactly one build launch. Cargo/rustc subprocesses inside it
            # are transitive effects of this launch and are reported as
            # such, never as one native operation, singly or in aggregate.
            build = run_counted(bwrap(local, ["/tool/bin/cargo", "build", "--manifest-path", "/capture/work/signer/Cargo.toml", "--offline", "--locked", "--release"], wall), result, "build", local + "/logs/build.stdout.log", local + "/logs/build.stderr.log", caps, wall, caps["cleanupWaitSeconds"])
            result["calls"].append(build)
            try:
                enforce_output_caps(build, caps)
                if build["exitCode"] != 0: raise Fail("helper build failed")
                artifact = local + "/capture/work/target/release/moriarty-vi-signer"
                st = os.lstat(artifact)
                if stat.S_ISLNK(st.st_mode) or not stat.S_ISREG(st.st_mode) or not (st.st_mode & 0o111): raise Fail("build artifact is not a regular executable file at the typed path")
                build["artifact"] = {"pathInsideSandbox": "/capture/work/target/release/moriarty-vi-signer", "bytes": st.st_size, "sha256": sha(read_regular(artifact))}
                build["validationStatus"] = "VALIDATED"; result["counters"]["build"]["validated"] += 1
            except Exception:
                build["validationStatus"] = "FAILED_VALIDATION"; raise
            for scheme in ("schnorr_bip340", "ecdsa_secp256k1_sha256"):
                wall = min(caps["helperWallSecondsEach"], remaining_wall(deadline, caps["reserveSeconds"]))
                if wall < 5: raise Fail("Phase A aggregate deadline exhausted before helper launch (no attempt counted)")
                step = run_counted(bwrap(local, ["/capture/work/target/release/moriarty-vi-signer", scheme, "/inputs/installment-invoice.mori", "/inputs/invoice-partial.request.json"], wall), result, "helper", local + "/logs/helper-%s.stdout.log" % scheme, local + "/logs/helper-%s.stderr.log" % scheme, caps, wall, caps["cleanupWaitSeconds"])
                result["calls"].append(step)  # raw returned step appended BEFORE parsing or validation
                try:
                    enforce_output_caps(step, caps)
                    if step["exitCode"] != 0: raise Fail("helper failed for " + scheme + "; no entropy or key-generation retry")
                    step["validation"] = validate_producer(step, scheme, local)
                    step["validationStatus"] = "VALIDATED"; result["counters"]["helper"]["validated"] += 1
                except Exception:
                    step["validationStatus"] = "FAILED_VALIDATION"; raise
            c = result["counters"]
            if c["build"] != {"attempts": 1, "returned": 1, "validated": 1} or c["helper"] != {"attempts": 2, "returned": 2, "validated": 2}: raise Fail("Phase A counter mismatch")
            result["status"] = "COMPLETED_AWAITING_FRESH_ACTUAL_PUBLIC_INPUT_AUDIT"
        else:
            deadline = time.monotonic() + caps["phaseWallSeconds"]
            for case in EXPECTED:
                term = read_regular(local + "/inputs/contexts/%s.uplc.txt" % case[0], cap=caps["maxContextBytes"]).decode("utf-8")
                if term.endswith("\n"): term = term[:-1]
                if not term or "\n" in term: raise Fail("context must be one line: " + case[0])
                wall = min(caps["perCallWallSeconds"], remaining_wall(deadline, caps["reserveSeconds"]))
                if wall < 5: raise Fail("Phase B aggregate deadline exhausted before launch (no attempt counted)")
                step = run_counted(bwrap(local, ["/tool/bin/aiken", "uplc", "eval", "--cbor", "/inputs/script.hex", term], wall), result, "aiken", local + "/logs/%s.stdout.log" % case[0], local + "/logs/%s.stderr.log" % case[0], caps, wall, caps["cleanupWaitSeconds"])
                result["calls"].append(step)  # raw returned step appended BEFORE validation
                try:
                    enforce_output_caps(step, caps)
                    step["validation"] = validate_call(step, case, local)
                    step["validationStatus"] = "VALIDATED"; result["counters"]["aiken"]["validated"] += 1
                except Exception:
                    step["validationStatus"] = "FAILED_VALIDATION"; raise
            if result["counters"]["aiken"] != {"attempts": 8, "returned": 8, "validated": 8}: raise Fail("exactly eight returned and eight validated aiken calls required")
            result["status"] = "COMPLETED_AWAITING_FRESH_ACTUAL_RESULT_AUDIT"
    except Exception as e: result["error"] = str(e)
    print(json.dumps(result, sort_keys=True))
    return 0 if result["status"].startswith("COMPLETED") else 1

if __name__ == "__main__": sys.exit(main(sys.argv))
