"""Reproduce two observations from committed s5 source, without another checkout.

This confirms the findings remain reproducible at the pinned commit; success
does not mean the source meets the U031/U033 contracts.
"""
import json
import io
import subprocess
import sys
import types
import unittest

COMMIT = "cfbab29c591e5f4aa634952596aee601fffcde1f"
FIXED_COMMIT = "a824466981168933041fdba85f6a03a94fa1dc93"


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
    }
    print(json.dumps(observations, indent=2))
    assert missing == ["R_call"] and not ir["genericFallbacks"]
    assert untyped == ["n0", "n1", "n2", "n3", "n4"]


if __name__ == "__main__":
    main()
