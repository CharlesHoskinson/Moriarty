"""Receipt-only GNU time capture. Eligibility is not native acceptance."""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import time

TIME = Path("/usr/bin/time")
TIME_SHA = "34ba90f8199989d88f2f02d40ddcee60be27a7f361db9fb7c100578cc8d7bf1e"
RESOURCE_LIMIT = 65536
FIELDS = (
    "Command being timed", "User time (seconds)", "System time (seconds)",
    "Percent of CPU this job got", "Elapsed (wall clock) time (h:mm:ss or m:ss)",
    "Average shared text size (kbytes)", "Average unshared data size (kbytes)",
    "Average stack size (kbytes)", "Average total size (kbytes)",
    "Maximum resident set size (kbytes)", "Average resident set size (kbytes)",
    "Major (requiring I/O) page faults", "Minor (reclaiming a frame) page faults",
    "Voluntary context switches", "Involuntary context switches", "Swaps",
    "File system inputs", "File system outputs", "Socket messages sent",
    "Socket messages received", "Signals delivered", "Page size (bytes)", "Exit status",
)


class ResourceError(ValueError):
    pass


def sha(path):
    digest = hashlib.sha256()
    with Path(path).open("rb") as stream:
        while data := stream.read(1048576):
            digest.update(data)
    return digest.hexdigest()


def parse_resource(path, argv):
    with Path(path).open("rb") as stream:
        data = stream.read(RESOURCE_LIMIT + 1)
    if len(data) > RESOURCE_LIMIT:
        raise ResourceError("resource report exceeds 64 KiB")
    try:
        text = data.decode("utf-8")
    except UnicodeError as error:
        raise ResourceError("resource report encoding") from error
    values, diagnostics = {}, []
    for line in text.splitlines():
        if not values and re.fullmatch(r"Command (?:exited with non-zero status [0-9]+|terminated by signal [0-9]+)", line):
            diagnostics.append(line)
            continue
        matched = [key for key in FIELDS if line.startswith("\t" + key + ": ")]
        if len(matched) != 1 or matched[0] in values:
            raise ResourceError("unknown, duplicate or malformed resource field")
        key = matched[0]
        values[key] = line[len(key) + 3:]
    if set(values) != set(FIELDS):
        raise ResourceError("incomplete resource field inventory")
    if values[FIELDS[0]] != '"' + " ".join(argv) + '"':
        raise ResourceError("resource command binding")
    for key in FIELDS[1:3]:
        if not re.fullmatch(r"[0-9]+(?:\.[0-9]+)?", values[key]):
            raise ResourceError("invalid resource seconds")
    if not re.fullmatch(r"[0-9]+(?:\.[0-9]+)?%", values[FIELDS[3]]):
        raise ResourceError("invalid resource CPU percentage")
    wall = re.fullmatch(r"(?:(\d+):)?(\d+):(\d+(?:\.\d+)?)", values[FIELDS[4]])
    if wall is None or float(wall[3]) >= 60 or (wall[1] is not None and int(wall[2]) >= 60):
        raise ResourceError("invalid resource elapsed time")
    for key in FIELDS[5:]:
        if not re.fullmatch(r"[0-9]+", values[key]):
            raise ResourceError("invalid resource counter")
    rss = int(values["Maximum resident set size (kbytes)"])
    if rss <= 0:
        raise ResourceError("nonpositive maximum RSS")
    return {"fields": values, "diagnostics": diagnostics,
            "maximum_rss_kib": rss, "exit_status": int(values["Exit status"])}


def resource_complete(path, argv):
    try:
        parse_resource(path, argv)
        return True
    except (OSError, ValueError):
        return False


def safe_absolute(path):
    path = Path(path).absolute()
    if any(part.is_symlink() for part in (path, *path.parents)):
        raise ValueError("symlink destination/ancestor")
    return path.resolve()


def write_json(path, value):
    with path.open("x", encoding="utf-8") as stream:
        json.dump(value, stream, sort_keys=True, indent=2)
        stream.write("\n")


def controlled_environment():
    environment = dict(os.environ)
    removed = sorted(k for k in environment if k.startswith(("PYTHON", "PYTEST", "NODE_", "LD_")))
    for key in removed:
        environment.pop(key)
    fixed = {"PYTHONNOUSERSITE": "1", "PYTHONDONTWRITEBYTECODE": "1",
             "NODE_DISABLE_COMPILE_CACHE": "1", "LC_ALL": "C"}
    environment.update(fixed)
    return environment, removed, fixed


def group_exists(pgid):
    try:
        os.killpg(pgid, 0)
        return True
    except ProcessLookupError:
        return False


def cleanup_group(process, grace_seconds, cleanup_seconds):
    result = {"forced": True, "signals": [], "errors": [], "complete": False}
    for sig, delay in ((signal.SIGTERM, grace_seconds), (signal.SIGKILL, cleanup_seconds)):
        try:
            os.killpg(process.pid, sig)
            result["signals"].append({"signal": sig.name, "sent": True})
        except ProcessLookupError:
            result["signals"].append({"signal": sig.name, "sent": False})
        except BaseException as error:
            result["errors"].append({"type": type(error).__name__, "message": str(error)})
        # Always progress to the group SIGKILL step, regardless of leader exit.
        deadline = time.monotonic() + delay
        while time.monotonic() < deadline:
            try:
                process.poll()
                absent = not group_exists(process.pid)
                if absent:
                    break
                time.sleep(min(0.02, max(0, deadline - time.monotonic())))
            except BaseException as error:
                result["errors"].append({"type": type(error).__name__, "message": str(error)})
                break
    try:
        process.wait(timeout=cleanup_seconds)
    except BaseException as error:
        result["errors"].append({"type": type(error).__name__, "message": str(error)})
    try:
        result["complete"] = process.returncode is not None and not group_exists(process.pid) and not result["errors"]
    except BaseException as error:
        result["errors"].append({"type": type(error).__name__, "message": str(error)})
    return result


def pins(paths):
    result = {}
    for path in paths:
        try:
            result[str(path)] = sha(path)
        except OSError:
            result[str(path)] = None
    return result


def capture(argv, receipt_dir, inner_receipt, *, wall_seconds=900, cwd=None,
            grace_seconds=5, cleanup_seconds=5):
    if type(wall_seconds) is not int or wall_seconds <= 0:
        raise ValueError("positive integer wall budget required")
    if type(argv) is not list or not argv or any(type(a) is not str or not a for a in argv):
        raise ValueError("nonempty string argv required")
    for interval in (grace_seconds, cleanup_seconds):
        if type(interval) not in (int, float) or not math.isfinite(interval) or interval <= 0 or interval > 5:
            raise ValueError("cleanup interval must be positive and at most 5 seconds")
    receipt_dir, inner = safe_absolute(receipt_dir), safe_absolute(inner_receipt)
    if receipt_dir.exists() or inner.exists():
        raise ValueError("receipt destination or inner receipt already exists")
    if inner == receipt_dir or receipt_dir in inner.parents:
        raise ValueError("inner receipt must be outside dedicated outer directory")
    if sha(TIME) != TIME_SHA:
        raise ValueError("unadmitted GNU time executable")
    sources = [Path(__file__).resolve(), Path(sys.executable).resolve(), TIME.resolve()]
    before = pins(sources)
    receipt_dir.mkdir(parents=False, exist_ok=False)
    with (receipt_dir / "runner.py").open("xb") as stream:
        stream.write(sources[0].read_bytes())
    environment, removed, fixed = controlled_environment()
    command = [str(TIME), "-v", "-o", str(receipt_dir / "resources.txt"), *argv]
    working = Path.cwd() if cwd is None else Path(cwd)
    write_json(receipt_dir / "launch.json", {
        "original_parent_argv": sys.orig_argv, "wrapper_argv": sys.argv,
        "argv": command, "child_argv": argv, "cwd": str(working.absolute()),
        "inner_receipt": str(inner), "wall_seconds": wall_seconds,
        "grace_seconds": grace_seconds, "cleanup_seconds": cleanup_seconds,
        "source_runtime_before": before, "removed_environment_keys": removed,
        "fixed_environment": fixed,
        "measurement_scope": "GNU time maximum RSS for recorder/native-descendant invocation, KiB; not simultaneous RSS sum or evaluator-only"})
    process = None
    timed_out = False
    launch_error = monitor_error = None
    cleanup = {"forced": False, "signals": [], "errors": [], "complete": False}
    started = time.monotonic()
    with (receipt_dir / "stdout.bin").open("xb") as stdout, (receipt_dir / "stderr.bin").open("xb") as stderr:
        try:
            process = subprocess.Popen(command, cwd=working, stdout=stdout, stderr=stderr,
                                       env=environment, start_new_session=True)
        except Exception as error:
            launch_error = {"type": type(error).__name__, "message": str(error)}
        if process is not None:
            try:
                process.wait(timeout=wall_seconds)
            except subprocess.TimeoutExpired:
                timed_out = True
            except BaseException as error:
                monitor_error = {"type": type(error).__name__, "message": str(error)}
            finally:
                try:
                    observation_error = None
                    try:
                        remaining = group_exists(process.pid)
                    except BaseException as error:
                        remaining = True
                        observation_error = {"type": type(error).__name__, "message": str(error)}
                    if timed_out or process.returncode is None or remaining:
                        cleanup = cleanup_group(process, grace_seconds, cleanup_seconds)
                        if observation_error is not None:
                            cleanup["errors"].append(observation_error)
                            cleanup["complete"] = False
                    else:
                        cleanup["complete"] = True
                except BaseException as error:
                    cleanup["errors"].append({"type": type(error).__name__, "message": str(error)})
                    cleanup["complete"] = False
    after = pins(sources)
    sidecars = pins([receipt_dir / name for name in ("runner.py", "launch.json", "stdout.bin", "stderr.bin", "resources.txt")])
    stable = before == after and sidecars[str(receipt_dir / "runner.py")] == before[str(sources[0])]
    resource = None
    resource_error = None
    try:
        resource = parse_resource(receipt_dir / "resources.txt", argv)
    except (OSError, ValueError) as error:
        resource_error = {"type": type(error).__name__, "message": str(error)}
    inner_present = inner.is_file() and not inner.is_symlink()
    inner_hash = sha(inner) if inner_present else None
    actual_exit = process.returncode if process is not None else None
    complete = resource is not None and not timed_out
    ready = complete and stable and cleanup["complete"] and not cleanup["forced"] and inner_present and launch_error is None and monitor_error is None
    eligible = ready and actual_exit == 0 and resource["exit_status"] == 0 and not resource["diagnostics"]
    normalized = (128 - actual_exit if actual_exit < 0 else actual_exit) if actual_exit is not None else 2
    return_code = 124 if timed_out else 2 if not ready else normalized if normalized != 0 else 0 if eligible else 2
    terminal = {
        "actual_exit": actual_exit, "return_code": return_code, "timed_out": timed_out,
        "launch_error": launch_error, "monitor_error": monitor_error,
        "elapsed_seconds": time.monotonic() - started, "wall_seconds": wall_seconds,
        "owned_pgid": process.pid if process is not None else None, "cleanup": cleanup,
        "source_runtime_before": before, "source_runtime_after": after, "source_runtime_stable": stable,
        "sidecar_sha256": sidecars, "resource": resource, "resource_error": resource_error,
        "resource_complete": complete, "inner_receipt_present": inner_present,
        "inner_receipt_sha256": inner_hash, "inner_receipt_independent_validation_required": True,
        "inner_receipt_complete": False, "eligible": eligible,
        "scope": "receipt eligibility only; not native acceptance"}
    write_json(receipt_dir / "terminal.json", terminal)
    return return_code


def main(argv=None):
    parser = argparse.ArgumentParser()
    parser.add_argument("--receipt-dir", required=True)
    parser.add_argument("--inner-receipt", required=True)
    parser.add_argument("--wall-seconds", type=int, default=900)
    parser.add_argument("--cwd")
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args(argv)
    command = args.command[1:] if args.command[:1] == ["--"] else args.command
    try:
        return capture(command, args.receipt_dir, args.inner_receipt, wall_seconds=args.wall_seconds, cwd=args.cwd)
    except (OSError, ValueError) as error:
        print(json.dumps({"error": type(error).__name__, "message": str(error)}), file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
