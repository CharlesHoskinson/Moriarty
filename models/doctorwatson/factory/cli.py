"""Command line: `python3 -m factory <cmd>` (wrapped by scripts/doctorwatson)."""

import argparse
import json
import os
import sys

from . import bus, head_sha, heavy, holdout, now_iso, preflight, repo_root, spawn, status, stoprule
from . import predicates as preds_mod

NO_RESULTS_HINT = "(no results yet — run scripts/doctorwatson check --tier T0)"


def _load_or_die():
    try:
        return preds_mod.load(repo_root())
    except preds_mod.PredicateError as exc:
        for e in exc.errors:
            print(f"error: {e}", file=sys.stderr)
        return None


def cmd_next(args):
    preds = _load_or_die()
    if preds is None:
        return 1
    st = status.load()
    entries = status.entries_of(st)
    counts, first = status.view(preds, entries)
    by_id = {p.id: p for p in preds}
    no_results = st is None
    if args.json:
        ff = None
        if first:
            ff = dict(first, title=by_id[first["id"]].title)
        print(json.dumps({"counts": counts, "firstFailing": ff, "noResults": no_results}, indent=2))
        return 0
    for s in preds_mod.SETS:
        print(f"{s}: {counts[s][0]}/{counts[s][1]}")
    if no_results:
        print(NO_RESULTS_HINT)
    if first:
        print(f"FIRST FAILING: {first['id']} — {by_id[first['id']].title}")
        print(f"CHECK: {first['check']}")
    else:
        print("ALL PASSING")
    return 0


def _select(preds, args):
    if args.ids:
        known = {p.id for p in preds}
        unknown = [i for i in args.ids if i not in known]
        if unknown:
            print(f"error: unknown predicate id(s): {', '.join(unknown)}", file=sys.stderr)
            return None
    wanted = set(args.ids or [])
    out = []
    for p in preds_mod.topo_order(preds):
        if wanted and p.id not in wanted:
            continue
        if args.set and p.set != args.set:
            continue
        if args.tier and p.tier != args.tier:
            continue
        out.append(p)
    return out


def _unit_status(code):
    return "PASS" if code == 0 else "FAIL"


def cmd_check(args):
    repo = repo_root()
    preds = _load_or_die()
    if preds is None:
        return 1
    selected = _select(preds, args)
    if selected is None:
        return 2
    prev_status = bus.snapshot(repo)  # bus hook input only
    prior = status.entries_of(status.load(repo))
    passing = {p.id: (prior.get(p.id) or {}).get("passes") is True for p in preds}
    head = head_sha(repo)
    results = {}
    all_pass = True
    for p in selected:
        blockers = [d for d in p.depends if not passing.get(d)]
        if blockers:  # applies to explicitly named ids too: never record a pass past a failing dependency
            print(f"{p.id}: BLOCKED-BY {blockers[0]}")
            passing[p.id] = False
            all_pass = False
            results[p.id] = {"passes": False, "exit": None, "at": now_iso(), "run": None,
                             "blockedBy": blockers[0]}
            continue
        argv = ["bash", "-c", p.check]
        if p.heavy:
            r = heavy.run_heavy(p.id, argv, p.check, p.budget, args.hypothesis, repo, echo=False)
            code = r["exit"]
            if code in (heavy.EXIT_UNFIT, heavy.EXIT_HYPOTHESIS):
                print(f"{p.id}: FAIL exit={code} {r['message']}")
                passing[p.id] = False
                all_pass = False
                continue  # nothing ran: leave the stored result alone
            rid, trace = r["run"], r["trace"]
        else:
            rid = heavy.run_id(p.id, repo)
            trace = heavy.trace_rel(rid)
            res = heavy.run_command(argv, repo, os.path.join(repo, trace), False)
            code = res["exit"]
            status.append_run(heavy.run_record(rid, p.id, p.check, args.hypothesis, res, code,
                                               None, head, repo), repo)
        ok = code == 0
        passing[p.id] = ok
        all_pass &= ok
        suffix = "" if ok else f" exit={code}"
        print(f"{p.id}: {_unit_status(code)}{suffix}  ({trace})")
        results[p.id] = {"passes": ok, "exit": code, "at": now_iso(), "run": rid}
    new_status = status.update(preds, results, increment=True, repo=repo)
    bus.safe(bus.after_check, repo, prev_status, new_status, results)  # doorbell; never raises
    return 0 if all_pass else 1


def cmd_preflight(args):
    return preflight.main(args.json)


def cmd_heavy(args):
    return heavy.cmd_heavy(args.id, args.budget, args.hypothesis, args.cmd)


def cmd_stop_rule(args):
    return stoprule.main(args.json)


def cmd_holdout(args):
    return holdout.main(args.action)


def cmd_bus(args):
    return {
        "doctor": lambda: bus.cmd_doctor(args.accept_off),
        "status": bus.cmd_status,
        "tail": lambda: bus.cmd_tail(args.since),
        "inbox": bus.cmd_inbox,
        "publish": lambda: bus.cmd_publish(args.type, args.data, args.to),
        "review": lambda: bus.cmd_review(args.range, args.t3, args.out),
        "watch": lambda: bus.cmd_watch(args.max_time),
    }[args.bus_cmd]()


def cmd_spawn(args):
    args.extra = args.extra or []
    return spawn.main(args)


def cmd_validate(args):
    errors = preds_mod.validate(repo_root())
    if errors:
        for e in errors:
            print(e)
        return 1
    print("predicates OK")
    return 0


def build_parser():
    ap = argparse.ArgumentParser(prog="doctorwatson", description="DoctorWatson factory")
    sub = ap.add_subparsers(dest="cmd", required=True)

    p = sub.add_parser("next", help="counts and first failing predicate")
    p.add_argument("--json", action="store_true")
    p.set_defaults(fn=cmd_next)

    p = sub.add_parser("check", help="run predicate checks")
    p.add_argument("ids", nargs="*", metavar="ID")
    p.add_argument("--set", choices=preds_mod.SETS)
    p.add_argument("--tier", choices=preds_mod.TIERS, help="predicates whose tier is exactly this")
    p.add_argument("--hypothesis")
    p.set_defaults(fn=cmd_check)

    p = sub.add_parser("preflight", help="host fitness check")
    p.add_argument("--json", action="store_true")
    p.set_defaults(fn=cmd_preflight)

    p = sub.add_parser("heavy", help="run a heavy command: heavy --id ID [--budget KEY] -- CMD...")
    p.add_argument("--id", required=True)
    p.add_argument("--budget")
    p.add_argument("--hypothesis")
    p.set_defaults(fn=cmd_heavy)

    p = sub.add_parser("stop-rule", help="escalation rule")
    p.add_argument("--json", action="store_true")
    p.set_defaults(fn=cmd_stop_rule)

    p = sub.add_parser("holdout", help="verify or seal qa/holdout")
    p.add_argument("action", choices=("verify", "seal"))
    p.set_defaults(fn=cmd_holdout)

    p = sub.add_parser("validate", help="schema-check qa/sherlock/predicates.yaml")
    p.set_defaults(fn=cmd_validate)

    p = sub.add_parser("bus", help="AgentBus: doctor|status|tail|inbox|publish|review|watch")
    bsub = p.add_subparsers(dest="bus_cmd", required=True)
    q = bsub.add_parser("doctor", help="agentbus doctor plus DW_BUS/identity/breaker summary")
    q.add_argument("--accept-off", action="store_true", help="exit 0 when the bus is declared or found off")
    bsub.add_parser("status", help="retained kv status and stop")
    q = bsub.add_parser("tail", help="recent doctorwatson.bus/1 events from the durable log")
    q.add_argument("--since", default="1h")
    bsub.add_parser("inbox", help="classified, typed view of unread mail (peek, never consumes)")
    q = bsub.add_parser("publish", help="manually emit an event")
    q.add_argument("--type", required=True)
    q.add_argument("--data", default="{}", help="JSON object")
    q.add_argument("--to", help="override the route (agent name, '*' or '#chan')")
    q = bsub.add_parser("review", help="publish review.request; --t3 also runs and waits for the cold review")
    q.add_argument("--range", required=True, metavar="A..B")
    q.add_argument("--t3", action="store_true", help="run the reviewer and wait at most 3600 s, then proceed")
    q.add_argument("--out", help="reviewer output file (--t3)")
    q = bsub.add_parser("watch", help="agentbus watch --exit-on-first on stop.escalate|review.reply|status.flip")
    q.add_argument("--max-time", type=int)
    p.set_defaults(fn=cmd_bus)

    p = sub.add_parser("spawn", help="run a vendor CLI as a bus participant: spawn --vendor V --as N --role R --brief F [--] ARGS")
    p.add_argument("--vendor", required=True, choices=sorted(spawn.VENDORS))
    p.add_argument("--as", dest="name", required=True, metavar="NAME")
    p.add_argument("--role", required=True, choices=spawn.ROLES)
    p.add_argument("--brief", required=True, metavar="FILE")
    p.add_argument("--effort", default="high", choices=spawn.EFFORTS)
    p.add_argument("--model")
    p.add_argument("--sandbox", choices=("read-only", "workspace-write"))
    p.add_argument("--out", help="output directory (default qa/sherlock/agents/NAME-STAMP)")
    p.add_argument("--slot-wait", type=float, default=0.0, metavar="SECS", help="wait for a vendor slot (default: fail fast, exit 10)")
    p.add_argument("--no-bus", action="store_true", help="plain run, no identity or bookends")
    p.add_argument("--dry-run", action="store_true", help="print the vendor command line and exit")
    p.set_defaults(fn=cmd_spawn)
    return ap


def main(argv=None):
    argv = list(sys.argv[1:] if argv is None else argv)
    cmd_tail = None
    if argv and argv[0] in ("heavy", "spawn") and "--" in argv:
        i = argv.index("--")
        argv, cmd_tail = argv[:i], argv[i + 1:]
    ap = build_parser()
    args = ap.parse_args(argv)
    if args.cmd == "heavy":
        if not cmd_tail:
            ap.error("heavy needs a command after `--`")
        args.cmd = cmd_tail
    elif args.cmd == "spawn":
        args.extra = cmd_tail or []
    return args.fn(args)
