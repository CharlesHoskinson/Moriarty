#!/usr/bin/env python3
"""T23 Part A: rerun the T18 (a) Compact suite natively on Preview, beside (never over) the 2026-11-14 baseline.

  parta.py gate                 the Preview indexer must answer 200 three times in a row (exit 0), else exit 2
  parta.py setup                run tree ~/.cache/t23/partA/suite (a copy of the suite tools, native refs, scenarios;
                                build/ is a link to the T18 (a) full build), empty receipts, own harness home
                                ~/.cache/t23/partA/home; seeds the state of each REUSED program (see plan)
  parta.py plan                 per program: reuse or fresh deploy, and the planned Preview transaction count
  parta.py count                actual Preview transactions of this rerun, from the new harness state

LEAD PICK (reuse rule): a T18 (a) instance is reused when its contract state now equals its state at the deploy
block (preview_probe.py, midnight_contractState at both blocks; no callees), so the scenario runs against the same
state as native. Every other program gets a fresh deploy, because its scenario changed the instance's state on
2026-11-14 and a stateful rerun on it cannot be compared with the native refs. No program is redeployed because the
chain changed: preview_probe.py shows the same genesis, spec, ledger and block hashes.
"""
import json
import pathlib
import shutil
import sys
import time
import urllib.error
import urllib.request

H = pathlib.Path.home()
SRC = H / "Projects/Moriarty-wt/mycroft/mycroft/tests/compact-suite"
OLD_STATE = H / ".cache/mycroft-preview/state"
A = H / ".cache/t23/partA"
RUN, HOME = A / "suite", A / "home"
INDEXER = "https://indexer.preview.midnight.network/api/v4/graphql"
SUBMITTED = ("included", "refused-or-failed-after-submit")


def programs():
    comp = json.loads((SRC / "build/full/compile.json").read_text())
    return sorted(k for k, v in (comp.get("programs", comp)).items() if isinstance(v, dict) and v.get("status") == "ok") \
        if isinstance(comp, dict) else []


def old(id_):
    f = OLD_STATE / f"{id_}.json"
    return json.loads(f.read_text()) if f.exists() else None


PROBE = A / "probe.json"


def plan():
    probe = {r["id"]: r for r in json.loads(PROBE.read_text())["contracts"] if r["file"] == f"{r['id']}.json"}
    rows, tot = [], {"deploys": 0, "callees": 0, "keyInserts": 0, "calls": 0, "nodeRefusedSubmissions": 0}
    for id_ in programs():
        s = old(id_) or {"calls": {}}
        v = [c.get("verdict") for c in s.get("calls", {}).values()]
        inc, nref = v.count("included"), v.count("refused-or-failed-after-submit")
        pr = probe.get(id_, {})
        reuse = bool(pr.get("stateEqualsDeployBlockState")) and not s.get("callees") and bool(s.get("deploy", {}).get("address"))
        r = {"id": id_, "mode": "reuse" if reuse else "fresh",
             "why": ("contract state now equals its state at the deploy block (probe)" if reuse
                     else "contract state changed by the 2026-11-14 scenario (probe)" if pr else "no probe record"),
             "deploys": 0 if reuse else 1, "callees": 0 if reuse else len(s.get("callees", {})),
             "keyInserts": 0 if reuse else len((s.get("stagedDeploy") or {}).get("inserts", [])),
             "calls": inc, "nodeRefusedSubmissions": nref}
        r["tx"] = sum(r[k] for k in tot)
        for k in tot:
            tot[k] += r[k]
        rows.append(r)
    tot["tx"] = sum(tot.values())
    return rows, tot


def tip():
    req = urllib.request.Request("https://rpc.preview.midnight.network",
                                 b'{"id":1,"jsonrpc":"2.0","method":"chain_getHeader","params":[]}', {"content-type": "application/json"})
    with urllib.request.urlopen(req, timeout=40) as r:
        return int(json.loads(r.read())["result"]["number"], 16)


def gate():
    """Three answers in a row, each with the indexer height within 10 blocks of the node tip."""
    ok, seen = 0, []
    for i in range(3):
        req = urllib.request.Request(INDEXER, b'{"query":"{ block { height } }"}', {"content-type": "application/json"})
        try:
            with urllib.request.urlopen(req, timeout=40) as r:
                h = json.loads(r.read())["data"]["block"]["height"]
            t = tip()
            seen.append(f"{h}/{t}")
            ok += r.status == 200 and t - h <= 10
        except Exception as e:
            seen.append(str(getattr(e, "code", type(e).__name__)))
        time.sleep(5)
    print(f"indexer gate: {ok}/3 ({', '.join(seen)}) at {time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())}", flush=True)
    return 0 if ok == 3 else 2


def setup():
    RUN.mkdir(parents=True, exist_ok=True)
    for d in ("tools", "native", "scenarios", "programs"):
        if not (RUN / d).exists():
            shutil.copytree(SRC / d, RUN / d, symlinks=True)
    # the 2026-11-14 per-call verdict overrides would hide a changed outcome: the rerun reports raw verdicts
    nf = RUN / "tools/preview/notes.json"
    n = json.loads(nf.read_text())
    if n.get("calls") or n.get("findings"):
        nf.write_text(json.dumps({**n, "calls": {}, "findings": [], "overridesDroppedFrom20261114": n.get("calls")}, indent=1))
    for f in ("MANIFEST.json", "MANIFEST.md"):
        shutil.copy2(SRC / f, RUN / f)
    if not (RUN / "build").exists():
        (RUN / "build").symlink_to(SRC / "build")
    (RUN / "preview").mkdir(exist_ok=True)
    (HOME / "state").mkdir(parents=True, exist_ok=True)
    rows, _ = plan()
    for r in rows:
        dst = HOME / "state" / f"{r['id']}.json"
        if r["mode"] == "reuse" and not dst.exists():
            s = old(r["id"])
            dst.write_text(json.dumps({"id": r["id"], "calls": {}, "deploy": s["deploy"], "reusedFrom": "2026-11-14"}, indent=1))
    print(f"setup: {RUN} (receipts {RUN / 'preview'}), home {HOME}; reused seeded: {sum(r['mode'] == 'reuse' for r in rows)}")


def count():
    tot = {"deploys": 0, "callees": 0, "keyInserts": 0, "calls": 0, "nodeRefusedSubmissions": 0}
    for f in sorted((HOME / "state").glob("*.json")):
        if f.name.endswith(".txcmp.json") or f.name.startswith("start-"):
            continue
        s = json.loads(f.read_text())
        if s.get("deploy", {}).get("address") and not s.get("reusedFrom"):
            tot["deploys"] += 1
        tot["callees"] += sum(1 for c in (s.get("callees") or {}).values() if c.get("txHash"))
        tot["keyInserts"] += sum(1 for k in (s.get("stagedDeploy") or {}).get("inserts", []) if k.get("txHash"))
        v = [c.get("verdict") for c in s.get("calls", {}).values()]
        tot["calls"] += v.count("included")
        tot["nodeRefusedSubmissions"] += v.count("refused-or-failed-after-submit")
        tot["pendingUnknown"] = tot.get("pendingUnknown", 0) + (1 if s.get("pending") else 0)
    tot["tx"] = sum(tot.values())
    print(json.dumps(tot))


if __name__ == "__main__":
    cmd = sys.argv[1]
    if cmd == "gate":
        sys.exit(gate())
    if cmd == "setup":
        setup()
    elif cmd == "plan":
        rows, tot = plan()
        for r in rows:
            print(f"{r['id']:28} {r['mode']:5} tx {r['tx']:3}  ({r['why']})")
        print("PLANNED PREVIEW TX:", json.dumps(tot))
    elif cmd == "count":
        count()
