"""One explicit fresh AGY task. No scheduling, retry, replay or capture reread."""
import argparse
import copy
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import selectors
import signal
import subprocess
import time

from agy_terminal_selector import select_terminal_review

SELECTOR_SHA256 = "ee3536dab1c2e33ec69ce734756973be0019dfa5a8622b27b58fda8371e9051a"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def pins_match(pins):
    try:
        return all(digest(Path(pin["path"]).read_bytes()) == pin["sha256"] for pin in pins)
    except OSError:
        return False


def load_task(task_path, task_sha256):
    """Read only explicit frozen public input paths, never an old capture."""
    pins = []

    def pinned(path, expected):
        path = Path(path)
        if not path.is_absolute():
            raise ValueError("input path must be absolute")
        data = path.read_bytes()
        if digest(data) != expected:
            raise ValueError("input pin mismatch")
        pins.append({"path": str(path), "sha256": expected})
        return data

    task = json.loads(pinned(task_path, task_sha256))
    if not isinstance(task, dict) or not isinstance(task.get("id"), str):
        raise ValueError("task object/id required")
    if task.get("model") != "gemini-4-argon-high":
        raise ValueError("this adapter requires the explicit AGY model")
    if type(task.get("wallSeconds")) is not int or task["wallSeconds"] <= 0:
        raise ValueError("explicit positive outer bound required")
    if not isinstance(task.get("reviewToken"), str) or not task["reviewToken"]:
        raise ValueError("explicit review token required")
    packet = Path(task["packet"])
    if not packet.is_absolute() or not packet.is_dir():
        raise ValueError("absolute packet directory required")
    manifest = json.loads(pinned(packet / "MANIFEST.json", task["manifestSha256"]))
    if not isinstance(manifest, dict) or not isinstance(manifest.get("files"), list):
        raise ValueError("public file manifest required")
    for item in manifest["files"]:
        relative = Path(item["path"])
        if relative.is_absolute() or ".." in relative.parts:
            raise ValueError("public pin must be packet-relative")
        pinned(packet / relative, item["sha256"])
    prompt = pinned(task["promptFile"], task["promptSha256"]).decode("utf-8")
    if len(prompt.encode("utf-8")) >= 125000:
        raise ValueError("prompt reaches existing 125000-byte boundary")
    schema = json.loads(pinned(task["schemaFile"], task["schemaSha256"]))
    if not isinstance(schema, dict):
        raise ValueError("explicit object schema required")
    pinned(Path(__file__).with_name("agy_terminal_selector.py").absolute(), SELECTOR_SHA256)
    own_path = Path(__file__).absolute()
    pins.append({"path": str(own_path), "sha256": digest(own_path.read_bytes())})
    return {"task": task, "taskSha256": task_sha256, "prompt": prompt,
            "schema": schema, "pins": pins}


def build_argv(bundle, output_dir):
    task = bundle["task"]
    return ["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", str(task["wallSeconds"]) + "s",
            "/home/charl/.local/bin/agy", "--model", task["model"], "--effort", "high",
            "--mode", "plan", "--sandbox", "--disable-slash-commands",
            "--add-dir", task["packet"], "--output-format", "stream-json",
            "--json-schema", task["schemaFile"], "--print-timeout", "0",
            "--log-file", str(output_dir / "agy.private.log"), "--print", bundle["prompt"]]


class LiveCapture:
    """Parse only bytes supplied by this new process; keep every parsed value."""
    def __init__(self, *, clock=time.monotonic):
        self._clock = clock
        self._origin = clock()
        self._event_counts = {"init": 0, "step_update": 0, "result": 0,
                              "otherObject": 0, "nonObject": 0}
        self._result_observations = []
        self._eof_observed = None
        self._process_exit = None
        self.events = []
        self.parse_errors = 0
        self.buffer = b""
        self.byte_count = 0
        self.complete = False

    def _elapsed(self):
        return round(max(0.0, self._clock() - self._origin), 6)

    @staticmethod
    def _json_type(value):
        if value is None:
            return "null"
        if isinstance(value, dict):
            return "object"
        if isinstance(value, list):
            return "array"
        if isinstance(value, str):
            return "string"
        if isinstance(value, bool):
            return "boolean"
        return "number"

    def _observe_value(self, value):
        # Fixed keys and finite enums only. No arbitrary event/status/text export.
        if not isinstance(value, dict):
            self._event_counts["nonObject"] += 1
            return
        event = value.get("event")
        category = event if event in ("init", "step_update", "result") else "otherObject"
        self._event_counts[category] += 1
        if category != "result" or len(self._result_observations) >= 2:
            return
        result = value.get("result")
        observation = {"parsedValueIndex": len(self.events) - 1,
            "observedAfterSeconds": self._elapsed(),
            "resultType": self._json_type(result) if "result" in value else "missing",
            "statusClass": "RESULT_NOT_OBJECT", "statusLiteral": None,
            "structuredOutputType": "missing"}
        if isinstance(result, dict):
            status = result.get("status")
            if "status" not in result:
                observation["statusClass"] = "MISSING"
            elif not isinstance(status, str):
                observation["statusClass"] = "NON_STRING"
            elif status == "SUCCESS":
                observation["statusClass"] = "ALLOWLISTED"
                observation["statusLiteral"] = "SUCCESS"
            else:
                observation["statusClass"] = "OTHER_STRING"
            if "structured_output" in result:
                observation["structuredOutputType"] = self._json_type(result["structured_output"])
        self._result_observations.append(observation)

    def observe_process_exit(self, exit_code):
        # Observation after wait/poll, never a claimed kernel/provider exit time.
        if self._process_exit is None:
            self._process_exit = {"code": exit_code, "observedAfterSeconds": self._elapsed()}

    def live_metadata(self):
        return copy.deepcopy({"policy": "future-live-allowlisted-transport-metadata-v1",
            "timingBasis": "local-monotonic-seconds-since-capture-construction",
            "eventCounts": self._event_counts,
            "resultObservations": self._result_observations,
            "resultObservationsOmitted": self._event_counts["result"] - len(self._result_observations),
            "captureParseErrors": self.parse_errors,
            "parsedTopLevelValues": len(self.events), "stdoutBytes": self.byte_count,
            "captureReachedEOF": self.complete, "eofObservedAfterSeconds": self._eof_observed,
            "processExit": self._process_exit, "actualBackendIdentityAttested": False,
            "reviewPayloadExportedHere": False, "noPrivateCaptureReread": True})

    def _line(self, line):
        if not line.strip():
            return
        try:
            self.events.append(json.loads(line.decode("utf-8")))
            self._observe_value(self.events[-1])
        except (ValueError, UnicodeError, RecursionError):
            self.parse_errors += 1

    def feed(self, chunk):
        self.complete = False
        self.byte_count += len(chunk)
        self.buffer += chunk
        while b"\n" in self.buffer:
            line, self.buffer = self.buffer.split(b"\n", 1)
            self._line(line)

    def finish(self):
        if self.buffer:
            self._line(self.buffer)
            self.buffer = b""
        self.complete = True
        if self._eof_observed is None:
            self._eof_observed = self._elapsed()


def finish_capture(capture, bundle, exit_code, unchanged):
    outcome = select_terminal_review(capture.events, schema=bundle["schema"],
        review_token=bundle["task"]["reviewToken"], terminal_exit_code=exit_code,
        input_pins_unchanged=unchanged, capture_parse_errors=capture.parse_errors)
    if not capture.complete:
        outcome = {"decision": "NO_EXPORT", "reason": "CAPTURE_NOT_COMPLETE", "review": None,
                   "receipt": dict(outcome["receipt"], decision="NO_EXPORT", reason="CAPTURE_NOT_COMPLETE")}
    return outcome


def _new_file(path, mode="wb"):
    return os.fdopen(os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600), mode)


def _write_json(path, value):
    with _new_file(path) as handle:
        handle.write((json.dumps(value, indent=2) + "\n").encode())


def _group_present(pgid):
    try:
        os.killpg(pgid, 0)
        return True
    except ProcessLookupError:
        return False


def _kill_owned_group(pgid):
    try:
        os.killpg(pgid, signal.SIGKILL)
    except ProcessLookupError:
        pass


def _run_new_process(bundle, output_dir, argv):
    capture = LiveCapture()
    deadline = time.monotonic() + bundle["task"]["wallSeconds"] + 5
    with _new_file(output_dir / "provider.private.jsonl") as private_stdout, \
            _new_file(output_dir / "stderr.private") as private_stderr:
        process = subprocess.Popen(argv, cwd=bundle["task"]["packet"], stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE, stderr=private_stderr, env=dict(os.environ), start_new_session=True)
        _write_json(output_dir / "CHILD.json", {"pid": process.pid, "ownedProcessGroup": process.pid})
        try:
            with selectors.DefaultSelector() as readiness:
                readiness.register(process.stdout, selectors.EVENT_READ)
                while time.monotonic() < deadline:
                    ready = readiness.select(min(0.2, max(0, deadline - time.monotonic())))
                    if not ready:
                        continue
                    chunk = os.read(process.stdout.fileno(), 65536)
                    if not chunk:
                        capture.finish()
                        break
                    private_stdout.write(chunk)
                    private_stdout.flush()
                    capture.feed(chunk)
            try:
                process.wait(timeout=max(0.01, deadline - time.monotonic()))
            except subprocess.TimeoutExpired:
                _kill_owned_group(process.pid)
                process.wait(timeout=1)
        finally:
            process.stdout.close()
            if process.poll() is None or _group_present(process.pid):
                _kill_owned_group(process.pid)
            if process.poll() is None:
                process.wait(timeout=1)
        capture.observe_process_exit(process.returncode)
        cleanup = {"ownedProcessGroup": process.pid, "ownedGroupRemaining": _group_present(process.pid),
                   "captureReachedEOF": capture.complete}
        return capture, process.returncode, cleanup


def run_one(task_path, task_sha256, output_dir):
    """One launch only. Existing output directory is an error, never a replay."""
    bundle = load_task(task_path, task_sha256)
    output_dir = Path(output_dir)
    if not output_dir.is_absolute():
        raise ValueError("new absolute output directory required")
    output_dir.mkdir(mode=0o700, parents=False, exist_ok=False)
    argv = build_argv(bundle, output_dir)
    started = datetime.now(timezone.utc).isoformat()
    _write_json(output_dir / "START.json", {"id": bundle["task"]["id"], "startedAt": started,
        "taskSha256": task_sha256, "inputPins": bundle["pins"], "requestedModel": bundle["task"]["model"],
        "argvWithoutPrompt": argv[:-1], "promptSha256": bundle["task"]["promptSha256"],
        "actualBackendIdentityAttested": False, "selectorSha256": SELECTOR_SHA256})
    capture, exit_code, cleanup = _run_new_process(bundle, output_dir, argv)
    _write_json(output_dir / "LIVE-METADATA.json", capture.live_metadata())
    unchanged = pins_match(bundle["pins"])
    outcome = finish_capture(capture, bundle, exit_code, unchanged)
    receipt = dict(outcome["receipt"], id=bundle["task"]["id"], taskSha256=task_sha256,
        startedAt=started, endedAt=datetime.now(timezone.utc).isoformat(), stdoutBytes=capture.byte_count,
        parsedTopLevelValues=len(capture.events), selectorSha256=SELECTOR_SHA256,
        requestedModel=bundle["task"]["model"], noPrivateCaptureReread=True,
        sourceReviewCorrectnessEstablished=False)
    if outcome["decision"] == "EXPORT":
        _write_json(output_dir / "TERMINAL-REVIEW.json", outcome["review"])
    _write_json(output_dir / "CLEANUP.json", cleanup)
    _write_json(output_dir / "TERMINAL-EXPORT-RECEIPT.json", receipt)
    return receipt


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--task", required=True)
    parser.add_argument("--task-sha256", required=True)
    parser.add_argument("--output-dir", required=True)
    args = parser.parse_args()
    receipt = run_one(args.task, args.task_sha256, args.output_dir)
    print(json.dumps(receipt), flush=True)
    return 0 if receipt["decision"] == "EXPORT" else 2


if __name__ == "__main__":
    raise SystemExit(main())
