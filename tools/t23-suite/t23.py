#!/usr/bin/env python3
"""T23 runner: the Compact suite through Mycroft and Sherlock, settled on Preview, compared with native.

LOOP row T23 (owner 2026-11-15): every program of the T18 (a) suite that compiles with stock compactc is
compiled by Mycroft to R_SL, every recorded call is proved by Sherlock (leaf, node, L, L_Fr, k = 21 wrap),
settled through SherlockLedger acceptRun on Preview, and its outcome compared with the native Preview
transcript of T18 (a).

Stages per call (pipeline order; the first stage that cannot finish names the call's BLOCKED reason):
  1. compile  Mycroft (mycroft/compiler, branch mycroft-factory) through `mycroft-suite`: real in every mode.
              The source is copied to a snapshot first (HEAD and dirty state recorded), built with its own
              CARGO_TARGET_DIR through heavy-slot.
  2. prove    --prover stub   returns "BLOCKED: R_SL chip set (T11)" (the default; no box is contacted)
              --prover odin|huginn   box dispatch: copy the mycroft-call file to the box, then
                          `herdr --machine BOX agent prompt w1:p1 "<msg>"`; the box replies on
                          ~/.cache/box-bus/all.log ("[BOX] MSG <ts> done hN | t23 <job> <result path>").
                          Only with --dispatch and T23_ALLOW_DISPATCH=1; otherwise the command is printed.
  3. settle   --settle dry    SherlockLedger acceptRun built offline (t10a-ledger-run.mjs dry-run, no network,
                              no wallet); 0 Preview transactions.
              --settle preview  t10a-ledger-run.mjs call with T10_NET=preview (the T17 runner's path), only with
                              T23_ALLOW_PREVIEW=1 and a proved outcome; prints planned and actual tx counts.
  4. compare  the proved outcome against the native Preview transcript (tx-vector of the T18 (a) receipts):
              W word-equal to the on-chain transcript words (guaranteed ++ fallible), or, where the native
              run itself passed only after address substitution, equal to the native ref words; comm equal to
              the native ref lane1_comm (the on-chain comm uses a fresh comm_rand).

Suite table columns: pass, refused-as-native, mismatch (must be 0), blocked (with reason), n/a (pure circuit).

Usage:
  t23.py run [--mode dry|live] [--prover stub|odin|huginn] [--settle dry|preview] [--dispatch]
             [--suite DIR] [--compiler DIR] [--native-preview DIR] [--out DIR] [--skip-build]
  t23.py final --final-dir RET [--final-dir RET ...] [--ledger DIR] [--settle STATE=PROG/CALL ...]
             [--rep-settle-dir DIR] [--out DIR] [--no-bench]
                             the suite table of the T23 final run from the box returns (final/final-box.sh), then
                             bench.py ingest --final-dir/--muninn-dir/--settle into OUT/bench (T23-FINAL-PLAN.md)
  t23.py selftest            fixture tests of the compare path, the bus parser and the guards
The wallet is never opened in dry mode and its seed is never read by this script.
"""
import argparse
import hashlib
import json
import os
import pathlib
import re
import shlex
import shutil
import subprocess
import sys
import time

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import bench  # noqa: E402  (T23 Part B: bench.jsonl, BENCH.md)

H = pathlib.Path.home()
WT = pathlib.Path(__file__).resolve().parents[2]
MYC_FACTORY = H / "Projects/Moriarty-wt/mycroft-factory"
T17_WT = H / "Projects/Moriarty-wt/t17-preview"
DEF = {
    "suite": MYC_FACTORY / "mycroft/tests/compact-suite",
    "compiler": MYC_FACTORY / "mycroft/compiler",
    "native_preview": H / "Projects/Moriarty-wt/mycroft/mycroft/tests/compact-suite/preview",
    "out": H / ".cache/t23/run",
}
CACHE = H / ".cache/t23"
HEAVY = H / ".local/bin/heavy-slot"
BUS = H / ".cache/box-bus/all.log"
KITS = "spike+sha256+bytes"
STUB_BLOCK = "BLOCKED: R_SL chip set (T11)"
# profile: the box's T16 setup profile, written by `t16 setup` as OUT/profile.json with a top-level "pipeline"
# object (openvm t16-elastic 1baf605, examples/common/t16.rs: v["pipeline"] = pipe.to_json(); PF-008). It sets the
# wrapd serve env: W5_WRAP_TAIL_MIB, W5_PIPELINE, W5_EVICT. LEAD PICK: the box paths below (override with
# T23_<BOX>_PROFILE) and W5_PIPELINE = the Overlap variant in lower case ("Full" -> "full", as odin ran w7 with
# W5_PIPELINE=full); no box profile with the pipeline object exists on muninn yet to check other variants.
BOXES = {"odin": {"pane": "w1:p1", "inbox": "~/sherlock/in/t23", "outbox": "~/sherlock/out/t23", "wrapd": "w6",
                  "profile": "~/sherlock/setup/profile.json"},
         "huginn": {"pane": "w1:p1", "inbox": "~/sherlock/in/t23", "outbox": "~/sherlock/out/t23", "wrapd": "w1",
                    "profile": "~/sherlock/setup/profile.json"}}


def wrapd_env(profile):
    """wrapd serve env from a setup profile (PF-008). Refuses a profile without pipeline.wrap_tail_mib: wrapd's
    default tail (12,300 MiB) counts resident cosets and refuses full overlap on odin (h66: 209 MiB short)."""
    pl = (profile or {}).get("pipeline") or {}
    if pl.get("wrap_tail_mib") is None or pl.get("evict") is None or pl.get("overlap") is None:
        raise ValueError("setup profile lacks pipeline.wrap_tail_mib/evict/overlap (PF-008, openvm 1baf605); "
                         "rerun the box setup; the wrapd default tail is not used")
    return {"W5_WRAP_TAIL_MIB": str(int(pl["wrap_tail_mib"])), "W5_PIPELINE": str(pl["overlap"]).lower(),
            "W5_EVICT": "1" if pl["evict"] else "0"}


def log(*a):
    print(f"[{time.strftime('%Y-%m-%dT%H:%M:%S')}]", *a, flush=True)


def sha256_file(p):
    return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()


# ---------------------------------------------------------------- stage 1: compile (real in every mode)
def snapshot(src, dst):
    """Copy the compiler source (no target/, no out/) and record where it came from."""
    src, dst = pathlib.Path(src), pathlib.Path(dst)
    if dst.exists():
        shutil.rmtree(dst)
    shutil.copytree(src, dst, ignore=shutil.ignore_patterns("target", "out"))
    files = sorted(p for p in dst.rglob("*") if p.is_file())
    tree = hashlib.sha256()
    for p in files:
        tree.update(str(p.relative_to(dst)).encode() + b"\0" + p.read_bytes())
    git = lambda *a: subprocess.run(["git", "-C", str(src), *a], capture_output=True, text=True).stdout.strip()
    dirty = git("status", "--porcelain", "--", ".").splitlines()
    return {"source": str(src), "branch": git("rev-parse", "--abbrev-ref", "HEAD"), "head": git("rev-parse", "HEAD"),
            "dirty_paths": len(dirty), "dirty": dirty, "snapshot": str(dst), "tree_sha256": tree.hexdigest(), "files": len(files)}


def build(snap, target, logs, mem):
    # systemd user units do not carry the login PATH: put rustup's cargo first
    env = dict(os.environ, CARGO_TARGET_DIR=str(target), PATH=f"{H / '.cargo/bin'}:{os.environ.get('PATH', '/usr/bin')}")
    cmd = [str(HEAVY), "--mem", mem, "--name", "t23-mycroft-build", "--",
           "cargo", "build", "--release", "--offline", "--bin", "mycroft-suite"]
    log("build:", " ".join(cmd), f"(cwd {snap}, CARGO_TARGET_DIR {target})")
    t = time.time()
    with open(logs / "build.log", "w") as f:
        r = subprocess.run(cmd, cwd=snap, env=env, stdout=f, stderr=subprocess.STDOUT)
    if r.returncode != 0:
        raise SystemExit(f"compile stage: cargo build failed (exit {r.returncode}); see {logs / 'build.log'}")
    b = pathlib.Path(target) / "release/mycroft-suite"
    return {"binary": str(b), "binary_sha256": sha256_file(b), "seconds": round(time.time() - t, 1),
            "rustc": subprocess.run([str(H / ".cargo/bin/rustc"), "--version"], capture_output=True, text=True).stdout.strip()}


def compile_suite(binary, suite, out, logs, mem):
    calls = out / "calls"
    if calls.exists():
        shutil.rmtree(calls)
    env = dict(os.environ, MYCROFT_EMIT_DIR=str(calls))
    cmd = [str(HEAVY), "--mem", mem, "--name", "t23-mycroft-suite", "--", str(binary), str(suite), str(out / "suite.json")]
    log("compile:", " ".join(cmd))
    t = time.time()
    with open(logs / "compile.log", "w") as f:
        r = subprocess.run(cmd, env=env, stdout=f, stderr=subprocess.STDOUT)
    if r.returncode != 0:
        raise SystemExit(f"compile stage: mycroft-suite failed (exit {r.returncode}); see {logs / 'compile.log'}")
    summary = [l for l in (logs / "compile.log").read_text().splitlines() if l.startswith("MYCROFT-SUITE")]
    return {"seconds": round(time.time() - t, 1), "summary": summary, "suite_json": str(out / "suite.json"), "calls_dir": str(calls)}


def kit_block(k):
    """LEAD PICK 2026-11-15 (T11 kit table): jubjub joins the T11 rekey; secp256k1 and tier3 are deferred."""
    if not k or k["status"] == "compiled":
        return None
    why = k.get("reason", "")
    if "Secp256k1" in why:
        return "BLOCKED: kit secp256k1 deferred to a later chip set"
    if "Tier3" in why:
        return "BLOCKED: kit tier3 (keccak256) deferred to a later chip set"
    if "Jubjub" in why:
        return "BLOCKED: kit jubjub, in the T11 rekey"
    return None


def suite_calls(suite):
    """Every recorded native call of the programs that compile with stock compactc (contract-info.json present)."""
    out = []
    for prog in sorted((suite / "native").iterdir()):
        if not (prog / "contract-info.json").exists():
            continue
        info = json.loads((prog / "contract-info.json").read_text())
        proof = {c["name"]: c.get("proof", True) for c in info.get("circuits", [])}
        for f in sorted((prog / "transcripts").glob("*.json")):
            if f.name.endswith(".ref.json") or f.name.startswith("_"):
                continue
            d = json.loads(f.read_text())
            out.append({"id": prog.name, "label": d["label"], "circuit": d["circuit"], "native_outcome": d["outcome"],
                        "pure": proof.get(d["circuit"]) is False})
    return out


# ---------------------------------------------------------------- native Preview transcripts (Part A receipts)
def load_native_preview(pdir):
    """Per (program, label): T18 (a) verdict and the tx-vectors of the included call."""
    pdir = pathlib.Path(pdir)
    res = {}
    for f in sorted(pdir.glob("*.json")):
        if f.name.startswith(("balance-", "summary")) or ".first-instance." in f.name:
            continue
        r = json.loads(f.read_text())
        for c in r.get("calls", []):
            vecs = sorted((pdir / r["id"]).glob(f"{c['label']}.*.tx-vector.json")) if (pdir / r["id"]).is_dir() else []
            res[(r["id"], c["label"])] = {"verdict": c.get("verdict"), "compare": c.get("compare", {}),
                                          "txHash": c.get("txHash"), "blockHeight": c.get("blockHeight"),
                                          "vectors": [json.loads(v.read_text()) for v in vecs]}
    return res


def native_pass(np_call):
    """True when T18 (a) found the included call equal to native (directly or after substitution)."""
    if not np_call or np_call["verdict"] != "included":
        return False
    cmp_, vecs = np_call["compare"], np_call["vectors"]
    if not vecs:
        return False
    # the PASS rules of report.py (T18 (a) verdict_of), in its order
    if all(v.get("equalsNativeRef") is True for v in vecs) and cmp_.get("resultEqualsNative") is not False:
        return True
    sh = [p.get("equalsShadowAtRealAddress") for p in cmp_.get("proofs", [])]
    if sh and all(x is True for x in sh):
        return True  # address-dependent words equal the native driver at the real address
    return all(v.get("equalsNativeRef") is True or v.get("equalsNativeRefAfterCalleeSubstitution") is True for v in vecs)


# ---------------------------------------------------------------- stage 4: compare
def compare(outcome, np_call, native_refs):
    """outcome: {"calls": [{"index", "eff": [str], "comm": str}]} for one scenario call (one entry per ContractCall).
    np_call: the native Preview record (load_native_preview). native_refs: {index: ref.json dict}.
    Returns (verdict, detail): verdict in {"equal", "different", "no-native"}."""
    if not np_call or not np_call["vectors"]:
        return "no-native", "no native Preview transcript for this call"
    if not native_pass(np_call):
        # e.g. z02 mintShieldedToSelfTest on 2026-11-14: the recorded lanes came from the zswap output proof
        return "no-native", "the native Preview transcript of this call did not pass against native (T18 (a))"
    # pair by the native ref each on-chain vector was compared with: a cross-contract transaction orders its
    # ContractCalls caller first on chain, the native ref and the outcome callee first
    by_idx = {}
    for v in np_call["vectors"]:
        m = re.search(r"\.(\d+)\.ref\.json$", v.get("nativeRef") or "")
        by_idx[int(m.group(1)) if m else v["index"]] = v
    detail = []
    for c in outcome["calls"]:
        v, ref = by_idx.get(c["index"]), native_refs.get(c["index"])
        if v is None or ref is None:
            return "different", f"call {c['index']}: no native vector to compare"
        chain = [str(w) for w in (v.get("guaranteed_words") or []) + (v.get("fallible_words") or [])]
        eff = [str(w) for w in c["eff"]]
        if eff == chain:
            how = "W = on-chain words"
        elif (v.get("equalsNativeRef") is not True and native_pass(np_call)
              and eff == [str(w) for w in ref["transcript_words"]]):
            # T18 (a) found these on-chain words equal to native only after address substitution
            how = "W = native ref words (on-chain words equal native after address substitution)"
        else:
            first = next((i for i, (a, b) in enumerate(zip(eff, chain)) if a != b), min(len(eff), len(chain)))
            return "different", f"call {c['index']}: W differs from the on-chain words at word {first}"
        if str(c["comm"]) != str(ref["lane1_comm"]):
            return "different", f"call {c['index']}: comm differs from the native lane1_comm"
        detail.append(how)
    return "equal", "; ".join(detail)


# ---------------------------------------------------------------- stage 2: prove
class StubProver:
    name = "stub"

    def prove(self, job):
        return {"status": "blocked", "reason": STUB_BLOCK}


BUS_RE = re.compile(r"^\[(?P<box>[a-z0-9-]+)\] MSG (?P<ts>\S+) (?P<kind>\w+) (?P<h>h\d+) \| (?P<text>.*)$")


def parse_bus(lines, box, job):
    """The box's reply to one job: the last 'done' or 'fail' message from BOX that names `t23 <job>`."""
    hit = None
    for line in lines:
        m = BUS_RE.match(line.rstrip("\n"))
        tok = re.search(rf"(?:^|\s)t23 {re.escape(job)}(?:\s|$)", m["text"]) if m else None
        if not m or m["box"] != box or not tok:
            continue
        if m["kind"] in ("done", "fail"):
            rest = m["text"][tok.end():].strip()
            hit = {"kind": m["kind"], "ts": m["ts"], "h": m["h"], "rest": rest}
    return hit


class BoxProver:
    """Dispatch one mycroft-call file to a prover box. Never sends anything unless `live` is True."""

    def __init__(self, box, live, bus=BUS, timeout=6 * 3600):
        self.name, self.box, self.live, self.bus, self.timeout = box, BOXES[box], live, pathlib.Path(bus), timeout

    def message(self, job, remote_call, env=None):
        b = self.box
        envs = " ".join(f"{k}={v}" for k, v in env.items()) if env else (
            "W5_WRAP_TAIL_MIB, W5_PIPELINE and W5_EVICT from the pipeline object of " + b["profile"])
        return (f"t23 {job}: run wrapd serve ({b['wrapd']}) with {envs} (setup profile, PF-008; never the default "
                f"tail 12300); then prove the mycroft-call/1 files {remote_call} (one per ContractCall of the scenario call) through the Sherlock pipeline "
                f"(leaf, node, L, L_Fr, k=21 wrap with the {b['wrapd']} wrapd) on the T11 keys with the R_SL chip set; "
                f"check the wrap with the stock verifier and the ledger 9.1.0.1 verifier; write {b['outbox']}/{job}/result.json "
                f"(schema t23-prove-result/1: claim_pub, input_pub, outcome_pub, c_p, calls[{{index, eff, comm}}], "
                f"wrap_proof, wrap_vk_sha256, stock_verify, ledger_verify, wrap_proof_bytes, keys (cold|warm), "
                f"seconds{{{', '.join(bench.RESULT_SECONDS)}}} (the tools' own timers: RSLPROBE, T7CHAIN, T10CALL, E5JSON), "
                f"device{{stage: GPU|CPU}}; partb-bench.sh stage names). "
                f"Reply 'done t23 {job} <result path>' or 'fail t23 {job} <reason>'.")

    def profile_path(self):
        return os.environ.get(f"T23_{self.name.upper()}_PROFILE", self.box["profile"])

    def fetch_env(self):
        local = CACHE / "profiles" / f"{self.name}.json"
        local.parent.mkdir(parents=True, exist_ok=True)
        subprocess.run(["rsync", "-e", "ssh", "-q", f"{self.name}:{self.profile_path()}", str(local)], check=True)
        return wrapd_env(json.loads(local.read_text()))

    def commands(self, job, call_files, env=None):
        """Files go over the box's existing ssh master (remote-connect: only remote-link logs in with the
        YubiKey; `ssh -O check` fails at once when no master is up), with rsync, since the box sshd may lack sftp."""
        rdir = f"{self.box['inbox']}/{job}"
        remote = " ".join(f"{rdir}/{pathlib.Path(f).name}" for f in call_files)
        return remote, [["ssh", "-O", "check", self.name],
                        ["ssh", self.name, "mkdir", "-p", rdir],
                        ["rsync", "-e", "ssh", "-q", *map(str, call_files), f"{self.name}:{rdir}/"],
                        ["herdr", "--machine", self.name, "agent", "prompt", self.box["pane"], self.message(job, remote, env)]]

    def prove(self, job):
        if not job.get("call_files"):
            return {"status": "blocked", "reason": "no mycroft-call file (compile stage did not emit one)"}
        if not self.live:
            remote, cmds = self.commands(job["job"], job["call_files"])
            cmds.insert(1, ["rsync", "-e", "ssh", "-q", f"{self.name}:{self.profile_path()}", str(CACHE / "profiles" / f"{self.name}.json")])
            # dry: the stub verdict, plus the dispatch commands that a live run would send (none is run)
            return {"status": "blocked", "reason": STUB_BLOCK, "commands": [shlex.join(c) for c in cmds]}
        subprocess.run(["ssh", "-O", "check", self.name], check=True)
        try:
            env = self.fetch_env()
        except ValueError as e:
            return {"status": "failed", "reason": str(e)}
        remote, cmds = self.commands(job["job"], job["call_files"], env)
        start = self.bus.stat().st_size if self.bus.exists() else 0
        for c in cmds:
            subprocess.run(c, check=True)
        t0 = time.time()
        while time.time() - t0 < self.timeout:
            if self.bus.exists() and self.bus.stat().st_size > start:
                with open(self.bus) as f:
                    f.seek(start)
                    hit = parse_bus(f.readlines(), self.name, job["job"])
                if hit:
                    if hit["kind"] == "fail":
                        return {"status": "failed", "reason": hit["rest"]}
                    rpath = hit["rest"].split()[0]
                    local = CACHE / "results" / job["job"] / "result.json"
                    local.parent.mkdir(parents=True, exist_ok=True)
                    subprocess.run(["rsync", "-e", "ssh", "-q", f"{self.name}:{rpath}", str(local)], check=True)
                    return {"status": "proved", "result": json.loads(local.read_text()), "result_file": str(local)}
            time.sleep(20)
        return {"status": "failed", "reason": f"no reply from {self.name} within {self.timeout} s"}


# ---------------------------------------------------------------- stage 3: settle
# t10a-ledger-run.mjs on t23-suite (from t17-preview 9e1e7f398) with the T12-SLOT15 host side (anchor-slot.mjs, from
# t12-slot15 707c9a8ed): a 16-child deploy state with the anchor at slot 15 and the acceptRun prologue rewritten to
# read slot 15 (MEASURED offline: the call vector differs from the slot-0 call only in lane 6, 0 -> 15), and a suite
# mode: one SherlockLedger per program (`deploy` into PREVIEW_DIR/<program>), each call in its own dir against it
# (`call --deploy-from`), refusals R1 to R4 off per call (T10A_REFUSALS=0). acceptRun words: accept_args.py (t23-args
# a030afc26). LEAD PICK (T23-SETTLE, for T15): the ledger anchor of a program is the C_P of its runs (FP-3 A_0 = C_P);
# the binding words come from t10a's tx build, so the wrap proof of a call is made by T10A_PROVER (the box prover of
# the R_SL chain) for the 1,026-word vector that t10a hands it; the Preview submit time is submissions.A.seconds.
# The proof server's /check (stock compactc zkir, prologue key 0) refuses slot-15 transcripts, so t10a checks the slot-0
# preimage of the same circuit run after asserting the two differ only in input 4 (anchor-slot.mjs, Preview rehearsal
# 2026-11-15). The box prover hook is tools/sherlock-sdk/box/box-preview-prover.sh (HP_* env, T17 (c)).
SDK = WT / "tools/sherlock-sdk"
T10A = SDK / "t10a-ledger-run.mjs"
T17_IN = H / ".cache/t17/in"
T17_BUILD = H / ".cache/t17/contract/build/sherlock-ledger"
PREVIEW_DIR = CACHE / "preview"
ANCHOR_SLOT = "15"


def accept_words(result):
    """The acceptRun words of a proved call, checked with accept_args: claim_pub recomputed from (c_p, input_pub,
    outcome_pub), anchor = C_P, and the call-vector lanes 10, 20, 34, 39 when the result carries `call_vector`.
    Returns (words, ledger-args object for t10a --args). Raises accept_args.Mismatch."""
    import accept_args as aa
    w = {k: int(result[k]) for k in ("c_p", "input_pub", "outcome_pub")}
    w["claim_pub"] = aa.transient_hash([w["c_p"], w["input_pub"], w["outcome_pub"]])
    w["anchor"] = w["c_p"]
    aa.check_public(w, {k: result[k] for k in ("claim_pub", "anchor", "call_vector") if k in result})
    return w, aa.ledger_args(w)


def settle_plan(prog, call, args_file, wrap_vk, build=T17_BUILD, prover="<T23_PREVIEW_PROVER>"):
    """The Preview commands of one call: deploy the program's ledger once, then the call against it."""
    pdir = PREVIEW_DIR / prog
    cdir = pdir / "calls" / call
    common = ["--vk", str(wrap_vk), "--args", str(args_file), "--build", str(build), "--anchor-slot", ANCHOR_SLOT]
    return {"program_dir": pdir, "call_dir": cdir,
            "deploy": ["node", str(T10A), "deploy", *common, "--out", str(pdir)],
            "call": ["node", str(T10A), "call", *common, "--out", str(cdir), "--deploy-from", str(pdir)],
            "env": {"T10_NET": "preview", "T10A_REFUSALS": "0", "T10A_PROVER": prover}}


def settle_rehearsal(out, logs):
    """One offline acceptRun build (t10a-ledger-run.mjs dry-run, anchor slot 15): padded call, stub prover, JS ledger
    shape check, privacy scan, post-state readback and replay refusal, against a locally constructed SherlockLedger.
    No network, no wallet. The args are the T17 ledger-args (stand-ins: the T23 claim words exist only after an R_SL
    proof)."""
    need = [T10A, T17_IN / "wrap.vk", T17_IN / "ledger-args.json", T17_BUILD / "keys/acceptRun.verifier"]
    miss = [str(p) for p in need if not pathlib.Path(p).exists()]
    if miss:
        return {"status": "blocked", "reason": f"settle rehearsal inputs missing: {miss}"}
    d = out / "settle-dry"
    d.mkdir(parents=True, exist_ok=True)
    env = {k: v for k, v in os.environ.items() if not k.startswith(("T10_NET", "T10A_PROVER", "MORIARTY_PREVIEW"))}
    cmd = ["node", str(T10A), "dry-run", "--vk", str(T17_IN / "wrap.vk"), "--args", str(T17_IN / "ledger-args.json"),
           "--out", str(d), "--build", str(T17_BUILD), "--anchor-slot", ANCHOR_SLOT]
    log("settle (dry):", shlex.join(cmd))
    t = time.time()
    with open(logs / "settle-dry.log", "w") as f:
        r = subprocess.run(cmd, env=env, stdout=f, stderr=subprocess.STDOUT, timeout=1800)
    st = json.loads((d / "t10a-dry-run.json").read_text()) if (d / "t10a-dry-run.json").exists() else {}
    v = st.get("dry-run", {}).get("shapeVerdict", {})
    vec = d / "dry-run-vector.json"
    lane6 = None
    if vec.exists():
        vv = json.loads(vec.read_text())
        lane6 = (vv["words"] if isinstance(vv, dict) else vv)[6]
    ok = (r.returncode == 0 and v.get("jsLedgerAcceptsShapeAndApplies") and v.get("postAReadback")
          and v.get("replayRefusedWithProofsOff") and st.get("anchorSlot") == 15 and str(lane6) == ANCHOR_SLOT)
    return {"status": "ok" if ok else "failed", "exit": r.returncode, "seconds": round(time.time() - t, 1),
            "shapeVerdict": v, "anchorSlot": st.get("anchorSlot"), "lane6": lane6, "network": "none (offline)",
            "previewTxPlanned": 0, "previewTxSent": 0, "state": str(d / "t10a-dry-run.json"), "t10a_sha256": sha256_file(T10A)}


def settle_call(mode, proved, job, wrap_vk=None, build=T17_BUILD):
    res = proved["result"]
    prog, call = job.get("id", "-"), job.get("label", "-")
    if mode == "dry":
        try:
            words, la = accept_words(res)
        except Exception as e:  # accept_args.Mismatch or missing words
            return {"status": "args-refused", "reason": str(e), "previewTxPlanned": 0, "previewTxSent": 0}
        plan = settle_plan(prog, call, PREVIEW_DIR / prog / "calls" / call / "ledger-args.json", wrap_vk or "<wrap.vk>", build)
        return {"status": "dry", "previewTxPlanned": 1, "previewTxSent": 0, "args": la, "claim_pub": str(words["claim_pub"]),
                "plan": {k: shlex.join(v) if isinstance(v, list) else str(v) for k, v in plan.items() if k != "env"}}
    if os.environ.get("T23_ALLOW_PREVIEW") != "1":
        raise SystemExit("settle preview: set T23_ALLOW_PREVIEW=1 (Preview only; Mainnet is never a target)")
    prover = os.environ.get("T23_PREVIEW_PROVER")
    if not prover or not wrap_vk:
        raise SystemExit("settle preview: T23_PREVIEW_PROVER (the R_SL chain box prover, rsl-chain) and the wrap.vk of the final keys")
    words, la = accept_words(res)
    plan = settle_plan(prog, call, PREVIEW_DIR / prog / "calls" / call / "ledger-args.json", wrap_vk, build, prover)
    pdir, cdir = plan["program_dir"], plan["call_dir"]
    cdir.mkdir(parents=True, exist_ok=True)
    (cdir / "ledger-args.json").write_text(json.dumps(la))
    env = {**os.environ, **plan["env"]}
    pst = pdir / "t10a-state.json"
    if not (pst.exists() and json.loads(pst.read_text()).get("deploy", {}).get("blockHeight")):
        log("settle (preview): deploy", shlex.join(plan["deploy"]))
        subprocess.run(plan["deploy"], cwd=SDK, env=env, check=True, timeout=7200)
    log("settle (preview): call", shlex.join(plan["call"]))
    r = subprocess.run(plan["call"], cwd=SDK, env=env, timeout=4 * 3600)
    st = json.loads((cdir / "t10a-state.json").read_text()) if (cdir / "t10a-state.json").exists() else {}
    acc = (st.get("calls") or {}).get("accept") or {}
    sub = (st.get("submissions") or {}).get("A") or {}
    ok = r.returncode == 0 and acc.get("claimInRuns") is True and acc.get("runsLookup") == str(words["outcome_pub"])
    return {"status": "settled" if ok else "settle-failed", "exit": r.returncode, "previewTxPlanned": 1,
            "previewTxSent": 1 if sub.get("txHash") else 0, "txHash": sub.get("txHash"), "blockHeight": sub.get("blockHeight"),
            "claim_pub": str(words["claim_pub"]), "state": str(cdir / "t10a-state.json"),
            "bench": bench.ingest_preview("t23", cdir / "t10a-state.json", {"A": f"{prog}/{call}"})}


# ---------------------------------------------------------------- the run
def classify(calls, suite_json, emitted, natprev, suite, prover, settle_mode):
    res = json.loads(pathlib.Path(suite_json).read_text())
    by = {(c["id"], c["circuit"]): next((k for k in c["results"] if k["kits"] == KITS), None) for c in res["circuits"]}
    vec_rows = {}
    for (pid, circ), k in by.items():
        if k and k["status"] == "compiled":
            for v in k["rows"]:
                vec_rows.setdefault((v.get("call_id") or pid, v["label"]), []).append((pid, circ, v))
    rows = []
    for c in calls:
        key = (c["id"], c["label"])
        k = by.get((c["id"], c["circuit"]))
        np_ = natprev.get(key)
        row = {**c, "native_preview": (np_ or {}).get("verdict"), "native_preview_pass": native_pass(np_),
               "compile": None, "prove": None, "settle": None, "compare": None, "precheck": None, "result": None, "reason": ""}
        if c["native_outcome"] != "ok":
            row["compile"] = "no native preimage"
            row["result"] = "refused-as-native"
            row["reason"] = ("DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; "
                             f"native Preview: {(np_ or {}).get('verdict') or 'no record'}")
            rows.append(row)
            continue
        if c["pure"] and k is None:
            row["result"], row["reason"] = "n/a", "pure circuit: no ZKIR, no proof, no transaction"
            rows.append(row)
            continue
        blk = kit_block(k)
        if blk:
            row["compile"], row["result"], row["reason"] = "kit refused", "blocked", blk
            rows.append(row)
            continue
        vs = vec_rows.get(key, [])
        if k is None or k["status"] != "compiled" or not vs or not all(v["status"] == "equal" and v.get("reference_equal") for _, _, v in vs):
            why = "no ZKIR" if k is None else (f"compile refused ({k.get('stage')}): {k.get('reason')}" if k["status"] != "compiled"
                                               else "; ".join(f"{v['status']}" for _, _, v in vs) or "no ref vector")
            row["compile"], row["result"], row["reason"] = "different", "mismatch", f"compile stage: {why}"
            rows.append(row)
            continue
        row["compile"] = "W, comm, outputs equal to native ref"
        files = []
        for owner, circ, v in vs:  # a cross-contract callee vector is emitted under the callee's (ZKIR owner's) id
            f = emitted / f"{owner}.{circ}.{c['label']}.{v['index']}.{KITS}.call.json"
            files.append(f if f.exists() else None)
        # precheck (no proof needed): the compiled W against the on-chain words of the native Preview run
        refs = {}
        for owner, circ, v in vs:
            rp = suite / "native" / c["id"] / "transcripts" / f"{c['label']}.{v['index']}.ref.json"
            if rp.exists():
                refs[v["index"]] = json.loads(rp.read_text())
        if all(files):
            calls_out = []
            for f in files:
                cj = json.loads(f.read_text())
                calls_out.append({"index": cj["index"], "eff": cj["eff"], "comm": cj["comm"]})
            row["precheck"] = compare({"calls": calls_out}, np_, refs)[0]
        else:
            row["precheck"] = "no emitted file"
        row["vectors"] = len(vs)
        job = {"job": f"{c['id']}.{c['label']}", "call_files": [f for f in files if f] if all(files) else None}
        pr = prover.prove(job)
        row["prove"] = pr["status"]
        if pr["status"] != "proved":
            row["result"], row["reason"] = ("blocked" if pr["status"] == "blocked" else "prove-failed"), pr["reason"]
            if pr.get("commands"):
                row["dispatch"] = pr["commands"]
            rows.append(row)
            continue
        row["bench"] = bench.from_result("t23", c["id"], c["label"], getattr(prover, "name", "-"), pr["result"])
        sc = settle_call(settle_mode, pr, {**job, "id": c["id"], "label": c["label"]}, wrap_vk=getattr(prover, "wrap_vk", None))
        row["settle"], row["settle_detail"] = sc["status"], {k: v for k, v in sc.items() if k != "bench"}
        row["bench"] += sc.get("bench", [])
        verdict, detail = compare({"calls": pr["result"]["calls"]}, np_, refs)
        row["compare"] = verdict
        if verdict == "equal":
            row["result"], row["reason"] = "pass", detail
        elif verdict == "no-native":
            row["result"], row["reason"] = "blocked", "no native Preview transcript (" + str(row["native_preview"]) + ")"
        else:
            row["result"], row["reason"] = "mismatch", detail
        rows.append(row)
    return rows


def table(rows, meta):
    cols = ["pass", "refused-as-native", "mismatch", "blocked", "prove-failed", "n/a"]
    tot = {k: sum(1 for r in rows if r["result"] == k) for k in cols}
    reasons = {}
    for r in rows:
        if r["result"] == "blocked":
            reasons[r["reason"].split(": would run")[0]] = reasons.get(r["reason"].split(": would run")[0], 0) + 1
    pre = [r["precheck"] for r in rows if r["precheck"]]
    nonat = sum(1 for r in rows if r["result"] == "blocked" and r["native_outcome"] == "ok" and not r["native_preview_pass"])
    live = meta.get("dispatch_live")
    sr = meta.get("settle_rehearsal") or {}
    ref = [r for r in rows if r["result"] == "refused-as-native"]
    ref_prev = sum(1 for r in ref if r["native_preview"] == "refused-locally")
    L = [f"# T23 suite table ({meta['mode'].upper()} RUN)", "",
         f"Run {meta['started']} to {meta['finished']}; prover `{meta['prover']}`"
         f"{'' if meta['prover'] == 'stub' else (', dispatch ON' if live else ', dispatch off')}, settle `{meta['settle']}`, "
         f"Preview transactions sent: {meta['preview_tx_sent']}.", "",
         f"Settle rehearsal (SherlockLedger acceptRun built offline by t10a-ledger-run.mjs dry-run with the T17 stand-in "
         f"arguments, no network, no wallet): {sr.get('status', 'not run')}"
         + (f"; JS ledger accepts and applies: {sr['shapeVerdict'].get('jsLedgerAcceptsShapeAndApplies')}, "
            f"runs[claim_pub] = outcome_pub read back: {sr['shapeVerdict'].get('postAReadback')}, "
            f"replay refused: {sr['shapeVerdict'].get('replayRefusedWithProofsOff')}, privacy scan: {sr['shapeVerdict'].get('scanPass')}"
            if sr.get("shapeVerdict") else "") + ".", "",
         "Open T23 dependency: the map from a Mycroft statement (`[bind, comm] ++ anchor ++ W`) to the acceptRun "
         "arguments (c_p, input_pub, outcome_pub) is not defined yet; it comes with the T11 R_SL chip set and the wrap "
         "public-value layout. Live settle stays off until then.", "",
         f"Mycroft source: {meta['compiler']['branch']} {meta['compiler']['head'][:9]} plus {meta['compiler']['dirty_paths']} "
         f"uncommitted paths (snapshot tree sha256 {meta['compiler']['tree_sha256'][:16]}); binary sha256 "
         f"{meta['build'].get('binary_sha256', '-')[:16]}; native Preview receipts: `{meta['native_preview']}`.", "",
         "| result | calls |", "|---|---:|"]
    for k in cols:
        L.append(f"| {k} | {tot[k]} |")
    L.append(f"| total | {len(rows)} |")
    L += ["", f"Mismatch: {tot['mismatch']} of {sum(1 for r in rows if r['compare'])} compared after a proof "
          f"(the compare stage runs only on a proved outcome).", "",
          "Blocked, by first blocking stage:", "", "| reason | calls |", "|---|---:|"]
    for k, n in sorted(reasons.items(), key=lambda x: -x[1]):
        L.append(f"| {k} | {n} |")
    L += ["", f"Refused-as-native: {len(ref)} calls, DERIVED, not run (the native runtime refuses them before proving). "
          f"{ref_prev} of them were refused on Preview as native on the T18 (a) run; the rest: "
          + (", ".join(f"`{r['id']}/{r['label']}` (native Preview {r['native_preview'] or 'no record'})" for r in ref
                       if r["native_preview"] != "refused-locally") or "none") + ".", "",
          f"Of the blocked calls, {nonat} also have no passing native Preview transcript (T18 (a) verdict not PASS), "
          "so they stay blocked after T11/T12 until the native run includes them.", "",
          f"Precheck without a proof (compiled W and comm against the native Preview transcript): "
          f"{pre.count('equal')} equal, {pre.count('different')} different, {pre.count('no-native')} without a passing native Preview transcript, "
          f"of {len(pre)} calls with an emitted mycroft-call file.", "",
          "| program | call | circuit | native | native Preview | compile | precheck | prove | result | reason |",
          "|---|---|---|---|---|---|---|---|---|---|"]
    for r in rows:
        L.append(f"| `{r['id']}` | `{r['label']}` | `{r['circuit']}` | {r['native_outcome']} | {r['native_preview'] or '-'} | "
                 f"{r['compile'] or '-'} | {r['precheck'] or '-'} | {r['prove'] or '-'} | {r['result']} | {r['reason'].split(': would run')[0]} |")
    return tot, "\n".join(L) + "\n"


def run(a):
    out = pathlib.Path(a.out)
    logs = CACHE / "logs"
    for d in (out, logs):
        d.mkdir(parents=True, exist_ok=True)
    meta = {"schema": "t23-run/1", "mode": a.mode, "prover": a.prover, "settle": a.settle,
            "started": time.strftime("%Y-%m-%dT%H:%M:%S%z"), "suite": str(a.suite), "native_preview": str(a.native_preview)}
    if a.mode == "dry" and (a.prover != "stub" or a.settle != "dry") and a.dispatch:
        raise SystemExit("dry mode: --dispatch is refused")
    meta["compiler"] = snapshot(a.compiler, CACHE / "src/compiler")
    if a.skip_build and (CACHE / "target/release/mycroft-suite").exists():
        b = CACHE / "target/release/mycroft-suite"
        meta["build"] = {"binary": str(b), "binary_sha256": sha256_file(b), "skipped": True}
    else:
        meta["build"] = build(meta["compiler"]["snapshot"], CACHE / "target", logs, a.build_mem)
    meta["compile"] = compile_suite(meta["build"]["binary"], pathlib.Path(a.suite), out, logs, a.run_mem)
    meta["dispatch_live"] = bool(a.dispatch and os.environ.get("T23_ALLOW_DISPATCH") == "1")
    prover = StubProver() if a.prover == "stub" else BoxProver(a.prover, live=meta["dispatch_live"])
    meta["settle_rehearsal"] = settle_rehearsal(out, logs) if a.settle == "dry" else None
    calls = suite_calls(pathlib.Path(a.suite))
    rows = classify(calls, out / "suite.json", out / "calls", load_native_preview(a.native_preview), pathlib.Path(a.suite), prover, a.settle)
    meta["preview_tx_sent"] = 0
    meta["finished"] = time.strftime("%Y-%m-%dT%H:%M:%S%z")
    tot, md = table(rows, meta)
    meta["totals"] = tot
    (out / "t23-suite.json").write_text(json.dumps({"meta": meta, "rows": rows}, indent=1, default=str))
    (out / "T23-SUITE.md").write_text(md)
    # Part B benchmark: suite-level Mycroft compile (not apportioned to programs) and every proved call's stage times
    brows = [bench.row(meta["started"], "suite", "-", "-", "mycroft-suite", meta["compile"].get("seconds"), "muninn", "CPU", "-",
                       "t23.py compile_suite (wall, whole suite)")]
    for r in rows:
        brows += r.get("bench", [])
    bench.write(brows, out, f"T23 Part B benchmark ({a.mode} run)", {"prover": a.prover, "settle": a.settle})
    log("T23-SUITE", json.dumps(tot), "preview_tx_sent=0", "settle_rehearsal=" + str((meta["settle_rehearsal"] or {}).get("status")))
    if tot["mismatch"]:
        return 4
    return 5 if (meta["settle_rehearsal"] or {}).get("status") == "failed" else 0


# ---------------------------------------------------------------- final mode (T23 final run on the T25 keys)
# `t23.py final` builds the suite table from the box returns of final/final-box.sh (T23-FINAL-PLAN.md sections 2 and
# 3): the 208 suite rows, each provable row mapped to its call files (final/calls.sha256; a cross-contract row has the
# outer x01 file and the inner x02 file), each file read from the return of the box its list assigns. Checks per call:
# rc 0, keys.out `T25SUITE EQUAL`, wrap.vk 6faac46f, ledger 9.1.0.1 Ok(()) (with --ledger), then the proved effect
# words and comm against the native Preview transcript (compare), or against the native ref only where the call has no
# passing native Preview transcript (labelled "native ref only"). The Preview settle column comes from --settle states.
# It then runs bench.py ingest on the same returns (--final-dir, --muninn-dir, --settle) unless --no-bench.
HERE = pathlib.Path(__file__).resolve().parent
FINAL = HERE / "final"
T25_VK_SHA = "6faac46fa12e5fc1b561e033f301f055117144cd3dd6a020f5a5f675704d6d5b"
FORBIDDEN = re.compile(r"\.secret|args[^/]*\.json|forbid|vector\.json|accept-bundle\.json|lfr_pk|lfr-call\.json|call-lfr")
# the calibration calls are on every list; the plan's share table assigns them (huginn t02, odin d02)
CAL_OWNER = {"t02-counter-example.increment.increment.0": "huginn", "d02-battleship.player1Shoot.04-alice-shoot-5.0": "odin"}
CHIP_BLOCK = {("s02-foreign", "ecdsa-valid"): "BLOCKED: secp256k1 chip set deferred (T18 (d) LEAD PICK)",
              ("s02-foreign", "ecdsa-bad"): "BLOCKED: secp256k1 chip set deferred (T18 (d) LEAD PICK)",
              ("s02-foreign", "keccak"): "BLOCKED: keccak256 tier 3 chip set deferred (T18 (d) LEAD PICK)"}
CALL_DIRS = [H / ".cache/rekey-chips/calls3", H / ".cache/rekey-chips/calls4"]


def final_files(calls_sha=FINAL / "calls.sha256", call_dirs=CALL_DIRS):
    """The final run's call files: {file label: (program id, label, index)} from the rsl-call/1 metadata."""
    out = {}
    for line in pathlib.Path(calls_sha).read_text().splitlines():
        if not line.strip():
            continue
        name = line.split(None, 1)[1].strip().split("/", 1)[-1]
        lbl = name[: -len(".rsl.json")]
        src = next((pathlib.Path(d) / name for d in call_dirs if (pathlib.Path(d) / name).exists()), None)
        if src:
            m = json.loads(src.read_text())
            out[lbl] = (m["id"], m["label"], int(m["index"]))
        else:  # file name form PROG.CIRCUIT.LABEL.INDEX
            p = lbl.split(".")
            out[lbl] = (p[0], ".".join(p[2:-1]), int(p[-1]))
    return out


def final_assign(calls, files):
    """{(id, label): [file labels]}: a file belongs to its own program's row, else to the one row with its label
    (the inner x02 file of an x01 cross-contract row)."""
    rows = {(c["id"], c["label"]) for c in calls}
    by = {}
    for f, (pid, lab, _) in sorted(files.items()):
        key = (pid, lab) if (pid, lab) in rows else None
        if key is None:
            cand = [r for r in rows if r[1] == lab]
            if len(cand) != 1:
                raise SystemExit(f"final: call file {f} matches {len(cand)} suite rows")
            key = cand[0]
        by.setdefault(key, []).append(f)
    return by


def runs_log_tail_ok(d, rl):
    """runs.log minus its last line hashes to the RETURN.sha256 entry (final-box.sh appends after hashing)."""
    want = next((l.split()[0] for l in rl.splitlines() if l.split()[-1].lstrip("./").endswith("runs.log")), None)
    lines = (d / "runs.log").read_bytes().splitlines(keepends=True)
    return want is not None and len(lines) > 1 and hashlib.sha256(b"".join(lines[:-1])).hexdigest() == want


def read_return(d, vk_sha=T25_VK_SHA):
    """One box return: RETURN.sha256 verified, no forbidden name; {file label: call record}."""
    d = pathlib.Path(d)
    rl = (d / "RETURN.sha256").read_text()
    bad = [l for l in rl.splitlines() if FORBIDDEN.search(l)]
    if bad:
        raise SystemExit(f"final: FORBIDDEN FILE IN RETURN {d} ({len(bad)} lines)")
    r = subprocess.run(["sha256sum", "-c", "--quiet", "RETURN.sha256"], cwd=d, capture_output=True, text=True)
    failed = [l.rsplit(":", 1)[0] for l in r.stdout.splitlines() if l.endswith(": FAILED")]
    # final-box.sh logs its own "RETURN ..." line to runs.log after it hashed runs.log. Accept exactly that case:
    # runs.log is the only mismatch and runs.log without its last line has the recorded sum.
    if failed == ["runs.log"] and runs_log_tail_ok(d, rl):
        r = subprocess.CompletedProcess(r.args, 0, "", "")
    if r.returncode:
        raise SystemExit(f"final: RETURN.sha256 check failed in {d}: {(r.stdout + r.stderr).strip().splitlines()[:3]}")
    box = None
    for line in (d / "times.tsv").read_text().splitlines() if (d / "times.tsv").exists() else []:
        box = line.split("\t")[3]
        break
    box = box or d.name.removeprefix("ret-")
    out = {}
    for cd in sorted((d / "calls").iterdir()) if (d / "calls").is_dir() else []:
        rc = (cd / "rc").read_text().strip() if (cd / "rc").exists() else None
        kl = (cd / "keys.out").read_text().splitlines() if (cd / "keys.out").exists() else []
        vk = cd / "wrap/wrap.vk"
        pr = cd / "prove-result.partial.json"
        rs = json.loads((cd / "rsl-chain.json").read_text()) if (cd / "rsl-chain.json").exists() else {}
        out[cd.name] = {"box": box, "rc": rc, "keys": kl[0] if kl else None,
                        "vk_ok": vk.exists() and sha256_file(vk) == vk_sha, "bucket": rs.get("rsl_bucket"),
                        "result": json.loads(pr.read_text()) if pr.exists() else None}
    return box, out


def read_ledger(d):
    """partb-bench.sh ledger rows: {(program, call): "Ok" | reason}; Ok needs result Ok(()) and every negative refused."""
    d, out = pathlib.Path(d), {}
    for line in (d / "times.tsv").read_text().splitlines() if (d / "times.tsv").exists() else []:
        t = line.split("\t")
        if len(t) < 14 or t[0] != "ledger_check":
            continue
        e = None
        lp = d / t[13]
        for l in lp.read_text(errors="replace").splitlines() if lp.exists() else []:
            if l.startswith("E5JSON ") and '"ledger_9_1_0_1_verify"' in l:
                e = json.loads(l[7:])
        neg = e and all(e.get("changed_lane_refused", {}).values()) and all(e.get("changed_public_input_refused", [])) \
            and all(e.get("flipped_proof_byte_refused", []))
        out[(t[1], t[2])] = "Ok" if (t[7] == "0" and e and e.get("result") == "Ok(())" and neg) else \
            f"ledger 9.1.0.1 not Ok (rc {t[7]}, {('result ' + str(e.get('result'))) if e else 'no E5JSON'})"
    return out


def read_settle(state):
    s = json.loads(pathlib.Path(state).read_text()).get("submissions", {})
    a = s.get("A") or {}
    ref = sum(1 for k, v in s.items() if k not in ("deploy-ledger", "A") and v.get("verdict") != "included")
    return (f"acceptRun block {a.get('blockHeight')}" if a.get("verdict") == "included" else f"not included ({a.get('verdict')})") \
        + (f", {ref} refused" if ref else "")


def list_box(lists):
    """{file label: box} for rep and run lines (cal lines: CAL_OWNER)."""
    own = dict(CAL_OWNER)
    for bx, p in lists.items():
        for line in pathlib.Path(p).read_text().splitlines():
            t = line.split("\t")
            if len(t) >= 2 and t[0] in ("rep", "run"):
                own[t[1][: -len(".rsl.json")]] = bx
    return own


def ref_only(outcome, refs):
    for c in outcome["calls"]:
        ref = refs.get(c["index"])
        if ref is None:
            return "different", f"call {c['index']}: no native ref"
        if [str(w) for w in c["eff"]] != [str(w) for w in ref["transcript_words"]]:
            return "different", f"call {c['index']}: W differs from the native ref words"
        if str(c["comm"]) != str(ref["lane1_comm"]):
            return "different", f"call {c['index']}: comm differs from the native ref lane1_comm"
    return "equal", "native ref only (no passing native Preview transcript): W and comm equal the native ref"


def classify_final(calls, assign, rets, owner, natprev, suite, ledger=None, settles=None):
    """rets: {box: {file label: record}}. ledger: read_ledger map or None (not given). settles: {file label: text}."""
    settles = settles or {}
    rows = []
    for c in calls:
        key = (c["id"], c["label"])
        np_ = natprev.get(key)
        files = assign.get(key, [])
        row = {**c, "native_preview": (np_ or {}).get("verdict"), "native_preview_pass": native_pass(np_), "files": files,
               "box": None, "keys": None, "ledger": None, "compare": None, "settle": None, "bucket": None,
               "result": None, "reason": ""}
        rows.append(row)
        if c["native_outcome"] != "ok":
            row["result"], row["reason"] = "refused-as-native", "DERIVED, not run: native refuses before proving (no tape)"
            continue
        if c["pure"]:
            row["result"], row["reason"] = "n/a", "pure circuit: no ZKIR, no proof, no transaction"
            continue
        if not files:
            row["result"], row["reason"] = "blocked", CHIP_BLOCK.get(key, "BLOCKED: no call file in the final run")
            continue
        recs = []
        for f in files:
            b = owner.get(f)
            rec = rets.get(b, {}).get(f) if b else None
            if rec is None:  # assigned box has not returned it (or moved by REBALANCE): any box that did
                rec = next((r[f] for r in rets.values() if f in r), None)
            recs.append((f, rec))
        miss = [f for f, r in recs if r is None]
        if miss:
            row["result"], row["reason"] = "blocked", f"not returned yet: {', '.join(miss)}"
            continue
        row["box"] = "+".join(sorted({r["box"] for _, r in recs}))
        row["bucket"] = "+".join(sorted({str(r["bucket"]) for _, r in recs if r["bucket"]})) or None
        bad = [(f, r["rc"]) for f, r in recs if r["rc"] != "0"]
        if bad:
            row["result"], row["reason"] = "prove-failed", "; ".join(f"{f}: exit {rc}" for f, rc in bad)
            continue
        keys_ok = all((r["keys"] or "").startswith("T25SUITE EQUAL") and r["vk_ok"] for _, r in recs)
        row["keys"] = "T25SUITE EQUAL, wrap.vk 6faac46f" if keys_ok else "MISMATCH"
        if not keys_ok:
            row["result"], row["reason"] = "mismatch", "key check: " + "; ".join(
                f"{f}: {r['keys'] or 'no keys.out'}{'' if r['vk_ok'] else ', wrap.vk not 6faac46f'}" for f, r in recs)
            continue
        if ledger is not None:
            ls = [ledger.get(tuple(f.split(".", 1)), "ledger check not run") for f in files]
            row["ledger"] = "Ok" if all(x == "Ok" for x in ls) else "; ".join(x for x in ls if x != "Ok")
        st = [settles[f] for f in files if f in settles]
        row["settle"] = "; ".join(st) or None
        out = {"calls": [cc for _, r in recs for cc in r["result"]["calls"]]}
        refs = {}
        for cc in out["calls"]:
            rp = suite / "native" / c["id"] / "transcripts" / f"{c['label']}.{cc['index']}.ref.json"
            if rp.exists():
                refs[cc["index"]] = json.loads(rp.read_text())
        verdict, detail = compare(out, np_, refs) if row["native_preview_pass"] else ref_only(out, refs)
        row["compare"] = verdict if row["native_preview_pass"] else f"{verdict} (native ref only)"
        if verdict != "equal":
            row["result"], row["reason"] = "mismatch", detail
        elif row["ledger"] not in (None, "Ok"):
            row["result"], row["reason"] = ("blocked" if row["ledger"] == "ledger check not run" else "prove-failed"), row["ledger"]
        else:
            row["result"], row["reason"] = "pass", detail
    return rows


FINAL_COUNTS = {"rows": 208, "refused-as-native": 62, "n/a": 1, "chip-blocked": 3, "provable": 142, "files": 145}


def final_invariant(rows, assign):
    n = {"rows": len(rows), "refused-as-native": sum(r["result"] == "refused-as-native" for r in rows),
         "n/a": sum(r["result"] == "n/a" for r in rows),
         "chip-blocked": sum(r["result"] == "blocked" and r["reason"].startswith(("BLOCKED: secp", "BLOCKED: keccak")) for r in rows),
         "files": sum(len(v) for v in assign.values())}
    n["provable"] = sum(1 for r in rows if r["files"])
    return n, n == FINAL_COUNTS


def final_table(rows, meta):
    cols = ["pass", "refused-as-native", "mismatch", "blocked", "prove-failed", "n/a"]
    tot = {k: sum(1 for r in rows if r["result"] == k) for k in cols}
    reasons = {}
    for r in rows:
        if r["result"] == "blocked":
            k = "not returned yet" if r["reason"].startswith("not returned yet") else r["reason"]
            reasons[k] = reasons.get(k, 0) + 1
    inv, ok = meta["invariant"], meta["invariant_ok"]
    L = [f"# T23 suite table (final run on the T25 keys)", "",
         f"Built {meta['built']} from the box returns {', '.join(meta['returns']) or 'none'}"
         f"; ledger check {meta['ledger'] or 'not given'}; Preview settles {meta['settles']}.", "",
         f"Row count: {inv['rows']} rows = {inv['refused-as-native']} refused-as-native + {inv['n/a']} n/a + "
         f"{inv['chip-blocked']} BLOCKED (chip set) + {inv['provable']} provable rows over {inv['files']} call files "
         f"({'equal to the plan' if ok else 'DIFFERS FROM THE PLAN 208 = 62 + 1 + 3 + 142 over 145'}).", "",
         "| result | calls |", "|---|---:|"]
    L += [f"| {k} | {tot[k]} |" for k in cols] + [f"| total | {len(rows)} |", ""]
    cmp_ = [r["compare"] for r in rows if r["compare"]]
    L += [f"Compared after a proof: {len(cmp_)} rows; against the native Preview transcript "
          f"{sum(1 for x in cmp_ if 'native ref only' not in x)}, against the native ref only "
          f"{sum(1 for x in cmp_ if 'native ref only' in x)} (no passing native Preview transcript).", "",
          "Blocked, by reason:", "", "| reason | calls |", "|---|---:|"]
    L += [f"| {k} | {n} |" for k, n in sorted(reasons.items(), key=lambda x: -x[1])]
    L += ["", "| program | call | box | bucket | keys | ledger 9.1.0.1 | compare | Preview settle | result | reason |",
          "|---|---|---|---|---|---|---|---|---|---|"]
    for r in rows:
        L.append(f"| `{r['id']}` | `{r['label']}` | {r['box'] or '-'} | {r['bucket'] or '-'} | {r['keys'] or '-'} | "
                 f"{r['ledger'] or '-'} | {r['compare'] or '-'} | {r['settle'] or '-'} | {r['result']} | {r['reason']} |")
    return tot, "\n".join(L) + "\n"


def final(a):
    out = pathlib.Path(a.out)
    out.mkdir(parents=True, exist_ok=True)
    suite = pathlib.Path(a.suite)
    calls = suite_calls(suite)
    assign = final_assign(calls, final_files())
    rets = {}
    for d in a.final_dir:
        box, recs = read_return(d)
        rets[box] = recs
    owner = list_box({b: FINAL / f"{b}.list" for b in ("huginn", "odin", "loki")})
    ledger = read_ledger(a.ledger) if a.ledger else None
    settle_specs = list(a.settle)
    for st in sorted(pathlib.Path(a.rep_settle_dir).glob("*/ledger/t10a-state.json")) if a.rep_settle_dir else []:
        lb = st.parent.parent.name
        settle_specs.append(f"{st}={lb.split('.', 1)[0]}/{lb.split('.', 1)[1]}")
    settles = {}
    for sm in settle_specs:
        st, pc = sm.split("=", 1)
        settles[pc.replace("/", ".", 1)] = read_settle(st)
    rows = classify_final(calls, assign, rets, owner, load_native_preview(a.native_preview), suite, ledger, settles)
    inv, ok = final_invariant(rows, assign)
    meta = {"schema": "t23-final/1", "built": time.strftime("%Y-%m-%dT%H:%M:%S%z"), "returns": [str(d) for d in a.final_dir],
            "ledger": a.ledger, "settles": len(settles), "invariant": inv, "invariant_ok": ok}
    tot, md = final_table(rows, meta)
    meta["totals"] = tot
    (out / "t23-suite-final.json").write_text(json.dumps({"meta": meta, "rows": rows}, indent=1, default=str))
    (out / "T23-SUITE.md").write_text(md)
    log("T23-FINAL", json.dumps(tot), "invariant", "ok" if ok else json.dumps(inv))
    if not a.no_bench:
        cmd = [sys.executable, str(HERE / "bench.py"), "ingest", "--run", a.run, "--title", a.title, "--out", str(out / "bench")]
        for d in a.final_dir:
            cmd += ["--final-dir", str(d)]
        if a.ledger:
            cmd += ["--muninn-dir", a.ledger]
        for sm in settle_specs:
            cmd += ["--settle", sm]
        cmd += ["--derived-tx", a.derived_tx, "--upper-note", a.upper_note, "--preview-note", a.preview_note]
        r = subprocess.run(cmd)
        if r.returncode:
            return r.returncode
    if tot["mismatch"]:
        return 4
    return 0 if ok else 6


# ---------------------------------------------------------------- self test (fixtures)
def selftest():
    fx = pathlib.Path(__file__).resolve().parent / "fixtures"
    ok = True

    def check(name, cond):
        nonlocal ok
        print(("PASS " if cond else "FAIL ") + name)
        ok &= bool(cond)

    np_ = json.loads((fx / "native-preview-call.json").read_text())
    ref = json.loads((fx / "native-ref.json").read_text())
    words = [str(w) for w in np_["vectors"][0]["guaranteed_words"] + np_["vectors"][0]["fallible_words"]]
    good = {"calls": [{"index": 0, "eff": words, "comm": ref["lane1_comm"]}]}
    check("compare: equal outcome", compare(good, np_, {0: ref})[0] == "equal")
    bad = {"calls": [{"index": 0, "eff": words[:-1] + [str(int(words[-1]) + 1)], "comm": ref["lane1_comm"]}]}
    v, d = compare(bad, np_, {0: ref})
    check(f"compare: planted word difference -> different ({d})", v == "different" and f"word {len(words) - 1}" in d)
    badc = {"calls": [{"index": 0, "eff": words, "comm": str(int(ref["lane1_comm"]) + 1)}]}
    check("compare: planted comm difference -> different", compare(badc, np_, {0: ref})[0] == "different")
    check("compare: no native transcript -> no-native", compare(good, {"vectors": []}, {0: ref})[0] == "no-native")
    lines = (fx / "all.log").read_text().splitlines()
    h = parse_bus(lines, "odin", "t01-tiny.get-set")
    check("bus: done reply parsed", h and h["kind"] == "done" and h["rest"].startswith("~/sherlock/out/t23/t01-tiny.get-set/result.json"))
    check("bus: other box ignored", parse_bus(lines, "huginn", "t01-tiny.get-set") is None)
    check("bus: fail reply parsed", (parse_bus(lines, "odin", "t02-counter-example.increment") or {}).get("kind") == "fail")
    check("bus: job prefix not confused", parse_bus(lines, "odin", "t01-tiny.get") is None)
    bp = BoxProver("odin", live=False)
    r = bp.prove({"job": "t01-tiny.get-set", "call_files": [fx / "native-ref.json"]})
    check("dispatch: dry prints commands, sends nothing", r["status"] == "blocked" and r["reason"] == STUB_BLOCK and len(r["commands"]) == 5
          and r["commands"][0] == "ssh -O check odin" and r["commands"][4].startswith("herdr --machine odin agent prompt w1:p1 ")
          and "W5_WRAP_TAIL_MIB" in r["commands"][4])
    odin_prof = {"pipeline": {"label": "DERIVED", "evict": False, "overlap": "Full", "wrap_tail": 5000 << 20, "wrap_tail_mib": 5000}}
    check("wrapd env from the profile (PF-008)", wrapd_env(odin_prof) == {"W5_WRAP_TAIL_MIB": "5000", "W5_PIPELINE": "full", "W5_EVICT": "0"}
          and "W5_WRAP_TAIL_MIB=5000 W5_PIPELINE=full W5_EVICT=0" in bp.message("j", "f", wrapd_env(odin_prof)))
    try:
        wrapd_env({"pipeline": {"evict": False, "overlap": "Full"}})
        check("wrapd env refuses a profile without wrap_tail_mib", False)
    except ValueError:
        check("wrapd env refuses a profile without wrap_tail_mib", True)
    check("stub prover blocks on T11", StubProver().prove({})["reason"] == STUB_BLOCK)
    try:
        settle_call("preview", {"result": {}}, {})
        check("settle preview refused without T23_ALLOW_PREVIEW", False)
    except SystemExit:
        check("settle preview refused without T23_ALLOW_PREVIEW", True)
    fa = pathlib.Path(__file__).resolve().parent / "fixtures-accept"
    la17 = json.loads((fa / "t17-ledger-args.json").read_text())
    rc17 = json.loads((fa / "t17-receipt.json").read_text())
    w17, a17 = accept_words(la17)
    check("settle args: T17 accepted call claim_pub recomputed (block 115,187)", str(w17["claim_pub"]) == rc17["acceptRun"]["claimPub"]
          and a17 == la17)
    try:
        accept_words({**la17, "claim_pub": str(w17["claim_pub"] + 1)})
        check("settle args: a wrong reported claim_pub is refused", False)
    except Exception as e:
        check("settle args: a wrong reported claim_pub is refused", type(e).__name__ == "Mismatch")
    pl = settle_plan("p", "c", "/x/ledger-args.json", "/x/wrap.vk")
    check("settle plan: slot 15, one ledger per program, refusals off per call",
          pl["deploy"][-2:] == ["--out", str(PREVIEW_DIR / "p")] and "--anchor-slot" in pl["call"]
          and pl["call"][pl["call"].index("--anchor-slot") + 1] == "15" and pl["call"][-2:] == ["--deploy-from", str(PREVIEW_DIR / "p")]
          and pl["env"]["T10A_REFUSALS"] == "0" and pl["env"]["T10_NET"] == "preview")
    ok &= integration(check)
    final_selftest(check)
    return 0 if ok else 1


class FixtureProver:
    """Selftest only: 'proves' a call by returning the emitted W and comm, with one planted difference."""
    name = "fixture"

    def __init__(self, plant):
        self.plant = plant

    def prove(self, job):
        calls = [json.loads(pathlib.Path(f).read_text()) for f in job["call_files"]]
        calls = [{"index": c["index"], "eff": list(c["eff"]), "comm": c["comm"]} for c in calls]
        if job["job"] == self.plant:
            calls[0]["eff"][-1] = str(int(calls[0]["eff"][-1]) + 1)
        return {"status": "proved", "result": {"calls": calls, "c_p": "1", "input_pub": "2", "outcome_pub": "3"}}


def integration(check):
    """classify -> settle (dry) -> compare -> table on real suite data: t01-tiny and the cross-contract x01, with
    mycroft-call files synthesized from the native refs and one planted-different outcome."""
    import tempfile
    suite, sj = DEF["suite"], DEF["compiler"] / "out/suite.json"
    if not (suite.exists() and sj.exists() and pathlib.Path(DEF["native_preview"]).exists()):
        print("SKIP integration (suite data not on this host)")
        return True
    res = json.loads(sj.read_text())
    progs = ("t01-tiny", "x01-composable-outer", "x02-composable-inner")
    with tempfile.TemporaryDirectory() as td:
        em = pathlib.Path(td)
        for c in res["circuits"]:
            k = next((k for k in c["results"] if k["kits"] == KITS), None)
            if not k or k["status"] != "compiled":
                continue
            for v in k["rows"]:
                if v["status"] != "equal":
                    continue
                owner_call = v.get("call_id") or c["id"]
                if owner_call not in progs:
                    continue
                rp = suite / "native" / owner_call / "transcripts" / f"{v['label']}.{v['index']}.ref.json"
                if not rp.exists():
                    continue
                ref = json.loads(rp.read_text())
                (em / f"{c['id']}.{c['circuit']}.{v['label']}.{v['index']}.{KITS}.call.json").write_text(json.dumps(
                    {"schema": "mycroft-call/1", "index": v["index"], "eff": ref["transcript_words"], "comm": ref["lane1_comm"]}))
        calls = [c for c in suite_calls(suite) if c["id"] in progs]
        rows = classify(calls, sj, em, load_native_preview(DEF["native_preview"]), suite, FixtureProver("t01-tiny.get-set"), "dry")
        meta = {"mode": "selftest", "prover": "fixture", "settle": "dry", "started": "-", "finished": "-", "preview_tx_sent": 0,
                "native_preview": "-", "compiler": {"branch": "-", "head": "-", "dirty_paths": 0, "tree_sha256": "-"}, "build": {}}
        tot, md = table(rows, meta)
    by = {(r["id"], r["label"]): r for r in rows}
    check(f"integration: planted outcome is the only mismatch ({tot})", tot["mismatch"] == 1 and by[("t01-tiny", "get-set")]["result"] == "mismatch")
    xs = [r for r in rows if r["id"] == "x01-composable-outer" and r.get("vectors", 0) > 1]
    check(f"integration: cross-contract calls resolve every vector ({len(xs)} multi-vector calls)",
          xs and all(r["result"] in ("pass", "blocked") and r["precheck"] in ("equal", "no-native") for r in xs))
    check("integration: passes need a native Preview transcript", all(r["native_preview_pass"] for r in rows if r["result"] == "pass"))
    check("integration: settle is dry with 0 Preview tx", all(r["settle"] in (None, "dry") for r in rows))
    return tot["mismatch"] == 1


def final_selftest(check):
    """final mode on a synthetic box return: a pass (t02, W and comm from the native ref), a planted word difference
    (a05), a failed chain (d06, rc 1), a key mismatch (s03), the ledger column and the forbidden-name refusal."""
    import tempfile
    suite = DEF["suite"]
    if not (suite.exists() and pathlib.Path(DEF["native_preview"]).exists()):
        print("SKIP final selftest (suite data not on this host)")
        return True
    calls = suite_calls(suite)
    files = final_files()
    assign = final_assign(calls, files)
    with tempfile.TemporaryDirectory() as td:
        d = pathlib.Path(td) / "ret-odin"
        vk = b"selftest wrap key"
        spec = {"t02-counter-example.increment.increment.0": "pass", "a05-list-field.test.test.0": "plant",
                "d06-nft.transfer.16-transfer-1-to-bob.0": "rc1", "s03-language.loop.loop.0": "keys"}
        times = []
        for f, kind in spec.items():
            pid, lab, idx = files[f]
            ref = json.loads((suite / "native" / pid / "transcripts" / f"{lab}.{idx}.ref.json").read_text())
            eff = [str(w) for w in ref["transcript_words"]]
            if kind == "plant":
                eff[-1] = str(int(eff[-1]) + 1)
            cd = d / "calls" / f
            (cd / "wrap").mkdir(parents=True)
            (cd / "rc").write_text("1\n" if kind == "rc1" else "0\n")
            (cd / "keys.out").write_text(("MISMATCH x bucket Plain" if kind == "keys" else "T25SUITE EQUAL x bucket Plain") + "\n")
            (cd / "wrap/wrap.vk").write_bytes(vk)
            if kind != "rc1":
                (cd / "prove-result.partial.json").write_text(json.dumps({"calls": [{"index": idx, "eff": eff, "comm": ref["lane1_comm"]}]}))
            times.append("\t".join(["final_chain", f.split(".", 1)[0], f.split(".", 1)[1], "odin", "GPU", "cold", "1.0",
                                    "1" if kind == "rc1" else "0", "-", "-", "-", "-", "-", f"calls/{f}/chain.log"]))
        (d / "times.tsv").write_text("\n".join(times) + "\n")
        lst = [str(x.relative_to(d)) for x in sorted(d.rglob("*")) if x.is_file()]
        subprocess.run(["sha256sum", *lst], cwd=d, stdout=open(d / "RETURN.sha256", "w"), check=True)
        box, recs = read_return(d, vk_sha=hashlib.sha256(vk).hexdigest())
        lg = pathlib.Path(td) / "ledger"
        (lg / "logs").mkdir(parents=True)
        e = {"phase": "ledger_9_1_0_1_verify", "result": "Ok(())", "changed_lane_refused": {"a": True},
             "changed_public_input_refused": [True], "flipped_proof_byte_refused": [True]}
        (lg / "logs/l.log").write_text("E5JSON " + json.dumps(e) + "\n")
        (lg / "times.tsv").write_text("\t".join(["ledger_check", "t02-counter-example", "increment.increment.0", "muninn", "CPU",
                                                 "-", "0.1", "0", "-", "-", "-", "-", "l", "logs/l.log"]) + "\n")
        led = read_ledger(lg)
        rows = classify_final(calls, assign, {box: recs}, list_box({b: FINAL / f"{b}.list" for b in ("huginn", "odin", "loki")}),
                              load_native_preview(DEF["native_preview"]), suite, led,
                              {"t02-counter-example.increment.increment.0": "acceptRun block 136144, 3 refused"})
        inv, ok = final_invariant(rows, assign)
        by = {(r["id"], r["label"]): r for r in rows}
        check(f"final: row invariant 208 = 62 + 1 + 3 + 142 over 145 files ({inv})", ok)
        t02 = by[("t02-counter-example", "increment")]
        check(f"final: t02 passes with ledger Ok and the settle block ({t02['result']}: {t02['reason']})",
              t02["result"] == "pass" and t02["ledger"] == "Ok" and t02["settle"].startswith("acceptRun block") and t02["box"] == "odin")
        check("final: planted word difference is a mismatch", by[("a05-list-field", "test")]["result"] == "mismatch")
        check("final: rc 1 is prove-failed, never pass", by[("d06-nft", "16-transfer-1-to-bob")]["result"] == "prove-failed")
        check("final: a key MISMATCH line is a mismatch", by[("s03-language", "loop")]["result"] == "mismatch"
              and by[("s03-language", "loop")]["reason"].startswith("key check"))
        check("final: a call without a return is blocked, not pass",
              by[("a01-adt-counter", "test_all")]["result"] == "blocked")
        xr = [r for r in rows if r["id"] == "x01-composable-outer" and len(r["files"]) == 2]
        check(f"final: the 3 cross-contract rows hold the outer and the inner file ({len(xr)})", len(xr) == 3)
        (d / "calls/t02-counter-example.increment.increment.0/call-lfr.json").write_text("{}")
        with open(d / "RETURN.sha256", "a") as fh:
            fh.write("0" * 64 + "  calls/t02-counter-example.increment.increment.0/call-lfr.json\n")
        try:
            read_return(d)
            check("final: a forbidden name in RETURN.sha256 is refused", False)
        except SystemExit as ex:
            check("final: a forbidden name in RETURN.sha256 is refused", "FORBIDDEN" in str(ex))
    return ok


def main():
    p = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    sub = p.add_subparsers(dest="cmd", required=True)
    r = sub.add_parser("run")
    r.add_argument("--mode", choices=["dry", "live"], default="dry")
    r.add_argument("--prover", choices=["stub", "odin", "huginn"], default="stub")
    r.add_argument("--settle", choices=["dry", "preview"], default="dry")
    r.add_argument("--dispatch", action="store_true", help="really dispatch to the box (also needs T23_ALLOW_DISPATCH=1)")
    r.add_argument("--suite", default=str(DEF["suite"]))
    r.add_argument("--compiler", default=str(DEF["compiler"]))
    r.add_argument("--native-preview", default=str(DEF["native_preview"]))
    r.add_argument("--out", default=str(DEF["out"]))
    r.add_argument("--skip-build", action="store_true")
    r.add_argument("--build-mem", default="8G")
    r.add_argument("--run-mem", default="4G")
    f = sub.add_parser("final", help="suite table from the final-run box returns, then bench.py ingest")
    f.add_argument("--final-dir", action="append", default=[], help="box return of final/final-box.sh (ret-<box>)")
    f.add_argument("--ledger", help="partb-bench.sh ledger output dir (T23B_OUT) of the returned wraps")
    f.add_argument("--settle", action="append", default=[], help="STATE.json=PROG/CALL (t10a-state.json of one ledger)")
    f.add_argument("--rep-settle-dir", help="post-t25-settle.sh rep states: DIR/<label>/ledger/t10a-state.json")
    f.add_argument("--suite", default=str(DEF["suite"]))
    f.add_argument("--native-preview", default=str(DEF["native_preview"]))
    f.add_argument("--out", default=str(CACHE / "final/suite"))
    f.add_argument("--no-bench", action="store_true")
    f.add_argument("--run", default="t23-final")
    f.add_argument("--title", default="T23 final run on the T25 keys")
    f.add_argument("--derived-tx", default="290")
    f.add_argument("--upper-note", default="not measured in the final run")
    f.add_argument("--preview-note", default="not settled: DERIVED (LEAD PICK SETTLE-REP)")
    sub.add_parser("selftest")
    a = p.parse_args()
    if a.cmd == "selftest":
        return selftest()
    if a.cmd == "final":
        return final(a)
    if a.mode == "dry" and a.settle != "dry":
        raise SystemExit("dry mode settles dry only")
    return run(a)


if __name__ == "__main__":
    sys.exit(main())
