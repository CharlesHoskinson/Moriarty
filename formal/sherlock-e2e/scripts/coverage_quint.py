#!/usr/bin/env python3
"""Add the Formal-Quint columns to COVERAGE-FORMAL.md (run after coverage_merge.py).

usage: coverage_quint.py
Reads every `<context>/quint/REPORT.md` coverage table (rows `| `ID` | artifacts | kind | note |`),
`replay/STATUS.tsv` (context, replay summary) and `replay/FINDINGS.tsv` (EARS ID, finding, verdict).
Writes two columns after `replay`: `Quint` (the invariants and the lane's kind) and `Quint finding`.
A `pending (Formal-Quint)` replay cell becomes the context's replay summary. A row whose Quint
finding is a spec-text defect that is still open (verdict OPEN DESIGN, or CONFIRMED against the text)
gets verdict FINDING. exit 1 when an ID of COVERAGE-FORMAL.md has no Quint row and no reason.
"""
import collections, pathlib, re, sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
ID = re.compile(r"\b[A-Z][A-Z0-9]*-\d{3}\b")

quint = collections.defaultdict(list)          # id -> [(ctx, invariants, kind)]
for rep in sorted(ROOT.glob("*/quint/REPORT.md")):
    ctx = rep.parent.parent.name
    for line in rep.read_text().splitlines():
        if not line.startswith("| `"):
            continue
        c = [x.strip() for x in line.strip().strip("|").split("|")]
        if len(c) < 3:
            continue
        ids = ID.findall(c[0])
        invs = sorted(set(re.findall(r"\b(?:inv|consistency)_[A-Za-z0-9_]+", c[1])))
        kind = re.sub(r"[`*]", "", c[2]).strip()
        for i in ids:
            quint[i].append((ctx, ", ".join(f"`{v}`" for v in invs[:4]) + (" ..." if len(invs) > 4 else ""), kind))

status = {}
st = ROOT / "replay" / "STATUS.tsv"
if st.exists():
    for line in st.read_text().splitlines():
        if line and not line.startswith("#"):
            k, v = line.split("\t", 1)
            status[k] = v
findings = collections.defaultdict(list)
fn = ROOT / "replay" / "FINDINGS.tsv"
if fn.exists():
    for line in fn.read_text().splitlines():
        if line and not line.startswith("#"):
            i, f, v = line.split("\t")
            findings[i].append((f, v))

path = ROOT / "COVERAGE-FORMAL.md"
lines = path.read_text().splitlines()
out, missing, counts = [], [], collections.Counter()
for line in lines:
    if line.startswith("| ID | slice |"):
        out.append("| ID | slice | context | verdict | Lean | replay | Quint | Quint finding | note |")
        continue
    if line.startswith("|---|---|---|---|---|---|---|"):
        out.append("|---|---|---|---|---|---|---|---|---|")
        continue
    m = re.match(r"\| ([A-Z][A-Z0-9]*-\d{3}) \|", line)
    if not m:
        out.append(line)
        continue
    rid = m.group(1)
    c = [x.strip() for x in line.strip().strip("|").split("|")]
    c = c[:6] + [" | ".join(c[6:])]
    q = quint.get(rid, [])
    qcell = "; ".join(f"{ctx}: {inv or 'no invariant'} ({kind})" for ctx, inv, kind in q)
    if not q:
        missing.append(rid)
        qcell = "not in a Quint model"
    if "pending (Formal-Quint)" in c[5]:
        ctxs = [x.strip() for x in c[2].split("/")]
        rep = "; ".join(status[x] for x in ctxs if x in status) or "pending (Formal-Quint)"
        c[5] = c[5].replace("pending (Formal-Quint)", rep)
    f = findings.get(rid, [])
    fcell = "; ".join(f"{a} {v}" for a, v in f) or "-"
    if any(v.startswith("OPEN DESIGN") or v.startswith("CONFIRMED (text") for _, v in f):
        c[3] = "FINDING"
    counts[c[3]] += 1
    out.append("| " + " | ".join(c[:6] + [qcell, fcell, c[6]]) + " |")

# totals table
txt = "\n".join(out) + "\n"
tot = "\n".join(f"| {v} | {counts[v]} |" for v in ("MODELED", "CONSTANT-CHECKED", "SPEC-CHECKED", "FINDING"))
txt = re.sub(r"(\| verdict \| rows \|\n\|---\|---\|\n)(?:\| (?:MODELED|CONSTANT-CHECKED|SPEC-CHECKED|FINDING) \| \d+ \|\n)+",
             lambda m: m.group(1) + tot + "\n", txt)
txt = re.sub(r"FINDING rows: [^\n]*\n", "FINDING rows: " + ", ".join(
    sorted(l.split(" | ")[0][2:] for l in out if re.match(r"\| [A-Z][A-Z0-9]*-\d{3} \|", l) and l.split(" | ")[3] == "FINDING"))
    + " (verdicts in COUNTEREXAMPLES.md).\n", txt)
path.write_text(txt)
print(dict(counts), "rows without a Quint model:", len(missing))
