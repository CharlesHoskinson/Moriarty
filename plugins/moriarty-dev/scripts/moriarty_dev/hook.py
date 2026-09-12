#!/usr/bin/env python3
"""Read-only lifecycle adapter; the guarded CLI remains the launch gate."""
from __future__ import annotations

import json
import os
from pathlib import Path
import re
import shlex
import signal
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
from moriarty_dev.hook_protocol import (
    diagnostic as _diagnostic,
    output as _output,
    read_payload,
    serialize as _bounded_json,
    supported_event,
)
from moriarty_dev.policy import assess
from moriarty_dev.records import load_snapshot, read_actions
from moriarty_dev.store import get_db_path, get_undelivered_txs, make_history_reader
from moriarty_dev.notifications import notification_line


def _last_result(repo):
    try:
        data = json.loads((repo / "openspec/moriarty-completion-program.json").read_text(encoding="utf-8"))
        stages = data.get("reportReconciliation", {}).get("stageAdmission", {}).get("stages", [])
        done = [row["id"] for row in stages if row.get("status") == "complete"]
        return "Stages complete: " + (", ".join(done) if done else "none")
    except (OSError, ValueError, KeyError, TypeError, AttributeError):
        return "No readable program stage record"


def _pending(repo):
    rows = get_undelivered_txs(get_db_path(repo), str(repo), timeout=0.05, limit=5)
    if not rows:
        return ""
    text = "Pending public observations: post dedicated notification message, then use deliver.\n"
    text += "\n".join(notification_line(tx) for tx in rows[:4])
    if len(rows) > 4:
        text += "\nMore pending IDs: use report --json."
    return text


def main():
    event = sys.argv[1] if len(sys.argv) > 1 else ""
    timer_supported = hasattr(signal, "setitimer")
    previous_handler = None

    class HookDeadline(BaseException):
        pass

    def deadline(_signum, _frame):
        raise HookDeadline()

    try:
        if timer_supported:
            previous_handler = signal.signal(signal.SIGALRM, deadline)
            signal.setitimer(signal.ITIMER_REAL, 0.75)
        payload = read_payload(sys.stdin)
        event = event or payload.get("hook_event_name", payload.get("hookEventName", ""))
        if not supported_event(event):
            raise ValueError("unsupported hook event")
        repo = Path(payload.get("cwd", os.getcwd())).resolve()
        for ancestor in (repo, *repo.parents):
            if (ancestor / ".moriarty-dev/actions.json").is_file():
                repo = ancestor
                break
            if (ancestor / ".git").exists():
                break
        response = handle_event(event, payload, repo)
    except (HookDeadline, Exception):
        response = _diagnostic(event)
    finally:
        if timer_supported and previous_handler is not None:
            signal.setitimer(signal.ITIMER_REAL, 0)
            signal.signal(signal.SIGALRM, previous_handler)
    print(_bounded_json(response))


def handle_event(event_name, payload, repo):
    if not supported_event(event_name):
        return _diagnostic(event_name)
    if not (repo / ".moriarty-dev/actions.json").is_file():
        return _output(event_name)
    if event_name == "SessionStart":
        return handle_session_start(repo)
    if event_name == "PreToolUse":
        return handle_pre_tool_use(payload, repo)
    if event_name == "PostToolUse":
        try:
            return _output(event_name, additionalContext=_pending(repo))
        except Exception:
            return _diagnostic(event_name)
    return _output(event_name)  # Stop never blocks or invokes another action.


def handle_session_start(repo):
    try:
        actions = read_actions(str(repo))
        if not actions:
            return _output("SessionStart")
        action = next((row for row in actions if row.get("kind") != "report"), actions[0])
        snapshot = load_snapshot(str(repo), action["id"], history_reader=make_history_reader(get_db_path(repo), timeout=0.05))
        lines = [f"Capability: {action.get('requirement')} {action.get('capability')}",
                 f"Last result: {_last_result(repo)}",
                 f"Next: {snapshot.get('nextActionId') or action['id']}"]
        failures = snapshot.get("sameDefectFailures")
        if isinstance(failures, int) and failures >= 2:
            lines.append("Blocked: another broad correction; reproduce the unresolved defect.")
        elif failures is None:
            lines.append("History unresolved; inspect CLI status before dispatch.")
        pending = _pending(repo)
        if pending:
            lines.append(pending)
        return _output("SessionStart", additionalContext="\n".join(lines))
    except Exception:
        return _diagnostic("SessionStart")


def _registered_argv(repo, action):
    ref = action.get("commandRef", "")
    if "#" not in ref:
        return None
    filename, key = ref.split("#", 1)
    path = (repo / filename).resolve()
    if not path.is_relative_to(repo.resolve()):
        return None
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
        argv = data.get("commands", {}).get(key, {}).get("argv")
        return argv if isinstance(argv, list) and all(isinstance(value, str) for value in argv) else None
    except (OSError, ValueError, AttributeError):
        return None


def _dispatch_request(tokens):
    """Recognize simple Python CLI run forms, never arbitrary action mentions.

    Shell composition, wrappers and interpreter code strings are deliberately
    outside this recognizer. It is a workflow guard, not a shell parser.
    """
    if not tokens:
        return None
    args = list(tokens)
    if re.fullmatch(r"python(?:\d+(?:\.\d+)*)?", Path(args[0]).name):
        args.pop(0)
        while args and args[0] in ("-B", "-u"):
            args.pop(0)
        if args[:2] == ["-m", "moriarty_dev.cli"]:
            args = args[2:]
        elif args and Path(args[0]).name == "cli.py":
            args.pop(0)
        else:
            return None
    elif Path(args[0]).name == "cli.py":
        args.pop(0)
    else:
        return None
    target_repo = None
    while args:
        if args[0] == "--json":
            args.pop(0)
        elif args[0] == "--repo" and len(args) > 1:
            target_repo = args[1]
            args = args[2:]
        elif args[0].startswith("--repo="):
            target_repo = args.pop(0).split("=", 1)[1]
        else:
            break
    if not args or args.pop(0) != "run":
        return None
    action_id = None
    while args:
        flag = args.pop(0)
        if flag == "--json":
            continue
        if flag == "--action" and args:
            action_id = args.pop(0)
        elif flag.startswith("--action="):
            action_id = flag.split("=", 1)[1]
        else:
            return None
    return (action_id, target_repo) if action_id else None


def handle_pre_tool_use(payload, repo):
    event = "PreToolUse"
    name = payload.get("tool_name", payload.get("toolName", ""))
    if not isinstance(name, str):
        return _diagnostic(event)
    if name not in ("Bash", "bash", "shell", "exec_command", "execute_command"):
        return _output(event)
    try:
        inp = payload.get("tool_input", payload.get("toolInput", {}))
        if not isinstance(inp, dict):
            return _diagnostic(event)
        value = next((inp[key] for key in ("command", "cmd", "argv") if key in inp), "")
        tokens = shlex.split(value) if isinstance(value, str) else value
        if not isinstance(tokens, list) or not all(isinstance(token, str) for token in tokens):
            return _diagnostic(event)
        request = _dispatch_request(tokens)
        if request and request[1] is not None:
            repo = (Path(payload.get("cwd", repo)) / request[1]).resolve()
        actions = read_actions(str(repo))
        for action in actions:
            argv = _registered_argv(repo, action)
            if argv and tokens == argv:
                return _output(event, permissionDecision="deny",
                               permissionDecisionReason=f"Registered dispatch requires guarded CLI run --action {action['id']}.")
            if not request or action["id"] != request[0]:
                continue
            snapshot = load_snapshot(str(repo), action["id"], history_reader=make_history_reader(get_db_path(repo), timeout=0.05))
            decision = assess(snapshot, action)
            if not decision["allow"]:
                return _output(event, permissionDecision="deny",
                               permissionDecisionReason=f"Repeated unresolved failure or unavailable admission ({decision.get('reasonCode')}). Run {decision.get('nextActionId') or 'CLI status'}.")
    except Exception:
        return _diagnostic(event)
    return _output(event)


if __name__ == "__main__":
    main()
