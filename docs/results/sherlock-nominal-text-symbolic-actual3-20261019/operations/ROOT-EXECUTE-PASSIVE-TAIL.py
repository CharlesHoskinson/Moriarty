"""Root-only, one-use ordinary capture composition; prepared but NEVER run here.
ONLY operations 2/3 are available; actual Root Op01 PASS23 is reused, never replayed.
No source changes, producer or Core replay.
Uses the existing reviewed capture/check/static_check/save functions only.
"""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import runpy
import sys

HERE = Path(__file__).resolve().parent
Q = HERE.parent
QA = Q / "ordinary-QA"
ACT = QA / "actual-three-01"
SEMANTIC = ("final_value", "calls", "matches", "constructors", "pending_checkpoints")


def load(path):
    return json.loads(Path(path).read_text())


def fsync_file(path):
    with Path(path).open("rb") as stream:
        os.fsync(stream.fileno())


def fsync_directory(path):
    fd = os.open(path, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def exclusive(path, raw):
    with Path(path).open("xb") as stream:
        stream.write(raw)
        stream.flush()
        os.fsync(stream.fileno())
    fsync_directory(Path(path).parent)


def durable_save(h, path, value):
    h["save"](path, value)
    fsync_file(path)
    fsync_directory(Path(path).parent)


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def unique(rows):
    found = {}
    for row in rows:
        p = row["path"]
        assert p not in found or found[p] == row, "conflicting pins: " + p
        found[p] = row
    return list(found.values())


def check_rows(h, rows):
    result = h["static_check"](unique(rows))
    assert result["all_match"], "full byte/hash input check failed"
    return result


def prior(h, number, controls_pin):
    if number == 1:
        controls = load(controls_pin["path"])
        for row in controls["actual_op01_receipts"] + [controls["actual_op01_composition"]]:
            h["check"](row)
        root = ACT / "01-synthetic-only"
        verdict = load(root / "ROOT-OUTCOME.json")
        raw = load(root / "RESULT.json")
        assert verdict["status"] == "PASS_EXACT23_SYNTHETIC_TESTS_ONLY" and verdict["pins"] == 707
        assert verdict["noRealProjectionOrExpectedRead"] is True
        source = load(controls["source_freeze"]["path"])["files"]
        assert verdict["source"] == source[0] and verdict["tests"] == source[1]
        assert raw["actual_exit"] == 0 and raw["controller_failure"] is None
        assert len(raw["streams"]) == 2 and all(x["full_stream_retained"] for x in raw["streams"])
        for row in raw["streams"]:
            h["check"]({k: row[k] for k in ("path", "bytes", "sha256")})
        for name in ("PRE-PIN-CHECK.json", "POST-PIN-CHECK.json"):
            check = load(root / name)
            assert check["all_match"] is True and len(check["files"]) == 707
            assert all(x["match"] is True for x in check["files"])
        return {"accepted": True, "operation_number": 1, "outputs": [],
                "actual_Root_receipt": h["pin"](root / "ROOT-OUTCOME.json")}
    path = ACT / "02-actual-only-project/ACCEPTANCE.json"
    result = load(path)
    assert result["accepted"] is True and result["operation_number"] == number == 2
    assert result["execution_controls"] == controls_pin
    for row in result["outputs"]:
        h["check"](row)
    return result


def metadata_stdout(h, phase):
    assert (phase / "stderr.bin").read_bytes() == b"", "unexpected passive-consumer stderr"
    raw = (phase / "stdout.bin").read_bytes()
    assert raw.endswith(b"\n") and raw.count(b"\n") == 1, "expected one complete JSON metadata line"
    return json.loads(raw)


def run_one(number):
    assert number in (2, 3), "Op01 is complete and cannot be selected here"
    # This direct Root invocation selects only this operation. Existing accepted
    # earlier phases are read back, never rerun. No helper.main/FROZEN dispatch.
    controls_pin = load(HERE / "COMPOSITION-PINS-TAIL.json")["controls"]
    controls = load(controls_pin["path"])
    helper_pin = controls["helper"]
    raw = Path(helper_pin["path"]).read_bytes()
    assert len(raw) == helper_pin["bytes"] and hashlib.sha256(raw).hexdigest() == helper_pin["sha256"]
    h = runpy.run_path(helper_pin["path"], run_name="ordinary_capture_primitives_only")
    composition = load(HERE / "COMPOSITION-PINS-TAIL.json")
    h["check"](composition["script"])
    h["check"](composition["controls"])
    for key in ("plan", "static", "source_freeze", "review", "helper", "case_bindings_proposal",
                "producer_result", "producer_freeze", "producer_readback", "safe_bindings", "source_manifest"):
        h["check"](controls[key])
    for row in controls["original_sources"]:
        h["check"](row)
    plan = load(controls["plan"]["path"])
    flat = load(controls["static"]["path"])["files"]
    assert len(flat) == 698
    op = plan["operations"][number - 1]
    assert op["wallSeconds"] == 30 and op["cpuSeconds"] == 25 and op["keptBytesPerStream"] == 131072
    assert op["environment"]["PYTHONDONTWRITEBYTECODE"] == "1" and "-B" in op["argv"]
    assert op["cwd"] == str(Q / "source")
    assert op["environment"]["TMPDIR"] == str(QA / "tmp")
    assert QA.is_dir() and not QA.is_symlink() and (QA / "tmp").is_dir() and not (QA / "tmp").is_symlink()
    assert Path("/usr/bin/python3").resolve() == Path("/usr/bin/python3.14")
    timeout_target = Path("/usr/bin/timeout").resolve()
    assert str(timeout_target) in {r["path"] for r in flat}, "timeout symlink target not pinned"
    assert not (ACT / "FAILURE.json").exists(), "earlier failure is terminal; no retry"
    assert ACT.is_dir()
    for n in range(1, number):
        prior(h, n, controls_pin)
    phase = ACT / op["id"]
    phase.mkdir(mode=0o700, exist_ok=False)
    generated = []
    controls_rows = [composition["script"], controls_pin]
    controls_rows += [controls[k] for k in ("plan", "static", "source_freeze", "review", "helper", "case_bindings_proposal",
                     "producer_result", "producer_freeze", "producer_readback", "safe_bindings", "source_manifest")]
    controls_rows += controls["original_sources"] + controls["actual_op01_receipts"] + [controls["actual_op01_composition"]]
    if number >= 2:
        producer = load(controls["producer_result"]["path"])
        frozen = load(controls["producer_freeze"]["path"])
        rb = load(controls["producer_readback"]["path"])
        assert producer["four_operations_capture_qualified"] is True and producer["failure"] is None
        assert producer["accepted_case_pins"] == frozen["accepted_case_pins"]
        assert rb["caseFiles10Verified"] is True and rb["RootOpenedPrivateExpected"] is False
        assert len(frozen["accepted_case_pins"]) == 10
        generated += frozen["accepted_case_pins"]
        for row in generated:
            p = Path(row["path"])
            assert p.is_file() and not p.is_symlink() and str(p.resolve()) == row["path"]
            h["check"](row)  # Opaque bytes/hash only; no actual semantic parsing by composition.
        if number == 2:
            assert not (QA / "phase-A").exists()
            binding_path = QA / "ROOT-CASE-BINDINGS.json"
            exclusive(binding_path, Path(controls["case_bindings_proposal"]["path"]).read_bytes())
            binding_pin = h["pin"](binding_path)
            for row in generated:
                fsync_file(row["path"])
            durable_save(h, QA / "ROOT-ACTUAL-ONLY-INPUT-FREEZE.json", {
                "schema": "root-nominal-actual-only-input-freeze/1", "created_utc": utc(),
                "Root_direct_invocation_selects_passive_project_after_actual_synthetic_PASS": True,
                "original_producer_freeze": controls["producer_freeze"], "case_bindings": binding_pin,
                "capture_files": generated, "record_counts_metadata": controls["record_count_metadata"],
                "expected_semantic_read": False, "producer_or_Core_replayed": False})
        generated += [h["pin"](QA / "ROOT-CASE-BINDINGS.json"), h["pin"](QA / "ROOT-ACTUAL-ONLY-INPUT-FREEZE.json")]
    if number == 3:
        previous = prior(h, 2, controls_pin)
        assert not (QA / "comparison").exists()
        projection = QA / "phase-A/ACTUAL-PROJECTION.json"
        receipt = QA / "phase-A/PHASE-A-RECEIPT.json"
        expected_outputs = [h["pin"](projection), h["pin"](receipt)]
        assert previous["outputs"] == expected_outputs
        for path in (projection, receipt):
            fsync_file(path)
        root_freeze = QA / "ROOT-ACTUAL-AND-PROJECTION-FREEZE.json"
        exclusive(root_freeze, (json.dumps({
            "schema": "root-nominal-actual-projection-freeze/1",
            "actual_and_projection_frozen_before_expected_read": True,
            "phase_a_receipt": expected_outputs[1], "projection": expected_outputs[0],
            "safe_bindings": controls["safe_bindings"]}, indent=2) + "\n").encode())
        # This external Root freeze is durable BEFORE the first expected byte hash.
        generated += expected_outputs + [h["pin"](root_freeze)]
        safe = load(controls["safe_bindings"]["path"])  # safe opaque bindings only
        assert safe["expected_schema"] == "sherlock-nominal-text-symbolic-expectation/1"
        generated.append(safe["private_expected"])
    before = check_rows(h, flat + controls_rows + generated)
    durable_save(h, phase / "PRE-PIN-CHECK.json", before)
    durable_save(h, phase / "ROOT-SELECTION.json", {
        "created_utc": utc(), "Root_direct_invocation_selected_operation": number,
        "operation_id": op["id"], "argv_env_caps": op, "execution_controls": controls_pin,
        "composition": composition["script"], "actual_review": controls["review"],
        "review_identity": controls["review_identity"], "generated_inputs": generated,
        "prior_accepted_operations_reused_not_replayed": list(range(1, number)),
        "no_retry_no_cap_raise_no_new_producer": True})
    outcome = {"operation_number": number, "id": op["id"], "accepted": False,
               "execution_controls": controls_pin, "outputs": []}
    try:
        raw_result = h["capture"](op, phase)
        # Save the raw actual return IMMEDIATELY; never assert on it first.
        durable_save(h, phase / "RAW-CAPTURE-RESULT.json", raw_result)
        for name in ("stdout.bin", "stderr.bin"):
            if (phase / name).exists():
                fsync_file(phase / name)
        if (phase / "COMMAND.json").exists():
            fsync_file(phase / "COMMAND.json")
        observed_outputs = []
        for path in op["outputs"]:
            p = Path(path)
            if p.exists():
                assert p.is_file() and not p.is_symlink()
                fsync_file(p)
                observed_outputs.append(h["pin"](p))
        # Freeze all returned artifact bytes BEFORE any output JSON/status parsing.
        durable_save(h, phase / "OUTPUT-BYTES-FROZEN-BEFORE-PARSE.json", {
            "created_utc": utc(), "raw_result": h["pin"](phase / "RAW-CAPTURE-RESULT.json"),
            "outputs": observed_outputs, "producer_or_expected_repair": False})
        outcome["outputs"] = observed_outputs
        assert raw_result["actual_exit"] == 0 and raw_result["controller_failure"] is None
        assert len(raw_result["streams"]) == 2 and all(x["full_stream_retained"] for x in raw_result["streams"])
        for row in raw_result["streams"]:
            h["check"]({k: row[k] for k in ("path", "bytes", "sha256")})
        assert len(observed_outputs) == len(op["outputs"])
        if number == 2:
            stdout = metadata_stdout(h, phase)
            assert stdout == {"projection": observed_outputs[0], "receipt": observed_outputs[1], "expected_read": False}
            projection = load(observed_outputs[0]["path"])
            receipt = load(observed_outputs[1]["path"])
            assert projection["expected_read"] is False and receipt["expected_read"] is False
            statuses = {k: v["status"] for k, v in projection["cases"].items()}
            assert set(statuses) == {"ordered-schedule.mori", "generic-recursive-fields.mori"}
            assert all(v == "ESTABLISHED" for v in statuses.values()) and receipt["case_status"] == statuses
            assert receipt["outputs"] == observed_outputs[:1]
            check_rows(h, receipt["inputs"] + receipt["outputs"])
            outcome["case_status"] = statuses
            outcome["expected_read"] = False
            del projection  # no semantic fields rendered or adopted by this metadata check
        else:
            stdout = metadata_stdout(h, phase)
            assert stdout == observed_outputs[0]
            comparison = load(observed_outputs[0]["path"])
            assert comparison["root_freeze"] == h["pin"](QA / "ROOT-ACTUAL-AND-PROJECTION-FREEZE.json")
            assert set(comparison["cases"]) == {"ordered-schedule.mori", "generic-recursive-fields.mori"}
            statuses = {}
            for key, result in comparison["cases"].items():
                assert result["status"] == "PASS" and result["fields"] == dict.fromkeys(SEMANTIC, "PASS")
                assert result["first_divergence"] is None
                statuses[key] = {"status": result["status"], "fields": result["fields"]}
            outcome["case_status"] = statuses  # never copies protected excerpts/values
            outcome["scope"] = "Full declared source-symbolic fields only; no native/physical-clock/EARS claim"
        outcome["accepted"] = True
    except BaseException as error:
        outcome["failure"] = type(error).__name__ + ": " + str(error)[:1024]
    finally:
        try:
            after = check_rows(h, flat + controls_rows + generated + outcome["outputs"])
            durable_save(h, phase / "POST-PIN-CHECK.json", after)
        except BaseException as error:
            outcome["accepted"] = False
            outcome["postcheck_failure"] = type(error).__name__ + ": " + str(error)[:1024]
        durable_save(h, phase / "ACCEPTANCE.json", outcome)
        phases = [{"operation_number": 1, "accepted": True,
                   "actual_Root_receipt": h["pin"](ACT / "01-synthetic-only/ROOT-OUTCOME.json"),
                   "scope": "Existing exact23 synthetic PASS reused, never replayed"}]
        for op_index in (1, 2):
            p = ACT / plan["operations"][op_index]["id"] / "ACCEPTANCE.json"
            if p.exists():
                phases.append(load(p))
        durable_save(h, ACT / "ROOT-RESULT.json", {
            "phases": phases, "all_three_passed": len(phases) == 3 and all(p["accepted"] for p in phases),
            "downstream_unrun": [n for n in range(number + 1, 4)],
            "source_and_expected_immutable": True, "no_retry_or_cap_raise": True,
            "no_new_Core_or_producer_prefix": True,
            "scope": "Passive nominal-text symbolic projection/comparison only, no physical/native/EARS acceptance"})
        if not outcome["accepted"]:
            durable_save(h, ACT / "FAILURE.json", {"operation_number": number, "outcome": outcome,
                "later_operations_UNRUN": list(range(number + 1, 4)), "no_retry_or_repair": True})
    print(json.dumps({"operation": number, "accepted": outcome["accepted"], "receipt": h["pin"](phase / "ACCEPTANCE.json")}))
    return 0 if outcome["accepted"] else 1


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("operation", type=int, choices=(2, 3))
    args = parser.parse_args()
    try:
        raise SystemExit(run_one(args.operation))
    except (AssertionError, OSError, ValueError, KeyError, TypeError) as error:
        # A pre-dispatch stop is metadata only, never a failed subject/retry.
        path = HERE / ("ROOT-PRECHECK-OP%02d.json" % args.operation)
        value = {"operation": args.operation, "precheck_or_composition_failure": True,
                 "subject_dispatch_status": "inspect any COMMAND/raw receipt; not inferred from this handler",
                 "error": type(error).__name__ + ": " + str(error)[:1024]}
        try:
            exclusive(path, (json.dumps(value, indent=2) + "\n").encode())
        except OSError:
            pass
        print(json.dumps(value), file=sys.stderr)
        raise SystemExit(2)
