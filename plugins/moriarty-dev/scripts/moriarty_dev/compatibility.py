"""Read-only package observations, never host activation or trust claims."""
import hashlib
import json
import os
from pathlib import Path
import platform
import sys

PLUGIN_NAME = "moriarty-dev"
CORE_FILES = (
    ".codex-plugin/plugin.json", "hooks/hooks.json",
    "scripts/moriarty_dev/__init__.py", "scripts/moriarty_dev/hook.py",
    "scripts/moriarty_dev/cli.py", "scripts/moriarty_dev/policy.py",
    "scripts/moriarty_dev/records.py", "scripts/moriarty_dev/store.py",
    "scripts/moriarty_dev/runner.py", "scripts/moriarty_dev/notifications.py",
)


def _observe_root(root):
    root = Path(root).expanduser().resolve()
    missing = [name for name in CORE_FILES if not (root / name).is_file()]
    errors = []
    manifest = {}
    try:
        manifest = json.loads((root / ".codex-plugin/plugin.json").read_text(encoding="utf-8"))
        if not isinstance(manifest, dict) or manifest.get("name") != PLUGIN_NAME:
            errors.append("manifest identity mismatch")
            manifest = {}
    except (OSError, ValueError):
        errors.append("manifest unavailable or invalid")
    hook_hash = None
    try:
        hook_hash = hashlib.sha256((root / "scripts/moriarty_dev/hook.py").read_bytes()).hexdigest()
    except OSError:
        errors.append("hook script unavailable")
    return {
        "path": str(root), "version": manifest.get("version"),
        "usable": not missing and not errors,
        "missingFiles": missing, "errors": errors, "hookSha256": hook_hash,
    }


def inspect_compatibility(plugin_root, expected_roots=()):
    """Observe source and cache entries without changing any installation state.

    `usable` is a basic package-presence check, not runtime execution evidence.
    Cached versions may be inactive. CODEX_HOME comes from the invoking process.
    """
    codex_home = Path(os.environ.get("CODEX_HOME") or Path.home() / ".codex").expanduser().resolve()
    cache = codex_home / "plugins/cache"
    candidates = sorted(cache.glob(f"*/{PLUGIN_NAME}/*"))
    legacy = codex_home / "plugins" / PLUGIN_NAME
    if legacy.exists():
        candidates.append(legacy)
    roots = sorted({candidate.resolve() for candidate in candidates if candidate.is_dir()})
    return {
        "codexHome": str(codex_home),
        "executingPlugin": _observe_root(plugin_root),
        "cachedRoots": [_observe_root(root) for root in roots],
        "expectedRoots": [_observe_root(root) for root in expected_roots],
        "environmentRoots": {
            name: _observe_root(os.environ[name])
            for name in ("PLUGIN_ROOT", "CLAUDE_PLUGIN_ROOT") if os.environ.get(name)
        },
        "activation": "unknown", "hookTrust": "unknown",
        "runtime": {
            "python": platform.python_version(), "pythonMinimum": "3.11",
            "pythonSupported": sys.version_info >= (3, 11),
            "platform": sys.platform,
            "hookShell": "POSIX shell with python3",
            "runnerPlatform": "Linux with the admitted Foreman launcher and strong containment",
        },
    }


def observed_coverage(observation):
    installed = any(root["usable"] for root in observation["cachedRoots"])
    return "installed-unverified" if installed else "unverified"
