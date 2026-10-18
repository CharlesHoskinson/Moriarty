"""Pre-freeze self-check of the independent world builder against frozen DATA only (no checker import).

1. mcj_ind + world_ind.manifest_rows reproduce the genuine v1.1 manifest digest and every manifest row.
2. Scan the genuine catalogue for values that only pass the old validators because of '$'-before-newline or
   Unicode '\\d' (a fixed fullmatch/ASCII checker would otherwise refuse the genuine catalogue).
3. Report sha256 of SemanticEntry inputs for PLN heads (sanity).

Usage: python3 -I tools/selfcheck_world.py
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "harness"))
import world_ind as W  # noqa: E402

ASCII_DEC = re.compile(r"(0|-?[1-9][0-9]*)", re.ASCII)
ASCII_ID = re.compile(r"MCP-[A-Z]+-[0-9]{2}", re.ASCII)
ASCII_VER = re.compile(r"(0|[1-9][0-9]*)", re.ASCII)


def strings(o, where="$"):
    if isinstance(o, str):
        yield where, o
    elif isinstance(o, dict):
        for k, v in o.items():
            yield where + ".<key>", k
            yield from strings(v, f"{where}.{k}")
    elif isinstance(o, list):
        for i, v in enumerate(o):
            yield from strings(v, f"{where}[{i}]")


def main():
    doc = json.loads((ROOT / "inputs/catalogue-v1.1.json").read_bytes())
    man = json.loads((ROOT / "inputs/manifest-v1.1.json").read_bytes())
    out = {"checks": {}}
    out["checks"]["docDigestEqualsManifest"] = W.doc_digest(doc) == man["catalogue_manifest_digest"]
    rows = W.manifest_rows(doc)
    out["checks"]["manifestRowsEqual"] = rows == man["rows"]
    out["checks"]["manifestRowCount"] = [len(rows), len(man["rows"])]
    heads = W.heads_of(doc["rows"])
    out["checks"]["rowCount"] = len(doc["rows"])
    out["checks"]["currentHeads"] = len(heads)
    out["checks"]["patternFamilies"] = len(doc["patternFamilies"])
    # alias scan over row identity/typing fields
    bad = []
    for r in doc["rows"]:
        vid = r["versionedId"]
        if not ASCII_ID.fullmatch(r["id"]):
            bad.append((vid, "id", r["id"]))
        if not ASCII_VER.fullmatch(r["version"]):
            bad.append((vid, "version", r["version"]))
        if r["versionedId"] != f"{r['id']}@{r['version']}":
            bad.append((vid, "versionedId", r["versionedId"]))
        s = r.get("statementSchema")
        if isinstance(s, dict):
            for f in s.get("fields", []):
                for k in ("min", "max"):
                    if k in f and not ASCII_DEC.fullmatch(f[k]):
                        bad.append((vid, f"field {f.get('name')}.{k}", f[k]))
                if not re.fullmatch(r"[a-z][a-z0-9_]*", f.get("name", ""), re.ASCII):
                    bad.append((vid, "field name", f.get("name")))
                if "value" in f and "\n" in f["value"]:
                    bad.append((vid, f"field {f.get('name')}.value", f["value"]))
    nl = [w for w, s in strings(doc) if s.endswith("\n")]
    out["checks"]["rowAliasProneValues"] = bad
    out["checks"]["stringsEndingInNewlineCount"] = len(nl)
    out["checks"]["stringsEndingInNewlineSample"] = nl[:20]
    out["checks"]["nonAsciiDigitInIds"] = [r["id"] for r in doc["rows"] if any(c.isdigit() and not c.isascii() for c in r["id"])]
    out["plnHeads"] = {i: heads[i] for i in ("MCP-PLN-01", "MCP-PLN-02")}
    out["plnPredicateDigests"] = {heads[i]: W.predicate_digest(next(r for r in doc["rows"] if r["versionedId"] == heads[i]))
                                  for i in ("MCP-PLN-01", "MCP-PLN-02")}
    print(json.dumps(out, indent=1, ensure_ascii=False))
    ok = out["checks"]["docDigestEqualsManifest"] and out["checks"]["manifestRowsEqual"] and not bad
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
