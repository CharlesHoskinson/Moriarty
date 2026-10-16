#!/usr/bin/env python3
"""Ordinary captured-data validator for the K8 genuine-proof record.

Consumes only three pinned, already-captured public inputs. It regenerates
no proof, reruns no native test, imports no operator/SDK/native code, and
preserves the historical wrapper status FAILED. See README.md.
"""
import argparse
import hashlib
import json
import re
import sys
from pathlib import Path

MAX_INPUT = 65536
U32_MAX = 4294967295
MODULE = "pvc2/k8_genuine_proof_test"
ORIGINAL_ERROR = (
    "step2 JSON on_failure mismatch for k8_short_vk_is_refused: "
    "want succeed_immediately"
)
PINS = {
    "source": (4782, "d25c3811132ee99dab4bba9862ec87147931c80cd516e6eae5940e2088551eec"),
    "telemetry": (2914, "9ac4871e8923873843d607b42648f31987834ac1fdbf79cf5b7607898a015800"),
    "original_result": (4875, "5bc3033c0274cb50f2d3e473c74974a547bd4a896c5e7cd1d4e82c47b45ece8f"),
}
EXPECTED_TITLES = [
    "k8_positive_verify_digest",
    "k8_n1_body_rederived_digest_rejected",
    "k8_n2_context_rederived_digest_rejected",
    "k8_n3_proof_ac_swapped_rejected",
    "k8_scalars_are_be128_halves_of_statement_digest",
    "k8_n1_scalars_are_be128_halves_of_n1_digest",
    "k8_n2_scalars_are_be128_halves_of_n2_digest",
    "k8_byte_profile_matches_groth16_layout",
    "k8_negative_cases_are_distinct_from_positive",
    "k8_short_vk_is_refused",
]
PAIRING_TITLES = EXPECTED_TITLES[:4]
TEST_RE = re.compile(r"^test ([a-z0-9_]+)\(\)(?: fail( once)?)? \{$" , re.MULTILINE)


def fail(msg):
    print(f"validate_capture: error: {msg}", file=sys.stderr)
    raise SystemExit(1)


def read_pinned(path, kind):
    want_bytes, want_sha = PINS[kind]
    try:
        with open(path, "rb") as fh:
            data = fh.read(MAX_INPUT + 1)
    except OSError as exc:
        fail(f"{kind}: cannot read {path}: {exc}")
    if len(data) > MAX_INPUT:
        fail(f"{kind}: input exceeds {MAX_INPUT} bytes")
    if len(data) != want_bytes:
        fail(f"{kind}: size {len(data)} != pinned {want_bytes}")
    digest = hashlib.sha256(data).hexdigest()
    if digest != want_sha:
        fail(f"{kind}: sha256 {digest} != pinned {want_sha}")
    return data, {"path": str(path), "bytes": len(data), "sha256": digest}


def no_dupes(pairs):
    obj = {}
    for key, value in pairs:
        if key in obj:
            fail(f"duplicate JSON key: {key!r}")
        obj[key] = value
    return obj


def parse_json(data, kind):
    try:
        return json.loads(data.decode("utf-8"), object_pairs_hook=no_dupes)
    except (UnicodeDecodeError, json.JSONDecodeError) as exc:
        fail(f"{kind}: invalid JSON: {exc}")


def is_int(value):
    return type(value) is int


def exact_keys(obj, keys, where):
    if not isinstance(obj, dict) or set(obj) != set(keys):
        fail(f"{where}: keys must be exactly {sorted(keys)}")


def check_summary(obj, where):
    exact_keys(obj, {"total", "passed", "failed", "kind"}, where)
    for field in ("total", "passed", "failed"):
        if not is_int(obj[field]):
            fail(f"{where}.{field}: must be an integer, not bool/other")
    if (obj["total"], obj["passed"], obj["failed"]) != (10, 10, 0):
        fail(f"{where}: must be total=10 passed=10 failed=0")
    exact_keys(obj["kind"], {"unit", "property"}, f"{where}.kind")
    if not is_int(obj["kind"]["unit"]) or not is_int(obj["kind"]["property"]):
        fail(f"{where}.kind: unit/property must be integers, not bool/other")
    if (obj["kind"]["unit"], obj["kind"]["property"]) != (10, 0):
        fail(f"{where}.kind: must be unit=10 property=0")


def source_modes(source_bytes):
    try:
        text = source_bytes.decode("utf-8")
    except UnicodeDecodeError as exc:
        fail(f"source: not UTF-8: {exc}")
    found = TEST_RE.findall(text)
    names = [name for name, _once in found]
    if names != EXPECTED_TITLES:
        fail(f"source: declarations {names} != pinned ordered titles")
    modes = {}
    for name, once in found:
        if once:
            modes[name] = "succeed_immediately"
        elif f"test {name}() fail {{" in text:
            modes[name] = "succeed_eventually"
        else:
            modes[name] = "fail_immediately"
    return modes


def main():
    ap = argparse.ArgumentParser(description="Validate captured K8 telemetry.")
    ap.add_argument("--source", required=True)
    ap.add_argument("--telemetry", required=True)
    ap.add_argument("--original-result", required=True)
    ap.add_argument("--output-dir", required=True)
    args = ap.parse_args()
    out_dir = Path(args.output_dir)
    if out_dir.exists():
        fail(f"output-dir already exists, refusing to overwrite: {out_dir}")
    src_data, src_pin = read_pinned(Path(args.source), "source")
    tel_data, tel_pin = read_pinned(Path(args.telemetry), "telemetry")
    res_data, res_pin = read_pinned(Path(args.original_result), "original_result")
    modes = source_modes(src_data)
    tel = parse_json(tel_data, "telemetry")
    exact_keys(tel, {"seed", "summary", "modules"}, "telemetry")
    if not is_int(tel["seed"]) or not 0 <= tel["seed"] <= U32_MAX:
        fail("telemetry.seed: must be an integer in u32 range")
    check_summary(tel["summary"], "telemetry.summary")
    if not isinstance(tel["modules"], list) or len(tel["modules"]) != 1:
        fail("telemetry.modules: must be a single-module list")
    module = tel["modules"][0]
    exact_keys(module, {"name", "summary", "tests"}, "telemetry.modules[0]")
    if module["name"] != MODULE:
        fail(f"telemetry.modules[0].name: must be {MODULE!r}")
    check_summary(module["summary"], "telemetry.modules[0].summary")
    rows = module["tests"]
    if not isinstance(rows, list) or len(rows) != 10:
        fail("telemetry tests: must be exactly 10 rows")
    titles = []
    for row in rows:
        exact_keys(row, {"title", "status", "on_failure", "execution_units"}, "test row")
        if not isinstance(row["title"], str):
            fail("test row title: must be a string")
        titles.append(row["title"])
        if row["status"] != "pass":
            fail(f"test {row['title']}: status must be pass")
        if row["on_failure"] != modes[row["title"]]:
            fail(f"test {row['title']}: on_failure must be {modes[row['title']]}")
        exact_keys(row["execution_units"], {"mem", "cpu"}, "execution_units")
        for field in ("mem", "cpu"):
            value = row["execution_units"][field]
            if not is_int(value) or value < 0:
                fail(f"test {row['title']}: execution_units.{field} typed non-neg int required")
    if titles != EXPECTED_TITLES or len(set(titles)) != 10:
        fail("telemetry titles: missing/extra/duplicate or wrong order")
    original = parse_json(res_data, "original-result")
    if not isinstance(original, dict):
        fail("original-result: must be a JSON object")
    if original.get("status") != "FAILED":
        fail("original-result: status must remain FAILED")
    if original.get("error") != ORIGINAL_ERROR:
        fail("original-result: error wording changed")
    record = {
        "format": "cardano-k2-k8-captured-telemetry-validation/1",
        "ordinaryValidationOnly": True,
        "nativeReexecuted": False,
        "newNativeAuthority": False,
        "originalWrapperStatus": "FAILED",
        "originalWrapperStatusChanged": False,
        "rawCapturedPassedUnits": 10,
        "normalRows": 9,
        "pairingOutcomesCount": 4,
        "pairingTitles": PAIRING_TITLES,
        "module": MODULE,
        "seed": tel["seed"],
        "inputs": {"source": src_pin, "telemetry": tel_pin, "originalResult": res_pin},
        "rows": [
            {"title": r["title"], "status": r["status"],
             "on_failure": r["on_failure"], "expected_on_failure": modes[r["title"]],
             "execution_units": r["execution_units"]} for r in rows
        ],
        "outputs": ["VALIDATION.json"],
    }
    payload = (json.dumps(record, indent=2, sort_keys=True) + "\n").encode()
    try:
        out_dir.mkdir(parents=False)
        with open(out_dir / "VALIDATION.json", "xb") as fh:
            fh.write(payload)
    except OSError as exc:
        fail(f"cannot write fresh output: {exc}")
    print(f"wrote {out_dir / 'VALIDATION.json'}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
