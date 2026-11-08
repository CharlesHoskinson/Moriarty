#!/usr/bin/env bash
# F21: the Lean composition theorems (task 2.12) build with zero `sorry`, declare no `axiom`, and
# every theorem's `#print axioms` (queried in AggregationAxioms.lean) stays within the standard
# Lean axioms; every theorem of Aggregation.lean is queried. Exit 2 while the toolchain is absent.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
MOD=clean/fork/Clean/Sherlock/Aggregation.lean
AX=clean/fork/Clean/Sherlock/AggregationAxioms.lean
[ -f "$MOD" ] && [ -f "$AX" ] || { echo "missing $MOD or $AX"; exit 1; }
command -v lake >/dev/null 2>&1 || { echo "UNIMPLEMENTED F21 (lake not on PATH; Lean toolchain absent)"; exit 2; }
if grep -nE '\bsorry\b' "$MOD" "$AX"; then echo "sorry found"; exit 1; fi
if grep -nE '^\s*(noncomputable\s+)?axiom\b' "$MOD" "$AX"; then echo "axiom declaration found"; exit 1; fi
# Memory cap (SRC-016): a user scope when systemd is reachable, otherwise plain.
run() {
  if [ -z "${LEAN_NO_SCOPE:-}" ] && command -v systemd-run >/dev/null 2>&1 &&
      systemd-run --user --scope --collect -q -p MemoryMax=1G -- true >/dev/null 2>&1; then
    systemd-run --user --scope --collect -q --expand-environment=no \
      -p "MemoryMax=${LEAN_MEMORY_MAX:-16G}" -p MemorySwapMax=0 -- "$@"
  else
    "$@"
  fi
}
out=$(run lake -d clean/fork build SherlockSpec 2>&1) || { echo "$out" | tail -40; exit 1; }
echo "$out" | tail -1
ax=$(run lake -d clean/fork env lean "$AX" 2>&1) || { echo "$ax" | tail -40; exit 1; }
AX_OUT="$ax" python3 -I - "$MOD" "$AX" <<'PY'
import os, re, sys
mod, ax = sys.argv[1], sys.argv[2]
src = open(mod, encoding="utf-8").read()
# Theorem names with their namespaces (the module opens `namespace NodeCert` blocks).
names, stack = [], []
for line in src.splitlines():
    m = re.match(r"^namespace\s+(\S+)", line)
    if m: stack.append(m.group(1)); continue
    m = re.match(r"^end\s+(\S+)", line)
    if m and stack and stack[-1] == m.group(1): stack.pop(); continue
    m = re.match(r"^(?:@\[[^\]]*\]\s*)?theorem\s+([A-Za-z0-9_.'₀-₉]+)", line)
    if m: names.append(".".join(stack + [m.group(1)]))
queried = set(re.findall(r"^#print axioms\s+(\S+)", open(ax, encoding="utf-8").read(), re.M))
missing = [n for n in names if n not in queried]
if missing:
    print("theorems without a #print axioms query:", *missing, sep="\n  "); sys.exit(1)
allowed = {"propext", "Quot.sound", "Classical.choice"}
seen, bad = set(), []
for line in os.environ["AX_OUT"].splitlines():
    m = re.search(r"'([^']+)' depends on axioms: \[([^\]]*)\]", line)
    if m:
        seen.add(m.group(1))
        axs = [a.strip() for a in m.group(2).split(",") if a.strip()]
        extra = [a for a in axs if a not in allowed]
        if extra: bad.append((m.group(1), extra))
        continue
    m = re.search(r"'([^']+)' does not depend on any axioms", line)
    if m: seen.add(m.group(1))
    if re.search(r"\berror\b", line): bad.append(("<output>", [line.strip()]))
unseen = [n for n in queried if n not in seen]
if unseen:
    print("queried but no axiom report (unknown name?):", *unseen, sep="\n  "); sys.exit(1)
if bad:
    print("non-standard axioms or errors:", *[f"{n}: {e}" for n, e in bad], sep="\n  "); sys.exit(1)
print(f"F21 ok: {len(names)} theorems, zero sorry, no axiom declarations, "
      f"{len(seen)} axiom reports within {sorted(allowed)}")
PY
