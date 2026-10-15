"""Read-only structure dump of frozen reviewer JSON (stdlib only; no candidate code).

Usage: python3 -I peek_vectors.py <json-path> [vector-id ...]
Prints top-level keys, per-vector keys/classes and the full JSON of named vectors.
"""
import json
import sys


def main() -> None:
    path = sys.argv[1]
    want = set(sys.argv[2:])
    with open(path, "rb") as f:
        doc = json.loads(f.read())
    print("top-level keys:", list(doc.keys()) if isinstance(doc, dict) else type(doc).__name__)
    for key, value in (doc.items() if isinstance(doc, dict) else []):
        if isinstance(value, list):
            print(f"  {key}: list[{len(value)}]")
        elif isinstance(value, dict):
            print(f"  {key}: dict keys {list(value.keys())[:40]}")
        else:
            text = json.dumps(value)
            print(f"  {key}: {text[:300]}")
    vectors = doc.get("vectors") if isinstance(doc, dict) else None
    if isinstance(vectors, list):
        for v in vectors:
            ex = v.get("expected", {})
            print(v.get("id"), v.get("program"), v.get("action"), "cap" if v.get("logical_cap") else "-",
                  "ACCEPT" if ex.get("accepted") else "REJECT", sorted(ex.keys()))
    for v in vectors or []:
        if v.get("id") in want:
            print(json.dumps(v, indent=1, sort_keys=True))


if __name__ == "__main__":
    main()
