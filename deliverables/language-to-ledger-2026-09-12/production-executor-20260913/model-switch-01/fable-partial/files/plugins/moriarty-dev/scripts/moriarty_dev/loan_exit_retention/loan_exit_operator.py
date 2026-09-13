"""Caller used by the installed loan executor.

This module does not admit an allocation, launch a launcher, open a wallet,
or contact Preview. After an authenticated invocation has launched a Type=exec
RemainAfterExit=yes unit and saved a nonzero startup InvocationID, the
executor calls retain_loan_main_exit with that saved identity, the admitted
unit name, an evidence directory, an explicit transport and the remaining
show/stop bounds.
"""
from .exit_retention import retain_exit_evidence


def retain_loan_main_exit(unit, startup_observed, transport, evidence_dir, now_utc=None,
                          show_seconds=None, stop_seconds=None, persist_timeout_seconds=None, save=None):
    return retain_exit_evidence(
        unit=unit,
        startup_observed=startup_observed,
        transport=transport,
        evidence_dir=evidence_dir,
        now_utc=now_utc,
        show_seconds=show_seconds,
        stop_seconds=stop_seconds,
        persist_timeout_seconds=persist_timeout_seconds,
        save=save,
    )
