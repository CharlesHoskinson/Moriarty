#!/usr/bin/env python3
"""Host wire-format hook adapter for Moriarty development plugin."""

from __future__ import annotations

import json
import os
import sys
from pathlib import Path

# Ensure package is importable
sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

from moriarty_dev.policy import assess
from moriarty_dev.records import load_snapshot, read_actions
from moriarty_dev.store import get_db_path, init_db, make_history_reader


def _last_result(repo: Path) -> str:
    """Read the same program record used by the CLI without launching it."""
    path = repo / "openspec" / "moriarty-completion-program.json"
    try:
        stages = json.loads(path.read_text(encoding="utf-8")).get("reportReconciliation", {}).get("stageAdmission", {}).get("stages", [])
        complete = [row.get("id") for row in stages if row.get("status") == "complete"]
        remaining = [row.get("id") for row in stages if row.get("status") != "complete"]
        return f"Stages complete: {', '.join(complete) if complete else 'none'}" + (f"; remaining stages: {', '.join(remaining)}" if remaining else "")
    except Exception:
        return "No readable program stage record"


def main():
    raw_input = ""
    try:
        if not sys.stdin.isatty():
            raw_input = sys.stdin.read(8192)
    except Exception:
        raw_input = ""

    payload = {}
    if raw_input.strip():
        try:
            payload = json.loads(raw_input)
        except Exception:
            payload = {}

    event_name = sys.argv[1] if len(sys.argv) > 1 else payload.get("hookEventName", "")
    cwd_str = payload.get("cwd") or os.getcwd()
    repo = Path(cwd_str).resolve()

    output = handle_event(event_name, payload, repo)
    out_str = json.dumps(output, indent=2)
    if len(out_str.encode("utf-8")) > 2048:
        out_str = out_str[:2048]
    print(out_str)


def handle_event(event_name: str, payload: dict, repo: Path) -> dict:
    if event_name == "SessionStart":
        return handle_session_start(repo)
    elif event_name == "PreToolUse":
        return handle_pre_tool_use(payload, repo)
    elif event_name == "PostToolUse":
        return {"hookSpecificOutput": {"hookEventName": "PostToolUse"}}
    elif event_name == "Stop":
        return {"hookSpecificOutput": {"hookEventName": "Stop"}}
    return {}


def handle_session_start(repo: Path) -> dict:
    actions_file = repo / ".moriarty-dev" / "actions.json"
    if not actions_file.is_file():
        return {"hookSpecificOutput": {"hookEventName": "SessionStart"}}

    try:
        db_path = get_db_path(repo)
        history_reader = make_history_reader(db_path)
        actions = read_actions(str(repo))
        if not actions:
            return {"hookSpecificOutput": {"hookEventName": "SessionStart"}}
        act = actions[0]
        snap = load_snapshot(str(repo), act["id"], history_reader=history_reader)
        next_act = snap.get("nextActionId") or act["id"]
        status_lines = [
            f"Capability: {act.get('requirement')} {act.get('capability')}",
            f"Last result: {_last_result(repo)}",
            f"Next: {next_act}",
        ]
        if snap.get("sameDefectFailures", 0) >= 2:
            status_lines.insert(2, f"Blocked action: another broad {act.get('kind', 'correction')}")
            status_lines.insert(3, f"Reason: {snap['sameDefectFailures']} failures of required execution behavior")
        return {
            "hookSpecificOutput": {
                "hookEventName": "SessionStart",
                "statusMessage": "\n".join(status_lines),
            }
        }
    except Exception:
        return {"hookSpecificOutput": {"hookEventName": "SessionStart"}}


def handle_pre_tool_use(payload: dict, repo: Path) -> dict:
    actions_file = repo / ".moriarty-dev" / "actions.json"
    if not actions_file.is_file():
        return {
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "allow",
            }
        }

    try:
        tool_name = payload.get("toolName", "")
        tool_input = payload.get("toolInput", {})
        command_str = ""
        if isinstance(tool_input, dict):
            command_str = str(tool_input.get("command", "") or tool_input.get("argv", ""))

        db_path = get_db_path(repo)
        history_reader = make_history_reader(db_path)
        actions = read_actions(str(repo))

        # Check if the tool invocation targets an action or driver execution
        for act in actions:
            snap = load_snapshot(str(repo), act["id"], history_reader=history_reader)
            dec = assess(snap, act)
            if not dec["allow"]:
                # Check if this command attempts to run the denied action
                act_id = act["id"]
                cmd_ref = act.get("commandRef", "")
                if (act_id and act_id in command_str) or (cmd_ref and cmd_ref in command_str):
                    next_id = dec.get("nextActionId") or "a reproducer"
                    reason = f"Repeated unresolved failure. Run {next_id}."
                    return {
                        "hookSpecificOutput": {
                            "hookEventName": "PreToolUse",
                            "permissionDecision": "deny",
                            "permissionDecisionReason": reason,
                        }
                    }
    except Exception:
        pass

    return {
        "hookSpecificOutput": {
            "hookEventName": "PreToolUse",
            "permissionDecision": "allow",
        }
    }


if __name__ == "__main__":
    main()
