"""Reproduce two observations from committed s5 source, without another checkout.

This confirms the findings remain reproducible at the pinned commit; success
does not mean the source meets the U031/U033 contracts.
"""
import json
import subprocess

COMMIT = "cfbab29c591e5f4aa634952596aee601fffcde1f"


def source_module(name):
    path = f"models/sherlock/qa/{name}.py"
    source = subprocess.run(
        ["git", "show", f"{COMMIT}:{path}"],
        check=True, capture_output=True, text=True,
    ).stdout
    namespace = {}
    exec(compile(source, f"{COMMIT}:{path}", "exec"), namespace)
    return namespace


def main():
    residual = source_module("residualize")
    ir = residual["specialize"]("family", family="arith", limit=64)
    fetch = next(block for block in ir["blocks"] if block["rule"] == "R_fetch")
    missing = sorted(set(residual["RULES"]["R_fetch"][2]) - set(fetch["targets"]))
    plans = source_module("export_plans")["prepare"]()
    untyped = [node["nodeId"] for node in plans["witnessPlan"]["nodes"]
               if "typedInputs" not in node or "typedOutputs" not in node]
    observations = {
        "source_commit": COMMIT,
        "residual_dispatch": fetch,
        "missing_dynamic_targets": missing,
        "generic_fallbacks": ir["genericFallbacks"],
        "untyped_witness_nodes": untyped,
        "first_witness_node": plans["witnessPlan"]["nodes"][0],
    }
    print(json.dumps(observations, indent=2))
    assert missing == ["R_call"] and not ir["genericFallbacks"]
    assert untyped == ["n0", "n1", "n2", "n3", "n4"]


if __name__ == "__main__":
    main()
