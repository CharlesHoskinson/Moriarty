#!/usr/bin/env python3
"""SDK-free first FSYN P0/S0 interface. Exit 1 means fixture mismatch; exit 2 means invalid or stale input."""
import argparse
import json
import sys
from common import Error
from cut import check_cut, refresh_cut
from s0 import VerificationError, generate, verify

def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    cut = commands.add_parser("cut")
    cuts = cut.add_subparsers(dest="action", required=True)
    check = cuts.add_parser("check")
    check.add_argument("--cut", required=True)
    refresh = cuts.add_parser("refresh")
    refresh.add_argument("--cut", required=True)
    refresh.add_argument("--out", required=True)
    for name in ("generate", "verify"):
        command = commands.add_parser(name)
        command.add_argument("--cut", required=True)
        command.add_argument("--golden-root", required=True)
        command.add_argument("--out" if name == "generate" else "--fixture", required=True)
        if name == "generate":
            command.add_argument("--seed", type=lambda s: int(s, 0), default=0x5354524157)
            command.add_argument("--profile", default="both")
            command.add_argument("--scale", default="S0")
            command.add_argument("--mix", default="strawman")
    args = parser.parse_args(argv)
    try:
        if args.command == "cut":
            result = check_cut(args.cut) if args.action == "check" else refresh_cut(args.cut, args.out)
        elif args.command == "generate":
            result = generate(args.cut, args.golden_root, args.out, seed=args.seed, profile=args.profile, scale=args.scale, mix=args.mix)
        else: result = verify(args.fixture, args.cut, args.golden_root)
    except VerificationError as exc:
        print(json.dumps({"status": "refused", "error": str(exc)}, sort_keys=True), file=sys.stderr)
        return 1
    except (Error, OSError) as exc:
        print(json.dumps({"status": "refused", "error": str(exc)}, sort_keys=True), file=sys.stderr)
        return 2
    print(json.dumps(result, sort_keys=True))
    return 0

if __name__ == "__main__": raise SystemExit(main())
