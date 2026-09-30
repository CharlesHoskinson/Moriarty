#!/usr/bin/env python3
"""Run six independent read-only architect consultations with exact model requests."""

import concurrent.futures
import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
THEMES = {"abi": "PROMPT-ABI.md", "ops": "PROMPT-OPS.md", "trust": "PROMPT-TRUST.md"}


def run_one(provider: str, theme: str) -> dict:
    prompt_path = OUT / THEMES[theme]
    prompt = prompt_path.read_text(encoding="utf-8")
    if provider == "grok":
        packet_files = [
            ROOT / "deliverables/near-kernel-scope-2026-09-29/PROPOSAL.md",
            ROOT / "deliverables/hyperliquid-kernel-comparison-2026-09-29/COMPARISON.md",
            ROOT / "deliverables/aeon-kernel-experiments-2026-09-29/authority/REPORT.md",
            ROOT / "deliverables/aeon-kernel-experiments-2026-09-29/bridge/RESULT.md",
            ROOT / "deliverables/aeon-kernel-experiments-2026-09-29/signing/RESULT.md",
        ]
        source_packet = "\n\n".join(
            f"### {p.relative_to(ROOT)}\n{p.read_text(encoding='utf-8')}"
            for p in packet_files
        )
        prompt = prompt + "\n\nRead the following complete local source packet. The Moriarty lead has already run guarded status; your role is an independent, tool-free architecture consultation. Cite the linked primary URLs in the packet and identify any further primary literature you know precisely. Do not claim to have browsed beyond this packet.\n\n" + source_packet
        cmd = [
            "grok", "--model", "grok-4.7", "--reasoning-effort", "high",
            "--no-subagents", "--verbatim", "--disable-web-search", "--max-turns", "3",
            "--output-format", "json", "--single", prompt,
        ]
    else:
        cmd = [
            "claude", "-p", "--model", "claude-opus-5", "--effort", "high",
            "--permission-mode", "plan", "--no-session-persistence",
            "--output-format", "json", "--tools", "Read,WebSearch,WebFetch,Bash",
        ]
    try:
        proc = subprocess.run(cmd, cwd=ROOT, input=prompt if provider == "opus" else None,
                              text=True, capture_output=True, timeout=900)
        raw = proc.stdout
        err = proc.stderr
        code = proc.returncode
    except subprocess.TimeoutExpired as exc:
        raw = exc.stdout.decode() if isinstance(exc.stdout, bytes) else (exc.stdout or "")
        err = exc.stderr.decode() if isinstance(exc.stderr, bytes) else (exc.stderr or "")
        code = 124
    stem = f"{provider}-{theme}"
    (OUT / f"{stem}.raw.json").write_text(raw, encoding="utf-8")
    (OUT / f"{stem}.stderr.log").write_text(err, encoding="utf-8")
    parsed = None
    try:
        parsed = json.loads(raw)
    except json.JSONDecodeError:
        pass
    if isinstance(parsed, dict):
        answer = parsed.get("text") if provider == "grok" else parsed.get("result")
        if isinstance(answer, str):
            (OUT / f"{stem}.md").write_text(answer, encoding="utf-8")
    receipt = {
        "requested_model": "grok-4.7" if provider == "grok" else "claude-opus-5",
        "provider": provider,
        "theme": theme,
        "exit_code": code,
        "returned_model_usage": parsed.get("modelUsage") if isinstance(parsed, dict) else None,
        "terminal_reason": parsed.get("terminal_reason") if isinstance(parsed, dict) else None,
        "stop_reason": parsed.get("stopReason") if isinstance(parsed, dict) else None,
        "session_id": parsed.get("sessionId", parsed.get("session_id")) if isinstance(parsed, dict) else None,
        "cost_usd": parsed.get("total_cost_usd") if isinstance(parsed, dict) else None,
        "answer_file": f"{stem}.md" if (OUT / f"{stem}.md").exists() else None,
    }
    (OUT / f"{stem}.receipt.json").write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8")
    return receipt


def main() -> int:
    requested = tuple(sys.argv[1:]) or ("grok", "opus")
    tasks = [(p, t) for p in ("grok", "opus") for t in THEMES
             if (p in requested or f"{p}:{t}" in requested)]
    with concurrent.futures.ThreadPoolExecutor(max_workers=3) as pool:
        futures = {pool.submit(run_one, p, t): (p, t) for p, t in tasks}
        for future in concurrent.futures.as_completed(futures):
            p, t = futures[future]
            try:
                receipt = future.result()
                print(json.dumps(receipt), flush=True)
            except Exception as exc:
                print(json.dumps({"provider": p, "theme": t, "error": str(exc)}), flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
