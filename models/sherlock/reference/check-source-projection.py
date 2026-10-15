#!/usr/bin/env python3
"""Check actual Core compilation and six fixed scalar-source projection tests."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import time

EXTERNS = {"mori_lexer", "serde", "serde_json", "thiserror", "sha2", "k256"}
EXPECTED_TESTS = {
    "weighted_source_retains_dynamic_input_order_and_exact_body",
    "threshold_source_preserves_predicate_type_and_unary_occurrence",
    "report_serializes_actual_mapping_and_explicit_coverage",
    "ordinary_valid_core_outside_projection_remains_explicitly_unmapped",
    "source_order_is_retained_across_distinct_action_reports",
    "shared_subexpression_fanout_stops_at_the_preparation_budget",
}
PHASE_SECONDS = 30


def digest(path: Path) -> str:
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(block)
    return value.hexdigest()


def absolute_file(value: str) -> Path:
    path = Path(value)
    if not path.is_absolute() or not path.is_file():
        raise ValueError(f"expected an existing absolute file: {value}")
    return path.resolve()


def absolute_directory(value: str) -> Path:
    path = Path(value)
    if not path.is_absolute() or not path.is_dir():
        raise ValueError(f"expected an existing absolute directory: {value}")
    return path.resolve()


def rust_string(value: str) -> str:
    """Rust ordinary string literal; JSON's \\uXXXX is not Rust syntax."""
    result = ['"']
    escapes = {"\\": "\\\\", '"': '\\"', "\n": "\\n", "\r": "\\r", "\t": "\\t"}
    for character in value:
        if character in escapes:
            result.append(escapes[character])
        elif ord(character) < 32 or ord(character) == 127:
            result.append("\\u{" + format(ord(character), "x") + "}")
        else:
            result.append(character)
    result.append('"')
    return "".join(result)


def read_dependencies(path: Path) -> tuple[dict[str, Path], dict[str, Path], dict[str, str]]:
    manifest = json.loads(path.read_text(encoding="utf-8"))
    if set(manifest) - {"externs", "proc_macros"}:
        raise ValueError("dependency manifest fields are externs and optional proc_macros")
    if set(manifest.get("externs", {})) != EXTERNS:
        raise ValueError("externs must contain exactly: " + ", ".join(sorted(EXTERNS)))
    if set(manifest.get("proc_macros", {})) - {"serde_derive", "thiserror_impl"}:
        raise ValueError("listed proc_macros may name serde_derive and thiserror_impl")
    externs: dict[str, Path] = {}
    macros: dict[str, Path] = {}
    expected: dict[str, str] = {}
    for section, destination in (("externs", externs), ("proc_macros", macros)):
        rows = manifest.get(section, {})
        if not isinstance(rows, dict):
            raise ValueError(f"{section} must be a crate-to-file mapping")
        for name, row in rows.items():
            if not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", name):
                raise ValueError(f"invalid crate name in {section}: {name}")
            if not isinstance(row, dict) or set(row) != {"path", "sha256"}:
                raise ValueError(f"{section}.{name} requires path and sha256")
            artifact = absolute_file(row["path"])
            sha = row["sha256"]
            if not isinstance(sha, str) or not re.fullmatch(r"[0-9a-f]{64}", sha):
                raise ValueError(f"invalid sha256 for {name}")
            if str(artifact) in expected and expected[str(artifact)] != sha:
                raise ValueError(f"conflicting expected hash for {artifact}")
            destination[name] = artifact
            expected[str(artifact)] = sha
    return externs, macros, expected


def source_files(repo: Path) -> list[Path]:
    src = repo / "crates/mori-core/src"
    files = list(src.rglob("*.rs"))
    fixtures = src / "sherlock/source_adapter_fixtures"
    if not fixtures.is_dir():
        raise ValueError("integrated source_adapter_fixtures directory is absent")
    files.extend(path for path in fixtures.rglob("*") if path.is_file())
    files.append(repo / "crates/mori-core/Cargo.toml")
    return sorted(set(path.resolve() for path in files))


def snapshot(repo: Path, fixed: list[Path]) -> dict[str, str]:
    return {str(path): digest(path) for path in sorted(set(source_files(repo) + fixed))}


def command_result(command: list[str], cwd: Path, phase: str) -> dict:
    started = time.monotonic()
    try:
        process = subprocess.run(command, cwd=cwd, text=True, capture_output=True,
                                 timeout=PHASE_SECONDS, check=False)
        return {"phase": phase, "command": command, "timeout_seconds": PHASE_SECONDS,
                "elapsed_seconds": time.monotonic() - started,
                "returncode": process.returncode, "timed_out": False,
                "stdout": process.stdout, "stderr": process.stderr}
    except subprocess.TimeoutExpired as error:
        def decoded(value):
            return value.decode("utf-8", errors="replace") if isinstance(value, bytes) else value or ""
        return {"phase": phase, "command": command, "timeout_seconds": PHASE_SECONDS,
                "elapsed_seconds": time.monotonic() - started,
                "returncode": None, "timed_out": True,
                "stdout": decoded(error.stdout), "stderr": decoded(error.stderr)}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", required=True)
    parser.add_argument("--rustc", required=True)
    parser.add_argument("--dependency-manifest", required=True)
    parser.add_argument("--dependency-directory", required=True)
    args = parser.parse_args()
    report = {
        "format": "moriarty-real-core-source-projection-check/1", "status": "not-run",
        "scope": "actual Core ordinary library and five original source-static adapter tests and one Opus-authored preparation regression",
        "phases": [], "artifacts": {}, "expected_test_count": 6,
        "expected_test_names": sorted(EXPECTED_TESTS),
        "limits": {"phase_timeout_seconds": PHASE_SECONDS,
                   "outer_total_seconds": "caller-owned 90", "memory_bytes": "caller-owned 4GiB"},
        "residuals": [
            "Cached transitive dependency and lexer/procedural-macro source provenance is unestablished.",
            "Only explicitly selected extern and listed macro artifacts are hashed; the dependency directory is not scanned.",
            "No backend/CLI report-publication, SDK, native relation or target result follows from these tests.",
            "Compiler version is caller-selected by explicit absolute path; this consumer makes no version probe.",
        ],
    }
    fixed: list[Path] = []
    repo: Path | None = None
    before: dict[str, str] | None = None
    result = 1
    try:
        repo = absolute_directory(args.repo)
        rustc = absolute_file(args.rustc)
        manifest = absolute_file(args.dependency_manifest)
        dependencies = absolute_directory(args.dependency_directory)
        externs, macros, expected = read_dependencies(manifest)
        test_source = repo / "crates/mori-core/src/sherlock/source_adapter_tests.rs"
        lib_source = repo / "crates/mori-core/src/lib.rs"
        if not test_source.is_file() or not lib_source.is_file():
            raise ValueError("integrated actual Core library or original adapter test file is absent")
        names = set(re.findall(r"#\[test\]\s*fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(",
                               test_source.read_text(encoding="utf-8")))
        if names != EXPECTED_TESTS:
            raise ValueError("original test-file names differ from the six specified tests")
        fixed = [rustc, manifest, Path(__file__).resolve(), *externs.values(), *macros.values()]
        before = snapshot(repo, fixed)
        report.update({"source_hashes_before": before,
                       "source_rs_count": len(list((repo / "crates/mori-core/src").rglob("*.rs"))),
                       "test_source_count": len(names), "selected_rustc": str(rustc),
                       "dependency_directory": str(dependencies),
                       "selected_externs": {name: str(path) for name, path in sorted(externs.items())},
                       "listed_proc_macros": {name: str(path) for name, path in sorted(macros.items())}})
        differences = {path: {"expected": sha, "observed": before[path]}
                       for path, sha in expected.items() if before[path] != sha}
        if differences:
            report["dependency_hash_mismatches"] = differences
            raise ValueError("selected dependency artifact hash mismatch")
        with tempfile.TemporaryDirectory(prefix="mori-source-projection-") as temporary:
            output = Path(temporary)
            library = output / "libmori_core.rlib"
            harness = output / "source_adapter_harness.rs"
            executable = output / "source_adapter_tests"
            harness.write_text(
                "pub use mori_core::{LANGUAGE, SEMANTICS, Type, parse, validate, Program, Statement, Expr, BinaryOp, Value, UInt, Action};\n"
                "mod actual_adapter_tests {\n"
                " use mori_core::sherlock_source::*;\n"
                " use mori_core::sherlock::{BinaryOp, Expr, ExprRef, Function, Parameter, Type, UnaryOp, Value, PROFILE};\n"
                " use std::sync::Arc;\n"
                " use mori_core::{Program, Statement};\n"
                f" #[path = {rust_string(str(test_source))}]\n"
                " mod original_source_adapter_tests;\n}\n", encoding="utf-8")
            report["artifacts"]["harness_source_sha256"] = digest(harness)
            compile_library = [str(rustc), "--edition=2024", "--crate-name", "mori_core",
                               "--crate-type", "rlib", str(lib_source),
                               "-L", f"dependency={dependencies}", "-o", str(library)]
            for name in sorted(EXTERNS):
                compile_library.extend(["--extern", f"{name}={externs[name]}"])
            compile_harness = [str(rustc), "--edition=2024", "--test", str(harness),
                               "--crate-name", "source_projection_normal_tests",
                               "-C", "strip=debuginfo",
                               "--extern", f"mori_core={library}",
                               "--extern", f"serde_json={externs['serde_json']}",
                               "-L", f"dependency={dependencies}", "-L", f"dependency={output}",
                               "-o", str(executable)]
            commands = [("compile-actual-core-library", compile_library),
                        ("compile-original-test-harness", compile_harness),
                        ("run-six-source-projection-tests", [str(executable), "--test-threads=1"])]
            for phase, command in commands:
                observed = command_result(command, repo, phase)
                report["phases"].append(observed)
                if observed["timed_out"] or observed["returncode"] != 0:
                    report["status"] = "timeout" if observed["timed_out"] else "failed"
                    break
                if phase == "compile-actual-core-library":
                    report["artifacts"]["actual_core_library_sha256"] = digest(library)
                elif phase == "compile-original-test-harness":
                    report["artifacts"]["compiled_harness_sha256"] = digest(executable)
                else:
                    stdout = observed["stdout"]
                    summary = re.search(r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out", stdout)
                    observed_names = set(re.findall(r"^test .*::([^ :]+) \.\.\. ok$", stdout, re.MULTILINE))
                    report["observed_test_names"] = sorted(observed_names)
                    if (not summary or tuple(map(int, summary.groups())) != (6, 0, 0, 0, 0)
                            or observed_names != EXPECTED_TESTS):
                        raise ValueError("runner output does not show exactly the six specified tests passing")
                    report["status"] = "passed"
                    result = 0
    except (OSError, ValueError, TypeError, KeyError) as error:
        report["status"] = "failed"
        report["error"] = f"{type(error).__name__}: {error}"
    finally:
        if before is not None and repo is not None:
            try:
                after = snapshot(repo, fixed)
                report["source_hashes_after"] = after
                changed = sorted(path for path in before.keys() | after.keys()
                                 if before.get(path) != after.get(path))
                report["changed_inputs"] = changed
                if changed:
                    report["status"] = "failed-input-change"
                    result = 1
            except (OSError, ValueError) as error:
                report["status"] = "failed-post-hash"
                report["post_hash_error"] = str(error)
                result = 1
        print(json.dumps(report, indent=2, sort_keys=True))
    return result


if __name__ == "__main__":
    sys.exit(main())
