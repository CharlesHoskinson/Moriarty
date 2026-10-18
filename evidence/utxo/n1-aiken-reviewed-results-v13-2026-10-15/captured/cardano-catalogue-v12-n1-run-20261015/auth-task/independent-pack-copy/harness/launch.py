"""Resource-limited launcher for one driver batch.

  python3 -I -B harness/launch.py LABEL BATCH

Spawns  python3 -I -B harness/driver.py run LABEL BATCH scratch/oldv11-src old  as a child with
RLIMIT_CPU=60s (soft=hard), RLIMIT_AS=1GiB and a 120 s wall limit (process-group kill on timeout), env
{PYTHONDONTWRITEBYTECODE=1, LC_ALL=C.UTF-8, PATH=/usr/bin:/bin}. Hashes the ORIGINAL old candidate tree and the
scratch copy before and after. Writes runs/LABEL/COMMAND-NN.json plus raw stdout/stderr; never overwrites.
"""
import datetime
import hashlib
import json
import os
import resource
import signal
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
OLD_CANDIDATE = Path("/home/charl/moriarty-eutxo-validation-2026-10-12/cardano-catalogue-v11-hardening-20261015/candidate")
SCRATCH_SRC = ROOT / "scratch" / "oldv11-src"
ENV = {"PYTHONDONTWRITEBYTECODE": "1", "LC_ALL": "C.UTF-8", "PATH": "/usr/bin:/bin"}
CPU_S, AS_B, WALL_S = 60, 1 << 30, 120


def sha(b):
    return hashlib.sha256(b).hexdigest()


def tree(base):
    h, n, caches = hashlib.sha256(), 0, []
    for p in sorted(Path(base).rglob("*")):
        if p.is_dir() and p.name == "__pycache__":
            caches.append(str(p.relative_to(base)))
        if p.is_file():
            h.update(str(p.relative_to(base)).encode() + b"\0" + sha(p.read_bytes()).encode() + b"\n")
            n += 1
    return {"files": n, "treeSha256": h.hexdigest(), "pycacheDirs": caches}


def limits():
    os.setsid()
    resource.setrlimit(resource.RLIMIT_CPU, (CPU_S, CPU_S))
    resource.setrlimit(resource.RLIMIT_AS, (AS_B, AS_B))


def main():
    label, batch = sys.argv[1], int(sys.argv[2])
    out = ROOT / "runs" / label
    out.mkdir(parents=True, exist_ok=True)
    rec_path = out / f"COMMAND-{batch:02d}.json"
    if rec_path.exists():
        raise SystemExit(f"{rec_path} exists")
    argv = [sys.executable, "-I", "-B", str(HERE / "driver.py"), "run", label, str(batch), str(SCRATCH_SRC), "old"]
    old_before, scr_before = tree(OLD_CANDIDATE), tree(SCRATCH_SRC)
    ru0 = resource.getrusage(resource.RUSAGE_CHILDREN)
    t0 = time.monotonic()
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    p = subprocess.Popen(argv, cwd=ROOT, env=ENV, stdout=subprocess.PIPE, stderr=subprocess.PIPE, preexec_fn=limits)
    timed_out = False
    try:
        so, se = p.communicate(timeout=WALL_S)
    except subprocess.TimeoutExpired:
        timed_out = True
        os.killpg(p.pid, signal.SIGKILL)
        so, se = p.communicate()
    wall = time.monotonic() - t0
    ru1 = resource.getrusage(resource.RUSAGE_CHILDREN)
    old_after, scr_after = tree(OLD_CANDIDATE), tree(SCRATCH_SRC)
    (out / f"COMMAND-{batch:02d}.stdout").write_bytes(so)
    (out / f"COMMAND-{batch:02d}.stderr").write_bytes(se)
    rec = {"label": label, "batch": batch, "startedAt": started,
           "finishedAt": datetime.datetime.now(datetime.timezone.utc).isoformat(),
           "argv": argv, "cwd": str(ROOT), "env": ENV,
           "requestedLimits": {"RLIMIT_CPU_s": CPU_S, "RLIMIT_AS_bytes": AS_B, "wall_s": WALL_S},
           "exitCode": p.returncode, "timedOut": timed_out, "wallS": round(wall, 3),
           "childrenCpuS": round((ru1.ru_utime - ru0.ru_utime) + (ru1.ru_stime - ru0.ru_stime), 3),
           "childrenMaxRssKiB": ru1.ru_maxrss,
           "stdoutSha256": sha(so), "stderrSha256": sha(se), "stdoutBytes": len(so), "stderrBytes": len(se),
           "oldCandidateBefore": old_before, "oldCandidateAfter": old_after,
           "oldCandidateUnchanged": old_before == old_after,
           "scratchBefore": scr_before, "scratchAfter": scr_after, "scratchUnchanged": scr_before == scr_after}
    bfile = out / f"batch-{batch:02d}.json"
    rec["batchResultSha256"] = sha(bfile.read_bytes()) if bfile.exists() else None
    rec_path.write_text(json.dumps(rec, indent=1) + "\n")
    print(json.dumps({k: rec[k] for k in ("batch", "exitCode", "timedOut", "wallS", "childrenCpuS",
                                          "oldCandidateUnchanged", "scratchUnchanged")}))
    print(so.decode("utf-8", "replace")[-1500:])
    if se:
        print("STDERR:", se.decode("utf-8", "replace")[-1500:])


if __name__ == "__main__":
    main()
