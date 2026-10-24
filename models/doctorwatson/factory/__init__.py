"""DoctorWatson factory core. Interfaces are frozen in CONTRACT.md."""

import datetime
import os
import subprocess

SHERLOCK = "qa/sherlock"


def repo_root():
    """Repo root: DW_REPO if set, else `git rev-parse --show-toplevel`, else cwd."""
    env = os.environ.get("DW_REPO")
    if env:
        return os.path.abspath(env)
    try:
        out = subprocess.run(
            ["git", "rev-parse", "--show-toplevel"],
            capture_output=True, text=True, check=True,
        ).stdout.strip()
        if out:
            return out
    except (OSError, subprocess.CalledProcessError):
        pass
    return os.getcwd()


def head_sha(repo=None):
    repo = repo or repo_root()
    try:
        out = subprocess.run(
            ["git", "-C", repo, "rev-parse", "HEAD"],
            capture_output=True, text=True, check=True,
        ).stdout.strip()
        return out or "unknown"
    except (OSError, subprocess.CalledProcessError):
        return "unknown"


def now_iso():
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def sherlock_path(name, repo=None):
    return os.path.join(repo or repo_root(), SHERLOCK, name)
