"""Reproduce pinned s5 observations and check the committed U033 run binding.

This confirms the findings remain reproducible at the pinned commit; success
does not mean the source meets the U031/U033 contracts.
"""
import json
import io
import hashlib
import re
import subprocess
import sys
import types
import unittest

COMMIT = "cfbab29c591e5f4aa634952596aee601fffcde1f"
FIXED_COMMIT = "a824466981168933041fdba85f6a03a94fa1dc93"
RUST_SOURCE_COMMIT = "8ffed57fc0e0362f928a2f106c4ce660b54440a2"
RUST_RUN_COMMIT = "abc64975ddaaf825d6488cd89a61f57a4be08869"
S5_WIRING_COMMIT = "3fabcca1cd34b3fbad2bf8e1a890e8bffba0e1e5"


def committed_rust_run_record():
    def read(path, commit):
        return subprocess.run(
            ["git", "show", f"{commit}:{path}"],
            check=True, capture_output=True, text=True,
        ).stdout

    records = [json.loads(line) for line in read("qa/sherlock/runs.jsonl", RUST_RUN_COMMIT).splitlines()]
    run = next(r for r in records if r["run"] == "20261026T012533Z-U033")
    assert run["id"] == "U033" and run["cmd"] == "qa/holdout/run.sh uireq-033"
    assert run["exit"] == 0 and run["head"] == RUST_SOURCE_COMMIT
    path = "crates/mori-core/src/sherlock/compile_artifact/draft_plans.rs"
    assert read(path, RUST_RUN_COMMIT) == read(path, RUST_SOURCE_COMMIT)
    subprocess.run(["git", "merge-base", "--is-ancestor", RUST_SOURCE_COMMIT, RUST_RUN_COMMIT], check=True)
    trace_path = "qa/sherlock/receipt-traces/20261026T012533Z-U033.log"
    trace = read(trace_path, S5_WIRING_COMMIT)
    trace_hash = hashlib.sha256(trace.encode()).hexdigest()
    assert trace_hash == "a226a8942fde60983d79ad00eb0ee24b6df4fa650a1916b4d8f1d2cbfa387b1f"
    assert trace == "PASS uireq-033 (1 scenario(s), coverage=primary)\n"
    current = read(path, S5_WIRING_COMMIT)
    original = read(path, RUST_SOURCE_COMMIT)
    functions = ("folded_kinds", "block_value", "dispatch_group", "outcome_value", "snapshot_value",
                 "execute_residual", "residualize")
    for name in functions:
        pattern = rf"^(?:pub )?fn {name}\b.*?^}}"
        before = re.search(pattern, original, re.M | re.S)
        after = re.search(pattern, current, re.M | re.S)
        assert before and after and before.group() == after.group(), name
    return {"record_commit": RUST_RUN_COMMIT, "run": run, "residual_source_unchanged": True,
            "trace_copy_commit": S5_WIRING_COMMIT, "trace_copy_path": trace_path,
            "trace_copy_sha256": trace_hash, "trace_inspected": True,
            "unchanged_residual_functions_at_wiring_commit": list(functions),
            "rust_executed_by_this_audit": False}


def committed_export_regression():
    path = "crates/mori-core/src/sherlock/export_plans.rs"

    def read(commit, file=path):
        return subprocess.run(["git", "show", f"{commit}:{file}"], check=True,
                              capture_output=True, text=True).stdout

    assert read(S5_WIRING_COMMIT) == read("b705bfacb")
    assert read(S5_WIRING_COMMIT) != read("3bb404f15")
    cli = read(S5_WIRING_COMMIT, "crates/mori/src/commands/simulate_sherlock.rs")
    assert "host::export_plans::export_plans(" in cli
    return {"wiring_commit": S5_WIRING_COMMIT, "exporter_identical_to": "b705bfacb",
            "repaired_exporter_not_included": "3bb404f15", "cli_uses_old_module": True,
            "counterexample_model": "rustExportClassification.qnt",
            "counterexample_log": "s5-export-regression-counterexample.log",
            "rust_executed_by_this_audit": False}


def source_module(name, commit=COMMIT):
    path = f"models/sherlock/qa/{name}.py"
    source = subprocess.run(
        ["git", "show", f"{commit}:{path}"],
        check=True, capture_output=True, text=True,
    ).stdout
    namespace = {"__name__": name}
    exec(compile(source, f"{commit}:{path}", "exec"), namespace)
    return namespace


def fixed_source_tests():
    # Imports resolve to the exact committed source, never another worktree.
    for name in ("export_plans", "residualize"):
        module = types.ModuleType(name)
        module.__dict__.update(source_module(name, FIXED_COMMIT))
        sys.modules[name] = module
    module = types.ModuleType("test_u031_u033_models")
    module.__dict__.update(source_module("test_u031_u033_models", FIXED_COMMIT))
    suite = unittest.defaultTestLoader.loadTestsFromModule(module)
    output = io.StringIO()
    result = unittest.TextTestRunner(stream=output, verbosity=2).run(suite)
    assert result.wasSuccessful(), output.getvalue()
    return {"tests": result.testsRun, "passed": True, "log": output.getvalue()}


def fixed_bound_cases():
    specialize = source_module("residualize", FIXED_COMMIT)["specialize"]
    # Hand-authored BFS prefixes/frontiers for the declared five-rule graph.
    # Family selection delegates R_call, rather than removing its control edge.
    ordinary_order = ["R_fetch", "R_cond", "R_call", "R_then", "R_else"]
    arith_order = ["R_fetch", "R_cond", "R_then", "R_else"]
    ordinary_frontiers = [
        {"R_fetch"}, {"R_cond", "R_call"}, {"R_call", "R_then", "R_else"},
        {"R_then", "R_else"}, {"R_else"}, set(),
    ]
    arith_frontiers = [
        {"R_fetch"}, {"R_cond"}, {"R_then", "R_else"}, {"R_else"}, set(), set(),
    ]
    cases = 0
    for config in ({"mask": "empty"}, {"mask": "family", "family": "arith"},
                   {"mask": "family", "family": "recursive"}, {"mask": "image", "image": "img-sum"}):
        arith = config.get("family") == "arith"
        for limit in range(6):
            ir = specialize(**config, limit=limit)
            blocks = {block["rule"] for block in ir["blocks"]}
            order = arith_order if arith else ordinary_order
            expected_frontier = (arith_frontiers if arith else ordinary_frontiers)[limit]
            frontier = {f["rule"] for f in ir["genericFallbacks"] if f["reason"] == "residual-size-limit"}
            delegated = {f["target"] for f in ir["genericFallbacks"] if f["reason"] == "target-outside-static-binding"}
            assert blocks == set(order[:limit]), (config, limit, blocks)
            assert frontier == expected_frontier, (config, limit, frontier)
            assert delegated == ({"R_call"} if arith and limit > 0 else set()), (config, limit, delegated)
            assert all(block["charge"] == 1 for block in ir["blocks"])
            cases += 1
    return {"realizations": cases, "limits": [0, 1, 2, 3, 4, 5], "passed": True}


def main():
    residual = source_module("residualize")
    ir = residual["specialize"]("family", family="arith", limit=64)
    fetch = next(block for block in ir["blocks"] if block["rule"] == "R_fetch")
    missing = sorted(set(residual["RULES"]["R_fetch"][2]) - set(fetch["targets"]))
    plans = source_module("export_plans")["prepare"]()
    untyped = [node["nodeId"] for node in plans["witnessPlan"]["nodes"]
               if "typedInputs" not in node or "typedOutputs" not in node]
    fixed_ir = source_module("residualize", FIXED_COMMIT)["specialize"]("family", family="arith", limit=64)
    fixed_fetch = next(block for block in fixed_ir["blocks"] if block["rule"] == "R_fetch")
    fixed_plans = source_module("export_plans", FIXED_COMMIT)["prepare"]()
    assert fixed_fetch["targets"] == ["R_cond", "R_call"]
    assert fixed_fetch["genericTargets"] == ["R_call"]
    assert any(fallback.get("target") == "R_call" for fallback in fixed_ir["genericFallbacks"])
    assert all("typedInputs" in node and "typedOutputs" in node for node in fixed_plans["witnessPlan"]["nodes"])
    observations = {
        "source_commit": COMMIT,
        "residual_dispatch": fetch,
        "missing_dynamic_targets": missing,
        "generic_fallbacks": ir["genericFallbacks"],
        "untyped_witness_nodes": untyped,
        "first_witness_node": plans["witnessPlan"]["nodes"][0],
        "fixed_commit": FIXED_COMMIT,
        "fixed_dispatch": fixed_fetch,
        "fixed_fallbacks": fixed_ir["genericFallbacks"],
        "fixed_first_witness_node": fixed_plans["witnessPlan"]["nodes"][0],
        "fixed_source_unit_tests": fixed_source_tests(),
        "fixed_source_bound_cases": fixed_bound_cases(),
        "committed_rust_run_record": committed_rust_run_record(),
        "committed_export_regression": committed_export_regression(),
    }
    print(json.dumps(observations, indent=2))
    assert missing == ["R_call"] and not ir["genericFallbacks"]
    assert untyped == ["n0", "n1", "n2", "n3", "n4"]


if __name__ == "__main__":
    main()
