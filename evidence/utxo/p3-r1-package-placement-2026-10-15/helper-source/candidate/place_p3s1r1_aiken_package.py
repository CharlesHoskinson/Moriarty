#!/usr/bin/env python3
"""Place only the frozen P3S1-R1 Aiken package; never execute package code."""

import argparse
import hashlib
import json
from pathlib import Path
import sys


MANIFEST_SHA256 = "a54de5695c7bc29bab50376a92506a3247127a0c1b1341b7c0067e1fac10a957"
MANIFEST_FORMAT = "p3s1r1-source-manifest/1"
PACKAGE_PREFIX = "aiken/"
PACKAGE_FILE_COUNT = 16
RECEIPT_NAME = "PLACEMENT-RECEIPT.json"


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def require_fresh_output(output: Path, source_root: Path) -> Path:
    """Require one absent leaf under an existing parent, outside the input root."""
    try:
        output.lstat()
    except FileNotFoundError:
        pass
    else:
        raise ValueError("output path already exists")
    parent = output.parent.resolve(strict=True)
    if not parent.is_dir():
        raise ValueError("output parent must be an existing directory")
    destination = parent / output.name
    if destination == source_root or source_root in destination.parents:
        raise ValueError("output must be outside the source root")
    return destination


def read_package(source_root: Path, manifest_path: Path) -> tuple[bytes, list]:
    """Read and check all selected input bytes before any output is created."""
    manifest_bytes = manifest_path.read_bytes()
    if digest(manifest_bytes) != MANIFEST_SHA256:
        raise ValueError("source manifest SHA256 does not match the frozen pin")
    manifest = json.loads(manifest_bytes)
    if manifest["format"] != MANIFEST_FORMAT:
        raise ValueError("unexpected frozen manifest format")
    members = [
        entry for entry in manifest["files"]
        if entry["path"].startswith(PACKAGE_PREFIX)
    ]
    if len(members) != PACKAGE_FILE_COUNT:
        raise ValueError("unexpected frozen package membership")

    package = []
    for entry in members:
        source_path = entry["path"]
        relative_path = source_path[len(PACKAGE_PREFIX):]
        data = (source_root / source_path).read_bytes()
        if len(data) != entry["bytes"] or digest(data) != entry["sha256"]:
            raise ValueError(f"source identity mismatch: {source_path}")
        package.append((source_path, relative_path, data, entry["sha256"]))
    return manifest_bytes, package


def place(source_root: Path, manifest_path: Path, output: Path) -> dict:
    source_root = source_root.resolve(strict=True)
    manifest_path = manifest_path.resolve(strict=True)
    if not source_root.is_dir():
        raise ValueError("source root must be an existing directory")
    output = require_fresh_output(output, source_root)
    manifest_bytes, package = read_package(source_root, manifest_path)

    # All 16 inputs have been checked and retained in memory at this point.
    # mkdir refuses an output created since the freshness check as well.
    output.mkdir()
    copied = []
    for source_path, relative_path, data, expected_sha256 in package:
        destination = output / relative_path
        destination.parent.mkdir(parents=True, exist_ok=True)
        with destination.open("xb") as stream:
            stream.write(data)
        actual = destination.read_bytes()
        actual_sha256 = digest(actual)
        if len(actual) != len(data) or actual_sha256 != expected_sha256:
            raise ValueError(f"copied identity mismatch: {relative_path}")
        copied.append({
            "sourcePath": source_path,
            "path": relative_path,
            "bytes": len(actual),
            "sha256": actual_sha256,
        })

    receipt = {
        "format": "p3-aiken-placement-receipt/1",
        "sourceRoot": str(source_root),
        "sourceManifest": {
            "path": str(manifest_path),
            "bytes": len(manifest_bytes),
            "sha256": digest(manifest_bytes),
        },
        "packagePrefixStripped": PACKAGE_PREFIX,
        "output": str(output),
        "fileCount": len(copied),
        "packageBytes": sum(entry["bytes"] for entry in copied),
        "files": copied,
    }
    with (output / RECEIPT_NAME).open("x", encoding="utf-8") as stream:
        stream.write(json.dumps(receipt, indent=2) + "\n")
    return {
        "status": "placed",
        "fileCount": receipt["fileCount"],
        "packageBytes": receipt["packageBytes"],
        "output": str(output),
        "receipt": str(output / RECEIPT_NAME),
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-root", required=True, type=Path,
                        help="candidate root containing the frozen aiken/ files")
    parser.add_argument("--source-manifest", required=True, type=Path,
                        help="exact frozen SOURCE-MANIFEST-P3S1.json bytes")
    parser.add_argument("--output", required=True, type=Path,
                        help="fresh output directory under an existing parent")
    args = parser.parse_args()
    try:
        summary = place(args.source_root, args.source_manifest, args.output)
    except (OSError, ValueError, KeyError, TypeError) as error:
        print(f"placement failed: {error}", file=sys.stderr)
        return 1
    print(json.dumps(summary, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
