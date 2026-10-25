"""Recover relocated public sources without refreshing the frozen design cut."""
from pathlib import Path, PurePosixPath
import os
import subprocess

from common import Error, canonical, digest, regular
from cut import _document

# Fenrir's import of the original revision-20 research snapshot. Later revisions
# have different bytes; only a matching SHA256 may serve as a source alias.
DESIGN_IMPORT = "b5e04373628187d8532cf31f5e05b4730e98102c"
PREFIXES = (
    ("~/research/rollup-subarch-2026-10-21/lean/FenrirRollup/", "formal/rollup/lean/"),
    ("~/research/rollup-subarch-2026-10-21/models/", "formal/rollup/quint/"),
    ("~/research/rollup-subarch-2026-10-21/synthesis/", "design/rollup/"),
    ("~/research/rollup-subarch-2026-10-21/", "design/rollup/"),
    ("~/research/fenrir-rollup-design-2026-10-19/", "design/rollup/"),
    ("~/projectfenrir/", ""),
    ("~/fenrir-sequencer-spike/", "sequencer/"),
    ("~/fenrir/", "fenrir-workspace/"),
)


def relocated(name):
    for original, current in PREFIXES:
        if name.startswith(original):
            tail = name[len(original):]
            if ".." in PurePosixPath(tail).parts:
                raise Error("unsafe source relocation: " + name)
            return current + tail
    return None


def matches(path, row):
    if not path.is_file() or path.is_symlink():
        return False
    raw = regular(path)
    return len(raw) == row["bytes"] and digest(raw) == row["sha256"]


def prepare_sources(cut, fenrir, cache, archive=None):
    """Return an alias-map path only after recovering every original source pin."""
    doc = _document(cut)
    fenrir, cache = Path(fenrir).resolve(), Path(cache).resolve()
    aliases, missing = {}, []
    for rows in doc["groups"].values():
        for row in rows:
            name = row["path"]
            rel = relocated(name)
            candidates = [Path(name).expanduser()]
            if rel is not None:
                candidates.append(fenrir / rel)
            if archive is not None and name.startswith("~/"):
                # An archive mirrors the old home-relative public source paths.
                tail = name[2:]
                if ".." in PurePosixPath(tail).parts:
                    raise Error("unsafe archive source: " + name)
                candidates.append(Path(archive).resolve() / tail)
            candidates.append(cache / row["sha256"])
            found = next((p for p in candidates if matches(p, row)), None)
            if found is None and rel is not None:
                result = subprocess.run(
                    ["git", "-C", str(fenrir), "show", DESIGN_IMPORT + ":" + rel],
                    capture_output=True, timeout=30,
                    # Missing partial-clone objects must not trigger a network fetch.
                    env={**os.environ, "GIT_NO_LAZY_FETCH": "1"},
                )
                if (result.returncode == 0 and len(result.stdout) == row["bytes"]
                        and digest(result.stdout) == row["sha256"]):
                    cache.mkdir(parents=True, exist_ok=True)
                    found = cache / row["sha256"]
                    if found.is_symlink():
                        raise Error("source cache member is a symlink: " + str(found))
                    found.write_bytes(result.stdout)
            if found is None:
                missing.append(f"{name} bytes={row['bytes']} sha256={row['sha256']}")
            else:
                aliases[name] = str(found.absolute())
    if missing:
        raise Error("missing pinned public sources (set FSYN_ARCHIVE_ROOT to a home-relative archive):\n"
                    + "\n".join(missing))
    cache.mkdir(parents=True, exist_ok=True)
    output = cache / "source-map.json"
    temporary = cache / "source-map.json.tmp"
    with temporary.open("w") as stream:
        stream.write(canonical(aliases).decode("utf-8") + "\n")
    temporary.replace(output)
    return output


def main():
    import argparse
    import sys
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cut", required=True)
    parser.add_argument("--fenrir-root", required=True)
    parser.add_argument("--cache", required=True)
    args = parser.parse_args()
    try:
        output = prepare_sources(args.cut, args.fenrir_root, args.cache, os.environ.get("FSYN_ARCHIVE_ROOT"))
    except (Error, OSError, subprocess.TimeoutExpired) as exc:
        print(str(exc), file=sys.stderr)
        return 2
    print(output)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
