"""One changed-candidate metadata check; no linking or test execution."""
import datetime
import hashlib
import json
import shutil
import subprocess
import time
from pathlib import Path

OUT = Path(__file__).resolve().parent
R = OUT.parent
OLD = R / "66-compiler-preflight-metadata"
SNAP = OUT / "snapshot"


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def record(p):
    return {"path": str(p), "bytes": p.stat().st_size, "sha256": sha(p)}


def write(name, data):
    (OUT / name).write_text(json.dumps(data, indent=2) + "\n")


def inventory():
    return [record(p) for p in sorted(SNAP.rglob("*")) if p.is_file()]


assert not (OUT / "PREEXEC.json").exists(), "No retries into this attempt"
old_manifest = json.loads((OLD / "SOURCE-MANIFEST.json").read_text())
for row in old_manifest["files"]:
    p = OLD / "snapshot" / row["path"]
    assert sha(p) == row["sha256"] and p.stat().st_size == row["bytes"]
assert not SNAP.exists()
shutil.copytree(OLD / "snapshot", SNAP)
fix = json.loads((R / "67-compiler-preflight-api-fix/candidate-01/SOURCE-FREEZE.json").read_text())
applied = []
for row in fix["candidate_fullfiles"]:
    src = Path(row["path"])
    assert src.stat().st_size == row["bytes"] and sha(src) == row["sha256"]
    dst = SNAP / row["logical_path"]
    before = sha(dst)
    shutil.copyfile(src, dst)
    applied.append({"logical_path": row["logical_path"], "before_sha256": before,
                    "after_sha256": sha(dst), "owner_source": str(src)})
test = R / "62-compiler-preflight-regression-author/test-afterimage/crates/mori-core/tests/sherlock_compile_preflight_regressions.rs"
assert sha(test) == "4848ea3764c8ac12c8013216ea9be1d24199716c084d9030134296d51c2601c7"
test_dst = SNAP / "crates/mori-core/tests" / test.name
test_dst.parent.mkdir(parents=True, exist_ok=True)
shutil.copyfile(test, test_dst)
before_sources = inventory()
write("SOURCE-MANIFEST.json", {"base_commit": old_manifest["base_commit"],
      "previous_failed_attempt": str(OLD), "owner_API_afterimages": applied,
      "files": before_sources, "tests_executed": False, "W_mutations": False})
prior = json.loads((R / "61-b-pairwise-actual/attempt-01/PREEXEC-INVENTORY.json").read_text())
pins = prior["normal_dependency_inventory"] + prior["toolchain_linker_sysroot"]
for row in pins:
    p = Path(row["path"])
    assert p.stat().st_size == row["bytes"] and sha(p) == row["sha256"], str(p)
deps = Path(prior["normal_dependency_inventory"][0]["path"]).parent
assert sorted(p.name for p in deps.iterdir() if p.is_file()) == sorted(Path(x["path"]).name for x in prior["normal_dependency_inventory"])
(OUT / "tmp").mkdir()
env = {"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8", "TMPDIR": str(OUT / "tmp")}
old_pre = json.loads((OLD / "PREEXEC.json").read_text())
core_args = [s.replace(str(OLD), str(OUT)) for s in old_pre["argv"]]
rustc = core_args[3]
serde = next(s for s in core_args if s.startswith("serde_json="))
test_args = ["/usr/bin/timeout", "--signal=KILL", "30s", rustc,
             "--edition=2024", "--test", str(test_dst),
             "--extern", "mori_core=" + str(OUT / "libmori_core.rmeta"),
             "--extern", serde, "-L", "dependency=" + str(deps),
             "--emit=metadata,dep-info=" + str(OUT / "tests.d"),
             "-o", str(OUT / "tests.rmeta"), "-C", "debuginfo=0"]
write("PREEXEC.json", {"utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
      "core_argv": core_args, "test_argv_if_core_pass": test_args,
      "environment": env, "cwd": str(OUT), "pinned_inputs": pins,
      "collector": record(Path(__file__)),
      "scope": "Production Core metadata then external five-function test metadata only; no linking, tests, compiler host evaluation, SDK, native or target execution"})
phases = []
for name, args in [("core", core_args), ("tests", test_args)]:
    started = time.monotonic()
    with (OUT / (name + ".stdout")).open("wb") as stdout, (OUT / (name + ".stderr")).open("wb") as stderr:
        done = subprocess.run(args, cwd=OUT, env=env, stdout=stdout, stderr=stderr, check=False)
    row = {"name": name, "argv": args, "exit": done.returncode,
           "seconds": time.monotonic() - started,
           "stdout": record(OUT / (name + ".stdout")), "stderr": record(OUT / (name + ".stderr"))}
    phases.append(row)
    write(name.upper() + "-RESULT.json", row)
    if done.returncode != 0:
        break
after_sources = inventory()
assert after_sources == before_sources
for row in pins:
    p = Path(row["path"])
    assert p.stat().st_size == row["bytes"] and sha(p) == row["sha256"], str(p)
assert sorted(p.name for p in deps.iterdir() if p.is_file()) == sorted(Path(x["path"]).name for x in prior["normal_dependency_inventory"])
outputs = [record(OUT / name) for name in ["core.d", "tests.d", "libmori_core.rmeta", "tests.rmeta"] if (OUT / name).exists()]
write("RESULT.json", {"phases": phases, "sources_unchanged": True,
      "dependency_and_toolchain_pins_unchanged": True, "outputs": outputs,
      "scope": "metadata/typechecking only, not compiler execution, test execution, review approval, native or target acceptance"})
print(json.dumps({"phases": [{k: row[k] for k in ("name", "exit", "seconds")} for row in phases],
                  "source_files": len(before_sources), "pinned_files": len(pins),
                  "result": str(OUT / "RESULT.json")}))
