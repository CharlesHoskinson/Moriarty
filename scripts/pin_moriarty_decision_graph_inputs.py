#!/usr/bin/env python3
"""Pin Graphify decision inputs without workstation-specific paths."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
GRAPH_ROOT = ROOT / "graphs/moriarty-decision-corpus"
SOURCE_SEMANTIC = GRAPH_ROOT / ".graphify_semantic.json"
SOURCE_DETECTION = GRAPH_ROOT / ".graphify_detect.json"
TARGET_SEMANTIC = ROOT / "evidence/moriarty-decision-semantic-2026-09-03.json"
TARGET_DETECTION = ROOT / "evidence/moriarty-decision-detection-2026-09-03.json"
REPOSITORY_DIRECTORIES = frozenset(
    {"deliverables", "docs", "evidence", "experiments", "raw", "wiki"}
)


def repository_relative(value: str) -> str:
    path = Path(value)
    if not path.is_absolute():
        return path.as_posix()
    for index, part in enumerate(path.parts):
        if part in REPOSITORY_DIRECTORIES:
            return Path(*path.parts[index:]).as_posix()
    raise ValueError(f"cannot make path repository-relative: {value}")


def normalize_paths(value, key: str | None = None):
    if isinstance(value, dict):
        return {
            item_key: normalize_paths(item_value, item_key)
            for item_key, item_value in value.items()
        }
    if isinstance(value, list):
        return [normalize_paths(item, key) for item in value]
    if isinstance(value, str) and key == "source_file":
        return repository_relative(value)
    return value


def main() -> None:
    semantic = normalize_paths(
        json.loads(SOURCE_SEMANTIC.read_text(encoding="utf-8"))
    )
    detection = json.loads(SOURCE_DETECTION.read_text(encoding="utf-8"))
    detection["scan_root"] = "."
    for kind, files in detection.get("files", {}).items():
        detection["files"][kind] = [repository_relative(path) for path in files]

    TARGET_SEMANTIC.write_text(
        json.dumps(semantic, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    TARGET_DETECTION.write_text(
        json.dumps(detection, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    print(TARGET_SEMANTIC.relative_to(ROOT))
    print(TARGET_DETECTION.relative_to(ROOT))


if __name__ == "__main__":
    main()
