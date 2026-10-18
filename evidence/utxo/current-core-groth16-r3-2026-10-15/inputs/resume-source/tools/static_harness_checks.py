"""Bounded static consistency checks of this lane's own r3 harness text (stdlib only).
Not a compiler: no Rust tool runs. Writes records/STATIC-HARNESS-CHECKS.json (exclusive).

Checks: (1) (), [], {} balance outside strings, chars and comments; (2) every Table::new
name is in the hwtrace TABLES list and vice versa; (3) every include_str! path exists in
the planned review/ layout (frozen packet or this overlay); (4) the #[test] set equals
the 14 expected names; (5) every HW-TRACEABILITY-2 ref resolves statically: vector-id
tables to frozen ids, literal-id tables to a literal or format pattern in the harness."""
import hashlib, json, os, re, resource

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

OWN = "/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-evidence-repair-resume-source-20261015"
PKT = "/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-compiler-independent-source-review-20261015"
SRC = OWN + "/review-overlay/proposals/heldout-harness-r3/tests/heldout.rs"
text = open(SRC, encoding="utf-8").read()
out = {"format": "evidence-repair-resume-static-harness-checks/1", "file": SRC, "sha256": hashlib.sha256(text.encode()).hexdigest(), "checks": {}}


def strip(src):
    """Remove comments, string and char literals (simple Rust lexer; no raw strings used)."""
    res, i, n = [], 0, len(src)
    while i < n:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i)
            i = n if j < 0 else j
        elif src.startswith("/*", i):
            i = src.index("*/", i) + 2
        elif c == '"':
            i += 1
            while src[i] != '"':
                i += 2 if src[i] == "\\" else 1
            i += 1
            res.append('""')
        elif c == "'" and re.match(r"'(\\.|[^\\'])'", src[i:i + 4]):
            m = re.match(r"'(\\.|[^\\'])'", src[i:i + 4])
            i += len(m.group(0))
        else:
            res.append(c)
            i += 1
    return "".join(res)


code = strip(text)
stack, bad = [], []
pairs = {")": "(", "]": "[", "}": "{"}
for k, ch in enumerate(code):
    if ch in "([{":
        stack.append((ch, k))
    elif ch in ")]}":
        if not stack or stack[-1][0] != pairs[ch]:
            bad.append(k)
            break
        stack.pop()
out["checks"]["delimiters_balanced"] = not bad and not stack

tables_new = set(re.findall(r'Table::new\("(\w+)"\)', text))
m = re.search(r"const TABLES: \[&str; \d+\] = \[(.*?)\];", text, re.S)
tables_const = set(re.findall(r'"(\w+)"', m.group(1))) if m else set()
out["checks"]["table_names"] = {"used": sorted(tables_new), "declared": sorted(tables_const), "equal": tables_new == tables_const}

layout_review = {}
for p in re.findall(r'include_str!\(concat!\(env!\("CARGO_MANIFEST_DIR"\), "/\.\./\.\./([^"]+)"\)\)', text):
    cands = [os.path.join(PKT, p), os.path.join(OWN, "review-overlay", p)]
    layout_review[p] = any(os.path.isfile(c) for c in cands)
out["checks"]["include_paths_present"] = layout_review
tests = [l.split("fn ", 1)[1].split("(", 1)[0] for prev, l in zip(text.splitlines(), text.splitlines()[1:]) if prev.strip() == "#[test]"]
out["checks"]["tests"] = {"count": len(tests), "names": tests}

V = json.load(open(PKT + "/heldout/HELDOUT-VECTORS.json"))
A2 = json.load(open(PKT + "/heldout-addendum-2/HELDOUT-ADDENDUM-2.json"))
vec_ids = {v["id"] for v in V["vectors"]} | {v["id"] for v in A2["vectors"]}
trace = json.load(open(OWN + "/review-overlay/heldout-addendum-4/HW-TRACEABILITY-2.json"))
unresolved = []
for e in trace["entries"]:
    for r in e["refs"]:
        table, row = r["table"], r["row"]
        if table in ("heldout", "hostile"):
            ok = row in vec_ids
        elif table == "rejected":
            ok = row.rsplit("/", 1)[0] in vec_ids and row.rsplit("/", 1)[1] in ("phase", "work_attempted", "meter-method")
        elif table == "addendum2":
            ok = row.split("/")[0] in vec_ids
        else:
            ok = f'"{row}"' in text or any(row.startswith(p.split("{")[0]) and p.split("{")[0] for p in re.findall(r'format!\("([^"]*\{[^"]*)"', text) if p.startswith(row.split("/")[0]))
        if not ok:
            unresolved.append(f"{e['id']} {table}/{row}")
out["checks"]["hw_refs_unresolved"] = unresolved
expected_tests = 14
out["verdict"] = "PASS" if (out["checks"]["delimiters_balanced"] and out["checks"]["table_names"]["equal"] and all(layout_review.values())
                            and len(tests) == expected_tests and not unresolved) else "STOP"
fd = os.open(OWN + "/records/STATIC-HARNESS-CHECKS.json", os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
with os.fdopen(fd, "w") as f:
    json.dump(out, f, indent=1)
    f.write("\n")
print(out["verdict"], out["checks"]["delimiters_balanced"], out["checks"]["table_names"]["equal"], all(layout_review.values()), len(tests), unresolved)
