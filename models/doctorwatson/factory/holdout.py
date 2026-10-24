"""Sealed holdout scenarios: sha256 manifest verify and seal."""

import hashlib
import os

from . import repo_root

MANIFEST = "MANIFEST.sha256"
EXIT_MISMATCH = 7
EXIT_REFUSED = 8


def holdout_dir(repo=None):
    return os.path.join(repo or repo_root(), "qa", "holdout")


def digest(path):
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def scan(root):
    """Return {relative posix path: sha256} for every file except the top-level manifest."""
    out = {}
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames.sort()
        for name in filenames:
            full = os.path.join(dirpath, name)
            rel = os.path.relpath(full, root).replace(os.sep, "/")
            if rel == MANIFEST:
                continue
            out[rel] = digest(full)
    return out


def read_manifest(root):
    """Return ({path: sha}, [malformed lines]) or (None, [reason]) if absent."""
    p = os.path.join(root, MANIFEST)
    if not os.path.isfile(p):
        return None, [f"{MANIFEST} not found"]
    entries, bad = {}, []
    with open(p, encoding="utf-8") as fh:
        for n, line in enumerate(fh, 1):
            line = line.rstrip("\n")
            if not line.strip():
                continue
            sha, sep, rel = line.partition("  ")
            if not sep or len(sha) != 64 or not rel:
                bad.append(f"{MANIFEST}:{n}: malformed line")
                continue
            entries[rel] = sha
    return entries, bad


def verify(repo=None):
    """Return (ok, problems)."""
    root = holdout_dir(repo)
    entries, bad = read_manifest(root)
    if entries is None:
        return False, bad
    actual = scan(root) if os.path.isdir(root) else {}
    problems = list(bad)
    for rel in sorted(entries):
        if rel not in actual:
            problems.append(f"MISSING {rel}")
        elif actual[rel] != entries[rel]:
            problems.append(f"CHANGED {rel}")
    for rel in sorted(actual):
        if rel not in entries:
            problems.append(f"UNLISTED {rel}")
    return not problems, problems


def seal(repo=None):
    root = holdout_dir(repo)
    os.makedirs(root, exist_ok=True)
    actual = scan(root)
    text = "".join(f"{sha}  {rel}\n" for rel, sha in sorted(actual.items()))
    tmp = os.path.join(root, f".{MANIFEST}.tmp.{os.getpid()}")
    with open(tmp, "w", encoding="utf-8") as fh:
        fh.write(text)
    os.replace(tmp, os.path.join(root, MANIFEST))
    return len(actual)


def main(action):
    if action == "seal":
        if os.environ.get("DW_ROLE") != "evaluator":
            print("holdout seal refused: DW_ROLE=evaluator is required (holdout is evaluator-owned)")
            return EXIT_REFUSED
        n = seal()
        print(f"sealed {n} files into qa/holdout/{MANIFEST}")
        return 0
    ok, problems = verify()
    if ok:
        print("holdout OK")
        return 0
    for p in problems:
        print(p)
    print("holdout MISMATCH")
    return EXIT_MISMATCH
