"""Append one event to CHRONOLOGY.jsonl (append-only; never rewrites earlier lines).

Usage: python3 -I tools/chrono.py EVENT [DETAIL...]
"""
import datetime
import hashlib
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LOG = ROOT / "CHRONOLOGY.jsonl"


def main():
    prev = LOG.read_bytes() if LOG.exists() else b""
    ev = {"at": datetime.datetime.now(datetime.timezone.utc).isoformat(), "event": sys.argv[1],
          "detail": " ".join(sys.argv[2:]), "prevLogSha256": hashlib.sha256(prev).hexdigest(),
          "prevLogBytes": len(prev)}
    with LOG.open("ab") as f:
        f.write((json.dumps(ev, ensure_ascii=False) + "\n").encode())
    print(json.dumps(ev))


if __name__ == "__main__":
    main()
