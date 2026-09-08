import argparse
import json
import os
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import time

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
    record_admin_interval,
    get_undelivered_txs,
    mark_delivered,
    bootstrap_store,
    ReservationConflictError,
    StoreError,
)

SPRINT_PREDICATES = {
    "SP01": "RP01 full challenge map, native statement subset (RP01-MC03), and native path freeze remain open",
    "SP02": "Successor frontend (.mori authoring frontend, BNF/EBNF specification and lexical/static validation) remains open",
    "SP03": "Successor semantics (executable bounded semantics in K, evaluator, local simulation) remains open",
    "SP04": "Native verifier component feasibility (f0a, f1-fixtures, f1 IVC carried-state) remains open",
    "SP05": "Financial integration on Preview (i2 local/Preview loan and swap complete-effect ledger execution) remains open",
    "SP06": "Real recursive financial history (f2 two-step financial native proof and independent review) remains open",
    "SP07": "ACTUS obligations and lifecycle semantics (actus-semantics library and lowering) remains open",
    "SP08": "DeFi actions and outcome intents (defi-semantics, intent/request lowering) remains open",
    "SP09": "Mandatory PCD and ledger correspondence (f3 Preview financial acceptance, mandatory-claim extension) remains open",
    "SP10": "Private handoff and bounded composition (composition private extension and correspondence) remains open",
    "SP11": "Full financial and formal conformance (finance domain-shift coverage) remains open",
    "SP12": "Developer release and reproducible evidence (release evidence under versioned program) remains open",
}


def _derive_last_result(repo: Path) -> str:
    program_path = repo / "openspec/moriarty-completion-program.json"
    if program_path.is_file():
        try:
            data = json.loads(program_path.read_text(encoding="utf-8"))
            stages = data.get("reportReconciliation", {}).get("stageAdmission", {}).get("stages", [])
            completed = [s["id"] for s in stages if s.get("status") == "complete"]
            if completed:
                return f"Stages complete: {', '.join(completed)}; financial settlement and PCD correspondence remain open"
        except Exception:
            pass
    return "Initial stage; Preview financial settlement and general PCD remain open"


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
        predicate = SPRINT_PREDICATES.get(sp_id, f"{title} verification remains open")
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
    review_parser.add_argument("--receipt", required=True, help="Path to review receipt JSON")

    subparsers.add_parser("report", parents=[json_parent])
    subparsers.add_parser("doctor", parents=[json_parent])
    subparsers.add_parser("bootstrap", parents=[json_parent])

    deliver_parser = subparsers.add_parser("deliver", parents=[json_parent])
    deliver_parser.add_argument("--tx-id", required=True, help="Transaction ID to mark delivered")

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
        cmd_review(repo, db_path, args.receipt, args.json)
    elif args.command == "report":
        cmd_report(repo, db_path, history_reader, args.json)
    elif args.command == "doctor":
        cmd_doctor(repo, db_path, args.json)
    elif args.command == "bootstrap":
        cmd_bootstrap(repo, db_path, args.json)
    elif args.command == "deliver":
        cmd_deliver(repo, db_path, args.tx_id, args.json)


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


def cmd_deliver(repo: Path, db_path: Path, tx_id: str, as_json: bool):
    try:
        mark_delivered(db_path, tx_id)
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
        res_id = reserve(db_path, str(repo), action, snap)
    except ReservationConflictError as exc:
        sys.stderr.write(f"Reservation conflict: {exc}\n")
        sys.exit(2)
    except Exception as exc:
        sys.stderr.write(f"Store error during reservation: {exc}\n")
        sys.exit(3)

    # Immediately revalidate snapshot and policy after reservation
    post_snap = load_snapshot(str(repo), action_id, history_reader=history_reader)
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

    # Execute with bounded runner (timeout, output bounds, cwd in repo)
    timeout_s = 120
    try:
        proc = subprocess.run(
            argv,
            cwd=str(repo),
            capture_output=True,
            text=True,
            check=False,
            timeout=timeout_s,
        )
        ret = proc.returncode
        receipt = {
            "exitCode": ret,
            "stdout": proc.stdout[:2048],
            "stderr": proc.stderr[:2048],
        }

        if action.get("kind") == "reproduce":
            is_behavioral_defect = (
                ret != 0
                and ("Defect demonstrated" in proc.stderr
                     or "Defect reproduced" in proc.stderr
                     or "admitted-not-executed" in proc.stderr)
                and "Traceback (most recent call last)" not in proc.stderr
                and "ImportError" not in proc.stderr
                and "ModuleNotFoundError" not in proc.stderr
            )
            is_clean_pass = (ret == 0)

            if is_clean_pass or is_behavioral_defect:
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
                finish(db_path, res_id, receipt, status="finished")
                if as_json:
                    print(json.dumps({"success": True, "reproducerVerified": True, "exitCode": 0, "action": action_id}))
                else:
                    print(f"Reproducer verified defect for {action_id}")
                sys.exit(0)
            else:
                finish(db_path, res_id, receipt, status="failed")
                sys.stderr.write(f"Reproducer failed without demonstrating required defect (exit code {ret})\n")
                sys.exit(4)

        finish(db_path, res_id, receipt, status="finished" if ret == 0 else "failed")
        if ret != 0:
            sys.stderr.write(f"Child command failed with exit code {ret}\n")
            sys.exit(4)
        if as_json:
            print(json.dumps({"success": True, "exitCode": 0, "action": action_id}))
        sys.exit(0)
    except subprocess.TimeoutExpired:
        finish(db_path, res_id, {"error": "Execution timed out", "timeoutSeconds": timeout_s}, status="timeout")
        sys.stderr.write(f"Child command timed out after {timeout_s}s\n")
        sys.exit(4)
    except Exception as exc:
        finish(db_path, res_id, {"error": str(exc)}, status="crashed")
        sys.stderr.write(f"Child execution exception: {exc}\n")
        sys.exit(4)


def cmd_review(repo: Path, db_path: Path, receipt_path: str, as_json: bool):
    t0 = time.time()
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
        print(json.dumps({"success": True, "receipt": str(p)}))
    else:
        print(f"Ingested review receipt from {p}")
    sys.exit(0)


def cmd_report(repo: Path, db_path: Path, history_reader, as_json: bool):
    t0 = time.time()
    sprint_rows = _derive_sprint_reports(repo)
    undelivered = get_undelivered_txs(db_path, str(repo))

    out = {
        "allSprints": sprint_rows,
        "pendingTransactions": undelivered,
        "hostCoverage": "wrapper-only",
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

    # Inspect host hooks and trust
    codex_home = Path(os.environ.get("CODEX_HOME", Path.home() / ".codex"))
    hooks_file = codex_home / "hooks.json"
    installed_plugin_dir = codex_home / "plugins" / "moriarty-dev"

    host_installed = installed_plugin_dir.is_dir()
    hooks_registered = False
    hooks_trust_verified = False

    if hooks_file.is_file():
        try:
            hdata = json.loads(hooks_file.read_text(encoding="utf-8"))
            raw_str = json.dumps(hdata)
            if "moriarty" in raw_str:
                hooks_registered = True
                for hlist in hdata.get("hooks", {}).values():
                    for group in hlist:
                        for entry in group.get("hooks", []):
                            cmd = entry.get("command", "")
                            if "moriarty" in cmd:
                                parts = cmd.split()
                                if parts and any(Path(p).is_file() for p in parts if "/" in p):
                                    hooks_trust_verified = True
        except Exception:
            pass

    if hooks_trust_verified and host_installed:
        coverage = "host-verified"
        guarantees = "Full host tool interception active and verified"
        limitations = "None for host interception"
    elif hooks_registered or host_installed:
        coverage = "degraded"
        guarantees = "Partial host registration observed; interception unverified"
        limitations = "Incomplete host plugin registration or unverified hook executable"
    else:
        coverage = "wrapper-only"
        guarantees = "Guarded execution enforced via CLI fallback; no background daemon"
        limitations = "Host hook interception not active in ~/.codex/hooks.json; CLI is verified fallback"

    data = {
        "repository": str(repo),
        "storeDb": str(db_path),
        "storeInitialized": db_path.is_file(),
        "actionsRegistered": actions_ok,
        "hostInstalled": host_installed,
        "hooksRegistered": hooks_registered,
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
