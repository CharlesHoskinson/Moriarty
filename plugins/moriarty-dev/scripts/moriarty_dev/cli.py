import argparse
import json
import os
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import time

from moriarty_dev.policy import assess
from moriarty_dev.runner import load_runner, execute
from moriarty_dev.notifications import observe_delivery, notification_line
from moriarty_dev.store import has_other_primary
from moriarty_dev.records import load_snapshot, read_actions
from moriarty_dev.store import (
    get_db_path,
    init_db,
    make_history_reader,
    reserve,
    finish,
    record_review,
    record_event,
    record_admin_interval,
    get_undelivered_txs,
    enqueue_tx,
    update_tx_status,
    VALID_TX_STATUSES,
    mark_delivered,
    bootstrap_store,
    ReservationConflictError,
    StoreError,
)

def _derive_last_result(repo: Path) -> str:
    program_path = repo / "openspec/moriarty-completion-program.json"
    if program_path.is_file():
        try:
            data = json.loads(program_path.read_text(encoding="utf-8"))
            stages = data.get("reportReconciliation", {}).get("stageAdmission", {}).get("stages", [])
            completed = [s["id"] for s in stages if s.get("status") == "complete"]
            open_stages = [s["id"] for s in stages if s.get("status") != "complete"]
            if completed or open_stages:
                result = f"Stages complete: {', '.join(completed) if completed else 'none'}"
                if open_stages:
                    result += f"; remaining stages: {', '.join(open_stages)}"
                return result
        except Exception:
            pass
    return "No readable program stage record"


def _derive_sprint_reports(repo: Path) -> list[dict]:
    sprints_file = repo / "openspec/sprints/sprints.json"
    program_file = repo / "openspec/moriarty-completion-program.json"
    stage_status_map = {}
    if program_file.is_file():
        try:
            pdata = json.loads(program_file.read_text(encoding="utf-8"))
            for s in pdata.get("reportReconciliation", {}).get("stageAdmission", {}).get("stages", []):
                stage_status_map[s["id"]] = s.get("status", "open")
        except Exception:
            pass

    sprints_data = []
    if sprints_file.is_file():
        try:
            sdata = json.loads(sprints_file.read_text(encoding="utf-8"))
            sprints_data = sdata.get("sprints", [])
        except Exception:
            pass

    rows = []
    for sp in sprints_data:
        sp_id = sp.get("id")
        title = sp.get("title")
        stages = sp.get("stages", [])
        all_complete = stages and all(stage_status_map.get(st) == "complete" for st in stages)
        missing_stages = [stage for stage in stages if stage_status_map.get(stage) != "complete"]
        required = sp.get("completionRequires", [])
        if all_complete and not required:
            predicate = "All registered stages are complete"
        else:
            pieces = []
            if missing_stages:
                pieces.append("stages: " + ", ".join(missing_stages))
            if required:
                pieces.append("completion requirements: " + ", ".join(str(x) for x in required))
            predicate = "; ".join(pieces) or "No completion record"
        rows.append({
            "sprint": sp_id,
            "goal": title,
            "status": "complete" if all_complete else "open",
            "demonstrated": bool(all_complete),
            "predicate": predicate,
        })
    return rows


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
    review_parser.add_argument("receipt_pos", nargs="?", default=None, help="Path to review receipt JSON")
    review_parser.add_argument("--receipt", default=None, help="Path to review receipt JSON")
    review_parser.add_argument("--action", help="Action ID to bind review to")

    subparsers.add_parser("report", parents=[json_parent])
    subparsers.add_parser("doctor", parents=[json_parent])
    subparsers.add_parser("bootstrap", parents=[json_parent])

    deliver_parser = subparsers.add_parser("deliver", parents=[json_parent])
    deliver_parser.add_argument("--tx-id", required=True, help="Transaction ID to mark delivered")
    deliver_parser.add_argument("--acknowledgement", help="Unsupported legacy caller receipt; delivery requires host observation")
    notify_parser = subparsers.add_parser("notify", parents=[json_parent])
    notify_parser.add_argument("--tx-id", required=True, help="Selected public Preview transaction ID")
    notify_parser.add_argument("--status", required=True, choices=sorted(VALID_TX_STATUSES), help="Reported observation, not product acceptance")

    args = parser.parse_args()
    if not args.command:
        parser.print_help()
        sys.exit(3)

    repo = Path(args.repo).resolve()
    if not repo.is_dir():
        sys.stderr.write(f"Error: Invalid repository path {repo}\n")
        sys.exit(3)

    db_path = get_db_path(repo)
    history_reader = make_history_reader(db_path)

    if args.command == "status":
        cmd_status(repo, db_path, history_reader, args.json)
    elif args.command == "next":
        cmd_next(repo, db_path, history_reader, args.json)
    elif args.command == "run":
        cmd_run(repo, db_path, history_reader, args.action, args.json)
    elif args.command == "review":
        receipt_arg = args.receipt or args.receipt_pos
        if not receipt_arg:
            sys.stderr.write("Review rejected: receipt file required (specify --receipt or positional argument)\n")
            sys.exit(3)
        cmd_review(repo, db_path, receipt_arg, args.json, getattr(args, "action", None))
    elif args.command == "report":
        cmd_report(repo, db_path, history_reader, args.json)
    elif args.command == "doctor":
        cmd_doctor(repo, db_path, args.json)
    elif args.command == "bootstrap":
        cmd_bootstrap(repo, db_path, args.json)
    elif args.command == "deliver":
        cmd_deliver(repo, db_path, args.tx_id, args.acknowledgement, args.json)
    elif args.command == "notify":
        cmd_notify(repo, db_path, args.tx_id, args.status, args.json)


def cmd_bootstrap(repo: Path, db_path: Path, as_json: bool):
    try:
        t0 = time.time()
        bootstrap_store(db_path, str(repo))
        record_admin_interval(db_path, str(repo), t0, time.time(), "bootstrap")
        if as_json:
            print(json.dumps({"success": True, "bootstrapped": True, "db": str(db_path)}))
        else:
            print(f"Bootstrapped operational store at {db_path}")
        sys.exit(0)
    except Exception as exc:
        sys.stderr.write(f"Bootstrap error: {exc}\n")
        sys.exit(3)


def cmd_notify(repo: Path, db_path: Path, tx_id: str, status: str, as_json: bool):
    try:
        if status == "submitted":
            enqueue_tx(db_path, str(repo), tx_id, status, {"network": "preview"})
        else:
            update_tx_status(db_path, tx_id, status)
        pending = get_undelivered_txs(db_path, str(repo))
        if as_json:
            print(json.dumps({"observationRecorded": True, "pendingTransactions": pending,
                              "scope": "Reported public observation; no network query or acceptance verdict"}))
        else:
            for tx in pending:
                print(notification_line(tx))
        sys.exit(0)
    except Exception as exc:
        sys.stderr.write(f"Notification error: {exc}\n")
        sys.exit(3)


def cmd_deliver(repo: Path, db_path: Path, tx_id: str, acknowledgement_path: str | None, as_json: bool):
    try:
        if acknowledgement_path:
            raise StoreError("Caller JSON is not delivery evidence; post the notification in this conversation")
        tx = next((tx for tx in get_undelivered_txs(db_path, str(repo)) if tx["txId"] == tx_id), None)
        if tx is None:
            raise StoreError("No pending notification for this transaction")
        mark_delivered(db_path, tx_id, observe_delivery(tx))
        if as_json:
            print(json.dumps({"success": True, "delivered": tx_id}))
        else:
            print(f"Marked transaction {tx_id} as delivered")
        sys.exit(0)
    except Exception as exc:
        sys.stderr.write(f"Delivery error: {exc}\n")
        sys.exit(3)


def cmd_status(repo: Path, db_path: Path, history_reader, as_json: bool):
    t0 = time.time()
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
    last_result = _derive_last_result(repo)
    failures = snap.get("sameDefectFailures")
    if failures is not None and failures >= 2:
        blocked_action = f"another broad {active_action.get('kind', 'correction')}"
        reason = f"{failures} failures of required execution behavior"
    elif failures is None or any(
        isinstance(item, str) and (item == "operational-history" or item.startswith("operational-history"))
        for item in snap.get("missingEvidence", [])
    ):
        blocked_action = f"implementation/repair of {active_action.get('capability', 'action')}"
        reason = "operational history is unresolved or unverified"
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

    if db_path.is_file():
        try:
            record_admin_interval(db_path, str(repo), t0, time.time(), "status")
        except Exception:
            pass


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


def _load_admitted_runner(repo: Path, action: dict) -> tuple[dict, str]:
    return load_runner(repo, action)


def cmd_run(repo: Path, db_path: Path, history_reader, action_id: str, as_json: bool):
    t0 = time.time()
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
        runner, runner_digest = _load_admitted_runner(repo, action)
    except Exception as exc:
        msg = f"Launch unavailable for '{action_id}': {exc}"
        if as_json:
            print(json.dumps({"error": msg, "reasonCode": "RUNNER_UNAVAILABLE"}))
        else:
            sys.stderr.write(msg + "\n")
        sys.exit(2)

    try:
        res_id = reserve(db_path, str(repo), action, snap, charge_id=runner["chargeId"])
    except ReservationConflictError as exc:
        sys.stderr.write(f"Reservation conflict: {exc}\n")
        sys.exit(2)
    except Exception as exc:
        sys.stderr.write(f"Store error during reservation: {exc}\n")
        sys.exit(3)

    # Immediately revalidate snapshot and policy after reservation
    post_snap = load_snapshot(str(repo), action_id, history_reader=history_reader)
    post_snap["primaryActive"] = has_other_primary(db_path, res_id)
    post_dec = assess(post_snap, action)
    if not post_dec["allow"]:
        finish(db_path, res_id, {"error": f"Policy denied after reservation: {post_dec['reasonCode']}"}, status="failed")
        msg = f"Policy denied action '{action_id}' after reservation: {post_dec['reasonCode']}"
        if as_json:
            print(json.dumps({"error": msg, "decision": post_dec}))
        else:
            sys.stderr.write(msg + "\n")
        sys.exit(2)

    command_ref = action.get("commandRef", "")
    if not command_ref or action.get("kind") == "report":
        finish(db_path, res_id, {"exitCode": 0}, status="finished")
        if as_json:
            print(json.dumps({"success": True, "action": action_id}))
        sys.exit(0)

    # Revalidate action and runner bytes after reservation, before launch.
    try:
        current_action = next(a for a in read_actions(str(repo)) if a.get("id") == action_id)
        current_runner, current_digest = _load_admitted_runner(repo, current_action)
        if current_action != action or current_digest != runner_digest:
            raise ValueError("admitted action or runner changed after reservation")
    except Exception as exc:
        finish(db_path, res_id, {"error": f"Runner revalidation failed: {exc}"}, status="failed")
        sys.stderr.write(f"Runner revalidation failed: {exc}\n")
        sys.exit(2)

    receipt = None
    try:
        receipt = execute(repo, runner, runner_digest, res_id)
        observed = receipt["runnerReceipt"]
        # Check current command/input and durable debit again before claiming
        # completion. Changed evidence or supervisor loss stays unresolved.
        _, terminal_digest = _load_admitted_runner(repo, action)
        if terminal_digest != runner_digest or observed["completionAmbiguous"]:
            raise StoreError("Runner completion is ambiguous; reservation and charge remain claimed")
        verified = bool(observed["behavioralAssertions"])
        success = not observed["outputLimitExceeded"] and (
            verified if action["kind"] == "reproduce" else receipt["exitCode"] == 0
        )
        if verified:
            record_event(db_path, str(repo), action["requirement"], action["capability"],
                         action["candidate"], action_id, "reproducer_verified", receipt)
        finish(db_path, res_id, receipt, status="finished" if success else "failed")
        result = {"success": success, "action": action_id, "exitCode": receipt["exitCode"],
                  "chargeId": runner["chargeId"], "reservationId": res_id}
        if action["kind"] == "reproduce":
            result["reproducerVerified"] = verified
        if as_json:
            print(json.dumps(result))
        elif not success:
            sys.stderr.write("Bound runner did not establish the required result\n")
        sys.exit(0 if success else 4)
    except Exception as exc:
        # Parent/process death is not evidence that its children and charge
        # stopped. Keep the reservation active and preserve the uncertainty.
        record_event(db_path, str(repo), action["requirement"], action["capability"],
                     action["candidate"], action_id, "runner_unresolved", {"error": str(exc), "reservationId": res_id, "receipt": receipt})
        sys.stderr.write(f"Runner completion unresolved: {exc}\n")
        sys.exit(4)


def cmd_review(repo: Path, db_path: Path, receipt_path: str, as_json: bool, action_id: str | None = None):
    t0 = time.time()
    p = Path(receipt_path)
    if not p.is_absolute():
        p = (repo / p).resolve()
    if not p.is_file():
        sys.stderr.write(f"Receipt file missing: {p}\n")
        sys.exit(3)

    try:
        data = json.loads(p.read_text(encoding="utf-8"))
    except Exception as exc:
        sys.stderr.write(f"Error reading receipt JSON: {exc}\n")
        sys.exit(3)

    if not isinstance(data, dict):
        sys.stderr.write("Review rejected: receipt must be a JSON object\n")
        sys.exit(3)

    try:
        actions = read_actions(str(repo))
    except Exception as exc:
        sys.stderr.write(f"Error reading actions: {exc}\n")
        sys.exit(3)

    target_action = None
    if action_id:
        matching = [a for a in actions if a.get("id") == action_id]
        if not matching:
            sys.stderr.write(f"Review rejected: unknown action '{action_id}'\n")
            sys.exit(3)
        target_action = matching[0]
    else:
        req_receipt = data.get("requirement")
        cap_receipt = data.get("capability")
        cand_receipt = data.get("candidateHash") or data.get("candidate")
        act_id_receipt = data.get("actionId") or data.get("action")

        if act_id_receipt:
            matching = [a for a in actions if a.get("id") == act_id_receipt]
            if matching:
                target_action = matching[0]

        if not target_action and req_receipt and cap_receipt:
            matching = [a for a in actions if a.get("requirement") == req_receipt and a.get("capability") == cap_receipt]
            if matching:
                rev_matching = [a for a in matching if a.get("kind") == "review"]
                target_action = rev_matching[0] if rev_matching else matching[0]

        if not target_action and cand_receipt:
            matching = [a for a in actions if a.get("candidate") == cand_receipt]
            if matching:
                rev_matching = [a for a in matching if a.get("kind") == "review"]
                target_action = rev_matching[0] if rev_matching else matching[0]

    if not target_action:
        sys.stderr.write("Review rejected: receipt does not bind to any registered action or capability in repository\n")
        sys.exit(3)

    cand = data.get("candidateHash") or data.get("candidate")
    if not cand or not isinstance(cand, str) or len(cand) < 16:
        sys.stderr.write("Review rejected: valid candidateHash of at least 16 hex chars required\n")
        sys.exit(3)

    action_cand = target_action.get("candidate", "")
    if cand != action_cand:
        sys.stderr.write(f"Review rejected: candidateHash {cand} does not match action candidate {action_cand}\n")
        sys.exit(3)

    receipt_scope = data.get("scope") or data.get("auditScope")
    if not receipt_scope or not isinstance(receipt_scope, str) or receipt_scope.strip() in ("", "none", "None"):
        sys.stderr.write("Review rejected: valid non-empty scope required\n")
        sys.exit(3)

    admission_ref = target_action.get("admissionRef", "")
    expected_scope = None
    if admission_ref.startswith("campaign:"):
        camp_id = admission_ref.split(":", 1)[1]
        camp_file = repo / "evidence" / "moriarty-completion-program-2026-09-07" / "report-reconciliation" / "campaign-admission.json"
        if camp_file.is_file():
            try:
                cdata = json.loads(camp_file.read_text(encoding="utf-8"))
                camp_record = cdata.get("campaigns", {}).get(camp_id, {})
                expected_scope = camp_record.get("scope")
            except Exception:
                pass

    if expected_scope and receipt_scope != expected_scope:
        sys.stderr.write(f"Review rejected: scope '{receipt_scope}' does not match campaign scope '{expected_scope}'\n")
        sys.exit(3)

    history_reader = make_history_reader(db_path)
    snap = load_snapshot(str(repo), target_action["id"], history_reader=history_reader)
    if snap.get("authorityCurrent") is False:
        sys.stderr.write(f"Review rejected: authority unavailable for action {target_action['id']}\n")
        sys.exit(3)
    if snap.get("candidateCurrent") is False:
        sys.stderr.write(f"Review rejected: candidate not current for action {target_action['id']}\n")
        sys.exit(3)

    author = data.get("author")
    reviewer = data.get("reviewer") or data.get("reviewerObserved") or data.get("reviewerRequested")
    if not reviewer or not isinstance(reviewer, str):
        sys.stderr.write("Review rejected: reviewer identity required\n")
        sys.exit(3)
    if author and isinstance(author, str):
        a_low = author.lower().strip()
        r_low = reviewer.lower().strip()
        if a_low in r_low or r_low in a_low:
            sys.stderr.write("Review rejected: author self-review is forbidden\n")
            sys.exit(3)

    try:
        record_review(db_path, str(repo), data, action=target_action)
        if db_path.is_file():
            try:
                record_admin_interval(db_path, str(repo), t0, time.time(), "review")
            except Exception:
                pass
    except StoreError as exc:
        sys.stderr.write(f"Review rejected: {exc}\n")
        sys.exit(3)
    except Exception as exc:
        sys.stderr.write(f"Store error recording review: {exc}\n")
        sys.exit(3)

    if as_json:
        print(json.dumps({"success": True, "receipt": str(p), "action": target_action["id"]}))
    else:
        print(f"Ingested review receipt from {p} for action {target_action['id']}")
    sys.exit(0)


def cmd_report(repo: Path, db_path: Path, history_reader, as_json: bool):
    t0 = time.time()
    sprint_rows = _derive_sprint_reports(repo)
    undelivered = get_undelivered_txs(db_path, str(repo))

    out = {
        "allSprints": sprint_rows,
        "pendingTransactions": undelivered,
        "hostCoverage": "unverified" if not (Path(os.environ.get("CODEX_HOME", str(Path.home() / ".codex"))) / "plugins" / "moriarty-dev").is_dir() else "installed-unverified",
    }
    if as_json:
        print(json.dumps(out, indent=2))
    else:
        print("# Moriarty Development Status Report\n")
        for row in sprint_rows:
            print(f"- **{row['sprint']}**: {row['goal']} (Status: {row['status']}) - {row['predicate']}")
    if db_path.is_file():
        try:
            record_admin_interval(db_path, str(repo), t0, time.time(), "report")
        except Exception:
            pass
    sys.exit(0)


def cmd_doctor(repo: Path, db_path: Path, as_json: bool):
    actions_file = repo / ".moriarty-dev" / "actions.json"
    actions_ok = actions_file.is_file()

    # Registration/source presence is observable; actual host interception is
    # not derivable from files. Never promote a self-authored coverage flag.
    codex_home = Path(os.environ.get("CODEX_HOME", Path.home() / ".codex"))
    source = Path(__file__).resolve().parents[2]
    cache = codex_home / "plugins" / "cache"
    host_installed = source.is_relative_to(cache.resolve()) or (codex_home / "plugins" / "moriarty-dev").is_dir()
    hook_definitions_present = (source / "hooks" / "hooks.json").is_file()
    hooks_registered = False  # Host activation/trust is not observable from source files.
    coverage = "installed-unverified" if host_installed else "unverified"
    guarantees = "Registered action launch is guarded through CLI run"
    limitations = "Host tool interception requires observed host evidence; source or registration presence does not establish coverage"

    data = {
        "repository": str(repo),
        "storeDb": str(db_path),
        "storeInitialized": db_path.is_file(),
        "actionsRegistered": actions_ok,
        "hostInstalled": host_installed,
        "hooksRegistered": hooks_registered,
        "hookDefinitionsPresent": hook_definitions_present,
        "hostCoverage": coverage,
        "guarantees": guarantees,
        "limitations": limitations,
    }
    if as_json:
        print(json.dumps(data, indent=2))
    else:
        print(f"Repository: {repo}")
        print(f"Store DB: {db_path} ({'initialized' if db_path.is_file() else 'missing'})")
        print(f"Actions file: {'present' if actions_ok else 'missing'}")
        print(f"Host coverage: {coverage}")
        print(f"Guarantees: {guarantees}")
        print(f"Limitations: {limitations}")
    sys.exit(0)


if __name__ == "__main__":
    main()
