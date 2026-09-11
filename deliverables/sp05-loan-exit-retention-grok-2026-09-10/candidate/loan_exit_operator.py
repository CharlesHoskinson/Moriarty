"""Caller used by a separately admitted future loan execute-once.

This module does not admit an allocation, launch a launcher, open a wallet,
or contact Preview. After a new admission has launched a Type=exec
RemainAfterExit=yes unit and saved a nonzero startup InvocationID, call
retain_loan_main_exit with that saved identity, the new unit name, an
evidence directory, and an explicit transport.

Example future wiring after admission, not authorized by this packet:

    from loan_exit_operator import retain_loan_main_exit
    from exit_retention import UserSystemctlTransport, invoke_user_systemctl

    result = retain_loan_main_exit(
        unit=allocation_unit,
        startup_observed=launcher_active["observed"],
        transport=UserSystemctlTransport(invoke_user_systemctl),
        evidence_dir=output_dir,
    )
    if result["status"] == "NOT_TERMINAL" or result.get("pending"):
        # still running; parent monitor may call again under its original deadline
        ...
    elif not result["exitRetained"]:
        # command-exit unmet; do not stop here; parent cleanup/timer owns the unit
        ...
"""
from exit_retention import retain_exit_evidence


def retain_loan_main_exit(unit, startup_observed, transport, evidence_dir, now_utc=None):
    return retain_exit_evidence(
        unit=unit,
        startup_observed=startup_observed,
        transport=transport,
        evidence_dir=evidence_dir,
        now_utc=now_utc,
    )
