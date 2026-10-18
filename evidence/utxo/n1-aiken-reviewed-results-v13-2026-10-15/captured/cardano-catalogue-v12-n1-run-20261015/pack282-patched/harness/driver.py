"""Independent falsifier driver.

  python3 -I -B harness/driver.py dry  LABEL [PACKDIR]       materialize every case world; NEVER imports the checker
  python3 -I -B harness/driver.py run  LABEL BATCH SRC [IMPL] run one RUN-PLAN batch against SRC/mcpcat (IMPL=old|fixed)

'run' imports the checker only from SRC (a scratch copy), with bytecode writing disabled. Expectations are
read from the frozen pack files; nothing here computes or adapts an expectation. Results go to
runs/LABEL/batch-NN.json and are never overwritten.
"""
import copy
import datetime
import hashlib
import json
import os
import re
import resource
import subprocess
import sys
import time
import traceback
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
sys.path.insert(0, str(HERE))
import world_ind as W  # noqa: E402

PACK = ROOT / "pack"
NEVER = {"VERIFIED", "ACCEPTED", "PROOF_VERIFIED"}
KEEP = ("outcome", "codes", "labels", "historicalWarnings", "stats", "qualification", "proofVerification",
        "ledgerAcceptance", "cryptographicVerification", "bundleDigest", "admission_digest", "catalogueManifestDigest",
        "detail")
PATTERN = {"catalogue": r"^CAT_[A-Z0-9_]+$", "context": r"^(CTX|CAT)_[A-Z0-9_]+$",
           "admission": r"^(CTX|CAT|ADM)_[A-Z0-9_]+$", "bundle": r"^(CTX|CAT|ADM|BND)_[A-Z0-9_]+$"}
CHILD_ENV = {"PYTHONDONTWRITEBYTECODE": "1", "LC_ALL": "C.UTF-8", "PATH": "/usr/bin:/bin"}
CPU_S, AS_B = 60, 1 << 30


def sha(b):
    return hashlib.sha256(b).hexdigest()


def tree(base):
    base = Path(base)
    h, n, caches = hashlib.sha256(), 0, []
    for p in sorted(base.rglob("*")):
        if p.is_dir() and p.name == "__pycache__":
            caches.append(str(p.relative_to(base)))
        if p.is_file():
            h.update(str(p.relative_to(base)).encode() + b"\0" + sha(p.read_bytes()).encode() + b"\n")
            n += 1
    return {"files": n, "treeSha256": h.hexdigest(), "pycacheDirs": caches}


def load_pack(pack_dir=None):
    global PACK
    if pack_dir:
        PACK = ROOT / pack_dir
    files = ["independent-pack.json", "positive-controls.json", "policy-grant-data.json", "RUN-PLAN.json"]
    raw = {f: (PACK / f).read_bytes() for f in files}
    freeze = PACK / "FREEZE.json"
    frozen = None
    if freeze.exists():
        fz = json.loads(freeze.read_bytes())
        want = fz["files"]
        bad = []
        for rel, h in want.items():
            if sha((ROOT / rel).read_bytes()) != h:
                bad.append(rel)
        frozen = {"freezeSha256": sha(freeze.read_bytes()), "mismatches": bad}
        if bad:
            raise SystemExit(f"FROZEN FILE MISMATCH {bad}")
    pack = json.loads(raw["independent-pack.json"])
    ctl = json.loads(raw["positive-controls.json"])
    pdata = json.loads(raw["policy-grant-data.json"])
    plan = json.loads(raw["RUN-PLAN.json"])
    cases = {c["id"]: c for c in pack["cases"] + ctl["cases"]}
    return pack, pdata, plan, cases, {f: sha(b) for f, b in raw.items()}, frozen


def inputs():
    i = ROOT / "inputs"
    doc = json.loads((i / "catalogue-v1.1.json").read_bytes())
    man = json.loads((i / "manifest-v1.1.json").read_bytes())
    bases = {"v0": (i / "v0.json").read_bytes(), "v1": (i / "v1.json").read_bytes()}
    return doc, man, bases


def world(case, pack, pdata, gdoc, gman, bases):
    w = W.World(gdoc, gman, bases, pack["templates"], pdata["data"])
    return w.build(case.get("world"), case["target"])


# ------------------------------------------------------------ matching
def match(exp, res, target):
    bad = []
    if "exception" in res:
        return [f"exception {res['exception']['type']}"]
    if target.startswith("cli"):
        if not res.get("stdoutIsJson"):
            bad.append("stdout is not exactly one JSON object")
        if res.get("stderrHasTraceback"):
            bad.append("stderr has a traceback")
        if "exitCode" in exp and res.get("exitCode") != exp["exitCode"]:
            bad.append(f"exitCode {res.get('exitCode')} != {exp['exitCode']}")
    if "outcome" in exp and res.get("outcome") != exp["outcome"]:
        bad.append(f"outcome {res.get('outcome')} != {exp['outcome']}")
    codes = set(res.get("codes") or [])
    for c in exp.get("codesInclude", []):
        if c not in codes:
            bad.append(f"missing code {c}")
    for c in exp.get("codesExclude", []):
        if c in codes:
            bad.append(f"forbidden code {c}")
    if exp.get("codesAnyOf") and not codes & set(exp["codesAnyOf"]):
        bad.append(f"none of {exp['codesAnyOf']}")
    if "codesExactly" in exp and sorted(codes) != sorted(exp["codesExactly"]):
        bad.append(f"codes {sorted(codes)} != exactly {exp['codesExactly']}")
    labels = set(res.get("labels") or [])
    for lab in exp.get("labelsInclude", []):
        if lab not in labels:
            bad.append(f"missing label {lab}")
    for k, v in exp.get("mustReport", {}).items():
        if res.get(k) != v:
            bad.append(f"{k} {res.get(k)!r} != {v!r}")
    for k, v in exp.get("stats", {}).items():
        if (res.get("stats") or {}).get(k) != v:
            bad.append(f"stats.{k} {(res.get('stats') or {}).get(k)!r} != {v!r}")
    hw = set(res.get("historicalWarnings") or [])
    for x in exp.get("historicalWarningsInclude", []):
        if x not in hw:
            bad.append(f"missing historical warning {x}")
    return bad


def invariants(target, res):
    bad = []
    if "exception" in res:
        return bad
    if res.get("outcome") in NEVER:
        bad.append("forbidden outcome")
    if target == "bundle" and (res.get("proofVerification") != "NOT_PERFORMED" or res.get("ledgerAcceptance") != "NOT_ESTABLISHED"):
        bad.append("bundle result lacks NOT_PERFORMED/NOT_ESTABLISHED")
    if target == "admission" and (res.get("cryptographicVerification") != "NOT_PERFORMED" or res.get("ledgerAcceptance") != "NOT_ESTABLISHED"):
        bad.append("admission result lacks NOT_PERFORMED/NOT_ESTABLISHED")
    if not target.startswith("cli") and res.get("qualification") != "UNIT":
        bad.append("qualification is not UNIT")
    pat = PATTERN.get(target)
    if pat:
        for c in res.get("codes") or []:
            if not re.match(pat, c):
                bad.append(f"untyped/foreign code {c}")
    return bad


def predicted_match(pred, res, target):
    if pred is None:
        return None
    k = pred["kind"]
    if k == "NOT_APPLICABLE_SCHEMA":
        return None
    if k == "EXCEPTION":
        ex = res.get("exception")
        return bool(ex and ex["type"] == pred["type"] and pred.get("messageContains", "") in ex["message"])
    if k == "CLI_CRASH":
        return (res.get("exitCode") == pred["exitCode"] and pred["stderrContains"] in res.get("stderrText", "")
                and not res.get("stdoutIsJson"))
    exp = {x: y for x, y in pred.items() if x != "kind"}
    if k == "CLI":
        # an old CLI result may be a structured internal error; only the predicted fields are compared
        if "exception" in res:
            return False
        sub = dict(res, stderrHasTraceback=False)
        return not match(exp, sub, target)
    return not match(exp, res, target)


# ------------------------------------------------------------ calls
def call(api, cat, case, w, bases):
    t = case["target"]
    if t == "catalogue":
        return cat.validate_catalogue(w.doc, w.manifest, bases, w.expected_digest)
    if t == "context":
        r = api.open_context(w.doc, w.manifest, bases, w.policy)
        return {k: v for k, v in r.items() if k != "context"}
    if t == "admission":
        return api.admission_full(w.doc, w.manifest, bases, w.policy, w.records, case["index"])
    if t == "bundle":
        return api.bundle_full(w.doc, w.manifest, bases, w.policy, w.records, w.bundle)
    raise W.HarnessError(f"target {t}")


def _limits():
    resource.setrlimit(resource.RLIMIT_CPU, (CPU_S, CPU_S))
    resource.setrlimit(resource.RLIMIT_AS, (AS_B, AS_B))


def run_cli(src, case, w, work):
    work.mkdir(parents=True, exist_ok=False)
    files = {}
    for name, obj in (("catalogue", w.doc), ("manifest", w.manifest), ("records", w.records), ("policy", w.policy),
                      ("bundle", w.bundle)):
        if obj is None:
            continue
        p = work / f"{name}.json"
        p.write_bytes(W.doc_file_bytes(obj))
        files[name] = str(p)
    files["v0"] = str(ROOT / "inputs" / "v0.json")
    files["v1"] = str(ROOT / "inputs" / "v1.json")
    files["presentedDigest"] = w.presented_digest
    files["genuineDigest"] = w.genuine_digest
    argv, shown = [], []
    for a in case["argv"]:
        if isinstance(a, dict) and "$bytesHex" in a:
            argv.append(bytes.fromhex(a["$bytesHex"]))
            shown.append({"bytesHex": a["$bytesHex"]})
            continue
        if isinstance(a, str) and a.startswith("{") and a.endswith("}") and a[1:-1] in files:
            a = files[a[1:-1]]
        argv.append(a)
        shown.append(a)
    cli = Path(src) / "mcpcat" / "cli.py"
    cmd = [sys.executable, "-I", "-B", str(cli)] + argv
    t0 = time.monotonic()
    p = subprocess.run(cmd, cwd=work, capture_output=True, timeout=50, env=CHILD_ENV, preexec_fn=_limits)
    wall = time.monotonic() - t0
    (work / "argv.json").write_text(json.dumps([sys.executable, "-I", "-B", str(cli)] + shown, indent=1) + "\n")
    (work / "stdout").write_bytes(p.stdout)
    (work / "stderr").write_bytes(p.stderr)
    (work / "exit").write_text(f"{p.returncode}\n")
    res = {"exitCode": p.returncode, "stdoutIsJson": False, "stdoutSha256": sha(p.stdout),
           "stderrSha256": sha(p.stderr), "stderrHasTraceback": b"Traceback" in p.stderr,
           "stderrText": p.stderr.decode("utf-8", "replace")[-800:], "wallS": round(wall, 3),
           "inputFileSha256": {k: sha(Path(v).read_bytes()) for k, v in files.items()
                               if k in ("catalogue", "manifest", "records", "policy", "bundle", "v0", "v1")}}
    try:
        out = json.loads(p.stdout)
        res["stdoutIsJson"] = isinstance(out, dict)
        if isinstance(out, dict):
            for k in KEEP:
                if k in out:
                    res[k] = out[k]
    except ValueError:
        pass
    return res


def keep(r):
    if not isinstance(r, dict):
        return {"outcome": None, "raw": repr(r)[:300]}
    out = {k: r[k] for k in KEEP if k in r}
    if "errors" in r and isinstance(r["errors"], list):
        out["errorsHead"] = r["errors"][:12]
    if "notes" in r and isinstance(r["notes"], list):
        out["notesHead"] = [str(x)[:300] for x in r["notes"][:8]]
    return out


def main():
    mode, label = sys.argv[1], sys.argv[2]
    pack, pdata, plan, cases, packsha, frozen = load_pack(sys.argv[3] if mode == "dry" and len(sys.argv) > 3 else None)
    gdoc, gman, bases = inputs()
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    if mode == "dry":
        rows, errs = [], []
        for cid, c in cases.items():
            t0 = time.process_time()
            try:
                w = world(c, pack, pdata, gdoc, gman, bases)
                rows.append({"id": cid, "ok": True, "notes": w.notes, "inputDigests": w.input_digests(),
                             "cpuS": round(time.process_time() - t0, 3)})
            except Exception as e:  # noqa: BLE001
                errs.append(cid)
                rows.append({"id": cid, "ok": False, "error": f"{type(e).__name__}: {e}",
                             "traceback": traceback.format_exc()[-1200:]})
        planned = [i for b in plan["batches"] for i in b]
        out = {"mode": "dry", "label": label, "startedAt": started, "packSha256": packsha, "frozen": frozen,
               "checkerImported": "mcpcat" in sys.modules, "cases": len(cases),
               "planCoversAll": sorted(planned) == sorted(cases), "harnessErrors": errs, "rows": rows}
        dst = ROOT / "prefreeze" / f"dry-{label}.json"
        dst.parent.mkdir(exist_ok=True)
        if dst.exists():
            raise SystemExit(f"{dst} exists")
        dst.write_text(json.dumps(out, indent=1, ensure_ascii=False) + "\n")
        print(json.dumps({k: out[k] for k in ("cases", "planCoversAll", "harnessErrors", "checkerImported")}))
        return
    batch, src = int(sys.argv[3]), Path(sys.argv[4]).resolve()
    impl = sys.argv[5] if len(sys.argv) > 5 else "old"
    if frozen is None:
        raise SystemExit("refusing to run the checker before pack/FREEZE.json exists")
    outdir = ROOT / "runs" / label
    outdir.mkdir(parents=True, exist_ok=True)
    dst = outdir / f"batch-{batch:02d}.json"
    if dst.exists():
        raise SystemExit(f"{dst} exists")
    limits = {"RLIMIT_CPU": resource.getrlimit(resource.RLIMIT_CPU), "RLIMIT_AS": resource.getrlimit(resource.RLIMIT_AS)}
    src_before = tree(src)
    sys.path.insert(0, str(src))
    from mcpcat import api, catalogue  # noqa: E402  (checker under test, scratch copy only)
    imported = {m: getattr(sys.modules[m], "__file__", None) for m in sorted(sys.modules) if m.startswith("mcpcat")}
    rows = []
    for cid in plan["batches"][batch]:
        c = cases[cid]
        e = {"id": cid, "class": c["class"], "target": c["target"], "schemaGeneration": c["schemaGeneration"]}
        t0, c0 = time.monotonic(), time.process_time()
        try:
            w = world(c, pack, pdata, gdoc, gman, bases)
            e["inputDigests"] = w.input_digests()
            e["worldNotes"] = w.notes
        except Exception as ex:  # noqa: BLE001
            e.update(disposition="HARNESS_ERROR", harnessError=f"{type(ex).__name__}: {ex}",
                     traceback=traceback.format_exc()[-1500:])
            rows.append(e)
            continue
        try:
            if c["target"].startswith("cli"):
                res = run_cli(src, c, w, outdir / "cli-work" / cid)
            else:
                res = keep(call(api, catalogue, c, w, bases))
        except Exception as ex:  # noqa: BLE001 - an exception is an observed result
            res = {"exception": {"type": type(ex).__name__, "message": str(ex)[:400]},
                   "traceback": traceback.format_exc()[-1500:]}
        if not c["target"].startswith("cli"):
            e["inputDigestsAfterCall"] = w.input_digests()
            e["inputsMutatedByCall"] = e["inputDigestsAfterCall"] != e["inputDigests"]
        e["wallS"], e["cpuS"] = round(time.monotonic() - t0, 3), round(time.process_time() - c0, 3)
        e["result"] = res
        norm = c["normative"]
        nbad = match(norm, res, c["target"]) + invariants(c["target"], res)
        e["normativeMismatches"] = nbad
        if c["schemaGeneration"] == "future" and impl == "old":
            e["disposition"] = "NOT_APPLICABLE_SCHEMA"
            e["note"] = "original v1.1 lacks the future policy schema; the raw observation implies nothing about a fix"
        elif impl == "old":
            pm = predicted_match(c["predictedOld"], res, c["target"])
            e["predictedOldMatched"] = pm
            if pm is None:
                e["disposition"] = "OBSERVED_UNPREDICTED"
            elif pm is False:
                e["disposition"] = "DISCREPANCY"
            else:
                e["disposition"] = "OLD_MEETS_NORM" if not nbad else "COUNTEREXAMPLE_REPRODUCED"
        else:
            e["disposition"] = "PASS" if not nbad else "FAIL"
        rows.append(e)
    src_after = tree(src)
    ru_self = resource.getrusage(resource.RUSAGE_SELF)
    ru_ch = resource.getrusage(resource.RUSAGE_CHILDREN)
    out = {"mode": "run", "impl": impl, "label": label, "batch": batch, "startedAt": started,
           "finishedAt": datetime.datetime.now(datetime.timezone.utc).isoformat(), "python": sys.version,
           "packSha256": packsha, "frozen": frozen,
           "harnessSha256": {p.name: sha(p.read_bytes()) for p in sorted(HERE.glob("*.py"))},
           "src": str(src), "srcTreeBefore": src_before, "srcTreeAfter": src_after,
           "srcUnchanged": src_before["treeSha256"] == src_after["treeSha256"] and not src_after["pycacheDirs"],
           "importedModules": imported, "observedLimits": limits,
           "rusage": {"selfCpuS": round(ru_self.ru_utime + ru_self.ru_stime, 3), "selfMaxRssKiB": ru_self.ru_maxrss,
                      "childrenCpuS": round(ru_ch.ru_utime + ru_ch.ru_stime, 3), "childrenMaxRssKiB": ru_ch.ru_maxrss},
           "counts": {}, "rows": rows}
    for r in rows:
        out["counts"][r["disposition"]] = out["counts"].get(r["disposition"], 0) + 1
    dst.write_text(json.dumps(out, indent=1, ensure_ascii=False) + "\n")
    print(json.dumps({"batch": batch, "counts": out["counts"], "srcUnchanged": out["srcUnchanged"],
                      "rusage": out["rusage"], "observedLimits": limits}))


if __name__ == "__main__":
    main()
