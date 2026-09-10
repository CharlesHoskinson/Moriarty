"""Pure public-property check only. No process, filesystem or manager access."""
import re

def require_exit_zero(fields, expected_invocation):
    if not isinstance(expected_invocation,str) or not re.fullmatch(r"[0-9a-f]{32}",expected_invocation) or expected_invocation=="0"*32:
        raise ValueError("INVOCATION_REQUIRED")
    expected={"LoadState":"loaded","ActiveState":"active","SubState":"exited","Type":"exec","RemainAfterExit":"yes","Transient":"yes","MainPID":"0","Result":"success","ExecMainCode":"1","ExecMainStatus":"0","InvocationID":expected_invocation}
    if not isinstance(fields,dict) or any(fields.get(k)!=v for k,v in expected.items()):
        raise ValueError("EXIT_ZERO_NOT_ESTABLISHED")
    return {"status":"EXIT_ZERO_OBSERVED","exitCode":0,"invocationId":expected_invocation,"externalContainmentEstablished":False}
