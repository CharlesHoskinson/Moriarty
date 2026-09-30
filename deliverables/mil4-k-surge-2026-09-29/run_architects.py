#!/usr/bin/env python3
"""Independent external K semantics consultations; preserve exact model receipts."""

from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
import json
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent

AREAS = {
    "static": "syntax, nominal assets, dimensional types, exact arithmetic, formula formation, static rejection, canonical authority boundaries",
    "transition": "stage and episode transitions, complete effects, conservation, cumulative authority, pending/partial/unknown, recovery, authenticated history",
    "defi": "all eight DeFi family first profiles, oracle/governance/bridge/vault semantics, and optional kernel versus language versus venue boundary",
}

INPUTS = {
    "static": ["concepts/intent-language/DESIGN-MIL2.md", "deliverables/mil4-successor-2026-09-29/DESIGN-MIL4-WORKING.md", "docs/decisions/u0-numeric-profile-decision.md"],
    "transition": ["concepts/intent-language/DESIGN-MIL2.md", "deliverables/mil4-successor-2026-09-29/DESIGN-MIL4-WORKING.md", "docs/MORIARTY-CONSOLIDATED-DESIGN.md"],
    "defi": ["concepts/intent-language/CATEGORY-MAP.md", "deliverables/mil4-successor-2026-09-29/DESIGN-MIL4-WORKING.md", "wiki-llm/kernel-api-recommendation-2026-09-29.md"],
}


def prompt_for(provider: str, area: str) -> str:
    paths = INPUTS[area]
    preamble = "You are one of nine independent architects in a Moriarty K semantics implementation surge.\n"
    preamble += f"Focus: {AREAS[area]}. Model requested: {provider}. Date: 2026-09-29.\n"
    preamble += "Working directory: /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929.\n"
    preamble += "First apply AGENTS.md and the checked-in moriarty-dev:develop skill, and inspect guarded status read-only.\n"
    preamble += "This is a proposal semantics exercise; do not edit product files, run tests, or claim adopted MIL/4, proofs, or Midnight execution.\n"
    preamble += "Read the following files and existing K conventions in experiments/moriarty-language/formal/k/:\n" + "\n".join(paths) + "\n"
    preamble += "Write a substantive independent design review plus concrete K module/rule text for your focus. Every intended constructor needs defined admission or explicit rejection; do not use permissive uninterpreted predicates for critical checks. Distinguish runnable K from pseudocode. Include gaps, counterexamples, and a coverage table tied to exact source sections. Do not inspect other architects' answers.\n"
    if provider == "grok-4.7":
        # The host has already loaded the repo skill and checked guarded status.
        # Keep this consultation to one answer; larger packets induced a cancelled
        # startup-only turn in the first transition attempt.
        preamble = (
            "Independent Grok 4.7 adversarial audit of a specified-only Moriarty MIL/4 K draft. "
            "No tools, browsing, startup narration, or file edits. The host has applied AGENTS.md and "
            "moriarty-dev:develop and guarded status reports SP01.6 blocked with no pending transactions. "
            f"Focus: {AREAS[area]}. Produce the complete final answer immediately. "
            "For each defect cite a K rule/line, give a concrete counterexample, and propose corrected K text. "
            "Separate compile defects, semantic defects, and unimplemented scope. Do not claim proof or adoption.\n\n"
        )
        design = ROOT / "deliverables/mil4-successor-2026-09-29/DESIGN-MIL4-WORKING.md"
        lines = design.read_text().splitlines()
        packet = "### MIL/4 working decision and eight first profiles (excerpt)\n" + "\n".join(lines[:65])
        draft = OUT / f"sol-{area}.k"
        if draft.exists():
            packet += f"\n\n### {draft.relative_to(ROOT)}\n{draft.read_text()}"
        preamble += "Source packet follows. Cite packet paths and do not claim fresh browsing.\n\n" + packet
    return preamble


def run_one(provider: str, area: str) -> dict:
    prompt = prompt_for(provider, area)
    if provider == "claude-opus-5-5":
        cmd = ["claude", "-p", "--model", provider, "--effort", "high", "--permission-mode", "plan", "--no-session-persistence", "--output-format", "json", "--tools", "Read,Bash"]
        input_text = prompt
        stem = "opus55-" + area
    else:
        cmd = ["grok", "--model", "grok-4.7", "--reasoning-effort", "high", "--no-subagents", "--verbatim", "--disable-web-search", "--max-turns", "1", "--output-format", "json", "--single", prompt]
        input_text = None
        stem = "grok47-" + area
    try:
        proc = subprocess.run(cmd, cwd=ROOT, input=input_text, text=True, capture_output=True, timeout=1200)
        raw, err, code = proc.stdout, proc.stderr, proc.returncode
    except subprocess.TimeoutExpired as exc:
        raw = exc.stdout.decode() if isinstance(exc.stdout, bytes) else (exc.stdout or "")
        err = exc.stderr.decode() if isinstance(exc.stderr, bytes) else (exc.stderr or "")
        code = 124
    (OUT / (stem + ".raw.json")).write_text(raw)
    (OUT / (stem + ".stderr.log")).write_text(err)
    try:
        obj = json.loads(raw)
    except json.JSONDecodeError:
        obj = {}
    answer = obj.get("result") if provider == "claude-opus-5-5" else obj.get("text")
    if isinstance(answer, str) and answer:
        (OUT / (stem + ".md")).write_text(answer)
    receipt = {"requested_model": provider, "area": area, "exit_code": code,
               "model_usage": obj.get("modelUsage"), "terminal_reason": obj.get("terminal_reason"),
               "stop_reason": obj.get("stopReason"), "is_error": obj.get("is_error"),
               "answer_file": stem + ".md" if (OUT / (stem + ".md")).exists() else None}
    (OUT / (stem + ".receipt.json")).write_text(json.dumps(receipt, indent=2) + "\n")
    return receipt


def main() -> None:
    provider = sys.argv[1]
    areas = tuple(sys.argv[2:]) or tuple(AREAS)
    with ThreadPoolExecutor(max_workers=3) as pool:
        futures = {pool.submit(run_one, provider, area): area for area in areas}
        for future in as_completed(futures):
            try:
                print(json.dumps(future.result()), flush=True)
            except Exception as exc:
                print(json.dumps({"area": futures[future], "error": str(exc)}), flush=True)


if __name__ == "__main__":
    main()
