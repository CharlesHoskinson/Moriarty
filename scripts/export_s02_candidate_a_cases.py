"""Structural ITF export only; no candidate or reference semantic evaluation."""

from __future__ import annotations

import argparse
from hashlib import sha256
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
BASE_PINS = frozenset({"moriarty/core.py", "moriarty/swap.py"} | {
    f"specs/quint/s02/{name}.qnt" for name in (
        "candidate_a_core", "candidate_a_types", "candidate_a_programs",
        "candidate_a_projection", "effects", "observations")})
GENERATORS = {
    "swapTrace": ("specs/quint/s02/candidate_a_harness.qnt", "SwapComputedA", "canonical-swap-v1"),
    "installmentTrace": ("specs/quint/s02/candidate_a_installment_harness.qnt", "InstallmentComputedA", "installment-two-when-v1"),
    "diagnosticCases": ("specs/quint/s02/candidate_a_cases.qnt", "CaseComputedA", None),
}
OPTIONAL_PINS = frozenset({value[0] for value in GENERATORS.values()} | {"scripts/export_s02_candidate_a_cases.py"})
FIXTURE_IDS = frozenset({"canonical-swap-v1", "installment-two-when-v1", "close-v1",
    "pay-zero-v1", "pay-negative-v1", "pay-ten-v1", "if-settle-zero-v1",
    "deposit-five-close-v1", "deposit-zero-close-v1", "pre-deposit-post-v1",
    "pre-warning-deposit-v1", "ordered-choice-v1", "ordered-choice-overlap-v1"})


class ExportError(ValueError):
    """A structural or source-binding error; never a Core result."""


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ExportError(message)


def fields(value, names: set[str], label: str) -> None:
    require(isinstance(value, dict) and set(value) == names, f"malformed {label} fields")


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, f"duplicate JSON key: {key}")
        result[key] = value
    return result


def load_json(data: bytes, label: str):
    try:
        return json.loads(data, object_pairs_hook=unique_object,
                          parse_constant=lambda value: (_ for _ in ()).throw(ExportError(f"invalid JSON constant {value}")))
    except (UnicodeError, json.JSONDecodeError) as exc:
        raise ExportError(f"malformed JSON {label}: {exc}") from exc


def relative_path(name: str) -> None:
    require(isinstance(name, str) and name != "" and "\\" not in name
            and not Path(name).is_absolute() and all(part not in ("", ".", "..") for part in name.split("/")),
            f"unsafe relative path: {name}")


def bound_path(root: Path, name: str) -> Path:
    relative_path(name)
    target = (root / name).resolve()
    require(target.is_relative_to(root.resolve()), f"path escapes root: {name}")
    return target


def tagged(value, label: str) -> None:
    fields(value, {"tag", "value"}, label)
    require(isinstance(value["tag"], str) and value["tag"] != "", f"malformed {label} tag")


def raw_state(value, label: str, projected: bool = False) -> None:
    fields(value, {"accounts", "choices", "continuation", "minimumTime"}, label)
    for name in ("accounts", "choices"):
        fields(value[name], {"#map"}, f"{label}.{name}")
        require(isinstance(value[name]["#map"], list), f"malformed {label}.{name}")
    if projected:
        fields(value["continuation"], {"program", "node"}, f"{label}.continuation")
    else:
        tagged(value["continuation"], f"{label}.continuation")


def raw_itf(value) -> None:
    """Validate generic ITF container syntax, without decoding symbols/maps."""
    if isinstance(value, list):
        for item in value:
            raw_itf(item)
    elif isinstance(value, dict):
        special = [key for key in value if key.startswith("#")]
        if special:
            require(len(value) == 1, "malformed ITF special wrapper")
            if special == ["#bigint"]:
                require(isinstance(value["#bigint"], str) and re.fullmatch(r"-?(0|[1-9][0-9]*)", value["#bigint"]) is not None,
                        "malformed ITF bigint")
                return
            require(special[0] in ("#map", "#tup", "#set") and isinstance(value[special[0]], list),
                    "malformed ITF container")
            if special == ["#map"]:
                require(all(isinstance(pair, list) and len(pair) == 2 for pair in value["#map"]), "malformed ITF map pairs")
        for item in value.values():
            raw_itf(item)
    else:
        require(type(value) in (str, bool, int), "malformed ITF scalar")


def selected_fields(entry, normal_tag: str) -> dict:
    request, payload = entry["request"], entry["payload"]
    fields(request, {"program", "before", "input", "now"}, "request")
    fields(request["program"], {"root", "nodes"}, "request.program")
    raw_state(request["before"], "request.state")
    tagged(request["input"], "request.input")
    tagged(request["now"], "request.now")
    tagged(payload, "payload")
    require(payload["tag"] == normal_tag, f"diagnostic or unknown payload: {payload['tag']}")
    fields(payload["value"], {"raw", "projection", "effects"}, "payload")
    computed = payload["value"]
    for name in ("raw", "projection"):
        value = computed[name]
        fields(value, {"accepted", "state", "error", "payments", "warnings", "reductions"}, name)
        require(type(value["accepted"]) is bool, f"malformed {name}.accepted")
        require(isinstance(value["payments"], list) and isinstance(value["warnings"], list), f"malformed {name} lists")
        tagged(value["error"], f"{name}.error")
        fields(value["reductions"], {"#bigint"}, f"{name}.reductions")
        raw_state(value["state"], f"{name}.state", projected=name == "projection")
    require(isinstance(computed["effects"], list), "malformed effects list")
    for transfer in computed["effects"]:
        fields(transfer, {"source", "destination", "asset", "quantity"}, "effects transfer")
    selected = {"request": request, "result": computed["raw"],
                "projection": computed["projection"], "effects": computed["effects"]}
    raw_itf(selected)
    return selected


def export_cases(itf_dir: Path, source_root: Path = ROOT) -> dict:
    itf_dir, source_root = Path(itf_dir), Path(source_root)
    try:
        binding = load_json((itf_dir / "source-bindings.json").read_bytes(), "source binding")
    except OSError as exc:
        raise ExportError(f"source binding unavailable: {exc}") from exc
    fields(binding, {"schema_version", "source_pins", "input_pins"}, "source binding")
    require(type(binding["schema_version"]) is int and binding["schema_version"] == 1, "unsupported binding schema")
    pins, inputs = binding["source_pins"], binding["input_pins"]
    require(isinstance(pins, dict) and BASE_PINS <= set(pins) <= BASE_PINS | OPTIONAL_PINS, "missing or unknown source pins")
    require(isinstance(inputs, dict) and bool(inputs), "empty input inventory")
    for name, digest in {**pins, **inputs}.items():
        relative_path(name)
        require(isinstance(digest, str) and re.fullmatch(r"[0-9a-f]{64}", digest) is not None, f"malformed hash: {name}")
    for name, digest in pins.items():
        try:
            actual = sha256(bound_path(source_root, name).read_bytes()).hexdigest()
        except OSError as exc:
            raise ExportError(f"source unavailable: {name}: {exc}") from exc
        require(actual == digest, f"source hash mismatch: {name}")
    discovered = {path.relative_to(itf_dir).as_posix() for path in itf_dir.rglob("*.itf.json")}
    require(discovered == set(inputs), "declared ITF inventory mismatch")
    cases = []
    for name in sorted(inputs):
        data = bound_path(itf_dir, name).read_bytes()
        digest = sha256(data).hexdigest()
        require(digest == inputs[name], f"input hash mismatch: {name}")
        document = load_json(data, name)
        fields(document, {"#meta", "vars", "states"}, "ITF document")
        require(isinstance(document["vars"], list) and len(document["vars"]) == 1
                and isinstance(document["vars"][0], str) and document["vars"][0] in GENERATORS,
                "unknown ITF variable")
        variable = document["vars"][0]
        generator, normal_tag, fixture = GENERATORS[variable]
        require(generator in pins, f"missing generator source pin: {generator}")
        require(isinstance(document["#meta"], dict) and document["#meta"].get("format") == "ITF"
                and document["#meta"].get("source") == generator, "unbound ITF source metadata")
        require(isinstance(document["states"], list) and bool(document["states"]), "empty ITF states")
        previous, seen = [], {}
        for state_index, state in enumerate(document["states"]):
            require(isinstance(state, dict) and variable in state and set(state) <= {"#meta", variable}, "missing or extra state variable")
            value = state[variable]
            if variable == "diagnosticCases":
                entries = value
            else:
                fields(value, {"agreement", "ledger", "records"}, "trace")
                entries = value["records"]
            require(isinstance(entries, list), "malformed record history")
            require(len(entries) >= len(previous), "shortened record history")
            require(entries[:len(previous)] == previous, "conflicting repeated history prefix")
            previous = entries
            for step_index, entry in enumerate(entries):
                if variable == "diagnosticCases":
                    fields(entry, {"case_id", "fixture_id", "request", "payload"}, "case payload")
                    require(isinstance(entry["case_id"], str) and bool(entry["case_id"]), "malformed case identifier")
                    require(entry["fixture_id"] in FIXTURE_IDS, "unknown fixture identifier")
                    label, fixture_id = entry["case_id"], entry["fixture_id"]
                else:
                    fields(entry, {"request", "beforeLedger", "afterLedger", "payload"}, "trace payload")
                    label, fixture_id = str(step_index), fixture
                selected = selected_fields(entry, normal_tag)
                case_id = f"{name}::{label}"
                provenance = {"trace_id": name, "state_index": state_index, "step_index": step_index,
                              "input_path": name, "input_sha256": digest}
                if step_index in seen:
                    seen[step_index]["provenance"].append(provenance)
                else:
                    require(not any(case["case_id"] == case_id for case in cases), "duplicate case identifier")
                    exported = {"case_id": case_id, "fixture_id": fixture_id, "trace_id": name,
                                "step_index": step_index, **selected, "provenance": [provenance]}
                    cases.append(exported)
                    seen[step_index] = exported
        require(bool(seen), f"empty transaction corpus: {name}")
    return {"schema_version": 1, "source_pins": pins, "cases": cases}


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--itf-dir", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--source-root", type=Path, default=ROOT)
    args = parser.parse_args(argv)
    try:
        document = export_cases(args.itf_dir, args.source_root)
        with args.out.open("x", encoding="utf-8") as stream:
            json.dump(document, stream, ensure_ascii=False, indent=2)
            stream.write("\n")
    except (ExportError, OSError) as exc:
        print(json.dumps({"status": "error", "error": str(exc)}), file=sys.stderr)
        return 1
    print(json.dumps({"status": "exported", "cases": len(document["cases"]), "out": str(args.out)}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
