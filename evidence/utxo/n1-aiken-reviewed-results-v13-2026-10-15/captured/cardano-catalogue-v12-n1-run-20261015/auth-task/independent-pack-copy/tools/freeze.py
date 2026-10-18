"""Freeze the pack, harness, tools and data inputs BEFORE any import or execution of the checker under test.

Writes pack/FREEZE.json (refuses to overwrite), marks every frozen file read-only, and appends a chronology
event. Usage: python3 -I -B tools/freeze.py
"""
import datetime
import hashlib
import json
import os
import stat
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FROZEN = (["pack/independent-pack.json", "pack/positive-controls.json", "pack/policy-grant-data.json",
           "pack/RUN-PLAN.json"]
          + [f"harness/{n}" for n in ("driver.py", "world_ind.py", "mcj_ind.py", "launch.py")]
          + [f"tools/{n}" for n in ("chrono.py", "copy_inputs.py", "selfcheck_world.py", "gen_pack.py", "freeze.py",
                                     "copy_checker.py")]
          + [f"inputs/{n}" for n in ("catalogue-v1.1.json", "manifest-v1.1.json", "v0.json", "v1.json",
                                     "COPY-RECORD.json")])


def main():
    out = ROOT / "pack" / "FREEZE.json"
    if out.exists():
        raise SystemExit("FREEZE.json exists")
    if (ROOT / "scratch").exists() or (ROOT / "runs").exists():
        raise SystemExit("scratch/ or runs/ already exists: freeze must precede any checker copy or run")
    files = {rel: hashlib.sha256((ROOT / rel).read_bytes()).hexdigest() for rel in FROZEN}
    chrono = (ROOT / "CHRONOLOGY.jsonl").read_bytes()
    pack = json.loads((ROOT / "pack/independent-pack.json").read_bytes())
    ctl = json.loads((ROOT / "pack/positive-controls.json").read_bytes())
    rec = {
        "frozenAt": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "frozenBy": "claude-opus-5-5 (requested model; self-reported identity), independent falsifier author",
        "statement": "Frozen before the checker under test was copied, imported or executed. No scratch/ or runs/ directory existed at freeze time. Expectations were written from the review/audit/spec text; predictedOld values are pre-execution code-reading predictions kept separate from the normative expectations.",
        "caseCounts": {"cases": len(pack["cases"]), "controls": len(ctl["cases"]),
                       "total": len(pack["cases"]) + len(ctl["cases"])},
        "chronologySha256AtFreeze": hashlib.sha256(chrono).hexdigest(), "chronologyBytesAtFreeze": len(chrono),
        "python": sys.version,
        "files": files,
    }
    out.write_text(json.dumps(rec, indent=1) + "\n")
    for rel in FROZEN + ["pack/FREEZE.json"]:
        p = ROOT / rel
        os.chmod(p, stat.S_IRUSR | stat.S_IRGRP | stat.S_IROTH)
    print(json.dumps({"freezeSha256": hashlib.sha256(out.read_bytes()).hexdigest(), **rec["caseCounts"],
                      "frozenFiles": len(files)}))


if __name__ == "__main__":
    main()
