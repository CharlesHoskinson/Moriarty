#!/usr/bin/env python3
"""SOURCE_ONLY / UNRUN proposal for eleven draft-source host checks.

Exact command classification remains pending after integration. The caller must
retain the prospective 90s/4GiB/swap0/tasks256/16MiB/netoff/read-only-root/fresh
scratch envelope and the exact current Opus-selected masks. This source does not
supply that outer isolation or authorize execution. Cached lexer/transitive
source correspondence remains unestablished.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import tempfile
import time

EXPECTED = {
    "factorial_literal_ast_and_complete_source_provenance",
    "treefold_literal_ast_order_patterns_and_complete_source_provenance",
    "mutual_calls_lists_let_bool_and_all_shallow_list_patterns_remain_actual_ast",
    "arithmetic_precedence_parentheses_and_lexer_comments_prepare_without_evaluation",
    "factorial4_parser_checker_actual_image_and_host_consumers_expect24",
    "treefold_parser_checker_actual_image_and_host_consumers_expect7",
    "unknown_variable_and_arity_are_rejected_before_any_run",
    "declared_return_type_and_duplicate_binders_are_rejected",
    "shadowed_duplicate_function_body_is_still_checked",
    "pinned_choices_cross_sort_eq_and_nonexhaustive_match_are_accepted",
    "uint_literal_overflow_fails_closed_without_wrapping",
}
PHASE_SECONDS = 30
OLD_TEST_COUNT = 86


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", required=True)
    parser.add_argument("--rustc", required=True)
    parser.add_argument("--lexer", required=True)
    parser.add_argument("--lexer-sha256", required=True)
    parser.add_argument("--dependency-directory", required=True)
    args = parser.parse_args()
    report = {"scope": "six original normal and five Opus-authored draft-source boundary tests only", "status": "not-run",
              "phases": [], "expected_names": sorted(EXPECTED),
              "native_authority": 0, "old_host_tests_rerun": False,
              "command_classification": "PENDING separate exact-command review; SOURCE_ONLY / UNRUN",
              "compile_graph": "actual full Sherlock mod.rs and original nested test modules, without filtering or shims",
              "expected_old_tests_filtered_out": OLD_TEST_COUNT,
              "limits": {"phase_timeout_seconds": PHASE_SECONDS,
                         "outer_envelope": "caller-owned 90s/4GiB/swap0/tasks256/16MiB/netoff/read-only-root/fresh-scratch/current-Opus-masks"},
              "cache_provenance": "UNESTABLISHED lexer/transitive cache source correspondence",
              "no_claims": "No Core/1,/2 compiler, CLI, effects, exact work, K/Quint, constraint, native or target result"}
    before = None
    result = 1
    try:
        for value in [args.repo, args.rustc, args.lexer, args.dependency_directory]:
            if not Path(value).is_absolute() or not Path(value).exists():
                raise ValueError("explicit existing absolute paths required")
        repo, rustc, lexer, dependencies = map(Path, [args.repo, args.rustc, args.lexer, args.dependency_directory])
        src = repo / "crates/mori-core/src"
        siblings = {
            "sherlock": src / "sherlock/mod.rs",
            "sherlock_draft": src / "sherlock/draft_source.rs",
            "sherlock_draft_types": src / "sherlock/draft_typecheck.rs",
            "sherlock_draft_tests": src / "sherlock/draft_source_tests.rs",
            "sherlock_draft_image": src / "sherlock/draft_image_bridge.rs",
            "sherlock_draft_image_tests": src / "sherlock/draft_image_bridge_tests.rs",
        }
        fixed = [Path(__file__).resolve(), rustc, lexer, *siblings.values()]
        def snapshot():
            files = set(fixed + [src / "lib.rs", repo / "crates/mori-core/Cargo.toml"]
                        + list((src / "sherlock").rglob("*.rs"))
                        + list((src / "sherlock/specimens").rglob("*.mori")))
            for crate in ["mori-lexer", "mori-span", "mori-diagnostics"]:
                files.add(repo / "crates" / crate / "Cargo.toml")
                files.update((repo / "crates" / crate / "src").rglob("*.rs"))
            return {str(path): digest(path) for path in sorted(files)}
        before = snapshot()
        report["before"] = before
        if digest(lexer) != args.lexer_sha256:
            raise ValueError("lexer digest mismatch")
        names = set()
        test_sources = [siblings["sherlock_draft_tests"], siblings["sherlock_draft_image_tests"],
                        src / "sherlock/draft_typecheck_tests.rs"]
        for path in test_sources:
            names.update(re.findall(r"#\[test\]\s*fn\s+(\w+)\s*\(", path.read_text()))
        if names != EXPECTED:
            raise ValueError("new test membership differs")
        with tempfile.TemporaryDirectory(prefix="mori-draft-source-") as temporary:
            output = Path(temporary)
            harness, binary = output / "harness.rs", output / "draft-source-tests"
            def rust_literal(value):
                return '"' + str(value).replace("\\", "\\\\").replace('"', '\\"') + '"'
            harness.write_text("\n".join(
                f"#[path = {rust_literal(path)}] mod {name};" for name, path in siblings.items()) + "\n")
            report["harness_sha256"] = digest(harness)
            commands = [
                ("compile-actual-modules", [str(rustc), "--edition=2024", "--test", str(harness),
                 "--crate-name", "draft_source_tests", "-C", "strip=debuginfo",
                 "--extern", f"mori_lexer={lexer}",
                 "-L", f"dependency={dependencies}", "-o", str(binary)]),
                ("run-eleven-new-tests", [str(binary), "sherlock_draft", "--test-threads=1"]),
            ]
            for phase, command in commands:
                start = time.monotonic()
                observed = subprocess.run(command, cwd=repo, capture_output=True, text=True, timeout=PHASE_SECONDS)
                report["phases"].append({"phase": phase, "command": command,
                    "elapsed_seconds": time.monotonic() - start, "timeout_seconds": PHASE_SECONDS, "returncode": observed.returncode,
                    "stdout": observed.stdout, "stderr": observed.stderr})
                if observed.returncode:
                    report["status"] = "failed"
                    break
                if phase == "run-eleven-new-tests":
                    names = set(re.findall(r"^test .*::([^ :]+) \.\.\. ok$", observed.stdout, re.M))
                    summary = re.search(r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out", observed.stdout)
                    if names != EXPECTED or not summary or tuple(map(int, summary.groups())) != (11, 0, 0, 0, OLD_TEST_COUNT):
                        raise ValueError("output does not show precisely the eleven expected new checks with all 86 original tests filtered out")
                    report["old_tests_filtered_out"] = int(summary.group(5))
                    report["status"] = "passed"
                    result = 0
    except (OSError, ValueError, subprocess.TimeoutExpired) as error:
        report["status"] = "failed"
        report["error"] = str(error)
    finally:
        if before is not None:
            try:
                after = snapshot()
                report["after"] = after
                if after != before:
                    report["status"] = "failed-input-change"
                    result = 1
            except OSError as error:
                report["status"] = "failed-post-hash"
                report["error"] = str(error)
                result = 1
        print(json.dumps(report, indent=2, sort_keys=True))
    return result


if __name__ == "__main__":
    raise SystemExit(main())
