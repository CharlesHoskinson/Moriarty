#!/usr/bin/env python3
"""SDK-free FSYN P0/S0 interface over the in-repository golden. Exit 1 means fixture mismatch; exit 2 means invalid or stale input."""
import argparse
import json
import sys
from common import Error
import golden
from s0 import VerificationError, generate, verify

def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    pins = commands.add_parser("golden")
    pins.add_subparsers(dest="action", required=True).add_parser("check")
    command = commands.add_parser("generate")
    command.add_argument("--out", required=True)
    command.add_argument("--seed", type=lambda s: int(s, 0), default=0x5354524157)
    command.add_argument("--profile", default="both")
    command.add_argument("--scale", default="S0")
    command.add_argument("--mix", default="strawman")
    commands.add_parser("verify").add_argument("--fixture", required=True)
    args = parser.parse_args(argv)
    try:
        if args.command == "golden": result = golden.check(golden.ROOT, golden.PIN)
        elif args.command == "generate":
            result = generate(args.out, seed=args.seed, profile=args.profile, scale=args.scale, mix=args.mix)
        else: result = verify(args.fixture)
    except VerificationError as exc:
        print(json.dumps({"status": "refused", "error": str(exc)}, sort_keys=True), file=sys.stderr)
        return 1
    except (Error, OSError) as exc:
        print(json.dumps({"status": "refused", "error": str(exc)}, sort_keys=True), file=sys.stderr)
        return 2
    print(json.dumps(result, sort_keys=True))
    return 0

if __name__ == "__main__": raise SystemExit(main())
