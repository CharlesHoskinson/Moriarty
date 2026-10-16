"""AUTH regression runner for the v1.2 candidate (same-author expectations; additive to the frozen independent pack).

  python3 -I -B adapters/auth_runner.py LABEL SRC

Runs inside the controller's wrapper (adapters/controller.py). SRC is a scratch byte copy of candidate/src.
Reuses, read-only, the frozen independent harness copy (independent-pack-copy/harness: driver.match/invariants/keep/
call and world_ind.World); nothing there is edited or re-scored. Expectations come only from the frozen
adapters/AUTH-PACK-v12.json (checked against adapters/AUTH-FREEZE.json). Writes only runs/LABEL/.
"""
import hashlib
import inspect
import io
import json
import re
import resource
import subprocess
import sys
import time
import traceback
from pathlib import Path

sys.dont_write_bytecode = True
TASK = Path(__file__).resolve().parents[1]
HARN = TASK / "independent-pack-copy" / "harness"
sys.path.insert(0, str(HARN))
import driver as D  # noqa: E402  frozen independent driver (byte-identical copy; FREEZE-checked by load_pack)

W = D.W
LABEL, SRC = sys.argv[1], Path(sys.argv[2]).resolve()
OUT = TASK / "runs" / LABEL
ORIGIN = TASK / "origin" / "v11-candidate"
DATA = TASK / "candidate" / "data"


def sha(b):
    return hashlib.sha256(b).hexdigest()


def frozen_auth():
    fz = json.loads((TASK / "adapters" / "AUTH-FREEZE.json").read_bytes())
    raw = (TASK / "adapters" / "AUTH-PACK-v12.json").read_bytes()
    if sha(raw) != fz["files"]["adapters/AUTH-PACK-v12.json"]["sha256"]:
        raise SystemExit("AUTH pack differs from its freeze")
    return json.loads(raw), sha(raw)


def run_cli12(case, w, work, v11_path):
    """driver.run_cli plus the '{v11}' file; same child env, limits and result keys as the frozen driver."""
    work.mkdir(parents=True, exist_ok=False)
    files = {}
    for name, obj in (("catalogue", w.doc), ("manifest", w.manifest), ("records", w.records), ("policy", w.policy),
                      ("bundle", w.bundle)):
        if obj is None:
            continue
        p = work / f"{name}.json"
        p.write_bytes(W.doc_file_bytes(obj))
        files[name] = str(p)
    files["v0"] = str(TASK / "independent-pack-copy" / "inputs" / "v0.json")
    files["v1"] = str(TASK / "independent-pack-copy" / "inputs" / "v1.json")
    files["v11"] = str(v11_path)
    files["presentedDigest"] = w.presented_digest
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
    cli = SRC / "mcpcat" / "cli.py"
    t0 = time.monotonic()
    p = subprocess.run([sys.executable, "-I", "-B", str(cli)] + argv, cwd=work, capture_output=True, timeout=50,
                       env=D.CHILD_ENV, preexec_fn=D._limits)
    (work / "argv.json").write_text(json.dumps([sys.executable, "-I", "-B", str(cli)] + shown, indent=1) + "\n")
    (work / "stdout").write_bytes(p.stdout)
    (work / "stderr").write_bytes(p.stderr)
    (work / "exit").write_text(f"{p.returncode}\n")
    res = {"exitCode": p.returncode, "stdoutIsJson": False, "stdoutSha256": sha(p.stdout), "stdoutIsAscii": p.stdout.isascii(),
           "stderrSha256": sha(p.stderr), "stderrHasTraceback": b"Traceback" in p.stderr,
           "stderrText": p.stderr.decode("utf-8", "replace")[-800:], "wallS": round(time.monotonic() - t0, 3)}
    try:
        out = json.loads(p.stdout)
        res["stdoutIsJson"] = isinstance(out, dict)
        if isinstance(out, dict):
            res.update({k: out[k] for k in D.KEEP if k in out})
    except ValueError:
        pass
    return res


def unit_checks(mods, v12_bytes, v12, m12, v11_bytes, v11, m11, v1_bytes):
    api, catalogue, cli, mcj, semantic, pins = mods
    out = []

    def check(uid, fn):
        try:
            ok, detail = fn()
            out.append({"id": uid, "status": "pass" if ok else "FAIL", "detail": detail})
        except Exception as e:  # noqa: BLE001 - an exception is a recorded failure
            out.append({"id": uid, "status": "ERROR", "detail": f"{type(e).__name__}: {e}", "traceback": traceback.format_exc()[-1500:]})

    def data1():
        rows_eq = len(v12["rows"]) == len(v11["rows"]) == 143 and all(
            mcj.canonical_bytes(a) == mcj.canonical_bytes(b) for a, b in zip(v12["rows"], v11["rows"]))
        heads = sum(1 for r in m12["rows"] if r["current"] is True)
        old = {r["versionedId"]: r for r in m11["rows"]}
        ids_eq = all(old[r["versionedId"]]["row_digest"] == r["row_digest"]
                     and old[r["versionedId"]]["predicate_digest"] == r["predicate_digest"] for r in m12["rows"])
        return rows_eq and heads == 89 and ids_eq, {"rowsEqual": rows_eq, "currentHeads": heads, "rowIdentitiesEqual": ids_eq}

    def data2():
        built = semantic.build_manifest(json.loads(v12_bytes), v12_bytes)
        return built == m12, {"candidateBuildManifestEqualsCheckedIn": built == m12,
                              "catalogue_manifest_digest": built["catalogue_manifest_digest"]}

    def data3():
        b = v12["v11Base"]
        d11 = semantic.catalogue_manifest_digest(v11)
        d1 = semantic.catalogue_manifest_digest(json.loads(v1_bytes))
        ok = (b["fileSha256"] == sha(v11_bytes) and b["fileBytes"] == len(v11_bytes) and b["catalogueManifestDigest"] == d11
              and b["rowCount"] == len(v11["rows"]) and d11 == pins.V11_PIN["catalogueManifestDigest"]
              and d11 == "c856d17bb166d95bcb4315e4a58c9451de09d9ab09fff7532f46c5f0be28bee2"
              and d1 == "4dbe36a58a04f09e13461a28a96b43b1dc2d5e1c8043835849df0ef4c9093498")
        return ok, {"v11Digest": d11, "v1Digest": d1, "v11BaseInDoc": b}

    def mcj1():
        a, b = sha((SRC / "mcpcat" / "mcj.py").read_bytes()), sha((ORIGIN / "src" / "mcpcat" / "mcj.py").read_bytes())
        return a == b, {"candidateMcjSha256": a, "originMcjSha256": b}

    def api1():
        sig = {n: list(inspect.signature(f).parameters) for n, f in (
            ("validate_catalogue", catalogue.validate_catalogue), ("open_context", api.open_context),
            ("admission_full", api.admission_full), ("bundle_full", api.bundle_full))}
        want = {"validate_catalogue": ["doc", "manifest", "base_bytes", "expected_manifest_digest"],
                "open_context": ["doc", "manifest", "base_bytes", "policy"],
                "admission_full": ["doc", "manifest", "base_bytes", "policy", "records", "index"],
                "bundle_full": ["doc", "manifest", "base_bytes", "policy", "records", "bundle"]}
        return sig == want, sig

    def static1():
        matches, dollar = [], []
        for p in sorted((SRC / "mcpcat").glob("*.py")):
            for i, line in enumerate(p.read_text().splitlines(), 1):
                if ".match(" in line:
                    matches.append(f"{p.name}:{i}:{line.strip()}")
                for lit in re.findall(r"re\.compile\(r?\"([^\"]*)\"", line):
                    if lit.endswith("$"):
                        dollar.append(f"{p.name}:{i}:{lit}")
        ok = not dollar and all("HOST_PATH_RE.match(" in m for m in matches)
        return ok, {"matchCalls": matches, "dollarAnchoredCompiles": dollar,
                    "note": "HOST_PATH_RE.match is a prefix test, not a typed validator"}

    def l7strict():
        buf = io.BytesIO()
        stream = io.TextIOWrapper(buf, encoding="utf-8", errors="strict", newline="\n", write_through=True)
        old = sys.stdout
        sys.stdout = stream
        try:
            rc = cli.main(["validate-catalogue", "a", "b", "c", "d", "--expected-manifest-digest", "x", "\udcff"])
            stream.flush()
        finally:
            sys.stdout = old
        raw = buf.getvalue()
        obj = json.loads(raw.decode("ascii"))
        return rc == 2 and obj.get("codes") == ["CLI_USAGE"], {"exit": rc, "stdoutIsAscii": raw.isascii(), "codes": obj.get("codes"),
                                                               "detail": obj.get("detail")}

    for uid, fn in (("U-DATA-1", data1), ("U-DATA-2", data2), ("U-DATA-3", data3), ("U-MCJ-1", mcj1), ("U-API-1", api1),
                    ("U-STATIC-1", static1), ("U-L7-STRICT", l7strict)):
        check(uid, fn)
    out.append({"id": "U-NET-PROBE", "status": "SKIPPED",
                "detail": "removed before the first run: a loopback connect is itself a socket syscall and the task allows no "
                          "network action; isolation rests on the inspected unshare --net wrapper only (not demonstrated here)"})
    return out


def main():
    OUT.mkdir(parents=True, exist_ok=False)
    pack, pdata, plan, cases_ind, packsha, frozen = D.load_pack()  # verifies all 19 FREEZE pins of the copy
    auth, auth_sha = frozen_auth()
    src_before = D.tree(SRC)
    sys.path.insert(0, str(SRC))
    from mcpcat import api, catalogue, cli, mcj, pins, semantic  # noqa: E402  candidate (scratch copy only)
    imported = {m: getattr(sys.modules[m], "__file__", None) for m in sorted(sys.modules) if m.startswith("mcpcat")}
    v12_bytes = (DATA / "CARDANO-PROPERTY-CATALOGUE-v1.2.json").read_bytes()
    m12_bytes = (DATA / "CATALOGUE-v1.2-MANIFEST.json").read_bytes()
    v11_path = ORIGIN / "data" / "CARDANO-PROPERTY-CATALOGUE-v1.1.json"
    v11_bytes, m11_bytes = v11_path.read_bytes(), (ORIGIN / "data" / "CATALOGUE-v1.1-MANIFEST.json").read_bytes()
    v0_bytes = (TASK / "independent-pack-copy" / "inputs" / "v0.json").read_bytes()
    v1_path = TASK / "independent-pack-copy" / "inputs" / "v1.json"
    v1_bytes = v1_path.read_bytes()
    v12, m12, v11, m11 = (json.loads(b) for b in (v12_bytes, m12_bytes, v11_bytes, m11_bytes))
    units = unit_checks((api, catalogue, cli, mcj, semantic, pins), v12_bytes, v12, m12, v11_bytes, v11, m11, v1_bytes)
    rows = []
    for c in auth["cases"]:
        e = {"id": c["id"], "class": c["class"], "target": c["target"], "label": "AUTH"}
        t0, c0 = time.monotonic(), time.process_time()
        gdoc, gman = (v11, m11) if c.get("genuine") == "v11" else (v12, m12)
        bmode = c.get("bases", "v0v1v11")
        bases = {"v0": v0_bytes, "v1": v1_bytes}
        if bmode == "v0v1v11":
            bases["v11"] = v11_bytes
        elif bmode == "v11-is-v1":
            bases["v11"] = v1_bytes
        try:
            w = W.World(gdoc, gman, bases, pack["templates"], pdata["data"]).build(c.get("world"), c["target"])
            e["inputDigests"], e["worldNotes"] = w.input_digests(), w.notes
        except Exception as ex:  # noqa: BLE001
            e.update(disposition="HARNESS_ERROR", harnessError=f"{type(ex).__name__}: {ex}", traceback=traceback.format_exc()[-1500:])
            rows.append(e)
            continue
        try:
            if c["target"].startswith("cli"):
                res = run_cli12(c, w, OUT / "cli-work" / c["id"], v1_path if bmode == "v11-is-v1" else v11_path)
            else:
                res = D.keep(D.call(api, catalogue, c, w, bases))
        except Exception as ex:  # noqa: BLE001 - an exception is an observed result
            res = {"exception": {"type": type(ex).__name__, "message": str(ex)[:400]}, "traceback": traceback.format_exc()[-1500:]}
        if not c["target"].startswith("cli"):
            e["inputsMutatedByCall"] = w.input_digests() != e["inputDigests"]
        e["wallS"], e["cpuS"] = round(time.monotonic() - t0, 3), round(time.process_time() - c0, 3)
        e["result"] = res
        bad = D.match(c["normative"], res, c["target"]) + D.invariants(c["target"], res)
        e["normativeMismatches"] = bad
        e["disposition"] = "PASS" if not bad else "FAIL"
        rows.append(e)
    src_after = D.tree(SRC)
    counts = {}
    for r in rows:
        counts[r["disposition"]] = counts.get(r["disposition"], 0) + 1
    ru, ruc = resource.getrusage(resource.RUSAGE_SELF), resource.getrusage(resource.RUSAGE_CHILDREN)
    doc = {"mode": "auth", "label": LABEL, "qualification": "UNIT", "evidenceLabel": "AUTH (same-author expectations)",
           "python": sys.version, "authPackSha256": auth_sha, "independentPackSha256": packsha, "independentFreeze": frozen,
           "harnessSha256": {p.name: sha(p.read_bytes()) for p in sorted(HARN.glob("*.py"))},
           "src": str(SRC), "srcTreeBefore": src_before, "srcTreeAfter": src_after,
           "srcUnchanged": src_before["treeSha256"] == src_after["treeSha256"] and not src_after["pycacheDirs"],
           "importedModules": imported,
           "observedLimits": {"RLIMIT_CPU": resource.getrlimit(resource.RLIMIT_CPU), "RLIMIT_AS": resource.getrlimit(resource.RLIMIT_AS)},
           "rusage": {"selfCpuS": round(ru.ru_utime + ru.ru_stime, 3), "selfMaxRssKiB": ru.ru_maxrss,
                      "childrenCpuS": round(ruc.ru_utime + ruc.ru_stime, 3), "childrenMaxRssKiB": ruc.ru_maxrss},
           "unitChecks": units, "caseCounts": counts, "cases": len(rows), "rows": rows}
    (OUT / "AUTH-RESULTS.json").write_text(json.dumps(doc, indent=1, ensure_ascii=True) + "\n")
    print(json.dumps({"caseCounts": counts, "unit": {u["id"]: u["status"] for u in units}, "srcUnchanged": doc["srcUnchanged"]}))
    sys.exit(0 if all(u["status"] in ("pass", "SKIPPED") for u in units) and counts.get("PASS", 0) == len(rows) else 1)


if __name__ == "__main__":
    main()
