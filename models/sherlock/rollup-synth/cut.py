"""P0 source identity checks. Matching pins are not semantic approval."""
from pathlib import Path
import copy
from common import Error, canonical, digest, load_json, regular, sha

SCHEMA = "moriarty-fsyn-design-cut/1"

def _document(path):
    doc = load_json(path)
    if not isinstance(doc, dict) or doc.get("schema") != SCHEMA: raise Error("unknown source-cut schema")
    groups = doc.get("groups")
    if not isinstance(groups, dict) or not groups: raise Error("empty source-cut groups")
    seen = set()
    for group, rows in groups.items():
        if not isinstance(group, str) or not group or not isinstance(rows, list) or not rows: raise Error("invalid group")
        for row in rows:
            if not isinstance(row, dict) or not {"path", "bytes", "sha256"} <= row.keys(): raise Error("invalid source row")
            name = row["path"]
            if not isinstance(name, str) or not name or "\x00" in name or (name.startswith("~") and not name.startswith("~/")):
                raise Error("invalid source path")
            actual = Path(name).expanduser()
            if not actual.is_absolute(): raise Error("source paths must be absolute or ~/ relative")
            identity = str(actual.resolve())
            if identity in seen: raise Error("duplicate source row: " + name)
            seen.add(identity)
            if type(row["bytes"]) is not int or row["bytes"] < 0: raise Error("invalid source byte count")
            sha(row["sha256"])
    if sha(doc.get("cut_digest_sha256")) != digest(canonical(groups)): raise Error("source-cut digest mismatch")
    return doc

def check_cut(path):
    doc = _document(path)
    count = total = 0
    for rows in doc["groups"].values():
        for row in rows:
            raw = regular(Path(row["path"]).expanduser())
            if len(raw) != row["bytes"] or digest(raw) != row["sha256"]: raise Error("stale source: " + row["path"])
            count += 1
            total += len(raw)
    return {"schema": "fsyn-cut-check/1", "cut_digest": doc["cut_digest_sha256"], "files": count, "bytes": total,
            "status": "pins-match", "semantic_approval": False, "architecture_generated_check": "not-run"}

def refresh_cut(path, out):
    doc = copy.deepcopy(_document(path))
    out = Path(out)
    if out.exists(): raise Error("refresh requires a new output path")
    changed = []
    for rows in doc["groups"].values():
        for row in rows:
            raw = regular(Path(row["path"]).expanduser())
            current = (len(raw), digest(raw))
            if current != (row["bytes"], row["sha256"]): changed.append(row["path"])
            row["bytes"], row["sha256"] = current
    doc["cut_digest_sha256"] = digest(canonical(doc["groups"]))
    doc["qualification"] = "refreshed source identity only; fresh dependent review required"
    with out.open("xb") as stream: stream.write(canonical(doc) + b"\n")
    return {"cut_digest": doc["cut_digest_sha256"], "changed": changed, "semantic_approval": False}
