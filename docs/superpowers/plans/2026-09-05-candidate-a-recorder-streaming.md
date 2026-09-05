# A4 recorder streamed artifact hashing and native resource receipt addendum

Status: proposed only. No source edits or new validation runs are authorized while Task2 final jobs are active. This addendum does not change old receipts or admit a native pilot/export.

## Finding and exact scope

Current scripts/record_s02_candidate_a_integrated.py computes native ITF artifact pins using sha(path.read_bytes()). This reads the complete native file and violates CT-007 even though the future structural exporter streams its files. Existing receipt elapsed_ns measures duration, but the recorder neither bounds child wall time nor records peak RSS.

The admitted scripts/a4_json_stream.py already supplies hash_stream(binary_stream,chunk_size=1048576). Its Python source and tests already belong to the exact semantic source closure; this correction adds no pin-map key. The shared runtime store also already pins /usr/bin/time as SHA25634ba90f8199989d88f2f02d40ddcee60be27a7f361db9fb7c100578cc8d7bf1e. The existing A5 runtime helper demonstrates verbose time output plus process-group timeout handling, but its semantic source collection is A5-specific and must not be reused as if it were an A4 receipt.

## RH001: Bounded artifact hashing

WHEN the recorder hashes an emitted native ITF, it SHALL open the file in binary mode and request only positive bounded chunks. The returned SHA256 and existing artifacts map shape SHALL be unchanged.

Scenario: a guarded path rejects read_bytes and a guarded reader rejects read(-1) or oversized reads. The original whole-read implementation fails the actual assertion; the streamed implementation passes and equals hashlib.sha256 on known bytes.

After separate root dispatch, create the following helper with the original behavior and replace only the artifacts comprehension's call with artifact_hash(path). This extraction is behavior-preserving, not the fix:

```python
def artifact_hash(path):
    return sha(path.read_bytes())

# Existing main:
artifacts = {path.name: artifact_hash(path) for path in sorted(stage.glob("*.itf.json"))}
```

Append this exact test to tests/test_s02_candidate_a_integrated_export.py:

```python
def test_recorder_artifact_hash_uses_bounded_reads():
    import hashlib
    import io
    from scripts.record_s02_candidate_a_integrated import artifact_hash
    payload = b"original native bytes" * 100_001
    sizes = []
    class GuardedReader(io.BytesIO):
        def read(self, size=-1):
            assert 0 < size <= 1_048_576, "whole/oversized native artifact read"
            sizes.append(size)
            return super().read(size)
    class GuardedPath:
        def read_bytes(self):
            raise AssertionError("whole native artifact read_bytes")
        def open(self, mode):
            assert mode == "rb"
            return GuardedReader(payload)
    assert artifact_hash(GuardedPath()) == hashlib.sha256(payload).hexdigest()
    assert len(sizes) >= 3 and all(size == 1_048_576 for size in sizes)
```

Record genuine RED under a fresh stage, with the complete original source-byte/runtime closure and terminal streams:

```bash
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a4-producer-receipts/recorder-stream-red/python-cache scripts/record_s02_candidate_a_integrated.py --stage recorder-stream-red -- /home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s02_candidate_a_integrated_export.py -vv -k recorder_artifact_hash
```

Expected assertion failure: whole native artifact read_bytes. No ITF is emitted by this Python unit, so the recorder's own artifact loop does not interfere with this controlled RED. Then implement only the streamed helper and the script-entry bootstrap needed for the newly explicit utility import:

```python
# Immediately after ROOT is defined, before importing scripts:
if __package__ in (None, ""):
    sys.path.insert(0, str(ROOT))
from scripts.a4_json_stream import hash_stream

def artifact_hash(path):
    with path.open("rb") as stream:
        return hash_stream(stream)[0]
```

No existing receipt field, native command, actual-command mapping, original source/runtime check or dispatch base changes. Tool/source archives and ordinary small metadata reads are not relabeled as native artifact reads. The same artifact_hash call must remain on the actual main path; a tested unused helper is insufficient.

Add this explicit structural main-path regression alongside the guarded behavioral test. It inspects the actual main function's artifact assignment and published receipt mapping, so reverting only the call site to an inline whole-file read cannot leave an unused passing helper. This is a structural regression assertion, not another original behavioral RED claim.

```python
def test_recorder_main_publishes_streamed_artifact_hashes():
    import ast
    import inspect
    import scripts.record_s02_candidate_a_integrated as recorder
    tree = ast.parse(inspect.getsource(recorder.main))
    assignments = {target.id: node.value for node in ast.walk(tree)
                   if isinstance(node, ast.Assign) for target in node.targets
                   if isinstance(target, ast.Name)}
    artifact_map = assignments["artifacts"]
    assert isinstance(artifact_map, ast.DictComp)
    assert ast.dump(artifact_map.value) == ast.dump(ast.parse("artifact_hash(path)", mode="eval").body)
    assert ast.dump(artifact_map.generators[0].iter) == ast.dump(
        ast.parse('sorted(stage.glob("*.itf.json"))', mode="eval").body)
    receipt = assignments["receipt"]
    assert isinstance(receipt, ast.Dict)
    published = {key.value: value for key, value in zip(receipt.keys, receipt.values)
                 if isinstance(key, ast.Constant)}
    assert isinstance(published["artifacts"], ast.Name) and published["artifacts"].id == "artifacts"
```

```bash
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a4-producer-receipts/recorder-stream-green/python-cache scripts/record_s02_candidate_a_integrated.py --stage recorder-stream-green -- /home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s02_candidate_a_integrated_export.py -vv
```

The expected collection is the then-current full producer Python inventory plus these two tests, not an assumed old count. Record actual names/count. No Quint rerun is needed solely to establish this Python hashing unit; root decides any final integrated regression gate after all source units are frozen. Preserve old Task2 receipts unchanged.

## RH002: Separate native-pilot wall and RSS receipt

WHEN a native largest-case pilot is run, its evidence SHALL retain an explicit wall budget and a measured maximum resident-set value, not only an estimate. A timeout SHALL remain a resource failure, never a semantic counterexample or successful complete case.

To preserve the recorder receipt schema and exact executed_command identity, place resource measurement OUTSIDE the existing recorder process. An independently pinned root-owned receipt runner can invoke the exact existing recorder argv under already-admitted /usr/bin/time. Its own resource/launch/terminal sidecars belong to receipt_pins, not semantic source_pins. Do not insert time into executed_command while still claiming that Node was the directly launched child.

Proposed exact outer pattern for later separately assigned implementation:

```python
import hashlib
import json
import os
import signal
import subprocess
import sys
import time
from pathlib import Path

TIME = Path("/usr/bin/time")
TIME_SHA = "34ba90f8199989d88f2f02d40ddcee60be27a7f361db9fb7c100578cc8d7bf1e"

def file_sha(path):
    digest = hashlib.sha256()
    with Path(path).open("rb") as stream:
        while chunk := stream.read(1_048_576):
            digest.update(chunk)
    return digest.hexdigest()

def group_exists(pgid):
    try:
        os.killpg(pgid, 0)
        return True
    except ProcessLookupError:
        return False

def signal_group(pgid, sig):
    try:
        os.killpg(pgid, sig)
    except ProcessLookupError:
        pass

def native_resources(recorder_argv, receipt_dir, inner_receipt, *, wall_seconds=900):
    """Receipt-only outer wrapper; this does not authorize or synthesize a native case."""
    if type(wall_seconds) is not int or wall_seconds <= 0:
        raise ValueError("explicit positive wall budget")
    receipt_dir = Path(receipt_dir)
    receipt_dir.mkdir(parents=False, exist_ok=False)
    if file_sha(TIME) != TIME_SHA:
        raise ValueError("unadmitted time executable")
    wrapper = Path(__file__).resolve()
    interpreter = Path(sys.executable).resolve()
    pinned = {str(path): file_sha(path) for path in (wrapper, interpreter, TIME)}
    with (receipt_dir / "runner.py").open("xb") as stream:
        stream.write(wrapper.read_bytes())
    environment = dict(os.environ)
    removed = sorted(key for key in environment if key.startswith(("PYTHON", "PYTEST", "NODE_", "LD_")))
    for key in removed:
        environment.pop(key)
    environment.update(PYTHONNOUSERSITE="1", PYTHONDONTWRITEBYTECODE="1", NODE_DISABLE_COMPILE_CACHE="1")
    argv = [str(TIME), "-v", "-o", str(receipt_dir / "resources.txt"), *recorder_argv]
    with (receipt_dir / "launch.json").open("x") as stream:
        json.dump({"argv": argv, "recorder_argv": recorder_argv, "wall_seconds": wall_seconds,
                   "time_sha256": TIME_SHA, "cwd": str(Path.cwd()),
                   "original_parent_argv": sys.orig_argv, "wrapper_argv": sys.argv,
                   "source_runtime_before": pinned, "removed_environment_keys": removed,
                   "fixed_environment": {key: environment[key] for key in
                                         ("PYTHONNOUSERSITE", "PYTHONDONTWRITEBYTECODE", "NODE_DISABLE_COMPILE_CACHE")},
                   "inner_receipt": str(Path(inner_receipt).resolve()),
                   "measurement_scope": "whole recorder/native descendant tree; GNU time maximum RSS, not simultaneous RSS sum"},
                  stream, sort_keys=True)
    started = time.monotonic()
    timed_out = False
    cleanup_complete = True
    with (receipt_dir / "stdout.bin").open("xb") as stdout, (receipt_dir / "stderr.bin").open("xb") as stderr:
        process = subprocess.Popen(argv, stdout=stdout, stderr=stderr, env=environment, start_new_session=True)
        try:
            exit_code = process.wait(timeout=wall_seconds)
        except subprocess.TimeoutExpired:
            timed_out = True
            signal_group(process.pid, signal.SIGTERM)
            # Leader exit is NOT group exit. Give the owned group its grace
            # period, then signal the group regardless of leader wait status.
            grace = time.monotonic() + 5
            while group_exists(process.pid) and time.monotonic() < grace:
                process.poll()
                time.sleep(0.05)
            signal_group(process.pid, signal.SIGKILL)
            try:
                exit_code = process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                exit_code = None
            deadline = time.monotonic() + 5
            while group_exists(process.pid) and time.monotonic() < deadline:
                time.sleep(0.05)
            cleanup_complete = exit_code is not None and not group_exists(process.pid)
    after = {path: file_sha(path) for path in pinned}
    sidecars = {name: file_sha(receipt_dir / name) if (receipt_dir / name).is_file() else None
                for name in ("runner.py", "launch.json", "stdout.bin", "stderr.bin", "resources.txt")}
    inner = Path(inner_receipt)
    result = {"exit_code": exit_code, "timed_out": timed_out,
              "elapsed_seconds": time.monotonic() - started, "wall_seconds": wall_seconds,
              "inner_receipt_required": True, "inner_receipt_present": inner.is_file(),
              "inner_receipt_sha256": file_sha(inner) if inner.is_file() else None,
              "resource_receipt_present": sidecars["resources.txt"] is not None,
              "resource_receipt_complete": not timed_out and sidecars["resources.txt"] is not None,
              "inner_receipt_complete": not timed_out and inner.is_file(),
              "process_group_cleanup_complete": cleanup_complete,
              "source_runtime_after": after, "source_runtime_stable": pinned == after,
              "sidecar_sha256": sidecars}
    with (receipt_dir / "terminal.json").open("x") as stream:
        json.dump(result, stream, sort_keys=True)
    return 124 if timed_out else (2 if pinned != after else exit_code)
```

This is a proposal for an outer receipt layer, not yet assigned code. Root must bind its exact source/argv and include all resource sidecars in the externally admitted receipt set before a pilot. Invoke the wrapper itself with the admitted Python executable, -B and a fresh explicit -X pycache_prefix; its interpreter hash is backed by the already admitted Python archive, not a duplicate runtime snapshot. The inner recorder keeps its existing fresh-cache/runtime checks. The sanitized child environment retains ordinary task variables but removes Python/Pytest/Node/dynamic-loader overrides without recording their values. Original parent argv, wrapper bytes, interpreter/time hashes before and after, and original stdout/stderr/resource hashes are receipt-only data.

The 900-second budget is explicit and may be changed only by a reviewed fresh run; timeout preserves originals. The SIGKILL group signal is unconditional after the grace period, even if GNU time has already exited. Reaping that leader alone never establishes group cleanup: the bounded group-absence check is recorded separately, and failure remains incomplete. This covers the owned process group, not descendants that deliberately escape it; fixed admitted native commands must not daemonize or create detached sessions. GNU time can fail to finish its resource report when its entire process group is killed: then the run has an explicit incomplete resource receipt and cannot pass the pilot gate. A missing inner recorder terminal receipt after forced timeout likewise remains incomplete evidence, not a fabricated terminal. Even a present inner receipt is marked incomplete on timeout. Presence and digest fields are not independent validation of inner receipt semantics; root must still validate that receipt. terminal.json has no self-hash; its own digest belongs to the enclosing root-admitted receipt map.

The measurement includes recorder setup/teardown and native descendants, not just the evaluator. GNU time reports maximum RSS according to its installed semantics; it does not establish the sum of simultaneous Node/Rust residency. Retain the original resources.txt and label the metric exactly. A successful largest-case pilot must additionally have the original stable inner recorder receipt, complete witness/invariants and raw output; this outer status alone cannot accept it.

Native Node currently uses its pinned default heap; this proposal does not silently introduce a different native heap argument into the adopted command. The separately adopted Task3 parser has explicit4096MiB/900s bounds. If root requires a fixed native Node heap in addition to measurement, that needs an explicit executed-command/receipt-validator amendment before the pilot; it is not included in this hashing-only source correction.

## Required handoff

Root first admits Task2, then assigns the narrow hashing RED/GREEN unit and separately resolves/owns the resource-wrapper implementation. Retain source closure, original terminal bytes, exact changed lines and full current Python regression results. No old receipt rewrite, no extra model/semantic source edit, no native export, no provider/Foreman work or acceptance claim is authorized by this draft.
