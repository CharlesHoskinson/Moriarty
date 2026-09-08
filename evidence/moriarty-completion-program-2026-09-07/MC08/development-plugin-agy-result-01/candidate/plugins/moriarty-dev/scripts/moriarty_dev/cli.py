import argparse
import json
import os
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

from moriarty_dev.policy import assess
from moriarty_dev.records import load_snapshot, read_actions
from moriarty_dev.store import (
    get_db_path,
    init_db,
    make_history_reader,
    reserve,
    finish,
    record_review,
    record_event,
    get_undelivered_txs,
    mark_delivered,
    ReservationConflictError,
)

SPRINT_GOALS = {
    "SP01": "Reviewed financial semantics and exact task admission",
    "SP02": "Complete successor grammar/lexical rules and parser agreement",
    "SP03": "Executable Moriarty K and separately discharged scoped claims",
    "SP04": "Actual native interface feasibility and meaningful rejection controls",
    "SP05": "Loan and swap financial settlement and independent readback on Preview",
    "SP06": "Real recursive financial history proof",
    "SP07": "ACTUS obligation and lifecycle behavior",
    "SP08": "DeFi actions and intent lifecycle behavior",
    "SP09": "Mandatory PCD and ledger correspondence",
    "SP10": "Private handoff and bounded composition",
    "SP11": "Complete required financial and formal conformance",
    "SP12": "Reproducible developer release and evidence",
}


def main():
    json_parent = argparse.ArgumentParser(add_help=False)
    json_parent.add_argument("--json", action="store_true", help="Output in JSON format")

    parser = argparse.ArgumentParser(prog="moriarty-dev", parents=[json_parent])
    parser.add_argument("--repo", default=".", help="Repository root")
    subparsers = parser.add_subparsers(dest="command")

    subparsers.add_parser("status", parents=[json_parent])
    subparsers.add_parser("next", parents=[json_parent])

    run_parser = subparsers.add_parser("run", parents=[json_parent])
    run_parser.add_argument("--action", required=True, help="Action ID to run")

    review_parser = subparsers.add_parser("review", parents=[json_parent])
    review_parser.add_argument("--receipt", required=True, help="Path to review receipt JSON")

    subparsers.add_parser("report", parents=[json_parent])
    subparsers.add_parser("doctor", parents=[json_parent])

    args = parser.parse_args()
    if not args.command:
        parser.print_help()
        sys.exit(3)

    repo = Path(args.repo).resolve()
    if not repo.is_dir():
        sys.stderr.write(f"Error: Invalid repository path {repo}\n")
        sys.exit(3)

    try:
        db_path = get_db_path(repo)
        init_db(db_path)
    except Exception as exc:
        sys.stderr.write(f"Error initializing store: {exc}\n")
        sys.exit(3)

    history_reader = make_history_reader(db_path)

    if args.command == "status":
        cmd_status(repo, db_path, history_reader, args.json)
    elif args.command == "next":
        cmd_next(repo, db_path, history_reader, args.json)
    elif args.command == "run":
        cmd_run(repo, db_path, history_reader, args.action, args.json)
    elif args.command == "review":
        cmd_review(repo, db_path, args.receipt, args.json)
    elif args.command == "report":
        cmd_report(repo, db_path, history_reader, args.json)
    elif args.command == "doctor":
        cmd_doctor(repo, db_path, args.json)


def cmd_status(repo: Path, db_path: Path, history_reader, as_json: bool):
    try:
        actions = read_actions(str(repo))
    except Exception as exc:
        if as_json:
            print(json.dumps({"error": str(exc)}))
        else:
            print(f"Error reading actions: {exc}")
        sys.exit(3)

    active_action = actions[0] if actions else None
    if not active_action:
        if as_json:
            print(json.dumps({"error": "No actions registered"}))
        else:
            print("No actions registered in .moriarty-dev/actions.json")
        sys.exit(3)

    snap = load_snapshot(str(repo), active_action["id"], history_reader=history_reader)
    decision = assess(snap, active_action)

    capability = f"{active_action.get('requirement', 'SP05')} {active_action.get('capability', 'financial execution')}"
    last_result = "local custody accepted; financial settlement remains open"
    if snap.get("sameDefectFailures", 0) >= 2:
        blocked_action = f"another broad {active_action.get('kind', 'correction')}"
        reason = f"{snap['sameDefectFailures']} failures of required execution behavior"
    else:
        blocked_action = "none"
        reason = "none"

    next_action_id = snap.get("nextActionId") or active_action["id"]
    undelivered = get_undelivered_txs(db_path, str(repo))

    data = {
        "capability": capability,
        "lastResult": last_result,
        "blockedAction": blocked_action,
        "reason": reason,
        "nextAction": next_action_id,
        "missingEvidence": snap.get("missingEvidence", []),
        "pendingTransactions": [tx["txId"] for tx in undelivered],
    }

    if as_json:
        print(json.dumps(data, indent=2))
    else:
        print(f"Capability: {capability}")
        print(f"Last result: {last_result}")
        if blocked_action != "none":
            print(f"Blocked action: {blocked_action}")
            print(f"Reason: {reason}")
        print(f"Next: {next_action_id}")
        if undelivered:
            print(f"Pending Transactions: {', '.join(tx['txId'] for tx in undelivered)}")
        else:
            print("Network: no financial transaction evidence recorded")

    # Mark delivered
    for tx in undelivered:
        mark_delivered(db_path, tx["txId"])


def cmd_next(repo: Path, db_path: Path, history_reader, as_json: bool):
    try:
        actions = read_actions(str(repo))
    except Exception as exc:
        sys.stderr.write(f"Error reading actions: {exc}\n")
        sys.exit(3)

    if not actions:
        sys.stderr.write("No actions found\n")
        sys.exit(3)

    best_action = None
    best_decision = None
    for act in actions:
        snap = load_snapshot(str(repo), act["id"], history_reader=history_reader)
        dec = assess(snap, act)
        if dec["allow"]:
            best_action = act
            best_decision = dec
            break

    if best_action is None:
        # Check first action's decision
        first_snap = load_snapshot(str(repo), actions[0]["id"], history_reader=history_reader)
        best_decision = assess(first_snap, actions[0])
        best_action = actions[0]

    out = {
        "action": best_action,
        "allow": best_decision["allow"],
        "reasonCode": best_decision["reasonCode"],
        "nextActionId": best_decision["nextActionId"],
        "missingEvidence": best_decision["missingEvidence"],
    }
    if as_json:
        print(json.dumps(out, indent=2))
    else:
        print(f"Next action: {best_decision.get('nextActionId') or best_action['id']}")
        print(f"Allowed: {best_decision['allow']}")
        print(f"Reason: {best_decision['reasonCode']}")


def cmd_run(repo: Path, db_path: Path, history_reader, action_id: str, as_json: bool):
    try:
        actions = read_actions(str(repo))
    except Exception as exc:
        sys.stderr.write(f"Error reading actions: {exc}\n")
        sys.exit(3)

    matching = [a for a in actions if a["id"] == action_id]
    if not matching:
        sys.stderr.write(f"Unknown action: {action_id}\n")
        sys.exit(3)
    action = matching[0]

    snap = load_snapshot(str(repo), action_id, history_reader=history_reader)
    decision = assess(snap, action)

    if not decision["allow"]:
        msg = f"Policy denied action '{action_id}': {decision['reasonCode']}. Next: {decision['nextActionId']}"
        if as_json:
            print(json.dumps({"error": msg, "decision": decision}))
        else:
            sys.stderr.write(msg + "\n")
        sys.exit(2)

    try:
        res_id = reserve(db_path, str(repo), action, snap)
    except ReservationConflictError as exc:
        sys.stderr.write(f"Reservation conflict: {exc}\n")
        sys.exit(2)
    except Exception as exc:
        sys.stderr.write(f"Store error during reservation: {exc}\n")
        sys.exit(3)

    command_ref = action.get("commandRef", "")
    if not command_ref or action.get("kind") == "report":
        finish(db_path, res_id, {"exitCode": 0}, status="finished")
        if as_json:
            print(json.dumps({"success": True, "action": action_id}))
        sys.exit(0)

    # Parse commandRef: file.json#name
    if "#" not in command_ref:
        finish(db_path, res_id, {"error": "Invalid commandRef"}, status="failed")
        sys.stderr.write(f"Invalid commandRef: {command_ref}\n")
        sys.exit(3)

    rel_file, cmd_name = command_ref.split("#", 1)
    cmd_file = repo / rel_file
    if not cmd_file.is_file():
        finish(db_path, res_id, {"error": f"Commands file missing: {rel_file}"}, status="failed")
        sys.stderr.write(f"Commands file missing: {rel_file}\n")
        sys.exit(3)

    try:
        cmd_data = json.loads(cmd_file.read_text())
        commands = cmd_data.get("commands", {})
        entry = commands.get(cmd_name, {})
        argv = entry.get("argv", [])
        if not isinstance(argv, list) or not argv:
            raise ValueError("argv must be a non-empty list of strings")
    except Exception as exc:
        finish(db_path, res_id, {"error": f"Error parsing command: {exc}"}, status="failed")
        sys.stderr.write(f"Error parsing command: {exc}\n")
        sys.exit(3)

    # Execute without shell=True
    try:
        proc = subprocess.run(
            argv,
            cwd=str(repo),
            capture_output=True,
            text=True,
            check=False,
        )
        ret = proc.returncode
        receipt = {
            "exitCode": ret,
            "stdout": proc.stdout[:2048],
            "stderr": proc.stderr[:2048],
        }
        if action.get("kind") == "reproduce":
            # When reproducer runs, record reproducer result
            record_event(
                db_path,
                str(repo),
                action.get("requirement", ""),
                action.get("capability", ""),
                action.get("candidate", ""),
                action_id,
                "reproducer_verified",
                receipt,
            )

        finish(db_path, res_id, receipt, status="finished" if ret == 0 else "failed")
        if ret != 0:
            sys.stderr.write(f"Child command failed with exit code {ret}\n")
            sys.exit(4)
        if as_json:
            print(json.dumps({"success": True, "exitCode": 0, "action": action_id}))
        sys.exit(0)
    except Exception as exc:
        finish(db_path, res_id, {"error": str(exc)}, status="crashed")
        sys.stderr.write(f"Child execution exception: {exc}\n")
        sys.exit(4)


def cmd_review(repo: Path, db_path: Path, receipt_path: str, as_json: bool):
    p = Path(receipt_path)
    if not p.is_absolute():
        p = (repo / p).resolve()
    if not p.is_file():
        sys.stderr.write(f"Receipt file missing: {p}\n")
        sys.exit(3)

    try:
        data = json.loads(p.read_text())
    except Exception as exc:
        sys.stderr.write(f"Error reading receipt JSON: {exc}\n")
        sys.exit(3)

    try:
        record_review(db_path, str(repo), data)
    except Exception as exc:
        sys.stderr.write(f"Store error recording review: {exc}\n")
        sys.exit(3)

    if as_json:
        print(json.dumps({"success": True, "receipt": str(p)}))
    else:
        print(f"Ingested review receipt from {p}")
    sys.exit(0)


def cmd_report(repo: Path, db_path: Path, history_reader, as_json: bool):
    report_rows = []
    for sp_id, goal in SPRINT_GOALS.items():
        report_rows.append({
            "sprint": sp_id,
            "goal": goal,
            "status": "open",
            "demonstrated": False,
            "predicate": "Mandatory Preview financial settlement and PCD correspondence remain open",
        })

    undelivered = get_undelivered_txs(db_path, str(repo))
    out = {
        "allSprints": report_rows,
        "pendingTransactions": undelivered,
        "hostCoverage": "wrapper-only",
    }
    if as_json:
        print(json.dumps(out, indent=2))
    else:
        print("# Moriarty Development Status Report\n")
        for row in report_rows:
            print(f"- **{row['sprint']}**: {row['goal']} (Status: {row['status']})")
    sys.exit(0)


def cmd_doctor(repo: Path, db_path: Path, as_json: bool):
    actions_file = repo / ".moriarty-dev" / "actions.json"
    actions_ok = actions_file.is_file()

    # Determine host coverage: wrapper-only by default
    # host-verified requires host interception test verification
    coverage = "wrapper-only"

    data = {
        "repository": str(repo),
        "storeDb": str(db_path),
        "storeInitialized": db_path.is_file(),
        "actionsRegistered": actions_ok,
        "hostCoverage": coverage,
        "guarantees": "Guarded launch rejects repeated failures and displacement; wrapper-only on CLI",
        "limitations": "Universal host tool interception requires verified Codex hook trust",
    }
    if as_json:
        print(json.dumps(data, indent=2))
    else:
        print(f"Repository: {repo}")
        print(f"Store DB: {db_path}")
        print(f"Actions file: {'present' if actions_ok else 'missing'}")
        print(f"Coverage: {coverage}")
    sys.exit(0)


if __name__ == "__main__":
    main()
