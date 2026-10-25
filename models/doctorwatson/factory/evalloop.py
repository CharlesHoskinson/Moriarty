"""Evaluator-loop helpers: factory-state clean check, state auto-commit, contract-check.

`clean`        exit 0 when the tree is clean ignoring qa/sherlock/{status.json,runs.jsonl}.
`state-commit` commit only those two files as chore(factory-state); never touches other paths.
`contract-check ID|PATH` run qa/holdout/contracts/<ID>.json (schema: qa/holdout/contracts/README.md).
"""

import json
import os
import subprocess

from . import repo_root

STATE_FILES = ("qa/sherlock/status.json", "qa/sherlock/runs.jsonl")
CONTRACT_FORMAT = "doctorwatson-contract/1"


def _git(repo, *args):
    return subprocess.run(["git", "-C", repo, *args], capture_output=True, text=True)


def dirty_paths(repo, ignore=STATE_FILES):
    r = _git(repo, "status", "--porcelain", "--untracked-files=all")
    out = []
    for line in r.stdout.splitlines():
        path = line[3:].split(" -> ")[-1].strip('"')
        if path not in ignore:
            out.append(line)
    return out


def cmd_clean():
    bad = dirty_paths(repo_root())
    for line in bad:
        print(line)
    return 1 if bad else 0


def cmd_state_commit():
    repo = repo_root()
    r = _git(repo, "status", "--porcelain", "--", *STATE_FILES)
    if not r.stdout.strip():
        print("factory state clean")
        return 0
    c = _git(repo, "commit", "-q", "-m", "chore(factory-state): record check runs and status",
             "--only", "--", *STATE_FILES)
    if c.returncode:
        print(c.stdout + c.stderr)
        return 1
    print("committed factory state")
    return 0


def _walk(node, parts):
    """Yield values at a dotted path; `[]` fans out over list elements."""
    if not parts:
        yield node
        return
    head, rest = parts[0], parts[1:]
    fan = head.endswith("[]")
    key = head[:-2] if fan else head
    if key:
        if not isinstance(node, dict) or key not in node:
            return
        node = node[key]
    if fan:
        if not isinstance(node, list) or not node:
            return
        for el in node:
            yield from _walk(el, rest)
    else:
        yield from _walk(node, rest)


def _present(doc, path):
    parts = path.split(".")
    want = 1
    if any(p.endswith("[]") for p in parts):
        # every element must carry the path: compare counts against the fan-out size
        head = []
        for p in parts:
            head.append(p)
            if p.endswith("[]"):
                break
        want = len(list(_walk(doc, head)))
        if want == 0:
            return False
    return len(list(_walk(doc, parts))) >= want


def cmd_contract_check(ident):
    repo = repo_root()
    path = ident if os.path.isfile(ident) else os.path.join(repo, "qa/holdout/contracts", ident + ".json")
    try:
        with open(path) as fh:
            c = json.load(fh)
        assert c.get("format") == CONTRACT_FORMAT, "format must be " + CONTRACT_FORMAT
        argv = c["entrypoint"]["argv"]
        assert isinstance(argv, list) and argv, "entrypoint.argv must be a non-empty list"
        exp = c.get("expect", {})
    except (OSError, ValueError, KeyError, AssertionError) as exc:
        print(f"bad contract {ident}: {exc}")
        return 2
    cwd = os.path.join(repo, c["entrypoint"].get("cwd", "."))
    errors = []
    try:
        r = subprocess.run(argv, cwd=cwd, capture_output=True, timeout=exp.get("timeout_s", 600))
    except (OSError, subprocess.TimeoutExpired) as exc:
        print(f"contract {ident}: entrypoint failed to run: {exc}")
        return 1
    want_exit = exp.get("exit", 0)
    if r.returncode != want_exit:
        errors.append(f"exit {r.returncode}, expected {want_exit}")
    cap = exp.get("max_stdout_bytes")
    if cap is not None and len(r.stdout) > cap:
        errors.append(f"stdout {len(r.stdout)} bytes exceeds cap {cap}")
    if exp.get("json", True):
        try:
            doc = json.loads(r.stdout)
        except ValueError:
            errors.append("stdout is not JSON")
            doc = None
        if doc is not None:
            for p in exp.get("required_paths", []):
                if not _present(doc, p):
                    errors.append(f"missing required path {p}")
            for p in exp.get("forbidden_paths", []):
                if list(_walk(doc, p.split("."))):
                    errors.append(f"forbidden path present {p}")
    if errors:
        for e in errors:
            print(f"contract {ident}: {e}")
        return 1
    print(f"contract {ident}: OK")
    return 0
