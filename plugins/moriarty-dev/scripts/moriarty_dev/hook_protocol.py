"""Conservative JSON wire contract shared by supported hook hosts.

The host owns permissions. Absence of a decision is the only permitted-call
response we emit; Stop uses common output only. No host version guessing.
"""
import json

EVENTS = frozenset({"SessionStart", "PreToolUse", "PostToolUse", "Stop"})
INPUT_LIMIT = 8192
OUTPUT_LIMIT = 2048
DIAGNOSTIC = (
    "Moriarty hook input or records unavailable; use guarded CLI status/run. "
    "Host coverage remains unverified."
)


def supported_event(value):
    return isinstance(value, str) and value in EVENTS


def output(event, **fields):
    if event == "Stop" or not supported_event(event):
        return {}
    return {"hookSpecificOutput": {"hookEventName": event, **fields}}


def diagnostic(event):
    return {**output(event), "systemMessage": DIAGNOSTIC}


def read_payload(stream):
    """Read at most limit+1 bytes; reject damaged input before repository work."""
    raw = getattr(stream, "buffer", stream).read(INPUT_LIMIT + 1)
    if isinstance(raw, str):  # StringIO/in-process callers, not host stdin.
        raw = raw.encode("utf-8")
    if len(raw) > INPUT_LIMIT:
        raise ValueError("hook input exceeds byte limit")
    text = raw.decode("utf-8")
    payload = json.loads(text) if text.strip() else {}
    if not isinstance(payload, dict):
        raise ValueError("hook input must be an object")
    if "cwd" in payload:
        cwd = payload["cwd"]
        if not isinstance(cwd, str) or not cwd or "\x00" in cwd:
            raise ValueError("invalid hook cwd")
        cwd.encode("utf-8")
    return payload


def serialize(response):
    """Keep denial semantics even when escaped JSON exceeds the byte budget."""
    response = dict(response)
    inner = dict(response.get("hookSpecificOutput", {}))
    if "hookSpecificOutput" in response:
        response["hookSpecificOutput"] = inner
    for owner, key, limit in ((inner, "additionalContext", 1300),
                              (inner, "permissionDecisionReason", 350),
                              (response, "systemMessage", 180)):
        if isinstance(owner.get(key), str):
            owner[key] = owner[key].encode("utf-8", errors="replace")[:limit].decode("utf-8", errors="ignore")
    # ASCII JSON also works under non-UTF8 host stdout encodings.
    text = json.dumps(response, ensure_ascii=True, separators=(",", ":"))
    if len(text.encode("ascii")) + 1 <= OUTPUT_LIMIT:
        return text
    if inner.get("permissionDecision") == "deny":
        response = output("PreToolUse", permissionDecision="deny",
                          permissionDecisionReason="Moriarty denied this action; use CLI status for details.")
    else:
        response = {"systemMessage": "Moriarty hook output exceeded bound; use CLI status."}
    return json.dumps(response, separators=(",", ":"))
