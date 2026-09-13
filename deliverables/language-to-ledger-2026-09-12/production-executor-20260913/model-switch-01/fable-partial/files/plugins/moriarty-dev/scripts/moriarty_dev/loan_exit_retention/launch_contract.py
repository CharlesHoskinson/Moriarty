"""Launch and show properties required to retain a loaded main-process exit.

Provenance is the inspected preview-swap-exit-01 launcher and terminal
observation. This module does not admit an allocation, choose resource
limits, or name a live unit. Argv builders accept explicit remaining-time
bounds; the retained argv is the argv actually invoked.
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
SYSTEMCTL = "/usr/bin/systemctl"
TIMEOUT = "/usr/bin/timeout"
MAX_SHOW_SECONDS = 5
MAX_STOP_SECONDS = 25

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
    # Actual positive activation time and a nonempty control group are part of
    # the loaded startup identity; they are never reconstructed later.
    started = observed.get("ActiveEnterTimestampMonotonic")
    if not isinstance(started, str) or not re.fullmatch(r"[1-9][0-9]*", started):
        raise ValueError("STARTUP_IDENTITY_REQUIRED")
    group = observed.get("ControlGroup")
    if not isinstance(group, str) or not group.startswith("/"):
        raise ValueError("STARTUP_IDENTITY_REQUIRED")
    try:
        return require_invocation_id(observed.get("InvocationID"))
    except ValueError:
        raise ValueError("STARTUP_IDENTITY_REQUIRED")


def require_unit_name(unit):
    if not isinstance(unit, str) or not re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9:_.@-]*\.service", unit):
        raise ValueError("UNIT_REQUIRED")
    return unit


def require_bound_seconds(value, maximum, code="BOUND_REQUIRED"):
    """Explicit positive integer second bound, capped at the fixed maximum."""
    if type(value) is bool or type(value) is not int or value <= 0:
        raise ValueError(code)
    return min(value, maximum)


def timeout_prefix(seconds):
    return [TIMEOUT, "--signal=KILL", "%ds" % seconds]


def terminal_show_argv(unit, show_seconds=None):
    """systemctl show argv; with show_seconds it is wrapped in the exact timeout."""
    unit = require_unit_name(unit)
    argv = [SYSTEMCTL, "--user", "show", unit, "--property=" + TERMINAL_SHOW_PROPERTIES]
    if show_seconds is None:
        return argv
    return timeout_prefix(require_bound_seconds(show_seconds, MAX_SHOW_SECONDS)) + argv


def startup_show_argv(unit, show_seconds):
    unit = require_unit_name(unit)
    argv = [SYSTEMCTL, "--user", "show", unit, "--property=" + STARTUP_SHOW_PROPERTIES]
    return timeout_prefix(require_bound_seconds(show_seconds, MAX_SHOW_SECONDS)) + argv


def explicit_stop_argv(unit, stop_seconds=MAX_STOP_SECONDS):
    unit = require_unit_name(unit)
    bound = require_bound_seconds(stop_seconds, MAX_STOP_SECONDS)
    return timeout_prefix(bound) + [SYSTEMCTL, "--user", "stop", unit]


def describe_contract():
    return {
        "retentionSystemdRunProperties": list(RETENTION_SYSTEMD_RUN_PROPERTIES),
        "startupShowProperties": STARTUP_SHOW_PROPERTIES,
        "terminalShowProperties": TERMINAL_SHOW_PROPERTIES,
        "terminalShowArgvTemplate": terminal_show_argv("unit.service"),
        "terminalShowBoundedArgvTemplate": terminal_show_argv("unit.service", MAX_SHOW_SECONDS),
        "explicitStopArgvTemplate": explicit_stop_argv("unit.service"),
        "maxShowSeconds": MAX_SHOW_SECONDS,
        "maxStopSeconds": MAX_STOP_SECONDS,
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
            "ExecMainCode=2 is CLD_KILLED and 3 is CLD_DUMPED with the signal number. "
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
