#!/usr/bin/env python3
"""Bounded retries for Opus seats that timed out before producing a final answer."""
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
import json
import subprocess

out = Path(__file__).resolve().parent
areas = {
    "static": "sol-static.k",
    "defi": "sol-defi.k",
}

def run(area, filename):
    source = (out / filename).read_text()
    # The earlier tool-enabled requests timed out; this packet is code-focused
    # and the host has already applied repository guidance and guarded status.
    prompt = (
        "Independent Claude Opus 5.5 review of a specified-only Moriarty MIL/4 K draft. "
        "The host has applied AGENTS.md and moriarty-dev:develop; SP01.6 is blocked. "
        "Do not use tools or browse. Produce a final answer immediately in at most 1200 words. "
        "Give at least five concrete semantic defects or omissions, each with a K rule or constructor, "
        "a counterexample, and a precise rule repair. Distinguish compile defects from semantic gaps. "
        "Say what this module can validly claim and what must remain explicitly rejected. "
        "Do not claim proof, adoption or native settlement. Focus area: " + area + ".\n\n"
        + filename + ":\n" + source[:26000]
    )
    proc = subprocess.run(
        ["claude", "-p", "--model", "claude-opus-5-5", "--effort", "medium",
         "--permission-mode", "plan", "--no-session-persistence", "--output-format", "json",
         "--tools", ""],
        input=prompt, text=True, capture_output=True, timeout=600,
        cwd=out.parent.parent,
    )
    stem = f"opus55-{area}.retry2"
    (out / (stem + ".raw.json")).write_text(proc.stdout)
    (out / (stem + ".stderr.log")).write_text(proc.stderr)
    try:
        obj = json.loads(proc.stdout)
    except json.JSONDecodeError:
        obj = {}
    answer = obj.get("result")
    if isinstance(answer, str) and answer:
        (out / (stem + ".md")).write_text(answer)
    receipt = {"requested_model": "claude-opus-5-5", "area": area,
               "exit_code": proc.returncode, "model_usage": obj.get("modelUsage"),
               "terminal_reason": obj.get("terminal_reason"), "is_error": obj.get("is_error"),
               "answer_file": stem + ".md" if (out / (stem + ".md")).exists() else None}
    (out / (stem + ".receipt.json")).write_text(json.dumps(receipt, indent=2) + "\n")
    return receipt

with ThreadPoolExecutor(max_workers=2) as pool:
    futures = [pool.submit(run, area, filename) for area, filename in areas.items()]
    for future in as_completed(futures):
        print(json.dumps(future.result()), flush=True)
