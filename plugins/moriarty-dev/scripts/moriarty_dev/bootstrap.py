"""Embedded in hook registrations so cache eviction cannot remove the launcher."""
import hashlib
import json
import os
from pathlib import Path
import runpy
import sys

UNAVAILABLE = {"systemMessage": "Moriarty hook runtime unavailable; interception degraded. Repair from source and use guarded CLI status/run. Host coverage remains unverified."}


def inventory(root):
    base = Path(root) / "scripts/moriarty_dev"
    if not (base / "hook.py").is_file():
        raise ValueError("missing hook runtime")
    rows = {}
    for path in sorted(base.rglob("*")):
        if path.is_symlink():
            raise ValueError("runtime symlink")
        if path.is_file() and path.suffix == ".py":
            rows[path.relative_to(base).as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
    return rows


def runtime_digest(root):
    return hashlib.sha256(json.dumps(inventory(root), sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def main():
    try:
        digest, event = sys.argv[1:]
        if len(digest) != 64 or any(c not in "0123456789abcdef" for c in digest):
            raise ValueError("invalid runtime pin")
        native = os.environ.get("PLUGIN_ROOT") or os.environ.get("CLAUDE_PLUGIN_ROOT")
        candidates = ([Path(native)] if native else []) + [Path.home() / ".local/share/moriarty-dev/runtimes" / digest]
        selected = None
        for candidate in candidates:
            try:
                if runtime_digest(candidate) == digest:
                    selected = candidate
                    break
            except (OSError, ValueError):
                continue
        if selected is None:
            raise ValueError("pinned runtime unavailable")
        # Isolated Python ignores PYTHONPATH/user site; only this verified package
        # is added. Same-user hostile mutation between hash/read is out of scope.
        sys.dont_write_bytecode = True
        entry = selected / "scripts/moriarty_dev/hook.py"
        sys.argv = [str(entry), event]
        runpy.run_path(str(entry), run_name="__main__")
    except Exception:
        print(json.dumps(UNAVAILABLE, separators=(",", ":")))


if __name__ == "__main__":
    main()
