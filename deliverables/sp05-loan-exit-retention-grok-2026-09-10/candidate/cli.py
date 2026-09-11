"""Import-safe CLI for the loan exit-retention candidate.

Import is a no-op. Without an explicit mode the process prints usage and
exits 2. collect-once refuses live systemctl: this packet ships no admission.
"""
import json
import pathlib
import sys

_CANDIDATE = pathlib.Path(__file__).resolve().parent
if str(_CANDIDATE) not in sys.path:
    sys.path.insert(0, str(_CANDIDATE))

from launch_contract import describe_contract

USAGE = """loan exit-retention source CLI
usage:
  cli.py describe
  cli.py collect-once --admission PATH --unit UNIT --startup PATH --evidence-dir PATH

Import does not call systemctl. describe prints the launch/terminal contract.
collect-once refuses: this packet ships no admission and does not authorize
live user systemctl. Call loan_exit_operator.retain_loan_main_exit from a
separately admitted executor.
"""


def main(argv):
    if not __debug__:
        sys.stderr.write("ASSERTIONS_REQUIRED\n")
        return 2
    if argv is None:
        argv = []
    argv = list(argv)
    if not argv:
        sys.stderr.write(USAGE)
        return 2
    if argv[0] in ("-h", "--help", "help"):
        sys.stdout.write(USAGE)
        return 0
    if argv[0] == "describe":
        json.dump(describe_contract(), sys.stdout, indent=2)
        sys.stdout.write("\n")
        return 0
    if argv[0] == "collect-once":
        sys.stderr.write(
            "collect-once refused: no admission is shipped in this source packet; "
            "live user systemctl is not authorized here; "
            "call loan_exit_operator.retain_loan_main_exit after a separate "
            "admitted allocation\n"
        )
        return 2
    sys.stderr.write(USAGE)
    return 2


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
