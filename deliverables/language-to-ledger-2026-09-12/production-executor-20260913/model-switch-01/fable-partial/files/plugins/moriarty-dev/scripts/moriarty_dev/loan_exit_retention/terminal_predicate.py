"""Pure public-property checks only. No process, filesystem or manager access.

require_exit_zero is the original zero-only predicate and is unchanged in
strength. classify_main_exit extends it with the closed nonzero/signal/unknown
table from the afk-loan-executor specification; unknown combinations are
never an implicit zero.
"""
import re

_IDENTITY = {
    "LoadState": "loaded",
    "Type": "exec",
    "RemainAfterExit": "yes",
    "Transient": "yes",
}
_SIGNAL_CODES = ("2", "3")  # CLD_KILLED, CLD_DUMPED


def _require_expected_invocation(expected_invocation):
    if (
        not isinstance(expected_invocation, str)
        or not re.fullmatch(r"[0-9a-f]{32}", expected_invocation)
        or expected_invocation == "0" * 32
    ):
        raise ValueError("INVOCATION_REQUIRED")
    return expected_invocation


def require_exit_zero(fields, expected_invocation):
    _require_expected_invocation(expected_invocation)
    expected = {
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
        "InvocationID": expected_invocation,
    }
    if not isinstance(fields, dict) or any(fields.get(k) != v for k, v in expected.items()):
        raise ValueError("EXIT_ZERO_NOT_ESTABLISHED")
    return {
        "status": "EXIT_ZERO_OBSERVED",
        "exitCode": 0,
        "invocationId": expected_invocation,
        "externalContainmentEstablished": False,
    }


def _canonical_int(value, low, high):
    if not isinstance(value, str) or not re.fullmatch(r"0|[1-9][0-9]*", value):
        return None
    number = int(value)
    if number < low or number > high:
        return None
    return number


def classify_main_exit(fields, expected_invocation):
    """Closed classification of one terminal-show observation.

    Returns {"rawMainExit": {"kind","code"}, "status", "failureCode"}.
    status is "pending" only for the exact same-invocation running row.
    """
    _require_expected_invocation(expected_invocation)
    unknown = {"rawMainExit": {"kind": "unknown", "code": None},
               "status": "PROCESS_UNKNOWN", "failureCode": "MAIN_OBSERVATION_INVALID"}
    if not isinstance(fields, dict):
        return unknown
    if any(fields.get(k) != v for k, v in _IDENTITY.items()):
        return unknown
    if fields.get("InvocationID") != expected_invocation:
        return unknown
    active = fields.get("ActiveState")
    sub = fields.get("SubState")
    main_pid = fields.get("MainPID")
    if active == "active" and sub == "running":
        # The exact running row: a live main process and no terminal record.
        if _canonical_int(main_pid, 1, 2 ** 22) is None:
            return unknown
        if fields.get("ExecMainCode") != "0" or fields.get("ExecMainStatus") != "0" \
                or fields.get("Result") not in ("", "success"):
            return unknown
        return {"rawMainExit": {"kind": "unknown", "code": None},
                "status": "pending", "failureCode": None}
    if main_pid != "0":
        return unknown
    terminal = (active == "active" and sub == "exited") or (active == "failed" and sub == "failed")
    if not terminal:
        return unknown
    result = fields.get("Result")
    code = fields.get("ExecMainCode")
    status = fields.get("ExecMainStatus")
    if code == "1":
        number = _canonical_int(status, 0, 255)
        if number is None:
            return unknown
        if number == 0:
            if result != "success" or not (active == "active" and sub == "exited"):
                return unknown
            return {"rawMainExit": {"kind": "exit", "code": 0},
                    "status": "PROCESS_SUCCESS", "failureCode": None}
        if result != "exit-code":
            return unknown
        return {"rawMainExit": {"kind": "exit", "code": number},
                "status": "PROCESS_FAILED", "failureCode": "MAIN_EXIT_NONZERO"}
    if code in _SIGNAL_CODES:
        number = _canonical_int(status, 1, 64)
        if number is None or result not in ("signal", "core-dump"):
            return unknown
        return {"rawMainExit": {"kind": "signal", "code": number},
                "status": "PROCESS_FAILED", "failureCode": "MAIN_SIGNAL"}
    return unknown
