"""Bounded Task4 wrapper, adapted from retained liability-native-driver.py.

--dry-run validates frozen files and prints commands; it spawns no native child
and never opens the SRS. Controller must check both fresh exact resource votes
before setting the explicit admission environment variable. No retry or k search.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import signal
import subprocess
import time

CASES = ["positive", "raw-honest", "response-plus-one", "wrong-private-key",
         "tight-gross", "tight-fee", "tight-net", "tight-duty", "alternative",
         "reused-proof"]
MAX_CALLS = [12, 20, 8, 4, 1, 7, 17]
AS_BYTES = 2147483648
TEST = "proving::task4_native_driver::task4_native_resource_gated_entry"
ADMISSION = "root-approved-exact-resource-votes"


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def write_new(path, data):
    with Path(path).open("x") as handle:
        json.dump(data, handle, indent=2)
        handle.write("\n")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("config", type=Path)
    parser.add_argument("--dry-run", action="store_true")
    options = parser.parse_args()
    config = json.loads(options.config.read_text())
    assert config["profile"] == "moriarty-task4-exact-native-config/1"
    assert config["cases"] == CASES
    assert config["max_calls"] == MAX_CALLS
    assert config["address_space_bytes"] == AS_BYTES
    assert config["deadline_seconds"] == 60
    assert config["kill_grace_seconds"] == 2
    assert config["aggregate_deadline_seconds"] == 600
    assert config["aggregate_kill_grace_seconds"] == 20
    assert config["processes"] == 10 and config["k"] == 13
    assert sha(config["binary"]) == config["binary_sha256"]
    assert sha(__file__) == config["wrapper_sha256"]
    root = Path(config["root"])
    for spec in config["pins"]:
        assert sha(spec["path"]) == spec["sha256"], spec["path"]
    for object_name, expected in [("HEAD", config["candidate"]),
                                  ("HEAD^{tree}", config["tree"])]:
        actual = subprocess.check_output(["git", "rev-parse", object_name],
                                         cwd=root, text=True).strip()
        assert actual == expected, (object_name, actual, expected)
    assert not subprocess.check_output(["git", "status", "--porcelain"], cwd=root)
    execution = Path(config["execution_root"])
    wrapper_root = Path(config["wrapper_output"])
    assert not execution.exists(), "consumed experiment: preserve all captures"
    assert not wrapper_root.exists(), "consumed wrapper allocation"
    argv = [config["binary"], "--exact", TEST, "--ignored", "--nocapture",
            "--test-threads=1"]
    if options.dry_run:
        print(json.dumps({"qualification": "dry-run-only-no-SRS-no-native-child",
                          "binary_sha256": config["binary_sha256"],
                          "cases": CASES, "argv": argv,
                          "max_calls": MAX_CALLS, "address_space_bytes": AS_BYTES,
                          "seconds_per_process": 60, "kill_grace_seconds": 2,
                          "native_calls": 0}, indent=2))
        return
    assert os.environ.get("MORIARTY_TASK4_NATIVE_ADMISSION") == ADMISSION, \
        "controller must validate both exact current resource votes"
    wrapper_root.mkdir(exist_ok=False)
    start = time.monotonic()
    total = [0] * 7
    processes = 0

    def set_limits():
        resource.setrlimit(resource.RLIMIT_AS, (AS_BYTES, AS_BYTES))
        resource.setrlimit(resource.RLIMIT_CORE, (0, 0))

    for case in CASES:
        # Charge process budget before invocation. Stop globally on any failure.
        processes += 1
        assert processes <= 10
        assert time.monotonic() - start < 620
        env = dict(os.environ)
        env["MORIARTY_TASK4_CASE"] = case
        env["MORIARTY_TASK4_BINARY_SHA"] = config["binary_sha256"]
        env["RAYON_NUM_THREADS"] = "1"
        env["RUST_BACKTRACE"] = "1"
        stdout_path = wrapper_root / (case + ".stdout.txt")
        stderr_path = wrapper_root / (case + ".stderr.txt")
        before = time.monotonic()
        # Each child is a new process group; stdout/stderr are retained even
        # when timeout/OOM/panic prevents a Rust result.json from being written.
        with stdout_path.open("xb") as stdout, stderr_path.open("xb") as stderr:
            child = subprocess.Popen(argv, cwd=root, env=env, stdout=stdout,
                                     stderr=stderr, start_new_session=True,
                                     preexec_fn=set_limits)
            timed_out = False
            try:
                child.wait(timeout=60)
            except subprocess.TimeoutExpired:
                timed_out = True
                os.killpg(child.pid, signal.SIGTERM)
                try:
                    child.wait(timeout=2)
                except subprocess.TimeoutExpired:
                    os.killpg(child.pid, signal.SIGKILL)
                    child.wait()
        result_path = execution / case / "result.json"
        result = json.loads(result_path.read_text()) if result_path.exists() else None
        events_path = execution / case / "calls.jsonl"
        calls = [0] * 7
        if events_path.exists():
            for line in events_path.read_text().splitlines():
                event = json.loads(line)
                calls = event["counts"]
        total = [a + b for a, b in zip(total, calls)]
        assert all(a <= b for a, b in zip(total, MAX_CALLS))
        passed = not timed_out and child.returncode == 0 and result is not None \
            and result["passed"] is True and result["binary_sha256"] == config["binary_sha256"]
        capture = {"case": case, "argv": argv, "process_number": processes,
                   "pid_and_group": child.pid, "exit": child.returncode,
                   "timed_out": timed_out, "seconds": time.monotonic() - before,
                   "counts": calls, "aggregate_counts": total, "passed": passed,
                   "result": result, "stdout_sha256": sha(stdout_path),
                   "stderr_sha256": sha(stderr_path)}
        write_new(wrapper_root / (case + ".capture.json"), capture)
        print(json.dumps(capture), flush=True)
        if not passed:
            write_new(wrapper_root / "summary.json",
                      {"passed": False, "stopped_at": case, "processes": processes,
                       "counts": total, "qualification": "FAILED-unqualified-no-retry",
                       "seconds": time.monotonic() - start})
            raise SystemExit(1)
    assert total == MAX_CALLS and processes == 10
    write_new(wrapper_root / "summary.json",
              {"passed": True, "processes": processes, "counts": total,
               "seconds": time.monotonic() - start,
               "qualification": "scoped-native-results-await-fresh-independent-review"})


if __name__ == "__main__":
    main()
