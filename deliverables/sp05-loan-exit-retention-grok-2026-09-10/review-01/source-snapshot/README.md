# SP05 loan exit-retention source candidate

Source-only repair for the Preview loan missing raw main-process exit. It reuses the inspected `preview-swap-exit-01` launcher contract (`Type=exec`, `RemainAfterExit=yes`, nonzero startup `InvocationID`, loaded/exited `systemctl show` retained before explicit stop) and the existing terminal predicate.

This packet does not admit an allocation, launch a unit, or contact a wallet, network, prover, Docker, or live user systemd. Guarded SP05 dispatch remains blocked. No admission receipt is included or invented.

## Callable path for a later admitted loan run

After a **separate** admitted allocation has launched the unit with `launch_contract.RETENTION_SYSTEMD_RUN_PROPERTIES` and saved a loaded/active startup observation (`MainPID>0`, nonzero 32-hex `InvocationID`):

```python
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
    # command-exit unmet; FINANCIAL_COMPLETE is not exit evidence; do not stop here
    ...
```

`retain_loan_main_exit` shows once. A same-invocation still-running unit returns `status=NOT_TERMINAL` and `pending=True` with no stop and no final observation/stop files; the existing parent monitor may call again under its original deadline. This packet does not poll or schedule. Matching terminal-zero evidence is saved before any explicit stop. Invalid startup, mismatched/failed/malformed observations, and persistence failure do not stop the unit; the separately admitted parent cleanup/timer remains responsible on error or deadline. Save-before-stop is required for `exitRetained`. Cleanup/stop errors stay explicit and never imply financial or containment acceptance. No resources or admission are waived.

## CLI

Import is a no-op. Missing mode prints usage and exits 2. `describe` prints the contract. `collect-once` always refuses: this packet ships no admission and does not authorize live `systemctl`.

```sh
python3 deliverables/sp05-loan-exit-retention-grok-2026-09-10/candidate/cli.py describe
```

## Offline checks (root-owned)

From the checkout root, assertions enabled:

```sh
python3 -m unittest discover -s deliverables/sp05-loan-exit-retention-grok-2026-09-10/candidate -v
```

Tests drive `loan_exit_operator.retain_loan_main_exit` with scripted transports, a local Python subprocess stand-in, and the authentic loan (unloaded/`not-found`) and swap (loaded/exited) observations. They are not service, network, or financial evidence.

## Limits

Does not close SP05, MC02, Preview command-exit, containment, or PCD. Historical loan attempts remain consumed. A new public run needs a later reviewed resource amendment and exclusive admission, which this source does not grant.
