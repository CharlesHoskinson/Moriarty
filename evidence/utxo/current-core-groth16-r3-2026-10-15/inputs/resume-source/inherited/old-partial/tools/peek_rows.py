"""Read-only dump of frozen reviewer rows by key path (stdlib only; no candidate code).

Usage: python3 -I peek_rows.py <json-path> <top-key> [id-substring ...]
Prints every element of doc[top-key] (a list) whose JSON contains one of the given
substrings (all elements when none is given). Also summarises rejected-vector phase,
step and work_attempted when <top-key> is "vectors".
"""
import json
import sys


def main() -> None:
    path, key = sys.argv[1], sys.argv[2]
    subs = sys.argv[3:]
    with open(path, "rb") as f:
        doc = json.loads(f.read())
    rows = doc[key]
    if isinstance(rows, dict):
        print(json.dumps(rows, indent=1, sort_keys=True))
        return
    if key == "vectors" and not subs:
        for v in rows:
            ex = v["expected"]
            if not ex["accepted"]:
                print(v["id"], ex["error"], "phase", ex["phase"], "step", ex["step"], "attempted", ex["work_attempted"], "cap", v.get("logical_cap"))
        return
    for r in rows:
        text = json.dumps(r, sort_keys=True)
        if not subs or any(s in text for s in subs):
            print(json.dumps(r, indent=1, sort_keys=True))


if __name__ == "__main__":
    main()
