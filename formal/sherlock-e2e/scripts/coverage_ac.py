#!/usr/bin/env python3
"""Merge the per-context COVERAGE.part.md files of Formal-Lean-AC into
formal/sherlock-e2e/COVERAGE-FORMAL-AC.md and check that every EARS ID of the scope has
exactly one row with one of the four verdicts.

usage: coverage_ac.py INVENTORY.json   (the inventory of scripts/inventory_ac.py)
exit 1 when a row is missing, duplicated, blank or has an unknown verdict.
"""
import collections, json, pathlib, sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
CTX = ["wrap", "ledger-verification", "settlement", "governance", "network-operations", "placement",
       "compact-compilation", "sherlock-execution", "conformance", "call-flow"]
VERDICTS = {"MODELED", "CONSTANT-CHECKED", "SPEC-CHECKED", "FINDING"}

inv = json.load(open(sys.argv[1]))["rows"]
parts = {}
errors = []
for c in CTX:
    p = ROOT / c / "COVERAGE.part.md"
    if not p.exists():
        errors.append(f"missing {p}")
        continue
    for line in p.read_text().splitlines():
        if not line.startswith("| ") or line.startswith("| ID") or line.startswith("|---"):
            continue
        cells = [x.strip() for x in line.strip().strip("|").split("|")]
        if len(cells) < 7:
            errors.append(f"{c}: short row {line[:80]}")
            continue
        rid = cells[0]
        if rid in parts:
            errors.append(f"duplicate row {rid} ({parts[rid][0]} and {c})")
        if cells[3] not in VERDICTS:
            errors.append(f"{rid}: unknown verdict {cells[3]!r}")
        if not all(cells[:7]):
            errors.append(f"{rid}: blank cell")
        parts[rid] = (c, cells)

out = ["# Formal coverage of slices A and C and the completion change (Formal-Lean-AC)", "",
       "One row per EARS ID of the scope of Formal-Lean-AC: slice C (11 specs), slice A (6 specs),",
       "change sherlock-completion-20261115 (62 IDs) and the reused IDs that those specs adopt",
       "(inventory: `formal/sherlock-e2e/scripts/inventory_ac.py`). Verdicts per FORMAL-BRIEF.md.",
       "MODELED rows are final only when the Quint replay agrees; until then the Quint/replay column",
       "reads `pending`. Completion IDs that Formal-Quint placed in a slice B context are listed in",
       "the last section with a pointer to COVERAGE-FORMAL-B.md (no verdict here, so no double row).", "",
       "Rule for code gaps (LEAD PICK): the verdict names the kind of formal check of the EARS text.",
       "FINDING is kept for spec-level defects (two EARS that contradict, or an ambiguity that changes",
       "a transition). A gap between a requirement and the built code (CONFIRMED on code, or recorded",
       "as TO BUILD / PARTIAL in the spec's status) keeps the row's check verdict and names the gap and",
       "its FAL-* entry in the reason column. A CONSTANT-CHECKED row whose measured value misses its",
       "bound says `bound NOT met` (EUI-010, RFB-003).",
       "",
       "Rows shared with slice B (DG, LW, ANC, ZK, PRD adopted by both slices) appear here and in",
       "COVERAGE-FORMAL-B.md; the final COVERAGE-FORMAL.md merge keeps one row per ID.", ""]
by_group = collections.defaultdict(list)
counts = collections.Counter()
seen = set()
for r in inv:
    rid = r["id"]
    if rid in seen:
        errors.append(f"inventory duplicate {rid}")
    seen.add(rid)
    if r["assigned"].startswith("B:"):
        by_group["B"].append(f"| {rid} | {r['group']} | {r['assigned'][2:]} | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |")
        continue
    if rid not in parts:
        errors.append(f"no row for {rid} (context {r['assigned']})")
        continue
    c, cells = parts[rid]
    if c != r["assigned"]:
        errors.append(f"{rid}: row in {c}, assigned to {r['assigned']}")
    counts[cells[3]] += 1
    by_group[r["group"]].append("| " + " | ".join(cells[:7]) + " |")
extra = set(parts) - seen
for rid in sorted(extra):
    errors.append(f"row {rid} is not in the inventory")

hdr = ["| ID | group | context | verdict | Lean artifact | Quint/replay | source / reason |", "|---|---|---|---|---|---|---|"]
titles = {"C": "Slice C (sherlock-e2e-prototype-20261115)", "A": "Slice A (sherlock-e2e-prototype-20261115)",
          "completion": "Completion change (sherlock-completion-20261115), placed in A and C contexts",
          "reused": "Reused IDs adopted by slice A and C specs"}
out.append("## Totals")
out.append("")
out.append("| verdict | rows |")
out.append("|---|---|")
for v in ["MODELED", "CONSTANT-CHECKED", "SPEC-CHECKED", "FINDING"]:
    out.append(f"| {v} | {counts[v]} |")
out.append(f"| total with a verdict here | {sum(counts.values())} |")
out.append(f"| completion IDs placed in slice B contexts (pointer rows) | {len(by_group['B'])} |")
out.append("")
for g in ["C", "A", "completion", "reused"]:
    out += [f"## {titles[g]}", ""] + hdr + by_group[g] + [""]
out += ["## Completion IDs placed in slice B contexts", "",
        "| ID | group | context | verdict | note |", "|---|---|---|---|---|"] + by_group["B"] + [""]
(ROOT / "COVERAGE-FORMAL-AC.md").write_text("\n".join(out))
print(dict(counts), "B pointers", len(by_group["B"]))
if errors:
    print("\n".join(errors))
    sys.exit(1)
