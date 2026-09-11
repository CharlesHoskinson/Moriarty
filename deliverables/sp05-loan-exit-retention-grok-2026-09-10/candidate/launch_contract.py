"""Launch and show properties required to retain a loaded main-process exit.

Provenance is the inspected preview-swap-exit-01 launcher and terminal
observation. This module does not admit an allocation, choose resource
limits, or name a live unit.
"""
import re

STARTUP_SHOW_PROPERTIES = (
    "LoadState,ActiveState,SubState,MainPID,ControlGroup,"
    "ActiveEnterTimestampMonotonic,InvocationID"
)
TERMINAL_SHOW_PROPERTIES = (
    "LoadState,ActiveState,SubState,Result,ExecMainCode,ExecMainStatus,"
    "MainPID,ControlGroup,InvocationID,Type,RemainAfterExit,Transient"
)
RETENTION_SYSTEMD_RUN_PROPERTIES = (
    "Type=exec",
    "RemainAfterExit=yes",
)

SCOPE = (
    "Actual loaded main exit retained before explicit unit stop. "
    "Not financial or containment acceptance."
)


def require_invocation_id(value):
    if (
        not isinstance(value, str)
        or not re.fullmatch(r"[0-9a-f]{32}", value)
        or value == "0" * 32
    ):
        raise ValueError("INVOCATION_REQUIRED")
    if int(value, 16) <= 0:
        raise ValueError("INVOCATION_REQUIRED")
    return value


def require_startup_identity(observed):
    if not isinstance(observed, dict):
        raise ValueError("STARTUP_IDENTITY_REQUIRED")
    if observed.get("LoadState") != "loaded":
        raise ValueError("STARTUP_IDENTITY_REQUIRED")
    if observed.get("ActiveState") != "active":
        raise ValueError("STARTUP_IDENTITY_REQUIRED")
    main_pid = observed.get("MainPID")
    if not isinstance(main_pid, str) or not re.fullmatch(r"[1-9][0-9]*", main_pid):
        raise ValueError("STARTUP_IDENTITY_REQUIRED")
    try:
        return require_invocation_id(observed.get("InvocationID"))
    except ValueError:
        raise ValueError("STARTUP_IDENTITY_REQUIRED")


def require_unit_name(unit):
    if not isinstance(unit, str) or not re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9:_.@-]*\.service", unit):
        raise ValueError("UNIT_REQUIRED")
    return unit


def terminal_show_argv(unit):
    return [
        "/usr/bin/systemctl",
        "--user",
        "show",
        require_unit_name(unit),
        "--property=" + TERMINAL_SHOW_PROPERTIES,
    ]


def explicit_stop_argv(unit):
    return [
        "/usr/bin/timeout",
        "--signal=KILL",
        "25s",
        "/usr/bin/systemctl",
        "--user",
        "stop",
        require_unit_name(unit),
    ]


def describe_contract():
    return {
        "retentionSystemdRunProperties": list(RETENTION_SYSTEMD_RUN_PROPERTIES),
        "startupShowProperties": STARTUP_SHOW_PROPERTIES,
        "terminalShowProperties": TERMINAL_SHOW_PROPERTIES,
        "terminalShowArgvTemplate": terminal_show_argv("unit.service"),
        "explicitStopArgvTemplate": explicit_stop_argv("unit.service"),
        "terminalPredicateExpected": {
            "LoadState": "loaded",
            "ActiveState": "active",
            "SubState": "exited",
            "Type": "exec",
            "RemainAfterExit": "yes",
            "Transient": "yes",
            "MainPID": "0",
            "Result": "success",
            "ExecMainCode": "1",
            "ExecMainStatus": "0",
            "InvocationID": "same-nonzero-32-hex-startup-id",
        },
        "execMainCodeMeaning": (
            "ExecMainCode=1 is CLD_EXITED; ExecMainStatus=0 is the main exit code. "
            "ExecMainCode=0 with ExecMainStatus=0 is an unloaded-unit default, not exit zero."
        ),
        "caller": "loan_exit_operator.retain_loan_main_exit",
        "admission": None,
        "liveCollectionAuthorized": False,
        "financialAcceptance": False,
        "containmentAcceptance": False,
        "sp05Complete": False,
        "scope": SCOPE,
    }
