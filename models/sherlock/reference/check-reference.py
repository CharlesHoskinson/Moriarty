#!/usr/bin/env python3
"""Compare production Sherlock against eight frozen normal-program trace oracles.

Only Python's standard library and the explicitly selected rustc are used. This script
verifies committed Rust constants are the exact transcription of existing JSON
literals, compiles the actual production module and runs its comparison tests.
It does not interpret programs, calculate expected work, or update any oracle file.
The executable lives only in a temporary directory.
"""

import argparse
import hashlib
import json
from pathlib import Path
import shlex
import subprocess
import sys
import tempfile


FROZEN = {
    "models/sherlock/oracles/normal-programs.json":
        "ef87f94f7e5d22a75b1c2e5e61af016b08175beee9a3599e3e69ddf92945d32f",
    "models/sherlock/oracles/normal-prefixes.json":
        "dca67b553d7483fdf7d9148e7f4bf1a6022b42b8843635f33f23c58a2415d56d",
    "models/sherlock/k/main.k":
        "b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41",
    "models/sherlock/oracles/extra-normal-programs.json":
        "dd85e5d400f24e9be5e2ff017f4b8c141a0bf6e4270053917b735cbe5153f153",
    "models/sherlock/oracles/extra-normal-prefixes.json":
        "acf460094b258910bad0499fb8feda0cd85b680023939800ad714d2cb71bee16",
    "models/sherlock/oracles/EXTRA-NORMAL-DERIVATION.md":
        "72edaf6d2dc93c91569833bad360d7601b51c1dfa3949c8f68a2bc654f0a8935",
    "models/sherlock/oracles/build-extra-normal-literals.py":
        "3dc30c55e1fecdadadc95f1945a5475144d31a6f9389a9bc7f8f368eb09149ba",
}
SOURCE_PATHS = (
    "crates/mori-core/src/sherlock/mod.rs",
    "crates/mori-core/src/sherlock/compare.rs",
    "crates/mori-core/src/sherlock/compare_tests.rs",
    "crates/mori-core/src/sherlock/reference.rs",
    "crates/mori-core/src/sherlock/reference_types.rs",
    "crates/mori-core/src/sherlock/reference_display.rs",
    "crates/mori-core/src/sherlock/reference_tests.rs",
    "crates/mori-core/src/sherlock/resource_tests.rs",
    "crates/mori-core/src/sherlock/reference_oracles_tests.rs",
    "crates/mori-core/src/sherlock/normal_oracles.rs",
    "crates/mori-core/src/sherlock/extra_normal_oracles.rs",
    "crates/mori-core/src/sherlock/extra_oracle_programs.rs",
    "crates/mori-core/src/sherlock/program_image.rs",
    "crates/mori-core/src/sherlock/program_image_tests.rs",
    "crates/mori-core/src/sherlock/program_image_oracles.rs",
    "crates/mori-core/src/sherlock/image_reference.rs",
    "crates/mori-core/src/sherlock/image_reference_tests.rs",
    "crates/mori-core/src/sherlock/reference_slice.rs",
    "crates/mori-core/src/sherlock/reference_slice_tests.rs",
    "crates/mori-core/src/sherlock/machine_image.rs",
    "crates/mori-core/src/sherlock/machine_image_tests.rs",
    "crates/mori-core/src/sherlock/helper_schedule.rs",
    "crates/mori-core/src/sherlock/helper_schedule_tests.rs",
    "crates/mori-core/src/sherlock/helper_call_adapter.rs",
    "crates/mori-core/src/sherlock/helper_call_adapter_tests.rs",
    "crates/mori-core/src/sherlock/source_origin_normal_tests.rs",
    "crates/mori-core/src/sherlock/checkpoint_normal_tests.rs",
    "models/sherlock/reference/check-reference.py",
)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def emit(record):
    print(json.dumps(record, sort_keys=True), flush=True)


def rust_string(value):
    # Frozen source terms use printable ASCII. On this domain JSON only needs
    # quote/backslash escapes, which are also valid Rust escapes. Reject controls
    # rather than emitting JSON's Rust-incompatible backspace/formfeed/u escapes.
    if not isinstance(value, str) or any(not 32 <= ord(char) <= 126 for char in value):
        raise ValueError("expected a printable ASCII source literal")
    return json.dumps(value, ensure_ascii=True)


def integer(value):
    if type(value) is not int or value < 0:
        raise ValueError("expected a nonnegative integer literal")
    return str(value)


def field(name, value):
    return f"{name}: {value},"


def serialize_oracles(programs, prefixes, *,
                      case_ids=("N01", "N02", "N03", "N04", "N05", "N06"),
                      programs_file="normal-programs.json", constant_prefix=""):
    """Serialize supplied literals; no evaluator or expected-work derivation."""
    if programs["profile"] != prefixes["profile"]:
        raise ValueError("oracle profile mismatch")
    if programs["source"] != prefixes["source"]:
        raise ValueError("oracle source mismatch")
    inputs = {program["case_id"]: program for program in programs["programs"]}
    traces = prefixes["traces"]
    if list(inputs) != list(case_ids):
        raise ValueError("frozen normal-program membership mismatch")
    if [trace["case_id"] for trace in traces] != list(inputs):
        raise ValueError("trace membership mismatch")
    lines = [
        "// Mechanical transcription of the frozen JSON literals; do not edit.",
        f"const {constant_prefix}ORACLE_PROFILE: &str = {rust_string(programs['profile'])};",
        f"const {constant_prefix}ORACLE_SOURCE_SHA256: &str = {rust_string(programs['source']['sha256'])};",
        f"static {constant_prefix}CASES: &[ExpectedCase] = &[",
    ]
    for trace in traces:
        source = inputs[trace["case_id"]]
        fixed = trace["fixed_state_fields"]
        code_ref = fixed["code_ref"]
        if (trace["input_sha256"] != source["input_sha256"]
                or code_ref["input_sha256"] != source["input_sha256"]
                or code_ref["case_id"] != source["case_id"]
                or code_ref["file"] != programs_file):
            raise ValueError("immutable source reference mismatch")
        if fixed["initial_fuel"] != source["exact_completion_fuel"]:
            raise ValueError("literal completion fuel mismatch")
        if fixed["stack_limit"] != source["stack_limit"]:
            raise ValueError("literal stack limit mismatch")
        lines.append("ExpectedCase {")
        for name, key in (("case_id", "case_id"), ("input_sha256", "input_sha256"),
                          ("code", "code_k"), ("entry", "entry_k"),
                          ("expected_value", "expected_value_k")):
            lines.append(field(name, rust_string(source[key])))
        lines.extend([
            field("initial_fuel", integer(fixed["initial_fuel"])),
            field("stack_limit", integer(fixed["stack_limit"])),
            field("pending_output", rust_string(fixed["output_k"])),
            "states: &[",
        ])
        for state in trace["states"]:
            lines.append("ExpectedState {")
            for name in ("charged_steps", "remaining_fuel", "depth"):
                lines.append(field(name, integer(state[name])))
            for name, key in (("control", "control_k"), ("environment", "environment_k")):
                lines.append(field(name, rust_string(state[key])))
            lines.append(field("frames", "&[" + ", ".join(
                rust_string(frame) for frame in state["frames_top_first_k"]) + "]"))
            via_rule = state["via_rule_start_line"]
            lines.append(field("via_rule_start_line", "None" if via_rule is None
                               else f"Some({integer(via_rule)})"))
            lines.append("},")
        publication = trace["publication"]
        lines.extend([
            "],",
            field("publication_pre_state_index", integer(publication["pre_state_index"])),
            field("publication_rule_start_line", integer(publication["rule_start_line"])),
            field("publication_charge", integer(publication["charge"])),
            field("publication_control", rust_string(publication["post_overrides"]["control_k"])),
            field("publication_output", rust_string(publication["post_overrides"]["output_k"])),
            "prefixes: &[",
        ])
        for row in trace["ordinary_fuel_prefixes"]:
            if len(row) != 5:
                raise ValueError("expected five literal prefix columns")
            lines.extend([
                "ExpectedPrefix {",
                field("initial_fuel", integer(row[0])),
                field("boundary_state_index", integer(row[1])),
                field("remaining_fuel", integer(row[2])),
                field("output", rust_string(row[3])),
                field("publication_rule_start_line", integer(row[4])),
                "},",
            ])
        lines.extend(["],", "},"])
    lines.append("];\n")
    return "\n".join(lines)


def command(argv, repo, timeout):
    emit({"event": "command", "argv": argv, "cwd": str(repo),
          "shell_display": shlex.join(argv), "timeout_seconds": timeout})
    try:
        result = subprocess.run(argv, cwd=repo, timeout=timeout,
                                check=False)
    except subprocess.TimeoutExpired:
        emit({"event": "command_result", "status": "timeout", "timeout_seconds": timeout})
        return 124
    emit({"event": "command_result", "exit_code": result.returncode})
    return result.returncode


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rustc", type=Path, required=True, metavar="ABSOLUTE_PATH",
                        help="exact compiler executable; no toolchain discovery or installation")
    parser.add_argument("--timeout-seconds", type=int, default=30,
                        help="timeout for each of compile and test (1 through 60; default 30)")
    args = parser.parse_args()
    if not 1 <= args.timeout_seconds <= 60:
        parser.error("--timeout-seconds must be between 1 and 60")
    if not args.rustc.is_absolute() or not args.rustc.is_file():
        parser.error("--rustc must name an existing absolute compiler file")
    repo = Path(__file__).resolve().parents[3]
    for relative, expected in FROZEN.items():
        actual = digest(repo / relative)
        if actual != expected:
            raise ValueError(f"frozen source changed: {relative}: {actual} != {expected}")
    hashes = {relative: digest(repo / relative) for relative in (*FROZEN, *SOURCE_PATHS)}
    hashes["rustc"] = digest(args.rustc)
    emit({"event": "inputs", "sha256": hashes,
          "scope": "eight normal A-profile programs; no native/model execution or refinement claim"})
    programs = json.loads((repo / "models/sherlock/oracles/normal-programs.json").read_text())
    prefixes = json.loads((repo / "models/sherlock/oracles/normal-prefixes.json").read_text())
    generated = serialize_oracles(programs, prefixes)
    constants = repo / "crates/mori-core/src/sherlock/normal_oracles.rs"
    if constants.read_bytes() != generated.encode("utf-8"):
        raise ValueError("committed normal oracle constants differ from frozen JSON transcription")
    emit({"event": "oracle_transcription", "path": str(constants),
          "sha256": digest(constants), "exact_match": True})
    extra_programs = json.loads(
        (repo / "models/sherlock/oracles/extra-normal-programs.json").read_text())
    extra_prefixes = json.loads(
        (repo / "models/sherlock/oracles/extra-normal-prefixes.json").read_text())
    extra_generated = serialize_oracles(
        extra_programs, extra_prefixes, case_ids=("N07", "N08"),
        programs_file="extra-normal-programs.json", constant_prefix="EXTRA_")
    extra_constants = repo / "crates/mori-core/src/sherlock/extra_normal_oracles.rs"
    if extra_constants.read_bytes() != extra_generated.encode("utf-8"):
        raise ValueError("extra normal oracle constants differ from frozen JSON transcription")
    emit({"event": "oracle_transcription", "path": str(extra_constants),
          "sha256": digest(extra_constants), "exact_match": True})
    with tempfile.TemporaryDirectory(prefix="sherlock-reference-") as temporary:
        temporary = Path(temporary)
        executable = temporary / "reference_check"
        production = repo / "crates/mori-core/src/sherlock/mod.rs"
        compile_command = [str(args.rustc), "--edition=2024", "--test", str(production),
                           "-o", str(executable)]
        result = command(compile_command, repo, args.timeout_seconds)
        if result:
            return result
        emit({"event": "executable", "path": str(executable), "sha256": digest(executable)})
        result = command([str(executable), "--test-threads=1", "--nocapture"],
                         repo, args.timeout_seconds)
        after = {relative: digest(repo / relative) for relative in (*FROZEN, *SOURCE_PATHS)}
        if after != {relative: hashes[relative] for relative in after}:
            raise ValueError("source bytes changed during comparison; result is not a frozen candidate")
        emit({"event": "comparison", "exit_code": result,
              "normal_programs": 8, "literal_prefixes": 216,
              "qualification": "finite normal reference comparison only"})
        return result


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, KeyError, TypeError) as error:
        emit({"event": "error", "message": str(error)})
        sys.exit(2)
